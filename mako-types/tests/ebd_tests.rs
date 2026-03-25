use std::cell::Cell;

use mako_types::ebd::Ebd;

const E_0003_JSON: &str = include_str!("E_0003.json");
const E_0408_JSON: &str = include_str!("E_0408.json");

#[test]
fn deserialize_e_0003() {
	let ebd = Ebd::from_json(E_0003_JSON).expect("failed to deserialize E_0003");
	assert_eq!(ebd.metadata.chapter, "MaBiS");
	assert_eq!(ebd.metadata.ebd_code, "E_0003");
	assert_eq!(ebd.rows.len(), 2);
}

#[test]
fn deserialize_e_0408() {
	let ebd = Ebd::from_json(E_0408_JSON).expect("failed to deserialize E_0408");
	assert_eq!(ebd.metadata.chapter, "GPKE");
	assert_eq!(ebd.metadata.role, "LF");
}

#[test]
fn evaluate_e_0003_all_checks_pass() {
	let ebd = Ebd::from_json(E_0003_JSON).unwrap();
	// Both checks pass (true) -> reaches "Ende" with no error codes
	let codes = ebd.evaluate(&|_| true);
	assert!(codes.is_empty(), "expected no result codes, got: {:?}", codes);
}

#[test]
fn evaluate_e_0003_first_check_fails() {
	let ebd = Ebd::from_json(E_0003_JSON).unwrap();
	// First check fails -> A01
	let codes = ebd.evaluate(&|_| false);
	assert_eq!(codes, vec!["A01"]);
}

#[test]
fn evaluate_e_0003_first_passes_second_fails() {
	let ebd = Ebd::from_json(E_0003_JSON).unwrap();
	let call_count = Cell::new(0);
	let codes = ebd.evaluate(&|_| {
		let n = call_count.get() + 1;
		call_count.set(n);
		// First check (step 1) passes, second check (step 2) fails
		n == 1
	});
	assert_eq!(codes, vec!["A02"]);
}
