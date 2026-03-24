use chrono::NaiveDate;
use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, UtilmdMsbWechselAnmeldung};
use mako_types::ids::{MeLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// WiM 2020 (MaKo 2020, gueltig ab 01.02.2020): MSB-Wechsel
///
/// MSB-centric Messwesen. First introduction of MSB as data hub.
/// Fristen:
/// - Pruefungsfrist NB: 7 WT
/// - Abmeldung MSB-alt: 5 WT
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

pub fn reduce(
	state: MsbWechselState,
	event: MsbWechselEvent,
) -> Result<ReducerOutput<MsbWechselState>, ProzessFehler> {
	match (state, event) {
		(MsbWechselState::Idle, MsbWechselEvent::AnmeldungEmpfangen(anm)) => {
			Ok(ReducerOutput {
				state: MsbWechselState::AnmeldungEingegangen {
					melo: anm.melo_id,
					msb_neu: anm.msb_neu,
					wechseldatum: anm.wechseldatum,
				},
				nachrichten: vec![],
			})
		}

		(
			MsbWechselState::AnmeldungEingegangen { melo, msb_neu, wechseldatum },
			MsbWechselEvent::NbBestaetigt,
		) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let nachricht = Nachricht {
				absender: nb,
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: msb_neu.clone(),
				empfaenger_rolle: MarktRolle::Messstellenbetreiber,
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

		(
			MsbWechselState::Bestaetigt { melo, msb_neu, wechseldatum },
			MsbWechselEvent::AbmeldungMsbAltInformiert,
		) => Ok(ReducerOutput {
			state: MsbWechselState::AbmeldungInformiert { melo, msb_neu, wechseldatum },
			nachrichten: vec![],
		}),

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
