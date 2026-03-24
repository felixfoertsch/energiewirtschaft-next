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
		gueltig_ab: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
	}
}

// --- Happy path ---

#[test]
fn full_happy_path() {
	// v2022: new clearing rules
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
fn bestaetigt_is_terminal() {
	let state = BilanzkreiszuordnungState::Bestaetigt {
		malo: malo(),
		bilanzkreis: "11XAB-1234-5678".to_string(),
	};
	let result = reduce(state, BilanzkreiszuordnungEvent::Bestaetigt);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}
