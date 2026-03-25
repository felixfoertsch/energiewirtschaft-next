use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::{test_melo, test_mp_id};

// ---------------------------------------------------------------------------
// UBP: Bestellantwort (ORDRSP / BGM+Z09) — MSB -> LF
// ---------------------------------------------------------------------------

pub fn bestellantwort_edi() -> String {
	let sender = test_mp_id(3); // MSB
	let empfaenger = test_mp_id(0); // LF
	let melo = test_melo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+ORDRSP:D:01B:UN:1.4a'\
		 BGM+Z09+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{melo}'\
		 STS+7++Z08'\
		 FTX+AAO++Kapazitaet erschoepft'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		melo = melo.as_str(),
	)
}

pub fn bestellantwort_erwartet() -> Nachricht {
	let msb = test_mp_id(3);
	let lf = test_mp_id(0);

	Nachricht {
		absender: msb,
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::OrdrspBestellantwort(OrdrspBestellantwort {
			melo_id: test_melo(0),
			angenommen: false,
			grund: Some("Kapazitaet erschoepft".to_string()),
		}),
	}
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use mako_codec::edifact::dispatch::{parse_nachricht, serialize_nachricht};

	use super::*;

	#[test]
	fn parse_bestellantwort() {
		let edi = bestellantwort_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, bestellantwort_erwartet());
	}

	#[test]
	fn roundtrip_bestellantwort() {
		let parsed = parse_nachricht(&bestellantwort_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
