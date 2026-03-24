use mako_types::fehler::ProzessFehler;

use super::kapazitaet::{KapazitaetEvent, KapazitaetState, reduce};

fn netzgebiet() -> String {
	"NCG".to_string()
}

// --- Happy path ---

#[test]
fn idle_plus_buchung_transitions_to_angefragt() {
	let out = reduce(
		KapazitaetState::Idle,
		KapazitaetEvent::BuchungAngefragt {
			netzgebiet: netzgebiet(),
			kapazitaet_kwh_h: 10000.0,
		},
	)
	.expect("should succeed");
	assert!(matches!(out.state, KapazitaetState::BuchungAngefragt { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(
		KapazitaetState::Idle,
		KapazitaetEvent::BuchungAngefragt {
			netzgebiet: netzgebiet(),
			kapazitaet_kwh_h: 10000.0,
		},
	)
	.expect("step 1");

	let out = reduce(out.state, KapazitaetEvent::Bestaetigt).expect("step 2");
	assert_eq!(
		out.state,
		KapazitaetState::Bestaetigt {
			netzgebiet: netzgebiet(),
			kapazitaet_kwh_h: 10000.0,
		}
	);
}

// --- Rejection ---

#[test]
fn rejection_from_angefragt() {
	let state = KapazitaetState::BuchungAngefragt {
		netzgebiet: netzgebiet(),
		kapazitaet_kwh_h: 10000.0,
	};
	let out = reduce(
		state,
		KapazitaetEvent::Abgelehnt { grund: "Engpass".to_string() },
	)
	.expect("should succeed");
	assert!(matches!(out.state, KapazitaetState::Abgelehnt { .. }));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(KapazitaetState::Idle, KapazitaetEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_is_terminal() {
	let state = KapazitaetState::Bestaetigt {
		netzgebiet: netzgebiet(),
		kapazitaet_kwh_h: 10000.0,
	};
	let result = reduce(state, KapazitaetEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_is_terminal() {
	let state = KapazitaetState::Abgelehnt {
		netzgebiet: netzgebiet(),
		grund: "Engpass".to_string(),
	};
	let result = reduce(state, KapazitaetEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
