use chrono::NaiveDate;
use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, UtilmdMsbWechselAnmeldung};
use mako_types::ids::{MeLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// WiM 2.1 MSB-Wechsel states
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
		// 2.1.1: Idle + Anmeldung -> AnmeldungEingegangen
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

		// 2.1.2: AnmeldungEingegangen + NbBestaetigt -> Bestaetigt
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

		// Rejection
		(
			MsbWechselState::AnmeldungEingegangen { melo, .. },
			MsbWechselEvent::NbAbgelehnt { grund },
		) => {
			Ok(ReducerOutput {
				state: MsbWechselState::Abgelehnt { melo, grund },
				nachrichten: vec![],
			})
		}

		// 2.1.3: Bestaetigt + AbmeldungMsbAltInformiert -> AbmeldungInformiert
		(
			MsbWechselState::Bestaetigt { melo, msb_neu, wechseldatum },
			MsbWechselEvent::AbmeldungMsbAltInformiert,
		) => {
			Ok(ReducerOutput {
				state: MsbWechselState::AbmeldungInformiert { melo, msb_neu, wechseldatum },
				nachrichten: vec![],
			})
		}

		// 2.1.4: AbmeldungInformiert + Schlusszählerstand -> Abgeschlossen
		(
			MsbWechselState::AbmeldungInformiert { melo, .. },
			MsbWechselEvent::SchlusszaehlerstandEmpfangen { zaehlerstand },
		) => {
			Ok(ReducerOutput {
				state: MsbWechselState::Abgeschlossen {
					melo,
					schlusszaehlerstand: zaehlerstand,
				},
				nachrichten: vec![],
			})
		}

		// Timeout from AnmeldungEingegangen
		(
			MsbWechselState::AnmeldungEingegangen { melo, .. },
			MsbWechselEvent::FristUeberschritten,
		) => {
			Ok(ReducerOutput {
				state: MsbWechselState::Abgelehnt {
					melo,
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
