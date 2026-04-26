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

// ===========================================================================
// MaBiS Variants
// ===========================================================================

// ---------------------------------------------------------------------------
// 18. Aggregierte Zeitreihen (MSCONS, no PID, RFF+Z06) — NB -> BKV
// ---------------------------------------------------------------------------

pub fn aggregierte_zeitreihen_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(4); // BKV

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+MSCONS:D:04B:UN:2.4c'\
		 BGM+7+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+Z06:11XDE-BKTEST-X'\
		 STS+7++SUM'\
		 QTY+220:100.5:kWh'\
		 DTM+163:20260701000000:203'\
		 QTY+220:95.3:kWh'\
		 DTM+163:20260701001500:203'\
		 UNT+12+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn aggregierte_zeitreihen_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let bkv = test_mp_id(4);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: bkv,
		empfaenger_rolle: MarktRolle::Bilanzkreisverantwortlicher,
		pruef_id: None,
		payload: NachrichtenPayload::MsconsAggregierteZeitreihen(MsconsAggregierteZeitreihen {
			bilanzkreis: "11XDE-BKTEST-X".to_string(),
			zeitreihen: vec![
				Messwert {
					zeitpunkt: NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
						chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
					),
					wert: 100.5,
					einheit: "kWh".to_string(),
					status: MesswertStatus::Gemessen,
				},
				Messwert {
					zeitpunkt: NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
						chrono::NaiveTime::from_hms_opt(0, 15, 0).unwrap(),
					),
					wert: 95.3,
					einheit: "kWh".to_string(),
					status: MesswertStatus::Gemessen,
				},
			],
			typ: ZeitreihenTyp::Summenzeitreihe,
		}),
	}
}

// ---------------------------------------------------------------------------
// 19. Mehr-/Mindermengen (MSCONS, no PID, QTY+46/47) — NB -> LF
// ---------------------------------------------------------------------------

pub fn mehr_mindermengen_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(0); // LF
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+MSCONS:D:04B:UN:2.4c'\
		 BGM+7+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 LOC+172+{malo}'\
		 QTY+46:250.5:kWh'\
		 QTY+47:80.2:kWh'\
		 DTM+163:20260101:102'\
		 DTM+164:20260331:102'\
		 UNT+11+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn mehr_mindermengen_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(0);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::MsconsMehrMindermengen(MsconsMehrMindermengen {
			malo_id: test_malo(0),
			mehrmenge_kwh: 250.5,
			mindermenge_kwh: 80.2,
			abrechnungszeitraum_von: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
			abrechnungszeitraum_bis: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
		}),
	}
}

// ===========================================================================
// MPES MSCONS Variants
// ===========================================================================

// ---------------------------------------------------------------------------
// MPES 2. EinspeiseMesswerte (MSCONS 7, no PID) — MSB -> NB
// ---------------------------------------------------------------------------

pub fn einspeise_messwerte_edi() -> String {
	let sender = test_mp_id(3); // MSB
	let empfaenger = test_mp_id(1); // NB
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+MSCONS:D:04B:UN:2.4c'\
		 BGM+7+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 LOC+172+{malo}'\
		 QTY+220:5.2:kWh'\
		 DTM+163:20260701100000:203'\
		 QTY+220:6.1:kWh'\
		 DTM+163:20260701110000:203'\
		 UNT+11+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn einspeise_messwerte_erwartet() -> Nachricht {
	let msb = test_mp_id(3);
	let nb = test_mp_id(1);

	Nachricht {
		absender: msb,
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::MsconsEinspeiseMesswerte(MsconsEinspeiseMesswerte {
			malo_id: test_malo(0),
			werte: vec![
				Messwert {
					zeitpunkt: NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
						chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
					),
					wert: 5.2,
					einheit: "kWh".to_string(),
					status: MesswertStatus::Gemessen,
				},
				Messwert {
					zeitpunkt: NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
						chrono::NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
					),
					wert: 6.1,
					einheit: "kWh".to_string(),
					status: MesswertStatus::Gemessen,
				},
			],
		}),
	}
}

// ===========================================================================
// Gas MSCONS Variants
// ===========================================================================

// ---------------------------------------------------------------------------
// Gas 5. Nominierung (MSCONS 7, RFF+Z06 bilanzkreis) — BKV -> MGV
// ---------------------------------------------------------------------------

pub fn nominierung_edi() -> String {
	let sender = test_mp_id(4); // BKV
	let empfaenger = test_mp_id(5); // MGV

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+MSCONS:D:04B:UN:2.4c'\
		 BGM+7+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+Z06:11XDE-GASTEST-Y'\
		 QTY+220:500:kWh'\
		 DTM+163:20260701060000:203'\
		 QTY+220:450:kWh'\
		 DTM+163:20260701070000:203'\
		 UNT+11+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn nominierung_erwartet() -> Nachricht {
	let bkv = test_mp_id(4);
	let mgv = test_mp_id(5);

	Nachricht {
		absender: bkv,
		absender_rolle: MarktRolle::Bilanzkreisverantwortlicher,
		empfaenger: mgv,
		empfaenger_rolle: MarktRolle::Marktgebietsverantwortlicher,
		pruef_id: None,
		payload: NachrichtenPayload::Nominierung(Nominierung {
			bilanzkreis: "11XDE-GASTEST-Y".to_string(),
			zeitreihe_soll: vec![
				Messwert {
					zeitpunkt: NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
						chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
					),
					wert: 500.0,
					einheit: "kWh".to_string(),
					status: MesswertStatus::Gemessen,
				},
				Messwert {
					zeitpunkt: NaiveDateTime::new(
						NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
						chrono::NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
					),
					wert: 450.0,
					einheit: "kWh".to_string(),
					status: MesswertStatus::Gemessen,
				},
			],
		}),
	}
}

// ---------------------------------------------------------------------------
// Gas 6. NominierungBestaetigung (MSCONS 7, RFF+Z06 + STS) — MGV -> BKV
// ---------------------------------------------------------------------------

pub fn nominierung_bestaetigung_edi() -> String {
	let sender = test_mp_id(5); // MGV
	let empfaenger = test_mp_id(4); // BKV

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+MSCONS:D:04B:UN:2.4c'\
		 BGM+7+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+Z06:11XDE-GASTEST-Y'\
		 STS+7++Z06'\
		 UNT+8+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn nominierung_bestaetigung_erwartet() -> Nachricht {
	let mgv = test_mp_id(5);
	let bkv = test_mp_id(4);

	Nachricht {
		absender: mgv,
		absender_rolle: MarktRolle::Marktgebietsverantwortlicher,
		empfaenger: bkv,
		empfaenger_rolle: MarktRolle::Bilanzkreisverantwortlicher,
		pruef_id: None,
		payload: NachrichtenPayload::NominierungBestaetigung(NominierungBestaetigung {
			bilanzkreis: "11XDE-GASTEST-Y".to_string(),
			matching_ergebnis: MatchingErgebnis::Bestaetigt,
		}),
	}
}

// ---------------------------------------------------------------------------
// Gas 7. Renominierung (MSCONS 7, RFF+Z06 + RFF+ACE) — BKV -> MGV
// ---------------------------------------------------------------------------

pub fn renominierung_edi() -> String {
	let sender = test_mp_id(4); // BKV
	let empfaenger = test_mp_id(5); // MGV

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+MSCONS:D:04B:UN:2.4c'\
		 BGM+7+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+Z06:11XDE-GASTEST-Y'\
		 RFF+ACE:RENOM'\
		 QTY+220:520:kWh'\
		 DTM+163:20260701060000:203'\
		 UNT+10+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn renominierung_erwartet() -> Nachricht {
	let bkv = test_mp_id(4);
	let mgv = test_mp_id(5);

	Nachricht {
		absender: bkv,
		absender_rolle: MarktRolle::Bilanzkreisverantwortlicher,
		empfaenger: mgv,
		empfaenger_rolle: MarktRolle::Marktgebietsverantwortlicher,
		pruef_id: None,
		payload: NachrichtenPayload::Renominierung(Renominierung {
			bilanzkreis: "11XDE-GASTEST-Y".to_string(),
			zeitreihe_soll: vec![Messwert {
				zeitpunkt: NaiveDateTime::new(
					NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
					chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
				),
				wert: 520.0,
				einheit: "kWh".to_string(),
				status: MesswertStatus::Gemessen,
			}],
		}),
	}
}

// ---------------------------------------------------------------------------
// Gas 8. Brennwertmitteilung (MSCONS 7, MOA+BRENNWERT) — FNB -> LF
// ---------------------------------------------------------------------------

pub fn brennwert_edi() -> String {
	let sender = test_mp_id(6); // FNB
	let empfaenger = test_mp_id(0); // LF

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+MSCONS:D:04B:UN:2.4c'\
		 BGM+7+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+Z13:Netzgebiet-Nord'\
		 MOA+BRENNWERT:11.42'\
		 MOA+ZUSTAND:0.9635'\
		 DTM+163:20260701:102'\
		 DTM+164:20260731:102'\
		 UNT+11+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn brennwert_erwartet() -> Nachricht {
	let fnb = test_mp_id(6);
	let lf = test_mp_id(0);

	Nachricht {
		absender: fnb,
		absender_rolle: MarktRolle::Fernleitungsnetzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::MsconsBrennwert(MsconsBrennwert {
			netzgebiet: "Netzgebiet-Nord".to_string(),
			brennwert_kwh_per_m3: 11.42,
			zustandszahl: 0.9635,
			gueltig_ab: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
			gueltig_bis: NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
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
		let serialized = serialize_nachricht(&parsed).expect("serialize");
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
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 18. Aggregierte Zeitreihen ---

	#[test]
	fn parse_aggregierte_zeitreihen() {
		let edi = aggregierte_zeitreihen_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, aggregierte_zeitreihen_erwartet());
	}

	#[test]
	fn roundtrip_aggregierte_zeitreihen() {
		let parsed = parse_nachricht(&aggregierte_zeitreihen_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- 19. Mehr-/Mindermengen ---

	#[test]
	fn parse_mehr_mindermengen() {
		let edi = mehr_mindermengen_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, mehr_mindermengen_erwartet());
	}

	#[test]
	fn roundtrip_mehr_mindermengen() {
		let parsed = parse_nachricht(&mehr_mindermengen_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- MPES: EinspeiseMesswerte ---

	#[test]
	fn parse_einspeise_messwerte() {
		let edi = einspeise_messwerte_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, einspeise_messwerte_erwartet());
	}

	#[test]
	fn roundtrip_einspeise_messwerte() {
		let parsed = parse_nachricht(&einspeise_messwerte_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- Gas: Nominierung ---

	#[test]
	fn parse_nominierung() {
		let edi = nominierung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, nominierung_erwartet());
	}

	#[test]
	fn roundtrip_nominierung() {
		let parsed = parse_nachricht(&nominierung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- Gas: NominierungBestaetigung ---

	#[test]
	fn parse_nominierung_bestaetigung() {
		let edi = nominierung_bestaetigung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, nominierung_bestaetigung_erwartet());
	}

	#[test]
	fn roundtrip_nominierung_bestaetigung() {
		let parsed = parse_nachricht(&nominierung_bestaetigung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- Gas: Renominierung ---

	#[test]
	fn parse_renominierung() {
		let edi = renominierung_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, renominierung_erwartet());
	}

	#[test]
	fn roundtrip_renominierung() {
		let parsed = parse_nachricht(&renominierung_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}

	// --- Gas: Brennwert ---

	#[test]
	fn parse_brennwert() {
		let edi = brennwert_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, brennwert_erwartet());
	}

	#[test]
	fn roundtrip_brennwert() {
		let parsed = parse_nachricht(&brennwert_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
