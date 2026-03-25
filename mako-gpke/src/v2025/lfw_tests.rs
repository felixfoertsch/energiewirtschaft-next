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
	let result = reduce(LfwState::Idle, LfwEvent::AnmeldungEmpfangen(anmeldung()));
	let out = result.expect("should succeed");
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
	let result = reduce(state, LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() });
	let out = result.expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::AbmeldungAnLfaGesendet {
			malo: malo(),
			lfn: lfn_id(),
			lfa: lfa_id(),
			nb: nb_id(),
			lieferbeginn: lieferbeginn(),
		}
	);
	assert_eq!(out.nachrichten.len(), 2);
	// First message: Bestaetigung to LFN
	let msg0 = &out.nachrichten[0];
	assert_eq!(msg0.absender, nb_id());
	assert_eq!(msg0.absender_rolle, MarktRolle::Netzbetreiber);
	assert_eq!(msg0.empfaenger, lfn_id());
	assert_eq!(msg0.empfaenger_rolle, MarktRolle::LieferantNeu);
	assert!(matches!(msg0.payload, NachrichtenPayload::UtilmdBestaetigung(_)));
	// Second message: Abmeldung to LFA
	let msg1 = &out.nachrichten[1];
	assert_eq!(msg1.absender, nb_id());
	assert_eq!(msg1.absender_rolle, MarktRolle::Netzbetreiber);
	assert_eq!(msg1.empfaenger, lfa_id());
	assert_eq!(msg1.empfaenger_rolle, MarktRolle::LieferantAlt);
	assert!(matches!(msg1.payload, NachrichtenPayload::UtilmdAbmeldung(_)));
}

#[test]
fn abmeldung_gesendet_plus_lfa_bestaetigt() {
	let state = LfwState::AbmeldungAnLfaGesendet {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let result = reduce(state, LfwEvent::LfaHatBestaetigt);
	let out = result.expect("should succeed");
	match &out.state {
		LfwState::WiderspruchsfristLaeuft { malo: m, lfn, lfa, nb, lieferbeginn: lb, frist_bis } => {
			assert_eq!(m, &malo());
			assert_eq!(lfn, &lfn_id());
			assert_eq!(lfa, &lfa_id());
			assert_eq!(nb, &nb_id());
			assert_eq!(lb, &lieferbeginn());
			// frist_bis should be set (we just check it exists and is after lieferbeginn or at least a date)
			assert!(*frist_bis >= lieferbeginn());
		}
		other => panic!("expected WiderspruchsfristLaeuft, got {other:?}"),
	}
	assert!(out.nachrichten.is_empty());
}

#[test]
fn widerspruchsfrist_abgelaufen() {
	let state = LfwState::WiderspruchsfristLaeuft {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
		frist_bis: NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
	};
	let result = reduce(state, LfwEvent::WiderspruchsfristAbgelaufen);
	let out = result.expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::Zugeordnet {
			malo: malo(),
			lfn: lfn_id(),
			lieferbeginn: lieferbeginn(),
		}
	);
	assert_eq!(out.nachrichten.len(), 2);
	// Zuordnung to LFN
	let msg0 = &out.nachrichten[0];
	assert_eq!(msg0.empfaenger, lfn_id());
	assert_eq!(msg0.empfaenger_rolle, MarktRolle::LieferantNeu);
	assert!(matches!(msg0.payload, NachrichtenPayload::UtilmdZuordnung(_)));
	// Zuordnung to LFA
	let msg1 = &out.nachrichten[1];
	assert_eq!(msg1.empfaenger, lfa_id());
	assert_eq!(msg1.empfaenger_rolle, MarktRolle::LieferantAlt);
	assert!(matches!(msg1.payload, NachrichtenPayload::UtilmdZuordnung(_)));
}

#[test]
fn full_happy_path_idle_to_zugeordnet() {
	// Step 1: Idle -> AnmeldungEingegangen
	let out = reduce(LfwState::Idle, LfwEvent::AnmeldungEmpfangen(anmeldung()))
		.expect("step 1");
	let state = out.state;
	assert!(matches!(state, LfwState::AnmeldungEingegangen { .. }));

	// Step 2: AnmeldungEingegangen -> AbmeldungAnLfaGesendet
	let out = reduce(state, LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() })
		.expect("step 2");
	let state = out.state;
	assert!(matches!(state, LfwState::AbmeldungAnLfaGesendet { .. }));
	assert_eq!(out.nachrichten.len(), 2);

	// Step 3: AbmeldungAnLfaGesendet -> WiderspruchsfristLaeuft
	let out = reduce(state, LfwEvent::LfaHatBestaetigt)
		.expect("step 3");
	let state = out.state;
	assert!(matches!(state, LfwState::WiderspruchsfristLaeuft { .. }));

	// Step 4: WiderspruchsfristLaeuft -> Zugeordnet
	let out = reduce(state, LfwEvent::WiderspruchsfristAbgelaufen)
		.expect("step 4");
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
	let result = reduce(
		state,
		LfwEvent::LfaHatAbgelehnt {
			grund: AblehnungsGrund::KeinVertrag,
		},
	);
	let out = result.expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::KeinVertrag,
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	let msg = &out.nachrichten[0];
	assert_eq!(msg.absender, nb_id());
	assert_eq!(msg.absender_rolle, MarktRolle::Netzbetreiber);
	assert_eq!(msg.empfaenger, lfn_id());
	assert_eq!(msg.empfaenger_rolle, MarktRolle::LieferantNeu);
	assert!(matches!(msg.payload, NachrichtenPayload::UtilmdAblehnung(_)));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(
		LfwState::Idle,
		LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() },
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn zugeordnet_cannot_receive_any_event() {
	let state = LfwState::Zugeordnet {
		malo: malo(),
		lfn: lfn_id(),
		lieferbeginn: lieferbeginn(),
	};
	let result = reduce(state, LfwEvent::AnmeldungEmpfangen(anmeldung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_cannot_receive_any_event() {
	let state = LfwState::Abgelehnt {
		malo: malo(),
		grund: AblehnungsGrund::KeinVertrag,
	};
	let result = reduce(state, LfwEvent::AnmeldungEmpfangen(anmeldung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn eingegangen_cannot_receive_lfa_bestaetigt() {
	let state = LfwState::AnmeldungEingegangen {
		malo: malo(),
		lfn: lfn_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let result = reduce(state, LfwEvent::LfaHatBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

// --- Edge cases ---

#[test]
fn ec6_timeout_from_anmeldung_eingegangen() {
	let state = LfwState::AnmeldungEingegangen {
		malo: malo(),
		lfn: lfn_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let result = reduce(state, LfwEvent::FristUeberschritten);
	let out = result.expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::Fristverletzung,
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	let msg = &out.nachrichten[0];
	assert_eq!(msg.absender, nb_id());
	assert_eq!(msg.empfaenger, lfn_id());
	assert_eq!(msg.empfaenger_rolle, MarktRolle::LieferantNeu);
	assert!(matches!(msg.payload, NachrichtenPayload::UtilmdAblehnung(_)));
}

#[test]
fn ec6_timeout_from_abmeldung_gesendet() {
	let state = LfwState::AbmeldungAnLfaGesendet {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let result = reduce(state, LfwEvent::FristUeberschritten);
	let out = result.expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::Fristverletzung,
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	assert!(matches!(out.nachrichten[0].payload, NachrichtenPayload::UtilmdAblehnung(_)));
}

#[test]
fn ec6_timeout_from_widerspruchsfrist() {
	let state = LfwState::WiderspruchsfristLaeuft {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
		frist_bis: NaiveDate::from_ymd_opt(2025, 6, 15).unwrap(),
	};
	let result = reduce(state, LfwEvent::FristUeberschritten);
	let out = result.expect("should succeed");
	assert_eq!(
		out.state,
		LfwState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::Fristverletzung,
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	assert!(matches!(out.nachrichten[0].payload, NachrichtenPayload::UtilmdAblehnung(_)));
}

#[test]
fn ec4_second_anmeldung_while_process_running_is_invalid() {
	let state = LfwState::AnmeldungEingegangen {
		malo: malo(),
		lfn: lfn_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let result = reduce(state, LfwEvent::AnmeldungEmpfangen(anmeldung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn ec5_grundversorgung_after_rejection_leaves_abgelehnt() {
	let state = LfwState::Abgelehnt {
		malo: malo(),
		grund: AblehnungsGrund::KeinVertrag,
	};
	let result = reduce(state, LfwEvent::WiderspruchsfristAbgelaufen);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
