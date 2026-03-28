use std::path::Path;

use mako_codec::edifact::dispatch::parse_nachricht;
use mako_testdata::fixtures::utilmd;
use mako_verify::bericht::Urteil;
use mako_verify::referenzdaten::Referenzdaten;
use mako_verify::{verifiziere_batch, verifiziere_nachricht, verifiziere_prozess_schritt};

fn referenzdaten_pfad() -> &'static Path {
	Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("referenzdaten")
		.leak()
}

fn laden() -> Referenzdaten {
	Referenzdaten::laden(referenzdaten_pfad(), "FV2504", "FV2604")
}

// ---------------------------------------------------------------------------
// verifiziere_nachricht
// ---------------------------------------------------------------------------

#[test]
fn verifiziere_nachricht_mit_fixture() {
	let rd = laden();
	let edi = utilmd::anmeldung_lfw_edi();

	let ergebnis = verifiziere_nachricht(&edi, &rd);

	assert_eq!(ergebnis.nachrichtentyp, "UTILMD");
	assert_eq!(ergebnis.pruefidentifikator.as_deref(), Some("44001"));
	assert!(ergebnis.ahb.is_some(), "AHB result should be present");

	let ahb = ergebnis.ahb.unwrap();
	assert!(!ahb.felder.is_empty(), "AHB should have field results");
	// the fixture has core segments present, so overall should not be NichtPruefbar
	assert_ne!(ahb.urteil, Urteil::NichtPruefbar);
}

#[test]
fn verifiziere_nachricht_ohne_rff_z13() {
	let rd = laden();
	// EDIFACT without RFF+Z13 segment — no pruefidentifikator extractable
	let edi = "\
		UNB+UNOC:3+9900000000003:500+9900000000010:500+260325:1200+00001'\
		UNH+1+UTILMD:D:11A:UN:S2.1'\
		BGM+E01+DOK00001'\
		UNT+3+1'\
		UNZ+1+00001'";

	let ergebnis = verifiziere_nachricht(edi, &rd);

	assert_eq!(ergebnis.nachrichtentyp, "UTILMD");
	assert!(
		ergebnis.pruefidentifikator.is_none(),
		"no PI should be extracted without RFF+Z13"
	);
	assert!(ergebnis.ahb.is_none(), "no AHB without PI");
	assert_eq!(ergebnis.gesamt_urteil, Urteil::NichtPruefbar);
}

#[test]
fn verifiziere_nachricht_ungueltig() {
	let rd = laden();
	let ergebnis = verifiziere_nachricht("das ist kein edifact", &rd);

	assert_eq!(ergebnis.gesamt_urteil, Urteil::NichtPruefbar);
	assert!(ergebnis.ahb.is_none());
}

// ---------------------------------------------------------------------------
// verifiziere_prozess_schritt
// ---------------------------------------------------------------------------

#[test]
fn verifiziere_prozess_schritt_mit_zustimmung() {
	let rd = laden();

	let eingabe_edi = utilmd::anmeldung_lfw_edi();
	let eingabe = parse_nachricht(&eingabe_edi).expect("eingabe must parse");

	let ausgabe_edi = utilmd::bestaetigung_edi();
	let ausgabe = parse_nachricht(&ausgabe_edi).expect("ausgabe must parse");

	// E_0003 is the EBD for Anmeldung NB decision
	let ergebnis = verifiziere_prozess_schritt(&eingabe, &[ausgabe], "E_0003", &rd);

	// whether this passes depends on E_0003 reference data existing
	// if reference data is available, it should have an EBD result
	assert!(ergebnis.ebd.is_some(), "EBD result should be present");
}

#[test]
fn verifiziere_prozess_schritt_mit_ablehnung() {
	let rd = laden();

	let eingabe_edi = utilmd::anmeldung_lfw_edi();
	let eingabe = parse_nachricht(&eingabe_edi).expect("eingabe must parse");

	let ausgabe_edi = utilmd::ablehnung_edi();
	let ausgabe = parse_nachricht(&ausgabe_edi).expect("ausgabe must parse");

	let ergebnis = verifiziere_prozess_schritt(&eingabe, &[ausgabe], "E_0003", &rd);

	assert!(ergebnis.ebd.is_some(), "EBD result should be present");
	let ebd = ergebnis.ebd.unwrap();
	// the rejection should produce an antwortcode
	assert!(
		ebd.unser_ergebnis
			.as_ref()
			.and_then(|e| e.antwortcode.as_ref())
			.is_some(),
		"rejection should produce an antwortcode"
	);
}

// ---------------------------------------------------------------------------
// verifiziere_batch
// ---------------------------------------------------------------------------

#[test]
fn verifiziere_batch_mit_temp_verzeichnis() {
	let rd = laden();
	let tmp = std::env::temp_dir().join("mako_verify_batch_test");
	let _ = std::fs::remove_dir_all(&tmp);
	std::fs::create_dir_all(&tmp).unwrap();

	// write two .edi files
	let edi1 = utilmd::anmeldung_lfw_edi();
	let edi2 = utilmd::bestaetigung_edi();
	std::fs::write(tmp.join("anmeldung.edi"), &edi1).unwrap();
	std::fs::write(tmp.join("bestaetigung.edi"), &edi2).unwrap();

	let batch = verifiziere_batch(&tmp, &rd);

	assert_eq!(batch.gesamt, 2, "should process 2 files");
	assert_eq!(
		batch.bestanden + batch.fehlgeschlagen + batch.nicht_pruefbar,
		2,
		"counts should add up"
	);
	assert_eq!(batch.ergebnisse.len(), 2);

	// verify that datei paths are set
	for ergebnis in &batch.ergebnisse {
		assert!(!ergebnis.datei.is_empty(), "datei should be set");
		assert!(ergebnis.datei.ends_with(".edi"), "datei should end with .edi");
	}

	// cleanup
	let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn verifiziere_batch_leeres_verzeichnis() {
	let rd = laden();
	let tmp = std::env::temp_dir().join("mako_verify_batch_empty");
	let _ = std::fs::remove_dir_all(&tmp);
	std::fs::create_dir_all(&tmp).unwrap();

	let batch = verifiziere_batch(&tmp, &rd);

	assert_eq!(batch.gesamt, 0);
	assert_eq!(batch.bestanden, 0);
	assert_eq!(batch.fehlgeschlagen, 0);
	assert_eq!(batch.nicht_pruefbar, 0);

	let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn verifiziere_batch_mit_unterverzeichnissen() {
	let rd = laden();
	let tmp = std::env::temp_dir().join("mako_verify_batch_recursive");
	let _ = std::fs::remove_dir_all(&tmp);
	let sub = tmp.join("sub");
	std::fs::create_dir_all(&sub).unwrap();

	let edi1 = utilmd::anmeldung_lfw_edi();
	let edi2 = utilmd::bestaetigung_edi();
	std::fs::write(tmp.join("root.edi"), &edi1).unwrap();
	std::fs::write(sub.join("nested.edi"), &edi2).unwrap();

	let batch = verifiziere_batch(&tmp, &rd);
	assert_eq!(batch.gesamt, 2, "should find files in subdirectories");

	let _ = std::fs::remove_dir_all(&tmp);
}
