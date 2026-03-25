use mako_types::ahb::{Ahb, Feldanforderung};
use std::collections::HashSet;

fn load_ahb() -> Ahb {
	let json = include_str!("ahb_44001.json");
	Ahb::from_json(json).expect("failed to deserialize AHB")
}

#[test]
fn deserialize_ahb_44001_line_count() {
	let ahb = load_ahb();
	assert_eq!(ahb.lines.len(), 214);
}

#[test]
fn first_line_is_unh_muss() {
	let ahb = load_ahb();
	let first = &ahb.lines[0];
	assert_eq!(first.segment_code.as_deref(), Some("UNH"));
	assert_eq!(first.ahb_expression, "Muss");
	assert_eq!(first.anforderung(), Feldanforderung::Muss);
}

#[test]
fn muss_felder_returns_nonempty() {
	let ahb = load_ahb();
	let muss = ahb.muss_felder();
	assert!(!muss.is_empty());
	assert_eq!(muss.len(), 52);
}

#[test]
fn segmente_returns_known_segments() {
	let ahb = load_ahb();
	let segmente = ahb.segmente();
	for expected in &["UNH", "BGM", "DTM", "NAD", "UNT"] {
		assert!(
			segmente.contains(&expected.to_string()),
			"missing segment: {expected}"
		);
	}
}

#[test]
fn segment_lines_bgm() {
	let ahb = load_ahb();
	let bgm = ahb.segment_lines("BGM");
	assert_eq!(bgm.len(), 3);
	for line in &bgm {
		assert_eq!(line.segment_code.as_deref(), Some("BGM"));
	}
}

#[test]
fn validate_muss_felder_empty_set() {
	let ahb = load_ahb();
	let vorhanden: HashSet<(String, String)> = HashSet::new();
	let missing = ahb.validate_muss_felder(&vorhanden);
	// All Muss lines in this AHB are segment headers (data_element == None),
	// so they are skipped by validation — result is empty.
	assert!(
		missing.is_empty(),
		"expected no missing fields since all Muss lines are segment headers"
	);
}

#[test]
fn validate_muss_felder_all_present() {
	let ahb = load_ahb();
	// Provide every (segment_code, data_element) pair from the AHB
	let vorhanden: HashSet<(String, String)> = ahb
		.lines
		.iter()
		.filter_map(|l| {
			match (&l.segment_code, &l.data_element) {
				(Some(sc), Some(de)) => Some((sc.clone(), de.clone())),
				_ => None,
			}
		})
		.collect();
	let missing = ahb.validate_muss_felder(&vorhanden);
	assert!(missing.is_empty());
}

#[test]
fn anforderung_x_variant() {
	let ahb = load_ahb();
	// Second line has ahb_expression "X"
	let second = &ahb.lines[1];
	assert_eq!(second.ahb_expression, "X");
	assert_eq!(second.anforderung(), Feldanforderung::X);
}
