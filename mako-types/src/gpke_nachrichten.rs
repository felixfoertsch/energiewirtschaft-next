use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::ids::{MaLoId, MeLoId, MarktpartnerId};

/// UTILMD Anmeldung: LFN -> NB (GPKE 1.1.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdAnmeldung {
	pub malo_id: MaLoId,
	pub lieferant_neu: MarktpartnerId,
	pub lieferbeginn: NaiveDate,
}

/// UTILMD Bestaetigung: NB -> LFN (GPKE 1.1.2) or NB -> LFA (GPKE 1.1.6)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdBestaetigung {
	pub malo_id: MaLoId,
	pub bestaetigt_fuer: MarktpartnerId,
	pub lieferbeginn: NaiveDate,
}

/// UTILMD Abmeldung: NB -> LFA (GPKE 1.1.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdAbmeldung {
	pub malo_id: MaLoId,
	pub lieferant_alt: MarktpartnerId,
	pub lieferende: NaiveDate,
}

/// UTILMD Ablehnung: LFA -> NB (GPKE 1.1.4, rejection case)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdAblehnung {
	pub malo_id: MaLoId,
	pub grund: AblehnungsGrund,
}

/// UTILMD Zuordnung: NB -> LFN / NB -> LFA (GPKE 1.1.5 / 1.1.6)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdZuordnung {
	pub malo_id: MaLoId,
	pub zugeordnet_an: MarktpartnerId,
	pub lieferbeginn: NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AblehnungsGrund {
	Fristverletzung,
	MaloUnbekannt,
	KeinVertrag,
	Sonstiges(String),
}

/// UTILMD Lieferende-Abmeldung: LF -> NB (GPKE 1.2.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdLieferendeAbmeldung {
	pub malo_id: MaLoId,
	pub lieferant: MarktpartnerId,
	pub lieferende: NaiveDate,
}

/// UTILMD Lieferende-Bestätigung: NB -> LF (GPKE 1.2.2)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdLieferendeBestaetigung {
	pub malo_id: MaLoId,
	pub lieferende: NaiveDate,
}

/// MSCONS Schlussturnusmesswert: NB -> LF (GPKE 1.2.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsconsSchlussturnusmesswert {
	pub malo_id: MaLoId,
	pub zaehlerstand: f64,
	pub stichtag: NaiveDate,
	pub einheit: String,
}

/// MSCONS Lastgang: time series of energy values (15-min or hourly intervals)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsconsLastgang {
	pub malo_id: MaLoId,
	pub werte: Vec<Messwert>,
	pub intervall_minuten: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Messwert {
	pub zeitpunkt: NaiveDateTime,
	pub wert: f64,
	pub einheit: String,
	pub status: MesswertStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MesswertStatus {
	Gemessen,
	Ersatzwert,
	Vorläufig,
}

/// UTILMD Stammdatenaenderung (generic, used for GPKE 1.3.1-1.3.5)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdStammdatenaenderung {
	pub malo_id: MaLoId,
	pub initiator: MarktpartnerId,
	pub aenderungen: Vec<Stammdatenfeld>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stammdatenfeld {
	pub feld: String,
	pub alter_wert: Option<String>,
	pub neuer_wert: String,
}

/// UTILMD Zuordnungsliste (GPKE 1.4.1-1.4.4)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdZuordnungsliste {
	pub eintraege: Vec<ZuordnungsEintrag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZuordnungsEintrag {
	pub malo_id: MaLoId,
	pub zugeordnet_an: MarktpartnerId,
	pub gueltig_ab: NaiveDate,
}

/// UTILMD Geschäftsdatenanfrage: LF -> NB (GPKE 1.5.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdGeschaeftsdatenanfrage {
	pub malo_id: MaLoId,
	pub anfragender: MarktpartnerId,
}

/// UTILMD Geschäftsdatenantwort: NB -> LF (GPKE 1.5.2)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdGeschaeftsdatenantwort {
	pub malo_id: MaLoId,
	pub stammdaten: Vec<Stammdatenfeld>,
}

// === WiM Message Types ===

/// UTILMD MSB-Wechsel Anmeldung: MSB_neu -> NB (WiM 2.1.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdMsbWechselAnmeldung {
	pub melo_id: MeLoId,
	pub msb_neu: MarktpartnerId,
	pub wechseldatum: NaiveDate,
}

/// UTILMD Gerätewechsel: MSB -> NB (WiM 2.2.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdGeraetewechsel {
	pub melo_id: MeLoId,
	pub alte_geraete_nr: String,
	pub neue_geraete_nr: String,
	pub wechseldatum: NaiveDate,
}

/// ORDERS Werte-Anfrage: LF/ESA -> MSB (WiM 2.4.1/2.4.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrdersWerteAnfrage {
	pub malo_id: MaLoId,
	pub anfragender: MarktpartnerId,
	pub zeitraum_von: NaiveDate,
	pub zeitraum_bis: NaiveDate,
}

// === UBP Message Types ===

/// REQOTE Angebotsanfrage: LF/NB -> MSB (UBP 3.1.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReqoteAngebotsanfrage {
	pub melo_id: MeLoId,
	pub anfragender: MarktpartnerId,
	pub produkt_beschreibung: String,
}

/// QUOTES Angebot: MSB -> LF/NB (UBP 3.1.2)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotesAngebot {
	pub melo_id: MeLoId,
	pub anbieter: MarktpartnerId,
	pub preis_ct_pro_monat: f64,
	pub produkt_beschreibung: String,
}

/// ORDERS Bestellung: LF/NB -> MSB (UBP 3.1.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrdersBestellung {
	pub melo_id: MeLoId,
	pub besteller: MarktpartnerId,
	pub referenz_angebot: String,
}

/// ORDRSP Bestellantwort: MSB -> LF/NB (UBP 3.1.4)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrdrspBestellantwort {
	pub melo_id: MeLoId,
	pub angenommen: bool,
	pub grund: Option<String>,
}

/// PRICAT Preisblatt: MSB -> NB/LF (UBP 3.3.1-3.3.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricatPreisblatt {
	pub herausgeber: MarktpartnerId,
	pub gueltig_ab: NaiveDate,
	pub positionen: Vec<PreisPosition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreisPosition {
	pub bezeichnung: String,
	pub preis_ct: f64,
	pub einheit: String,
}
