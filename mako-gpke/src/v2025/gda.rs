use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, Stammdatenfeld, UtilmdAblehnung, UtilmdGeschaeftsdatenanfrage,
	UtilmdGeschaeftsdatenantwort,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

#[derive(Debug, Clone, PartialEq)]
pub enum GdaState {
	Idle,
	AnfrageGesendet {
		malo: MaLoId,
		anfragender: MarktpartnerId,
	},
	Beantwortet {
		malo: MaLoId,
		stammdaten: Vec<Stammdatenfeld>,
	},
	Abgelehnt {
		malo: MaLoId,
		grund: AblehnungsGrund,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum GdaEvent {
	AnfrageEingegangen(UtilmdGeschaeftsdatenanfrage),
	AntwortEmpfangen(UtilmdGeschaeftsdatenantwort),
	Abgelehnt { grund: AblehnungsGrund },
	FristUeberschritten,
}

pub fn reduce(
	state: GdaState,
	event: GdaEvent,
) -> Result<ReducerOutput<GdaState>, ProzessFehler> {
	match (state, event) {
		// 1.5.1: Idle + AnfrageEingegangen -> AnfrageGesendet
		(GdaState::Idle, GdaEvent::AnfrageEingegangen(a)) => {
			Ok(ReducerOutput {
				state: GdaState::AnfrageGesendet {
					malo: a.malo_id,
					anfragender: a.anfragender,
				},
				nachrichten: vec![],
			})
		}

		// 1.5.2: AnfrageGesendet + AntwortEmpfangen -> Beantwortet + message
		(
			GdaState::AnfrageGesendet { malo, anfragender },
			GdaEvent::AntwortEmpfangen(antwort),
		) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let nachricht = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: anfragender,
				empfaenger_rolle: MarktRolle::Lieferant,
			pruef_id: None,
				payload: NachrichtenPayload::UtilmdGeschaeftsdatenantwort(
					UtilmdGeschaeftsdatenantwort {
						malo_id: malo.clone(),
						stammdaten: antwort.stammdaten.clone(),
					},
				),
			};
			Ok(ReducerOutput {
				state: GdaState::Beantwortet {
					malo,
					stammdaten: antwort.stammdaten,
				},
				nachrichten: vec![nachricht],
			})
		}

		// Rejection from AnfrageGesendet + Ablehnung to anfragender
		(
			GdaState::AnfrageGesendet { malo, anfragender },
			GdaEvent::Abgelehnt { grund },
		) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let ablehnung = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: anfragender,
				empfaenger_rolle: MarktRolle::Lieferant,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdAblehnung(UtilmdAblehnung {
					malo_id: malo.clone(),
					grund: grund.clone(),
				}),
			};
			Ok(ReducerOutput {
				state: GdaState::Abgelehnt { malo, grund },
				nachrichten: vec![ablehnung],
			})
		}

		// Timeout from AnfrageGesendet + Ablehnung to anfragender
		(
			GdaState::AnfrageGesendet { malo, anfragender },
			GdaEvent::FristUeberschritten,
		) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let ablehnung = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: anfragender,
				empfaenger_rolle: MarktRolle::Lieferant,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdAblehnung(UtilmdAblehnung {
					malo_id: malo.clone(),
					grund: AblehnungsGrund::Fristverletzung,
				}),
			};
			Ok(ReducerOutput {
				state: GdaState::Abgelehnt {
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
