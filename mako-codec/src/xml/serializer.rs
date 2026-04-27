use chrono::{Datelike, NaiveDateTime, Timelike};

use mako_types::gpke_nachrichten::*;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rd2_quittung::AcknowledgementTyp;

use crate::fehler::CodecFehler;

/// Serialize a Nachricht with an RD 2.0 payload to XML.
pub fn serialize_xml(nachricht: &Nachricht) -> Result<String, CodecFehler> {
	let sender = nachricht.absender.as_str();
	let receiver = nachricht.empfaenger.as_str();

	let (root, body) = match &nachricht.payload {
		NachrichtenPayload::RdAktivierung(p) => (
			"ActivationDocument_MarketDocument",
			format!(
				"  <TimeSeries>\n\
				 \x20   <registeredResource.mRID>{}</registeredResource.mRID>\n\
				 \x20   <quantity.quantity>{}</quantity.quantity>\n\
				 \x20   <start>{}</start>\n\
				 \x20   <end>{}</end>\n\
				 \x20 </TimeSeries>",
				p.ressource_id,
				p.sollwert_kw,
				fmt_dt(&p.start),
				fmt_dt(&p.ende),
			),
		),
		NachrichtenPayload::RdFahrplan(p) => {
			let first = p.zeitreihe.first().ok_or_else(|| {
				CodecFehler::XmlParseFehler("RdFahrplan has empty zeitreihe".into())
			})?;
			(
				"PlannedResourceSchedule_MarketDocument",
				format!(
					"  <TimeSeries>\n\
					 \x20   <registeredResource.mRID>{}</registeredResource.mRID>\n\
					 \x20   <quantity.quantity>{}</quantity.quantity>\n\
					 \x20   <start>{}</start>\n\
					 \x20 </TimeSeries>",
					p.ressource_id,
					first.wert,
					fmt_dt(&first.zeitpunkt),
				),
			)
		}
		NachrichtenPayload::RdBestaetigung(p) => {
			let reason_code = if p.akzeptiert { "A01" } else { "A02" };
			let reason_text = match &p.grund {
				Some(g) => format!("\n  <Reason.text>{g}</Reason.text>"),
				None => String::new(),
			};
			(
				"Acknowledgement_MarketDocument",
				format!(
					"  <received_MarketDocument.mRID>{}</received_MarketDocument.mRID>\n\
					 \x20 <Reason.code>{reason_code}</Reason.code>{reason_text}",
					p.referenz_dokument_id,
				),
			)
		}
		NachrichtenPayload::AcknowledgementDocument(p) => {
			let status = match p.ack_typ {
				AcknowledgementTyp::Positiv => "positiv",
				AcknowledgementTyp::Negativ => "negativ",
			};
			let reason_text = match &p.reason {
				Some(reason) => format!("\n  <Reason.text>{reason}</Reason.text>"),
				None => String::new(),
			};
			(
				"Acknowledgement_MarketDocument",
				format!(
					"  <received_MarketDocument.mRID>{}</received_MarketDocument.mRID>\n\
					 \x20 <createdDateTime>{}</createdDateTime>\n\
					 \x20 <acknowledgementStatus>{status}</acknowledgementStatus>{reason_text}",
					p.original_message_mrid,
					p.received_at.to_rfc3339(),
				),
			)
		}
		NachrichtenPayload::RdStammdaten(p) => {
			let typ = match p.ressource_typ {
				RessourceTyp::TechnischeRessource => "TR",
				RessourceTyp::SteuerbareRessource => "SR",
			};
			(
				"Stammdaten_MarketDocument",
				format!(
					"  <registeredResource.mRID>{}</registeredResource.mRID>\n\
					 \x20 <resourceType>{typ}</resourceType>\n\
					 \x20 <standort_MaLo>{}</standort_MaLo>\n\
					 \x20 <installedCapacity>{}</installedCapacity>",
					p.ressource_id,
					p.standort_malo.as_str(),
					p.installierte_leistung_kw,
				),
			)
		}
		NachrichtenPayload::RdKostenblatt(p) => (
			"Kostenblatt_MarketDocument",
			format!(
				"  <registeredResource.mRID>{}</registeredResource.mRID>\n\
				 \x20 <cost.amount>{}</cost.amount>\n\
				 \x20 <start>{}</start>\n\
				 \x20 <end>{}</end>",
				p.ressource_id,
				p.kosten_ct,
				fmt_dt(&p.massnahme_start),
				fmt_dt(&p.massnahme_ende),
			),
		),
		NachrichtenPayload::RdEngpass(p) => (
			"NetworkConstraintDocument",
			format!(
				"  <area.mRID>{}</area.mRID>\n\
				 \x20 <constrainedCapacity>{}</constrainedCapacity>\n\
				 \x20 <start>{}</start>\n\
				 \x20 <end>{}</end>",
				p.netzgebiet,
				p.betroffene_leistung_kw,
				fmt_dt(&p.engpass_start),
				fmt_dt(&p.engpass_ende),
			),
		),
		NachrichtenPayload::RdNichtverfuegbarkeit(p) => (
			"Unavailability_MarketDocument",
			format!(
				"  <registeredResource.mRID>{}</registeredResource.mRID>\n\
				 \x20 <Reason.text>{}</Reason.text>\n\
				 \x20 <start>{}</start>\n\
				 \x20 <end>{}</end>",
				p.ressource_id,
				p.grund,
				fmt_dt(&p.von),
				fmt_dt(&p.bis),
			),
		),
		NachrichtenPayload::RdStatusRequest(p) => (
			"StatusRequest_MarketDocument",
			format!(
				"  <registeredResource.mRID>{}</registeredResource.mRID>\n\
				 \x20 <requestType>{}</requestType>",
				p.ressource_id, p.anfrage_typ,
			),
		),
		NachrichtenPayload::RdKaskade(p) => (
			"Kaskade_MarketDocument",
			format!(
				"  <registeredResource.mRID>{}</registeredResource.mRID>\n\
				 \x20 <cascadeLevel>{}</cascadeLevel>\n\
				 \x20 <quantity.quantity>{}</quantity.quantity>\n\
				 \x20 <start>{}</start>\n\
				 \x20 <end>{}</end>",
				p.ressource_id,
				p.kaskaden_stufe,
				p.sollwert_kw,
				fmt_dt(&p.start),
				fmt_dt(&p.ende),
			),
		),
		other => {
			return Err(CodecFehler::XmlParseFehler(format!(
				"payload type {other:?} is not an XML/RD 2.0 type"
			)));
		}
	};

	Ok(format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		 <{root}>\n\
		 \x20 <sender_MarketParticipant.mRID>{sender}</sender_MarketParticipant.mRID>\n\
		 \x20 <receiver_MarketParticipant.mRID>{receiver}</receiver_MarketParticipant.mRID>\n\
		 {body}\n\
		 </{root}>"
	))
}

/// Format NaiveDateTime as ISO 8601 without the alloc feature.
fn fmt_dt(dt: &NaiveDateTime) -> String {
	format!(
		"{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
		dt.year(),
		dt.month(),
		dt.day(),
		dt.hour(),
		dt.minute(),
		dt.second(),
	)
}
