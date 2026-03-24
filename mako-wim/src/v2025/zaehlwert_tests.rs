use mako_types::fehler::ProzessFehler;
use mako_types::ids::MeLoId;

use super::zaehlwert::{ZaehlwertEvent, ZaehlwertState, reduce};

fn melo() -> MeLoId {
	MeLoId::new("DE000000000000000000000000000000A").unwrap()
}

// --- Happy path ---

#[test]
fn idle_plus_messwerte_gesendet() {
	let out = reduce(
		ZaehlwertState::Idle,
		ZaehlwertEvent::MesswerteGesendet { melo: melo() },
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		ZaehlwertState::MesswerteGesendet { melo: melo() }
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn messwerte_plus_plausibilisiert() {
	let state = ZaehlwertState::MesswerteGesendet { melo: melo() };
	let out = reduce(state, ZaehlwertEvent::PlausibilisierteWerteEmpfangen)
		.expect("should succeed");
	assert_eq!(out.state, ZaehlwertState::Plausibilisiert { melo: melo() });
}

#[test]
fn full_happy_path() {
	let out = reduce(
		ZaehlwertState::Idle,
		ZaehlwertEvent::MesswerteGesendet { melo: melo() },
	)
	.expect("step 1");
	let out = reduce(out.state, ZaehlwertEvent::PlausibilisierteWerteEmpfangen)
		.expect("step 2");
	assert!(matches!(out.state, ZaehlwertState::Plausibilisiert { .. }));
}

// --- Timeout ---

#[test]
fn timeout_from_messwerte_gesendet() {
	let state = ZaehlwertState::MesswerteGesendet { melo: melo() };
	let result = reduce(state, ZaehlwertEvent::FristUeberschritten);
	assert!(matches!(result, Err(ProzessFehler::FristUeberschritten { .. })));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_plausibilisieren() {
	let result = reduce(
		ZaehlwertState::Idle,
		ZaehlwertEvent::PlausibilisierteWerteEmpfangen,
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn plausibilisiert_cannot_receive_event() {
	let state = ZaehlwertState::Plausibilisiert { melo: melo() };
	let result = reduce(
		state,
		ZaehlwertEvent::MesswerteGesendet { melo: melo() },
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn idle_cannot_timeout() {
	let result = reduce(ZaehlwertState::Idle, ZaehlwertEvent::FristUeberschritten);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
