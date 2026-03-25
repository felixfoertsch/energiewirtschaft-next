use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, MsconsSchlussturnusmesswert, UtilmdAblehnung, UtilmdLieferendeAbmeldung,
	UtilmdLieferendeBestaetigung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

#[derive(Debug, Clone, PartialEq)]
pub enum LieferendeState {
	Idle,
	AbmeldungGesendet {
		malo: MaLoId,
		lf: MarktpartnerId,
		nb: MarktpartnerId,
		lieferende: NaiveDate,
	},
	Bestaetigt {
		malo: MaLoId,
		lieferende: NaiveDate,
	},
	Abgeschlossen {
		malo: MaLoId,
		zaehlerstand: f64,
	},
	Abgelehnt {
		malo: MaLoId,
		grund: AblehnungsGrund,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum LieferendeEvent {
	AbmeldungEingegangen(UtilmdLieferendeAbmeldung),
	AbmeldungBestaetigt(UtilmdLieferendeBestaetigung),
	SchlussturnusmesswertEmpfangen(MsconsSchlussturnusmesswert),
	Abgelehnt { grund: AblehnungsGrund },
	FristUeberschritten,
}

pub fn reduce(
	state: LieferendeState,
	event: LieferendeEvent,
) -> Result<ReducerOutput<LieferendeState>, ProzessFehler> {
	match (state, event) {
		// 1.2.1: Idle + AbmeldungEingegangen -> AbmeldungGesendet
		(LieferendeState::Idle, LieferendeEvent::AbmeldungEingegangen(a)) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			Ok(ReducerOutput {
				state: LieferendeState::AbmeldungGesendet {
					malo: a.malo_id,
					lf: a.lieferant,
					nb,
					lieferende: a.lieferende,
				},
				nachrichten: vec![],
			})
		}

		// 1.2.2: AbmeldungGesendet + AbmeldungBestaetigt -> Bestaetigt + message
		(
			LieferendeState::AbmeldungGesendet { malo, lf, nb, lieferende },
			LieferendeEvent::AbmeldungBestaetigt(b),
		) => {
			let bestaetigung = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: lf,
				empfaenger_rolle: MarktRolle::Lieferant,
			pruef_id: None,
				payload: NachrichtenPayload::UtilmdLieferendeBestaetigung(
					UtilmdLieferendeBestaetigung {
						malo_id: malo.clone(),
						lieferende: b.lieferende,
					},
				),
			};
			Ok(ReducerOutput {
				state: LieferendeState::Bestaetigt {
					malo,
					lieferende,
				},
				nachrichten: vec![bestaetigung],
			})
		}

		// 1.2.3: Bestaetigt + SchlussturnusmesswertEmpfangen -> Abgeschlossen + message
		(
			LieferendeState::Bestaetigt { malo, .. },
			LieferendeEvent::SchlussturnusmesswertEmpfangen(m),
		) => {
			Ok(ReducerOutput {
				state: LieferendeState::Abgeschlossen {
					malo,
					zaehlerstand: m.zaehlerstand,
				},
				nachrichten: vec![],
			})
		}

		// Rejection from AbmeldungGesendet + Ablehnung to LF
		(
			LieferendeState::AbmeldungGesendet { malo, lf, nb, .. },
			LieferendeEvent::Abgelehnt { grund },
		) => {
			let ablehnung = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: lf,
				empfaenger_rolle: MarktRolle::Lieferant,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdAblehnung(UtilmdAblehnung {
					malo_id: malo.clone(),
					grund: grund.clone(),
				}),
			};
			Ok(ReducerOutput {
				state: LieferendeState::Abgelehnt { malo, grund },
				nachrichten: vec![ablehnung],
			})
		}

		// Timeout from AbmeldungGesendet + Ablehnung to LF
		(
			LieferendeState::AbmeldungGesendet { malo, lf, nb, .. },
			LieferendeEvent::FristUeberschritten,
		) => {
			let ablehnung = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: lf,
				empfaenger_rolle: MarktRolle::Lieferant,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdAblehnung(UtilmdAblehnung {
					malo_id: malo.clone(),
					grund: AblehnungsGrund::Fristverletzung,
				}),
			};
			Ok(ReducerOutput {
				state: LieferendeState::Abgelehnt {
					malo,
					grund: AblehnungsGrund::Fristverletzung,
				},
				nachrichten: vec![ablehnung],
			})
		}

		// Timeout from Bestaetigt (no routing info available for message)
		(
			LieferendeState::Bestaetigt { malo, .. },
			LieferendeEvent::FristUeberschritten,
		) => {
			Ok(ReducerOutput {
				state: LieferendeState::Abgelehnt {
					malo,
					grund: AblehnungsGrund::Fristverletzung,
				},
				nachrichten: vec![],
			})
		}

		// Catch-all: invalid transition
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
