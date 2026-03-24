use chrono::NaiveDate;

use mako_gpke::v2025::lfw::{LfwEvent, LfwState, reduce};
use mako_quittung::decorator::mit_quittung;
use mako_quittung::types::{QuittungsErgebnis, QuittungsTyp};
use mako_testdata::ids::test_mp_id;
use mako_types::nachricht::NachrichtenPayload;

/// Extract the UtilmdAnmeldung payload from a Nachricht, panicking if it is not one.
fn extract_anmeldung(
	nachricht: &mako_types::nachricht::Nachricht,
) -> mako_types::gpke_nachrichten::UtilmdAnmeldung {
	match &nachricht.payload {
		NachrichtenPayload::UtilmdAnmeldung(a) => a.clone(),
		other => panic!("expected UtilmdAnmeldung, got {other:?}"),
	}
}

#[test]
fn gpke_lfw_v2025_full_integration_with_quittungen() {
	let lieferbeginn = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();
	let stichtag = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();

	let nachricht = mako_testdata::utilmd::anmeldung(lieferbeginn);
	let anmeldung = extract_anmeldung(&nachricht);
	let event = LfwEvent::AnmeldungEmpfangen(anmeldung);

	let output = mit_quittung(&nachricht, LfwState::Idle, event, stichtag, reduce).unwrap();

	// State transitioned to AnmeldungEingegangen
	assert!(
		matches!(output.state, LfwState::AnmeldungEingegangen { .. }),
		"expected AnmeldungEingegangen, got {:?}",
		output.state,
	);

	// 2 quittungen: CONTRL positiv, APERAK positiv
	assert_eq!(output.quittungen.len(), 2);
	assert_eq!(output.quittungen[0].typ, QuittungsTyp::Contrl);
	assert_eq!(output.quittungen[0].ergebnis, QuittungsErgebnis::Positiv);
	assert_eq!(output.quittungen[1].typ, QuittungsTyp::Aperak);
	assert_eq!(output.quittungen[1].ergebnis, QuittungsErgebnis::Positiv);

	// Quittungen routed back to sender (the LFN's MP-ID)
	let lfn = test_mp_id(1);
	assert_eq!(output.quittungen[0].an, lfn);
	assert_eq!(output.quittungen[1].an, lfn);
}

#[test]
fn gpke_lfw_v2025_aperak_rejects_past_lieferbeginn() {
	let lieferbeginn = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();
	// Stichtag AFTER lieferbeginn -> APERAK should reject
	let stichtag = NaiveDate::from_ymd_opt(2025, 8, 1).unwrap();

	let nachricht = mako_testdata::utilmd::anmeldung(lieferbeginn);
	let anmeldung = extract_anmeldung(&nachricht);
	let event = LfwEvent::AnmeldungEmpfangen(anmeldung);

	let output = mit_quittung(&nachricht, LfwState::Idle, event, stichtag, reduce).unwrap();

	// State unchanged (still Idle)
	assert_eq!(output.state, LfwState::Idle);

	// CONTRL positiv, APERAK negativ
	assert_eq!(output.quittungen.len(), 2);
	assert_eq!(output.quittungen[0].typ, QuittungsTyp::Contrl);
	assert_eq!(output.quittungen[0].ergebnis, QuittungsErgebnis::Positiv);
	assert_eq!(output.quittungen[1].typ, QuittungsTyp::Aperak);
	assert!(
		matches!(output.quittungen[1].ergebnis, QuittungsErgebnis::Negativ(_)),
		"APERAK should be negative for past lieferbeginn",
	);

	// No process messages emitted
	assert!(output.nachrichten.is_empty(), "no process messages on APERAK rejection");
}
