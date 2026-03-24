use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, Stammdatenfeld, UtilmdStammdatenaenderung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};

use super::stammdaten::{StammdatenEvent, StammdatenState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn initiator() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn aenderung() -> UtilmdStammdatenaenderung {
	UtilmdStammdatenaenderung {
		malo_id: malo(),
		initiator: initiator(),
		aenderungen: vec![Stammdatenfeld {
			feld: "Brennwert".to_string(),
			alter_wert: Some("11.2".to_string()),
			neuer_wert: "11.4".to_string(),
		}],
	}
}

// --- Happy path ---

#[test]
fn idle_plus_aenderung_transitions_to_gesendet() {
	let out = reduce(StammdatenState::Idle, StammdatenEvent::AenderungEingegangen(aenderung()))
		.expect("should succeed");
	assert!(matches!(out.state, StammdatenState::AenderungGesendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(StammdatenState::Idle, StammdatenEvent::AenderungEingegangen(aenderung()))
		.expect("step 1");
	let out = reduce(out.state, StammdatenEvent::AenderungBestaetigt).expect("step 2");
	assert!(matches!(out.state, StammdatenState::AenderungBestaetigt { .. }));
}

// --- Rejection ---

#[test]
fn rejection_from_gesendet() {
	let state = StammdatenState::AenderungGesendet {
		malo: malo(),
		initiator: initiator(),
		aenderungen: vec![],
	};
	let out = reduce(state, StammdatenEvent::AenderungAbgelehnt { grund: AblehnungsGrund::Sonstiges("ungültig".to_string()) })
		.expect("should succeed");
	assert!(matches!(out.state, StammdatenState::Abgelehnt { .. }));
}

// --- Timeout ---

#[test]
fn timeout_from_gesendet() {
	let state = StammdatenState::AenderungGesendet {
		malo: malo(),
		initiator: initiator(),
		aenderungen: vec![],
	};
	let out = reduce(state, StammdatenEvent::FristUeberschritten).expect("should succeed");
	assert_eq!(
		out.state,
		StammdatenState::Abgelehnt { malo: malo(), grund: AblehnungsGrund::Fristverletzung }
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(StammdatenState::Idle, StammdatenEvent::AenderungBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_is_terminal() {
	let state = StammdatenState::AenderungBestaetigt { malo: malo() };
	let result = reduce(state, StammdatenEvent::AenderungBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
