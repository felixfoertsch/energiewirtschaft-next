//! WiM-Prozesskatalog für die Test-UI.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

pub fn katalog() -> Vec<ProzessDef> {
	vec![
		ProzessDef::new(
			"wim_msb_wechsel",
			"MSB-Wechsel",
			ProzessKategorie::Wim,
			vec![
				SchrittDef::new(
					"Anmeldung MSB neu",
					MarktRolle::MessstellenbetreiberNeu,
					MarktRolle::Netzbetreiber,
					"UtilmdMsbWechselAnmeldung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung an MSB neu",
					MarktRolle::Netzbetreiber,
					MarktRolle::MessstellenbetreiberNeu,
					"UtilmdMsbWechselAnmeldung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Konfigurationsdaten anfragen",
					MarktRolle::MessstellenbetreiberNeu,
					MarktRolle::MessstellenbetreiberAlt,
					"UtilmdMsbWechselAnmeldung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Abmeldung MSB alt",
					MarktRolle::Netzbetreiber,
					MarktRolle::MessstellenbetreiberAlt,
					"UtilmdMsbWechselAnmeldung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Schlusszählerstand",
					MarktRolle::MessstellenbetreiberAlt,
					MarktRolle::Netzbetreiber,
					"MsconsSchlussturnusmesswert",
					NachrichtenTyp::Mscons,
				),
			],
		),
		ProzessDef::new(
			"wim_geraetewechsel",
			"Gerätewechsel",
			ProzessKategorie::Wim,
			vec![
				SchrittDef::new(
					"Wechsel melden",
					MarktRolle::Messstellenbetreiber,
					MarktRolle::Netzbetreiber,
					"UtilmdGeraetewechsel",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"NB informiert",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantNeu,
					"UtilmdGeraetewechsel",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Zählerstände übermitteln",
					MarktRolle::Messstellenbetreiber,
					MarktRolle::Netzbetreiber,
					"MsconsSchlussturnusmesswert",
					NachrichtenTyp::Mscons,
				),
			],
		),
		ProzessDef::new(
			"wim_gmab",
			"gMSB-Inanspruchnahme",
			ProzessKategorie::Wim,
			vec![
				SchrittDef::new(
					"Beauftragung",
					MarktRolle::LieferantNeu,
					MarktRolle::GrundzustaendigerMessstellenbetreiber,
					"UtilmdGeschaeftsdatenanfrage",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung",
					MarktRolle::GrundzustaendigerMessstellenbetreiber,
					MarktRolle::LieferantNeu,
					"UtilmdGeschaeftsdatenantwort",
					NachrichtenTyp::Utilmd,
				),
			],
		),
		ProzessDef::new(
			"wim_werte_anfrage",
			"Werte-Anfrage",
			ProzessKategorie::Wim,
			vec![
				SchrittDef::new(
					"Anfrage",
					MarktRolle::LieferantNeu,
					MarktRolle::Messstellenbetreiber,
					"OrdersWerteAnfrage",
					NachrichtenTyp::Orders,
				),
				SchrittDef::new(
					"Werte liefern",
					MarktRolle::Messstellenbetreiber,
					MarktRolle::LieferantNeu,
					"MsconsLastgang",
					NachrichtenTyp::Mscons,
				),
			],
		),
		ProzessDef::new(
			"wim_zaehlwert",
			"Zählwertübermittlung",
			ProzessKategorie::Wim,
			vec![
				SchrittDef::new(
					"Messwerte senden",
					MarktRolle::Messstellenbetreiber,
					MarktRolle::Netzbetreiber,
					"MsconsLastgang",
					NachrichtenTyp::Mscons,
				),
				SchrittDef::new(
					"Plausibilisierte Werte",
					MarktRolle::Netzbetreiber,
					MarktRolle::LieferantNeu,
					"MsconsLastgang",
					NachrichtenTyp::Mscons,
				),
			],
		),
	]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn katalog_deckt_stabile_wim_keys_ab() {
		let keys: Vec<_> = katalog().into_iter().map(|p| p.key).collect();
		assert_eq!(
			keys,
			vec![
				"wim_msb_wechsel",
				"wim_geraetewechsel",
				"wim_gmab",
				"wim_werte_anfrage",
				"wim_zaehlwert",
			]
		);
	}

	#[test]
	fn msb_wechsel_nutzt_alt_neu_rollen_statt_generischem_msb() {
		let msb_wechsel = katalog()
			.into_iter()
			.find(|p| p.key == "wim_msb_wechsel")
			.expect("wim_msb_wechsel");
		assert_eq!(
			msb_wechsel.schritte[0].absender,
			MarktRolle::MessstellenbetreiberNeu.slug()
		);
		assert_eq!(msb_wechsel.schritte[0].empfaenger, MarktRolle::Netzbetreiber.slug());
		assert_eq!(
			msb_wechsel.schritte[2].absender,
			MarktRolle::MessstellenbetreiberNeu.slug()
		);
		assert_eq!(
			msb_wechsel.schritte[2].empfaenger,
			MarktRolle::MessstellenbetreiberAlt.slug()
		);
		assert_eq!(
			msb_wechsel.schritte[3].empfaenger,
			MarktRolle::MessstellenbetreiberAlt.slug()
		);
	}
}
