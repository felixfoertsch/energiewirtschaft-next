use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdAktivierung;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

#[derive(Debug, Clone, PartialEq)]
pub enum AbrufState {
	Idle,
	/// RD 7.3.1: Abruf sent by ÜNB
	AbrufGesendet {
		ressource_id: String,
		absender: MarktpartnerId,
	},
	/// RD 7.3.2: Forwarded to ANB
	Weitergeleitet {
		ressource_id: String,
	},
	/// RD 7.3.3: Quittiert (acknowledged)
	Quittiert {
		ressource_id: String,
	},
	Abgelehnt {
		ressource_id: String,
		grund: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum AbrufEvent {
	AbrufGesendet(RdAktivierung),
	Weitergeleitet,
	Quittiert,
	Abgelehnt { grund: String },
}

pub fn reduce(
	state: AbrufState,
	event: AbrufEvent,
) -> Result<ReducerOutput<AbrufState>, ProzessFehler> {
	match (state, event) {
		(AbrufState::Idle, AbrufEvent::AbrufGesendet(ak)) => {
			let absender = MarktpartnerId::new("9900000000010").expect("valid id");
			let empfaenger = MarktpartnerId::new("9900000000003").expect("valid id");
			let nachricht = Nachricht {
				absender: absender.clone(),
				absender_rolle: MarktRolle::Uebertragungsnetzbetreiber,
				empfaenger,
				empfaenger_rolle: MarktRolle::Netzbetreiber,
				payload: NachrichtenPayload::RdAktivierung(ak.clone()),
			};
			Ok(ReducerOutput {
				state: AbrufState::AbrufGesendet {
					ressource_id: ak.ressource_id,
					absender,
				},
				nachrichten: vec![nachricht],
			})
		}

		(
			AbrufState::AbrufGesendet { ressource_id, .. },
			AbrufEvent::Weitergeleitet,
		) => Ok(ReducerOutput {
			state: AbrufState::Weitergeleitet { ressource_id },
			nachrichten: vec![],
		}),

		(
			AbrufState::Weitergeleitet { ressource_id },
			AbrufEvent::Quittiert,
		) => Ok(ReducerOutput {
			state: AbrufState::Quittiert { ressource_id },
			nachrichten: vec![],
		}),

		(
			AbrufState::Weitergeleitet { ressource_id },
			AbrufEvent::Abgelehnt { grund },
		) => Ok(ReducerOutput {
			state: AbrufState::Abgelehnt {
				ressource_id,
				grund,
			},
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
