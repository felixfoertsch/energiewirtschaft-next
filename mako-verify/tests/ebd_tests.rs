use std::path::Path;

use mako_verify::ebd::{extrahiere_ausgaenge, vergleiche_ergebnis};
use mako_verify::bericht::Urteil;
use mako_verify::referenzdaten::Referenzdaten;

fn referenzdaten_pfad() -> &'static Path {
	Path::new(env!("CARGO_MANIFEST_DIR")).join("referenzdaten").leak()
}

fn laden() -> Referenzdaten {
	Referenzdaten::laden(referenzdaten_pfad(), "FV2504", "FV2604")
}

// ---- existing referenzdaten tests ----

#[test]
fn lade_ebd_e_0003() {
	let rd = laden();
	let dok = rd.ebd("E_0003").expect("EBD E_0003 should exist");

	assert_eq!(dok.metadata.ebd_code, "E_0003");
	assert_eq!(dok.metadata.chapter, "MaBiS");
	assert_eq!(dok.metadata.role, "ÜNB");
	assert!(!dok.rows.is_empty(), "EBD should have rows");

	// verify first row structure
	let erste_zeile = &dok.rows[0];
	assert_eq!(erste_zeile.step_number, "1");
	assert!(!erste_zeile.sub_rows.is_empty());
}

#[test]
fn ebd_cache_liefert_gleichen_inhalt() {
	let rd = laden();

	let erste = rd.ebd("E_0003").unwrap();
	let zweite = rd.ebd("E_0003").unwrap();

	assert_eq!(erste.metadata.ebd_code, zweite.metadata.ebd_code);
	assert_eq!(erste.rows.len(), zweite.rows.len());
}

#[test]
fn fehlender_ebd_liefert_none() {
	let rd = laden();
	assert!(rd.ebd("E_9999").is_none());
	assert!(rd.ebd("GIBTNICHT").is_none());
}

#[test]
fn ebd_codes_enthaelt_e_0003() {
	let rd = laden();
	let codes = rd.ebd_codes();
	assert!(codes.contains(&"E_0003".to_string()));
}

// ---- EBD outcome extraction tests ----

#[test]
fn extrahiert_ausgaenge() {
	let rd = laden();
	let dok = rd.ebd("E_0003").expect("EBD E_0003 should exist");
	let ausgaenge = extrahiere_ausgaenge(&dok);

	assert!(
		!ausgaenge.is_empty(),
		"E_0003 should have at least one terminal outcome"
	);

	// every outcome must have an ebd_code and step number
	for ausgang in &ausgaenge {
		assert_eq!(ausgang.ebd_code, "E_0003");
		assert!(!ausgang.schritt.is_empty(), "step number must not be empty");
	}

	// E_0003 has result_codes A01 and A02, plus an acceptance terminal
	let codes: Vec<Option<&str>> = ausgaenge.iter().map(|a| a.antwortcode.as_deref()).collect();
	assert!(
		codes.contains(&Some("A01")),
		"A01 should be a terminal outcome"
	);
	assert!(
		codes.contains(&Some("A02")),
		"A02 should be a terminal outcome"
	);
	// the acceptance path (step 2, result=true, subsequent_step_number="Ende") is also terminal
	assert!(
		codes.contains(&None),
		"acceptance path (no result_code) should be a terminal outcome"
	);
}

#[test]
fn akzeptanz_ist_gueltiger_ausgang() {
	let rd = laden();
	// E_0003 has an acceptance path (step 2, true -> "Ende", no result_code)
	let ergebnis = vergleiche_ergebnis("E_0003", None, Some("Akzeptiert"), &rd);

	assert_eq!(ergebnis.urteil, Urteil::Bestanden);
	assert_eq!(ergebnis.ebd_code, "E_0003");
	assert!(!ergebnis.ebd_name.is_empty());
	assert!(!ergebnis.rolle.is_empty());
}

#[test]
fn ablehnung_mit_bekanntem_code() {
	let rd = laden();
	// E_0003 has A01 as a valid rejection code
	let ergebnis = vergleiche_ergebnis("E_0003", Some("A01"), Some("Fristüberschreitung"), &rd);

	assert_eq!(ergebnis.urteil, Urteil::Bestanden);
	assert_eq!(ergebnis.ebd_code, "E_0003");
	assert!(
		ergebnis
			.gueltige_ausgaenge
			.iter()
			.any(|a| a.antwortcode.as_deref() == Some("A01")),
		"A01 should be among valid outcomes"
	);
}

#[test]
fn unbekannter_code_fehlgeschlagen() {
	let rd = laden();
	let ergebnis = vergleiche_ergebnis("E_0003", Some("Z99"), Some("Erfundener Code"), &rd);

	assert_eq!(ergebnis.urteil, Urteil::Fehlgeschlagen);
	assert!(
		ergebnis.details.is_some(),
		"failed comparison should include details"
	);
}

#[test]
fn fehlende_referenzdaten() {
	let rd = laden();
	let ergebnis = vergleiche_ergebnis("E_GIBTNICHT", Some("A01"), None, &rd);

	assert_eq!(ergebnis.urteil, Urteil::NichtPruefbar);
	assert!(ergebnis.gueltige_ausgaenge.is_empty());
	assert!(ergebnis.details.is_some());
}
