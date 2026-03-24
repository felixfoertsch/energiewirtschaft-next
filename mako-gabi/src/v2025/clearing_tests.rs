use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{ClearingEintrag, UtilmdClearingliste};
use mako_types::ids::MaLoId;

use super::clearing::{ClearingEvent, ClearingState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}

fn clearingliste() -> UtilmdClearingliste {
	UtilmdClearingliste {
		eintraege: vec![ClearingEintrag {
			malo_id: malo(),
			feld: "Brennwert".to_string(),
			nb_wert: "11.2 kWh/m³".to_string(),
			lf_wert: Some("11.4 kWh/m³".to_string()),
		}],
	}
}

fn antwortliste() -> UtilmdClearingliste {
	UtilmdClearingliste {
		eintraege: vec![ClearingEintrag {
			malo_id: malo(),
			feld: "Brennwert".to_string(),
			nb_wert: "11.2 kWh/m³".to_string(),
			lf_wert: None,
		}],
	}
}

// --- Happy path ---

#[test]
fn idle_plus_liste_transitions_to_gesendet() {
	let out = reduce(ClearingState::Idle, ClearingEvent::ListeEmpfangen(clearingliste()))
		.expect("should succeed");
	assert!(matches!(out.state, ClearingState::ListeGesendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(ClearingState::Idle, ClearingEvent::ListeEmpfangen(clearingliste()))
		.expect("step 1");
	let out = reduce(out.state, ClearingEvent::AntwortEmpfangen(antwortliste()))
		.expect("step 2");
	assert!(matches!(out.state, ClearingState::AntwortEmpfangen { .. }));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_antwort() {
	let result = reduce(ClearingState::Idle, ClearingEvent::AntwortEmpfangen(antwortliste()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn antwort_empfangen_is_terminal() {
	let state = ClearingState::AntwortEmpfangen {
		eintraege: antwortliste().eintraege,
	};
	let result = reduce(state, ClearingEvent::ListeEmpfangen(clearingliste()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn liste_gesendet_cannot_receive_liste() {
	let state = ClearingState::ListeGesendet {
		eintraege: clearingliste().eintraege,
	};
	let result = reduce(state, ClearingEvent::ListeEmpfangen(clearingliste()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
