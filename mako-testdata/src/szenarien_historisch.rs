use chrono::NaiveDate;

use mako_types::gpke_nachrichten::UtilmdAnmeldung;

use crate::ids::{test_malo, test_mp_id};

// --- GPKE v2022 scenarios ---

pub fn gpke_lfw_v2022_happy_path() -> Vec<mako_gpke::v2022::lfw::LfwEvent> {
	use mako_gpke::v2022::lfw::LfwEvent;
	vec![
		LfwEvent::AnmeldungEmpfangen(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: test_mp_id(1),
			lieferbeginn: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
		}),
		LfwEvent::AnmeldungBestaetigt { lfa: test_mp_id(3) },
		LfwEvent::LfaHatBestaetigt,
		LfwEvent::WiderspruchsfristAbgelaufen,
	]
}

pub fn run_gpke_v2022_scenario(
	events: Vec<mako_gpke::v2022::lfw::LfwEvent>,
) -> Vec<mako_gpke::v2022::lfw::LfwState> {
	use mako_gpke::v2022::lfw::{LfwState, reduce};
	let mut states = vec![LfwState::Idle];
	let mut current = LfwState::Idle;
	for event in events {
		let output = reduce(current, event).expect("scenario event must not fail");
		current = output.state.clone();
		states.push(output.state);
	}
	states
}

// --- GPKE v2020 scenarios ---

pub fn gpke_lfw_v2020_happy_path() -> Vec<mako_gpke::v2020::lfw::LfwEvent> {
	use mako_gpke::v2020::lfw::LfwEvent;
	vec![
		LfwEvent::AnmeldungEmpfangen(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: test_mp_id(1),
			lieferbeginn: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
		}),
		LfwEvent::AnmeldungBestaetigt { lfa: test_mp_id(3) },
		LfwEvent::LfaHatBestaetigt,
		LfwEvent::WiderspruchsfristAbgelaufen,
	]
}

pub fn run_gpke_v2020_scenario(
	events: Vec<mako_gpke::v2020::lfw::LfwEvent>,
) -> Vec<mako_gpke::v2020::lfw::LfwState> {
	use mako_gpke::v2020::lfw::{LfwState, reduce};
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
	fn gpke_v2022_happy_path_ends_in_zugeordnet() {
		let states = run_gpke_v2022_scenario(gpke_lfw_v2022_happy_path());
		let last = states.last().unwrap();
		assert!(matches!(last, mako_gpke::v2022::lfw::LfwState::Zugeordnet { .. }));
	}

	#[test]
	fn gpke_v2022_happy_path_has_five_states() {
		let states = run_gpke_v2022_scenario(gpke_lfw_v2022_happy_path());
		assert_eq!(states.len(), 5);
	}

	#[test]
	fn gpke_v2020_happy_path_ends_in_zugeordnet() {
		let states = run_gpke_v2020_scenario(gpke_lfw_v2020_happy_path());
		let last = states.last().unwrap();
		assert!(matches!(last, mako_gpke::v2020::lfw::LfwState::Zugeordnet { .. }));
	}

	#[test]
	fn gpke_v2020_happy_path_has_five_states() {
		let states = run_gpke_v2020_scenario(gpke_lfw_v2020_happy_path());
		assert_eq!(states.len(), 5);
	}
}
