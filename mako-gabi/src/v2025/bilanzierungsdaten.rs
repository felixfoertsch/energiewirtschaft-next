use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{MsconsAggregierteZeitreihen, ZeitreihenTyp};
use mako_types::reducer::ReducerOutput;

/// GABi Gas 4.2: Bilanzierungsdaten (Gas)
/// Idle -> AllokationsdatenGesendet -> SummenzeitreihenGesendet -> AbrechnungEmpfangen
#[derive(Debug, Clone, PartialEq)]
pub enum BilanzierungsdatenState {
	Idle,
	AllokationsdatenGesendet {
		bilanzkreis: String,
		typ: ZeitreihenTyp,
	},
	SummenzeitreihenGesendet {
		bilanzkreis: String,
	},
	AbrechnungEmpfangen {
		bilanzkreis: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum BilanzierungsdatenEvent {
	AllokationsdatenEmpfangen(MsconsAggregierteZeitreihen),
	SummenzeitreihenEmpfangen { bilanzkreis: String },
	AbrechnungEmpfangen { bilanzkreis: String },
}

pub fn reduce(
	state: BilanzierungsdatenState,
	event: BilanzierungsdatenEvent,
) -> Result<ReducerOutput<BilanzierungsdatenState>, ProzessFehler> {
	match (state, event) {
		// Idle + AllokationsdatenEmpfangen -> AllokationsdatenGesendet
		(
			BilanzierungsdatenState::Idle,
			BilanzierungsdatenEvent::AllokationsdatenEmpfangen(z),
		) => Ok(ReducerOutput {
			state: BilanzierungsdatenState::AllokationsdatenGesendet {
				bilanzkreis: z.bilanzkreis,
				typ: z.typ,
			},
			nachrichten: vec![],
		}),

		// AllokationsdatenGesendet + SummenzeitreihenEmpfangen -> SummenzeitreihenGesendet
		(
			BilanzierungsdatenState::AllokationsdatenGesendet { .. },
			BilanzierungsdatenEvent::SummenzeitreihenEmpfangen { bilanzkreis },
		) => Ok(ReducerOutput {
			state: BilanzierungsdatenState::SummenzeitreihenGesendet { bilanzkreis },
			nachrichten: vec![],
		}),

		// SummenzeitreihenGesendet + AbrechnungEmpfangen -> AbrechnungEmpfangen
		(
			BilanzierungsdatenState::SummenzeitreihenGesendet { .. },
			BilanzierungsdatenEvent::AbrechnungEmpfangen { bilanzkreis },
		) => Ok(ReducerOutput {
			state: BilanzierungsdatenState::AbrechnungEmpfangen { bilanzkreis },
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
