//! §14a-Prozesskatalog für die Test-UI.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

use super::rollen;

pub fn katalog() -> Vec<ProzessDef> {
	vec![
		ProzessDef::new(
			"14a_anmeldung_sve",
			"Anmeldung steuerbare Verbrauchseinrichtung",
			ProzessKategorie::Para14a,
			vec![SchrittDef::new(
				"Anmeldung SVE",
				MarktRolle::LieferantNeu,
				MarktRolle::Netzbetreiber,
				"UtilmdSteuerbareVerbrauchseinrichtung",
				NachrichtenTyp::Utilmd,
			)],
		),
		ProzessDef::new(
			"14a_modul2_daten",
			"Modul-2-Daten",
			ProzessKategorie::Para14a,
			vec![SchrittDef::new(
				"Modul-2-Daten an NB",
				MarktRolle::LieferantNeu,
				MarktRolle::Netzbetreiber,
				"UtilmdSteuerbareVerbrauchseinrichtung",
				NachrichtenTyp::Utilmd,
			)],
		),
		ProzessDef::new(
			"14a_information_an_lf",
			"Information an Lieferant",
			ProzessKategorie::Para14a,
			vec![SchrittDef::new(
				"Information an LF",
				MarktRolle::Netzbetreiber,
				MarktRolle::LieferantNeu,
				"UtilmdSteuerbareVerbrauchseinrichtung",
				NachrichtenTyp::Utilmd,
			)],
		),
		ProzessDef::new(
			"14a_steuerungsbefehl",
			"Steuerungsbefehl",
			ProzessKategorie::Para14a,
			vec![
				schritt(
					"Steuerungsbefehl",
					rollen::STEUERUNGSBEFEHL,
					"ClsSteuersignal",
					NachrichtenTyp::Cls,
				),
				schritt(
					"Steuerinformations-Quittung",
					rollen::STEUERINFORMATIONS_QUITTUNG,
					"MsconsEinspeiseMesswerte",
					NachrichtenTyp::Mscons,
				),
			],
		),
		ProzessDef::new(
			"14a_steuerungsbefehl_gmsb",
			"Steuerungsbefehl gMSB",
			ProzessKategorie::Para14a,
			vec![
				schritt(
					"Steuerungsbefehl",
					rollen::STEUERUNGSBEFEHL_GMSB,
					"ClsSteuersignal",
					NachrichtenTyp::Cls,
				),
				schritt(
					"Steuerinformations-Quittung",
					rollen::STEUERINFORMATIONS_QUITTUNG_GMSB,
					"MsconsEinspeiseMesswerte",
					NachrichtenTyp::Mscons,
				),
			],
		),
		ProzessDef::new(
			"14a_steuerungsquittungen",
			"Steuerungsquittungen",
			ProzessKategorie::Para14a,
			vec![
				schritt(
					"Quittung NB an MSB",
					rollen::QUITTUNG_NB_AN_MSB,
					"MsconsEinspeiseMesswerte",
					NachrichtenTyp::Mscons,
				),
				schritt(
					"Quittung MSB an NB",
					rollen::QUITTUNG_MSB_AN_NB,
					"MsconsEinspeiseMesswerte",
					NachrichtenTyp::Mscons,
				),
				SchrittDef::new(
					"Quittung NB an Lieferant",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantNeu,
					"UtilmdSteuerbareVerbrauchseinrichtung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Quittung Lieferant an NB",
					MarktRolle::LieferantNeu,
					MarktRolle::Netzbetreiber,
					"UtilmdSteuerbareVerbrauchseinrichtung",
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
	fn katalog_deckt_stabile_14a_keys_ab() {
		let keys: Vec<_> = katalog().into_iter().map(|p| p.key).collect();
		assert_eq!(
			keys,
			vec![
				"14a_anmeldung_sve",
				"14a_modul2_daten",
				"14a_information_an_lf",
				"14a_steuerungsbefehl",
				"14a_steuerungsbefehl_gmsb",
				"14a_steuerungsquittungen",
			]
		);
	}

	#[test]
	fn steuerungsbefehle_unterscheiden_msb_und_gmsb_rollenpfad() {
		let prozesse = katalog();
		let msb = prozesse
			.iter()
			.find(|p| p.key == "14a_steuerungsbefehl")
			.expect("14a_steuerungsbefehl");
		assert_eq!(msb.schritte[0].absender, MarktRolle::Netzbetreiber.slug());
		assert_eq!(msb.schritte[0].empfaenger, MarktRolle::Messstellenbetreiber.slug());

		let gmsb = prozesse
			.iter()
			.find(|p| p.key == "14a_steuerungsbefehl_gmsb")
			.expect("14a_steuerungsbefehl_gmsb");
		assert_eq!(gmsb.schritte[0].absender, MarktRolle::Netzbetreiber.slug());
		assert_eq!(
			gmsb.schritte[0].empfaenger,
			MarktRolle::GrundzustaendigerMessstellenbetreiber.slug()
		);
		assert_eq!(
			gmsb.schritte[1].absender,
			MarktRolle::GrundzustaendigerMessstellenbetreiber.slug()
		);
		assert_eq!(gmsb.schritte[1].empfaenger, MarktRolle::Netzbetreiber.slug());
	}
}
