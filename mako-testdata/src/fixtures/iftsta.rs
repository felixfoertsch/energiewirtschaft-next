use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::querschnitt::IftstaStatusmeldung;
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

// ---------------------------------------------------------------------------
// 10. IFTSTA Statusmeldung — NB -> LF
// ---------------------------------------------------------------------------

pub fn statusmeldung_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(0); // LF

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+IFTSTA:D:01B:UN:2.0g'\
		 BGM+23+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+ACE:DOK-REF-001'\
		 STS+7++E15'\
		 FTX+AAO++Nachricht erfolgreich verarbeitet'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn statusmeldung_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(0);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::IftstaStatusmeldung(IftstaStatusmeldung {
			referenz_nachricht: "DOK-REF-001".to_string(),
			status_code: "E15".to_string(),
			beschreibung: "Nachricht erfolgreich verarbeitet".to_string(),
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
	fn parse_iftsta_statusmeldung() {
		let edi = statusmeldung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, statusmeldung_erwartet());
	}

	#[test]
	fn roundtrip_iftsta_statusmeldung() {
		let parsed = parse_nachricht(&statusmeldung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
