use chrono::NaiveDateTime;

use mako_types::gpke_nachrichten::RdAktivierung;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

pub fn aktivierung_xml() -> String {
	let sender = test_mp_id(5); // UeNB
	let empfaenger = test_mp_id(1); // NB

	format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		 <ActivationDocument_MarketDocument>\n\
		 \x20 <sender_MarketParticipant.mRID>{sender}</sender_MarketParticipant.mRID>\n\
		 \x20 <receiver_MarketParticipant.mRID>{empfaenger}</receiver_MarketParticipant.mRID>\n\
		 \x20 <TimeSeries>\n\
		 \x20   <registeredResource.mRID>RES001</registeredResource.mRID>\n\
		 \x20   <quantity.quantity>500</quantity.quantity>\n\
		 \x20   <start>2026-07-01T08:00:00</start>\n\
		 \x20   <end>2026-07-01T10:00:00</end>\n\
		 \x20 </TimeSeries>\n\
		 </ActivationDocument_MarketDocument>",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn aktivierung_erwartet() -> Nachricht {
	Nachricht {
		absender: test_mp_id(5),
		absender_rolle: MarktRolle::Uebertragungsnetzbetreiber,
		empfaenger: test_mp_id(1),
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RdAktivierung(RdAktivierung {
			ressource_id: "RES001".to_string(),
			sollwert_kw: 500.0,
			start: NaiveDateTime::parse_from_str("2026-07-01T08:00:00", "%Y-%m-%dT%H:%M:%S")
				.unwrap(),
			ende: NaiveDateTime::parse_from_str("2026-07-01T10:00:00", "%Y-%m-%dT%H:%M:%S")
				.unwrap(),
		}),
	}
}

#[cfg(test)]
mod tests {
	use mako_codec::xml::parser::parse_xml;
	use mako_codec::xml::serializer::serialize_xml;

	use super::*;

	#[test]
	fn parse_aktivierung() {
		let xml = aktivierung_xml();
		let parsed = parse_xml(&xml).expect("parsing must succeed");
		assert_eq!(parsed, aktivierung_erwartet());
	}

	#[test]
	fn roundtrip_aktivierung() {
		let parsed = parse_xml(&aktivierung_xml()).unwrap();
		let serialized = serialize_xml(&parsed).unwrap();
		let reparsed = parse_xml(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
