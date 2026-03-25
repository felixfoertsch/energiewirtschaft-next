use mako_types::nachricht::{Nachricht, NachrichtenPayload};

/// Identifies which process a message belongs to.
pub struct ProzessZuordnung {
	pub prozess: String,
	pub key: String,
	pub beschreibung: String,
}

/// Determine which process and key a message belongs to.
pub fn zuordne_prozess(nachricht: &Nachricht) -> Option<ProzessZuordnung> {
	match &nachricht.payload {
		// GPKE LFW — all UTILMD types related to Lieferantenwechsel
		NachrichtenPayload::UtilmdAnmeldung(a) => Some(ProzessZuordnung {
			prozess: "gpke_lfw".into(),
			key: format!("gpke_lfw/{}", a.malo_id.as_str()),
			beschreibung: "GPKE Lieferantenwechsel".into(),
		}),
		NachrichtenPayload::UtilmdBestaetigung(b) => Some(ProzessZuordnung {
			prozess: "gpke_lfw".into(),
			key: format!("gpke_lfw/{}", b.malo_id.as_str()),
			beschreibung: "GPKE Lieferantenwechsel".into(),
		}),
		NachrichtenPayload::UtilmdAbmeldung(a) => Some(ProzessZuordnung {
			prozess: "gpke_lfw".into(),
			key: format!("gpke_lfw/{}", a.malo_id.as_str()),
			beschreibung: "GPKE Lieferantenwechsel".into(),
		}),
		NachrichtenPayload::UtilmdAblehnung(a) => Some(ProzessZuordnung {
			prozess: "gpke_lfw".into(),
			key: format!("gpke_lfw/{}", a.malo_id.as_str()),
			beschreibung: "GPKE Lieferantenwechsel".into(),
		}),
		NachrichtenPayload::UtilmdZuordnung(z) => Some(ProzessZuordnung {
			prozess: "gpke_lfw".into(),
			key: format!("gpke_lfw/{}", z.malo_id.as_str()),
			beschreibung: "GPKE Lieferantenwechsel".into(),
		}),
		// GPKE Lieferende
		NachrichtenPayload::UtilmdLieferendeAbmeldung(a) => Some(ProzessZuordnung {
			prozess: "gpke_lieferende".into(),
			key: format!("gpke_lieferende/{}", a.malo_id.as_str()),
			beschreibung: "GPKE Lieferende".into(),
		}),
		NachrichtenPayload::UtilmdLieferendeBestaetigung(b) => Some(ProzessZuordnung {
			prozess: "gpke_lieferende".into(),
			key: format!("gpke_lieferende/{}", b.malo_id.as_str()),
			beschreibung: "GPKE Lieferende".into(),
		}),
		NachrichtenPayload::MsconsSchlussturnusmesswert(m) => Some(ProzessZuordnung {
			prozess: "gpke_lieferende".into(),
			key: format!("gpke_lieferende/{}", m.malo_id.as_str()),
			beschreibung: "GPKE Lieferende".into(),
		}),
		// GPKE Stammdaten
		NachrichtenPayload::UtilmdStammdatenaenderung(s) => Some(ProzessZuordnung {
			prozess: "gpke_stammdaten".into(),
			key: format!("gpke_stammdaten/{}", s.malo_id.as_str()),
			beschreibung: "GPKE Stammdatenänderung".into(),
		}),
		// GPKE Zuordnungsliste
		NachrichtenPayload::UtilmdZuordnungsliste(_) => Some(ProzessZuordnung {
			prozess: "gpke_zuordnung".into(),
			key: "gpke_zuordnung/liste".into(),
			beschreibung: "GPKE Zuordnungsliste".into(),
		}),
		// GPKE GDA
		NachrichtenPayload::UtilmdGeschaeftsdatenanfrage(g) => Some(ProzessZuordnung {
			prozess: "gpke_gda".into(),
			key: format!("gpke_gda/{}", g.malo_id.as_str()),
			beschreibung: "GPKE Geschäftsdatenanfrage".into(),
		}),
		NachrichtenPayload::UtilmdGeschaeftsdatenantwort(g) => Some(ProzessZuordnung {
			prozess: "gpke_gda".into(),
			key: format!("gpke_gda/{}", g.malo_id.as_str()),
			beschreibung: "GPKE Geschäftsdatenanfrage".into(),
		}),
		// Other processes (WiM, UBP, etc.) — not yet mapped
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::NaiveDate;
	use mako_types::gpke_nachrichten::{UtilmdAnmeldung, UtilmdMsbWechselAnmeldung};
	use mako_types::ids::{MaLoId, MarktpartnerId, MeLoId};
	use mako_types::rolle::MarktRolle;

	fn test_malo() -> MaLoId {
		MaLoId::new("51238696705").unwrap()
	}

	fn test_mp(index: u64) -> MarktpartnerId {
		MarktpartnerId::new(&format!("{:013}", 9900000000000u64 + index)).unwrap()
	}

	#[test]
	fn anmeldung_maps_to_gpke_lfw() {
		let nachricht = Nachricht {
			absender: test_mp(0),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: test_mp(1),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: test_malo(),
				lieferant_neu: test_mp(0),
				lieferbeginn: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
			}),
		};
		let z = zuordne_prozess(&nachricht).expect("should map");
		assert_eq!(z.prozess, "gpke_lfw");
		assert!(z.key.contains("51238696705"));
		assert_eq!(z.beschreibung, "GPKE Lieferantenwechsel");
	}

	#[test]
	fn unknown_payload_returns_none() {
		let nachricht = Nachricht {
			absender: test_mp(0),
			absender_rolle: MarktRolle::Messstellenbetreiber,
			empfaenger: test_mp(1),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdMsbWechselAnmeldung(UtilmdMsbWechselAnmeldung {
				melo_id: MeLoId::new(&format!("DE{:031}", 0)).unwrap(),
				msb_neu: test_mp(0),
				wechseldatum: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
			}),
		};
		assert!(zuordne_prozess(&nachricht).is_none());
	}

	#[test]
	fn lieferende_abmeldung_maps_correctly() {
		let nachricht = Nachricht {
			absender: test_mp(0),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: test_mp(1),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdLieferendeAbmeldung(
				mako_types::gpke_nachrichten::UtilmdLieferendeAbmeldung {
					malo_id: test_malo(),
					lieferant: test_mp(0),
					lieferende: NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(),
				},
			),
		};
		let z = zuordne_prozess(&nachricht).expect("should map");
		assert_eq!(z.prozess, "gpke_lieferende");
		assert!(z.key.contains("51238696705"));
		assert_eq!(z.beschreibung, "GPKE Lieferende");
	}
}
