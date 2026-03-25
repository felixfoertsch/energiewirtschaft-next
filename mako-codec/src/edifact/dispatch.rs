use chrono::NaiveDate;

use mako_types::gpke_nachrichten::UtilmdAnmeldung;
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
use mako_types::rolle::MarktRolle;

use super::parser::parse_interchange;
use super::segment::Segment;
use crate::fehler::CodecFehler;

/// Parse an EDIFACT string into a typed Nachricht.
/// Dispatches based on UNH message type + BGM qualifier.
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
		other => Err(CodecFehler::UnbekannterNachrichtentyp {
			typ: other.to_string(),
		}),
	}
}

/// Dispatch UTILMD messages based on BGM qualifier.
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
		})?;

	match qualifier.as_str() {
		"E01" => parse_utilmd_anmeldung(unb_sender, unb_empfaenger, segs),
		other => Err(CodecFehler::UnbekannterNachrichtentyp {
			typ: format!("UTILMD/{other}"),
		}),
	}
}

/// Parse UTILMD E01 (Anmeldung) segments into a Nachricht.
fn parse_utilmd_anmeldung(
	unb_sender: &str,
	unb_empfaenger: &str,
	segs: &[Segment],
) -> Result<Nachricht, CodecFehler> {
	// NAD+MS = sender MP-ID
	let nad_ms = find_qualified_segment(segs, "NAD", "MS")?;
	let sender_id_str = nad_ms
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.ok_or(CodecFehler::FeldFehlt {
			segment: "NAD+MS".to_string(),
			feld: "MP-ID".to_string(),
		})?;
	let absender = MarktpartnerId::new(sender_id_str).map_err(|_| CodecFehler::UngueltigerWert {
		segment: "NAD+MS".to_string(),
		feld: "MP-ID".to_string(),
		wert: sender_id_str.clone(),
	})?;

	// NAD+MR = receiver MP-ID
	let nad_mr = find_qualified_segment(segs, "NAD", "MR")?;
	let empfaenger_id_str = nad_mr
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.ok_or(CodecFehler::FeldFehlt {
			segment: "NAD+MR".to_string(),
			feld: "MP-ID".to_string(),
		})?;
	let empfaenger =
		MarktpartnerId::new(empfaenger_id_str).map_err(|_| CodecFehler::UngueltigerWert {
			segment: "NAD+MR".to_string(),
			feld: "MP-ID".to_string(),
			wert: empfaenger_id_str.clone(),
		})?;

	// IDE+24 = MaLo-ID
	let ide = find_qualified_segment(segs, "IDE", "24")?;
	let malo_str = ide
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

	// DTM+92 = Lieferbeginn (format 102 = YYYYMMDD)
	let dtm_92 = find_qualified_segment(segs, "DTM", "92")?;
	let datum_str = dtm_92
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "DTM+92".to_string(),
			feld: "datum".to_string(),
		})?;
	let lieferbeginn =
		NaiveDate::parse_from_str(datum_str, "%Y%m%d").map_err(|_| CodecFehler::UngueltigesFormat {
			segment: "DTM+92".to_string(),
			feld: "datum".to_string(),
			erwartet: "YYYYMMDD (format 102)".to_string(),
		})?;

	// RFF+Z13 = Prüfidentifikator
	let rff = find_qualified_segment(segs, "RFF", "Z13")?;
	let code_str = rff
		.elements
		.first()
		.and_then(|e| e.components.get(1))
		.ok_or(CodecFehler::FeldFehlt {
			segment: "RFF+Z13".to_string(),
			feld: "code".to_string(),
		})?;
	let code_u32 = code_str
		.parse::<u32>()
		.map_err(|_| CodecFehler::UngueltigerWert {
			segment: "RFF+Z13".to_string(),
			feld: "code".to_string(),
			wert: code_str.clone(),
		})?;
	let pid = PruefIdentifikator::from_code(code_u32).ok_or(CodecFehler::UnbekannterPruefIdentifikator {
		code: code_str.clone(),
	})?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::LieferantNeu,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: Some(pid),
		payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
			malo_id,
			lieferant_neu: absender,
			lieferbeginn,
		}),
	})
}

/// Find the first segment with the given tag.
fn find_segment<'a>(segs: &'a [Segment], tag: &str) -> Result<&'a Segment, CodecFehler> {
	segs.iter()
		.find(|s| s.tag == tag)
		.ok_or(CodecFehler::SegmentFehlt {
			erwartet: tag.to_string(),
		})
}

/// Find the first segment with the given tag whose first element's first component matches the qualifier.
/// Handles segments like NAD+MS, NAD+MR, DTM+92, DTM+137, IDE+24, RFF+Z13.
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

/// Serialize a typed Nachricht to an EDIFACT string.
pub fn serialize_nachricht(nachricht: &Nachricht) -> String {
	todo!("Task 3 implements the first variant")
}
