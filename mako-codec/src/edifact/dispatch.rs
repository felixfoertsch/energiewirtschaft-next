use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

use mako_types::gpke_nachrichten::*;
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
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
		_ => unimplemented!("serialize_nachricht: payload type not yet supported"),
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

	// Fallback: BGM qualifier + heuristics
	match qualifier.as_str() {
		"E01" => {
			// E01 without PID = LieferendeBestaetigung
			parse_utilmd_lieferende_bestaetigung(unb_sender, unb_empfaenger, segs)
		}
		"E03" => parse_utilmd_stammdatenaenderung(unb_sender, unb_empfaenger, segs, None),
		"E06" => {
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
		None => Err(CodecFehler::SegmentFehlt {
			erwartet: "RFF+Z13".to_string(),
		}),
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
