use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{MsconsAggregierteZeitreihen, ZeitreihenTyp};

use super::bilanzierungsdaten::{
	BilanzierungsdatenEvent, BilanzierungsdatenState, reduce,
};

fn zeitreihen_slp() -> MsconsAggregierteZeitreihen {
	MsconsAggregierteZeitreihen {
		bilanzkreis: "11XAB-1234-5678".to_string(),
		zeitreihen: vec![],
		typ: ZeitreihenTyp::SlpSynthese,
	}
}

// --- Happy path ---

#[test]
fn idle_plus_zeitreihen_transitions_to_gesendet() {
	let out = reduce(
		BilanzierungsdatenState::Idle,
		BilanzierungsdatenEvent::ZeitreihenEmpfangen(zeitreihen_slp()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		BilanzierungsdatenState::ZeitreihenGesendet {
			bilanzkreis: "11XAB-1234-5678".to_string(),
			typ: ZeitreihenTyp::SlpSynthese,
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn gesendet_plus_fahrplan_transitions_to_fahrplan_empfangen() {
	let state = BilanzierungsdatenState::ZeitreihenGesendet {
		bilanzkreis: "11XAB-1234-5678".to_string(),
		typ: ZeitreihenTyp::SlpSynthese,
	};
	let out = reduce(
		state,
		BilanzierungsdatenEvent::FahrplanEmpfangen {
			bilanzkreis: "11XAB-1234-5678".to_string(),
		},
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		BilanzierungsdatenState::FahrplanEmpfangen {
			bilanzkreis: "11XAB-1234-5678".to_string(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(
		BilanzierungsdatenState::Idle,
		BilanzierungsdatenEvent::ZeitreihenEmpfangen(zeitreihen_slp()),
	)
	.expect("step 1");
	assert!(matches!(
		out.state,
		BilanzierungsdatenState::ZeitreihenGesendet { .. }
	));

	let out = reduce(
		out.state,
		BilanzierungsdatenEvent::FahrplanEmpfangen {
			bilanzkreis: "11XAB-1234-5678".to_string(),
		},
	)
	.expect("step 2");
	assert!(matches!(
		out.state,
		BilanzierungsdatenState::FahrplanEmpfangen { .. }
	));
}

#[test]
fn rlm_lastgang_typ_works() {
	let z = MsconsAggregierteZeitreihen {
		bilanzkreis: "11XAB-1234-5678".to_string(),
		zeitreihen: vec![],
		typ: ZeitreihenTyp::RlmLastgang,
	};
	let out = reduce(
		BilanzierungsdatenState::Idle,
		BilanzierungsdatenEvent::ZeitreihenEmpfangen(z),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		BilanzierungsdatenState::ZeitreihenGesendet {
			bilanzkreis: "11XAB-1234-5678".to_string(),
			typ: ZeitreihenTyp::RlmLastgang,
		}
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_fahrplan() {
	let result = reduce(
		BilanzierungsdatenState::Idle,
		BilanzierungsdatenEvent::FahrplanEmpfangen {
			bilanzkreis: "11XAB-1234-5678".to_string(),
		},
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn fahrplan_empfangen_cannot_receive_any_event() {
	let state = BilanzierungsdatenState::FahrplanEmpfangen {
		bilanzkreis: "11XAB-1234-5678".to_string(),
	};
	let result = reduce(
		state,
		BilanzierungsdatenEvent::FahrplanEmpfangen {
			bilanzkreis: "11XAB-1234-5678".to_string(),
		},
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}
