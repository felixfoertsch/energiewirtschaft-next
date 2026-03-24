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
		bilanzkreis: "11XAB-1234-5678".to_string(),
		gueltig_ab: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
	}
}

// --- Happy path ---

#[test]
fn full_happy_path() {
	// v2020: SLP/RLM distinction in Bilanzierung
	let out = reduce(
		BilanzkreiszuordnungState::Idle,
		BilanzkreiszuordnungEvent::ZuordnungEmpfangen(zuordnung()),
	)
	.expect("step 1");
	assert!(matches!(
		out.state,
		BilanzkreiszuordnungState::ZuordnungGesendet { .. }
	));

	let out = reduce(out.state, BilanzkreiszuordnungEvent::Bestaetigt)
		.expect("step 2");
	assert!(matches!(
		out.state,
		BilanzkreiszuordnungState::Bestaetigt { .. }
	));
}

// --- Invalid transition ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(
		BilanzkreiszuordnungState::Idle,
		BilanzkreiszuordnungEvent::Bestaetigt,
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn abgelehnt_is_terminal() {
	let state = BilanzkreiszuordnungState::Abgelehnt {
		malo: malo(),
		grund: "BK unbekannt".to_string(),
	};
	let result = reduce(state, BilanzkreiszuordnungEvent::Bestaetigt);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}
