use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{MsconsAggregierteZeitreihen, ZeitreihenTyp};
use mako_types::reducer::ReducerOutput;

/// MaBiS 4.2: Bilanzierungsdaten (Aggregierte Zeitreihen)
/// Multiple message flows: NB->BKV, NB->UeNB, UeNB->BKV, BKV->UeNB
/// Idle -> ZeitreihenGesendet -> FahrplanEmpfangen
#[derive(Debug, Clone, PartialEq)]
pub enum BilanzierungsdatenState {
	Idle,
	ZeitreihenGesendet {
		bilanzkreis: String,
		typ: ZeitreihenTyp,
	},
	FahrplanEmpfangen {
		bilanzkreis: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum BilanzierungsdatenEvent {
	ZeitreihenEmpfangen(MsconsAggregierteZeitreihen),
	FahrplanEmpfangen {
		bilanzkreis: String,
	},
}

pub fn reduce(
	state: BilanzierungsdatenState,
	event: BilanzierungsdatenEvent,
) -> Result<ReducerOutput<BilanzierungsdatenState>, ProzessFehler> {
	match (state, event) {
		// Idle + ZeitreihenEmpfangen -> ZeitreihenGesendet
		(
			BilanzierungsdatenState::Idle,
			BilanzierungsdatenEvent::ZeitreihenEmpfangen(z),
		) => Ok(ReducerOutput {
			state: BilanzierungsdatenState::ZeitreihenGesendet {
				bilanzkreis: z.bilanzkreis,
				typ: z.typ,
			},
			nachrichten: vec![],
		}),

		// ZeitreihenGesendet + FahrplanEmpfangen -> FahrplanEmpfangen
		(
			BilanzierungsdatenState::ZeitreihenGesendet { .. },
			BilanzierungsdatenEvent::FahrplanEmpfangen { bilanzkreis },
		) => Ok(ReducerOutput {
			state: BilanzierungsdatenState::FahrplanEmpfangen { bilanzkreis },
			nachrichten: vec![],
		}),

		// Catch-all: invalid transition
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
