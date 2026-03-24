use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdEngpass;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

#[derive(Debug, Clone, PartialEq)]
pub enum EngpassState {
	Idle,
	/// RD 7.4.1: Engpass reported
	EngpassGemeldet {
		netzgebiet: String,
		absender: MarktpartnerId,
	},
	/// Acknowledged
	Bestaetigt {
		netzgebiet: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum EngpassEvent {
	EngpassGemeldet(RdEngpass),
	Bestaetigt,
}

pub fn reduce(
	state: EngpassState,
	event: EngpassEvent,
) -> Result<ReducerOutput<EngpassState>, ProzessFehler> {
	match (state, event) {
		(EngpassState::Idle, EngpassEvent::EngpassGemeldet(e)) => {
			let absender = MarktpartnerId::new("9900000000010").expect("valid id");
			let empfaenger = MarktpartnerId::new("9900000000003").expect("valid id");
			let nachricht = Nachricht {
				absender: absender.clone(),
				absender_rolle: MarktRolle::Uebertragungsnetzbetreiber,
				empfaenger,
				empfaenger_rolle: MarktRolle::Netzbetreiber,
				payload: NachrichtenPayload::RdEngpass(e.clone()),
			};
			Ok(ReducerOutput {
				state: EngpassState::EngpassGemeldet {
					netzgebiet: e.netzgebiet,
					absender,
				},
				nachrichten: vec![nachricht],
			})
		}

		(
			EngpassState::EngpassGemeldet { netzgebiet, .. },
			EngpassEvent::Bestaetigt,
		) => Ok(ReducerOutput {
			state: EngpassState::Bestaetigt { netzgebiet },
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
