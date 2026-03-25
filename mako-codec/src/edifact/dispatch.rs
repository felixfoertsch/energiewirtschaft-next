use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

use mako_types::gpke_nachrichten::*;
use mako_types::ids::{MaLoId, MarktpartnerId, MeLoId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
use mako_types::querschnitt::{IftstaStatusmeldung, PartinMarktpartner, UtiltsZaehlzeitdefinition};
use mako_types::rolle::MarktRolle;

use super::parser::parse_interchange;
use super::segment::{EdifactNachricht, Element, Interchange, Segment};
use super::serializer::serialize_interchange;
use crate::fehler::CodecFehler;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parse an EDIFACT string into a typed Nachricht.
/// Dispatches based on UNH message type + BGM qualifier + PID.
pub fn parse_nachricht(input: &str) -> Result<Nachricht, CodecFehler> {
	let interchange = parse_interchange(input).map_err(|e| CodecFehler::Parse(e.to_string()))?;

	if interchange.nachrichten.is_empty() {
		return Err(CodecFehler::SegmentFehlt {
			erwartet: "UNH".to_string(),
		});
	}

	let msg = &interchange.nachrichten[0];
	let segs = &msg.segmente;

	match msg.typ.as_str() {
		"UTILMD" => parse_utilmd(&interchange.sender, &interchange.empfaenger, segs),
		"MSCONS" => parse_mscons(&interchange.sender, &interchange.empfaenger, segs),
		"ORDERS" => parse_orders(segs),
		"REQOTE" => parse_reqote(segs),
		"QUOTES" => parse_quotes(segs),
		"ORDRSP" => parse_ordrsp(segs),
		"PRICAT" => parse_pricat(segs),
		"INVOIC" => parse_invoic(segs),
		"REMADV" => parse_remadv(segs),
		"IFTSTA" => parse_iftsta(segs),
		"PARTIN" => parse_partin(segs),
		"UTILTS" => parse_utilts(segs),
		other => Err(CodecFehler::UnbekannterNachrichtentyp {
			typ: other.to_string(),
		}),
	}
}

/// Serialize a typed Nachricht to an EDIFACT string.
pub fn serialize_nachricht(nachricht: &Nachricht) -> String {
	match &nachricht.payload {
		NachrichtenPayload::UtilmdAnmeldung(p) => serialize_utilmd_anmeldung(nachricht, p),
		NachrichtenPayload::UtilmdBestaetigung(p) => serialize_utilmd_bestaetigung(nachricht, p),
		NachrichtenPayload::UtilmdAbmeldung(p) => serialize_utilmd_abmeldung(nachricht, p),
		NachrichtenPayload::UtilmdAblehnung(p) => serialize_utilmd_ablehnung(nachricht, p),
		NachrichtenPayload::UtilmdZuordnung(p) => serialize_utilmd_zuordnung(nachricht, p),
		NachrichtenPayload::UtilmdLieferendeAbmeldung(p) => {
			serialize_utilmd_lieferende_abmeldung(nachricht, p)
		}
		NachrichtenPayload::UtilmdLieferendeBestaetigung(p) => {
			serialize_utilmd_lieferende_bestaetigung(nachricht, p)
		}
		NachrichtenPayload::UtilmdStammdatenaenderung(p) => {
			serialize_utilmd_stammdatenaenderung(nachricht, p)
		}
		NachrichtenPayload::UtilmdZuordnungsliste(p) => {
			serialize_utilmd_zuordnungsliste(nachricht, p)
		}
		NachrichtenPayload::UtilmdGeschaeftsdatenanfrage(p) => {
			serialize_utilmd_geschaeftsdatenanfrage(nachricht, p)
		}
		NachrichtenPayload::UtilmdGeschaeftsdatenantwort(p) => {
			serialize_utilmd_geschaeftsdatenantwort(nachricht, p)
		}
		NachrichtenPayload::MsconsSchlussturnusmesswert(p) => {
			serialize_mscons_zaehlerstand(nachricht, p)
		}
		NachrichtenPayload::MsconsLastgang(p) => serialize_mscons_lastgang(nachricht, p),
		// WiM
		NachrichtenPayload::UtilmdMsbWechselAnmeldung(p) => {
			serialize_utilmd_msb_wechsel(nachricht, p)
		}
		NachrichtenPayload::UtilmdGeraetewechsel(p) => {
			serialize_utilmd_geraetewechsel(nachricht, p)
		}
		NachrichtenPayload::OrdersWerteAnfrage(p) => {
			serialize_orders_werte_anfrage(nachricht, p)
		}
		// UBP
		NachrichtenPayload::ReqoteAngebotsanfrage(p) => {
			serialize_reqote_angebotsanfrage(nachricht, p)
		}
		NachrichtenPayload::QuotesAngebot(p) => serialize_quotes_angebot(nachricht, p),
		NachrichtenPayload::OrdersBestellung(p) => serialize_orders_bestellung(nachricht, p),
		NachrichtenPayload::OrdrspBestellantwort(p) => {
			serialize_ordrsp_bestellantwort(nachricht, p)
		}
		NachrichtenPayload::PricatPreisblatt(p) => serialize_pricat_preisblatt(nachricht, p),
		// MaBiS
		NachrichtenPayload::UtilmdBilanzkreiszuordnung(p) => {
			serialize_utilmd_bilanzkreiszuordnung(nachricht, p)
		}
		NachrichtenPayload::MsconsAggregierteZeitreihen(p) => {
			serialize_mscons_aggregierte_zeitreihen(nachricht, p)
		}
		NachrichtenPayload::MsconsMehrMindermengen(p) => {
			serialize_mscons_mehr_mindermengen(nachricht, p)
		}
		NachrichtenPayload::UtilmdClearingliste(p) => {
			serialize_utilmd_clearingliste(nachricht, p)
		}
		// Abrechnung
		NachrichtenPayload::InvoicRechnung(p) => serialize_invoic_rechnung(nachricht, p),
		NachrichtenPayload::RemadvZahlungsavis(p) => {
			serialize_remadv_zahlungsavis(nachricht, p)
		}
		// MPES
		NachrichtenPayload::UtilmdAnmeldungErzeugung(p) => {
			serialize_utilmd_anmeldung_erzeugung(nachricht, p)
		}
		NachrichtenPayload::MsconsEinspeiseMesswerte(p) => {
			serialize_mscons_einspeise_messwerte(nachricht, p)
		}
		// §14a
		NachrichtenPayload::UtilmdSteuerbareVerbrauchseinrichtung(p) => {
			serialize_utilmd_steuerbare_verbrauchseinrichtung(nachricht, p)
		}
		NachrichtenPayload::ClsSteuersignal(p) => {
			serialize_cls_steuersignal(nachricht, p)
		}
		// Gas
		NachrichtenPayload::Nominierung(p) => serialize_nominierung(nachricht, p),
		NachrichtenPayload::NominierungBestaetigung(p) => {
			serialize_nominierung_bestaetigung(nachricht, p)
		}
		NachrichtenPayload::Renominierung(p) => serialize_renominierung(nachricht, p),
		NachrichtenPayload::MsconsBrennwert(p) => serialize_mscons_brennwert(nachricht, p),
		NachrichtenPayload::UtilmdAusspeisepunkt(p) => {
			serialize_utilmd_ausspeisepunkt(nachricht, p)
		}
		// Querschnitt
		NachrichtenPayload::IftstaStatusmeldung(p) => {
			serialize_iftsta_statusmeldung(nachricht, p)
		}
		NachrichtenPayload::PartinMarktpartner(p) => {
			serialize_partin_marktpartner(nachricht, p)
		}
		NachrichtenPayload::UtiltsZaehlzeitdefinition(p) => {
			serialize_utilts_zaehlzeitdefinition(nachricht, p)
		}
		// Redispatch 2.0 (XML-based, not EDIFACT)
		_ => unimplemented!("serialize_nachricht: payload type not yet supported (Redispatch 2.0 uses XML)"),
	}
}

// ---------------------------------------------------------------------------
// UTILMD dispatcher
// ---------------------------------------------------------------------------

/// Dispatch UTILMD messages using PID as primary discriminator, falling back
/// to BGM qualifier + heuristics when PID is absent.
fn parse_utilmd(
	unb_sender: &str,
	unb_empfaenger: &str,
	segs: &[Segment],
) -> Result<Nachricht, CodecFehler> {
	let bgm = find_segment(segs, "BGM")?;
	let qualifier = bgm
		.elements
		.first()
		.and_then(|e| e.components.first())
		.ok_or(CodecFehler::FeldFehlt {
			segment: "BGM".to_string(),
			feld: "qualifier".to_string(),
		})?
		.clone();

	// Try to extract PID (optional for some variants)
	let pid_opt = find_qualified_segment(segs, "RFF", "Z13")
		.ok()
		.and_then(|rff| rff.elements.first())
		.and_then(|e| e.components.get(1))
		.and_then(|s| s.parse::<u32>().ok())
		.and_then(PruefIdentifikator::from_code);

	// Primary dispatch: PID when present
	if let Some(pid) = pid_opt {
		return match pid {
			PruefIdentifikator::AnmeldungNn => {
				parse_utilmd_anmeldung(unb_sender, unb_empfaenger, segs, Some(pid))
			}
			PruefIdentifikator::AnmeldungBestaetigung => {
				parse_utilmd_bestaetigung(unb_sender, unb_empfaenger, segs, Some(pid))
			}
			PruefIdentifikator::AnmeldungAblehnung => {
				parse_utilmd_ablehnung(unb_sender, unb_empfaenger, segs, Some(pid))
			}
			PruefIdentifikator::AbmeldungNn => {
				parse_utilmd_abmeldung(unb_sender, unb_empfaenger, segs, Some(pid))
			}
			PruefIdentifikator::AbmeldungBestaetigung => {
				parse_utilmd_zuordnung(unb_sender, unb_empfaenger, segs, Some(pid))
			}
			PruefIdentifikator::AbmeldungAblehnung => {
				parse_utilmd_lieferende_abmeldung(unb_sender, unb_empfaenger, segs, Some(pid))
			}
			PruefIdentifikator::Stammdatenaenderung => {
				parse_utilmd_stammdatenaenderung(unb_sender, unb_empfaenger, segs, Some(pid))
			}
			_ => Err(CodecFehler::UnbekannterNachrichtentyp {
				typ: format!("UTILMD/PID:{}", pid.code()),
			}),
		};
	}

	// Check for bilanzkreis (RFF+Z06) — MaBiS Bilanzkreiszuordnung
	let has_bk = find_qualified_segment(segs, "RFF", "Z06").is_ok();

	// Check for IDE with MeLoId (33 chars, starts with "DE") — WiM
	let ide_value = find_qualified_segment(segs, "IDE", "24")
		.ok()
		.and_then(|ide| ide.elements.get(1))
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();
	let has_melo = ide_value.len() == 33 && ide_value.starts_with("DE");

	// Check for CCI+Z30 pairs (Geraetewechsel)
	let has_cci_z30 = segs.iter().any(|s| {
		s.tag == "CCI"
			&& s.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "Z30")
	});

	// Check for CCI+CLEARING (Clearingliste)
	let has_clearing = segs.iter().any(|s| {
		s.tag == "CCI"
			&& s.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "CLEARING")
	});

	// Check for QTY+220 (Leistung) — MPES AnmeldungErzeugung
	let has_qty_220 = segs.iter().any(|s| {
		s.tag == "QTY"
			&& s.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "220")
	});

	// Check for multiple NAD+DP — Ausspeisepunkt
	let nad_dp_count = segs
		.iter()
		.filter(|s| {
			s.tag == "NAD"
				&& s.elements
					.first()
					.and_then(|e| e.components.first())
					.is_some_and(|q| q == "DP")
		})
		.count();

	// Check for CCI+Z30 with geraetetyp values (SteuerbareVerbrauchseinrichtung)
	let has_geraetetyp_cci = segs.iter().any(|s| {
		s.tag == "CCI"
			&& s.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "Z30")
			&& s.elements
				.first()
				.and_then(|e| e.components.get(2))
				.is_some_and(|v| {
					matches!(v.as_str(), "Waermepumpe" | "Wallbox" | "Speicher")
						|| v.starts_with("Sonstiges:")
				})
	});

	// Fallback: BGM qualifier + heuristics
	match qualifier.as_str() {
		"E01" => {
			if has_bk {
				parse_utilmd_bilanzkreiszuordnung(segs)
			} else if has_geraetetyp_cci {
				// §14a: SteuerbareVerbrauchseinrichtung (CCI with Waermepumpe/Wallbox/Speicher)
				parse_utilmd_steuerbare_verbrauchseinrichtung(segs)
			} else if nad_dp_count >= 2 {
				// Gas: Ausspeisepunkt has two NAD+DP (nb + fnb)
				parse_utilmd_ausspeisepunkt(segs)
			} else if has_qty_220 {
				// MPES: AnmeldungErzeugung has QTY+220 for installierte_leistung
				parse_utilmd_anmeldung_erzeugung(segs)
			} else {
				// E01 without PID = LieferendeBestaetigung
				parse_utilmd_lieferende_bestaetigung(unb_sender, unb_empfaenger, segs)
			}
		}
		"E04" => {
			// §14a: ClsSteuersignal
			parse_cls_steuersignal_msg(segs)
		}
		"E03" => {
			if has_cci_z30 {
				parse_utilmd_geraetewechsel(segs)
			} else if has_melo {
				parse_utilmd_msb_wechsel(segs)
			} else {
				parse_utilmd_stammdatenaenderung(unb_sender, unb_empfaenger, segs, None)
			}
		}
		"E06" => {
			if has_clearing {
				parse_utilmd_clearingliste(segs)
			} else {
				// E06 with multiple IDE+24 = Zuordnungsliste, otherwise Zuordnung
				let ide_count = segs
					.iter()
					.filter(|s| {
						s.tag == "IDE"
							&& s.elements
								.first()
								.and_then(|e| e.components.first())
								.is_some_and(|q| q == "24")
					})
					.count();
				if ide_count > 1 {
					parse_utilmd_zuordnungsliste(unb_sender, unb_empfaenger, segs)
				} else {
					parse_utilmd_zuordnung(unb_sender, unb_empfaenger, segs, None)
				}
			}
		}
		"E09" => {
			// E09: check for CCI segments to distinguish anfrage vs antwort
			let has_cci = segs.iter().any(|s| s.tag == "CCI");
			if has_cci {
				parse_utilmd_geschaeftsdatenantwort(unb_sender, unb_empfaenger, segs)
			} else {
				parse_utilmd_geschaeftsdatenanfrage(unb_sender, unb_empfaenger, segs)
			}
		}
		other => Err(CodecFehler::UnbekannterNachrichtentyp {
			typ: format!("UTILMD/{other}"),
		}),
	}
}

// ---------------------------------------------------------------------------
// UTILMD parsers
// ---------------------------------------------------------------------------

fn parse_utilmd_anmeldung(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
	pid: Option<PruefIdentifikator>,
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;
	let lieferbeginn = extract_date(segs, "92")?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::LieferantNeu,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: pid,
		payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
			malo_id,
			lieferant_neu: absender,
			lieferbeginn,
		}),
	})
}

fn parse_utilmd_bestaetigung(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
	pid: Option<PruefIdentifikator>,
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;
	let lieferbeginn = extract_date(segs, "92")?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: empfaenger.clone(),
		empfaenger_rolle: MarktRolle::LieferantNeu,
		pruef_id: pid,
		payload: NachrichtenPayload::UtilmdBestaetigung(UtilmdBestaetigung {
			malo_id,
			bestaetigt_fuer: empfaenger,
			lieferbeginn,
		}),
	})
}

fn parse_utilmd_abmeldung(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
	pid: Option<PruefIdentifikator>,
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;
	let lieferende = extract_date(segs, "92")?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: empfaenger.clone(),
		empfaenger_rolle: MarktRolle::LieferantAlt,
		pruef_id: pid,
		payload: NachrichtenPayload::UtilmdAbmeldung(UtilmdAbmeldung {
			malo_id,
			lieferant_alt: empfaenger,
			lieferende,
		}),
	})
}

fn parse_utilmd_ablehnung(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
	pid: Option<PruefIdentifikator>,
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;

	// STS segment carries the rejection reason
	let grund = find_segment(segs, "STS")
		.ok()
		.and_then(|sts| sts.elements.first())
		.and_then(|e| e.components.first())
		.map(|s| match s.as_str() {
			"FRIST" => AblehnungsGrund::Fristverletzung,
			"MALO" => AblehnungsGrund::MaloUnbekannt,
			"VERTRAG" => AblehnungsGrund::KeinVertrag,
			other => AblehnungsGrund::Sonstiges(other.to_string()),
		})
		.unwrap_or(AblehnungsGrund::Sonstiges("unbekannt".to_string()));

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::LieferantAlt,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: pid,
		payload: NachrichtenPayload::UtilmdAblehnung(UtilmdAblehnung { malo_id, grund }),
	})
}

fn parse_utilmd_zuordnung(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
	pid: Option<PruefIdentifikator>,
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;
	let lieferbeginn = extract_date(segs, "92")?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: empfaenger.clone(),
		empfaenger_rolle: MarktRolle::LieferantNeu,
		pruef_id: pid,
		payload: NachrichtenPayload::UtilmdZuordnung(UtilmdZuordnung {
			malo_id,
			zugeordnet_an: empfaenger,
			lieferbeginn,
		}),
	})
}

fn parse_utilmd_lieferende_abmeldung(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
	pid: Option<PruefIdentifikator>,
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;
	let lieferende = extract_date(segs, "92")?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: pid,
		payload: NachrichtenPayload::UtilmdLieferendeAbmeldung(UtilmdLieferendeAbmeldung {
			malo_id,
			lieferant: absender,
			lieferende,
		}),
	})
}

fn parse_utilmd_lieferende_bestaetigung(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;
	let lieferende = extract_date(segs, "92")?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdLieferendeBestaetigung(UtilmdLieferendeBestaetigung {
			malo_id,
			lieferende,
		}),
	})
}

fn parse_utilmd_stammdatenaenderung(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
	pid: Option<PruefIdentifikator>,
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;

	// Extract CCI+CAV pairs for Stammdatenfelder
	let mut aenderungen = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "CCI" {
			let feld = segs[i]
				.elements
				.first()
				.and_then(|e| e.components.first())
				.cloned()
				.unwrap_or_default();
			// Next segment should be CAV
			let neuer_wert = if i + 1 < segs.len() && segs[i + 1].tag == "CAV" {
				segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.first())
					.cloned()
					.unwrap_or_default()
			} else {
				String::new()
			};
			aenderungen.push(Stammdatenfeld {
				feld,
				alter_wert: None,
				neuer_wert,
			});
			i += 2;
			continue;
		}
		i += 1;
	}

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: pid,
		payload: NachrichtenPayload::UtilmdStammdatenaenderung(UtilmdStammdatenaenderung {
			malo_id,
			initiator: absender,
			aenderungen,
		}),
	})
}

fn parse_utilmd_zuordnungsliste(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;

	// Each IDE+24 starts an entry group with its own DTM+92
	let mut eintraege = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "IDE"
			&& segs[i]
				.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "24")
		{
			let malo_str = segs[i]
				.elements
				.get(1)
				.and_then(|e| e.components.first())
				.ok_or(CodecFehler::FeldFehlt {
					segment: "IDE+24".to_string(),
					feld: "MaLo-ID".to_string(),
				})?;
			let malo_id = MaLoId::new(malo_str).map_err(|_| CodecFehler::UngueltigerWert {
				segment: "IDE+24".to_string(),
				feld: "MaLo-ID".to_string(),
				wert: malo_str.clone(),
			})?;

			// Look for the DTM+92 following this IDE
			let mut gueltig_ab = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
			for j in (i + 1)..segs.len() {
				if segs[j].tag == "IDE" {
					break; // next entry
				}
				if segs[j].tag == "DTM"
					&& segs[j]
						.elements
						.first()
						.and_then(|e| e.components.first())
						.is_some_and(|q| q == "92")
				{
					let d = segs[j]
						.elements
						.first()
						.and_then(|e| e.components.get(1))
						.ok_or(CodecFehler::FeldFehlt {
							segment: "DTM+92".to_string(),
							feld: "datum".to_string(),
						})?;
					gueltig_ab = NaiveDate::parse_from_str(d, "%Y%m%d").map_err(|_| {
						CodecFehler::UngueltigesFormat {
							segment: "DTM+92".to_string(),
							feld: "datum".to_string(),
							erwartet: "YYYYMMDD".to_string(),
						}
					})?;
					break;
				}
			}

			eintraege.push(ZuordnungsEintrag {
				malo_id,
				zugeordnet_an: empfaenger.clone(),
				gueltig_ab,
			});
		}
		i += 1;
	}

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: empfaenger.clone(),
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdZuordnungsliste(UtilmdZuordnungsliste { eintraege }),
	})
}

fn parse_utilmd_geschaeftsdatenanfrage(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdGeschaeftsdatenanfrage(UtilmdGeschaeftsdatenanfrage {
			malo_id,
			anfragender: absender,
		}),
	})
}

fn parse_utilmd_geschaeftsdatenantwort(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;

	// CCI+CAV pairs
	let mut stammdaten = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "CCI" {
			let feld = segs[i]
				.elements
				.first()
				.and_then(|e| e.components.first())
				.cloned()
				.unwrap_or_default();
			let neuer_wert = if i + 1 < segs.len() && segs[i + 1].tag == "CAV" {
				segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.first())
					.cloned()
					.unwrap_or_default()
			} else {
				String::new()
			};
			stammdaten.push(Stammdatenfeld {
				feld,
				alter_wert: None,
				neuer_wert,
			});
			i += 2;
			continue;
		}
		i += 1;
	}

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdGeschaeftsdatenantwort(UtilmdGeschaeftsdatenantwort {
			malo_id,
			stammdaten,
		}),
	})
}

// ---------------------------------------------------------------------------
// MSCONS dispatcher + parsers
// ---------------------------------------------------------------------------

fn parse_mscons(
	_unb_sender: &str,
	_unb_empfaenger: &str,
	segs: &[Segment],
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;

	// PID discriminates Zaehlerstand vs Lastgang
	let pid_opt = find_qualified_segment(segs, "RFF", "Z13")
		.ok()
		.and_then(|rff| rff.elements.first())
		.and_then(|e| e.components.get(1))
		.and_then(|s| s.parse::<u32>().ok())
		.and_then(PruefIdentifikator::from_code);

	match pid_opt {
		Some(PruefIdentifikator::Zaehlerstand) => {
			parse_mscons_zaehlerstand(segs, absender, empfaenger)
		}
		Some(PruefIdentifikator::Lastgang) => parse_mscons_lastgang(segs, absender, empfaenger),
		Some(other) => Err(CodecFehler::UnbekannterNachrichtentyp {
			typ: format!("MSCONS/PID:{}", other.code()),
		}),
		None => {
			// No PID: use heuristics. Check for bilanzkreis (RFF+Z06)
			let has_bk = find_qualified_segment(segs, "RFF", "Z06").is_ok();
			// Check for two QTY segments with different qualifiers (mehr/minder)
			let qty_46_count = segs
				.iter()
				.filter(|s| {
					s.tag == "QTY"
						&& s.elements
							.first()
							.and_then(|e| e.components.first())
							.is_some_and(|q| q == "46")
				})
				.count();
			let has_qty_47 = segs.iter().any(|s| {
				s.tag == "QTY"
					&& s.elements
						.first()
						.and_then(|e| e.components.first())
						.is_some_and(|q| q == "47")
			});
			// Check for MOA+BRENNWERT (Gas Brennwertmitteilung)
			let has_brennwert = find_qualified_segment(segs, "MOA", "BRENNWERT").is_ok();
			// Check for RFF+ACE (renomination reference)
			let has_rff_ace = find_qualified_segment(segs, "RFF", "ACE").is_ok();
			// Check STS code: NominierungBestaetigung uses Z06/Z08/TEIL, AggregierteZeitreihen uses SUM/SLP/RLM/FPL
			let sts_code = find_qualified_segment(segs, "STS", "7")
				.ok()
				.and_then(|sts| sts.elements.get(2))
				.and_then(|e| e.components.first())
				.cloned()
				.unwrap_or_default();
			let is_nom_status = matches!(sts_code.as_str(), "Z06" | "Z08")
				|| sts_code.starts_with("TEIL:");

			if has_brennwert {
				parse_mscons_brennwert(segs, absender, empfaenger)
			} else if has_bk && has_rff_ace {
				// Renominierung: has bilanzkreis + RFF+ACE (re-nomination reference)
				parse_renominierung_msg(segs, absender, empfaenger)
			} else if has_bk && is_nom_status {
				// NominierungBestaetigung: has bilanzkreis + STS with nomination status code
				parse_nominierung_bestaetigung_msg(segs, absender, empfaenger)
			} else if has_bk && !sts_code.is_empty() {
				// AggregierteZeitreihen: has bilanzkreis + STS with time series type code
				parse_mscons_aggregierte_zeitreihen(segs, absender, empfaenger)
			} else if has_bk {
				// BK without STS or other indicators = Nominierung
				parse_nominierung_msg(segs, absender, empfaenger)
			} else if qty_46_count > 0 && has_qty_47 {
				parse_mscons_mehr_mindermengen(segs, absender, empfaenger)
			} else {
				// MPES EinspeiseMesswerte: MSCONS without PID but with LOC
				let has_loc = find_qualified_segment(segs, "LOC", "172").is_ok();
				if has_loc {
					parse_mscons_einspeise_messwerte(segs, absender, empfaenger)
				} else {
					Err(CodecFehler::SegmentFehlt {
						erwartet: "RFF+Z13".to_string(),
					})
				}
			}
		}
	}
}

fn parse_mscons_zaehlerstand(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	// NAD+DP or LOC+172 carries the MaLo
	let malo_id = extract_malo_from_loc_or_nad_dp(segs)?;

	// QTY+220 = zaehlerstand
	let qty = find_qualified_segment(segs, "QTY", "220")?;
	let wert_str = qty
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "QTY+220".to_string(),
			feld: "wert".to_string(),
		})?;
	let zaehlerstand =
		wert_str
			.parse::<f64>()
			.map_err(|_| CodecFehler::UngueltigerWert {
				segment: "QTY+220".to_string(),
				feld: "wert".to_string(),
				wert: wert_str.clone(),
			})?;

	// DTM+163 = stichtag
	let stichtag = extract_date(segs, "163")?;

	// MOA or QTY qualifier for einheit — we use a simple default
	let einheit = extract_einheit_from_qty(qty);

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: Some(PruefIdentifikator::Zaehlerstand),
		payload: NachrichtenPayload::MsconsSchlussturnusmesswert(MsconsSchlussturnusmesswert {
			malo_id,
			zaehlerstand,
			stichtag,
			einheit,
		}),
	})
}

fn parse_mscons_lastgang(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	let malo_id = extract_malo_from_loc_or_nad_dp(segs)?;

	// Collect QTY+DTM pairs for time series
	let mut werte = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "QTY"
			&& segs[i]
				.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "220")
		{
			let wert_str = segs[i]
				.elements
				.first()
				.and_then(|e| e.components.get(1))
				.ok_or(CodecFehler::FeldFehlt {
					segment: "QTY+220".to_string(),
					feld: "wert".to_string(),
				})?;
			let wert = wert_str
				.parse::<f64>()
				.map_err(|_| CodecFehler::UngueltigerWert {
					segment: "QTY+220".to_string(),
					feld: "wert".to_string(),
					wert: wert_str.clone(),
				})?;

			let einheit = extract_einheit_from_qty(&segs[i]);

			// Look for DTM+163 after this QTY
			let zeitpunkt = if i + 1 < segs.len()
				&& segs[i + 1].tag == "DTM"
				&& segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.first())
					.is_some_and(|q| q == "163")
			{
				let ts_str = segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.get(1))
					.ok_or(CodecFehler::FeldFehlt {
						segment: "DTM+163".to_string(),
						feld: "zeitpunkt".to_string(),
					})?;
				parse_datetime(ts_str)?
			} else {
				return Err(CodecFehler::SegmentFehlt {
					erwartet: "DTM+163 after QTY".to_string(),
				});
			};

			werte.push(Messwert {
				zeitpunkt,
				wert,
				einheit,
				status: MesswertStatus::Gemessen,
			});
			i += 2;
			continue;
		}
		i += 1;
	}

	// Derive interval from first two timestamps (default 15 min)
	let intervall_minuten = if werte.len() >= 2 {
		let diff = werte[1]
			.zeitpunkt
			.signed_duration_since(werte[0].zeitpunkt);
		diff.num_minutes().unsigned_abs() as u32
	} else {
		15
	};

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: Some(PruefIdentifikator::Lastgang),
		payload: NachrichtenPayload::MsconsLastgang(MsconsLastgang {
			malo_id,
			werte,
			intervall_minuten,
		}),
	})
}

// ---------------------------------------------------------------------------
// UTILMD serializers
// ---------------------------------------------------------------------------

fn serialize_utilmd_anmeldung(nachricht: &Nachricht, p: &UtilmdAnmeldung) -> String {
	let pid_code = nachricht
		.pruef_id
		.map(|p| p.code().to_string())
		.unwrap_or_default();

	let mut segmente = vec![
		bgm_segment("E01"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
		dtm_date_segment("92", &p.lieferbeginn),
	];
	if !pid_code.is_empty() {
		segmente.push(rff_z13_segment(&pid_code));
	}

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_bestaetigung(nachricht: &Nachricht, p: &UtilmdBestaetigung) -> String {
	let pid_code = nachricht
		.pruef_id
		.map(|p| p.code().to_string())
		.unwrap_or_default();

	let mut segmente = vec![
		bgm_segment("E01"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
		dtm_date_segment("92", &p.lieferbeginn),
	];
	if !pid_code.is_empty() {
		segmente.push(rff_z13_segment(&pid_code));
	}

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_abmeldung(nachricht: &Nachricht, p: &UtilmdAbmeldung) -> String {
	let pid_code = nachricht
		.pruef_id
		.map(|p| p.code().to_string())
		.unwrap_or_default();

	let mut segmente = vec![
		bgm_segment("E02"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
		dtm_date_segment("92", &p.lieferende),
	];
	if !pid_code.is_empty() {
		segmente.push(rff_z13_segment(&pid_code));
	}

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_ablehnung(nachricht: &Nachricht, p: &UtilmdAblehnung) -> String {
	let pid_code = nachricht
		.pruef_id
		.map(|p| p.code().to_string())
		.unwrap_or_default();

	let grund_code = match &p.grund {
		AblehnungsGrund::Fristverletzung => "FRIST".to_string(),
		AblehnungsGrund::MaloUnbekannt => "MALO".to_string(),
		AblehnungsGrund::KeinVertrag => "VERTRAG".to_string(),
		AblehnungsGrund::Sonstiges(s) => s.clone(),
	};

	let mut segmente = vec![
		bgm_segment("E01"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
		Segment {
			tag: "STS".to_string(),
			elements: vec![Element {
				components: vec![grund_code],
			}],
		},
	];
	if !pid_code.is_empty() {
		segmente.push(rff_z13_segment(&pid_code));
	}

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_zuordnung(nachricht: &Nachricht, p: &UtilmdZuordnung) -> String {
	let pid_code = nachricht
		.pruef_id
		.map(|p| p.code().to_string())
		.unwrap_or_default();

	let mut segmente = vec![
		bgm_segment("E06"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
		dtm_date_segment("92", &p.lieferbeginn),
	];
	if !pid_code.is_empty() {
		segmente.push(rff_z13_segment(&pid_code));
	}

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_lieferende_abmeldung(
	nachricht: &Nachricht,
	p: &UtilmdLieferendeAbmeldung,
) -> String {
	let pid_code = nachricht
		.pruef_id
		.map(|p| p.code().to_string())
		.unwrap_or_default();

	let mut segmente = vec![
		bgm_segment("E02"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
		dtm_date_segment("92", &p.lieferende),
	];
	if !pid_code.is_empty() {
		segmente.push(rff_z13_segment(&pid_code));
	}

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_lieferende_bestaetigung(
	nachricht: &Nachricht,
	p: &UtilmdLieferendeBestaetigung,
) -> String {
	let segmente = vec![
		bgm_segment("E01"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
		dtm_date_segment("92", &p.lieferende),
	];

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_stammdatenaenderung(
	nachricht: &Nachricht,
	p: &UtilmdStammdatenaenderung,
) -> String {
	let pid_code = nachricht
		.pruef_id
		.map(|p| p.code().to_string())
		.unwrap_or_default();

	let mut segmente = vec![
		bgm_segment("E03"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
	];
	if !pid_code.is_empty() {
		segmente.push(rff_z13_segment(&pid_code));
	}
	for aenderung in &p.aenderungen {
		segmente.push(Segment {
			tag: "CCI".to_string(),
			elements: vec![Element {
				components: vec![aenderung.feld.clone()],
			}],
		});
		segmente.push(Segment {
			tag: "CAV".to_string(),
			elements: vec![Element {
				components: vec![aenderung.neuer_wert.clone()],
			}],
		});
	}

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_zuordnungsliste(nachricht: &Nachricht, p: &UtilmdZuordnungsliste) -> String {
	let mut segmente = vec![
		bgm_segment("E06"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
	];

	for eintrag in &p.eintraege {
		segmente.push(ide_segment(eintrag.malo_id.as_str()));
		segmente.push(dtm_date_segment("92", &eintrag.gueltig_ab));
	}

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_geschaeftsdatenanfrage(
	nachricht: &Nachricht,
	p: &UtilmdGeschaeftsdatenanfrage,
) -> String {
	let segmente = vec![
		bgm_segment("E09"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
	];

	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_geschaeftsdatenantwort(
	nachricht: &Nachricht,
	p: &UtilmdGeschaeftsdatenantwort,
) -> String {
	let mut segmente = vec![
		bgm_segment("E09"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
	];

	for sd in &p.stammdaten {
		segmente.push(Segment {
			tag: "CCI".to_string(),
			elements: vec![Element {
				components: vec![sd.feld.clone()],
			}],
		});
		segmente.push(Segment {
			tag: "CAV".to_string(),
			elements: vec![Element {
				components: vec![sd.neuer_wert.clone()],
			}],
		});
	}

	wrap_utilmd(nachricht, segmente)
}

// ---------------------------------------------------------------------------
// MSCONS serializers
// ---------------------------------------------------------------------------

fn serialize_mscons_zaehlerstand(
	nachricht: &Nachricht,
	p: &MsconsSchlussturnusmesswert,
) -> String {
	let segmente = vec![
		bgm_7_segment(),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_z13_segment(&nachricht.pruef_id.map(|p| p.code().to_string()).unwrap_or_default()),
		loc_segment(p.malo_id.as_str()),
		dtm_date_segment("163", &p.stichtag),
		qty_segment(p.zaehlerstand, &p.einheit),
	];

	wrap_mscons(nachricht, segmente)
}

fn serialize_mscons_lastgang(nachricht: &Nachricht, p: &MsconsLastgang) -> String {
	let mut segmente = vec![
		bgm_7_segment(),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_z13_segment(&nachricht.pruef_id.map(|p| p.code().to_string()).unwrap_or_default()),
		loc_segment(p.malo_id.as_str()),
	];

	for mw in &p.werte {
		segmente.push(qty_segment(mw.wert, &mw.einheit));
		segmente.push(Segment {
			tag: "DTM".to_string(),
			elements: vec![Element {
				components: vec![
					"163".to_string(),
					format!(
						"{:04}{:02}{:02}{:02}{:02}{:02}",
						mw.zeitpunkt.date().year(),
						mw.zeitpunkt.date().month(),
						mw.zeitpunkt.date().day(),
						mw.zeitpunkt.time().hour(),
						mw.zeitpunkt.time().minute(),
						mw.zeitpunkt.time().second(),
					),
					"203".to_string(),
				],
			}],
		});
	}

	wrap_mscons(nachricht, segmente)
}

// ---------------------------------------------------------------------------
// WiM UTILMD parsers
// ---------------------------------------------------------------------------

fn parse_utilmd_msb_wechsel(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let melo_id = extract_melo_id(segs)?;
	let wechseldatum = extract_date(segs, "92")?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdMsbWechselAnmeldung(UtilmdMsbWechselAnmeldung {
			melo_id,
			msb_neu: absender,
			wechseldatum,
		}),
	})
}

fn parse_utilmd_geraetewechsel(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let melo_id = extract_melo_id(segs)?;
	let wechseldatum = extract_date(segs, "92")?;

	// Extract two CCI+Z30 / CAV pairs for device numbers
	let mut geraete_nrs = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "CCI"
			&& segs[i]
				.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "Z30")
		{
			let nr = if i + 1 < segs.len() && segs[i + 1].tag == "CAV" {
				segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.first())
					.cloned()
					.unwrap_or_default()
			} else {
				String::new()
			};
			geraete_nrs.push(nr);
			i += 2;
			continue;
		}
		i += 1;
	}

	let alte_geraete_nr = geraete_nrs.first().cloned().unwrap_or_default();
	let neue_geraete_nr = geraete_nrs.get(1).cloned().unwrap_or_default();

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdGeraetewechsel(UtilmdGeraetewechsel {
			melo_id,
			alte_geraete_nr,
			neue_geraete_nr,
			wechseldatum,
		}),
	})
}

// ---------------------------------------------------------------------------
// MaBiS UTILMD parsers
// ---------------------------------------------------------------------------

fn parse_utilmd_bilanzkreiszuordnung(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_id(segs)?;
	let gueltig_ab = extract_date(segs, "92")?;

	let bk_seg = find_qualified_segment(segs, "RFF", "Z06")?;
	let bilanzkreis = bk_seg
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+Z06".to_string(),
			feld: "bilanzkreis".to_string(),
		})?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Lieferant,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdBilanzkreiszuordnung(UtilmdBilanzkreiszuordnung {
			malo_id,
			bilanzkreis,
			gueltig_ab,
		}),
	})
}

fn parse_utilmd_clearingliste(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;

	// Collect CCI+CLEARING / CAV groups: each group has malo_id, feld, nb_wert, lf_wert
	let mut eintraege = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "CCI"
			&& segs[i]
				.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "CLEARING")
		{
			// Components: CLEARING:malo:feld
			let comps = &segs[i]
				.elements
				.first()
				.map(|e| e.components.clone())
				.unwrap_or_default();
			let malo_str = comps.get(1).cloned().unwrap_or_default();
			let feld = comps.get(2).cloned().unwrap_or_default();
			let malo_id = MaLoId::new(&malo_str).map_err(|_| CodecFehler::UngueltigerWert {
				segment: "CCI+CLEARING".to_string(),
				feld: "MaLo-ID".to_string(),
				wert: malo_str,
			})?;

			// CAV has nb_wert:lf_wert
			let (nb_wert, lf_wert) = if i + 1 < segs.len() && segs[i + 1].tag == "CAV" {
				let cav_comps = &segs[i + 1]
					.elements
					.first()
					.map(|e| e.components.clone())
					.unwrap_or_default();
				let nb = cav_comps.first().cloned().unwrap_or_default();
				let lf = cav_comps.get(1).cloned();
				(nb, lf)
			} else {
				(String::new(), None)
			};

			eintraege.push(ClearingEintrag {
				malo_id,
				feld,
				nb_wert,
				lf_wert,
			});
			i += 2;
			continue;
		}
		i += 1;
	}

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdClearingliste(UtilmdClearingliste { eintraege }),
	})
}

// ---------------------------------------------------------------------------
// WiM UTILMD serializers
// ---------------------------------------------------------------------------

fn serialize_utilmd_msb_wechsel(
	nachricht: &Nachricht,
	p: &UtilmdMsbWechselAnmeldung,
) -> String {
	let segmente = vec![
		bgm_segment("E03"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.melo_id.as_str()),
		dtm_date_segment("92", &p.wechseldatum),
	];
	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_geraetewechsel(
	nachricht: &Nachricht,
	p: &UtilmdGeraetewechsel,
) -> String {
	let mut segmente = vec![
		bgm_segment("E03"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.melo_id.as_str()),
		dtm_date_segment("92", &p.wechseldatum),
	];
	// Two CCI+Z30 / CAV pairs for old and new device
	segmente.push(Segment {
		tag: "CCI".to_string(),
		elements: vec![Element {
			components: vec!["Z30".to_string()],
		}],
	});
	segmente.push(Segment {
		tag: "CAV".to_string(),
		elements: vec![Element {
			components: vec![p.alte_geraete_nr.clone()],
		}],
	});
	segmente.push(Segment {
		tag: "CCI".to_string(),
		elements: vec![Element {
			components: vec!["Z30".to_string()],
		}],
	});
	segmente.push(Segment {
		tag: "CAV".to_string(),
		elements: vec![Element {
			components: vec![p.neue_geraete_nr.clone()],
		}],
	});
	wrap_utilmd(nachricht, segmente)
}

// ---------------------------------------------------------------------------
// MaBiS UTILMD serializers
// ---------------------------------------------------------------------------

fn serialize_utilmd_bilanzkreiszuordnung(
	nachricht: &Nachricht,
	p: &UtilmdBilanzkreiszuordnung,
) -> String {
	let segmente = vec![
		bgm_segment("E01"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
		dtm_date_segment("92", &p.gueltig_ab),
		rff_segment("Z06", &p.bilanzkreis),
	];
	wrap_utilmd(nachricht, segmente)
}

fn serialize_utilmd_clearingliste(
	nachricht: &Nachricht,
	p: &UtilmdClearingliste,
) -> String {
	let mut segmente = vec![
		bgm_segment("E06"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
	];
	for eintrag in &p.eintraege {
		segmente.push(Segment {
			tag: "CCI".to_string(),
			elements: vec![Element {
				components: vec![
					"CLEARING".to_string(),
					eintrag.malo_id.as_str().to_string(),
					eintrag.feld.clone(),
				],
			}],
		});
		let mut cav_comps = vec![eintrag.nb_wert.clone()];
		if let Some(ref lf) = eintrag.lf_wert {
			cav_comps.push(lf.clone());
		}
		segmente.push(Segment {
			tag: "CAV".to_string(),
			elements: vec![Element {
				components: cav_comps,
			}],
		});
	}
	wrap_utilmd(nachricht, segmente)
}

// ---------------------------------------------------------------------------
// ORDERS dispatcher + parsers
// ---------------------------------------------------------------------------

fn parse_orders(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;

	// Distinguish WerteAnfrage (has DTM+163/164 period) vs Bestellung (has RFF+ON)
	let has_period = find_qualified_segment(segs, "DTM", "163").is_ok();
	let has_rff_on = find_qualified_segment(segs, "RFF", "ON").is_ok();

	if has_rff_on {
		parse_orders_bestellung(segs, absender, empfaenger)
	} else if has_period {
		parse_orders_werte_anfrage(segs, absender, empfaenger)
	} else {
		Err(CodecFehler::UnbekannterNachrichtentyp {
			typ: "ORDERS (cannot disambiguate)".to_string(),
		})
	}
}

fn parse_orders_werte_anfrage(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	let malo_id = extract_malo_id(segs)?;
	let zeitraum_von = extract_date(segs, "163")?;
	let zeitraum_bis = extract_date(segs, "164")?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::OrdersWerteAnfrage(OrdersWerteAnfrage {
			malo_id,
			anfragender: absender,
			zeitraum_von,
			zeitraum_bis,
		}),
	})
}

fn parse_orders_bestellung(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	let melo_id = extract_melo_id(segs)?;
	let rff_on = find_qualified_segment(segs, "RFF", "ON")?;
	let referenz = rff_on
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+ON".to_string(),
			feld: "referenz".to_string(),
		})?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::OrdersBestellung(OrdersBestellung {
			melo_id,
			besteller: absender,
			referenz_angebot: referenz,
		}),
	})
}

// ---------------------------------------------------------------------------
// ORDERS serializers
// ---------------------------------------------------------------------------

fn serialize_orders_werte_anfrage(nachricht: &Nachricht, p: &OrdersWerteAnfrage) -> String {
	let segmente = vec![
		bgm_segment("Z08"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.malo_id.as_str()),
		dtm_date_segment("163", &p.zeitraum_von),
		dtm_date_segment("164", &p.zeitraum_bis),
	];
	wrap_edifact(nachricht, "ORDERS", "D:01B:UN:1.4b", segmente)
}

fn serialize_orders_bestellung(nachricht: &Nachricht, p: &OrdersBestellung) -> String {
	let segmente = vec![
		bgm_segment("Z08"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.melo_id.as_str()),
		rff_segment("ON", &p.referenz_angebot),
	];
	wrap_edifact(nachricht, "ORDERS", "D:01B:UN:1.4b", segmente)
}

// ---------------------------------------------------------------------------
// REQOTE parser + serializer
// ---------------------------------------------------------------------------

fn parse_reqote(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let melo_id = extract_melo_id(segs)?;

	let imd = find_segment(segs, "IMD")?;
	let produkt = imd
		.elements
		.get(2)
		.and_then(|e| e.components.get(3))
		.cloned()
		.unwrap_or_default();

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::Lieferant,
		empfaenger,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::ReqoteAngebotsanfrage(ReqoteAngebotsanfrage {
			melo_id,
			anfragender: absender,
			produkt_beschreibung: produkt,
		}),
	})
}

fn serialize_reqote_angebotsanfrage(
	nachricht: &Nachricht,
	p: &ReqoteAngebotsanfrage,
) -> String {
	let segmente = vec![
		bgm_segment("Z08"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.melo_id.as_str()),
		imd_segment(&p.produkt_beschreibung),
	];
	wrap_edifact(nachricht, "REQOTE", "D:01B:UN:1.3c", segmente)
}

// ---------------------------------------------------------------------------
// QUOTES parser + serializer
// ---------------------------------------------------------------------------

fn parse_quotes(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let melo_id = extract_melo_id(segs)?;

	let imd = find_segment(segs, "IMD")?;
	let produkt = imd
		.elements
		.get(2)
		.and_then(|e| e.components.get(3))
		.cloned()
		.unwrap_or_default();

	let moa = find_qualified_segment(segs, "MOA", "9")?;
	let preis_str = moa
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "MOA+9".to_string(),
			feld: "preis_ct".to_string(),
		})?;
	let preis_ct = preis_str
		.parse::<f64>()
		.map_err(|_| CodecFehler::UngueltigerWert {
			segment: "MOA+9".to_string(),
			feld: "preis_ct".to_string(),
			wert: preis_str.clone(),
		})?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::QuotesAngebot(QuotesAngebot {
			melo_id,
			anbieter: absender,
			preis_ct_pro_monat: preis_ct,
			produkt_beschreibung: produkt,
		}),
	})
}

fn serialize_quotes_angebot(nachricht: &Nachricht, p: &QuotesAngebot) -> String {
	let segmente = vec![
		bgm_segment("Z09"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.melo_id.as_str()),
		imd_segment(&p.produkt_beschreibung),
		moa_segment("9", p.preis_ct_pro_monat),
	];
	wrap_edifact(nachricht, "QUOTES", "D:01B:UN:1.3b", segmente)
}

// ---------------------------------------------------------------------------
// ORDRSP parser + serializer
// ---------------------------------------------------------------------------

fn parse_ordrsp(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let melo_id = extract_melo_id(segs)?;

	// STS+7++{code} has 3 elements: ["7"], [], [code]
	let sts = find_qualified_segment(segs, "STS", "7")?;
	let status_code = sts
		.elements
		.get(2)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();
	let angenommen = status_code == "Z06";

	// FTX+AAO+++{text} has 3 elements: ["AAO"], [], [text]
	let grund = find_qualified_segment(segs, "FTX", "AAO")
		.ok()
		.and_then(|ftx| ftx.elements.get(2))
		.and_then(|e| e.components.first())
		.cloned();

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::OrdrspBestellantwort(OrdrspBestellantwort {
			melo_id,
			angenommen,
			grund,
		}),
	})
}

fn serialize_ordrsp_bestellantwort(nachricht: &Nachricht, p: &OrdrspBestellantwort) -> String {
	let status_code = if p.angenommen { "Z06" } else { "Z08" };
	let mut segmente = vec![
		bgm_segment("Z09"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		ide_segment(p.melo_id.as_str()),
		sts_segment("7", status_code),
	];
	if let Some(ref grund) = p.grund {
		segmente.push(ftx_segment("AAO", grund));
	}
	wrap_edifact(nachricht, "ORDRSP", "D:01B:UN:1.4a", segmente)
}

// ---------------------------------------------------------------------------
// PRICAT parser + serializer
// ---------------------------------------------------------------------------

fn parse_pricat(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	// PRICAT may not have NAD+MR in all cases, use empfaenger if present
	let empfaenger = extract_mp_id(segs, "MR").unwrap_or_else(|_| absender.clone());
	let gueltig_ab = extract_date(segs, "157")?;

	// Collect LIN+PRI+MEA triples
	let mut positionen = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "LIN" {
			let bezeichnung = segs[i]
				.elements
				.get(2)
				.and_then(|e| e.components.first())
				.cloned()
				.unwrap_or_default();

			let mut preis_ct = 0.0;
			let mut einheit = String::new();

			// Look for PRI and MEA after this LIN
			for j in (i + 1)..segs.len() {
				if segs[j].tag == "LIN" {
					break;
				}
				if segs[j].tag == "PRI" {
					preis_ct = segs[j]
						.elements
						.first()
						.and_then(|e| e.components.get(1))
						.and_then(|s| s.parse::<f64>().ok())
						.unwrap_or(0.0);
				}
				if segs[j].tag == "MEA" {
					einheit = segs[j]
						.elements
						.get(2)
						.and_then(|e| e.components.first())
						.cloned()
						.unwrap_or_default();
				}
			}

			positionen.push(PreisPosition {
				bezeichnung,
				preis_ct,
				einheit,
			});
		}
		i += 1;
	}

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::PricatPreisblatt(PricatPreisblatt {
			herausgeber: absender,
			gueltig_ab,
			positionen,
		}),
	})
}

fn serialize_pricat_preisblatt(nachricht: &Nachricht, p: &PricatPreisblatt) -> String {
	let mut segmente = vec![
		bgm_segment("Z33"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		dtm_date_segment("157", &p.gueltig_ab),
	];
	for (idx, pos) in p.positionen.iter().enumerate() {
		segmente.push(lin_segment(idx + 1, &pos.bezeichnung));
		segmente.push(pri_segment(pos.preis_ct));
		segmente.push(mea_segment(&pos.einheit));
	}
	wrap_edifact(nachricht, "PRICAT", "D:01B:UN:2.0e", segmente)
}

// ---------------------------------------------------------------------------
// MaBiS MSCONS parsers
// ---------------------------------------------------------------------------

fn parse_mscons_aggregierte_zeitreihen(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	let bk_seg = find_qualified_segment(segs, "RFF", "Z06")?;
	let bilanzkreis = bk_seg
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+Z06".to_string(),
			feld: "bilanzkreis".to_string(),
		})?;

	// Extract ZeitreihenTyp from STS+7++{code}
	let typ = find_qualified_segment(segs, "STS", "7")
		.ok()
		.and_then(|sts| sts.elements.get(2))
		.and_then(|e| e.components.first())
		.map(|s| match s.as_str() {
			"SLP" => ZeitreihenTyp::SlpSynthese,
			"RLM" => ZeitreihenTyp::RlmLastgang,
			"SUM" => ZeitreihenTyp::Summenzeitreihe,
			"FPL" => ZeitreihenTyp::Fahrplan,
			_ => ZeitreihenTyp::Summenzeitreihe,
		})
		.unwrap_or(ZeitreihenTyp::Summenzeitreihe);

	// Collect QTY+DTM pairs
	let mut zeitreihen = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "QTY"
			&& segs[i]
				.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "220")
		{
			let wert_str = segs[i]
				.elements
				.first()
				.and_then(|e| e.components.get(1))
				.ok_or(CodecFehler::FeldFehlt {
					segment: "QTY+220".to_string(),
					feld: "wert".to_string(),
				})?;
			let wert = wert_str
				.parse::<f64>()
				.map_err(|_| CodecFehler::UngueltigerWert {
					segment: "QTY+220".to_string(),
					feld: "wert".to_string(),
					wert: wert_str.clone(),
				})?;
			let einheit = extract_einheit_from_qty(&segs[i]);

			// Look for DTM+163 after this QTY
			let zeitpunkt = if i + 1 < segs.len()
				&& segs[i + 1].tag == "DTM"
				&& segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.first())
					.is_some_and(|q| q == "163")
			{
				let ts_str = segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.get(1))
					.ok_or(CodecFehler::FeldFehlt {
						segment: "DTM+163".to_string(),
						feld: "zeitpunkt".to_string(),
					})?;
				parse_datetime(ts_str)?
			} else {
				return Err(CodecFehler::SegmentFehlt {
					erwartet: "DTM+163 after QTY".to_string(),
				});
			};

			zeitreihen.push(Messwert {
				zeitpunkt,
				wert,
				einheit,
				status: MesswertStatus::Gemessen,
			});
			i += 2;
			continue;
		}
		i += 1;
	}

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Bilanzkreisverantwortlicher,
		pruef_id: None,
		payload: NachrichtenPayload::MsconsAggregierteZeitreihen(MsconsAggregierteZeitreihen {
			bilanzkreis,
			zeitreihen,
			typ,
		}),
	})
}

fn parse_mscons_mehr_mindermengen(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	let malo_id = extract_malo_from_loc_or_nad_dp(segs)?;

	// QTY+46 = Mehrmenge, QTY+47 = Mindermenge
	let qty_mehr = find_qualified_segment(segs, "QTY", "46")?;
	let mehr_str = qty_mehr
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "QTY+46".to_string(),
			feld: "mehrmenge".to_string(),
		})?;
	let mehrmenge_kwh = mehr_str
		.parse::<f64>()
		.map_err(|_| CodecFehler::UngueltigerWert {
			segment: "QTY+46".to_string(),
			feld: "mehrmenge".to_string(),
			wert: mehr_str.clone(),
		})?;

	let qty_minder = find_qualified_segment(segs, "QTY", "47")?;
	let minder_str = qty_minder
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "QTY+47".to_string(),
			feld: "mindermenge".to_string(),
		})?;
	let mindermenge_kwh = minder_str
		.parse::<f64>()
		.map_err(|_| CodecFehler::UngueltigerWert {
			segment: "QTY+47".to_string(),
			feld: "mindermenge".to_string(),
			wert: minder_str.clone(),
		})?;

	let abrechnungszeitraum_von = extract_date(segs, "163")?;
	let abrechnungszeitraum_bis = extract_date(segs, "164")?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::MsconsMehrMindermengen(MsconsMehrMindermengen {
			malo_id,
			mehrmenge_kwh,
			mindermenge_kwh,
			abrechnungszeitraum_von,
			abrechnungszeitraum_bis,
		}),
	})
}

// ---------------------------------------------------------------------------
// MaBiS MSCONS serializers
// ---------------------------------------------------------------------------

fn serialize_mscons_aggregierte_zeitreihen(
	nachricht: &Nachricht,
	p: &MsconsAggregierteZeitreihen,
) -> String {
	let typ_code = match p.typ {
		ZeitreihenTyp::SlpSynthese => "SLP",
		ZeitreihenTyp::RlmLastgang => "RLM",
		ZeitreihenTyp::Summenzeitreihe => "SUM",
		ZeitreihenTyp::Fahrplan => "FPL",
	};

	let mut segmente = vec![
		bgm_7_segment(),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_segment("Z06", &p.bilanzkreis),
		sts_segment("7", typ_code),
	];

	for mw in &p.zeitreihen {
		segmente.push(qty_segment(mw.wert, &mw.einheit));
		segmente.push(Segment {
			tag: "DTM".to_string(),
			elements: vec![Element {
				components: vec![
					"163".to_string(),
					format!(
						"{:04}{:02}{:02}{:02}{:02}{:02}",
						mw.zeitpunkt.date().year(),
						mw.zeitpunkt.date().month(),
						mw.zeitpunkt.date().day(),
						mw.zeitpunkt.time().hour(),
						mw.zeitpunkt.time().minute(),
						mw.zeitpunkt.time().second(),
					),
					"203".to_string(),
				],
			}],
		});
	}

	wrap_mscons(nachricht, segmente)
}

fn serialize_mscons_mehr_mindermengen(
	nachricht: &Nachricht,
	p: &MsconsMehrMindermengen,
) -> String {
	let segmente = vec![
		bgm_7_segment(),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		loc_segment(p.malo_id.as_str()),
		qty_qualified_segment("46", p.mehrmenge_kwh, "kWh"),
		qty_qualified_segment("47", p.mindermenge_kwh, "kWh"),
		dtm_date_segment("163", &p.abrechnungszeitraum_von),
		dtm_date_segment("164", &p.abrechnungszeitraum_bis),
	];
	wrap_mscons(nachricht, segmente)
}

// ---------------------------------------------------------------------------
// INVOIC parser + serializer
// ---------------------------------------------------------------------------

fn parse_invoic(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender_id = extract_mp_id(segs, "MS")?;
	let empfaenger_id = extract_mp_id(segs, "MR")?;

	let bgm = find_segment(segs, "BGM")?;
	let rechnungsnummer = bgm
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();

	let rechnungsdatum = extract_date(segs, "137")?;

	// PID for rechnungstyp
	let pid_opt = find_qualified_segment(segs, "RFF", "Z13")
		.ok()
		.and_then(|rff| rff.elements.first())
		.and_then(|e| e.components.get(1))
		.and_then(|s| s.parse::<u32>().ok())
		.and_then(PruefIdentifikator::from_code);

	let rechnungstyp = match pid_opt {
		Some(PruefIdentifikator::Netznutzungsrechnung) => RechnungsTyp::Netznutzung,
		Some(PruefIdentifikator::RechnungMessstellenbetrieb) => RechnungsTyp::Messstellenbetrieb,
		_ => RechnungsTyp::Netznutzung,
	};

	// Collect LIN+QTY+MOA+PRI groups
	let mut positionen = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "LIN" {
			let bezeichnung = segs[i]
				.elements
				.get(2)
				.and_then(|e| e.components.first())
				.cloned()
				.unwrap_or_default();

			let mut menge = 0.0;
			let mut einheit = "kWh".to_string();
			let mut einzelpreis_ct: i64 = 0;
			let mut betrag_ct: i64 = 0;

			for j in (i + 1)..segs.len() {
				if segs[j].tag == "LIN" || segs[j].tag == "MOA" && segs[j]
					.elements
					.first()
					.and_then(|e| e.components.first())
					.is_some_and(|q| q == "86")
				{
					// Stop at next LIN or MOA+86 (total)
					if segs[j].tag == "LIN" {
						break;
					}
				}
				if segs[j].tag == "QTY"
					&& segs[j]
						.elements
						.first()
						.and_then(|e| e.components.first())
						.is_some_and(|q| q == "47")
				{
					menge = segs[j]
						.elements
						.first()
						.and_then(|e| e.components.get(1))
						.and_then(|s| s.parse::<f64>().ok())
						.unwrap_or(0.0);
					einheit = extract_einheit_from_qty(&segs[j]);
				}
				if segs[j].tag == "MOA"
					&& segs[j]
						.elements
						.first()
						.and_then(|e| e.components.first())
						.is_some_and(|q| q == "203")
				{
					betrag_ct = segs[j]
						.elements
						.first()
						.and_then(|e| e.components.get(1))
						.and_then(|s| s.parse::<i64>().ok())
						.unwrap_or(0);
				}
				if segs[j].tag == "PRI" {
					einzelpreis_ct = segs[j]
						.elements
						.first()
						.and_then(|e| e.components.get(1))
						.and_then(|s| s.parse::<i64>().ok())
						.unwrap_or(0);
				}
			}

			positionen.push(RechnungsPosition {
				bezeichnung,
				menge,
				einheit,
				einzelpreis_ct,
				betrag_ct,
			});
		}
		i += 1;
	}

	// MOA+86 = Gesamtbetrag
	let gesamtbetrag_ct = find_qualified_segment(segs, "MOA", "86")
		.ok()
		.and_then(|moa| moa.elements.first())
		.and_then(|e| e.components.get(1))
		.and_then(|s| s.parse::<i64>().ok())
		.unwrap_or(0);

	Ok(Nachricht {
		absender: absender_id.clone(),
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: empfaenger_id.clone(),
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: pid_opt,
		payload: NachrichtenPayload::InvoicRechnung(InvoicRechnung {
			rechnungsnummer,
			rechnungsdatum,
			absender: absender_id,
			empfaenger: empfaenger_id,
			positionen,
			gesamtbetrag_ct,
			rechnungstyp,
		}),
	})
}

fn serialize_invoic_rechnung(nachricht: &Nachricht, p: &InvoicRechnung) -> String {
	let pid_code = nachricht
		.pruef_id
		.map(|pid| pid.code().to_string())
		.unwrap_or_default();

	let mut segmente = vec![
		Segment {
			tag: "BGM".to_string(),
			elements: vec![
				Element {
					components: vec!["380".to_string()],
				},
				Element {
					components: vec![p.rechnungsnummer.clone()],
				},
			],
		},
		dtm_date_segment("137", &p.rechnungsdatum),
	];
	if !pid_code.is_empty() {
		segmente.push(rff_z13_segment(&pid_code));
	}
	segmente.push(nad_segment("MS", nachricht.absender.as_str()));
	segmente.push(nad_segment("MR", nachricht.empfaenger.as_str()));

	for (idx, pos) in p.positionen.iter().enumerate() {
		segmente.push(lin_segment(idx + 1, &pos.bezeichnung));
		segmente.push(qty_qualified_segment("47", pos.menge, &pos.einheit));
		segmente.push(moa_i64_segment("203", pos.betrag_ct));
		segmente.push(pri_i64_segment(pos.einzelpreis_ct));
	}
	segmente.push(moa_i64_segment("86", p.gesamtbetrag_ct));

	wrap_edifact(nachricht, "INVOIC", "D:01B:UN:2.8e", segmente)
}

// ---------------------------------------------------------------------------
// REMADV parser + serializer
// ---------------------------------------------------------------------------

fn parse_remadv(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;

	let rff_on = find_qualified_segment(segs, "RFF", "ON")?;
	let referenz = rff_on
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+ON".to_string(),
			feld: "referenz_rechnungsnummer".to_string(),
		})?;

	let zahlungsdatum = extract_date(segs, "171")?;

	let moa = find_qualified_segment(segs, "MOA", "9")?;
	let betrag_str = moa
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "MOA+9".to_string(),
			feld: "betrag_ct".to_string(),
		})?;
	let betrag_ct = betrag_str
		.parse::<i64>()
		.map_err(|_| CodecFehler::UngueltigerWert {
			segment: "MOA+9".to_string(),
			feld: "betrag_ct".to_string(),
			wert: betrag_str.clone(),
		})?;

	// STS+7++{code} has 3 elements: ["7"], [], [code]
	let sts = find_qualified_segment(segs, "STS", "7")?;
	let status_code = sts
		.elements
		.get(2)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();
	let akzeptiert = status_code == "Z06";

	// FTX+AAO+++{text} has 3 elements: ["AAO"], [], [text]
	let ablehnungsgrund = find_qualified_segment(segs, "FTX", "AAO")
		.ok()
		.and_then(|ftx| ftx.elements.get(2))
		.and_then(|e| e.components.first())
		.cloned();

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Lieferant,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RemadvZahlungsavis(RemadvZahlungsavis {
			referenz_rechnungsnummer: referenz,
			zahlungsdatum,
			betrag_ct,
			akzeptiert,
			ablehnungsgrund,
		}),
	})
}

fn serialize_remadv_zahlungsavis(nachricht: &Nachricht, p: &RemadvZahlungsavis) -> String {
	let status_code = if p.akzeptiert { "Z06" } else { "Z08" };
	let mut segmente = vec![
		Segment {
			tag: "BGM".to_string(),
			elements: vec![
				Element {
					components: vec!["481".to_string()],
				},
				Element {
					components: vec!["DOK00001".to_string()],
				},
			],
		},
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_segment("ON", &p.referenz_rechnungsnummer),
		dtm_date_segment("171", &p.zahlungsdatum),
		moa_i64_segment("9", p.betrag_ct),
		sts_segment("7", status_code),
	];
	if let Some(ref grund) = p.ablehnungsgrund {
		segmente.push(ftx_segment("AAO", grund));
	}
	wrap_edifact(nachricht, "REMADV", "D:01B:UN:2.9d", segmente)
}

// ---------------------------------------------------------------------------
// Segment builder helpers
// ---------------------------------------------------------------------------

fn bgm_segment(qualifier: &str) -> Segment {
	Segment {
		tag: "BGM".to_string(),
		elements: vec![
			Element {
				components: vec![qualifier.to_string()],
			},
			Element {
				components: vec!["DOK00001".to_string()],
			},
		],
	}
}

fn bgm_7_segment() -> Segment {
	Segment {
		tag: "BGM".to_string(),
		elements: vec![
			Element {
				components: vec!["7".to_string()],
			},
			Element {
				components: vec!["DOK00001".to_string()],
			},
		],
	}
}

fn dtm_137_segment() -> Segment {
	Segment {
		tag: "DTM".to_string(),
		elements: vec![Element {
			components: vec![
				"137".to_string(),
				"20260101000000".to_string(),
				"303".to_string(),
			],
		}],
	}
}

fn nad_segment(qualifier: &str, mp_id: &str) -> Segment {
	Segment {
		tag: "NAD".to_string(),
		elements: vec![
			Element {
				components: vec![qualifier.to_string()],
			},
			Element {
				components: vec![mp_id.to_string(), String::new(), "293".to_string()],
			},
		],
	}
}

fn ide_segment(malo_id: &str) -> Segment {
	Segment {
		tag: "IDE".to_string(),
		elements: vec![
			Element {
				components: vec!["24".to_string()],
			},
			Element {
				components: vec![malo_id.to_string()],
			},
		],
	}
}

fn dtm_date_segment(qualifier: &str, date: &NaiveDate) -> Segment {
	Segment {
		tag: "DTM".to_string(),
		elements: vec![Element {
			components: vec![
				qualifier.to_string(),
				format!("{:04}{:02}{:02}", date.year(), date.month(), date.day()),
				"102".to_string(),
			],
		}],
	}
}

fn rff_z13_segment(code: &str) -> Segment {
	Segment {
		tag: "RFF".to_string(),
		elements: vec![Element {
			components: vec!["Z13".to_string(), code.to_string()],
		}],
	}
}

fn loc_segment(malo_id: &str) -> Segment {
	Segment {
		tag: "LOC".to_string(),
		elements: vec![
			Element {
				components: vec!["172".to_string()],
			},
			Element {
				components: vec![malo_id.to_string()],
			},
		],
	}
}

fn qty_segment(wert: f64, einheit: &str) -> Segment {
	Segment {
		tag: "QTY".to_string(),
		elements: vec![Element {
			components: vec!["220".to_string(), format!("{wert}"), einheit.to_string()],
		}],
	}
}

fn rff_segment(qualifier: &str, value: &str) -> Segment {
	Segment {
		tag: "RFF".to_string(),
		elements: vec![Element {
			components: vec![qualifier.to_string(), value.to_string()],
		}],
	}
}

fn imd_segment(beschreibung: &str) -> Segment {
	Segment {
		tag: "IMD".to_string(),
		elements: vec![
			Element {
				components: vec!["F".to_string()],
			},
			Element {
				components: vec![],
			},
			Element {
				components: vec![
					String::new(),
					String::new(),
					String::new(),
					beschreibung.to_string(),
				],
			},
		],
	}
}

fn moa_segment(qualifier: &str, wert: f64) -> Segment {
	Segment {
		tag: "MOA".to_string(),
		elements: vec![Element {
			components: vec![qualifier.to_string(), format!("{wert}")],
		}],
	}
}

fn moa_i64_segment(qualifier: &str, wert: i64) -> Segment {
	Segment {
		tag: "MOA".to_string(),
		elements: vec![Element {
			components: vec![qualifier.to_string(), wert.to_string()],
		}],
	}
}

fn sts_segment(qualifier: &str, code: &str) -> Segment {
	Segment {
		tag: "STS".to_string(),
		elements: vec![
			Element {
				components: vec![qualifier.to_string()],
			},
			Element {
				components: vec![],
			},
			Element {
				components: vec![code.to_string()],
			},
		],
	}
}

fn ftx_segment(qualifier: &str, text: &str) -> Segment {
	Segment {
		tag: "FTX".to_string(),
		elements: vec![
			Element {
				components: vec![qualifier.to_string()],
			},
			Element {
				components: vec![],
			},
			Element {
				components: vec![text.to_string()],
			},
		],
	}
}

fn lin_segment(nr: usize, bezeichnung: &str) -> Segment {
	Segment {
		tag: "LIN".to_string(),
		elements: vec![
			Element {
				components: vec![nr.to_string()],
			},
			Element {
				components: vec![],
			},
			Element {
				components: vec![bezeichnung.to_string()],
			},
		],
	}
}

fn pri_segment(preis_ct: f64) -> Segment {
	Segment {
		tag: "PRI".to_string(),
		elements: vec![Element {
			components: vec!["INV".to_string(), format!("{preis_ct}")],
		}],
	}
}

fn pri_i64_segment(preis_ct: i64) -> Segment {
	Segment {
		tag: "PRI".to_string(),
		elements: vec![Element {
			components: vec!["INV".to_string(), preis_ct.to_string()],
		}],
	}
}

fn mea_segment(einheit: &str) -> Segment {
	Segment {
		tag: "MEA".to_string(),
		elements: vec![
			Element {
				components: vec!["AAE".to_string()],
			},
			Element {
				components: vec!["AAF".to_string()],
			},
			Element {
				components: vec![einheit.to_string()],
			},
		],
	}
}

fn qty_qualified_segment(qualifier: &str, wert: f64, einheit: &str) -> Segment {
	Segment {
		tag: "QTY".to_string(),
		elements: vec![Element {
			components: vec![qualifier.to_string(), format!("{wert}"), einheit.to_string()],
		}],
	}
}

fn wrap_edifact(
	nachricht: &Nachricht,
	typ: &str,
	version: &str,
	segmente: Vec<Segment>,
) -> String {
	let interchange = Interchange {
		sender: nachricht.absender.as_str().to_string(),
		empfaenger: nachricht.empfaenger.as_str().to_string(),
		datum: "20260101".to_string(),
		nachrichten: vec![EdifactNachricht {
			typ: typ.to_string(),
			version: version.to_string(),
			segmente,
		}],
	};
	serialize_interchange(&interchange)
}

fn wrap_utilmd(nachricht: &Nachricht, segmente: Vec<Segment>) -> String {
	let interchange = Interchange {
		sender: nachricht.absender.as_str().to_string(),
		empfaenger: nachricht.empfaenger.as_str().to_string(),
		datum: "20260101".to_string(),
		nachrichten: vec![EdifactNachricht {
			typ: "UTILMD".to_string(),
			version: "D:11A:UN:S2.1".to_string(),
			segmente,
		}],
	};
	serialize_interchange(&interchange)
}

fn wrap_mscons(nachricht: &Nachricht, segmente: Vec<Segment>) -> String {
	let interchange = Interchange {
		sender: nachricht.absender.as_str().to_string(),
		empfaenger: nachricht.empfaenger.as_str().to_string(),
		datum: "20260101".to_string(),
		nachrichten: vec![EdifactNachricht {
			typ: "MSCONS".to_string(),
			version: "D:04B:UN:2.4c".to_string(),
			segmente,
		}],
	};
	serialize_interchange(&interchange)
}

// ---------------------------------------------------------------------------
// MPES parsers + serializers
// ---------------------------------------------------------------------------

fn parse_utilmd_anmeldung_erzeugung(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_from_loc_or_nad_dp(segs)?;

	// CCI+Z30 for eeg_anlage
	let eeg_anlage = segs.iter().any(|s| {
		s.tag == "CCI"
			&& s.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "Z30")
			&& s.elements
				.first()
				.and_then(|e| e.components.get(2))
				.is_some_and(|v| v == "true")
	});

	// QTY+220 for installierte_leistung_kw
	let qty = find_qualified_segment(segs, "QTY", "220")?;
	let leistung_str = qty
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "QTY+220".to_string(),
			feld: "installierte_leistung".to_string(),
		})?;
	let installierte_leistung_kw =
		leistung_str
			.parse::<f64>()
			.map_err(|_| CodecFehler::UngueltigerWert {
				segment: "QTY+220".to_string(),
				feld: "installierte_leistung".to_string(),
				wert: leistung_str.clone(),
			})?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::BetreiberErzeugungsanlage,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdAnmeldungErzeugung(UtilmdAnmeldungErzeugung {
			malo_id,
			anlagenbetreiber: absender,
			eeg_anlage,
			installierte_leistung_kw,
		}),
	})
}

fn serialize_utilmd_anmeldung_erzeugung(
	nachricht: &Nachricht,
	p: &UtilmdAnmeldungErzeugung,
) -> String {
	let eeg_str = if p.eeg_anlage { "true" } else { "false" };
	let segmente = vec![
		bgm_segment("E01"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		loc_segment(p.malo_id.as_str()),
		Segment {
			tag: "CCI".to_string(),
			elements: vec![Element {
				components: vec!["Z30".to_string(), String::new(), eeg_str.to_string()],
			}],
		},
		qty_segment(p.installierte_leistung_kw, "kW"),
	];
	wrap_utilmd(nachricht, segmente)
}

fn parse_mscons_einspeise_messwerte(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	let malo_id = extract_malo_from_loc_or_nad_dp(segs)?;

	// Collect QTY+DTM pairs
	let mut werte = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "QTY"
			&& segs[i]
				.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "220")
		{
			let wert_str = segs[i]
				.elements
				.first()
				.and_then(|e| e.components.get(1))
				.ok_or(CodecFehler::FeldFehlt {
					segment: "QTY+220".to_string(),
					feld: "wert".to_string(),
				})?;
			let wert = wert_str
				.parse::<f64>()
				.map_err(|_| CodecFehler::UngueltigerWert {
					segment: "QTY+220".to_string(),
					feld: "wert".to_string(),
					wert: wert_str.clone(),
				})?;
			let einheit = extract_einheit_from_qty(&segs[i]);

			let zeitpunkt = if i + 1 < segs.len()
				&& segs[i + 1].tag == "DTM"
				&& segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.first())
					.is_some_and(|q| q == "163")
			{
				let ts_str = segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.get(1))
					.ok_or(CodecFehler::FeldFehlt {
						segment: "DTM+163".to_string(),
						feld: "zeitpunkt".to_string(),
					})?;
				parse_datetime(ts_str)?
			} else {
				return Err(CodecFehler::SegmentFehlt {
					erwartet: "DTM+163 after QTY".to_string(),
				});
			};

			werte.push(Messwert {
				zeitpunkt,
				wert,
				einheit,
				status: MesswertStatus::Gemessen,
			});
			i += 2;
			continue;
		}
		i += 1;
	}

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::MsconsEinspeiseMesswerte(MsconsEinspeiseMesswerte {
			malo_id,
			werte,
		}),
	})
}

fn serialize_mscons_einspeise_messwerte(
	nachricht: &Nachricht,
	p: &MsconsEinspeiseMesswerte,
) -> String {
	let mut segmente = vec![
		bgm_7_segment(),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		loc_segment(p.malo_id.as_str()),
	];

	for mw in &p.werte {
		segmente.push(qty_segment(mw.wert, &mw.einheit));
		segmente.push(Segment {
			tag: "DTM".to_string(),
			elements: vec![Element {
				components: vec![
					"163".to_string(),
					format!(
						"{:04}{:02}{:02}{:02}{:02}{:02}",
						mw.zeitpunkt.date().year(),
						mw.zeitpunkt.date().month(),
						mw.zeitpunkt.date().day(),
						mw.zeitpunkt.time().hour(),
						mw.zeitpunkt.time().minute(),
						mw.zeitpunkt.time().second(),
					),
					"203".to_string(),
				],
			}],
		});
	}

	wrap_mscons(nachricht, segmente)
}

// ---------------------------------------------------------------------------
// §14a parsers + serializers
// ---------------------------------------------------------------------------

fn parse_utilmd_steuerbare_verbrauchseinrichtung(
	segs: &[Segment],
) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_from_loc_or_nad_dp(segs)?;

	// CCI+Z30 with geraetetyp
	let geraetetyp_str = segs
		.iter()
		.find(|s| {
			s.tag == "CCI"
				&& s.elements
					.first()
					.and_then(|e| e.components.first())
					.is_some_and(|q| q == "Z30")
		})
		.and_then(|s| s.elements.first())
		.and_then(|e| e.components.get(2))
		.cloned()
		.unwrap_or_default();

	let geraetetyp = match geraetetyp_str.as_str() {
		"Waermepumpe" => SteuerbarerGeraetetyp::Waermepumpe,
		"Wallbox" => SteuerbarerGeraetetyp::Wallbox,
		"Speicher" => SteuerbarerGeraetetyp::Speicher,
		other => {
			let s = other.strip_prefix("Sonstiges:").unwrap_or(other);
			SteuerbarerGeraetetyp::Sonstiges(s.to_string())
		}
	};

	// QTY+220 for max_leistung_kw
	let qty = find_qualified_segment(segs, "QTY", "220")?;
	let leistung_str = qty
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "QTY+220".to_string(),
			feld: "max_leistung".to_string(),
		})?;
	let max_leistung_kw =
		leistung_str
			.parse::<f64>()
			.map_err(|_| CodecFehler::UngueltigerWert {
				segment: "QTY+220".to_string(),
				feld: "max_leistung".to_string(),
				wert: leistung_str.clone(),
			})?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdSteuerbareVerbrauchseinrichtung(
			UtilmdSteuerbareVerbrauchseinrichtung {
				malo_id,
				geraetetyp,
				max_leistung_kw,
			},
		),
	})
}

fn serialize_utilmd_steuerbare_verbrauchseinrichtung(
	nachricht: &Nachricht,
	p: &UtilmdSteuerbareVerbrauchseinrichtung,
) -> String {
	let geraetetyp_str = match &p.geraetetyp {
		SteuerbarerGeraetetyp::Waermepumpe => "Waermepumpe".to_string(),
		SteuerbarerGeraetetyp::Wallbox => "Wallbox".to_string(),
		SteuerbarerGeraetetyp::Speicher => "Speicher".to_string(),
		SteuerbarerGeraetetyp::Sonstiges(s) => format!("Sonstiges:{s}"),
	};

	let segmente = vec![
		bgm_segment("E01"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		loc_segment(p.malo_id.as_str()),
		Segment {
			tag: "CCI".to_string(),
			elements: vec![Element {
				components: vec!["Z30".to_string(), String::new(), geraetetyp_str],
			}],
		},
		qty_segment(p.max_leistung_kw, "kW"),
	];
	wrap_utilmd(nachricht, segmente)
}

fn parse_cls_steuersignal_msg(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_from_loc_or_nad_dp(segs)?;

	// STS+7++{code} for Steuerungsart
	let sts = find_qualified_segment(segs, "STS", "7")?;
	let status_code = sts
		.elements
		.get(2)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();

	let steuerung = if status_code == "Z06" {
		Steuerungsart::Freigabe
	} else if status_code == "Z08" {
		Steuerungsart::Abschaltung
	} else if status_code.starts_with("DIM:") {
		let prozent = status_code[4..].parse::<u8>().unwrap_or(0);
		Steuerungsart::Dimmung { prozent }
	} else {
		Steuerungsart::Freigabe
	};

	// DTM+163 for zeitpunkt
	let dtm = find_qualified_segment(segs, "DTM", "163")?;
	let ts_str = dtm
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "DTM+163".to_string(),
			feld: "zeitpunkt".to_string(),
		})?;
	let zeitpunkt = parse_datetime(ts_str)?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Messstellenbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::ClsSteuersignal(ClsSteuersignal {
			malo_id,
			steuerung,
			zeitpunkt,
		}),
	})
}

fn serialize_cls_steuersignal(nachricht: &Nachricht, p: &ClsSteuersignal) -> String {
	let steuerung_code = match &p.steuerung {
		Steuerungsart::Freigabe => "Z06".to_string(),
		Steuerungsart::Abschaltung => "Z08".to_string(),
		Steuerungsart::Dimmung { prozent } => format!("DIM:{prozent}"),
	};

	let segmente = vec![
		bgm_segment("E04"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		loc_segment(p.malo_id.as_str()),
		sts_segment("7", &steuerung_code),
		Segment {
			tag: "DTM".to_string(),
			elements: vec![Element {
				components: vec![
					"163".to_string(),
					format!(
						"{:04}{:02}{:02}{:02}{:02}{:02}",
						p.zeitpunkt.date().year(),
						p.zeitpunkt.date().month(),
						p.zeitpunkt.date().day(),
						p.zeitpunkt.time().hour(),
						p.zeitpunkt.time().minute(),
						p.zeitpunkt.time().second(),
					),
					"203".to_string(),
				],
			}],
		},
	];
	wrap_utilmd(nachricht, segmente)
}

// ---------------------------------------------------------------------------
// Gas parsers + serializers
// ---------------------------------------------------------------------------

fn parse_nominierung_msg(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	let bk_seg = find_qualified_segment(segs, "RFF", "Z06")?;
	let bilanzkreis = bk_seg
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+Z06".to_string(),
			feld: "bilanzkreis".to_string(),
		})?;

	let zeitreihe_soll = extract_qty_dtm_timeseries(segs)?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Bilanzkreisverantwortlicher,
		empfaenger,
		empfaenger_rolle: MarktRolle::Marktgebietsverantwortlicher,
		pruef_id: None,
		payload: NachrichtenPayload::Nominierung(Nominierung {
			bilanzkreis,
			zeitreihe_soll,
		}),
	})
}

fn serialize_nominierung(nachricht: &Nachricht, p: &Nominierung) -> String {
	let mut segmente = vec![
		bgm_7_segment(),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_segment("Z06", &p.bilanzkreis),
	];

	for mw in &p.zeitreihe_soll {
		segmente.push(qty_segment(mw.wert, &mw.einheit));
		segmente.push(Segment {
			tag: "DTM".to_string(),
			elements: vec![Element {
				components: vec![
					"163".to_string(),
					format!(
						"{:04}{:02}{:02}{:02}{:02}{:02}",
						mw.zeitpunkt.date().year(),
						mw.zeitpunkt.date().month(),
						mw.zeitpunkt.date().day(),
						mw.zeitpunkt.time().hour(),
						mw.zeitpunkt.time().minute(),
						mw.zeitpunkt.time().second(),
					),
					"203".to_string(),
				],
			}],
		});
	}

	wrap_mscons(nachricht, segmente)
}

fn parse_nominierung_bestaetigung_msg(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	let bk_seg = find_qualified_segment(segs, "RFF", "Z06")?;
	let bilanzkreis = bk_seg
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+Z06".to_string(),
			feld: "bilanzkreis".to_string(),
		})?;

	// STS+7++{code} for matching_ergebnis
	let sts = find_qualified_segment(segs, "STS", "7")?;
	let status_code = sts
		.elements
		.get(2)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();

	let matching_ergebnis = if status_code == "Z06" {
		MatchingErgebnis::Bestaetigt
	} else if status_code.starts_with("TEIL:") {
		let menge = status_code[5..].parse::<f64>().unwrap_or(0.0);
		MatchingErgebnis::TeilweiseBestaetigt {
			bestaetigte_menge_kwh: menge,
		}
	} else {
		// Z08 = Abgelehnt, get grund from FTX
		let grund = find_qualified_segment(segs, "FTX", "AAO")
			.ok()
			.and_then(|ftx| ftx.elements.get(2))
			.and_then(|e| e.components.first())
			.cloned()
			.unwrap_or_else(|| "unbekannt".to_string());
		MatchingErgebnis::Abgelehnt { grund }
	};

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Marktgebietsverantwortlicher,
		empfaenger,
		empfaenger_rolle: MarktRolle::Bilanzkreisverantwortlicher,
		pruef_id: None,
		payload: NachrichtenPayload::NominierungBestaetigung(NominierungBestaetigung {
			bilanzkreis,
			matching_ergebnis,
		}),
	})
}

fn serialize_nominierung_bestaetigung(
	nachricht: &Nachricht,
	p: &NominierungBestaetigung,
) -> String {
	let mut segmente = vec![
		bgm_7_segment(),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_segment("Z06", &p.bilanzkreis),
	];

	match &p.matching_ergebnis {
		MatchingErgebnis::Bestaetigt => {
			segmente.push(sts_segment("7", "Z06"));
		}
		MatchingErgebnis::TeilweiseBestaetigt {
			bestaetigte_menge_kwh,
		} => {
			segmente.push(sts_segment("7", &format!("TEIL:{bestaetigte_menge_kwh}")));
		}
		MatchingErgebnis::Abgelehnt { grund } => {
			segmente.push(sts_segment("7", "Z08"));
			segmente.push(ftx_segment("AAO", grund));
		}
	}

	wrap_mscons(nachricht, segmente)
}

fn parse_renominierung_msg(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	let bk_seg = find_qualified_segment(segs, "RFF", "Z06")?;
	let bilanzkreis = bk_seg
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+Z06".to_string(),
			feld: "bilanzkreis".to_string(),
		})?;

	let zeitreihe_soll = extract_qty_dtm_timeseries(segs)?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Bilanzkreisverantwortlicher,
		empfaenger,
		empfaenger_rolle: MarktRolle::Marktgebietsverantwortlicher,
		pruef_id: None,
		payload: NachrichtenPayload::Renominierung(Renominierung {
			bilanzkreis,
			zeitreihe_soll,
		}),
	})
}

fn serialize_renominierung(nachricht: &Nachricht, p: &Renominierung) -> String {
	let mut segmente = vec![
		bgm_7_segment(),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_segment("Z06", &p.bilanzkreis),
		rff_segment("ACE", "RENOM"),
	];

	for mw in &p.zeitreihe_soll {
		segmente.push(qty_segment(mw.wert, &mw.einheit));
		segmente.push(Segment {
			tag: "DTM".to_string(),
			elements: vec![Element {
				components: vec![
					"163".to_string(),
					format!(
						"{:04}{:02}{:02}{:02}{:02}{:02}",
						mw.zeitpunkt.date().year(),
						mw.zeitpunkt.date().month(),
						mw.zeitpunkt.date().day(),
						mw.zeitpunkt.time().hour(),
						mw.zeitpunkt.time().minute(),
						mw.zeitpunkt.time().second(),
					),
					"203".to_string(),
				],
			}],
		});
	}

	wrap_mscons(nachricht, segmente)
}

fn parse_mscons_brennwert(
	segs: &[Segment],
	absender: MarktpartnerId,
	empfaenger: MarktpartnerId,
) -> Result<Nachricht, CodecFehler> {
	// MOA+BRENNWERT for brennwert_kwh_per_m3
	let moa_bw = find_qualified_segment(segs, "MOA", "BRENNWERT")?;
	let bw_str = moa_bw
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "MOA+BRENNWERT".to_string(),
			feld: "brennwert".to_string(),
		})?;
	let brennwert_kwh_per_m3 =
		bw_str
			.parse::<f64>()
			.map_err(|_| CodecFehler::UngueltigerWert {
				segment: "MOA+BRENNWERT".to_string(),
				feld: "brennwert".to_string(),
				wert: bw_str.clone(),
			})?;

	// MOA+ZUSTAND for zustandszahl
	let moa_zs = find_qualified_segment(segs, "MOA", "ZUSTAND")?;
	let zs_str = moa_zs
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "MOA+ZUSTAND".to_string(),
			feld: "zustandszahl".to_string(),
		})?;
	let zustandszahl = zs_str
		.parse::<f64>()
		.map_err(|_| CodecFehler::UngueltigerWert {
			segment: "MOA+ZUSTAND".to_string(),
			feld: "zustandszahl".to_string(),
			wert: zs_str.clone(),
		})?;

	// RFF+Z13 for netzgebiet (repurposed, no PID)
	let netzgebiet = find_qualified_segment(segs, "RFF", "Z13")
		.ok()
		.and_then(|rff| rff.elements.first())
		.and_then(|e| e.components.get(1))
		.cloned()
		.unwrap_or_default();

	let gueltig_ab = extract_date(segs, "163")?;
	let gueltig_bis = extract_date(segs, "164")?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Fernleitungsnetzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::MsconsBrennwert(MsconsBrennwert {
			netzgebiet,
			brennwert_kwh_per_m3,
			zustandszahl,
			gueltig_ab,
			gueltig_bis,
		}),
	})
}

fn serialize_mscons_brennwert(nachricht: &Nachricht, p: &MsconsBrennwert) -> String {
	let segmente = vec![
		bgm_7_segment(),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_z13_segment(&p.netzgebiet),
		moa_segment("BRENNWERT", p.brennwert_kwh_per_m3),
		moa_segment("ZUSTAND", p.zustandszahl),
		dtm_date_segment("163", &p.gueltig_ab),
		dtm_date_segment("164", &p.gueltig_bis),
	];
	wrap_mscons(nachricht, segmente)
}

fn parse_utilmd_ausspeisepunkt(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;
	let malo_id = extract_malo_from_loc_or_nad_dp(segs)?;

	// Two NAD+DP segments: first = nb, second = fnb
	let dp_segments: Vec<&Segment> = segs
		.iter()
		.filter(|s| {
			s.tag == "NAD"
				&& s.elements
					.first()
					.and_then(|e| e.components.first())
					.is_some_and(|q| q == "DP")
		})
		.collect();

	let nb = dp_segments
		.first()
		.and_then(|s| s.elements.get(1))
		.and_then(|e| e.components.first())
		.ok_or(CodecFehler::FeldFehlt {
			segment: "NAD+DP".to_string(),
			feld: "nb".to_string(),
		})?;
	let nb_id = MarktpartnerId::new(nb).map_err(|_| CodecFehler::UngueltigerWert {
		segment: "NAD+DP".to_string(),
		feld: "nb".to_string(),
		wert: nb.clone(),
	})?;

	let fnb = dp_segments
		.get(1)
		.and_then(|s| s.elements.get(1))
		.and_then(|e| e.components.first())
		.ok_or(CodecFehler::FeldFehlt {
			segment: "NAD+DP".to_string(),
			feld: "fnb".to_string(),
		})?;
	let fnb_id = MarktpartnerId::new(fnb).map_err(|_| CodecFehler::UngueltigerWert {
		segment: "NAD+DP".to_string(),
		feld: "fnb".to_string(),
		wert: fnb.clone(),
	})?;

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Fernleitungsnetzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::UtilmdAusspeisepunkt(UtilmdAusspeisepunkt {
			malo_id,
			nb: nb_id,
			fnb: fnb_id,
		}),
	})
}

fn serialize_utilmd_ausspeisepunkt(nachricht: &Nachricht, p: &UtilmdAusspeisepunkt) -> String {
	let segmente = vec![
		bgm_segment("E01"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		loc_segment(p.malo_id.as_str()),
		nad_segment("DP", p.nb.as_str()),
		nad_segment("DP", p.fnb.as_str()),
	];
	wrap_utilmd(nachricht, segmente)
}

// ---------------------------------------------------------------------------
// Querschnitt parsers + serializers
// ---------------------------------------------------------------------------

fn parse_iftsta(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;

	// RFF+ACE for referenz_nachricht
	let rff = find_qualified_segment(segs, "RFF", "ACE")?;
	let referenz_nachricht = rff
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+ACE".to_string(),
			feld: "referenz_nachricht".to_string(),
		})?;

	// STS+7++{code}
	let sts = find_qualified_segment(segs, "STS", "7")?;
	let status_code = sts
		.elements
		.get(2)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();

	// FTX+AAO+++{text}
	let beschreibung = find_qualified_segment(segs, "FTX", "AAO")
		.ok()
		.and_then(|ftx| ftx.elements.get(2))
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::IftstaStatusmeldung(IftstaStatusmeldung {
			referenz_nachricht,
			status_code,
			beschreibung,
		}),
	})
}

fn serialize_iftsta_statusmeldung(
	nachricht: &Nachricht,
	p: &IftstaStatusmeldung,
) -> String {
	let segmente = vec![
		bgm_segment("23"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_segment("ACE", &p.referenz_nachricht),
		sts_segment("7", &p.status_code),
		ftx_segment("AAO", &p.beschreibung),
	];
	wrap_edifact(nachricht, "IFTSTA", "D:01B:UN:2.0g", segmente)
}

fn parse_partin(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;

	// NAD+DP for mp_id
	let dp = find_qualified_segment(segs, "NAD", "DP")?;
	let mp_id_str = dp
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.ok_or(CodecFehler::FeldFehlt {
			segment: "NAD+DP".to_string(),
			feld: "mp_id".to_string(),
		})?;
	let mp_id = MarktpartnerId::new(mp_id_str).map_err(|_| CodecFehler::UngueltigerWert {
		segment: "NAD+DP".to_string(),
		feld: "mp_id".to_string(),
		wert: mp_id_str.clone(),
	})?;

	// CTA+IC+:{name}
	let cta = find_qualified_segment(segs, "CTA", "IC")?;
	let name = cta
		.elements
		.get(1)
		.and_then(|e| e.components.get(1))
		.cloned()
		.unwrap_or_default();

	// RFF+ACD:{rolle}
	let rff = find_qualified_segment(segs, "RFF", "ACD")?;
	let rolle = rff
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.unwrap_or_default();

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::PartinMarktpartner(PartinMarktpartner {
			mp_id,
			name,
			rolle,
		}),
	})
}

fn serialize_partin_marktpartner(nachricht: &Nachricht, p: &PartinMarktpartner) -> String {
	let segmente = vec![
		bgm_segment("Z34"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		nad_segment("DP", p.mp_id.as_str()),
		Segment {
			tag: "CTA".to_string(),
			elements: vec![
				Element {
					components: vec!["IC".to_string()],
				},
				Element {
					components: vec![String::new(), p.name.clone()],
				},
			],
		},
		rff_segment("ACD", &p.rolle),
	];
	wrap_edifact(nachricht, "PARTIN", "D:01B:UN:1.0e", segmente)
}

fn parse_utilts(segs: &[Segment]) -> Result<Nachricht, CodecFehler> {
	let absender = extract_mp_id(segs, "MS")?;
	let empfaenger = extract_mp_id(segs, "MR")?;

	// RFF+Z13 for formel_id
	let rff = find_qualified_segment(segs, "RFF", "Z13")?;
	let formel_id = rff
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.cloned()
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+Z13".to_string(),
			feld: "formel_id".to_string(),
		})?;

	// IMD+F++:::{bezeichnung}
	let imd = find_segment(segs, "IMD")?;
	let bezeichnung = imd
		.elements
		.get(2)
		.and_then(|e| e.components.get(3))
		.cloned()
		.unwrap_or_default();

	// CCI+Z30++{zeitreihen_typ}
	let cci = find_qualified_segment(segs, "CCI", "Z30")?;
	let zeitreihen_typ = cci
		.elements
		.first()
		.and_then(|e| e.components.get(2))
		.cloned()
		.unwrap_or_default();

	Ok(Nachricht {
		absender,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtiltsZaehlzeitdefinition(UtiltsZaehlzeitdefinition {
			formel_id,
			bezeichnung,
			zeitreihen_typ,
		}),
	})
}

fn serialize_utilts_zaehlzeitdefinition(
	nachricht: &Nachricht,
	p: &UtiltsZaehlzeitdefinition,
) -> String {
	let segmente = vec![
		bgm_segment("Z08"),
		dtm_137_segment(),
		nad_segment("MS", nachricht.absender.as_str()),
		nad_segment("MR", nachricht.empfaenger.as_str()),
		rff_z13_segment(&p.formel_id),
		imd_segment(&p.bezeichnung),
		Segment {
			tag: "CCI".to_string(),
			elements: vec![Element {
				components: vec!["Z30".to_string(), String::new(), p.zeitreihen_typ.clone()],
			}],
		},
	];
	wrap_edifact(nachricht, "UTILTS", "D:01B:UN:1.1e", segmente)
}

// ---------------------------------------------------------------------------
// Shared time-series extraction helper
// ---------------------------------------------------------------------------

fn extract_qty_dtm_timeseries(segs: &[Segment]) -> Result<Vec<Messwert>, CodecFehler> {
	let mut werte = Vec::new();
	let mut i = 0;
	while i < segs.len() {
		if segs[i].tag == "QTY"
			&& segs[i]
				.elements
				.first()
				.and_then(|e| e.components.first())
				.is_some_and(|q| q == "220")
		{
			let wert_str = segs[i]
				.elements
				.first()
				.and_then(|e| e.components.get(1))
				.ok_or(CodecFehler::FeldFehlt {
					segment: "QTY+220".to_string(),
					feld: "wert".to_string(),
				})?;
			let wert = wert_str
				.parse::<f64>()
				.map_err(|_| CodecFehler::UngueltigerWert {
					segment: "QTY+220".to_string(),
					feld: "wert".to_string(),
					wert: wert_str.clone(),
				})?;
			let einheit = extract_einheit_from_qty(&segs[i]);

			let zeitpunkt = if i + 1 < segs.len()
				&& segs[i + 1].tag == "DTM"
				&& segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.first())
					.is_some_and(|q| q == "163")
			{
				let ts_str = segs[i + 1]
					.elements
					.first()
					.and_then(|e| e.components.get(1))
					.ok_or(CodecFehler::FeldFehlt {
						segment: "DTM+163".to_string(),
						feld: "zeitpunkt".to_string(),
					})?;
				parse_datetime(ts_str)?
			} else {
				return Err(CodecFehler::SegmentFehlt {
					erwartet: "DTM+163 after QTY".to_string(),
				});
			};

			werte.push(Messwert {
				zeitpunkt,
				wert,
				einheit,
				status: MesswertStatus::Gemessen,
			});
			i += 2;
			continue;
		}
		i += 1;
	}
	Ok(werte)
}

// ---------------------------------------------------------------------------
// Extraction helpers
// ---------------------------------------------------------------------------

fn extract_mp_id(segs: &[Segment], qualifier: &str) -> Result<MarktpartnerId, CodecFehler> {
	let nad = find_qualified_segment(segs, "NAD", qualifier)?;
	let id_str = nad
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.ok_or(CodecFehler::FeldFehlt {
			segment: format!("NAD+{qualifier}"),
			feld: "MP-ID".to_string(),
		})?;
	MarktpartnerId::new(id_str).map_err(|_| CodecFehler::UngueltigerWert {
		segment: format!("NAD+{qualifier}"),
		feld: "MP-ID".to_string(),
		wert: id_str.clone(),
	})
}

fn extract_malo_id(segs: &[Segment]) -> Result<MaLoId, CodecFehler> {
	let ide = find_qualified_segment(segs, "IDE", "24")?;
	let malo_str = ide
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.ok_or(CodecFehler::FeldFehlt {
			segment: "IDE+24".to_string(),
			feld: "MaLo-ID".to_string(),
		})?;
	MaLoId::new(malo_str).map_err(|_| CodecFehler::UngueltigerWert {
		segment: "IDE+24".to_string(),
		feld: "MaLo-ID".to_string(),
		wert: malo_str.clone(),
	})
}

fn extract_melo_id(segs: &[Segment]) -> Result<MeLoId, CodecFehler> {
	let ide = find_qualified_segment(segs, "IDE", "24")?;
	let melo_str = ide
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.ok_or(CodecFehler::FeldFehlt {
			segment: "IDE+24".to_string(),
			feld: "MeLo-ID".to_string(),
		})?;
	MeLoId::new(melo_str).map_err(|_| CodecFehler::UngueltigerWert {
		segment: "IDE+24".to_string(),
		feld: "MeLo-ID".to_string(),
		wert: melo_str.clone(),
	})
}

fn extract_malo_from_loc_or_nad_dp(segs: &[Segment]) -> Result<MaLoId, CodecFehler> {
	// Try LOC+172 first, then NAD+DP
	if let Ok(loc) = find_qualified_segment(segs, "LOC", "172") {
		let malo_str = loc
			.elements
			.get(1)
			.and_then(|e| e.components.first())
			.ok_or(CodecFehler::FeldFehlt {
				segment: "LOC+172".to_string(),
				feld: "MaLo-ID".to_string(),
			})?;
		return MaLoId::new(malo_str).map_err(|_| CodecFehler::UngueltigerWert {
			segment: "LOC+172".to_string(),
			feld: "MaLo-ID".to_string(),
			wert: malo_str.clone(),
		});
	}
	Err(CodecFehler::SegmentFehlt {
		erwartet: "LOC+172 or NAD+DP".to_string(),
	})
}

fn extract_date(segs: &[Segment], qualifier: &str) -> Result<NaiveDate, CodecFehler> {
	let dtm = find_qualified_segment(segs, "DTM", qualifier)?;
	let datum_str = dtm
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: format!("DTM+{qualifier}"),
			feld: "datum".to_string(),
		})?;
	NaiveDate::parse_from_str(datum_str, "%Y%m%d").map_err(|_| CodecFehler::UngueltigesFormat {
		segment: format!("DTM+{qualifier}"),
		feld: "datum".to_string(),
		erwartet: "YYYYMMDD (format 102)".to_string(),
	})
}

fn extract_einheit_from_qty(qty: &Segment) -> String {
	qty.elements
		.first()
		.and_then(|e| e.components.get(2))
		.cloned()
		.unwrap_or_else(|| "kWh".to_string())
}

fn parse_datetime(s: &str) -> Result<NaiveDateTime, CodecFehler> {
	// Try format 203 (YYYYMMDDHHmmss) first, then 102 (YYYYMMDD)
	if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M%S") {
		return Ok(dt);
	}
	if let Ok(d) = NaiveDate::parse_from_str(s, "%Y%m%d") {
		return Ok(d.and_hms_opt(0, 0, 0).unwrap());
	}
	Err(CodecFehler::UngueltigesFormat {
		segment: "DTM".to_string(),
		feld: "zeitpunkt".to_string(),
		erwartet: "YYYYMMDD or YYYYMMDDHHmmss".to_string(),
	})
}

// ---------------------------------------------------------------------------
// Segment finders
// ---------------------------------------------------------------------------

/// Find the first segment with the given tag.
fn find_segment<'a>(segs: &'a [Segment], tag: &str) -> Result<&'a Segment, CodecFehler> {
	segs.iter()
		.find(|s| s.tag == tag)
		.ok_or(CodecFehler::SegmentFehlt {
			erwartet: tag.to_string(),
		})
}

/// Find the first segment with the given tag whose first element's first component matches the qualifier.
fn find_qualified_segment<'a>(
	segs: &'a [Segment],
	tag: &str,
	qualifier: &str,
) -> Result<&'a Segment, CodecFehler> {
	segs.iter()
		.find(|s| {
			s.tag == tag
				&& s.elements
					.first()
					.and_then(|e| e.components.first())
					.is_some_and(|q| q == qualifier)
		})
		.ok_or(CodecFehler::SegmentFehlt {
			erwartet: format!("{tag}+{qualifier}"),
		})
}
