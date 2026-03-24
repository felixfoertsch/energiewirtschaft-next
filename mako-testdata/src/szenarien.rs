use chrono::NaiveDate;

use mako_gpke::v2025::lfw::{LfwEvent, LfwState, reduce};
use mako_types::gpke_nachrichten::{AblehnungsGrund, UtilmdAnmeldung};

use crate::ids::{test_malo, test_mp_id};

/// GPKE LFW happy path: Anmeldung -> Bestaetigt -> LfaBestaetigt -> WiderspruchsfristAbgelaufen
pub fn gpke_lfw_happy_path() -> Vec<LfwEvent> {
	vec![
		LfwEvent::AnmeldungEmpfangen(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: test_mp_id(1),
			lieferbeginn: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
		}),
		LfwEvent::AnmeldungBestaetigt {
			lfa: test_mp_id(3),
		},
		LfwEvent::LfaHatBestaetigt,
		LfwEvent::WiderspruchsfristAbgelaufen,
	]
}

/// GPKE LFW rejection path: Anmeldung -> Bestaetigt -> LfaHatAbgelehnt
pub fn gpke_lfw_ablehnung() -> Vec<LfwEvent> {
	vec![
		LfwEvent::AnmeldungEmpfangen(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: test_mp_id(1),
			lieferbeginn: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
		}),
		LfwEvent::AnmeldungBestaetigt {
			lfa: test_mp_id(3),
		},
		LfwEvent::LfaHatAbgelehnt {
			grund: AblehnungsGrund::KeinVertrag,
		},
	]
}

/// Run a sequence of events through the LFW reducer, collecting all intermediate states.
/// Returns Idle + one state per event transition.
pub fn run_scenario(events: Vec<LfwEvent>) -> Vec<LfwState> {
	let mut states = vec![LfwState::Idle];
	let mut current = LfwState::Idle;
	for event in events {
		let output = reduce(current, event).expect("scenario event must not fail");
		current = output.state.clone();
		states.push(output.state);
	}
	states
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn happy_path_ends_in_zugeordnet() {
		let states = run_scenario(gpke_lfw_happy_path());
		let last = states.last().unwrap();
		assert!(matches!(last, LfwState::Zugeordnet { .. }));
	}

	#[test]
	fn ablehnung_ends_in_abgelehnt() {
		let states = run_scenario(gpke_lfw_ablehnung());
		let last = states.last().unwrap();
		assert!(matches!(last, LfwState::Abgelehnt { .. }));
	}

	#[test]
	fn happy_path_has_five_states() {
		let states = run_scenario(gpke_lfw_happy_path());
		// Idle + 4 transitions = 5 states
		assert_eq!(states.len(), 5);
	}
}
