use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::UtilmdBilanzkreiszuordnung;
use mako_types::ids::MaLoId;

use super::bilanzkreiszuordnung::{
	BilanzkreiszuordnungEvent, BilanzkreiszuordnungState, reduce,
};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}

fn zuordnung() -> UtilmdBilanzkreiszuordnung {
	UtilmdBilanzkreiszuordnung {
		malo_id: malo(),
		bilanzkreis: "21XDE-GAS-BK001".to_string(),
		gueltig_ab: NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
	}
}

// --- Happy path ---

#[test]
fn idle_plus_zuordnung_transitions_to_gesendet() {
	let out = reduce(
		BilanzkreiszuordnungState::Idle,
		BilanzkreiszuordnungEvent::ZuordnungEmpfangen(zuordnung()),
	)
	.expect("should succeed");
	assert!(matches!(out.state, BilanzkreiszuordnungState::ZuordnungGesendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(
		BilanzkreiszuordnungState::Idle,
		BilanzkreiszuordnungEvent::ZuordnungEmpfangen(zuordnung()),
	)
	.expect("step 1");

	let out = reduce(out.state, BilanzkreiszuordnungEvent::Bestaetigt).expect("step 2");
	assert!(matches!(out.state, BilanzkreiszuordnungState::Bestaetigt { .. }));
}

// --- Rejection ---

#[test]
fn rejection_from_gesendet() {
	let state = BilanzkreiszuordnungState::ZuordnungGesendet {
		malo: malo(),
		bilanzkreis: "21XDE-GAS-BK001".to_string(),
		gueltig_ab: NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
	};
	let out = reduce(
		state,
		BilanzkreiszuordnungEvent::Abgelehnt { grund: "BK unbekannt".to_string() },
	)
	.expect("should succeed");
	assert!(matches!(out.state, BilanzkreiszuordnungState::Abgelehnt { .. }));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(BilanzkreiszuordnungState::Idle, BilanzkreiszuordnungEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_is_terminal() {
	let state = BilanzkreiszuordnungState::Bestaetigt {
		malo: malo(),
		bilanzkreis: "21XDE-GAS-BK001".to_string(),
	};
	let result = reduce(state, BilanzkreiszuordnungEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
