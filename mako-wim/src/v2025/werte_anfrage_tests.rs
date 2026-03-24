use chrono::NaiveDate;
use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, OrdersWerteAnfrage};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::NachrichtenPayload;

use super::werte_anfrage::{WerteAnfrageEvent, WerteAnfrageState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn lf_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn msb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}

fn anfrage() -> OrdersWerteAnfrage {
	OrdersWerteAnfrage {
		malo_id: malo(),
		anfragender: lf_id(),
		zeitraum_von: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
		zeitraum_bis: NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
	}
}

// --- Happy path ---

#[test]
fn idle_plus_anfrage_transitions_to_gesendet() {
	let out = reduce(WerteAnfrageState::Idle, WerteAnfrageEvent::AnfrageGesendet(anfrage()))
		.expect("should succeed");
	assert_eq!(
		out.state,
		WerteAnfrageState::AnfrageGesendet {
			malo: malo(),
			anfragender: lf_id(),
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender, lf_id());
	assert_eq!(out.nachrichten[0].empfaenger, msb_id());
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::OrdersWerteAnfrage(_)
	));
}

#[test]
fn gesendet_plus_werte_geliefert() {
	let state = WerteAnfrageState::AnfrageGesendet {
		malo: malo(),
		anfragender: lf_id(),
	};
	let out = reduce(state, WerteAnfrageEvent::WerteGeliefert).expect("should succeed");
	assert_eq!(out.state, WerteAnfrageState::Beantwortet { malo: malo() });
}

#[test]
fn full_happy_path() {
	let out = reduce(WerteAnfrageState::Idle, WerteAnfrageEvent::AnfrageGesendet(anfrage()))
		.expect("step 1");
	let out = reduce(out.state, WerteAnfrageEvent::WerteGeliefert).expect("step 2");
	assert!(matches!(out.state, WerteAnfrageState::Beantwortet { .. }));
}

// --- Rejection ---

#[test]
fn rejection_from_gesendet() {
	let state = WerteAnfrageState::AnfrageGesendet {
		malo: malo(),
		anfragender: lf_id(),
	};
	let out = reduce(
		state,
		WerteAnfrageEvent::Abgelehnt {
			grund: AblehnungsGrund::Sonstiges("Keine Daten vorhanden".to_string()),
		},
	)
	.expect("should succeed");
	assert!(matches!(out.state, WerteAnfrageState::Abgelehnt { .. }));
}

// --- Timeout ---

#[test]
fn timeout_from_gesendet() {
	let state = WerteAnfrageState::AnfrageGesendet {
		malo: malo(),
		anfragender: lf_id(),
	};
	let out = reduce(state, WerteAnfrageEvent::FristUeberschritten).expect("should succeed");
	assert_eq!(
		out.state,
		WerteAnfrageState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::Fristverletzung,
		}
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_werte() {
	let result = reduce(WerteAnfrageState::Idle, WerteAnfrageEvent::WerteGeliefert);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn beantwortet_cannot_receive_event() {
	let state = WerteAnfrageState::Beantwortet { malo: malo() };
	let result = reduce(state, WerteAnfrageEvent::AnfrageGesendet(anfrage()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn idle_cannot_timeout() {
	let result = reduce(WerteAnfrageState::Idle, WerteAnfrageEvent::FristUeberschritten);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
