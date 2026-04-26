//! GABi-Gas-Prozesskatalog für die Test-UI.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

use super::rollen;

pub fn katalog() -> Vec<ProzessDef> {
	vec![
		ProzessDef::new(
			"gabi_nominierung",
			"Nominierung",
			ProzessKategorie::GabiGas,
			vec![
				schritt("Nominierung", rollen::NOMINIERUNG, "Nominierung", NachrichtenTyp::Mscons),
				SchrittDef::new(
					"Bestätigung",
					MarktRolle::Marktgebietsverantwortlicher,
					MarktRolle::Bilanzkreisverantwortlicher,
					"NominierungBestaetigung",
					NachrichtenTyp::Mscons,
				),
				schritt(
					"Renominierung",
					rollen::NOMINIERUNG,
					"Renominierung",
					NachrichtenTyp::Mscons,
				),
			],
		),
		ProzessDef::new(
			"gabi_allokation",
			"Allokation",
			ProzessKategorie::GabiGas,
			vec![schritt(
				"Allokationsdaten",
				rollen::ALLOKATION,
				"MsconsAggregierteZeitreihen",
				NachrichtenTyp::Mscons,
			)],
		),
		ProzessDef::new(
			"gabi_tageswerte",
			"Tageswerte",
			ProzessKategorie::GabiGas,
			vec![schritt(
				"Tageswerte",
				rollen::TAGESWERTE,
				"MsconsAggregierteZeitreihen",
				NachrichtenTyp::Mscons,
			)],
		),
		ProzessDef::new(
			"gabi_korrektur",
			"Korrektur",
			ProzessKategorie::GabiGas,
			vec![schritt(
				"Korrektur",
				rollen::KORREKTUR,
				"MsconsAggregierteZeitreihen",
				NachrichtenTyp::Mscons,
			)],
		),
		ProzessDef::new(
			"gabi_bilanzkreiszuordnung",
			"Bilanzkreiszuordnung Gas",
			ProzessKategorie::GabiGas,
			vec![
				SchrittDef::new(
					"Zuordnung LF → NB",
					rollen::BILANZKREISZUORDNUNG_GAS[0],
					rollen::BILANZKREISZUORDNUNG_GAS[1],
					"UtilmdBilanzkreiszuordnung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Zuordnung NB → BKV",
					rollen::BILANZKREISZUORDNUNG_GAS[1],
					rollen::BILANZKREISZUORDNUNG_GAS[2],
					"UtilmdBilanzkreiszuordnung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Zuordnung BKV → MGV",
					rollen::BILANZKREISZUORDNUNG_GAS[2],
					rollen::BILANZKREISZUORDNUNG_GAS[3],
					"UtilmdBilanzkreiszuordnung",
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
	fn katalog_deckt_gabi_rollenpfade_ab() {
		let keys: Vec<_> = katalog().into_iter().map(|p| p.key).collect();
		assert_eq!(keys.len(), 5);
		assert!(keys.contains(&"gabi_nominierung".to_string()));
		assert!(keys.contains(&"gabi_bilanzkreiszuordnung".to_string()));
	}

	#[test]
	fn nominierung_nutzt_rollenpfad() {
		let prozess = katalog()
			.into_iter()
			.find(|p| p.key == "gabi_nominierung")
			.expect("gabi_nominierung");
		let erster = prozess.schritte.first().expect("erster Schritt");
		assert_eq!(erster.absender, rollen::NOMINIERUNG[0].slug());
		assert_eq!(erster.empfaenger, rollen::NOMINIERUNG[1].slug());
	}
}
