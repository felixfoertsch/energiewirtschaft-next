use chrono::NaiveDate;
use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::UtilmdGeraetewechsel;
use mako_types::ids::{MeLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// WiM 2.2 Gerätewechsel states
#[derive(Debug, Clone, PartialEq)]
pub enum GeraetewechselState {
	Idle,
	WechselGemeldet {
		melo: MeLoId,
		alte_geraete_nr: String,
		neue_geraete_nr: String,
		wechseldatum: NaiveDate,
	},
	NbInformiert {
		melo: MeLoId,
		alte_geraete_nr: String,
		neue_geraete_nr: String,
		wechseldatum: NaiveDate,
	},
	ZaehlerstaendeUebermittelt {
		melo: MeLoId,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeraetewechselEvent {
	WechselGemeldet(UtilmdGeraetewechsel),
	NbHatWeitergeleitet,
	ZaehlerstaendeGesendet,
	FristUeberschritten,
}

pub fn reduce(
	state: GeraetewechselState,
	event: GeraetewechselEvent,
) -> Result<ReducerOutput<GeraetewechselState>, ProzessFehler> {
	match (state, event) {
		// 2.2.1: Idle + WechselGemeldet -> WechselGemeldet
		(GeraetewechselState::Idle, GeraetewechselEvent::WechselGemeldet(gw)) => {
			let msb = MarktpartnerId::new("9900000000027").expect("valid MSB id");
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let nachricht = Nachricht {
				absender: msb,
				absender_rolle: MarktRolle::Messstellenbetreiber,
				empfaenger: nb,
				empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
				payload: NachrichtenPayload::UtilmdGeraetewechsel(UtilmdGeraetewechsel {
					melo_id: gw.melo_id.clone(),
					alte_geraete_nr: gw.alte_geraete_nr.clone(),
					neue_geraete_nr: gw.neue_geraete_nr.clone(),
					wechseldatum: gw.wechseldatum,
				}),
			};
			Ok(ReducerOutput {
				state: GeraetewechselState::WechselGemeldet {
					melo: gw.melo_id,
					alte_geraete_nr: gw.alte_geraete_nr,
					neue_geraete_nr: gw.neue_geraete_nr,
					wechseldatum: gw.wechseldatum,
				},
				nachrichten: vec![nachricht],
			})
		}

		// 2.2.2: WechselGemeldet + NB forwards to LF -> NbInformiert
		(
			GeraetewechselState::WechselGemeldet { melo, alte_geraete_nr, neue_geraete_nr, wechseldatum },
			GeraetewechselEvent::NbHatWeitergeleitet,
		) => {
			Ok(ReducerOutput {
				state: GeraetewechselState::NbInformiert {
					melo,
					alte_geraete_nr,
					neue_geraete_nr,
					wechseldatum,
				},
				nachrichten: vec![],
			})
		}

		// 2.2.3: NbInformiert + MSB sends Zählerstände -> ZaehlerstaendeUebermittelt
		(
			GeraetewechselState::NbInformiert { melo, .. },
			GeraetewechselEvent::ZaehlerstaendeGesendet,
		) => {
			Ok(ReducerOutput {
				state: GeraetewechselState::ZaehlerstaendeUebermittelt { melo },
				nachrichten: vec![],
			})
		}

		// Timeout from WechselGemeldet (NB did not forward in time)
		(
			GeraetewechselState::WechselGemeldet { .. },
			GeraetewechselEvent::FristUeberschritten,
		) => Err(ProzessFehler::FristUeberschritten {
			frist: "WiM 2.2 NB-Weiterleitung".to_string(),
			eingang: "timeout".to_string(),
		}),

		// Catch-all
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
