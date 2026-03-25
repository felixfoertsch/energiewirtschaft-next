use chrono::NaiveDateTime;

use mako_types::gpke_nachrichten::RdKostenblatt;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

pub fn kostenblatt_xml() -> String {
	let sender = test_mp_id(5); // EIV
	let empfaenger = test_mp_id(5); // UeNB (same MP-ID for simplicity)

	format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		 <Kostenblatt_MarketDocument>\n\
		 \x20 <sender_MarketParticipant.mRID>{sender}</sender_MarketParticipant.mRID>\n\
		 \x20 <receiver_MarketParticipant.mRID>{empfaenger}</receiver_MarketParticipant.mRID>\n\
		 \x20 <registeredResource.mRID>RES001</registeredResource.mRID>\n\
		 \x20 <cost.amount>45000</cost.amount>\n\
		 \x20 <start>2026-07-01T08:00:00</start>\n\
		 \x20 <end>2026-07-01T10:00:00</end>\n\
		 </Kostenblatt_MarketDocument>",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn kostenblatt_erwartet() -> Nachricht {
	Nachricht {
		absender: test_mp_id(5),
		absender_rolle: MarktRolle::Einsatzverantwortlicher,
		empfaenger: test_mp_id(5),
		empfaenger_rolle: MarktRolle::Uebertragungsnetzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RdKostenblatt(RdKostenblatt {
			ressource_id: "RES001".to_string(),
			kosten_ct: 45000,
			massnahme_start: NaiveDateTime::parse_from_str(
				"2026-07-01T08:00:00",
				"%Y-%m-%dT%H:%M:%S",
			)
			.unwrap(),
			massnahme_ende: NaiveDateTime::parse_from_str(
				"2026-07-01T10:00:00",
				"%Y-%m-%dT%H:%M:%S",
			)
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
	fn parse_kostenblatt() {
		let xml = kostenblatt_xml();
		let parsed = parse_xml(&xml).expect("parsing must succeed");
		assert_eq!(parsed, kostenblatt_erwartet());
	}

	#[test]
	fn roundtrip_kostenblatt() {
		let parsed = parse_xml(&kostenblatt_xml()).unwrap();
		let serialized = serialize_xml(&parsed).unwrap();
		let reparsed = parse_xml(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
