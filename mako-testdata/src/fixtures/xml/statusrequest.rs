use mako_types::gpke_nachrichten::RdStatusRequest;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

pub fn statusrequest_xml() -> String {
	let sender = test_mp_id(5); // UeNB
	let empfaenger = test_mp_id(1); // NB

	format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		 <StatusRequest_MarketDocument>\n\
		 \x20 <sender_MarketParticipant.mRID>{sender}</sender_MarketParticipant.mRID>\n\
		 \x20 <receiver_MarketParticipant.mRID>{empfaenger}</receiver_MarketParticipant.mRID>\n\
		 \x20 <registeredResource.mRID>RES001</registeredResource.mRID>\n\
		 \x20 <requestType>A07</requestType>\n\
		 </StatusRequest_MarketDocument>",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn statusrequest_erwartet() -> Nachricht {
	Nachricht {
		absender: test_mp_id(5),
		absender_rolle: MarktRolle::Uebertragungsnetzbetreiber,
		empfaenger: test_mp_id(1),
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RdStatusRequest(RdStatusRequest {
			ressource_id: "RES001".to_string(),
			anfrage_typ: "A07".to_string(),
		}),
	}
}

#[cfg(test)]
mod tests {
	use mako_codec::xml::parser::parse_xml;
	use mako_codec::xml::serializer::serialize_xml;

	use super::*;

	#[test]
	fn parse_statusrequest() {
		let xml = statusrequest_xml();
		let parsed = parse_xml(&xml).expect("parsing must succeed");
		assert_eq!(parsed, statusrequest_erwartet());
	}

	#[test]
	fn roundtrip_statusrequest() {
		let parsed = parse_xml(&statusrequest_xml()).unwrap();
		let serialized = serialize_xml(&parsed).unwrap();
		let reparsed = parse_xml(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
