use std::path::Path;

use mako_codec::edifact::parser::parse_interchange;
use mako_verify::ahb::validiere_nachricht_ahb;
use mako_verify::bericht::Urteil;
use mako_verify::referenzdaten::Referenzdaten;

fn referenzdaten_pfad() -> &'static Path {
	Path::new(env!("CARGO_MANIFEST_DIR")).join("referenzdaten").leak()
}

fn laden() -> Referenzdaten {
	Referenzdaten::laden(referenzdaten_pfad(), "FV2504", "FV2604")
}

// ---------------------------------------------------------------------------
// Referenzdaten loading tests (existing)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// AHB validation tests
// ---------------------------------------------------------------------------

#[test]
fn bekannte_nachricht_besteht_ahb_validierung() {
	let rd = laden();

	let edi_str = mako_testdata::fixtures::utilmd::anmeldung_lfw_edi();
	let interchange = parse_interchange(&edi_str).expect("parse should succeed");
	let nachricht = &interchange.nachrichten[0];

	let ergebnis = validiere_nachricht_ahb(nachricht, "44001", "UTILMD", &rd);

	// The test fixture is minimal (only core segments: BGM, DTM, NAD, IDE, RFF).
	// The full AHB 44001 requires many more segments (CTA, COM, LOC, SEQ, etc.)
	// that the fixture doesn't include. So we verify:
	// 1. The function produces results
	// 2. Segments present in the fixture are recognized as Bestanden
	// 3. No false negatives for segments we do have

	assert!(
		!ergebnis.felder.is_empty(),
		"should produce field-level results"
	);

	// BGM, DTM, NAD, IDE, RFF must all be Bestanden at the segment level
	let vorhandene_tags = ["BGM", "DTM", "NAD", "IDE", "RFF"];
	for tag in &vorhandene_tags {
		let bestanden = ergebnis.felder.iter().any(|f| {
			f.segment_code.as_deref() == Some(tag)
				&& f.data_element.is_none()
				&& f.urteil == Urteil::Bestanden
		});
		assert!(
			bestanden,
			"Segment {tag} is present in the fixture and should be Bestanden"
		);
	}

	// Fixed-value check: BGM should have E01 detected
	let bgm_e01 = ergebnis.felder.iter().any(|f| {
		f.segment_code.as_deref() == Some("BGM")
			&& f.erwarteter_wert.as_deref() == Some("E01")
			&& f.urteil == Urteil::Bestanden
	});
	assert!(bgm_e01, "BGM fixed value E01 should be detected as Bestanden");
}

#[test]
fn fehlendes_segment_wird_erkannt() {
	let rd = laden();

	// Minimal EDIFACT with BGM removed — DTM and NAD present, but BGM missing
	let edi_str = "\
		UNB+UNOC:3+9900000000003:500+9900000000010:500+260325:1200+00001'\
		UNH+1+UTILMD:D:11A:UN:S2.1'\
		DTM+137:20260325120000+01:303'\
		NAD+MS+9900000000003::293'\
		NAD+MR+9900000000010::293'\
		IDE+24+DE0001234567890000000000000000001'\
		DTM+92:20260701:102'\
		RFF+Z13:44001'\
		UNT+8+1'\
		UNZ+1+00001'";

	let interchange = parse_interchange(edi_str).expect("parse should succeed");
	let nachricht = &interchange.nachrichten[0];

	let ergebnis = validiere_nachricht_ahb(nachricht, "44001", "UTILMD", &rd);

	// BGM is a Muss segment in 44001 — it should be detected as missing
	let bgm_fehler = ergebnis.felder.iter().find(|f| {
		f.segment_code.as_deref() == Some("BGM")
			&& f.data_element.is_none()
			&& f.urteil == Urteil::Fehlgeschlagen
	});

	assert!(
		bgm_fehler.is_some(),
		"missing BGM segment should be detected as Fehlgeschlagen"
	);

	assert_eq!(
		ergebnis.urteil,
		Urteil::Fehlgeschlagen,
		"overall verdict should be Fehlgeschlagen when a Muss segment is missing"
	);
}

#[test]
fn fehlende_referenzdaten_liefert_nicht_pruefbar() {
	let rd = laden();

	let edi_str = "\
		UNB+UNOC:3+9900000000003:500+9900000000010:500+260325:1200+00001'\
		UNH+1+UTILMD:D:11A:UN:S2.1'\
		BGM+E01+DOK00001'\
		UNT+3+1'\
		UNZ+1+00001'";

	let interchange = parse_interchange(edi_str).expect("parse should succeed");
	let nachricht = &interchange.nachrichten[0];

	let ergebnis = validiere_nachricht_ahb(nachricht, "99999", "UTILMD", &rd);

	assert_eq!(ergebnis.urteil, Urteil::NichtPruefbar);
	assert!(ergebnis.zusammenfassung.as_ref().unwrap().contains("nicht gefunden"));
}
