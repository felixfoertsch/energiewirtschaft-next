use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{UtilmdZuordnungsliste, ZuordnungsEintrag};
use mako_types::reducer::ReducerOutput;

/// GeLi Gas 2.4: Zuordnungsprozesse (Gas)
/// Same as GPKE zuordnung but for Gas MaLos.
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
		(ZuordnungState::Idle, ZuordnungEvent::ListeEmpfangen(liste)) => Ok(ReducerOutput {
			state: ZuordnungState::ListeVersendet {
				eintraege: liste.eintraege,
			},
			nachrichten: vec![],
		}),

		(ZuordnungState::ListeVersendet { .. }, ZuordnungEvent::Bestaetigt) => Ok(ReducerOutput {
			state: ZuordnungState::Bestaetigt,
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
