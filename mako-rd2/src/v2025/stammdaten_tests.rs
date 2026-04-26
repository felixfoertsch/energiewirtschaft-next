use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{RdStammdaten, RessourceTyp};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::rolle::MarktRolle::*;

use super::stammdaten::{STAMMDATEN_ROLLENTUPEL, StammdatenEvent, StammdatenState, reduce};

fn ressource_id() -> String {
	"TR-001".to_string()
}
fn absender_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}

fn stammdaten_msg() -> RdStammdaten {
	RdStammdaten {
		ressource_id: ressource_id(),
		ressource_typ: RessourceTyp::TechnischeRessource,
		standort_malo: MaLoId::new("51238696788").unwrap(),
		installierte_leistung_kw: 50.0,
	}
}

#[test]
fn happy_path_idle_to_bestaetigt() {
	let out = reduce(
		StammdatenState::Idle,
		StammdatenEvent::StammdatenGesendet(stammdaten_msg()),
	)
	.expect("step 1");
	assert!(matches!(
		out.state,
		StammdatenState::StammdatenGesendet { .. }
	));
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender_rolle, Anschlussnetzbetreiber);
	assert_eq!(out.nachrichten[0].empfaenger_rolle, DataProvider);

	let out = reduce(out.state, StammdatenEvent::Weitergeleitet).expect("step 2");
	assert!(matches!(out.state, StammdatenState::Weitergeleitet { .. }));

	let out = reduce(out.state, StammdatenEvent::Bestaetigt).expect("step 3");
	assert_eq!(
		out.state,
		StammdatenState::Bestaetigt {
			ressource_id: ressource_id(),
		}
	);
}

#[test]
fn rejection_from_weitergeleitet() {
	let state = StammdatenState::Weitergeleitet {
		ressource_id: ressource_id(),
		absender: absender_id(),
	};
	let out = reduce(
		state,
		StammdatenEvent::Abgelehnt {
			grund: "Daten fehlerhaft".to_string(),
		},
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		StammdatenState::Abgelehnt {
			ressource_id: ressource_id(),
			grund: "Daten fehlerhaft".to_string(),
		}
	);
}

#[test]
fn idle_cannot_receive_bestaetigt() {
	let result = reduce(StammdatenState::Idle, StammdatenEvent::Bestaetigt);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn bestaetigt_is_terminal() {
	let state = StammdatenState::Bestaetigt {
		ressource_id: ressource_id(),
	};
	let result = reduce(state, StammdatenEvent::Weitergeleitet);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn rollentupel_decken_stammdaten_kanon_ab() {
	assert_eq!(
		STAMMDATEN_ROLLENTUPEL[0],
		(Einsatzverantwortlicher, DataProvider)
	);
	assert_eq!(
		STAMMDATEN_ROLLENTUPEL[1],
		(DataProvider, Anschlussnetzbetreiber)
	);
	assert_eq!(
		STAMMDATEN_ROLLENTUPEL[2],
		(Anschlussnetzbetreiber, DataProvider)
	);
	assert_eq!(STAMMDATEN_ROLLENTUPEL[3], (DataProvider, Netzbetreiber));
	assert_eq!(STAMMDATEN_ROLLENTUPEL[4], (Netzbetreiber, DataProvider));
	assert_eq!(
		STAMMDATEN_ROLLENTUPEL[5],
		(Netzbetreiber, Anschlussnetzbetreiber)
	);
	assert_eq!(
		STAMMDATEN_ROLLENTUPEL[6],
		(DataProvider, Bilanzkreisverantwortlicher)
	);
	assert_eq!(
		STAMMDATEN_ROLLENTUPEL[7],
		(Bilanzkreisverantwortlicher, Uebertragungsnetzbetreiber)
	);
}
