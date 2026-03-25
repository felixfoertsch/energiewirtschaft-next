use chrono::NaiveDate;

use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

// ---------------------------------------------------------------------------
// Abrechnung: Rechnung (INVOIC / BGM+380) — NB -> LF
// ---------------------------------------------------------------------------

pub fn rechnung_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(0); // LF

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+INVOIC:D:01B:UN:2.8e'\
		 BGM+380+RG-2026-0001'\
		 DTM+137:20260315:102'\
		 RFF+Z13:31002'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 LIN+1++Netznutzung Q1'\
		 QTY+47:5000:kWh'\
		 MOA+203:12500'\
		 PRI+INV:250'\
		 MOA+86:12500'\
		 UNT+11+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn rechnung_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(0);

	Nachricht {
		absender: nb.clone(),
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf.clone(),
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: Some(PruefIdentifikator::Netznutzungsrechnung),
		payload: NachrichtenPayload::InvoicRechnung(InvoicRechnung {
			rechnungsnummer: "RG-2026-0001".to_string(),
			rechnungsdatum: NaiveDate::from_ymd_opt(2026, 3, 15).unwrap(),
			absender: nb,
			empfaenger: lf,
			positionen: vec![RechnungsPosition {
				bezeichnung: "Netznutzung Q1".to_string(),
				menge: 5000.0,
				einheit: "kWh".to_string(),
				einzelpreis_ct: 250,
				betrag_ct: 12500,
			}],
			gesamtbetrag_ct: 12500,
			rechnungstyp: RechnungsTyp::Netznutzung,
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
	fn parse_rechnung() {
		let edi = rechnung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, rechnung_erwartet());
	}

	#[test]
	fn roundtrip_rechnung() {
		let parsed = parse_nachricht(&rechnung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
