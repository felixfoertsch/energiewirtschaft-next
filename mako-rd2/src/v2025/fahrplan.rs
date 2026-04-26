use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdFahrplan;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub const FAHRPLAN_ROLLENTUPEL: &[(MarktRolle, MarktRolle)] = &[
	(Einsatzverantwortlicher, Uebertragungsnetzbetreiber),
	(Einsatzverantwortlicher, DataProvider),
	(DataProvider, Anschlussnetzbetreiber),
	(Anschlussnetzbetreiber, Einsatzverantwortlicher),
	(Anschlussnetzbetreiber, DataProvider),
	(Netzbetreiber, Anschlussnetzbetreiber),
	(AnfordernderNetzbetreiber, DataProvider),
	(DataProvider, Netzbetreiber),
	(AnfordernderNetzbetreiber, Anschlussnetzbetreiber),
];

#[derive(Debug, Clone, PartialEq)]
pub enum FahrplanState {
	Idle,
	/// RD 7.2.1: Fahrplan sent
	FahrplanGesendet {
		ressource_id: String,
		absender: MarktpartnerId,
	},
	/// RD 7.2.2: Forwarded by ÜNB
	Weitergeleitet {
		ressource_id: String,
	},
	/// RD 7.2.3: Acknowledged
	Bestaetigt {
		ressource_id: String,
	},
	Abgelehnt {
		ressource_id: String,
		grund: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum FahrplanEvent {
	FahrplanGesendet(RdFahrplan),
	Weitergeleitet,
	Bestaetigt,
	Abgelehnt { grund: String },
}

pub fn reduce(
	state: FahrplanState,
	event: FahrplanEvent,
) -> Result<ReducerOutput<FahrplanState>, ProzessFehler> {
	match (state, event) {
		(FahrplanState::Idle, FahrplanEvent::FahrplanGesendet(fp)) => {
			let absender = MarktpartnerId::new("9900000000003").expect("valid id");
			let empfaenger = MarktpartnerId::new("9900000000010").expect("valid id");
			let (absender_rolle, empfaenger_rolle) = FAHRPLAN_ROLLENTUPEL[0];
			let nachricht = Nachricht {
				absender: absender.clone(),
				absender_rolle,
				empfaenger,
				empfaenger_rolle,
				pruef_id: None,
				payload: NachrichtenPayload::RdFahrplan(fp.clone()),
			};
			Ok(ReducerOutput {
				state: FahrplanState::FahrplanGesendet {
					ressource_id: fp.ressource_id,
					absender,
				},
				nachrichten: vec![nachricht],
			})
		}

		(FahrplanState::FahrplanGesendet { ressource_id, .. }, FahrplanEvent::Weitergeleitet) => {
			Ok(ReducerOutput {
				state: FahrplanState::Weitergeleitet { ressource_id },
				nachrichten: vec![],
			})
		}

		(FahrplanState::Weitergeleitet { ressource_id }, FahrplanEvent::Bestaetigt) => {
			Ok(ReducerOutput {
				state: FahrplanState::Bestaetigt { ressource_id },
				nachrichten: vec![],
			})
		}

		(FahrplanState::Weitergeleitet { ressource_id }, FahrplanEvent::Abgelehnt { grund }) => {
			Ok(ReducerOutput {
				state: FahrplanState::Abgelehnt {
					ressource_id,
					grund,
				},
				nachrichten: vec![],
			})
		}

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
