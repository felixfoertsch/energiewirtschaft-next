use chrono::NaiveDateTime;

use mako_types::gpke_nachrichten::{MesswertStatus, Messwert, RdFahrplan};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

pub fn fahrplan_xml() -> String {
	let sender = test_mp_id(5); // EIV
	let empfaenger = test_mp_id(1); // NB

	format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		 <PlannedResourceSchedule_MarketDocument>\n\
		 \x20 <sender_MarketParticipant.mRID>{sender}</sender_MarketParticipant.mRID>\n\
		 \x20 <receiver_MarketParticipant.mRID>{empfaenger}</receiver_MarketParticipant.mRID>\n\
		 \x20 <TimeSeries>\n\
		 \x20   <registeredResource.mRID>RES002</registeredResource.mRID>\n\
		 \x20   <quantity.quantity>750</quantity.quantity>\n\
		 \x20   <start>2026-07-01T00:00:00</start>\n\
		 \x20 </TimeSeries>\n\
		 </PlannedResourceSchedule_MarketDocument>",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn fahrplan_erwartet() -> Nachricht {
	Nachricht {
		absender: test_mp_id(5),
		absender_rolle: MarktRolle::Einsatzverantwortlicher,
		empfaenger: test_mp_id(1),
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RdFahrplan(RdFahrplan {
			ressource_id: "RES002".to_string(),
			zeitreihe: vec![Messwert {
				zeitpunkt: NaiveDateTime::parse_from_str(
					"2026-07-01T00:00:00",
					"%Y-%m-%dT%H:%M:%S",
				)
				.unwrap(),
				wert: 750.0,
				einheit: "kW".to_string(),
				status: MesswertStatus::Gemessen,
			}],
		}),
	}
}

#[cfg(test)]
mod tests {
	use mako_codec::xml::parser::parse_xml;
	use mako_codec::xml::serializer::serialize_xml;

	use super::*;

	#[test]
	fn parse_fahrplan() {
		let xml = fahrplan_xml();
		let parsed = parse_xml(&xml).expect("parsing must succeed");
		assert_eq!(parsed, fahrplan_erwartet());
	}

	#[test]
	fn roundtrip_fahrplan() {
		let parsed = parse_xml(&fahrplan_xml()).unwrap();
		let serialized = serialize_xml(&parsed).unwrap();
		let reparsed = parse_xml(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
