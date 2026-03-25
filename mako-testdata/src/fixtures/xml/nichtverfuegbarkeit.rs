use chrono::NaiveDateTime;

use mako_types::gpke_nachrichten::RdNichtverfuegbarkeit;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

pub fn nichtverfuegbarkeit_xml() -> String {
	let sender = test_mp_id(5); // EIV
	let empfaenger = test_mp_id(1); // NB

	format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		 <Unavailability_MarketDocument>\n\
		 \x20 <sender_MarketParticipant.mRID>{sender}</sender_MarketParticipant.mRID>\n\
		 \x20 <receiver_MarketParticipant.mRID>{empfaenger}</receiver_MarketParticipant.mRID>\n\
		 \x20 <registeredResource.mRID>RES004</registeredResource.mRID>\n\
		 \x20 <Reason.text>Geplante Wartung Turbine</Reason.text>\n\
		 \x20 <start>2026-07-15T06:00:00</start>\n\
		 \x20 <end>2026-07-15T18:00:00</end>\n\
		 </Unavailability_MarketDocument>",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn nichtverfuegbarkeit_erwartet() -> Nachricht {
	Nachricht {
		absender: test_mp_id(5),
		absender_rolle: MarktRolle::Einsatzverantwortlicher,
		empfaenger: test_mp_id(1),
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RdNichtverfuegbarkeit(RdNichtverfuegbarkeit {
			ressource_id: "RES004".to_string(),
			von: NaiveDateTime::parse_from_str("2026-07-15T06:00:00", "%Y-%m-%dT%H:%M:%S")
				.unwrap(),
			bis: NaiveDateTime::parse_from_str("2026-07-15T18:00:00", "%Y-%m-%dT%H:%M:%S")
				.unwrap(),
			grund: "Geplante Wartung Turbine".to_string(),
		}),
	}
}

#[cfg(test)]
mod tests {
	use mako_codec::xml::parser::parse_xml;
	use mako_codec::xml::serializer::serialize_xml;

	use super::*;

	#[test]
	fn parse_nichtverfuegbarkeit() {
		let xml = nichtverfuegbarkeit_xml();
		let parsed = parse_xml(&xml).expect("parsing must succeed");
		assert_eq!(parsed, nichtverfuegbarkeit_erwartet());
	}

	#[test]
	fn roundtrip_nichtverfuegbarkeit() {
		let parsed = parse_xml(&nichtverfuegbarkeit_xml()).unwrap();
		let serialized = serialize_xml(&parsed).unwrap();
		let reparsed = parse_xml(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
