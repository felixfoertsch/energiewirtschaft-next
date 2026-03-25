use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, UtilmdAbmeldung, UtilmdAnmeldung, UtilmdBestaetigung, UtilmdZuordnung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// GeLi Gas 2.1: Lieferantenwechsel (Gas)
/// Same 6-state pattern as GPKE LFW but with Gas-specific fristen
/// (10 WT prüfung, 5 WT abmeldung, 3 WT widerspruch instead of 1 WT).
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
		// 2.1.1: Idle + AnmeldungEmpfangen -> AnmeldungEingegangen
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

		// 2.1.2: AnmeldungEingegangen + AnmeldungBestaetigt -> AbmeldungAnLfaGesendet + 2 messages
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

		// 2.1.3: AbmeldungAnLfaGesendet + LfaHatBestaetigt -> WiderspruchsfristLaeuft
		(
			LfwState::AbmeldungAnLfaGesendet { malo, lfn, lfa, nb, lieferbeginn },
			LfwEvent::LfaHatBestaetigt,
		) => {
			// Gas Widerspruchsfrist: 3 WT (simplified to lieferbeginn here)
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

		// 2.1.4: AbmeldungAnLfaGesendet + LfaHatAbgelehnt -> Abgelehnt
		(
			LfwState::AbmeldungAnLfaGesendet { malo, .. },
			LfwEvent::LfaHatAbgelehnt { grund },
		) => Ok(ReducerOutput {
			state: LfwState::Abgelehnt { malo, grund },
			nachrichten: vec![],
		}),

		// 2.1.5: WiderspruchsfristLaeuft + WiderspruchsfristAbgelaufen -> Zugeordnet + 2 messages
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

		// Timeout from any waiting state -> Abgelehnt(Fristverletzung)
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

		// Catch-all: invalid transition
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
