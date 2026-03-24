use chrono::NaiveDate;
use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::UtilmdGeraetewechsel;
use mako_types::ids::{MeLoId, MarktpartnerId};
use mako_types::nachricht::NachrichtenPayload;

use super::geraetewechsel::{GeraetewechselEvent, GeraetewechselState, reduce};

fn melo() -> MeLoId {
	MeLoId::new("DE000000000000000000000000000000A").unwrap()
}
fn msb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}
fn wechseldatum() -> NaiveDate {
	NaiveDate::from_ymd_opt(2025, 8, 15).unwrap()
}

fn geraetewechsel() -> UtilmdGeraetewechsel {
	UtilmdGeraetewechsel {
		melo_id: melo(),
		alte_geraete_nr: "ALT-001".to_string(),
		neue_geraete_nr: "NEU-002".to_string(),
		wechseldatum: wechseldatum(),
	}
}

// --- Happy path ---

#[test]
fn idle_plus_wechsel_gemeldet() {
	let out = reduce(
		GeraetewechselState::Idle,
		GeraetewechselEvent::WechselGemeldet(geraetewechsel()),
	)
	.expect("should succeed");
	assert!(matches!(out.state, GeraetewechselState::WechselGemeldet { .. }));
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender, msb_id());
	assert_eq!(out.nachrichten[0].empfaenger, nb_id());
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::UtilmdGeraetewechsel(_)
	));
}

#[test]
fn wechsel_gemeldet_plus_nb_weitergeleitet() {
	let state = GeraetewechselState::WechselGemeldet {
		melo: melo(),
		alte_geraete_nr: "ALT-001".to_string(),
		neue_geraete_nr: "NEU-002".to_string(),
		wechseldatum: wechseldatum(),
	};
	let out = reduce(state, GeraetewechselEvent::NbHatWeitergeleitet).expect("should succeed");
	assert!(matches!(out.state, GeraetewechselState::NbInformiert { .. }));
}

#[test]
fn nb_informiert_plus_zaehlerstaende() {
	let state = GeraetewechselState::NbInformiert {
		melo: melo(),
		alte_geraete_nr: "ALT-001".to_string(),
		neue_geraete_nr: "NEU-002".to_string(),
		wechseldatum: wechseldatum(),
	};
	let out = reduce(state, GeraetewechselEvent::ZaehlerstaendeGesendet).expect("should succeed");
	assert_eq!(
		out.state,
		GeraetewechselState::ZaehlerstaendeUebermittelt { melo: melo() }
	);
}

#[test]
fn full_happy_path() {
	let out = reduce(
		GeraetewechselState::Idle,
		GeraetewechselEvent::WechselGemeldet(geraetewechsel()),
	)
	.expect("step 1");
	let out = reduce(out.state, GeraetewechselEvent::NbHatWeitergeleitet).expect("step 2");
	let out = reduce(out.state, GeraetewechselEvent::ZaehlerstaendeGesendet).expect("step 3");
	assert!(matches!(
		out.state,
		GeraetewechselState::ZaehlerstaendeUebermittelt { .. }
	));
}

// --- Timeout ---

#[test]
fn timeout_from_wechsel_gemeldet() {
	let state = GeraetewechselState::WechselGemeldet {
		melo: melo(),
		alte_geraete_nr: "ALT-001".to_string(),
		neue_geraete_nr: "NEU-002".to_string(),
		wechseldatum: wechseldatum(),
	};
	let result = reduce(state, GeraetewechselEvent::FristUeberschritten);
	assert!(matches!(result, Err(ProzessFehler::FristUeberschritten { .. })));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_nb_weitergeleitet() {
	let result = reduce(
		GeraetewechselState::Idle,
		GeraetewechselEvent::NbHatWeitergeleitet,
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn zaehlerstaende_uebermittelt_cannot_receive_event() {
	let state = GeraetewechselState::ZaehlerstaendeUebermittelt { melo: melo() };
	let result = reduce(state, GeraetewechselEvent::ZaehlerstaendeGesendet);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
