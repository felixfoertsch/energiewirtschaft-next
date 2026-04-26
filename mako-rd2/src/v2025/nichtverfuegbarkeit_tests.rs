use chrono::NaiveDateTime;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdNichtverfuegbarkeit;
use mako_types::rolle::MarktRolle::*;

use super::nichtverfuegbarkeit::{
	NICHTVERFUEGBARKEIT_ROLLENTUPEL, NichtverfuegbarkeitEvent, NichtverfuegbarkeitState, reduce,
};

fn dt(s: &str) -> NaiveDateTime {
	NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
}

fn nv_msg() -> RdNichtverfuegbarkeit {
	RdNichtverfuegbarkeit {
		ressource_id: "TR-001".to_string(),
		von: dt("2025-07-01 00:00:00"),
		bis: dt("2025-07-02 00:00:00"),
		grund: "Wartung".to_string(),
	}
}

#[test]
fn happy_path_idle_to_weitergeleitet() {
	let out = reduce(
		NichtverfuegbarkeitState::Idle,
		NichtverfuegbarkeitEvent::Gemeldet(nv_msg()),
	)
	.expect("step 1");
	assert!(matches!(
		out.state,
		NichtverfuegbarkeitState::Gemeldet { .. }
	));
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender_rolle, Einsatzverantwortlicher);
	assert_eq!(out.nachrichten[0].empfaenger_rolle, DataProvider);

	let out = reduce(out.state, NichtverfuegbarkeitEvent::Weitergeleitet).expect("step 2");
	assert_eq!(
		out.state,
		NichtverfuegbarkeitState::Weitergeleitet {
			ressource_id: "TR-001".to_string(),
		}
	);
}

#[test]
fn idle_cannot_receive_weitergeleitet() {
	let result = reduce(
		NichtverfuegbarkeitState::Idle,
		NichtverfuegbarkeitEvent::Weitergeleitet,
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn weitergeleitet_is_terminal() {
	let state = NichtverfuegbarkeitState::Weitergeleitet {
		ressource_id: "TR-001".to_string(),
	};
	let result = reduce(state, NichtverfuegbarkeitEvent::Weitergeleitet);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn rollentupel_decken_unavailability_kanon_ab() {
	assert_eq!(
		NICHTVERFUEGBARKEIT_ROLLENTUPEL[0],
		(Einsatzverantwortlicher, DataProvider)
	);
	assert_eq!(
		NICHTVERFUEGBARKEIT_ROLLENTUPEL[1],
		(DataProvider, Anschlussnetzbetreiber)
	);
}
