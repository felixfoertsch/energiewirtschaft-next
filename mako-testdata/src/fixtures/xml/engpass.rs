use chrono::NaiveDateTime;

use mako_types::gpke_nachrichten::RdEngpass;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

pub fn engpass_xml() -> String {
	let sender = test_mp_id(5); // UeNB
	let empfaenger = test_mp_id(1); // NB

	format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		 <NetworkConstraintDocument>\n\
		 \x20 <sender_MarketParticipant.mRID>{sender}</sender_MarketParticipant.mRID>\n\
		 \x20 <receiver_MarketParticipant.mRID>{empfaenger}</receiver_MarketParticipant.mRID>\n\
		 \x20 <area.mRID>NETZGEBIET-NORD-01</area.mRID>\n\
		 \x20 <constrainedCapacity>1200</constrainedCapacity>\n\
		 \x20 <start>2026-07-01T06:00:00</start>\n\
		 \x20 <end>2026-07-01T14:00:00</end>\n\
		 </NetworkConstraintDocument>",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn engpass_erwartet() -> Nachricht {
	Nachricht {
		absender: test_mp_id(5),
		absender_rolle: MarktRolle::Uebertragungsnetzbetreiber,
		empfaenger: test_mp_id(1),
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RdEngpass(RdEngpass {
			netzgebiet: "NETZGEBIET-NORD-01".to_string(),
			engpass_start: NaiveDateTime::parse_from_str(
				"2026-07-01T06:00:00",
				"%Y-%m-%dT%H:%M:%S",
			)
			.unwrap(),
			engpass_ende: NaiveDateTime::parse_from_str(
				"2026-07-01T14:00:00",
				"%Y-%m-%dT%H:%M:%S",
			)
			.unwrap(),
			betroffene_leistung_kw: 1200.0,
		}),
	}
}

#[cfg(test)]
mod tests {
	use mako_codec::xml::parser::parse_xml;
	use mako_codec::xml::serializer::serialize_xml;

	use super::*;

	#[test]
	fn parse_engpass() {
		let xml = engpass_xml();
		let parsed = parse_xml(&xml).expect("parsing must succeed");
		assert_eq!(parsed, engpass_erwartet());
	}

	#[test]
	fn roundtrip_engpass() {
		let parsed = parse_xml(&engpass_xml()).unwrap();
		let serialized = serialize_xml(&parsed).unwrap();
		let reparsed = parse_xml(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
