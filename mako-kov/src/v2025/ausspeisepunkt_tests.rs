use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::UtilmdAusspeisepunkt;
use mako_types::ids::{MaLoId, MarktpartnerId};

use super::ausspeisepunkt::{AusspeisepunktEvent, AusspeisepunktState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}
fn fnb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}

fn anmeldung() -> UtilmdAusspeisepunkt {
	UtilmdAusspeisepunkt {
		malo_id: malo(),
		nb: nb_id(),
		fnb: fnb_id(),
	}
}

// --- Happy path ---

#[test]
fn idle_plus_anmeldung_transitions_to_gesendet() {
	let out = reduce(
		AusspeisepunktState::Idle,
		AusspeisepunktEvent::AnmeldungEingegangen(anmeldung()),
	)
	.expect("should succeed");
	assert!(matches!(out.state, AusspeisepunktState::AnmeldungGesendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(
		AusspeisepunktState::Idle,
		AusspeisepunktEvent::AnmeldungEingegangen(anmeldung()),
	)
	.expect("step 1");

	let out = reduce(out.state, AusspeisepunktEvent::Bestaetigt).expect("step 2");
	assert_eq!(out.state, AusspeisepunktState::Bestaetigt { malo: malo() });
}

// --- Rejection ---

#[test]
fn rejection_from_gesendet() {
	let state = AusspeisepunktState::AnmeldungGesendet {
		malo: malo(),
		nb: nb_id(),
		fnb: fnb_id(),
	};
	let out = reduce(
		state,
		AusspeisepunktEvent::Abgelehnt { grund: "MaLo unbekannt".to_string() },
	)
	.expect("should succeed");
	assert!(matches!(out.state, AusspeisepunktState::Abgelehnt { .. }));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(AusspeisepunktState::Idle, AusspeisepunktEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_is_terminal() {
	let state = AusspeisepunktState::Bestaetigt { malo: malo() };
	let result = reduce(state, AusspeisepunktEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_is_terminal() {
	let state = AusspeisepunktState::Abgelehnt {
		malo: malo(),
		grund: "test".to_string(),
	};
	let result = reduce(state, AusspeisepunktEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
