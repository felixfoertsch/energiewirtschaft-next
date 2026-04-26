//! GeLi-Gas-Prozesskatalog für die Test-UI.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

use super::rollen;

pub fn katalog() -> Vec<ProzessDef> {
	vec![
		ProzessDef::new(
			"geli_lfw",
			"Lieferantenwechsel Gas",
			ProzessKategorie::GeliGas,
			vec![
				schritt(
					"Anmeldung",
					rollen::LIEFERANTENWECHSEL_GAS_ANMELDUNG,
					"UtilmdAnmeldung",
					NachrichtenTyp::Utilmd,
				),
				schritt(
					"Bestätigung an LFN",
					rollen::LIEFERANTENWECHSEL_GAS_BESTAETIGUNG_AN_LFN,
					"UtilmdBestaetigung",
					NachrichtenTyp::Utilmd,
				),
				schritt(
					"Information an LFA",
					rollen::LIEFERANTENWECHSEL_GAS_INFORMATION_AN_LFA,
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
			"geli_ersatz_grundversorgung",
			"Ersatz-/Grundversorgung Gas",
			ProzessKategorie::GeliGas,
			vec![
				schritt(
					"Anmeldung",
					rollen::ERSATZ_GRUNDVERSORGUNG_GAS_ANMELDUNG,
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
			"geli_lieferende",
			"Lieferende Gas",
			ProzessKategorie::GeliGas,
			vec![
				SchrittDef::new(
					"Abmeldung",
					MarktRolle::LieferantAlt,
					MarktRolle::Netzbetreiber,
					"UtilmdLieferendeAbmeldung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantAlt,
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
			"geli_stammdaten",
			"Stammdatenänderung Gas",
			ProzessKategorie::GeliGas,
			vec![
				SchrittDef::new(
					"Änderung senden",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantNeu,
					"UtilmdStammdatenaenderung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung/Ablehnung",
					MarktRolle::LieferantNeu,
					MarktRolle::Netzbetreiber,
					"UtilmdBestaetigung",
					NachrichtenTyp::Utilmd,
				),
			],
		),
		ProzessDef::new(
			"geli_gda",
			"Geschäftsdatenanfrage Gas",
			ProzessKategorie::GeliGas,
			vec![
				SchrittDef::new(
					"Anfrage",
					MarktRolle::LieferantNeu,
					MarktRolle::Netzbetreiber,
					"UtilmdGeschaeftsdatenanfrage",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Antwort",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantNeu,
					"UtilmdGeschaeftsdatenantwort",
					NachrichtenTyp::Utilmd,
				),
			],
		),
		ProzessDef::new(
			"geli_zuordnungsliste",
			"Zuordnungsliste Gas",
			ProzessKategorie::GeliGas,
			vec![SchrittDef::new(
				"Liste versenden",
				MarktRolle::Netzbetreiber,
				MarktRolle::LieferantNeu,
				"UtilmdZuordnungsliste",
				NachrichtenTyp::Utilmd,
			)],
		),
		ProzessDef::new(
			"geli_gmsb_kommunikation",
			"gMSB-Kommunikation Gas",
			ProzessKategorie::GeliGas,
			vec![
				schritt(
					"Mitteilung gMSB an NB",
					rollen::GMSB_AN_NETZBETREIBER,
					"UtilmdGeschaeftsdatenanfrage",
					NachrichtenTyp::Utilmd,
				),
				schritt(
					"Antwort NB an gMSB",
					rollen::NETZBETREIBER_AN_GMSB,
					"UtilmdGeschaeftsdatenantwort",
					NachrichtenTyp::Utilmd,
				),
			],
		),
	]
}

fn schritt(
	name: &str,
	pfad: rollen::RollenPfad,
	typ: &str,
	nachrichten_typ: NachrichtenTyp,
) -> SchrittDef {
	SchrittDef::new(name, pfad[0], pfad[1], typ, nachrichten_typ)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn katalog_deckt_stabile_geli_keys_ab() {
		let keys: Vec<_> = katalog().into_iter().map(|p| p.key).collect();
		assert_eq!(
			keys,
			vec![
				"geli_lfw",
				"geli_ersatz_grundversorgung",
				"geli_lieferende",
				"geli_stammdaten",
				"geli_gda",
				"geli_zuordnungsliste",
				"geli_gmsb_kommunikation",
			]
		);
	}

	#[test]
	fn lfw_und_gmsb_nutzen_rollenpfade_aus_rollenmodul() {
		let prozesse = katalog();
		let lfw = prozesse.iter().find(|p| p.key == "geli_lfw").expect("geli_lfw");
		assert_eq!(
			lfw.schritte[0].absender,
			rollen::LIEFERANTENWECHSEL_GAS_ANMELDUNG[0].slug()
		);
		assert_eq!(
			lfw.schritte[0].empfaenger,
			rollen::LIEFERANTENWECHSEL_GAS_ANMELDUNG[1].slug()
		);

		let gmsb = prozesse
			.iter()
			.find(|p| p.key == "geli_gmsb_kommunikation")
			.expect("geli_gmsb_kommunikation");
		assert_eq!(
			gmsb.schritte[0].absender,
			MarktRolle::GrundzustaendigerMessstellenbetreiber.slug()
		);
		assert_eq!(gmsb.schritte[0].empfaenger, MarktRolle::Netzbetreiber.slug());
		assert_eq!(gmsb.schritte[1].absender, MarktRolle::Netzbetreiber.slug());
		assert_eq!(
			gmsb.schritte[1].empfaenger,
			MarktRolle::GrundzustaendigerMessstellenbetreiber.slug()
		);
	}
}
