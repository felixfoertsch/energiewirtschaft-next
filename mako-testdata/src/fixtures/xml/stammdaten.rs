use mako_types::gpke_nachrichten::{RdStammdaten, RessourceTyp};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::{test_malo, test_mp_id};

pub fn stammdaten_xml() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(5); // UeNB
	let malo = test_malo(0);

	format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		 <Stammdaten_MarketDocument>\n\
		 \x20 <sender_MarketParticipant.mRID>{sender}</sender_MarketParticipant.mRID>\n\
		 \x20 <receiver_MarketParticipant.mRID>{empfaenger}</receiver_MarketParticipant.mRID>\n\
		 \x20 <registeredResource.mRID>RES003</registeredResource.mRID>\n\
		 \x20 <resourceType>TR</resourceType>\n\
		 \x20 <standort_MaLo>{malo}</standort_MaLo>\n\
		 \x20 <installedCapacity>2000</installedCapacity>\n\
		 </Stammdaten_MarketDocument>",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn stammdaten_erwartet() -> Nachricht {
	Nachricht {
		absender: test_mp_id(1),
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: test_mp_id(5),
		empfaenger_rolle: MarktRolle::Uebertragungsnetzbetreiber,
		pruef_id: None,
		payload: NachrichtenPayload::RdStammdaten(RdStammdaten {
			ressource_id: "RES003".to_string(),
			ressource_typ: RessourceTyp::TechnischeRessource,
			standort_malo: test_malo(0),
			installierte_leistung_kw: 2000.0,
		}),
	}
}

#[cfg(test)]
mod tests {
	use mako_codec::xml::parser::parse_xml;
	use mako_codec::xml::serializer::serialize_xml;

	use super::*;

	#[test]
	fn parse_stammdaten() {
		let xml = stammdaten_xml();
		let parsed = parse_xml(&xml).expect("parsing must succeed");
		assert_eq!(parsed, stammdaten_erwartet());
	}

	#[test]
	fn roundtrip_stammdaten() {
		let parsed = parse_xml(&stammdaten_xml()).unwrap();
		let serialized = serialize_xml(&parsed).unwrap();
		let reparsed = parse_xml(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
