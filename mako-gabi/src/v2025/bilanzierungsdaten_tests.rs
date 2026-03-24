use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{MsconsAggregierteZeitreihen, ZeitreihenTyp};

use super::bilanzierungsdaten::{BilanzierungsdatenEvent, BilanzierungsdatenState, reduce};

fn allokation() -> MsconsAggregierteZeitreihen {
	MsconsAggregierteZeitreihen {
		bilanzkreis: "21XDE-GAS-BK001".to_string(),
		zeitreihen: vec![],
		typ: ZeitreihenTyp::SlpSynthese,
	}
}

// --- Happy path ---

#[test]
fn idle_plus_allokation_transitions_to_gesendet() {
	let out = reduce(
		BilanzierungsdatenState::Idle,
		BilanzierungsdatenEvent::AllokationsdatenEmpfangen(allokation()),
	)
	.expect("should succeed");
	assert!(matches!(out.state, BilanzierungsdatenState::AllokationsdatenGesendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(
		BilanzierungsdatenState::Idle,
		BilanzierungsdatenEvent::AllokationsdatenEmpfangen(allokation()),
	)
	.expect("step 1");

	let out = reduce(
		out.state,
		BilanzierungsdatenEvent::SummenzeitreihenEmpfangen {
			bilanzkreis: "21XDE-GAS-BK001".to_string(),
		},
	)
	.expect("step 2");
	assert!(matches!(out.state, BilanzierungsdatenState::SummenzeitreihenGesendet { .. }));

	let out = reduce(
		out.state,
		BilanzierungsdatenEvent::AbrechnungEmpfangen {
			bilanzkreis: "21XDE-GAS-BK001".to_string(),
		},
	)
	.expect("step 3");
	assert!(matches!(out.state, BilanzierungsdatenState::AbrechnungEmpfangen { .. }));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_summenzeitreihen() {
	let result = reduce(
		BilanzierungsdatenState::Idle,
		BilanzierungsdatenEvent::SummenzeitreihenEmpfangen {
			bilanzkreis: "21XDE-GAS-BK001".to_string(),
		},
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abrechnung_empfangen_is_terminal() {
	let state = BilanzierungsdatenState::AbrechnungEmpfangen {
		bilanzkreis: "21XDE-GAS-BK001".to_string(),
	};
	let result = reduce(
		state,
		BilanzierungsdatenEvent::AllokationsdatenEmpfangen(allokation()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
