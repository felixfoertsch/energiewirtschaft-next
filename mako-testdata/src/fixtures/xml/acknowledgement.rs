use mako_types::gpke_nachrichten::RdBestaetigung;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

pub fn bestaetigung_xml() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(5); // UeNB

	format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		 <Acknowledgement_MarketDocument>\n\
		 \x20 <sender_MarketParticipant.mRID>{sender}</sender_MarketParticipant.mRID>\n\
		 \x20 <receiver_MarketParticipant.mRID>{empfaenger}</receiver_MarketParticipant.mRID>\n\
		 \x20 <received_MarketDocument.mRID>DOC-ACT-001</received_MarketDocument.mRID>\n\
		 \x20 <Reason.code>A01</Reason.code>\n\
		 \x20 <Reason.text>Aktivierung akzeptiert</Reason.text>\n\
		 </Acknowledgement_MarketDocument>",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn bestaetigung_erwartet() -> Nachricht {
	Nachricht {
		absender: test_mp_id(1),
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: test_mp_id(5),
		empfaenger_rolle: MarktRolle::Uebertragungsnetzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RdBestaetigung(RdBestaetigung {
			referenz_dokument_id: "DOC-ACT-001".to_string(),
			akzeptiert: true,
			grund: Some("Aktivierung akzeptiert".to_string()),
		}),
	}
}

#[cfg(test)]
mod tests {
	use mako_codec::xml::parser::parse_xml;
	use mako_codec::xml::serializer::serialize_xml;

	use super::*;

	#[test]
	fn parse_bestaetigung() {
		let xml = bestaetigung_xml();
		let parsed = parse_xml(&xml).expect("parsing must succeed");
		assert_eq!(parsed, bestaetigung_erwartet());
	}

	#[test]
	fn roundtrip_bestaetigung() {
		let parsed = parse_xml(&bestaetigung_xml()).unwrap();
		let serialized = serialize_xml(&parsed).unwrap();
		let reparsed = parse_xml(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
