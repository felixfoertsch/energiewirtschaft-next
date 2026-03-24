use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, UtilmdAnmeldung};
use mako_types::ids::{MaLoId, MarktpartnerId};

use super::lfw::{LfwEvent, LfwState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn lfn_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn lfa_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}
fn lieferbeginn() -> NaiveDate {
	NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
}

fn anmeldung() -> UtilmdAnmeldung {
	UtilmdAnmeldung {
		malo_id: malo(),
		lieferant_neu: lfn_id(),
		lieferbeginn: lieferbeginn(),
	}
}

// --- Happy path ---

#[test]
fn full_happy_path_idle_to_zugeordnet() {
	// v2022: same state machine, longer fristen (5 WT Widerspruchsfrist, no 24h switch)
	let out = reduce(LfwState::Idle, LfwEvent::AnmeldungEmpfangen(anmeldung()))
		.expect("step 1");
	assert!(matches!(out.state, LfwState::AnmeldungEingegangen { .. }));

	let out = reduce(out.state, LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() })
		.expect("step 2");
	assert!(matches!(out.state, LfwState::AbmeldungAnLfaGesendet { .. }));
	assert_eq!(out.nachrichten.len(), 2);

	let out = reduce(out.state, LfwEvent::LfaHatBestaetigt).expect("step 3");
	assert!(matches!(out.state, LfwState::WiderspruchsfristLaeuft { .. }));

	let out = reduce(out.state, LfwEvent::WiderspruchsfristAbgelaufen).expect("step 4");
	assert!(matches!(out.state, LfwState::Zugeordnet { .. }));
	assert_eq!(out.nachrichten.len(), 2);
}

// --- Invalid transition ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(
		LfwState::Idle,
		LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() },
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn zugeordnet_is_terminal() {
	let state = LfwState::Zugeordnet {
		malo: malo(),
		lfn: lfn_id(),
		lieferbeginn: lieferbeginn(),
	};
	let result = reduce(state, LfwEvent::AnmeldungEmpfangen(anmeldung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
