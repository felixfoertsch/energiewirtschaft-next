use std::collections::HashMap;

use mako_codec::edifact::dispatch::parse_nachricht;
use mako_testdata::fixtures::utilmd;
use mako_verify::interop::{extrahiere_schluesselfelder, vergleiche_felder};
use mako_verify::bericht::Urteil;

#[test]
fn extrahiert_schluesselfelder_aus_nachricht() {
	let edi = utilmd::anmeldung_lfw_edi();
	let nachricht = parse_nachricht(&edi).expect("fixture must parse");
	let felder = extrahiere_schluesselfelder(&nachricht);

	assert!(felder.contains_key("absender"), "absender missing");
	assert!(felder.contains_key("empfaenger"), "empfaenger missing");
	assert!(felder.contains_key("absender_rolle"), "absender_rolle missing");
	assert!(felder.contains_key("empfaenger_rolle"), "empfaenger_rolle missing");
	assert!(felder.contains_key("pruefidentifikator"), "pruefidentifikator missing");
	assert!(felder.contains_key("malo_id"), "malo_id missing");
	assert!(felder.contains_key("lieferbeginn"), "lieferbeginn missing");

	assert_eq!(felder["absender_rolle"], "LieferantNeu");
	assert_eq!(felder["empfaenger_rolle"], "Netzbetreiber");
	assert_eq!(felder["pruefidentifikator"], "44001");
}

#[test]
fn vergleich_identische_felder() {
	let mut felder = HashMap::new();
	felder.insert("absender".into(), "9900000000000".into());
	felder.insert("empfaenger".into(), "9900000000001".into());
	felder.insert("pruefidentifikator".into(), "44001".into());

	let ergebnis = vergleiche_felder(&felder, &felder);
	assert_eq!(ergebnis.urteil, Urteil::Bestanden);
	assert!(ergebnis.feldvergleiche.iter().all(|f| f.stimmt_ueberein));
	assert_eq!(ergebnis.feldvergleiche.len(), 3);
}

#[test]
fn vergleich_unterschiedliche_felder() {
	let mut unsere = HashMap::new();
	unsere.insert("absender".into(), "9900000000000".to_string());
	unsere.insert("empfaenger".into(), "9900000000001".to_string());

	let mut drittanbieter = HashMap::new();
	drittanbieter.insert("absender".into(), "9900000000000".to_string());
	drittanbieter.insert("empfaenger".into(), "9900000000099".to_string());

	let ergebnis = vergleiche_felder(&unsere, &drittanbieter);
	assert_eq!(ergebnis.urteil, Urteil::Fehlgeschlagen);

	let empfaenger_vergleich = ergebnis
		.feldvergleiche
		.iter()
		.find(|f| f.feld == "empfaenger")
		.expect("empfaenger comparison must exist");
	assert!(!empfaenger_vergleich.stimmt_ueberein);
	assert_eq!(
		empfaenger_vergleich.drittanbieter_wert.as_deref(),
		Some("9900000000099")
	);
}

#[test]
fn vergleich_teilweise_felder() {
	let mut unsere = HashMap::new();
	unsere.insert("absender".into(), "9900000000000".to_string());
	unsere.insert("malo_id".into(), "51238696788".to_string());
	unsere.insert("lieferbeginn".into(), "2026-07-01".to_string());

	let mut drittanbieter = HashMap::new();
	drittanbieter.insert("absender".into(), "9900000000000".to_string());
	drittanbieter.insert("lieferbeginn".into(), "2026-07-01".to_string());
	drittanbieter.insert("extra_feld".into(), "ignoriert".to_string());

	let ergebnis = vergleiche_felder(&unsere, &drittanbieter);

	// only absender and lieferbeginn overlap — both match
	assert_eq!(ergebnis.urteil, Urteil::Bestanden);
	assert_eq!(ergebnis.feldvergleiche.len(), 2);
	assert!(ergebnis.feldvergleiche.iter().all(|f| f.stimmt_ueberein));
}
