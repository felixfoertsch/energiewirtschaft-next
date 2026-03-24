use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	MatchingErgebnis, Nominierung, NominierungBestaetigung, Renominierung,
};

use super::nominierung::{NominierungEvent, NominierungState, reduce};

fn bilanzkreis() -> String {
	"21XDE-GAS-BK001".to_string()
}

fn nominierung() -> Nominierung {
	Nominierung {
		bilanzkreis: bilanzkreis(),
		zeitreihe_soll: vec![],
	}
}

// --- Happy path: full confirmation ---

#[test]
fn idle_plus_nominierung_transitions_to_nominiert() {
	let out = reduce(
		NominierungState::Idle,
		NominierungEvent::NominierungEingegangen(nominierung()),
	)
	.expect("should succeed");
	assert!(matches!(out.state, NominierungState::Nominiert { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn nominiert_plus_bestaetigung_transitions_to_bestaetigt() {
	let state = NominierungState::Nominiert { bilanzkreis: bilanzkreis() };
	let out = reduce(
		state,
		NominierungEvent::BestaetigungEmpfangen(NominierungBestaetigung {
			bilanzkreis: bilanzkreis(),
			matching_ergebnis: MatchingErgebnis::Bestaetigt,
		}),
	)
	.expect("should succeed");
	assert!(matches!(out.state, NominierungState::Bestaetigt { .. }));
}

// --- Happy path: partial confirmation + renomination ---

#[test]
fn partial_confirmation_then_renominierung() {
	let state = NominierungState::Nominiert { bilanzkreis: bilanzkreis() };
	let out = reduce(
		state,
		NominierungEvent::BestaetigungEmpfangen(NominierungBestaetigung {
			bilanzkreis: bilanzkreis(),
			matching_ergebnis: MatchingErgebnis::TeilweiseBestaetigt {
				bestaetigte_menge_kwh: 5000.0,
			},
		}),
	)
	.expect("step 1");
	assert!(matches!(out.state, NominierungState::TeilweiseBestaetigt { .. }));

	let out = reduce(
		out.state,
		NominierungEvent::RenominierungEingegangen(Renominierung {
			bilanzkreis: bilanzkreis(),
			zeitreihe_soll: vec![],
		}),
	)
	.expect("step 2");
	assert!(matches!(out.state, NominierungState::Renominiert { .. }));

	let out = reduce(out.state, NominierungEvent::RenominierungBestaetigt).expect("step 3");
	assert!(matches!(out.state, NominierungState::Bestaetigt { .. }));
}

// --- Rejection ---

#[test]
fn nominierung_abgelehnt() {
	let state = NominierungState::Nominiert { bilanzkreis: bilanzkreis() };
	let out = reduce(
		state,
		NominierungEvent::BestaetigungEmpfangen(NominierungBestaetigung {
			bilanzkreis: bilanzkreis(),
			matching_ergebnis: MatchingErgebnis::Abgelehnt {
				grund: "Kapazität erschöpft".to_string(),
			},
		}),
	)
	.expect("should succeed");
	assert!(matches!(out.state, NominierungState::Abgelehnt { .. }));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(
		NominierungState::Idle,
		NominierungEvent::BestaetigungEmpfangen(NominierungBestaetigung {
			bilanzkreis: bilanzkreis(),
			matching_ergebnis: MatchingErgebnis::Bestaetigt,
		}),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_is_terminal() {
	let state = NominierungState::Bestaetigt { bilanzkreis: bilanzkreis() };
	let result = reduce(state, NominierungEvent::RenominierungBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_is_terminal() {
	let state = NominierungState::Abgelehnt {
		bilanzkreis: bilanzkreis(),
		grund: "test".to_string(),
	};
	let result = reduce(
		state,
		NominierungEvent::NominierungEingegangen(nominierung()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
