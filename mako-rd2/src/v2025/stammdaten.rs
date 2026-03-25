use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdStammdaten;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

#[derive(Debug, Clone, PartialEq)]
pub enum StammdatenState {
	Idle,
	/// RD 7.1.1: Stammdaten sent by ANB/BKV to ÜNB
	StammdatenGesendet {
		ressource_id: String,
		absender: MarktpartnerId,
		empfaenger: MarktpartnerId,
	},
	/// RD 7.1.2: ÜNB forwarded to downstream
	Weitergeleitet {
		ressource_id: String,
		absender: MarktpartnerId,
	},
	/// RD 7.1.3: Acknowledged
	Bestaetigt {
		ressource_id: String,
	},
	/// Terminal failure
	Abgelehnt {
		ressource_id: String,
		grund: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum StammdatenEvent {
	StammdatenGesendet(RdStammdaten),
	Weitergeleitet,
	Bestaetigt,
	Abgelehnt { grund: String },
}

pub fn reduce(
	state: StammdatenState,
	event: StammdatenEvent,
) -> Result<ReducerOutput<StammdatenState>, ProzessFehler> {
	match (state, event) {
		(StammdatenState::Idle, StammdatenEvent::StammdatenGesendet(sd)) => {
			let absender = MarktpartnerId::new("9900000000003").expect("valid id");
			let empfaenger = MarktpartnerId::new("9900000000010").expect("valid id");
			let nachricht = Nachricht {
				absender: absender.clone(),
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: empfaenger.clone(),
				empfaenger_rolle: MarktRolle::Uebertragungsnetzbetreiber,
			pruef_id: None,
				payload: NachrichtenPayload::RdStammdaten(sd.clone()),
			};
			Ok(ReducerOutput {
				state: StammdatenState::StammdatenGesendet {
					ressource_id: sd.ressource_id,
					absender,
					empfaenger,
				},
				nachrichten: vec![nachricht],
			})
		}

		(
			StammdatenState::StammdatenGesendet {
				ressource_id, absender, ..
			},
			StammdatenEvent::Weitergeleitet,
		) => Ok(ReducerOutput {
			state: StammdatenState::Weitergeleitet {
				ressource_id,
				absender,
			},
			nachrichten: vec![],
		}),

		(
			StammdatenState::Weitergeleitet { ressource_id, .. },
			StammdatenEvent::Bestaetigt,
		) => Ok(ReducerOutput {
			state: StammdatenState::Bestaetigt { ressource_id },
			nachrichten: vec![],
		}),

		(
			StammdatenState::Weitergeleitet { ressource_id, .. },
			StammdatenEvent::Abgelehnt { grund },
		) => Ok(ReducerOutput {
			state: StammdatenState::Abgelehnt {
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
