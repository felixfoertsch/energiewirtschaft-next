use mako_types::nachricht::Nachricht;

/// Serialize a Nachricht to JSON string.
pub fn to_json(nachricht: &Nachricht) -> Result<String, serde_json::Error> {
	serde_json::to_string_pretty(nachricht)
}

/// Deserialize a Nachricht from JSON string.
pub fn from_json(json: &str) -> Result<Nachricht, serde_json::Error> {
	serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
	use chrono::NaiveDate;
	use mako_types::gpke_nachrichten::UtilmdAnmeldung;
	use mako_types::ids::{MaLoId, MarktpartnerId};
	use mako_types::nachricht::{Nachricht, NachrichtenPayload};
	use mako_types::rolle::MarktRolle;

	use super::*;

	fn sample_anmeldung() -> Nachricht {
		Nachricht {
			absender: MarktpartnerId::new("9900000000003").unwrap(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: MarktpartnerId::new("9900000000010").unwrap(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696788").unwrap(),
				lieferant_neu: MarktpartnerId::new("9900000000003").unwrap(),
				lieferbeginn: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
			}),
		}
	}

	#[test]
	fn round_trip_nachricht() {
		let original = sample_anmeldung();
		let json_str = to_json(&original).unwrap();
		let deserialized = from_json(&json_str).unwrap();
		assert_eq!(original, deserialized);
	}

	#[test]
	fn json_contains_expected_fields() {
		let nachricht = sample_anmeldung();
		let json_str = to_json(&nachricht).unwrap();
		assert!(json_str.contains("UtilmdAnmeldung"));
		assert!(json_str.contains("51238696788"));
		assert!(json_str.contains("9900000000003"));
		assert!(json_str.contains("9900000000010"));
		assert!(json_str.contains("2025-07-01"));
	}

	#[test]
	fn deserialize_from_known_json() {
		let json_str = r#"{
  "absender": "9900000000003",
  "absender_rolle": "LieferantNeu",
  "empfaenger": "9900000000010",
  "empfaenger_rolle": "Netzbetreiber",
  "payload": {
    "UtilmdAnmeldung": {
      "malo_id": "51238696788",
      "lieferant_neu": "9900000000003",
      "lieferbeginn": "2025-07-01"
    }
  }
}"#;
		let nachricht = from_json(json_str).unwrap();
		assert_eq!(nachricht, sample_anmeldung());
	}
}
