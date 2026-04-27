use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdStammdaten;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BtrEivStammdatenState {
	Idle,
	Gesendet {
		ressource_id: String,
		absender: MarktpartnerId,
		empfaenger: MarktpartnerId,
	},
	Bestaetigt {
		ressource_id: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum BtrEivStammdatenEvent {
	StammdatenGesendet(RdStammdaten),
	Bestaetigt,
}

pub fn reduce(
	state: BtrEivStammdatenState,
	event: BtrEivStammdatenEvent,
) -> Result<ReducerOutput<BtrEivStammdatenState>, ProzessFehler> {
	match (state, event) {
		(BtrEivStammdatenState::Idle, BtrEivStammdatenEvent::StammdatenGesendet(sd)) => {
			let absender = MarktpartnerId::new("9900000000004").expect("valid id");
			let empfaenger = MarktpartnerId::new("9900000000005").expect("valid id");
			let nachricht = Nachricht {
				absender: absender.clone(),
				absender_rolle: BetreiberTechnischeRessource,
				empfaenger: empfaenger.clone(),
				empfaenger_rolle: Einsatzverantwortlicher,
				pruef_id: None,
				payload: NachrichtenPayload::RdStammdaten(sd.clone()),
			};
			Ok(ReducerOutput {
				state: BtrEivStammdatenState::Gesendet {
					ressource_id: sd.ressource_id,
					absender,
					empfaenger,
				},
				nachrichten: vec![nachricht],
			})
		}

		(
			BtrEivStammdatenState::Gesendet { ressource_id, .. },
			BtrEivStammdatenEvent::Bestaetigt,
		) => Ok(ReducerOutput {
			state: BtrEivStammdatenState::Bestaetigt { ressource_id },
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
