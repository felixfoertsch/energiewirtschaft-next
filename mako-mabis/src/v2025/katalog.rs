//! MaBiS-Prozesskatalog für die Test-UI.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

use super::rollen;

pub fn katalog() -> Vec<ProzessDef> {
	vec![
		ProzessDef::new(
			"mabis_bilanzkreiszuordnung",
			"Bilanzkreiszuordnung",
			ProzessKategorie::MaBis,
			vec![
				schritt(
					"Zuordnung",
					rollen::BILANZKREISZUORDNUNG,
					"UtilmdBilanzkreiszuordnung",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung",
					MarktRolle::Netzbetreiber,
					MarktRolle::Lieferant,
					"UtilmdBestaetigung",
					NachrichtenTyp::Utilmd,
				),
			],
		),
		ProzessDef::new(
			"mabis_aggregierte_zeitreihen",
			"Aggregierte Zeitreihen",
			ProzessKategorie::MaBis,
			vec![schritt(
				"Aggregierte Zeitreihen",
				rollen::AGGREGIERTE_ZEITREIHEN_LASTGANG_SLP,
				"MsconsAggregierteZeitreihen",
				NachrichtenTyp::Mscons,
			)],
		),
		ProzessDef::new(
			"mabis_mehrmindermengen",
			"Mehr-/Mindermengen",
			ProzessKategorie::MaBis,
			vec![
				schritt(
					"Liste",
					rollen::MEHR_MINDERMENGENLISTE,
					"MsconsMehrMindermengen",
					NachrichtenTyp::Mscons,
				),
				schritt(
					"Rechnung",
					rollen::MEHR_MINDERMENGENLISTE,
					"InvoicRechnung",
					NachrichtenTyp::Invoic,
				),
				SchrittDef::new(
					"Zahlungsavis",
					MarktRolle::Lieferant,
					MarktRolle::Netzbetreiber,
					"RemadvZahlungsavis",
					NachrichtenTyp::Remadv,
				),
			],
		),
		ProzessDef::new(
			"mabis_clearing",
			"Clearing",
			ProzessKategorie::MaBis,
			vec![
				schritt(
					"Clearingliste",
					rollen::CLEARINGLISTE,
					"UtilmdClearingliste",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Antwort",
					MarktRolle::Lieferant,
					MarktRolle::Netzbetreiber,
					"UtilmdClearingliste",
					NachrichtenTyp::Utilmd,
				),
			],
		),
		ProzessDef::new(
			"mabis_bkv_anmeldung",
			"BKV-Anmeldung Bilanzkreis",
			ProzessKategorie::MaBis,
			vec![schritt(
				"Anmeldung",
				rollen::BKV_ANMELDUNG_BILANZKREIS,
				"UtilmdBilanzkreiszuordnung",
				NachrichtenTyp::Utilmd,
			)],
		),
		ProzessDef::new(
			"mabis_biko_bestaetigung",
			"BIKO-Bestätigung",
			ProzessKategorie::MaBis,
			vec![schritt(
				"Bestätigung",
				rollen::BIKO_BESTAETIGUNG,
				"UtilmdBestaetigung",
				NachrichtenTyp::Utilmd,
			)],
		),
		ProzessDef::new(
			"mabis_anb_an_biko",
			"ANB-Übermittlung an BIKO",
			ProzessKategorie::MaBis,
			vec![schritt(
				"Übermittlung",
				rollen::ANB_UEBERMITTLUNG_AN_BIKO,
				"MsconsAggregierteZeitreihen",
				NachrichtenTyp::Mscons,
			)],
		),
		ProzessDef::new(
			"mabis_anfnb_anforderung",
			"anfNB-Anforderung",
			ProzessKategorie::MaBis,
			vec![schritt(
				"Anforderung",
				rollen::ANFNB_ANFORDERUNG,
				"",
				NachrichtenTyp::Intern,
			)],
		),
		ProzessDef::new(
			"mabis_unb_aggregat",
			"ÜNB-Aggregat",
			ProzessKategorie::MaBis,
			vec![schritt(
				"Aggregat",
				rollen::NETZBETREIBER_UEBERTRAGUNGSNETZ_AGGREGATE,
				"MsconsAggregierteZeitreihen",
				NachrichtenTyp::Mscons,
			)],
		),
		ProzessDef::new(
			"mabis_biko_saldenbericht",
			"BIKO-Saldenbericht",
			ProzessKategorie::MaBis,
			vec![schritt(
				"Saldenbericht",
				rollen::BIKO_SALDENBERICHT,
				"MsconsAggregierteZeitreihen",
				NachrichtenTyp::Mscons,
			)],
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
	fn katalog_deckt_alle_mabis_rollenpfade_ab() {
		let prozesse = katalog();
		assert_eq!(prozesse.len(), 10);
		assert!(prozesse.iter().any(|p| p.key == "mabis_bilanzkreiszuordnung"));
		assert!(prozesse.iter().any(|p| p.key == "mabis_biko_saldenbericht"));
	}

	#[test]
	fn bilanzkreiszuordnung_nutzt_rollenpfad() {
		let prozess = katalog()
			.into_iter()
			.find(|p| p.key == "mabis_bilanzkreiszuordnung")
			.expect("mabis_bilanzkreiszuordnung");
		let erster = prozess.schritte.first().expect("erster Schritt");
		assert_eq!(erster.absender, rollen::BILANZKREISZUORDNUNG[0].slug());
		assert_eq!(erster.empfaenger, rollen::BILANZKREISZUORDNUNG[1].slug());
	}
}
