use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{UtilmdZuordnungsliste, ZuordnungsEintrag};
use mako_types::reducer::ReducerOutput;

#[derive(Debug, Clone, PartialEq)]
pub enum ZuordnungState {
	Idle,
	ListeVersendet {
		eintraege: Vec<ZuordnungsEintrag>,
	},
	Bestaetigt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ZuordnungEvent {
	ListeEmpfangen(UtilmdZuordnungsliste),
	Bestaetigt,
}

pub fn reduce(
	state: ZuordnungState,
	event: ZuordnungEvent,
) -> Result<ReducerOutput<ZuordnungState>, ProzessFehler> {
	match (state, event) {
		// Idle + ListeEmpfangen -> ListeVersendet
		(ZuordnungState::Idle, ZuordnungEvent::ListeEmpfangen(liste)) => {
			Ok(ReducerOutput {
				state: ZuordnungState::ListeVersendet {
					eintraege: liste.eintraege,
				},
				nachrichten: vec![],
			})
		}

		// ListeVersendet + Bestaetigt -> Bestaetigt
		(ZuordnungState::ListeVersendet { .. }, ZuordnungEvent::Bestaetigt) => {
			Ok(ReducerOutput {
				state: ZuordnungState::Bestaetigt,
				nachrichten: vec![],
			})
		}

		// Catch-all: invalid transition
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
