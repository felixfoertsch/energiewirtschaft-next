use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, UtilmdAnmeldung};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::NachrichtenPayload;
use mako_types::rolle::MarktRolle;

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
	NaiveDate::from_ymd_opt(2025, 7, 1).unwrap()
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
fn idle_plus_anmeldung_transitions_to_eingegangen() {
	let out = reduce(LfwState::Idle, LfwEvent::AnmeldungEmpfangen(anmeldung()))
		.expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::AnmeldungEingegangen {
			malo: malo(),
			lfn: lfn_id(),
			nb: nb_id(),
			lieferbeginn: lieferbeginn(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn eingegangen_plus_bestaetigt_sends_two_messages() {
	let state = LfwState::AnmeldungEingegangen {
		malo: malo(),
		lfn: lfn_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let out = reduce(state, LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() })
		.expect("should succeed");
	assert_eq!(out.nachrichten.len(), 2);
	assert_eq!(out.nachrichten[0].empfaenger_rolle, MarktRolle::LieferantNeu);
	assert!(matches!(out.nachrichten[0].payload, NachrichtenPayload::UtilmdBestaetigung(_)));
	assert_eq!(out.nachrichten[1].empfaenger_rolle, MarktRolle::LieferantAlt);
	assert!(matches!(out.nachrichten[1].payload, NachrichtenPayload::UtilmdAbmeldung(_)));
}

#[test]
fn full_happy_path_idle_to_zugeordnet() {
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

// --- Rejection ---

#[test]
fn lfa_ablehnung_transitions_to_abgelehnt() {
	let state = LfwState::AbmeldungAnLfaGesendet {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let out = reduce(state, LfwEvent::LfaHatAbgelehnt { grund: AblehnungsGrund::KeinVertrag })
		.expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::Abgelehnt { malo: malo(), grund: AblehnungsGrund::KeinVertrag }
	);
}

// --- Timeout ---

#[test]
fn timeout_from_eingegangen() {
	let state = LfwState::AnmeldungEingegangen {
		malo: malo(),
		lfn: lfn_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let out = reduce(state, LfwEvent::FristUeberschritten).expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::Abgelehnt { malo: malo(), grund: AblehnungsGrund::Fristverletzung }
	);
}

#[test]
fn timeout_from_abmeldung_gesendet() {
	let state = LfwState::AbmeldungAnLfaGesendet {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let out = reduce(state, LfwEvent::FristUeberschritten).expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::Abgelehnt { malo: malo(), grund: AblehnungsGrund::Fristverletzung }
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(LfwState::Idle, LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() });
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

#[test]
fn abgelehnt_is_terminal() {
	let state = LfwState::Abgelehnt { malo: malo(), grund: AblehnungsGrund::KeinVertrag };
	let result = reduce(state, LfwEvent::AnmeldungEmpfangen(anmeldung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
