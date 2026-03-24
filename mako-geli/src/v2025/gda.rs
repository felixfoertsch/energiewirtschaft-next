use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, Stammdatenfeld, UtilmdGeschaeftsdatenanfrage, UtilmdGeschaeftsdatenantwort,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// GeLi Gas 2.5: Geschäftsdatenanfrage (Gas)
/// Same as GPKE GDA but for Gas MaLos.
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

fn malo_from_state(state: &GdaState) -> MaLoId {
	match state {
		GdaState::Idle => unreachable!("Idle has no MaLoId"),
		GdaState::AnfrageGesendet { malo, .. }
		| GdaState::Beantwortet { malo, .. }
		| GdaState::Abgelehnt { malo, .. } => malo.clone(),
	}
}

pub fn reduce(
	state: GdaState,
	event: GdaEvent,
) -> Result<ReducerOutput<GdaState>, ProzessFehler> {
	match (state, event) {
		// 2.5.1: Idle + AnfrageEingegangen -> AnfrageGesendet
		(GdaState::Idle, GdaEvent::AnfrageEingegangen(a)) => Ok(ReducerOutput {
			state: GdaState::AnfrageGesendet {
				malo: a.malo_id,
				anfragender: a.anfragender,
			},
			nachrichten: vec![],
		}),

		// 2.5.2: AnfrageGesendet + AntwortEmpfangen -> Beantwortet + message
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

		// Rejection from AnfrageGesendet
		(GdaState::AnfrageGesendet { malo, .. }, GdaEvent::Abgelehnt { grund }) => {
			Ok(ReducerOutput {
				state: GdaState::Abgelehnt { malo, grund },
				nachrichten: vec![],
			})
		}

		// Timeout from AnfrageGesendet
		(ref s @ GdaState::AnfrageGesendet { .. }, GdaEvent::FristUeberschritten) => {
			let malo = malo_from_state(s);
			Ok(ReducerOutput {
				state: GdaState::Abgelehnt {
					malo,
					grund: AblehnungsGrund::Fristverletzung,
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
