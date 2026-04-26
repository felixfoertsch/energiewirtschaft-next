use chrono::NaiveDate;
use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, UtilmdMsbWechselAnmeldung};
use mako_types::ids::{MeLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle::*;

/// WiM 2022 (MaKo 2022, gueltig ab 01.10.2023): MSB-Wechsel
///
/// Same process as v2025, minor AHB changes.
/// Fristen:
/// - Pruefungsfrist NB: 5 WT
/// - Abmeldung MSB-alt: 3 WT
#[derive(Debug, Clone, PartialEq)]
pub enum MsbWechselState {
	Idle,
	AnmeldungEingegangen {
		melo: MeLoId,
		msb_neu: MarktpartnerId,
		wechseldatum: NaiveDate,
	},
	Bestaetigt {
		melo: MeLoId,
		msb_neu: MarktpartnerId,
		wechseldatum: NaiveDate,
	},
	Abgelehnt {
		melo: MeLoId,
		grund: AblehnungsGrund,
	},
	AbmeldungInformiert {
		melo: MeLoId,
		msb_neu: MarktpartnerId,
		wechseldatum: NaiveDate,
	},
	Abgeschlossen {
		melo: MeLoId,
		schlusszaehlerstand: f64,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum MsbWechselEvent {
	AnmeldungEmpfangen(UtilmdMsbWechselAnmeldung),
	NbBestaetigt,
	NbAbgelehnt { grund: AblehnungsGrund },
	AbmeldungMsbAltInformiert,
	SchlusszaehlerstandEmpfangen { zaehlerstand: f64 },
	FristUeberschritten,
}

fn nb() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").expect("valid NB id")
}

fn msb_alt() -> MarktpartnerId {
	MarktpartnerId::new("9900000000028").expect("valid MSBA id")
}

pub fn reduce(
	state: MsbWechselState,
	event: MsbWechselEvent,
) -> Result<ReducerOutput<MsbWechselState>, ProzessFehler> {
	match (state, event) {
		// Idle + Anmeldung -> AnmeldungEingegangen (MSBN → NB)
		(MsbWechselState::Idle, MsbWechselEvent::AnmeldungEmpfangen(anm)) => {
			let nachricht = Nachricht {
				absender: anm.msb_neu.clone(),
				absender_rolle: MessstellenbetreiberNeu,
				empfaenger: nb(),
				empfaenger_rolle: Netzbetreiber,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdMsbWechselAnmeldung(UtilmdMsbWechselAnmeldung {
					melo_id: anm.melo_id.clone(),
					msb_neu: anm.msb_neu.clone(),
					wechseldatum: anm.wechseldatum,
				}),
			};
			Ok(ReducerOutput {
				state: MsbWechselState::AnmeldungEingegangen {
					melo: anm.melo_id,
					msb_neu: anm.msb_neu,
					wechseldatum: anm.wechseldatum,
				},
				nachrichten: vec![nachricht],
			})
		}

		// AnmeldungEingegangen + NbBestaetigt -> Bestaetigt (NB → MSBN)
		(
			MsbWechselState::AnmeldungEingegangen { melo, msb_neu, wechseldatum },
			MsbWechselEvent::NbBestaetigt,
		) => {
			let nachricht = Nachricht {
				absender: nb(),
				absender_rolle: Netzbetreiber,
				empfaenger: msb_neu.clone(),
				empfaenger_rolle: MessstellenbetreiberNeu,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdMsbWechselAnmeldung(
					UtilmdMsbWechselAnmeldung {
						melo_id: melo.clone(),
						msb_neu: msb_neu.clone(),
						wechseldatum,
					},
				),
			};
			Ok(ReducerOutput {
				state: MsbWechselState::Bestaetigt { melo, msb_neu, wechseldatum },
				nachrichten: vec![nachricht],
			})
		}

		(
			MsbWechselState::AnmeldungEingegangen { melo, .. },
			MsbWechselEvent::NbAbgelehnt { grund },
		) => Ok(ReducerOutput {
			state: MsbWechselState::Abgelehnt { melo, grund },
			nachrichten: vec![],
		}),

		// Bestaetigt + AbmeldungMsbAltInformiert -> AbmeldungInformiert (NB → MSBA)
		(
			MsbWechselState::Bestaetigt { melo, msb_neu, wechseldatum },
			MsbWechselEvent::AbmeldungMsbAltInformiert,
		) => {
			let nachricht = Nachricht {
				absender: nb(),
				absender_rolle: Netzbetreiber,
				empfaenger: msb_alt(),
				empfaenger_rolle: MessstellenbetreiberAlt,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdMsbWechselAnmeldung(
					UtilmdMsbWechselAnmeldung {
						melo_id: melo.clone(),
						msb_neu: msb_neu.clone(),
						wechseldatum,
					},
				),
			};
			Ok(ReducerOutput {
				state: MsbWechselState::AbmeldungInformiert { melo, msb_neu, wechseldatum },
				nachrichten: vec![nachricht],
			})
		}

		(
			MsbWechselState::AbmeldungInformiert { melo, .. },
			MsbWechselEvent::SchlusszaehlerstandEmpfangen { zaehlerstand },
		) => Ok(ReducerOutput {
			state: MsbWechselState::Abgeschlossen {
				melo,
				schlusszaehlerstand: zaehlerstand,
			},
			nachrichten: vec![],
		}),

		(
			MsbWechselState::AnmeldungEingegangen { melo, .. },
			MsbWechselEvent::FristUeberschritten,
		) => Ok(ReducerOutput {
			state: MsbWechselState::Abgelehnt {
				melo,
				grund: AblehnungsGrund::Fristverletzung,
			},
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
