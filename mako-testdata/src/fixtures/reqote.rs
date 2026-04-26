use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::{test_melo, test_mp_id};

// ---------------------------------------------------------------------------
// UBP: Angebotsanfrage (REQOTE / BGM+Z08) — LF -> MSB
// ---------------------------------------------------------------------------

pub fn angebotsanfrage_edi() -> String {
	let sender = test_mp_id(0); // LF
	let empfaenger = test_mp_id(3); // MSB
	let melo = test_melo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+REQOTE:D:01B:UN:1.3c'\
		 BGM+Z08+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{melo}'\
		 IMD+F++:::Intelligentes Messsystem'\
		 UNT+8+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		melo = melo.as_str(),
	)
}

pub fn angebotsanfrage_erwartet() -> Nachricht {
	let lf = test_mp_id(0);
	let msb = test_mp_id(3);

	Nachricht {
		absender: lf.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger: msb,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::ReqoteAngebotsanfrage(ReqoteAngebotsanfrage {
			melo_id: test_melo(0),
			anfragender: lf,
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
	fn parse_angebotsanfrage() {
		let edi = angebotsanfrage_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, angebotsanfrage_erwartet());
	}

	#[test]
	fn roundtrip_angebotsanfrage() {
		let parsed = parse_nachricht(&angebotsanfrage_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
