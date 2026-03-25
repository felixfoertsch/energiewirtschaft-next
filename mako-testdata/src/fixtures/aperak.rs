use mako_quittung::types::{FehlerCode, Quittung, QuittungsErgebnis, QuittungsTyp};

use crate::ids::test_mp_id;

// ---------------------------------------------------------------------------
// APERAK positiv (BGM response type AP = accepted)
// ---------------------------------------------------------------------------

pub fn aperak_positiv_edi() -> String {
	// The original message was sent by test_mp_id(0).
	// The APERAK quittung goes to test_mp_id(0) — NAD+MR = "an".
	let original_sender = test_mp_id(0);
	let quittung_sender = test_mp_id(1);

	format!(
		"UNB+UNOC:3+{qs}:500+{os}:500+260325:1200+ORIGREF001'\
		 UNH+1+APERAK:D:96A:UN:2.1i'\
		 BGM+11+DOK00001+9+AP'\
		 DTM+137:20260325120000:203'\
		 RFF+ACW:ORIGREF001'\
		 NAD+MS+{qs}::293'\
		 NAD+MR+{os}::293'\
		 UNT+7+1'\
		 UNZ+1+ORIGREF001'",
		qs = quittung_sender.as_str(),
		os = original_sender.as_str(),
	)
}

pub fn aperak_positiv_erwartet() -> Quittung {
	Quittung {
		an: test_mp_id(0),
		typ: QuittungsTyp::Aperak,
		ergebnis: QuittungsErgebnis::Positiv,
	}
}

// ---------------------------------------------------------------------------
// APERAK negativ (BGM response type RE = rejected, with ERC + FTX)
// ---------------------------------------------------------------------------

pub fn aperak_negativ_edi() -> String {
	let original_sender = test_mp_id(0);
	let quittung_sender = test_mp_id(1);

	format!(
		"UNB+UNOC:3+{qs}:500+{os}:500+260325:1200+ORIGREF001'\
		 UNH+1+APERAK:D:96A:UN:2.1i'\
		 BGM+11+DOK00001+9+RE'\
		 DTM+137:20260325120000:203'\
		 RFF+ACW:ORIGREF001'\
		 NAD+MS+{qs}::293'\
		 NAD+MR+{os}::293'\
		 ERC+Z03'\
		 FTX+AAO+++Lieferbeginn in der Vergangenheit'\
		 UNT+9+1'\
		 UNZ+1+ORIGREF001'",
		qs = quittung_sender.as_str(),
		os = original_sender.as_str(),
	)
}

pub fn aperak_negativ_erwartet() -> Quittung {
	Quittung {
		an: test_mp_id(0),
		typ: QuittungsTyp::Aperak,
		ergebnis: QuittungsErgebnis::Negativ(FehlerCode::LieferbeginnInVergangenheit),
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
	fn parse_aperak_positiv() {
		let edi = aperak_positiv_edi();
		let parsed = parse_quittung(&edi).expect("parsing must succeed");
		assert_eq!(parsed, aperak_positiv_erwartet());
	}

	#[test]
	fn roundtrip_aperak_positiv() {
		let edi = aperak_positiv_edi();
		let parsed = parse_quittung(&edi).unwrap();
		assert_eq!(parsed, aperak_positiv_erwartet());
		let serialized = serialize_quittung(&parsed, "ORIGREF001");
		let reparsed = parse_quittung(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	#[test]
	fn parse_aperak_negativ() {
		let edi = aperak_negativ_edi();
		let parsed = parse_quittung(&edi).expect("parsing must succeed");
		assert_eq!(parsed, aperak_negativ_erwartet());
	}

	#[test]
	fn roundtrip_aperak_negativ() {
		let edi = aperak_negativ_edi();
		let parsed = parse_quittung(&edi).unwrap();
		assert_eq!(parsed, aperak_negativ_erwartet());
		let serialized = serialize_quittung(&parsed, "ORIGREF001");
		let reparsed = parse_quittung(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
