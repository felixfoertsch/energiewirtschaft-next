use mako_quittung::types::{FehlerCode, Quittung, QuittungsErgebnis, QuittungsTyp};

use crate::ids::test_mp_id;

// ---------------------------------------------------------------------------
// CONTRL positiv (UCI action code 7 = accepted)
// ---------------------------------------------------------------------------

pub fn contrl_positiv_edi() -> String {
	// The original message was sent by test_mp_id(0).
	// The CONTRL quittung is addressed to test_mp_id(0) (the original sender).
	let original_sender = test_mp_id(0);
	let quittung_sender = test_mp_id(1);

	format!(
		"UNB+UNOC:3+{qs}:500+{os}:500+260325:1200+ORIGREF001'\
		 UNH+1+CONTRL:D:3:UN:2.0b'\
		 UCI+7+ORIGREF001+{os}+{qs}'\
		 UNT+3+1'\
		 UNZ+1+ORIGREF001'",
		qs = quittung_sender.as_str(),
		os = original_sender.as_str(),
	)
}

pub fn contrl_positiv_erwartet() -> Quittung {
	Quittung {
		an: test_mp_id(0),
		typ: QuittungsTyp::Contrl,
		ergebnis: QuittungsErgebnis::Positiv,
	}
}

// ---------------------------------------------------------------------------
// CONTRL negativ (UCI action code 4 = rejected)
// ---------------------------------------------------------------------------

pub fn contrl_negativ_edi() -> String {
	let original_sender = test_mp_id(0);
	let quittung_sender = test_mp_id(1);

	format!(
		"UNB+UNOC:3+{qs}:500+{os}:500+260325:1200+ORIGREF001'\
		 UNH+1+CONTRL:D:3:UN:2.0b'\
		 UCI+4+ORIGREF001+{os}+{qs}'\
		 UCM+1+UTILMD:D:11A:UN:S2.1+4'\
		 UCS+1+29'\
		 UNT+5+1'\
		 UNZ+1+ORIGREF001'",
		qs = quittung_sender.as_str(),
		os = original_sender.as_str(),
	)
}

pub fn contrl_negativ_erwartet() -> Quittung {
	Quittung {
		an: test_mp_id(0),
		typ: QuittungsTyp::Contrl,
		ergebnis: QuittungsErgebnis::Negativ(FehlerCode::AbsenderLeer),
	}
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use mako_codec::edifact::dispatch::{parse_quittung, serialize_quittung};

	use super::*;

	#[test]
	fn parse_contrl_positiv() {
		let edi = contrl_positiv_edi();
		let parsed = parse_quittung(&edi).expect("parsing must succeed");
		assert_eq!(parsed, contrl_positiv_erwartet());
	}

	#[test]
	fn roundtrip_contrl_positiv() {
		let edi = contrl_positiv_edi();
		let parsed = parse_quittung(&edi).unwrap();
		assert_eq!(parsed, contrl_positiv_erwartet());
		let serialized = serialize_quittung(&parsed, "ORIGREF001");
		let reparsed = parse_quittung(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	#[test]
	fn parse_contrl_negativ() {
		let edi = contrl_negativ_edi();
		let parsed = parse_quittung(&edi).expect("parsing must succeed");
		assert_eq!(parsed, contrl_negativ_erwartet());
	}

	#[test]
	fn roundtrip_contrl_negativ() {
		let edi = contrl_negativ_edi();
		let parsed = parse_quittung(&edi).unwrap();
		assert_eq!(parsed, contrl_negativ_erwartet());
		let serialized = serialize_quittung(&parsed, "ORIGREF001");
		let reparsed = parse_quittung(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
