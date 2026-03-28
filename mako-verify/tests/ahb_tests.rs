use std::path::Path;

use mako_verify::referenzdaten::Referenzdaten;

fn referenzdaten_pfad() -> &'static Path {
	Path::new(env!("CARGO_MANIFEST_DIR")).join("referenzdaten").leak()
}

fn laden() -> Referenzdaten {
	Referenzdaten::laden(referenzdaten_pfad(), "FV2504", "FV2604")
}

#[test]
fn lade_ahb_utilmd_44001() {
	let rd = laden();
	let dok = rd
		.ahb("UTILMD", "44001")
		.expect("AHB 44001 should exist");

	assert_eq!(dok.meta.pruefidentifikator, "44001");
	assert_eq!(dok.meta.description, "Anmeldung NN");
	assert_eq!(dok.meta.direction, "LF an NB");
	assert!(!dok.lines.is_empty(), "AHB should have lines");
}

#[test]
fn ahb_cache_liefert_gleichen_inhalt() {
	let rd = laden();

	let erste = rd.ahb("UTILMD", "44001").unwrap();
	let zweite = rd.ahb("UTILMD", "44001").unwrap();

	assert_eq!(erste.meta.pruefidentifikator, zweite.meta.pruefidentifikator);
	assert_eq!(erste.lines.len(), zweite.lines.len());
}

#[test]
fn fehlende_ahb_liefert_none() {
	let rd = laden();
	assert!(rd.ahb("UTILMD", "99999").is_none());
	assert!(rd.ahb("GIBTNICHT", "44001").is_none());
}

#[test]
fn nachrichtentypen_enthaelt_utilmd() {
	let rd = laden();
	let typen = rd.nachrichtentypen();
	assert!(typen.contains(&"UTILMD".to_string()));
	assert!(typen.contains(&"MSCONS".to_string()));
}

#[test]
fn pruefidentifikatoren_fuer_utilmd() {
	let rd = laden();
	let pis = rd.pruefidentifikatoren("UTILMD");
	assert!(pis.contains(&"44001".to_string()));
}
