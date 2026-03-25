use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, MsconsSchlussturnusmesswert, UtilmdLieferendeAbmeldung,
	UtilmdLieferendeBestaetigung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// GeLi Gas 2.2: Lieferende (Gas)
/// Idle -> AbmeldungGesendet -> Bestaetigt -> Abgeschlossen
/// With Gastag-aware Schlussturnusmesswert.
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
	/// Gastag-aware: zaehlerstand in m³, converted via Gasumrechnung
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

fn malo_from_state(state: &LieferendeState) -> MaLoId {
	match state {
		LieferendeState::Idle => unreachable!("Idle has no MaLoId"),
		LieferendeState::AbmeldungGesendet { malo, .. }
		| LieferendeState::Bestaetigt { malo, .. }
		| LieferendeState::Abgeschlossen { malo, .. }
		| LieferendeState::Abgelehnt { malo, .. } => malo.clone(),
	}
}

pub fn reduce(
	state: LieferendeState,
	event: LieferendeEvent,
) -> Result<ReducerOutput<LieferendeState>, ProzessFehler> {
	match (state, event) {
		// 2.2.1: Idle + AbmeldungEingegangen -> AbmeldungGesendet
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

		// 2.2.2: AbmeldungGesendet + AbmeldungBestaetigt -> Bestaetigt + message
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
				state: LieferendeState::Bestaetigt { malo, lieferende },
				nachrichten: vec![bestaetigung],
			})
		}

		// 2.2.3: Bestaetigt + SchlussturnusmesswertEmpfangen -> Abgeschlossen
		(
			LieferendeState::Bestaetigt { malo, .. },
			LieferendeEvent::SchlussturnusmesswertEmpfangen(m),
		) => Ok(ReducerOutput {
			state: LieferendeState::Abgeschlossen {
				malo,
				zaehlerstand: m.zaehlerstand,
			},
			nachrichten: vec![],
		}),

		// Rejection from AbmeldungGesendet
		(
			LieferendeState::AbmeldungGesendet { malo, .. },
			LieferendeEvent::Abgelehnt { grund },
		) => Ok(ReducerOutput {
			state: LieferendeState::Abgelehnt { malo, grund },
			nachrichten: vec![],
		}),

		// Timeout from any waiting state
		(
			ref s @ (LieferendeState::AbmeldungGesendet { .. }
			| LieferendeState::Bestaetigt { .. }),
			LieferendeEvent::FristUeberschritten,
		) => {
			let malo = malo_from_state(s);
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
