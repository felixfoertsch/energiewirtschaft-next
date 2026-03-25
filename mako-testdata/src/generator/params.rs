//! Parameter structs with fachlich sinnvolle defaults for each EDIFACT generator.

use chrono::NaiveDate;
use mako_types::ids::{MaLoId, MarktpartnerId, MeLoId};

use crate::ids::{test_malo, test_melo, test_mp_id};

// ===========================================================================
// GPKE UTILMD
// ===========================================================================

pub struct AnmeldungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub lieferbeginn: NaiveDate,
}

impl Default for AnmeldungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(0),
			empfaenger: test_mp_id(1),
			malo_id: test_malo(0),
			lieferbeginn: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}
	}
}

pub struct BestaetigungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub lieferbeginn: NaiveDate,
}

impl Default for BestaetigungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(0),
			malo_id: test_malo(0),
			lieferbeginn: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}
	}
}

pub struct AbmeldungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub lieferende: NaiveDate,
}

impl Default for AbmeldungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(2),
			malo_id: test_malo(0),
			lieferende: NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
		}
	}
}

pub struct AblehnungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
}

impl Default for AblehnungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(2),
			empfaenger: test_mp_id(1),
			malo_id: test_malo(0),
		}
	}
}

pub struct ZuordnungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub lieferbeginn: NaiveDate,
}

impl Default for ZuordnungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(0),
			malo_id: test_malo(0),
			lieferbeginn: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}
	}
}

pub struct LieferendeAbmeldungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub lieferende: NaiveDate,
}

impl Default for LieferendeAbmeldungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(3),
			empfaenger: test_mp_id(1),
			malo_id: test_malo(0),
			lieferende: NaiveDate::from_ymd_opt(2026, 9, 30).unwrap(),
		}
	}
}

pub struct LieferendeBestaetigungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub lieferende: NaiveDate,
}

impl Default for LieferendeBestaetigungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(3),
			malo_id: test_malo(0),
			lieferende: NaiveDate::from_ymd_opt(2026, 9, 30).unwrap(),
		}
	}
}

pub struct StammdatenaenderungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub felder: Vec<(String, String)>,
}

impl Default for StammdatenaenderungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(3),
			malo_id: test_malo(0),
			felder: vec![
				("Spannungsebene".to_string(), "Niederspannung".to_string()),
				("Netzgebiet".to_string(), "Berlin".to_string()),
			],
		}
	}
}

pub struct ZuordnungslisteParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub eintraege: Vec<(MaLoId, NaiveDate)>,
}

impl Default for ZuordnungslisteParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(3),
			eintraege: vec![
				(test_malo(0), NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()),
				(test_malo(1), NaiveDate::from_ymd_opt(2026, 8, 1).unwrap()),
			],
		}
	}
}

pub struct GeschaeftsdatenanfrageParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
}

impl Default for GeschaeftsdatenanfrageParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(3),
			empfaenger: test_mp_id(1),
			malo_id: test_malo(0),
		}
	}
}

pub struct GeschaeftsdatenantwortParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub felder: Vec<(String, String)>,
}

impl Default for GeschaeftsdatenantwortParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(3),
			malo_id: test_malo(0),
			felder: vec![
				("Spannungsebene".to_string(), "Niederspannung".to_string()),
				("Netzgebiet".to_string(), "Berlin".to_string()),
			],
		}
	}
}

// ===========================================================================
// GPKE MSCONS
// ===========================================================================

pub struct SchlussturnusmesswertParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub stichtag: NaiveDate,
	pub zaehlerstand: f64,
	pub einheit: String,
}

impl Default for SchlussturnusmesswertParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(3),
			malo_id: test_malo(0),
			stichtag: NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
			zaehlerstand: 12345.6,
			einheit: "kWh".to_string(),
		}
	}
}

pub struct LastgangParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	/// (datetime_str, value, unit) triples
	pub werte: Vec<(String, String, String)>,
}

impl Default for LastgangParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(3),
			malo_id: test_malo(0),
			werte: vec![
				("20260701000000".to_string(), "1.5".to_string(), "kWh".to_string()),
				("20260701001500".to_string(), "2.3".to_string(), "kWh".to_string()),
				("20260701003000".to_string(), "1.8".to_string(), "kWh".to_string()),
			],
		}
	}
}

// ===========================================================================
// WiM
// ===========================================================================

pub struct MsbWechselAnmeldungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub melo_id: MeLoId,
	pub wechseldatum: NaiveDate,
}

impl Default for MsbWechselAnmeldungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(3),
			empfaenger: test_mp_id(1),
			melo_id: test_melo(0),
			wechseldatum: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
		}
	}
}

pub struct GeraetewechselParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub melo_id: MeLoId,
	pub wechseldatum: NaiveDate,
	pub alte_geraete_nr: String,
	pub neue_geraete_nr: String,
}

impl Default for GeraetewechselParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(3),
			empfaenger: test_mp_id(1),
			melo_id: test_melo(0),
			wechseldatum: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
			alte_geraete_nr: "ALT-1234".to_string(),
			neue_geraete_nr: "NEU-5678".to_string(),
		}
	}
}

pub struct WerteAnfrageParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub zeitraum_von: NaiveDate,
	pub zeitraum_bis: NaiveDate,
}

impl Default for WerteAnfrageParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(0),
			empfaenger: test_mp_id(3),
			malo_id: test_malo(0),
			zeitraum_von: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
			zeitraum_bis: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
		}
	}
}

// ===========================================================================
// UBP
// ===========================================================================

pub struct AngebotsanfrageParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub melo_id: MeLoId,
	pub produkt: String,
}

impl Default for AngebotsanfrageParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(0),
			empfaenger: test_mp_id(3),
			melo_id: test_melo(0),
			produkt: "Intelligentes Messsystem".to_string(),
		}
	}
}

pub struct AngebotParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub melo_id: MeLoId,
	pub produkt: String,
	pub preis: String,
}

impl Default for AngebotParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(3),
			empfaenger: test_mp_id(0),
			melo_id: test_melo(0),
			produkt: "Intelligentes Messsystem".to_string(),
			preis: "1250".to_string(),
		}
	}
}

pub struct BestellungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub melo_id: MeLoId,
	pub referenz_angebot: String,
}

impl Default for BestellungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(0),
			empfaenger: test_mp_id(3),
			melo_id: test_melo(0),
			referenz_angebot: "ANG-2026-001".to_string(),
		}
	}
}

pub struct BestellantwortParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub melo_id: MeLoId,
	pub status_code: String,
	pub grund: Option<String>,
}

impl Default for BestellantwortParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(3),
			empfaenger: test_mp_id(0),
			melo_id: test_melo(0),
			status_code: "Z08".to_string(),
			grund: Some("Kapazitaet erschoepft".to_string()),
		}
	}
}

pub struct PreisblattParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub gueltig_ab: NaiveDate,
	pub positionen: Vec<(String, String, String)>, // (bezeichnung, preis, einheit)
}

impl Default for PreisblattParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(3),
			empfaenger: test_mp_id(0),
			gueltig_ab: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
			positionen: vec![
				("Grundpreis iMSys".to_string(), "850".to_string(), "ct/Monat".to_string()),
				("Arbeitspreis".to_string(), "5.2".to_string(), "ct/kWh".to_string()),
			],
		}
	}
}

// ===========================================================================
// MaBiS
// ===========================================================================

pub struct BilanzkreiszuordnungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub bilanzkreis: String,
	pub gueltig_ab: NaiveDate,
}

impl Default for BilanzkreiszuordnungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(0),
			empfaenger: test_mp_id(1),
			malo_id: test_malo(0),
			bilanzkreis: "11XDE-BKTEST-X".to_string(),
			gueltig_ab: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}
	}
}

pub struct AggregierteZeitreihenParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub bilanzkreis: String,
	pub werte: Vec<(String, String, String)>, // (datetime, value, unit)
}

impl Default for AggregierteZeitreihenParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(4),
			bilanzkreis: "11XDE-BKTEST-X".to_string(),
			werte: vec![
				("20260701000000".to_string(), "100.5".to_string(), "kWh".to_string()),
				("20260701001500".to_string(), "95.3".to_string(), "kWh".to_string()),
			],
		}
	}
}

pub struct MehrMindermengenParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub mehrmenge: String,
	pub mindermenge: String,
	pub von: NaiveDate,
	pub bis: NaiveDate,
}

impl Default for MehrMindermengenParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(0),
			malo_id: test_malo(0),
			mehrmenge: "250.5".to_string(),
			mindermenge: "80.2".to_string(),
			von: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
			bis: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
		}
	}
}

pub struct ClearinglisteParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub feld: String,
	pub nb_wert: String,
	pub lf_wert: String,
}

impl Default for ClearinglisteParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(0),
			malo_id: test_malo(0),
			feld: "Spannungsebene".to_string(),
			nb_wert: "Niederspannung".to_string(),
			lf_wert: "Mittelspannung".to_string(),
		}
	}
}

// ===========================================================================
// Abrechnung
// ===========================================================================

pub struct RechnungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub rechnungsnummer: String,
	pub rechnungsdatum: NaiveDate,
	pub positionen: Vec<(String, String, String, String, String)>, // (bezeichnung, menge, einheit, einzelpreis, betrag)
	pub gesamtbetrag: String,
}

impl Default for RechnungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(0),
			rechnungsnummer: "RG-2026-0001".to_string(),
			rechnungsdatum: NaiveDate::from_ymd_opt(2026, 3, 15).unwrap(),
			positionen: vec![(
				"Netznutzung Q1".to_string(),
				"5000".to_string(),
				"kWh".to_string(),
				"250".to_string(),
				"12500".to_string(),
			)],
			gesamtbetrag: "12500".to_string(),
		}
	}
}

pub struct ZahlungsavisParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub referenz_rechnungsnummer: String,
	pub zahlungsdatum: NaiveDate,
	pub betrag: String,
	pub status_code: String,
}

impl Default for ZahlungsavisParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(0),
			empfaenger: test_mp_id(1),
			referenz_rechnungsnummer: "RG-2026-0001".to_string(),
			zahlungsdatum: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
			betrag: "12500".to_string(),
			status_code: "Z06".to_string(),
		}
	}
}

// ===========================================================================
// MPES
// ===========================================================================

pub struct AnmeldungErzeugungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub eeg_anlage: bool,
	pub leistung_kw: String,
}

impl Default for AnmeldungErzeugungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(5),
			empfaenger: test_mp_id(1),
			malo_id: test_malo(0),
			eeg_anlage: true,
			leistung_kw: "9.9".to_string(),
		}
	}
}

pub struct EinspeiseMesswerteParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub werte: Vec<(String, String, String)>, // (datetime, value, unit)
}

impl Default for EinspeiseMesswerteParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(3),
			empfaenger: test_mp_id(1),
			malo_id: test_malo(0),
			werte: vec![
				("20260701100000".to_string(), "5.2".to_string(), "kWh".to_string()),
				("20260701110000".to_string(), "6.1".to_string(), "kWh".to_string()),
			],
		}
	}
}

// ===========================================================================
// 14a
// ===========================================================================

pub struct SteuerbareVerbrauchseinrichtungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub geraetetyp: String,
	pub max_leistung_kw: String,
}

impl Default for SteuerbareVerbrauchseinrichtungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(3),
			malo_id: test_malo(0),
			geraetetyp: "Wallbox".to_string(),
			max_leistung_kw: "11".to_string(),
		}
	}
}

pub struct ClsSteuersignalParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub steuerung_code: String,
	pub zeitpunkt: String,
}

impl Default for ClsSteuersignalParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(3),
			malo_id: test_malo(0),
			steuerung_code: "Z08".to_string(),
			zeitpunkt: "20260701140000".to_string(),
		}
	}
}

// ===========================================================================
// Gas
// ===========================================================================

pub struct NominierungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub bilanzkreis: String,
	pub werte: Vec<(String, String, String)>,
}

impl Default for NominierungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(4),
			empfaenger: test_mp_id(5),
			bilanzkreis: "11XDE-GASTEST-Y".to_string(),
			werte: vec![
				("20260701060000".to_string(), "500".to_string(), "kWh".to_string()),
				("20260701070000".to_string(), "450".to_string(), "kWh".to_string()),
			],
		}
	}
}

pub struct NominierungBestaetigungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub bilanzkreis: String,
	pub status_code: String,
}

impl Default for NominierungBestaetigungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(5),
			empfaenger: test_mp_id(4),
			bilanzkreis: "11XDE-GASTEST-Y".to_string(),
			status_code: "Z06".to_string(),
		}
	}
}

pub struct RenominierungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub bilanzkreis: String,
	pub werte: Vec<(String, String, String)>,
}

impl Default for RenominierungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(4),
			empfaenger: test_mp_id(5),
			bilanzkreis: "11XDE-GASTEST-Y".to_string(),
			werte: vec![
				("20260701060000".to_string(), "520".to_string(), "kWh".to_string()),
			],
		}
	}
}

pub struct BrennwertParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub netzgebiet: String,
	pub brennwert: String,
	pub zustandszahl: String,
	pub gueltig_ab: NaiveDate,
	pub gueltig_bis: NaiveDate,
}

impl Default for BrennwertParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(6),
			empfaenger: test_mp_id(0),
			netzgebiet: "Netzgebiet-Nord".to_string(),
			brennwert: "11.42".to_string(),
			zustandszahl: "0.9635".to_string(),
			gueltig_ab: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
			gueltig_bis: NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
		}
	}
}

pub struct AusspeisepunktParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub malo_id: MaLoId,
	pub nb: MarktpartnerId,
	pub fnb: MarktpartnerId,
}

impl Default for AusspeisepunktParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(6),
			malo_id: test_malo(0),
			nb: test_mp_id(1),
			fnb: test_mp_id(6),
		}
	}
}

// ===========================================================================
// Querschnitt
// ===========================================================================

pub struct IftstaStatusmeldungParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub referenz_nachricht: String,
	pub status_code: String,
	pub beschreibung: String,
}

impl Default for IftstaStatusmeldungParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(0),
			referenz_nachricht: "DOK-REF-001".to_string(),
			status_code: "E15".to_string(),
			beschreibung: "Nachricht erfolgreich verarbeitet".to_string(),
		}
	}
}

pub struct PartinMarktpartnerParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub mp_id: MarktpartnerId,
	pub name: String,
	pub rolle: String,
}

impl Default for PartinMarktpartnerParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(0),
			mp_id: test_mp_id(2),
			name: "Stadtwerke Musterstadt".to_string(),
			rolle: "Netzbetreiber".to_string(),
		}
	}
}

pub struct UtiltsZaehlzeitdefinitionParams {
	pub sender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub formel_id: String,
	pub bezeichnung: String,
	pub zeitreihen_typ: String,
}

impl Default for UtiltsZaehlzeitdefinitionParams {
	fn default() -> Self {
		Self {
			sender: test_mp_id(1),
			empfaenger: test_mp_id(0),
			formel_id: "HT-NT-2026".to_string(),
			bezeichnung: "Hochtarif/Niedertarif Umschaltzeiten".to_string(),
			zeitreihen_typ: "ZRTyp-HT-NT".to_string(),
		}
	}
}
