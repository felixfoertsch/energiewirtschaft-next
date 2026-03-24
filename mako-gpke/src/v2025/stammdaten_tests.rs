use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, Stammdatenfeld, UtilmdStammdatenaenderung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};

use super::stammdaten::{StammdatenEvent, StammdatenState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn initiator_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}

fn aenderung() -> UtilmdStammdatenaenderung {
	UtilmdStammdatenaenderung {
		malo_id: malo(),
		initiator: initiator_id(),
		aenderungen: vec![Stammdatenfeld {
			feld: "Jahresverbrauch".to_string(),
			alter_wert: Some("3500".to_string()),
			neuer_wert: "4200".to_string(),
		}],
	}
}

// --- Happy path ---

#[test]
fn idle_plus_aenderung_transitions_to_gesendet() {
	let out = reduce(
		StammdatenState::Idle,
		StammdatenEvent::AenderungEingegangen(aenderung()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		StammdatenState::AenderungGesendet {
			malo: malo(),
			initiator: initiator_id(),
			aenderungen: vec![Stammdatenfeld {
				feld: "Jahresverbrauch".to_string(),
				alter_wert: Some("3500".to_string()),
				neuer_wert: "4200".to_string(),
			}],
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn gesendet_plus_bestaetigt_transitions_to_bestaetigt() {
	let state = StammdatenState::AenderungGesendet {
		malo: malo(),
		initiator: initiator_id(),
		aenderungen: vec![],
	};
	let out = reduce(state, StammdatenEvent::AenderungBestaetigt)
		.expect("should succeed");
	assert_eq!(
		out.state,
		StammdatenState::AenderungBestaetigt { malo: malo() }
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(
		StammdatenState::Idle,
		StammdatenEvent::AenderungEingegangen(aenderung()),
	)
	.expect("step 1");
	assert!(matches!(out.state, StammdatenState::AenderungGesendet { .. }));

	let out = reduce(out.state, StammdatenEvent::AenderungBestaetigt)
		.expect("step 2");
	assert!(matches!(out.state, StammdatenState::AenderungBestaetigt { .. }));
}

// --- Rejection ---

#[test]
fn rejection_from_gesendet() {
	let state = StammdatenState::AenderungGesendet {
		malo: malo(),
		initiator: initiator_id(),
		aenderungen: vec![],
	};
	let out = reduce(
		state,
		StammdatenEvent::AenderungAbgelehnt {
			grund: AblehnungsGrund::Sonstiges("Ungültige Daten".to_string()),
		},
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		StammdatenState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::Sonstiges("Ungültige Daten".to_string()),
		}
	);
}

// --- Timeout ---

#[test]
fn timeout_from_gesendet() {
	let state = StammdatenState::AenderungGesendet {
		malo: malo(),
		initiator: initiator_id(),
		aenderungen: vec![],
	};
	let out = reduce(state, StammdatenEvent::FristUeberschritten)
		.expect("should succeed");
	assert_eq!(
		out.state,
		StammdatenState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::Fristverletzung,
		}
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(StammdatenState::Idle, StammdatenEvent::AenderungBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_cannot_receive_any_event() {
	let state = StammdatenState::AenderungBestaetigt { malo: malo() };
	let result = reduce(state, StammdatenEvent::AenderungBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_cannot_receive_any_event() {
	let state = StammdatenState::Abgelehnt {
		malo: malo(),
		grund: AblehnungsGrund::Fristverletzung,
	};
	let result = reduce(state, StammdatenEvent::AenderungEingegangen(aenderung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn idle_cannot_timeout() {
	let result = reduce(StammdatenState::Idle, StammdatenEvent::FristUeberschritten);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
