use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::{test_melo, test_mp_id};

// ---------------------------------------------------------------------------
// UBP: Angebot (QUOTES / BGM+Z09) — MSB -> LF
// ---------------------------------------------------------------------------

pub fn angebot_edi() -> String {
	let sender = test_mp_id(3); // MSB
	let empfaenger = test_mp_id(0); // LF
	let melo = test_melo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+QUOTES:D:01B:UN:1.3b'\
		 BGM+Z09+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{melo}'\
		 IMD+F++:::Intelligentes Messsystem'\
		 MOA+9:1250'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		melo = melo.as_str(),
	)
}

pub fn angebot_erwartet() -> Nachricht {
	let msb = test_mp_id(3);
	let lf = test_mp_id(0);

	Nachricht {
		absender: msb.clone(),
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::QuotesAngebot(QuotesAngebot {
			melo_id: test_melo(0),
			anbieter: msb,
			preis_ct_pro_monat: 1250.0,
			produkt_beschreibung: "Intelligentes Messsystem".to_string(),
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
	fn parse_angebot() {
		let edi = angebot_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, angebot_erwartet());
	}

	#[test]
	fn roundtrip_angebot() {
		let parsed = parse_nachricht(&angebot_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
