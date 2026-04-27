use chrono::{DateTime, FixedOffset, NaiveDateTime};
use quick_xml::Reader;
use quick_xml::events::Event;

use mako_types::gpke_nachrichten::*;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rd2_quittung::{AcknowledgementDocument, AcknowledgementTyp};
use mako_types::rolle::MarktRolle;

use crate::fehler::CodecFehler;

/// Parse an XML string (Redispatch 2.0 document) into a typed Nachricht.
pub fn parse_xml(input: &str) -> Result<Nachricht, CodecFehler> {
	let mut reader = Reader::from_str(input);

	// Detect root element
	let root_name = detect_root(&mut reader)?;
	reader = Reader::from_str(input);

	// Extract all text elements into a flat key-value map
	let fields = extract_fields(&mut reader)?;

	let sender_str = fields.get("sender_MarketParticipant.mRID").ok_or_else(|| {
		CodecFehler::XmlParseFehler("missing sender_MarketParticipant.mRID".into())
	})?;
	let receiver_str = fields
		.get("receiver_MarketParticipant.mRID")
		.ok_or_else(|| {
			CodecFehler::XmlParseFehler("missing receiver_MarketParticipant.mRID".into())
		})?;

	let absender = MarktpartnerId::new(sender_str)
		.map_err(|e| CodecFehler::XmlParseFehler(format!("invalid sender MP-ID: {e}")))?;
	let empfaenger = MarktpartnerId::new(receiver_str)
		.map_err(|e| CodecFehler::XmlParseFehler(format!("invalid receiver MP-ID: {e}")))?;

	let (absender_rolle, empfaenger_rolle, payload) = match root_name.as_str() {
		"ActivationDocument_MarketDocument" => {
			let p = RdAktivierung {
				ressource_id: require_field(&fields, "registeredResource.mRID")?,
				sollwert_kw: parse_f64(&fields, "quantity.quantity")?,
				start: parse_dt(&fields, "start")?,
				ende: parse_dt(&fields, "end")?,
			};
			(
				MarktRolle::Uebertragungsnetzbetreiber,
				MarktRolle::Netzbetreiber,
				NachrichtenPayload::RdAktivierung(p),
			)
		}
		"PlannedResourceSchedule_MarketDocument" => {
			let ressource_id = require_field(&fields, "registeredResource.mRID")?;
			let wert = parse_f64(&fields, "quantity.quantity")?;
			let zeitpunkt = parse_dt(&fields, "start")?;
			let p = RdFahrplan {
				ressource_id,
				zeitreihe: vec![Messwert {
					zeitpunkt,
					wert,
					einheit: "kW".to_string(),
					status: MesswertStatus::Gemessen,
				}],
			};
			(
				MarktRolle::Einsatzverantwortlicher,
				MarktRolle::Netzbetreiber,
				NachrichtenPayload::RdFahrplan(p),
			)
		}
		"Acknowledgement_MarketDocument" => {
			if fields.get("acknowledgementStatus").is_some() {
				let ack_typ = match require_field(&fields, "acknowledgementStatus")?.as_str() {
					"positiv" => AcknowledgementTyp::Positiv,
					"negativ" => AcknowledgementTyp::Negativ,
					other => {
						return Err(CodecFehler::XmlParseFehler(format!(
							"unknown acknowledgementStatus: {other}"
						)));
					}
				};
				let p = AcknowledgementDocument {
					receiver_mrid: empfaenger.clone(),
					sender_mrid: absender.clone(),
					original_message_mrid: require_field(&fields, "received_MarketDocument.mRID")?,
					received_at: parse_rfc3339(&fields, "createdDateTime")?,
					ack_typ,
					reason: fields.get("Reason.text").cloned(),
				};
				(
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantNeu,
					NachrichtenPayload::AcknowledgementDocument(p),
				)
			} else {
				let p = RdBestaetigung {
					referenz_dokument_id: require_field(&fields, "received_MarketDocument.mRID")?,
					akzeptiert: require_field(&fields, "Reason.code")? == "A01",
					grund: fields.get("Reason.text").cloned(),
				};
				(
					MarktRolle::Netzbetreiber,
					MarktRolle::Uebertragungsnetzbetreiber,
					NachrichtenPayload::RdBestaetigung(p),
				)
			}
		}
		"Stammdaten_MarketDocument" => {
			let typ_str = require_field(&fields, "resourceType")?;
			let ressource_typ = match typ_str.as_str() {
				"TR" => RessourceTyp::TechnischeRessource,
				_ => RessourceTyp::SteuerbareRessource,
			};
			let malo_str = require_field(&fields, "standort_MaLo")?;
			let p = RdStammdaten {
				ressource_id: require_field(&fields, "registeredResource.mRID")?,
				ressource_typ,
				standort_malo: mako_types::ids::MaLoId::new(&malo_str)
					.map_err(|e| CodecFehler::XmlParseFehler(format!("invalid MaLo-ID: {e}")))?,
				installierte_leistung_kw: parse_f64(&fields, "installedCapacity")?,
			};
			(
				MarktRolle::Netzbetreiber,
				MarktRolle::Uebertragungsnetzbetreiber,
				NachrichtenPayload::RdStammdaten(p),
			)
		}
		"Kostenblatt_MarketDocument" => {
			let p = RdKostenblatt {
				ressource_id: require_field(&fields, "registeredResource.mRID")?,
				kosten_ct: parse_f64(&fields, "cost.amount").map(|v| v as i64)?,
				massnahme_start: parse_dt(&fields, "start")?,
				massnahme_ende: parse_dt(&fields, "end")?,
			};
			(
				MarktRolle::Einsatzverantwortlicher,
				MarktRolle::Uebertragungsnetzbetreiber,
				NachrichtenPayload::RdKostenblatt(p),
			)
		}
		"NetworkConstraintDocument" => {
			let p = RdEngpass {
				netzgebiet: require_field(&fields, "area.mRID")?,
				engpass_start: parse_dt(&fields, "start")?,
				engpass_ende: parse_dt(&fields, "end")?,
				betroffene_leistung_kw: parse_f64(&fields, "constrainedCapacity")?,
			};
			(
				MarktRolle::Uebertragungsnetzbetreiber,
				MarktRolle::Netzbetreiber,
				NachrichtenPayload::RdEngpass(p),
			)
		}
		"Unavailability_MarketDocument" => {
			let p = RdNichtverfuegbarkeit {
				ressource_id: require_field(&fields, "registeredResource.mRID")?,
				von: parse_dt(&fields, "start")?,
				bis: parse_dt(&fields, "end")?,
				grund: require_field(&fields, "Reason.text")?,
			};
			(
				MarktRolle::Einsatzverantwortlicher,
				MarktRolle::Netzbetreiber,
				NachrichtenPayload::RdNichtverfuegbarkeit(p),
			)
		}
		"StatusRequest_MarketDocument" => {
			let p = RdStatusRequest {
				ressource_id: require_field(&fields, "registeredResource.mRID")?,
				anfrage_typ: require_field(&fields, "requestType")?,
			};
			(
				MarktRolle::Uebertragungsnetzbetreiber,
				MarktRolle::Netzbetreiber,
				NachrichtenPayload::RdStatusRequest(p),
			)
		}
		"Kaskade_MarketDocument" => {
			let p = RdKaskade {
				ressource_id: require_field(&fields, "registeredResource.mRID")?,
				kaskaden_stufe: parse_f64(&fields, "cascadeLevel").map(|v| v as u8)?,
				sollwert_kw: parse_f64(&fields, "quantity.quantity")?,
				start: parse_dt(&fields, "start")?,
				ende: parse_dt(&fields, "end")?,
			};
			(
				MarktRolle::Uebertragungsnetzbetreiber,
				MarktRolle::Netzbetreiber,
				NachrichtenPayload::RdKaskade(p),
			)
		}
		other => {
			return Err(CodecFehler::XmlParseFehler(format!(
				"unknown root element: {other}"
			)));
		}
	};

	Ok(Nachricht {
		absender,
		absender_rolle,
		empfaenger,
		empfaenger_rolle,
		pruef_id: None,
		payload,
	})
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn detect_root(reader: &mut Reader<&[u8]>) -> Result<String, CodecFehler> {
	loop {
		match reader.read_event() {
			Ok(Event::Start(e)) => {
				let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
				return Ok(name);
			}
			Ok(Event::Eof) => {
				return Err(CodecFehler::XmlParseFehler("empty document".into()));
			}
			Err(e) => {
				return Err(CodecFehler::XmlParseFehler(format!("XML error: {e}")));
			}
			_ => {} // skip declarations, comments, etc.
		}
	}
}

/// Extract all leaf text elements into a flat map.
/// For nested elements, the last occurrence of a tag name wins.
fn extract_fields(
	reader: &mut Reader<&[u8]>,
) -> Result<std::collections::HashMap<String, String>, CodecFehler> {
	let mut map = std::collections::HashMap::new();
	let mut current_tag: Option<String> = None;

	loop {
		match reader.read_event() {
			Ok(Event::Start(e)) => {
				current_tag = Some(String::from_utf8_lossy(e.name().as_ref()).to_string());
			}
			Ok(Event::Text(e)) => {
				if let Some(ref tag) = current_tag {
					let text = e.unescape().map_err(|err| {
						CodecFehler::XmlParseFehler(format!("UTF-8 error: {err}"))
					})?;
					let text = text.trim().to_string();
					if !text.is_empty() {
						map.insert(tag.clone(), text);
					}
				}
			}
			Ok(Event::End(_)) => {
				current_tag = None;
			}
			Ok(Event::Eof) => break,
			Err(e) => {
				return Err(CodecFehler::XmlParseFehler(format!("XML error: {e}")));
			}
			_ => {}
		}
	}

	Ok(map)
}

fn require_field(
	fields: &std::collections::HashMap<String, String>,
	key: &str,
) -> Result<String, CodecFehler> {
	fields
		.get(key)
		.cloned()
		.ok_or_else(|| CodecFehler::XmlParseFehler(format!("missing field: {key}")))
}

fn parse_f64(
	fields: &std::collections::HashMap<String, String>,
	key: &str,
) -> Result<f64, CodecFehler> {
	let s = require_field(fields, key)?;
	s.parse::<f64>()
		.map_err(|e| CodecFehler::XmlParseFehler(format!("invalid f64 for {key}: {e}")))
}

fn parse_dt(
	fields: &std::collections::HashMap<String, String>,
	key: &str,
) -> Result<NaiveDateTime, CodecFehler> {
	let s = require_field(fields, key)?;
	NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
		.map_err(|e| CodecFehler::XmlParseFehler(format!("invalid datetime for {key}: {e}")))
}

fn parse_rfc3339(
	fields: &std::collections::HashMap<String, String>,
	key: &str,
) -> Result<DateTime<FixedOffset>, CodecFehler> {
	let s = require_field(fields, key)?;
	DateTime::parse_from_rfc3339(&s)
		.map_err(|e| CodecFehler::XmlParseFehler(format!("invalid datetime for {key}: {e}")))
}
