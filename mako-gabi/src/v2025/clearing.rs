use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{ClearingEintrag, UtilmdClearingliste};
use mako_types::reducer::ReducerOutput;

/// GABi Gas 4.5: Clearing (Gas)
/// Same pattern as MaBiS.
/// Idle -> ListeGesendet -> AntwortEmpfangen
#[derive(Debug, Clone, PartialEq)]
pub enum ClearingState {
	Idle,
	ListeGesendet {
		eintraege: Vec<ClearingEintrag>,
	},
	AntwortEmpfangen {
		eintraege: Vec<ClearingEintrag>,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClearingEvent {
	ListeEmpfangen(UtilmdClearingliste),
	AntwortEmpfangen(UtilmdClearingliste),
}

pub fn reduce(
	state: ClearingState,
	event: ClearingEvent,
) -> Result<ReducerOutput<ClearingState>, ProzessFehler> {
	match (state, event) {
		(ClearingState::Idle, ClearingEvent::ListeEmpfangen(liste)) => Ok(ReducerOutput {
			state: ClearingState::ListeGesendet {
				eintraege: liste.eintraege,
			},
			nachrichten: vec![],
		}),

		(
			ClearingState::ListeGesendet { .. },
			ClearingEvent::AntwortEmpfangen(antwort),
		) => Ok(ReducerOutput {
			state: ClearingState::AntwortEmpfangen {
				eintraege: antwort.eintraege,
			},
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
