use chrono::NaiveDate;

use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::{test_malo, test_melo, test_mp_id};

// ---------------------------------------------------------------------------
// WiM: Werte-Anfrage (ORDERS / BGM+Z08, DTM+163/164) — LF -> MSB
// ---------------------------------------------------------------------------

pub fn werte_anfrage_edi() -> String {
	let sender = test_mp_id(0); // LF
	let empfaenger = test_mp_id(3); // MSB
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+ORDERS:D:01B:UN:1.4b'\
		 BGM+Z08+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 DTM+163:20260101:102'\
		 DTM+164:20260331:102'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn werte_anfrage_erwartet() -> Nachricht {
	let lf = test_mp_id(0);
	let msb = test_mp_id(3);

	Nachricht {
		absender: lf.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger: msb,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::OrdersWerteAnfrage(OrdersWerteAnfrage {
			malo_id: test_malo(0),
			anfragender: lf,
			zeitraum_von: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
			zeitraum_bis: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
		}),
	}
}

// ---------------------------------------------------------------------------
// UBP: Bestellung (ORDERS / BGM+Z08, RFF+ON) — LF -> MSB
// ---------------------------------------------------------------------------

pub fn bestellung_edi() -> String {
	let sender = test_mp_id(0); // LF
	let empfaenger = test_mp_id(3); // MSB
	let melo = test_melo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+ORDERS:D:01B:UN:1.4b'\
		 BGM+Z08+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{melo}'\
		 RFF+ON:ANG-2026-001'\
		 UNT+8+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		melo = melo.as_str(),
	)
}

pub fn bestellung_erwartet() -> Nachricht {
	let lf = test_mp_id(0);
	let msb = test_mp_id(3);

	Nachricht {
		absender: lf.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger: msb,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::OrdersBestellung(OrdersBestellung {
			melo_id: test_melo(0),
			besteller: lf,
			referenz_angebot: "ANG-2026-001".to_string(),
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
	fn parse_werte_anfrage() {
		let edi = werte_anfrage_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, werte_anfrage_erwartet());
	}

	#[test]
	fn roundtrip_werte_anfrage() {
		let parsed = parse_nachricht(&werte_anfrage_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	#[test]
	fn parse_bestellung() {
		let edi = bestellung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, bestellung_erwartet());
	}

	#[test]
	fn roundtrip_bestellung() {
		let parsed = parse_nachricht(&bestellung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
