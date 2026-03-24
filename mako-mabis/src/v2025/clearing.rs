use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{ClearingEintrag, UtilmdClearingliste};
use mako_types::reducer::ReducerOutput;

/// MaBiS 4.4: Clearing
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
		// Idle + ListeEmpfangen -> ListeGesendet
		(ClearingState::Idle, ClearingEvent::ListeEmpfangen(liste)) => {
			Ok(ReducerOutput {
				state: ClearingState::ListeGesendet {
					eintraege: liste.eintraege,
				},
				nachrichten: vec![],
			})
		}

		// ListeGesendet + AntwortEmpfangen -> AntwortEmpfangen
		(
			ClearingState::ListeGesendet { .. },
			ClearingEvent::AntwortEmpfangen(antwort),
		) => Ok(ReducerOutput {
			state: ClearingState::AntwortEmpfangen {
				eintraege: antwort.eintraege,
			},
			nachrichten: vec![],
		}),

		// Catch-all: invalid transition
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
