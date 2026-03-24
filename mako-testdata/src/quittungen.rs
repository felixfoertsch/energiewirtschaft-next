use mako_types::ids::MarktpartnerId;

/// Portable test receipt data — no dependency on mako-quittung.
/// Carries the essential fields needed to construct or assert on receipts.
#[derive(Debug, Clone, PartialEq)]
pub struct TestQuittungData {
	pub an: MarktpartnerId,
	pub ist_contrl: bool,
	pub ist_positiv: bool,
	pub fehler_code: Option<String>,
	pub fehler_text: Option<String>,
}

/// Positive CONTRL receipt (syntax-level acknowledgement).
pub fn contrl_positiv() -> TestQuittungData {
	TestQuittungData {
		an: crate::ids::test_mp_id(1),
		ist_contrl: true,
		ist_positiv: true,
		fehler_code: None,
		fehler_text: None,
	}
}

/// Negative CONTRL receipt (syntax error).
pub fn contrl_negativ() -> TestQuittungData {
	TestQuittungData {
		an: crate::ids::test_mp_id(1),
		ist_contrl: true,
		ist_positiv: false,
		fehler_code: Some("29".to_string()),
		fehler_text: Some("Syntaxfehler".to_string()),
	}
}

/// Positive APERAK receipt (application-level acknowledgement).
pub fn aperak_positiv() -> TestQuittungData {
	TestQuittungData {
		an: crate::ids::test_mp_id(1),
		ist_contrl: false,
		ist_positiv: true,
		fehler_code: None,
		fehler_text: None,
	}
}

/// Negative APERAK receipt (application-level rejection).
pub fn aperak_negativ() -> TestQuittungData {
	TestQuittungData {
		an: crate::ids::test_mp_id(1),
		ist_contrl: false,
		ist_positiv: false,
		fehler_code: Some("Z34".to_string()),
		fehler_text: Some("Nachricht nicht verarbeitbar".to_string()),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn contrl_positiv_flags() {
		let q = contrl_positiv();
		assert!(q.ist_contrl);
		assert!(q.ist_positiv);
		assert!(q.fehler_code.is_none());
	}

	#[test]
	fn contrl_negativ_flags() {
		let q = contrl_negativ();
		assert!(q.ist_contrl);
		assert!(!q.ist_positiv);
		assert!(q.fehler_code.is_some());
	}

	#[test]
	fn aperak_positiv_flags() {
		let q = aperak_positiv();
		assert!(!q.ist_contrl);
		assert!(q.ist_positiv);
	}

	#[test]
	fn aperak_negativ_flags() {
		let q = aperak_negativ();
		assert!(!q.ist_contrl);
		assert!(!q.ist_positiv);
		assert!(q.fehler_text.is_some());
	}
}
