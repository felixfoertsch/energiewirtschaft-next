use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::ids::{MaLoId, MarktpartnerId, MeLoId};

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

// === MaBiS Message Types ===

/// UTILMD Bilanzkreiszuordnung: LF -> NB (MaBiS 4.1.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdBilanzkreiszuordnung {
	pub malo_id: MaLoId,
	pub bilanzkreis: String, // BK-Nummer
	pub gueltig_ab: NaiveDate,
}

/// MSCONS Aggregierte Zeitreihen: NB -> BKV (MaBiS 4.2.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsconsAggregierteZeitreihen {
	pub bilanzkreis: String,
	pub zeitreihen: Vec<Messwert>,
	pub typ: ZeitreihenTyp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZeitreihenTyp {
	SlpSynthese,
	RlmLastgang,
	Summenzeitreihe,
	Fahrplan,
}

/// MSCONS Mehr-/Mindermengenliste: NB -> LF (MaBiS 4.3.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsconsMehrMindermengen {
	pub malo_id: MaLoId,
	pub mehrmenge_kwh: f64,
	pub mindermenge_kwh: f64,
	pub abrechnungszeitraum_von: NaiveDate,
	pub abrechnungszeitraum_bis: NaiveDate,
}

/// UTILMD Clearingliste: NB -> LF (MaBiS 4.4.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdClearingliste {
	pub eintraege: Vec<ClearingEintrag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearingEintrag {
	pub malo_id: MaLoId,
	pub feld: String,
	pub nb_wert: String,
	pub lf_wert: Option<String>,
}

// === Abrechnung (INVOIC/REMADV) ===

/// INVOIC Rechnung (generic, covers 6.1-6.9)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoicRechnung {
	pub rechnungsnummer: String,
	pub rechnungsdatum: NaiveDate,
	pub absender: MarktpartnerId,
	pub empfaenger: MarktpartnerId,
	pub positionen: Vec<RechnungsPosition>,
	pub gesamtbetrag_ct: i64, // in Cent to avoid floating point
	pub rechnungstyp: RechnungsTyp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RechnungsPosition {
	pub bezeichnung: String,
	pub menge: f64,
	pub einheit: String,
	pub einzelpreis_ct: i64,
	pub betrag_ct: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RechnungsTyp {
	Netznutzung,
	Messstellenbetrieb,
	MehrMindermengen,
	Ausgleichsenergie,
}

/// REMADV Zahlungsavis (covers 6.2-6.10)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemadvZahlungsavis {
	pub referenz_rechnungsnummer: String,
	pub zahlungsdatum: NaiveDate,
	pub betrag_ct: i64,
	pub akzeptiert: bool,
	pub ablehnungsgrund: Option<String>,
}

// === MPES Message Types ===

/// UTILMD Anmeldung Erzeugungsanlage: BV/DP -> NB (MPES 5.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdAnmeldungErzeugung {
	pub malo_id: MaLoId,
	pub anlagenbetreiber: MarktpartnerId,
	pub eeg_anlage: bool,
	pub installierte_leistung_kw: f64,
}

/// MSCONS Einspeise-Messwerte (MPES 5.4-5.6)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsconsEinspeiseMesswerte {
	pub malo_id: MaLoId,
	pub werte: Vec<Messwert>,
}

// === Redispatch 2.0 Message Types (XML-based) ===

/// XML Stammdaten Technische/Steuerbare Ressourcen (RD 7.1.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RdStammdaten {
	pub ressource_id: String,
	pub ressource_typ: RessourceTyp,
	pub standort_malo: MaLoId,
	pub installierte_leistung_kw: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RessourceTyp {
	TechnischeRessource,
	SteuerbareRessource,
}

/// PlannedResourceScheduleDocument (RD 7.2.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RdFahrplan {
	pub ressource_id: String,
	pub zeitreihe: Vec<Messwert>,
}

/// ActivationDocument: Redispatch-Abruf (RD 7.3.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RdAktivierung {
	pub ressource_id: String,
	pub sollwert_kw: f64,
	pub start: NaiveDateTime,
	pub ende: NaiveDateTime,
}

/// AcknowledgementDocument (RD 7.1.3, 7.2.3, 7.3.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RdBestaetigung {
	pub referenz_dokument_id: String,
	pub akzeptiert: bool,
	pub grund: Option<String>,
}

/// NetworkConstraintDocument (RD 7.4.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RdEngpass {
	pub netzgebiet: String,
	pub engpass_start: NaiveDateTime,
	pub engpass_ende: NaiveDateTime,
	pub betroffene_leistung_kw: f64,
}

/// Unavailability_MarketDocument (RD 7.4.2)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RdNichtverfuegbarkeit {
	pub ressource_id: String,
	pub von: NaiveDateTime,
	pub bis: NaiveDateTime,
	pub grund: String,
}

/// Kostenblatt (RD 7.5.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RdKostenblatt {
	pub ressource_id: String,
	pub kosten_ct: i64,
	pub massnahme_start: NaiveDateTime,
	pub massnahme_ende: NaiveDateTime,
}

// === §14a EnWG Message Types ===

/// UTILMD Anmeldung steuerbare Verbrauchseinrichtung (§14a 8.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdSteuerbareVerbrauchseinrichtung {
	pub malo_id: MaLoId,
	pub geraetetyp: SteuerbarerGeraetetyp,
	pub max_leistung_kw: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SteuerbarerGeraetetyp {
	Waermepumpe,
	Wallbox,
	Speicher,
	Sonstiges(String),
}

/// CLS-Kanal Steuersignal (§14a 8.4)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClsSteuersignal {
	pub malo_id: MaLoId,
	pub steuerung: Steuerungsart,
	pub zeitpunkt: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Steuerungsart {
	Dimmung { prozent: u8 },
	Abschaltung,
	Freigabe,
}

// === Gas-specific Types ===

/// Nominierung: BKV -> MGV (GABi Gas 4.3.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nominierung {
	pub bilanzkreis: String,
	pub zeitreihe_soll: Vec<Messwert>, // hourly values
}

/// Nominierungsbestätigung: MGV -> BKV (GABi Gas 4.3.2)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NominierungBestaetigung {
	pub bilanzkreis: String,
	pub matching_ergebnis: MatchingErgebnis,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MatchingErgebnis {
	Bestaetigt,
	TeilweiseBestaetigt { bestaetigte_menge_kwh: f64 },
	Abgelehnt { grund: String },
}

/// Renominierung: BKV -> MGV (GABi Gas 4.3.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Renominierung {
	pub bilanzkreis: String,
	pub zeitreihe_soll: Vec<Messwert>,
}

/// MSCONS Brennwertmitteilung: FNB/NB -> LF (KoV 5.3.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsconsBrennwert {
	pub netzgebiet: String,
	pub brennwert_kwh_per_m3: f64,
	pub zustandszahl: f64,
	pub gueltig_ab: NaiveDate,
	pub gueltig_bis: NaiveDate,
}

/// UTILMD Ausspeisepunkt: NB -> FNB (KoV 5.4.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdAusspeisepunkt {
	pub malo_id: MaLoId,
	pub nb: MarktpartnerId,
	pub fnb: MarktpartnerId,
}
