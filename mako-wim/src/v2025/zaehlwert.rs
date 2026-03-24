use mako_types::fehler::ProzessFehler;
use mako_types::ids::MeLoId;
use mako_types::reducer::ReducerOutput;

/// WiM 2.3 Zählwertübermittlung states
#[derive(Debug, Clone, PartialEq)]
pub enum ZaehlwertState {
	Idle,
	MesswerteGesendet {
		melo: MeLoId,
	},
	Plausibilisiert {
		melo: MeLoId,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum ZaehlwertEvent {
	MesswerteGesendet { melo: MeLoId },
	PlausibilisierteWerteEmpfangen,
	FristUeberschritten,
}

pub fn reduce(
	state: ZaehlwertState,
	event: ZaehlwertEvent,
) -> Result<ReducerOutput<ZaehlwertState>, ProzessFehler> {
	match (state, event) {
		// 2.3.1: Idle + MesswerteGesendet -> MesswerteGesendet
		(ZaehlwertState::Idle, ZaehlwertEvent::MesswerteGesendet { melo }) => {
			Ok(ReducerOutput {
				state: ZaehlwertState::MesswerteGesendet { melo },
				nachrichten: vec![],
			})
		}

		// 2.3.2: MesswerteGesendet + Plausibilisierung -> Plausibilisiert
		(
			ZaehlwertState::MesswerteGesendet { melo },
			ZaehlwertEvent::PlausibilisierteWerteEmpfangen,
		) => {
			Ok(ReducerOutput {
				state: ZaehlwertState::Plausibilisiert { melo },
				nachrichten: vec![],
			})
		}

		// Timeout from MesswerteGesendet (no plausibilisierte Werte received)
		(
			ZaehlwertState::MesswerteGesendet { .. },
			ZaehlwertEvent::FristUeberschritten,
		) => Err(ProzessFehler::FristUeberschritten {
			frist: "WiM 2.3 Plausibilisierung".to_string(),
			eingang: "timeout".to_string(),
		}),

		// Catch-all
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
