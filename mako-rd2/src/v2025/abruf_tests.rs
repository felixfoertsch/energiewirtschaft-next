use chrono::NaiveDateTime;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdAktivierung;
use mako_types::rolle::MarktRolle::*;

use super::abruf::{ABRUF_ROLLENTUPEL, AbrufEvent, AbrufState, reduce};

fn ressource_id() -> String {
	"TR-001".to_string()
}

fn dt(s: &str) -> NaiveDateTime {
	NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
}

fn aktivierung_msg() -> RdAktivierung {
	RdAktivierung {
		ressource_id: ressource_id(),
		sollwert_kw: 25.0,
		start: dt("2025-07-01 08:00:00"),
		ende: dt("2025-07-01 12:00:00"),
	}
}

#[test]
fn happy_path_idle_to_quittiert() {
	let out = reduce(
		AbrufState::Idle,
		AbrufEvent::AbrufGesendet(aktivierung_msg()),
	)
	.expect("step 1");
	assert!(matches!(out.state, AbrufState::AbrufGesendet { .. }));
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender_rolle, AnfordernderNetzbetreiber);
	assert_eq!(out.nachrichten[0].empfaenger_rolle, Anschlussnetzbetreiber);

	let out = reduce(out.state, AbrufEvent::Weitergeleitet).expect("step 2");
	assert!(matches!(out.state, AbrufState::Weitergeleitet { .. }));

	let out = reduce(out.state, AbrufEvent::Quittiert).expect("step 3");
	assert_eq!(
		out.state,
		AbrufState::Quittiert {
			ressource_id: ressource_id(),
		}
	);
}

#[test]
fn rejection_from_weitergeleitet() {
	let state = AbrufState::Weitergeleitet {
		ressource_id: ressource_id(),
	};
	let out = reduce(
		state,
		AbrufEvent::Abgelehnt {
			grund: "Ressource nicht verfügbar".to_string(),
		},
	)
	.expect("should succeed");
	assert!(matches!(out.state, AbrufState::Abgelehnt { .. }));
}

#[test]
fn idle_cannot_receive_quittiert() {
	let result = reduce(AbrufState::Idle, AbrufEvent::Quittiert);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn quittiert_is_terminal() {
	let state = AbrufState::Quittiert {
		ressource_id: ressource_id(),
	};
	let result = reduce(state, AbrufEvent::Weitergeleitet);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn rollentupel_decken_activation_kanon_ab() {
	assert_eq!(
		ABRUF_ROLLENTUPEL[0],
		(AnfordernderNetzbetreiber, Anschlussnetzbetreiber)
	);
	assert_eq!(
		ABRUF_ROLLENTUPEL[1],
		(AnfordernderNetzbetreiber, DataProvider)
	);
	assert_eq!(ABRUF_ROLLENTUPEL[2], (DataProvider, Anschlussnetzbetreiber));
	assert_eq!(ABRUF_ROLLENTUPEL[3], (Anschlussnetzbetreiber, DataProvider));
	assert_eq!(
		ABRUF_ROLLENTUPEL[4],
		(DataProvider, Einsatzverantwortlicher)
	);
	assert_eq!(
		ABRUF_ROLLENTUPEL[5],
		(Einsatzverantwortlicher, BetreiberTechnischeRessource)
	);
	assert_eq!(
		ABRUF_ROLLENTUPEL[6],
		(DataProvider, Bilanzkreisverantwortlicher)
	);
	assert_eq!(
		ABRUF_ROLLENTUPEL[7],
		(Bilanzkreisverantwortlicher, Uebertragungsnetzbetreiber)
	);
	assert_eq!(
		ABRUF_ROLLENTUPEL[8],
		(DataProvider, AnfordernderNetzbetreiber)
	);
}
