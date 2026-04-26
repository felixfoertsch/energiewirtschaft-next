use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdFahrplan;
use mako_types::rolle::MarktRolle::*;

use super::fahrplan::{FAHRPLAN_ROLLENTUPEL, FahrplanEvent, FahrplanState, reduce};

fn ressource_id() -> String {
	"TR-001".to_string()
}

fn fahrplan_msg() -> RdFahrplan {
	RdFahrplan {
		ressource_id: ressource_id(),
		zeitreihe: vec![],
	}
}

#[test]
fn happy_path_idle_to_bestaetigt() {
	let out = reduce(
		FahrplanState::Idle,
		FahrplanEvent::FahrplanGesendet(fahrplan_msg()),
	)
	.expect("step 1");
	assert!(matches!(out.state, FahrplanState::FahrplanGesendet { .. }));
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender_rolle, Einsatzverantwortlicher);
	assert_eq!(
		out.nachrichten[0].empfaenger_rolle,
		Uebertragungsnetzbetreiber
	);

	let out = reduce(out.state, FahrplanEvent::Weitergeleitet).expect("step 2");
	assert!(matches!(out.state, FahrplanState::Weitergeleitet { .. }));

	let out = reduce(out.state, FahrplanEvent::Bestaetigt).expect("step 3");
	assert_eq!(
		out.state,
		FahrplanState::Bestaetigt {
			ressource_id: ressource_id(),
		}
	);
}

#[test]
fn rejection_from_weitergeleitet() {
	let state = FahrplanState::Weitergeleitet {
		ressource_id: ressource_id(),
	};
	let out = reduce(
		state,
		FahrplanEvent::Abgelehnt {
			grund: "Zeitreihe ungültig".to_string(),
		},
	)
	.expect("should succeed");
	assert!(matches!(out.state, FahrplanState::Abgelehnt { .. }));
}

#[test]
fn idle_cannot_receive_bestaetigt() {
	let result = reduce(FahrplanState::Idle, FahrplanEvent::Bestaetigt);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn bestaetigt_is_terminal() {
	let state = FahrplanState::Bestaetigt {
		ressource_id: ressource_id(),
	};
	let result = reduce(state, FahrplanEvent::Weitergeleitet);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn rollentupel_decken_planungsdaten_kanon_ab() {
	assert_eq!(
		FAHRPLAN_ROLLENTUPEL[0],
		(Einsatzverantwortlicher, Uebertragungsnetzbetreiber)
	);
	assert_eq!(
		FAHRPLAN_ROLLENTUPEL[1],
		(Einsatzverantwortlicher, DataProvider)
	);
	assert_eq!(
		FAHRPLAN_ROLLENTUPEL[2],
		(DataProvider, Anschlussnetzbetreiber)
	);
	assert_eq!(
		FAHRPLAN_ROLLENTUPEL[3],
		(Anschlussnetzbetreiber, Einsatzverantwortlicher)
	);
	assert_eq!(
		FAHRPLAN_ROLLENTUPEL[4],
		(Anschlussnetzbetreiber, DataProvider)
	);
	assert_eq!(
		FAHRPLAN_ROLLENTUPEL[5],
		(Netzbetreiber, Anschlussnetzbetreiber)
	);
	assert_eq!(
		FAHRPLAN_ROLLENTUPEL[6],
		(AnfordernderNetzbetreiber, DataProvider)
	);
	assert_eq!(FAHRPLAN_ROLLENTUPEL[7], (DataProvider, Netzbetreiber));
	assert_eq!(
		FAHRPLAN_ROLLENTUPEL[8],
		(AnfordernderNetzbetreiber, Anschlussnetzbetreiber)
	);
}
