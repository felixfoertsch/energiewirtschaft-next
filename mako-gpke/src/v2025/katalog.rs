//! GPKE-Prozesskatalog für die Test-UI.
//!
//! Quelle der Wahrheit für Schritt-Sequenzen ist die jeweilige `reduce`-
//! Funktion zusammen mit `rollen.rs`. Wenn ein neuer Schritt im Reducer
//! emittiert wird oder ein Rollenpfad sich ändert, hier nachziehen.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

pub fn katalog() -> Vec<ProzessDef> {
	vec![
		ProzessDef::new(
			"gpke_lfw",
			"Lieferantenwechsel",
			ProzessKategorie::Gpke,
			vec![
				SchrittDef::new(
					"Anmeldung",
					MarktRolle::LieferantNeu,
					MarktRolle::Netzbetreiber,
					"UtilmdAnmeldung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung an LFN",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantNeu,
					"UtilmdBestaetigung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Abmeldung an LFA",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantAlt,
					"UtilmdAbmeldung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Widerspruchsfrist",
					MarktRolle::LieferantAlt,
					MarktRolle::Netzbetreiber,
					"",
					NachrichtenTyp::Intern,
				),
				SchrittDef::new(
					"Zuordnung an LFN",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantNeu,
					"UtilmdZuordnung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Zuordnung an LFA",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantAlt,
					"UtilmdZuordnung",
					NachrichtenTyp::Utilmd,
				),
			],
		),
		ProzessDef::new(
			"gpke_ersatz_grundversorgung",
			"Ersatz-/Grundversorgung",
			ProzessKategorie::Gpke,
			vec![
				SchrittDef::new(
					"Anmeldung",
					MarktRolle::LieferantErsatzGrundversorgung,
					MarktRolle::Netzbetreiber,
					"UtilmdAnmeldung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantErsatzGrundversorgung,
					"UtilmdBestaetigung",
					NachrichtenTyp::Utilmd,
				),
			],
		),
		ProzessDef::new(
			"gpke_lieferende",
			"Lieferende",
			ProzessKategorie::Gpke,
			vec![
				SchrittDef::new(
					"Abmeldung",
					MarktRolle::Lieferant,
					MarktRolle::Netzbetreiber,
					"UtilmdLieferendeAbmeldung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung",
					MarktRolle::Netzbetreiber,
					MarktRolle::Lieferant,
					"UtilmdLieferendeBestaetigung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Schlussturnusmesswert",
					MarktRolle::Messstellenbetreiber,
					MarktRolle::Netzbetreiber,
					"MsconsSchlussturnusmesswert",
					NachrichtenTyp::Mscons,
				),
			],
		),
		ProzessDef::new(
			"gpke_stammdaten",
			"Stammdatenänderung",
			ProzessKategorie::Gpke,
			vec![
				SchrittDef::new(
					"Änderung senden",
					MarktRolle::Netzbetreiber,
					MarktRolle::Lieferant,
					"UtilmdStammdatenaenderung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung/Ablehnung",
					MarktRolle::Lieferant,
					MarktRolle::Netzbetreiber,
					"UtilmdBestaetigung",
					NachrichtenTyp::Utilmd,
				),
			],
		),
		ProzessDef::new(
			"gpke_gda",
			"Geschäftsdatenanfrage",
			ProzessKategorie::Gpke,
			vec![
				SchrittDef::new(
					"Anfrage",
					MarktRolle::Lieferant,
					MarktRolle::Netzbetreiber,
					"UtilmdGeschaeftsdatenanfrage",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Antwort",
					MarktRolle::Netzbetreiber,
					MarktRolle::Lieferant,
					"UtilmdGeschaeftsdatenantwort",
					NachrichtenTyp::Utilmd,
				),
			],
		),
		ProzessDef::new(
			"gpke_zuordnungsliste",
			"Zuordnungsliste",
			ProzessKategorie::Gpke,
			vec![SchrittDef::new(
				"Liste versenden",
				MarktRolle::Netzbetreiber,
				MarktRolle::Lieferant,
				"UtilmdZuordnungsliste",
				NachrichtenTyp::Utilmd,
			)],
		),
		ProzessDef::new(
			"gpke_uebertragungsnetz_bilanzierung",
			"ÜNB-Bilanzierung (GPKE-Pfad)",
			ProzessKategorie::Gpke,
			vec![
				SchrittDef::new(
					"Aggregat NB → ÜNB",
					MarktRolle::Netzbetreiber,
					MarktRolle::Uebertragungsnetzbetreiber,
					"UtilmdBilanzaggregat",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"BKV-Zuordnung",
					MarktRolle::Bilanzkreisverantwortlicher,
					MarktRolle::Bilanzkoordinator,
					"UtilmdBilanzkreiszuordnung",
					NachrichtenTyp::Utilmd,
				),
			],
		),
	]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn katalog_deckt_alle_prozessmodule_ab() {
		let keys: Vec<_> = katalog().into_iter().map(|p| p.key).collect();
		// Ein Eintrag pro v2025-Submodul, plus ein zusätzlicher Eintrag für
		// den E/G-Sonderfall der LFW-Anmeldung.
		assert!(keys.contains(&"gpke_lfw".to_string()));
		assert!(keys.contains(&"gpke_lieferende".to_string()));
		assert!(keys.contains(&"gpke_stammdaten".to_string()));
		assert!(keys.contains(&"gpke_gda".to_string()));
		assert!(keys.contains(&"gpke_zuordnungsliste".to_string()));
		assert!(keys.contains(&"gpke_ersatz_grundversorgung".to_string()));
	}

	#[test]
	fn lfw_referenziert_korrekten_rollenpfad() {
		let lfw = katalog()
			.into_iter()
			.find(|p| p.key == "gpke_lfw")
			.expect("gpke_lfw");
		// Sollte Anmeldung von LFN zu NB enthalten — synchron mit
		// rollen::LIEFERANTENWECHSEL_ANMELDUNG.
		let anmeldung = lfw.schritte.first().expect("erster Schritt");
		assert_eq!(anmeldung.absender, "lieferant_neu");
		assert_eq!(anmeldung.empfaenger, "netzbetreiber");
	}
}
