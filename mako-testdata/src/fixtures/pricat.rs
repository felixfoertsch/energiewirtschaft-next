use chrono::NaiveDate;

use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

// ---------------------------------------------------------------------------
// UBP: Preisblatt (PRICAT / BGM+Z33) — MSB -> LF
// ---------------------------------------------------------------------------

pub fn preisblatt_edi() -> String {
	let sender = test_mp_id(3); // MSB
	let empfaenger = test_mp_id(0); // LF

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+PRICAT:D:01B:UN:2.0e'\
		 BGM+Z33+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 DTM+157:20260701:102'\
		 LIN+1++Grundpreis iMSys'\
		 PRI+INV:850'\
		 MEA+AAE+AAF+ct/Monat'\
		 LIN+2++Arbeitspreis'\
		 PRI+INV:5.2'\
		 MEA+AAE+AAF+ct/kWh'\
		 UNT+13+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn preisblatt_erwartet() -> Nachricht {
	let msb = test_mp_id(3);
	let lf = test_mp_id(0);

	Nachricht {
		absender: msb.clone(),
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::PricatPreisblatt(PricatPreisblatt {
			herausgeber: msb,
			gueltig_ab: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
			positionen: vec![
				PreisPosition {
					bezeichnung: "Grundpreis iMSys".to_string(),
					preis_ct: 850.0,
					einheit: "ct/Monat".to_string(),
				},
				PreisPosition {
					bezeichnung: "Arbeitspreis".to_string(),
					preis_ct: 5.2,
					einheit: "ct/kWh".to_string(),
				},
			],
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
	fn parse_preisblatt() {
		let edi = preisblatt_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, preisblatt_erwartet());
	}

	#[test]
	fn roundtrip_preisblatt() {
		let parsed = parse_nachricht(&preisblatt_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
