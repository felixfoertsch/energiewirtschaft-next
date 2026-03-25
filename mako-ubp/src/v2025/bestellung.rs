use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	OrdersBestellung, OrdrspBestellantwort, QuotesAngebot, ReqoteAngebotsanfrage,
};
use mako_types::ids::{MeLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// UBP 3.1 Messprodukt-Bestellung states
#[derive(Debug, Clone, PartialEq)]
pub enum BestellungState {
	Idle,
	AnfrageGesendet {
		melo: MeLoId,
		anfragender: MarktpartnerId,
		produkt: String,
	},
	AngebotErhalten {
		melo: MeLoId,
		anfragender: MarktpartnerId,
		anbieter: MarktpartnerId,
		preis_ct_pro_monat: f64,
		produkt: String,
	},
	Bestellt {
		melo: MeLoId,
		besteller: MarktpartnerId,
		referenz_angebot: String,
	},
	Bestaetigt {
		melo: MeLoId,
	},
	Abgelehnt {
		melo: MeLoId,
		grund: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum BestellungEvent {
	AnfrageGesendet(ReqoteAngebotsanfrage),
	AngebotEmpfangen(QuotesAngebot),
	BestellungGesendet(OrdersBestellung),
	AntwortEmpfangen(OrdrspBestellantwort),
	FristUeberschritten,
}

pub fn reduce(
	state: BestellungState,
	event: BestellungEvent,
) -> Result<ReducerOutput<BestellungState>, ProzessFehler> {
	match (state, event) {
		// 3.1.1: Idle + AnfrageGesendet -> AnfrageGesendet
		(BestellungState::Idle, BestellungEvent::AnfrageGesendet(req)) => {
			let msb = MarktpartnerId::new("9900000000027").expect("valid MSB id");
			let nachricht = Nachricht {
				absender: req.anfragender.clone(),
				absender_rolle: MarktRolle::Lieferant,
				empfaenger: msb,
				empfaenger_rolle: MarktRolle::Messstellenbetreiber,
			pruef_id: None,
				payload: NachrichtenPayload::ReqoteAngebotsanfrage(ReqoteAngebotsanfrage {
					melo_id: req.melo_id.clone(),
					anfragender: req.anfragender.clone(),
					produkt_beschreibung: req.produkt_beschreibung.clone(),
				}),
			};
			Ok(ReducerOutput {
				state: BestellungState::AnfrageGesendet {
					melo: req.melo_id,
					anfragender: req.anfragender,
					produkt: req.produkt_beschreibung,
				},
				nachrichten: vec![nachricht],
			})
		}

		// 3.1.2: AnfrageGesendet + AngebotEmpfangen -> AngebotErhalten
		(
			BestellungState::AnfrageGesendet { melo, anfragender, produkt: _ },
			BestellungEvent::AngebotEmpfangen(angebot),
		) => {
			Ok(ReducerOutput {
				state: BestellungState::AngebotErhalten {
					melo,
					anfragender,
					anbieter: angebot.anbieter,
					preis_ct_pro_monat: angebot.preis_ct_pro_monat,
					produkt: angebot.produkt_beschreibung,
				},
				nachrichten: vec![],
			})
		}

		// 3.1.3: AngebotErhalten + BestellungGesendet -> Bestellt
		(
			BestellungState::AngebotErhalten { melo, anfragender, .. },
			BestellungEvent::BestellungGesendet(best),
		) => {
			let msb = MarktpartnerId::new("9900000000027").expect("valid MSB id");
			let nachricht = Nachricht {
				absender: anfragender,
				absender_rolle: MarktRolle::Lieferant,
				empfaenger: msb,
				empfaenger_rolle: MarktRolle::Messstellenbetreiber,
			pruef_id: None,
				payload: NachrichtenPayload::OrdersBestellung(OrdersBestellung {
					melo_id: best.melo_id.clone(),
					besteller: best.besteller.clone(),
					referenz_angebot: best.referenz_angebot.clone(),
				}),
			};
			Ok(ReducerOutput {
				state: BestellungState::Bestellt {
					melo,
					besteller: best.besteller,
					referenz_angebot: best.referenz_angebot,
				},
				nachrichten: vec![nachricht],
			})
		}

		// 3.1.4: Bestellt + AntwortEmpfangen -> Bestaetigt or Abgelehnt
		(
			BestellungState::Bestellt { melo, .. },
			BestellungEvent::AntwortEmpfangen(antwort),
		) => {
			if antwort.angenommen {
				Ok(ReducerOutput {
					state: BestellungState::Bestaetigt { melo },
					nachrichten: vec![],
				})
			} else {
				Ok(ReducerOutput {
					state: BestellungState::Abgelehnt {
						melo,
						grund: antwort.grund.unwrap_or_else(|| "Kein Grund angegeben".to_string()),
					},
					nachrichten: vec![],
				})
			}
		}

		// Timeout from AnfrageGesendet
		(
			BestellungState::AnfrageGesendet { melo, .. },
			BestellungEvent::FristUeberschritten,
		) => {
			Ok(ReducerOutput {
				state: BestellungState::Abgelehnt {
					melo,
					grund: "Frist überschritten".to_string(),
				},
				nachrichten: vec![],
			})
		}

		// Timeout from Bestellt
		(
			BestellungState::Bestellt { melo, .. },
			BestellungEvent::FristUeberschritten,
		) => {
			Ok(ReducerOutput {
				state: BestellungState::Abgelehnt {
					melo,
					grund: "Frist überschritten".to_string(),
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
