use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, OrdersWerteAnfrage};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// WiM 2.4 Werte-Anfrage states
#[derive(Debug, Clone, PartialEq)]
pub enum WerteAnfrageState {
	Idle,
	AnfrageGesendet {
		malo: MaLoId,
		anfragender: MarktpartnerId,
	},
	Beantwortet {
		malo: MaLoId,
	},
	Abgelehnt {
		malo: MaLoId,
		grund: AblehnungsGrund,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum WerteAnfrageEvent {
	AnfrageGesendet(OrdersWerteAnfrage),
	WerteGeliefert,
	Abgelehnt { grund: AblehnungsGrund },
	FristUeberschritten,
}

pub fn reduce(
	state: WerteAnfrageState,
	event: WerteAnfrageEvent,
) -> Result<ReducerOutput<WerteAnfrageState>, ProzessFehler> {
	match (state, event) {
		// 2.4.1: Idle + AnfrageGesendet -> AnfrageGesendet
		(WerteAnfrageState::Idle, WerteAnfrageEvent::AnfrageGesendet(anfrage)) => {
			let msb = MarktpartnerId::new("9900000000027").expect("valid MSB id");
			let nachricht = Nachricht {
				absender: anfrage.anfragender.clone(),
				absender_rolle: MarktRolle::Lieferant,
				empfaenger: msb,
				empfaenger_rolle: MarktRolle::Messstellenbetreiber,
			pruef_id: None,
				payload: NachrichtenPayload::OrdersWerteAnfrage(OrdersWerteAnfrage {
					malo_id: anfrage.malo_id.clone(),
					anfragender: anfrage.anfragender.clone(),
					zeitraum_von: anfrage.zeitraum_von,
					zeitraum_bis: anfrage.zeitraum_bis,
				}),
			};
			Ok(ReducerOutput {
				state: WerteAnfrageState::AnfrageGesendet {
					malo: anfrage.malo_id,
					anfragender: anfrage.anfragender,
				},
				nachrichten: vec![nachricht],
			})
		}

		// 2.4.2: AnfrageGesendet + WerteGeliefert -> Beantwortet
		(
			WerteAnfrageState::AnfrageGesendet { malo, .. },
			WerteAnfrageEvent::WerteGeliefert,
		) => {
			Ok(ReducerOutput {
				state: WerteAnfrageState::Beantwortet { malo },
				nachrichten: vec![],
			})
		}

		// Rejection
		(
			WerteAnfrageState::AnfrageGesendet { malo, .. },
			WerteAnfrageEvent::Abgelehnt { grund },
		) => {
			Ok(ReducerOutput {
				state: WerteAnfrageState::Abgelehnt { malo, grund },
				nachrichten: vec![],
			})
		}

		// Timeout
		(
			WerteAnfrageState::AnfrageGesendet { malo, .. },
			WerteAnfrageEvent::FristUeberschritten,
		) => {
			Ok(ReducerOutput {
				state: WerteAnfrageState::Abgelehnt {
					malo,
					grund: AblehnungsGrund::Fristverletzung,
				},
				nachrichten: vec![],
			})
		}

		// Catch-all
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
