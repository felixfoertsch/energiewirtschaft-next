use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::MsconsBrennwert;

use super::brennwert::{BrennwertEvent, BrennwertState, reduce};

fn brennwert_mitteilung() -> MsconsBrennwert {
	MsconsBrennwert {
		netzgebiet: "NCG".to_string(),
		brennwert_kwh_per_m3: 11.2,
		zustandszahl: 0.9636,
		gueltig_ab: NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
		gueltig_bis: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
	}
}

// --- Happy path ---

#[test]
fn idle_plus_brennwert_transitions_to_mitgeteilt() {
	let out = reduce(
		BrennwertState::Idle,
		BrennwertEvent::BrennwertMitgeteilt(brennwert_mitteilung()),
	)
	.expect("should succeed");
	match &out.state {
		BrennwertState::BrennwertMitgeteilt {
			netzgebiet,
			brennwert_kwh_per_m3,
			zustandszahl,
			..
		} => {
			assert_eq!(netzgebiet, "NCG");
			assert!((brennwert_kwh_per_m3 - 11.2).abs() < f64::EPSILON);
			assert!((zustandszahl - 0.9636).abs() < f64::EPSILON);
		}
		other => panic!("expected BrennwertMitgeteilt, got {other:?}"),
	}
	assert!(out.nachrichten.is_empty());
}

// --- Invalid transitions ---

#[test]
fn mitgeteilt_is_terminal() {
	let state = BrennwertState::BrennwertMitgeteilt {
		netzgebiet: "NCG".to_string(),
		brennwert_kwh_per_m3: 11.2,
		zustandszahl: 0.9636,
		gueltig_ab: NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
		gueltig_bis: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
	};
	let result = reduce(state, BrennwertEvent::BrennwertMitgeteilt(brennwert_mitteilung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
