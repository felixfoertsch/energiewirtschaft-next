use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	MatchingErgebnis, Nominierung, NominierungBestaetigung, Renominierung,
};
use mako_types::reducer::ReducerOutput;

/// GABi Gas 4.3: Nominierung / Renominierung
/// Idle -> Nominiert -> Bestaetigt / TeilweiseBestaetigt / Abgelehnt -> Renominiert -> Bestaetigt
#[derive(Debug, Clone, PartialEq)]
pub enum NominierungState {
	Idle,
	Nominiert {
		bilanzkreis: String,
	},
	Bestaetigt {
		bilanzkreis: String,
	},
	TeilweiseBestaetigt {
		bilanzkreis: String,
		bestaetigte_menge_kwh: f64,
	},
	Abgelehnt {
		bilanzkreis: String,
		grund: String,
	},
	Renominiert {
		bilanzkreis: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum NominierungEvent {
	NominierungEingegangen(Nominierung),
	BestaetigungEmpfangen(NominierungBestaetigung),
	RenominierungEingegangen(Renominierung),
	RenominierungBestaetigt,
}

pub fn reduce(
	state: NominierungState,
	event: NominierungEvent,
) -> Result<ReducerOutput<NominierungState>, ProzessFehler> {
	match (state, event) {
		// 4.3.1: Idle + NominierungEingegangen -> Nominiert
		(NominierungState::Idle, NominierungEvent::NominierungEingegangen(n)) => Ok(ReducerOutput {
			state: NominierungState::Nominiert {
				bilanzkreis: n.bilanzkreis,
			},
			nachrichten: vec![],
		}),

		// 4.3.2: Nominiert + BestaetigungEmpfangen -> Bestaetigt / TeilweiseBestaetigt / Abgelehnt
		(
			NominierungState::Nominiert { bilanzkreis },
			NominierungEvent::BestaetigungEmpfangen(b),
		) => {
			let new_state = match b.matching_ergebnis {
				MatchingErgebnis::Bestaetigt => NominierungState::Bestaetigt { bilanzkreis },
				MatchingErgebnis::TeilweiseBestaetigt { bestaetigte_menge_kwh } => {
					NominierungState::TeilweiseBestaetigt {
						bilanzkreis,
						bestaetigte_menge_kwh,
					}
				}
				MatchingErgebnis::Abgelehnt { grund } => {
					NominierungState::Abgelehnt { bilanzkreis, grund }
				}
			};
			Ok(ReducerOutput {
				state: new_state,
				nachrichten: vec![],
			})
		}

		// 4.3.3: TeilweiseBestaetigt + RenominierungEingegangen -> Renominiert
		(
			NominierungState::TeilweiseBestaetigt { .. },
			NominierungEvent::RenominierungEingegangen(r),
		) => Ok(ReducerOutput {
			state: NominierungState::Renominiert {
				bilanzkreis: r.bilanzkreis,
			},
			nachrichten: vec![],
		}),

		// 4.3.4: Renominiert + RenominierungBestaetigt -> Bestaetigt
		(
			NominierungState::Renominiert { bilanzkreis },
			NominierungEvent::RenominierungBestaetigt,
		) => Ok(ReducerOutput {
			state: NominierungState::Bestaetigt { bilanzkreis },
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
