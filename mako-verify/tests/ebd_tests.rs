use std::path::Path;

use mako_verify::referenzdaten::Referenzdaten;

fn referenzdaten_pfad() -> &'static Path {
	Path::new(env!("CARGO_MANIFEST_DIR")).join("referenzdaten").leak()
}

fn laden() -> Referenzdaten {
	Referenzdaten::laden(referenzdaten_pfad(), "FV2504", "FV2604")
}

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
