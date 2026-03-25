//! Per-type EDIFACT generator functions.
//! Each composes segment builders into a complete EDIFACT interchange string.
//! Skips 7 RD 2.0 variants (XML, not EDIFACT) — those are Task 13.

use super::params::*;
use super::segmente::*;

// ===========================================================================
// GPKE UTILMD (11)
// ===========================================================================

pub fn erzeuge_utilmd_anmeldung(p: &AnmeldungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E01", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
			dtm_102("92", p.lieferbeginn),
			rff_z13(44001),
		],
	)
}

pub fn anmeldung() -> String {
	erzeuge_utilmd_anmeldung(&AnmeldungParams::default())
}

pub fn erzeuge_utilmd_bestaetigung(p: &BestaetigungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E01", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
			dtm_102("92", p.lieferbeginn),
			rff_z13(44002),
		],
	)
}

pub fn bestaetigung() -> String {
	erzeuge_utilmd_bestaetigung(&BestaetigungParams::default())
}

pub fn erzeuge_utilmd_abmeldung(p: &AbmeldungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E02", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
			dtm_102("92", p.lieferende),
			rff_z13(44004),
		],
	)
}

pub fn abmeldung() -> String {
	erzeuge_utilmd_abmeldung(&AbmeldungParams::default())
}

pub fn erzeuge_utilmd_ablehnung(p: &AblehnungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E01", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
			sts_bare("FRIST"),
			rff_z13(44003),
		],
	)
}

pub fn ablehnung() -> String {
	erzeuge_utilmd_ablehnung(&AblehnungParams::default())
}

pub fn erzeuge_utilmd_zuordnung(p: &ZuordnungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E06", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
			dtm_102("92", p.lieferbeginn),
			rff_z13(44005),
		],
	)
}

pub fn zuordnung() -> String {
	erzeuge_utilmd_zuordnung(&ZuordnungParams::default())
}

pub fn erzeuge_utilmd_lieferende_abmeldung(p: &LieferendeAbmeldungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E02", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
			dtm_102("92", p.lieferende),
			rff_z13(44006),
		],
	)
}

pub fn lieferende_abmeldung() -> String {
	erzeuge_utilmd_lieferende_abmeldung(&LieferendeAbmeldungParams::default())
}

pub fn erzeuge_utilmd_lieferende_bestaetigung(p: &LieferendeBestaetigungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E01", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
			dtm_102("92", p.lieferende),
		],
	)
}

pub fn lieferende_bestaetigung() -> String {
	erzeuge_utilmd_lieferende_bestaetigung(&LieferendeBestaetigungParams::default())
}

pub fn erzeuge_utilmd_stammdatenaenderung(p: &StammdatenaenderungParams) -> String {
	let mut body = vec![
		bgm("E03", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
		ide_24(p.malo_id.as_str()),
		rff_z13(44112),
	];
	for (feld, wert) in &p.felder {
		body.push(cci(feld));
		body.push(cav(wert));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		body,
	)
}

pub fn stammdatenaenderung() -> String {
	erzeuge_utilmd_stammdatenaenderung(&StammdatenaenderungParams::default())
}

pub fn erzeuge_utilmd_zuordnungsliste(p: &ZuordnungslisteParams) -> String {
	let mut body = vec![
		bgm("E06", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
	];
	for (malo, datum) in &p.eintraege {
		body.push(ide_24(malo.as_str()));
		body.push(dtm_102("92", *datum));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		body,
	)
}

pub fn zuordnungsliste() -> String {
	erzeuge_utilmd_zuordnungsliste(&ZuordnungslisteParams::default())
}

pub fn erzeuge_utilmd_geschaeftsdatenanfrage(p: &GeschaeftsdatenanfrageParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E09", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
		],
	)
}

pub fn geschaeftsdatenanfrage() -> String {
	erzeuge_utilmd_geschaeftsdatenanfrage(&GeschaeftsdatenanfrageParams::default())
}

pub fn erzeuge_utilmd_geschaeftsdatenantwort(p: &GeschaeftsdatenantwortParams) -> String {
	let mut body = vec![
		bgm("E09", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
		ide_24(p.malo_id.as_str()),
	];
	for (feld, wert) in &p.felder {
		body.push(cci(feld));
		body.push(cav(wert));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		body,
	)
}

pub fn geschaeftsdatenantwort() -> String {
	erzeuge_utilmd_geschaeftsdatenantwort(&GeschaeftsdatenantwortParams::default())
}

// ===========================================================================
// GPKE MSCONS (2)
// ===========================================================================

pub fn erzeuge_mscons_schlussturnusmesswert(p: &SchlussturnusmesswertParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"MSCONS",
		"D:04B:UN:2.4c",
		vec![
			bgm("7", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			rff_z13(13002),
			loc("172", p.malo_id.as_str()),
			dtm_102("163", p.stichtag),
			qty("220", &format!("{}", p.zaehlerstand), &p.einheit),
		],
	)
}

pub fn schlussturnusmesswert() -> String {
	erzeuge_mscons_schlussturnusmesswert(&SchlussturnusmesswertParams::default())
}

pub fn erzeuge_mscons_lastgang(p: &LastgangParams) -> String {
	let mut body = vec![
		bgm("7", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
		rff_z13(13008),
		loc("172", p.malo_id.as_str()),
	];
	for (datetime, value, unit) in &p.werte {
		body.push(qty("220", value, unit));
		body.push(dtm_203("163", datetime));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"MSCONS",
		"D:04B:UN:2.4c",
		body,
	)
}

pub fn lastgang() -> String {
	erzeuge_mscons_lastgang(&LastgangParams::default())
}

// ===========================================================================
// WiM (3)
// ===========================================================================

pub fn erzeuge_utilmd_msb_wechsel_anmeldung(p: &MsbWechselAnmeldungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E03", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.melo_id.as_str()),
			dtm_102("92", p.wechseldatum),
		],
	)
}

pub fn msb_wechsel_anmeldung() -> String {
	erzeuge_utilmd_msb_wechsel_anmeldung(&MsbWechselAnmeldungParams::default())
}

pub fn erzeuge_utilmd_geraetewechsel(p: &GeraetewechselParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E03", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.melo_id.as_str()),
			dtm_102("92", p.wechseldatum),
			cci("Z30"),
			cav(&p.alte_geraete_nr),
			cci("Z30"),
			cav(&p.neue_geraete_nr),
		],
	)
}

pub fn geraetewechsel() -> String {
	erzeuge_utilmd_geraetewechsel(&GeraetewechselParams::default())
}

pub fn erzeuge_orders_werte_anfrage(p: &WerteAnfrageParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"ORDERS",
		"D:01B:UN:1.4b",
		vec![
			bgm("Z08", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
			dtm_102("163", p.zeitraum_von),
			dtm_102("164", p.zeitraum_bis),
		],
	)
}

pub fn werte_anfrage() -> String {
	erzeuge_orders_werte_anfrage(&WerteAnfrageParams::default())
}

// ===========================================================================
// UBP (5)
// ===========================================================================

pub fn erzeuge_reqote_angebotsanfrage(p: &AngebotsanfrageParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"REQOTE",
		"D:01B:UN:1.3c",
		vec![
			bgm("Z08", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.melo_id.as_str()),
			imd(&p.produkt),
		],
	)
}

pub fn angebotsanfrage() -> String {
	erzeuge_reqote_angebotsanfrage(&AngebotsanfrageParams::default())
}

pub fn erzeuge_quotes_angebot(p: &AngebotParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"QUOTES",
		"D:01B:UN:1.3b",
		vec![
			bgm("Z09", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.melo_id.as_str()),
			imd(&p.produkt),
			moa("9", &p.preis),
		],
	)
}

pub fn angebot() -> String {
	erzeuge_quotes_angebot(&AngebotParams::default())
}

pub fn erzeuge_orders_bestellung(p: &BestellungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"ORDERS",
		"D:01B:UN:1.4b",
		vec![
			bgm("Z08", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.melo_id.as_str()),
			rff("ON", &p.referenz_angebot),
		],
	)
}

pub fn bestellung() -> String {
	erzeuge_orders_bestellung(&BestellungParams::default())
}

pub fn erzeuge_ordrsp_bestellantwort(p: &BestellantwortParams) -> String {
	let mut body = vec![
		bgm("Z09", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
		ide_24(p.melo_id.as_str()),
		sts("7", &p.status_code),
	];
	if let Some(grund) = &p.grund {
		body.push(ftx("AAO", grund));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"ORDRSP",
		"D:01B:UN:1.4a",
		body,
	)
}

pub fn bestellantwort() -> String {
	erzeuge_ordrsp_bestellantwort(&BestellantwortParams::default())
}

pub fn erzeuge_pricat_preisblatt(p: &PreisblattParams) -> String {
	let mut body = vec![
		bgm("Z33", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
		dtm_102("157", p.gueltig_ab),
	];
	for (i, (bezeichnung, preis, einheit)) in p.positionen.iter().enumerate() {
		body.push(lin((i + 1) as u32, bezeichnung));
		body.push(pri("INV", preis));
		body.push(mea("AAE", "AAF", einheit));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"PRICAT",
		"D:01B:UN:2.0e",
		body,
	)
}

pub fn preisblatt() -> String {
	erzeuge_pricat_preisblatt(&PreisblattParams::default())
}

// ===========================================================================
// MaBiS (4)
// ===========================================================================

pub fn erzeuge_utilmd_bilanzkreiszuordnung(p: &BilanzkreiszuordnungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E01", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			ide_24(p.malo_id.as_str()),
			dtm_102("92", p.gueltig_ab),
			rff("Z06", &p.bilanzkreis),
		],
	)
}

pub fn bilanzkreiszuordnung() -> String {
	erzeuge_utilmd_bilanzkreiszuordnung(&BilanzkreiszuordnungParams::default())
}

pub fn erzeuge_mscons_aggregierte_zeitreihen(p: &AggregierteZeitreihenParams) -> String {
	let mut body = vec![
		bgm("7", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
		rff("Z06", &p.bilanzkreis),
		sts("7", "SUM"),
	];
	for (datetime, value, unit) in &p.werte {
		body.push(qty("220", value, unit));
		body.push(dtm_203("163", datetime));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"MSCONS",
		"D:04B:UN:2.4c",
		body,
	)
}

pub fn aggregierte_zeitreihen() -> String {
	erzeuge_mscons_aggregierte_zeitreihen(&AggregierteZeitreihenParams::default())
}

pub fn erzeuge_mscons_mehr_mindermengen(p: &MehrMindermengenParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"MSCONS",
		"D:04B:UN:2.4c",
		vec![
			bgm("7", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			loc("172", p.malo_id.as_str()),
			qty("46", &p.mehrmenge, "kWh"),
			qty("47", &p.mindermenge, "kWh"),
			dtm_102("163", p.von),
			dtm_102("164", p.bis),
		],
	)
}

pub fn mehr_mindermengen() -> String {
	erzeuge_mscons_mehr_mindermengen(&MehrMindermengenParams::default())
}

pub fn erzeuge_utilmd_clearingliste(p: &ClearinglisteParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E06", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			cci_full("CLEARING", p.malo_id.as_str(), &p.feld),
			cav(&format!("{}:{}", p.nb_wert, p.lf_wert)),
		],
	)
}

pub fn clearingliste() -> String {
	erzeuge_utilmd_clearingliste(&ClearinglisteParams::default())
}

// ===========================================================================
// Abrechnung (2)
// ===========================================================================

pub fn erzeuge_invoic_rechnung(p: &RechnungParams) -> String {
	let mut body = vec![
		bgm("380", &p.rechnungsnummer),
		dtm_102("137", p.rechnungsdatum),
		rff_z13(31002),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
	];
	for (bezeichnung, menge, einheit, einzelpreis, betrag) in &p.positionen {
		body.push(lin(1, bezeichnung));
		body.push(qty("47", menge, einheit));
		body.push(moa("203", betrag));
		body.push(pri("INV", einzelpreis));
	}
	body.push(moa("86", &p.gesamtbetrag));
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"INVOIC",
		"D:01B:UN:2.8e",
		body,
	)
}

pub fn rechnung() -> String {
	erzeuge_invoic_rechnung(&RechnungParams::default())
}

pub fn erzeuge_remadv_zahlungsavis(p: &ZahlungsavisParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"REMADV",
		"D:01B:UN:2.9d",
		vec![
			bgm("481", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			rff("ON", &p.referenz_rechnungsnummer),
			dtm_102("171", p.zahlungsdatum),
			moa("9", &p.betrag),
			sts("7", &p.status_code),
		],
	)
}

pub fn zahlungsavis() -> String {
	erzeuge_remadv_zahlungsavis(&ZahlungsavisParams::default())
}

// ===========================================================================
// MPES (2)
// ===========================================================================

pub fn erzeuge_utilmd_anmeldung_erzeugung(p: &AnmeldungErzeugungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E01", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			loc("172", p.malo_id.as_str()),
			format!("CCI+Z30::{}'", if p.eeg_anlage { "true" } else { "false" }),
			qty("220", &p.leistung_kw, "kW"),
		],
	)
}

pub fn anmeldung_erzeugung() -> String {
	erzeuge_utilmd_anmeldung_erzeugung(&AnmeldungErzeugungParams::default())
}

pub fn erzeuge_mscons_einspeise_messwerte(p: &EinspeiseMesswerteParams) -> String {
	let mut body = vec![
		bgm("7", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
		loc("172", p.malo_id.as_str()),
	];
	for (datetime, value, unit) in &p.werte {
		body.push(qty("220", value, unit));
		body.push(dtm_203("163", datetime));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"MSCONS",
		"D:04B:UN:2.4c",
		body,
	)
}

pub fn einspeise_messwerte() -> String {
	erzeuge_mscons_einspeise_messwerte(&EinspeiseMesswerteParams::default())
}

// ===========================================================================
// 14a (2)
// ===========================================================================

pub fn erzeuge_utilmd_steuerbare_verbrauchseinrichtung(p: &SteuerbareVerbrauchseinrichtungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E01", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			loc("172", p.malo_id.as_str()),
			format!("CCI+Z30::{}'", p.geraetetyp),
			qty("220", &p.max_leistung_kw, "kW"),
		],
	)
}

pub fn steuerbare_verbrauchseinrichtung() -> String {
	erzeuge_utilmd_steuerbare_verbrauchseinrichtung(&SteuerbareVerbrauchseinrichtungParams::default())
}

pub fn erzeuge_utilmd_cls_steuersignal(p: &ClsSteuersignalParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E04", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			loc("172", p.malo_id.as_str()),
			sts("7", &p.steuerung_code),
			dtm_203("163", &p.zeitpunkt),
		],
	)
}

pub fn cls_steuersignal() -> String {
	erzeuge_utilmd_cls_steuersignal(&ClsSteuersignalParams::default())
}

// ===========================================================================
// Gas (5)
// ===========================================================================

pub fn erzeuge_mscons_nominierung(p: &NominierungParams) -> String {
	let mut body = vec![
		bgm("7", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
		rff("Z06", &p.bilanzkreis),
	];
	for (datetime, value, unit) in &p.werte {
		body.push(qty("220", value, unit));
		body.push(dtm_203("163", datetime));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"MSCONS",
		"D:04B:UN:2.4c",
		body,
	)
}

pub fn nominierung() -> String {
	erzeuge_mscons_nominierung(&NominierungParams::default())
}

pub fn erzeuge_mscons_nominierung_bestaetigung(p: &NominierungBestaetigungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"MSCONS",
		"D:04B:UN:2.4c",
		vec![
			bgm("7", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			rff("Z06", &p.bilanzkreis),
			sts("7", &p.status_code),
		],
	)
}

pub fn nominierung_bestaetigung() -> String {
	erzeuge_mscons_nominierung_bestaetigung(&NominierungBestaetigungParams::default())
}

pub fn erzeuge_mscons_renominierung(p: &RenominierungParams) -> String {
	let mut body = vec![
		bgm("7", "DOK00001"),
		dtm_137(),
		nad("MS", p.sender.as_str()),
		nad("MR", p.empfaenger.as_str()),
		rff("Z06", &p.bilanzkreis),
		rff("ACE", "RENOM"),
	];
	for (datetime, value, unit) in &p.werte {
		body.push(qty("220", value, unit));
		body.push(dtm_203("163", datetime));
	}
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"MSCONS",
		"D:04B:UN:2.4c",
		body,
	)
}

pub fn renominierung() -> String {
	erzeuge_mscons_renominierung(&RenominierungParams::default())
}

pub fn erzeuge_mscons_brennwert(p: &BrennwertParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"MSCONS",
		"D:04B:UN:2.4c",
		vec![
			bgm("7", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			rff("Z13", &p.netzgebiet),
			moa("BRENNWERT", &p.brennwert),
			moa("ZUSTAND", &p.zustandszahl),
			dtm_102("163", p.gueltig_ab),
			dtm_102("164", p.gueltig_bis),
		],
	)
}

pub fn brennwert() -> String {
	erzeuge_mscons_brennwert(&BrennwertParams::default())
}

pub fn erzeuge_utilmd_ausspeisepunkt(p: &AusspeisepunktParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILMD",
		"D:11A:UN:S2.1",
		vec![
			bgm("E01", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			loc("172", p.malo_id.as_str()),
			nad("DP", p.nb.as_str()),
			nad("DP", p.fnb.as_str()),
		],
	)
}

pub fn ausspeisepunkt() -> String {
	erzeuge_utilmd_ausspeisepunkt(&AusspeisepunktParams::default())
}

// ===========================================================================
// Querschnitt (3)
// ===========================================================================

pub fn erzeuge_iftsta_statusmeldung(p: &IftstaStatusmeldungParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"IFTSTA",
		"D:01B:UN:2.0g",
		vec![
			bgm("23", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			rff("ACE", &p.referenz_nachricht),
			sts("7", &p.status_code),
			ftx("AAO", &p.beschreibung),
		],
	)
}

pub fn iftsta_statusmeldung() -> String {
	erzeuge_iftsta_statusmeldung(&IftstaStatusmeldungParams::default())
}

pub fn erzeuge_partin_marktpartner(p: &PartinMarktpartnerParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"PARTIN",
		"D:01B:UN:1.0e",
		vec![
			bgm("Z34", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			nad("DP", p.mp_id.as_str()),
			cta("IC", &p.name),
			rff("ACD", &p.rolle),
		],
	)
}

pub fn partin_marktpartner() -> String {
	erzeuge_partin_marktpartner(&PartinMarktpartnerParams::default())
}

pub fn erzeuge_utilts_zaehlzeitdefinition(p: &UtiltsZaehlzeitdefinitionParams) -> String {
	nachricht(
		p.sender.as_str(),
		p.empfaenger.as_str(),
		"UTILTS",
		"D:01B:UN:1.1e",
		vec![
			bgm("Z08", "DOK00001"),
			dtm_137(),
			nad("MS", p.sender.as_str()),
			nad("MR", p.empfaenger.as_str()),
			rff("Z13", &p.formel_id),
			imd(&p.bezeichnung),
			cci(&format!("Z30::{}", p.zeitreihen_typ)),
		],
	)
}

pub fn zaehlzeitdefinition() -> String {
	erzeuge_utilts_zaehlzeitdefinition(&UtiltsZaehlzeitdefinitionParams::default())
}

// ===========================================================================
// Tests — roundtrip: generate -> parse -> verify payload variant
// ===========================================================================

#[cfg(test)]
mod tests {
	use mako_codec::edifact::dispatch::parse_nachricht;
	use mako_types::nachricht::NachrichtenPayload;

	use super::*;

	#[test]
	fn generator_anmeldung_parses() {
		let edi = anmeldung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdAnmeldung(_)));
	}

	#[test]
	fn generator_bestaetigung_parses() {
		let edi = bestaetigung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdBestaetigung(_)));
	}

	#[test]
	fn generator_abmeldung_parses() {
		let edi = abmeldung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdAbmeldung(_)));
	}

	#[test]
	fn generator_ablehnung_parses() {
		let edi = ablehnung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdAblehnung(_)));
	}

	#[test]
	fn generator_zuordnung_parses() {
		let edi = zuordnung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdZuordnung(_)));
	}

	#[test]
	fn generator_lieferende_abmeldung_parses() {
		let edi = lieferende_abmeldung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdLieferendeAbmeldung(_)));
	}

	#[test]
	fn generator_lieferende_bestaetigung_parses() {
		let edi = lieferende_bestaetigung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdLieferendeBestaetigung(_)));
	}

	#[test]
	fn generator_stammdatenaenderung_parses() {
		let edi = stammdatenaenderung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdStammdatenaenderung(_)));
	}

	#[test]
	fn generator_zuordnungsliste_parses() {
		let edi = zuordnungsliste();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdZuordnungsliste(_)));
	}

	#[test]
	fn generator_geschaeftsdatenanfrage_parses() {
		let edi = geschaeftsdatenanfrage();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdGeschaeftsdatenanfrage(_)));
	}

	#[test]
	fn generator_geschaeftsdatenantwort_parses() {
		let edi = geschaeftsdatenantwort();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdGeschaeftsdatenantwort(_)));
	}

	#[test]
	fn generator_schlussturnusmesswert_parses() {
		let edi = schlussturnusmesswert();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::MsconsSchlussturnusmesswert(_)));
	}

	#[test]
	fn generator_lastgang_parses() {
		let edi = lastgang();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::MsconsLastgang(_)));
	}

	#[test]
	fn generator_msb_wechsel_anmeldung_parses() {
		let edi = msb_wechsel_anmeldung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdMsbWechselAnmeldung(_)));
	}

	#[test]
	fn generator_geraetewechsel_parses() {
		let edi = geraetewechsel();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdGeraetewechsel(_)));
	}

	#[test]
	fn generator_werte_anfrage_parses() {
		let edi = werte_anfrage();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::OrdersWerteAnfrage(_)));
	}

	#[test]
	fn generator_angebotsanfrage_parses() {
		let edi = angebotsanfrage();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::ReqoteAngebotsanfrage(_)));
	}

	#[test]
	fn generator_angebot_parses() {
		let edi = angebot();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::QuotesAngebot(_)));
	}

	#[test]
	fn generator_bestellung_parses() {
		let edi = bestellung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::OrdersBestellung(_)));
	}

	#[test]
	fn generator_bestellantwort_parses() {
		let edi = bestellantwort();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::OrdrspBestellantwort(_)));
	}

	#[test]
	fn generator_preisblatt_parses() {
		let edi = preisblatt();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::PricatPreisblatt(_)));
	}

	#[test]
	fn generator_bilanzkreiszuordnung_parses() {
		let edi = bilanzkreiszuordnung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdBilanzkreiszuordnung(_)));
	}

	#[test]
	fn generator_aggregierte_zeitreihen_parses() {
		let edi = aggregierte_zeitreihen();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::MsconsAggregierteZeitreihen(_)));
	}

	#[test]
	fn generator_mehr_mindermengen_parses() {
		let edi = mehr_mindermengen();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::MsconsMehrMindermengen(_)));
	}

	#[test]
	fn generator_clearingliste_parses() {
		let edi = clearingliste();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdClearingliste(_)));
	}

	#[test]
	fn generator_rechnung_parses() {
		let edi = rechnung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::InvoicRechnung(_)));
	}

	#[test]
	fn generator_zahlungsavis_parses() {
		let edi = zahlungsavis();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::RemadvZahlungsavis(_)));
	}

	#[test]
	fn generator_anmeldung_erzeugung_parses() {
		let edi = anmeldung_erzeugung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdAnmeldungErzeugung(_)));
	}

	#[test]
	fn generator_einspeise_messwerte_parses() {
		let edi = einspeise_messwerte();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::MsconsEinspeiseMesswerte(_)));
	}

	#[test]
	fn generator_steuerbare_verbrauchseinrichtung_parses() {
		let edi = steuerbare_verbrauchseinrichtung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdSteuerbareVerbrauchseinrichtung(_)));
	}

	#[test]
	fn generator_cls_steuersignal_parses() {
		let edi = cls_steuersignal();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::ClsSteuersignal(_)));
	}

	#[test]
	fn generator_nominierung_parses() {
		let edi = nominierung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::Nominierung(_)));
	}

	#[test]
	fn generator_nominierung_bestaetigung_parses() {
		let edi = nominierung_bestaetigung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::NominierungBestaetigung(_)));
	}

	#[test]
	fn generator_renominierung_parses() {
		let edi = renominierung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::Renominierung(_)));
	}

	#[test]
	fn generator_brennwert_parses() {
		let edi = brennwert();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::MsconsBrennwert(_)));
	}

	#[test]
	fn generator_ausspeisepunkt_parses() {
		let edi = ausspeisepunkt();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtilmdAusspeisepunkt(_)));
	}

	#[test]
	fn generator_iftsta_statusmeldung_parses() {
		let edi = iftsta_statusmeldung();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::IftstaStatusmeldung(_)));
	}

	#[test]
	fn generator_partin_marktpartner_parses() {
		let edi = partin_marktpartner();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::PartinMarktpartner(_)));
	}

	#[test]
	fn generator_zaehlzeitdefinition_parses() {
		let edi = zaehlzeitdefinition();
		let parsed = parse_nachricht(&edi).unwrap();
		assert!(matches!(parsed.payload, NachrichtenPayload::UtiltsZaehlzeitdefinition(_)));
	}
}
