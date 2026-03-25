use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdNichtverfuegbarkeit;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

#[derive(Debug, Clone, PartialEq)]
pub enum NichtverfuegbarkeitState {
	Idle,
	/// RD 7.4.2: Unavailability reported
	Gemeldet {
		ressource_id: String,
		absender: MarktpartnerId,
	},
	/// Forwarded to ÜNB
	Weitergeleitet {
		ressource_id: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum NichtverfuegbarkeitEvent {
	Gemeldet(RdNichtverfuegbarkeit),
	Weitergeleitet,
}

pub fn reduce(
	state: NichtverfuegbarkeitState,
	event: NichtverfuegbarkeitEvent,
) -> Result<ReducerOutput<NichtverfuegbarkeitState>, ProzessFehler> {
	match (state, event) {
		(
			NichtverfuegbarkeitState::Idle,
			NichtverfuegbarkeitEvent::Gemeldet(nv),
		) => {
			let absender = MarktpartnerId::new("9900000000003").expect("valid id");
			let empfaenger = MarktpartnerId::new("9900000000010").expect("valid id");
			let nachricht = Nachricht {
				absender: absender.clone(),
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger,
				empfaenger_rolle: MarktRolle::Uebertragungsnetzbetreiber,
			pruef_id: None,
				payload: NachrichtenPayload::RdNichtverfuegbarkeit(nv.clone()),
			};
			Ok(ReducerOutput {
				state: NichtverfuegbarkeitState::Gemeldet {
					ressource_id: nv.ressource_id,
					absender,
				},
				nachrichten: vec![nachricht],
			})
		}

		(
			NichtverfuegbarkeitState::Gemeldet { ressource_id, .. },
			NichtverfuegbarkeitEvent::Weitergeleitet,
		) => Ok(ReducerOutput {
			state: NichtverfuegbarkeitState::Weitergeleitet { ressource_id },
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
