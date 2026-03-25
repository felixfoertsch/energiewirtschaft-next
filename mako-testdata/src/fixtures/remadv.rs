use chrono::NaiveDate;

use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

// ---------------------------------------------------------------------------
// Abrechnung: Zahlungsavis (REMADV / BGM+481) — LF -> NB
// ---------------------------------------------------------------------------

pub fn zahlungsavis_edi() -> String {
	let sender = test_mp_id(0); // LF
	let empfaenger = test_mp_id(1); // NB

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+REMADV:D:01B:UN:2.9d'\
		 BGM+481+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+ON:RG-2026-0001'\
		 DTM+171:20260401:102'\
		 MOA+9:12500'\
		 STS+7++Z06'\
		 UNT+10+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn zahlungsavis_erwartet() -> Nachricht {
	let lf = test_mp_id(0);
	let nb = test_mp_id(1);

	Nachricht {
		absender: lf,
		absender_rolle: MarktRolle::Lieferant,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RemadvZahlungsavis(RemadvZahlungsavis {
			referenz_rechnungsnummer: "RG-2026-0001".to_string(),
			zahlungsdatum: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
			betrag_ct: 12500,
			akzeptiert: true,
			ablehnungsgrund: None,
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
	fn parse_zahlungsavis() {
		let edi = zahlungsavis_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, zahlungsavis_erwartet());
	}

	#[test]
	fn roundtrip_zahlungsavis() {
		let parsed = parse_nachricht(&zahlungsavis_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
