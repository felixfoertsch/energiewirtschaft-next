use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, Stammdatenfeld, UtilmdAblehnung, UtilmdStammdatenaenderung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

#[derive(Debug, Clone, PartialEq)]
pub enum StammdatenState {
	Idle,
	AenderungGesendet {
		malo: MaLoId,
		initiator: MarktpartnerId,
		aenderungen: Vec<Stammdatenfeld>,
	},
	AenderungBestaetigt {
		malo: MaLoId,
	},
	Abgelehnt {
		malo: MaLoId,
		grund: AblehnungsGrund,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum StammdatenEvent {
	AenderungEingegangen(UtilmdStammdatenaenderung),
	AenderungBestaetigt,
	AenderungAbgelehnt { grund: AblehnungsGrund },
	FristUeberschritten,
}

pub fn reduce(
	state: StammdatenState,
	event: StammdatenEvent,
) -> Result<ReducerOutput<StammdatenState>, ProzessFehler> {
	match (state, event) {
		// Idle + AenderungEingegangen -> AenderungGesendet
		(StammdatenState::Idle, StammdatenEvent::AenderungEingegangen(a)) => {
			Ok(ReducerOutput {
				state: StammdatenState::AenderungGesendet {
					malo: a.malo_id,
					initiator: a.initiator,
					aenderungen: a.aenderungen,
				},
				nachrichten: vec![],
			})
		}

		// AenderungGesendet + AenderungBestaetigt -> AenderungBestaetigt + confirmation to initiator
		(
			StammdatenState::AenderungGesendet { malo, initiator, aenderungen },
			StammdatenEvent::AenderungBestaetigt,
		) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let bestaetigung = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: initiator.clone(),
				empfaenger_rolle: MarktRolle::Lieferant,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdStammdatenaenderung(
					UtilmdStammdatenaenderung {
						malo_id: malo.clone(),
						initiator,
						aenderungen,
					},
				),
			};
			Ok(ReducerOutput {
				state: StammdatenState::AenderungBestaetigt { malo },
				nachrichten: vec![bestaetigung],
			})
		}

		// AenderungGesendet + AenderungAbgelehnt -> Abgelehnt + Ablehnung to initiator
		(
			StammdatenState::AenderungGesendet { malo, initiator, .. },
			StammdatenEvent::AenderungAbgelehnt { grund },
		) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let ablehnung = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: initiator,
				empfaenger_rolle: MarktRolle::Lieferant,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdAblehnung(UtilmdAblehnung {
					malo_id: malo.clone(),
					grund: grund.clone(),
				}),
			};
			Ok(ReducerOutput {
				state: StammdatenState::Abgelehnt { malo, grund },
				nachrichten: vec![ablehnung],
			})
		}

		// Timeout from AenderungGesendet + Ablehnung to initiator
		(
			StammdatenState::AenderungGesendet { malo, initiator, .. },
			StammdatenEvent::FristUeberschritten,
		) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let ablehnung = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: initiator,
				empfaenger_rolle: MarktRolle::Lieferant,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdAblehnung(UtilmdAblehnung {
					malo_id: malo.clone(),
					grund: AblehnungsGrund::Fristverletzung,
				}),
			};
			Ok(ReducerOutput {
				state: StammdatenState::Abgelehnt {
					malo,
					grund: AblehnungsGrund::Fristverletzung,
				},
				nachrichten: vec![ablehnung],
			})
		}

		// Catch-all: invalid transition
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
