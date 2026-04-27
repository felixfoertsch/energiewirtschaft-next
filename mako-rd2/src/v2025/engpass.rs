use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdEngpass;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub const ENGPASS_ROLLENTUPEL: &[(MarktRolle, MarktRolle)] = &[
	(Netzbetreiber, DataProvider),
	(DataProvider, Anschlussnetzbetreiber),
	(Netzbetreiber, Anschlussnetzbetreiber),
];

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
			// MP-IDs entsprechen mako-cli/src/init.rs::ROLLEN — Index 1 = NB, 15 = ANB.
			let absender = MarktpartnerId::new("9900000000001").expect("valid id");
			let empfaenger = MarktpartnerId::new("9900000000015").expect("valid id");
			let (absender_rolle, empfaenger_rolle) = ENGPASS_ROLLENTUPEL[2];
			let nachricht = Nachricht {
				absender: absender.clone(),
				absender_rolle,
				empfaenger,
				empfaenger_rolle,
				pruef_id: None,
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

		(EngpassState::EngpassGemeldet { netzgebiet, .. }, EngpassEvent::Bestaetigt) => {
			Ok(ReducerOutput {
				state: EngpassState::Bestaetigt { netzgebiet },
				nachrichten: vec![],
			})
		}

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
