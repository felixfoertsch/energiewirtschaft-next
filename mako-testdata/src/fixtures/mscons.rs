use chrono::{NaiveDate, NaiveDateTime};

use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
use mako_types::rolle::MarktRolle;

use crate::ids::{test_malo, test_mp_id};

// ---------------------------------------------------------------------------
// 12. Schlussturnusmesswert (MSCONS / PID 13002) — NB -> LF
// ---------------------------------------------------------------------------

pub fn zaehlerstand_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(3); // LF
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+MSCONS:D:04B:UN:2.4c'\
		 BGM+7+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+Z13:13002'\
		 LOC+172+{malo}'\
		 DTM+163:20260630:102'\
		 QTY+220:12345.6:kWh'\
		 UNT+10+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn zaehlerstand_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(3);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: Some(PruefIdentifikator::Zaehlerstand),
		payload: NachrichtenPayload::MsconsSchlussturnusmesswert(MsconsSchlussturnusmesswert {
			malo_id: test_malo(0),
			zaehlerstand: 12345.6,
			stichtag: NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
			einheit: "kWh".to_string(),
		}),
	}
}

// ---------------------------------------------------------------------------
// 13. Lastgang (MSCONS / PID 13008) — NB -> LF
// ---------------------------------------------------------------------------

pub fn lastgang_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(3); // LF
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+MSCONS:D:04B:UN:2.4c'\
		 BGM+7+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+Z13:13008'\
		 LOC+172+{malo}'\
		 QTY+220:1.5:kWh'\
		 DTM+163:20260701000000:203'\
		 QTY+220:2.3:kWh'\
		 DTM+163:20260701001500:203'\
		 QTY+220:1.8:kWh'\
		 DTM+163:20260701003000:203'\
		 UNT+14+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn lastgang_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(3);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: Some(PruefIdentifikator::Lastgang),
		payload: NachrichtenPayload::MsconsLastgang(MsconsLastgang {
			malo_id: test_malo(0),
			werte: vec![
				Messwert {
					zeitpunkt: NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
						chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
					),
					wert: 1.5,
					einheit: "kWh".to_string(),
					status: MesswertStatus::Gemessen,
				},
				Messwert {
					zeitpunkt: NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
						chrono::NaiveTime::from_hms_opt(0, 15, 0).unwrap(),
					),
					wert: 2.3,
					einheit: "kWh".to_string(),
					status: MesswertStatus::Gemessen,
				},
				Messwert {
					zeitpunkt: NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
						chrono::NaiveTime::from_hms_opt(0, 30, 0).unwrap(),
					),
					wert: 1.8,
					einheit: "kWh".to_string(),
					status: MesswertStatus::Gemessen,
				},
			],
			intervall_minuten: 15,
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

	// --- 12. Zaehlerstand ---

	#[test]
	fn parse_zaehlerstand() {
		let edi = zaehlerstand_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, zaehlerstand_erwartet());
	}

	#[test]
	fn roundtrip_zaehlerstand() {
		let parsed = parse_nachricht(&zaehlerstand_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 13. Lastgang ---

	#[test]
	fn parse_lastgang() {
		let edi = lastgang_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, lastgang_erwartet());
	}

	#[test]
	fn roundtrip_lastgang() {
		let parsed = parse_nachricht(&lastgang_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
