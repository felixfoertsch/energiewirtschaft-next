use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, UtilmdAbmeldung, UtilmdAnmeldung, UtilmdBestaetigung, UtilmdZuordnung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// GPKE 2020 (MaKo 2020, gueltig ab 01.02.2020): Lieferantenwechsel
///
/// First introduction of the current 6-step process structure.
/// MSB acts as data hub for meter data.
/// Fristen:
/// - Pruefungsfrist NB: 7 WT
/// - Widerspruchsfrist LFA: 5 WT
/// - Vorlaufzeit Anmeldung: 6 Wochen
#[derive(Debug, Clone, PartialEq)]
pub enum LfwState {
	Idle,
	AnmeldungEingegangen {
		malo: MaLoId,
		lfn: MarktpartnerId,
		nb: MarktpartnerId,
		lieferbeginn: NaiveDate,
	},
	AbmeldungAnLfaGesendet {
		malo: MaLoId,
		lfn: MarktpartnerId,
		lfa: MarktpartnerId,
		nb: MarktpartnerId,
		lieferbeginn: NaiveDate,
	},
	/// Widerspruchsfrist: 5 WT
	WiderspruchsfristLaeuft {
		malo: MaLoId,
		lfn: MarktpartnerId,
		lfa: MarktpartnerId,
		nb: MarktpartnerId,
		lieferbeginn: NaiveDate,
		frist_bis: NaiveDate,
	},
	Zugeordnet {
		malo: MaLoId,
		lfn: MarktpartnerId,
		lieferbeginn: NaiveDate,
	},
	Abgelehnt {
		malo: MaLoId,
		grund: AblehnungsGrund,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum LfwEvent {
	AnmeldungEmpfangen(UtilmdAnmeldung),
	AnmeldungBestaetigt { lfa: MarktpartnerId },
	LfaHatBestaetigt,
	LfaHatAbgelehnt { grund: AblehnungsGrund },
	WiderspruchsfristAbgelaufen,
	FristUeberschritten,
}

fn malo_from_state(state: &LfwState) -> MaLoId {
	match state {
		LfwState::Idle => unreachable!("Idle has no MaLoId"),
		LfwState::AnmeldungEingegangen { malo, .. }
		| LfwState::AbmeldungAnLfaGesendet { malo, .. }
		| LfwState::WiderspruchsfristLaeuft { malo, .. }
		| LfwState::Zugeordnet { malo, .. }
		| LfwState::Abgelehnt { malo, .. } => malo.clone(),
	}
}

pub fn reduce(state: LfwState, event: LfwEvent) -> Result<ReducerOutput<LfwState>, ProzessFehler> {
	match (state, event) {
		(LfwState::Idle, LfwEvent::AnmeldungEmpfangen(a)) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			Ok(ReducerOutput {
				state: LfwState::AnmeldungEingegangen {
					malo: a.malo_id,
					lfn: a.lieferant_neu,
					nb,
					lieferbeginn: a.lieferbeginn,
				},
				nachrichten: vec![],
			})
		}

		(
			LfwState::AnmeldungEingegangen { malo, lfn, nb, lieferbeginn },
			LfwEvent::AnmeldungBestaetigt { lfa },
		) => {
			let bestaetigung = Nachricht {
				absender: nb.clone(),
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: lfn.clone(),
				empfaenger_rolle: MarktRolle::LieferantNeu,
			pruef_id: None,
				payload: NachrichtenPayload::UtilmdBestaetigung(UtilmdBestaetigung {
					malo_id: malo.clone(),
					bestaetigt_fuer: lfn.clone(),
					lieferbeginn,
				}),
			};
			let abmeldung = Nachricht {
				absender: nb.clone(),
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: lfa.clone(),
				empfaenger_rolle: MarktRolle::LieferantAlt,
			pruef_id: None,
				payload: NachrichtenPayload::UtilmdAbmeldung(UtilmdAbmeldung {
					malo_id: malo.clone(),
					lieferant_alt: lfa.clone(),
					lieferende: lieferbeginn,
				}),
			};
			Ok(ReducerOutput {
				state: LfwState::AbmeldungAnLfaGesendet {
					malo,
					lfn,
					lfa,
					nb,
					lieferbeginn,
				},
				nachrichten: vec![bestaetigung, abmeldung],
			})
		}

		(
			LfwState::AbmeldungAnLfaGesendet { malo, lfn, lfa, nb, lieferbeginn },
			LfwEvent::LfaHatBestaetigt,
		) => {
			// v2020 Widerspruchsfrist: 5 WT
			let frist_bis = lieferbeginn;
			Ok(ReducerOutput {
				state: LfwState::WiderspruchsfristLaeuft {
					malo,
					lfn,
					lfa,
					nb,
					lieferbeginn,
					frist_bis,
				},
				nachrichten: vec![],
			})
		}

		(
			LfwState::AbmeldungAnLfaGesendet { malo, .. },
			LfwEvent::LfaHatAbgelehnt { grund },
		) => Ok(ReducerOutput {
			state: LfwState::Abgelehnt { malo, grund },
			nachrichten: vec![],
		}),

		(
			LfwState::WiderspruchsfristLaeuft { malo, lfn, lfa, nb, lieferbeginn, .. },
			LfwEvent::WiderspruchsfristAbgelaufen,
		) => {
			let zuordnung_lfn = Nachricht {
				absender: nb.clone(),
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: lfn.clone(),
				empfaenger_rolle: MarktRolle::LieferantNeu,
			pruef_id: None,
				payload: NachrichtenPayload::UtilmdZuordnung(UtilmdZuordnung {
					malo_id: malo.clone(),
					zugeordnet_an: lfn.clone(),
					lieferbeginn,
				}),
			};
			let zuordnung_lfa = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: lfa.clone(),
				empfaenger_rolle: MarktRolle::LieferantAlt,
			pruef_id: None,
				payload: NachrichtenPayload::UtilmdZuordnung(UtilmdZuordnung {
					malo_id: malo.clone(),
					zugeordnet_an: lfn.clone(),
					lieferbeginn,
				}),
			};
			Ok(ReducerOutput {
				state: LfwState::Zugeordnet { malo, lfn, lieferbeginn },
				nachrichten: vec![zuordnung_lfn, zuordnung_lfa],
			})
		}

		(
			ref s @ (LfwState::AnmeldungEingegangen { .. }
			| LfwState::AbmeldungAnLfaGesendet { .. }
			| LfwState::WiderspruchsfristLaeuft { .. }),
			LfwEvent::FristUeberschritten,
		) => {
			let malo = malo_from_state(s);
			Ok(ReducerOutput {
				state: LfwState::Abgelehnt {
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
