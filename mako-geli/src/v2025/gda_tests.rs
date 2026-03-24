use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, Stammdatenfeld, UtilmdGeschaeftsdatenanfrage, UtilmdGeschaeftsdatenantwort,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::NachrichtenPayload;

use super::gda::{GdaEvent, GdaState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn anfragender() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}

fn anfrage() -> UtilmdGeschaeftsdatenanfrage {
	UtilmdGeschaeftsdatenanfrage {
		malo_id: malo(),
		anfragender: anfragender(),
	}
}

fn antwort() -> UtilmdGeschaeftsdatenantwort {
	UtilmdGeschaeftsdatenantwort {
		malo_id: malo(),
		stammdaten: vec![Stammdatenfeld {
			feld: "Brennwert".to_string(),
			alter_wert: None,
			neuer_wert: "11.2 kWh/m³".to_string(),
		}],
	}
}

// --- Happy path ---

#[test]
fn idle_plus_anfrage_transitions_to_gesendet() {
	let out = reduce(GdaState::Idle, GdaEvent::AnfrageEingegangen(anfrage()))
		.expect("should succeed");
	assert!(matches!(out.state, GdaState::AnfrageGesendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(GdaState::Idle, GdaEvent::AnfrageEingegangen(anfrage()))
		.expect("step 1");
	let out = reduce(out.state, GdaEvent::AntwortEmpfangen(antwort()))
		.expect("step 2");
	assert!(matches!(out.state, GdaState::Beantwortet { .. }));
	assert_eq!(out.nachrichten.len(), 1);
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::UtilmdGeschaeftsdatenantwort(_)
	));
}

// --- Rejection ---

#[test]
fn rejection_from_gesendet() {
	let state = GdaState::AnfrageGesendet { malo: malo(), anfragender: anfragender() };
	let out = reduce(state, GdaEvent::Abgelehnt { grund: AblehnungsGrund::MaloUnbekannt })
		.expect("should succeed");
	assert!(matches!(out.state, GdaState::Abgelehnt { .. }));
}

// --- Timeout ---

#[test]
fn timeout_from_gesendet() {
	let state = GdaState::AnfrageGesendet { malo: malo(), anfragender: anfragender() };
	let out = reduce(state, GdaEvent::FristUeberschritten).expect("should succeed");
	assert_eq!(
		out.state,
		GdaState::Abgelehnt { malo: malo(), grund: AblehnungsGrund::Fristverletzung }
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_antwort() {
	let result = reduce(GdaState::Idle, GdaEvent::AntwortEmpfangen(antwort()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn beantwortet_is_terminal() {
	let state = GdaState::Beantwortet { malo: malo(), stammdaten: vec![] };
	let result = reduce(state, GdaEvent::FristUeberschritten);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
