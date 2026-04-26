//! KoV-Prozesskatalog für die Test-UI.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

use super::rollen;

pub fn katalog() -> Vec<ProzessDef> {
	vec![
		kapazitaet(
			"kov_kapazitaetsbuchung_tk_mgv",
			"Kapazitätsbuchung TK → MGV",
			rollen::KAPAZITAETSBUCHUNG_TK_MGV,
		),
		kapazitaet(
			"kov_kapazitaetsbuchung_kn_fnb",
			"Kapazitätsbuchung KN → FNB",
			rollen::KAPAZITAETSBUCHUNG_KN_FNB,
		),
		ProzessDef::new(
			"kov_kapazitaetsabrechnung_mgv_kn",
			"Kapazitätsabrechnung MGV → KN",
			ProzessKategorie::KoV,
			abrechnung(rollen::KAPAZITAETSABRECHNUNG_MGV_KN),
		),
		ProzessDef::new(
			"kov_kapazitaetsabrechnung_fnb_tk",
			"Kapazitätsabrechnung FNB → TK",
			ProzessKategorie::KoV,
			abrechnung(rollen::KAPAZITAETSABRECHNUNG_FNB_TK),
		),
		kapazitaet("kov_speicherzugang", "Speicherzugang", rollen::SPEICHERZUGANG),
		ProzessDef::new(
			"kov_ausspeisepunkt",
			"Ausspeisepunkt",
			ProzessKategorie::KoV,
			vec![
				schritt(
					"Anmeldung",
					rollen::AUSSPEISEPUNKT,
					"UtilmdAusspeisepunkt",
					NachrichtenTyp::Utilmd,
				),
				SchrittDef::new(
					"Bestätigung/Ablehnung",
					MarktRolle::Fernleitungsnetzbetreiber,
					MarktRolle::Netzbetreiber,
					"",
					NachrichtenTyp::Intern,
				),
			],
		),
		ProzessDef::new(
			"kov_brennwert",
			"Brennwertmitteilung",
			ProzessKategorie::KoV,
			vec![
				SchrittDef::new(
					"Brennwert NB → LF",
					MarktRolle::Netzbetreiber,
					MarktRolle::Lieferant,
					"MsconsBrennwert",
					NachrichtenTyp::Mscons,
				),
				SchrittDef::new(
					"Brennwert FNB → LF",
					MarktRolle::Fernleitungsnetzbetreiber,
					MarktRolle::Lieferant,
					"MsconsBrennwert",
					NachrichtenTyp::Mscons,
				),
			],
		),
		ProzessDef::new(
			"kov_netzkontoabrechnung",
			"Netzkontoabrechnung",
			ProzessKategorie::KoV,
			abrechnung(rollen::KAPAZITAETSABRECHNUNG_MGV_KN),
		),
	]
}

fn kapazitaet(key: &str, name: &str, pfad: rollen::RollenPfad) -> ProzessDef {
	ProzessDef::new(
		key,
		name,
		ProzessKategorie::KoV,
		vec![
			SchrittDef::new("Anfrage", pfad[0], pfad[1], "", NachrichtenTyp::Intern),
			SchrittDef::new("Bestätigung/Ablehnung", pfad[1], pfad[0], "", NachrichtenTyp::Intern),
		],
	)
}

fn abrechnung(pfad: rollen::RollenPfad) -> Vec<SchrittDef> {
	vec![
		schritt("Rechnung", pfad, "InvoicRechnung", NachrichtenTyp::Invoic),
		SchrittDef::new(
			"Zahlungsavis",
			pfad[1],
			pfad[0],
			"RemadvZahlungsavis",
			NachrichtenTyp::Remadv,
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
	fn katalog_deckt_kov_schluessel_ab() {
		let prozesse = katalog();
		assert_eq!(prozesse.len(), 8);
		assert!(prozesse.iter().any(|p| p.key == "kov_kapazitaetsbuchung_tk_mgv"));
		assert!(prozesse.iter().any(|p| p.key == "kov_netzkontoabrechnung"));
	}

	#[test]
	fn kapazitaetsbuchung_nutzt_rollenpfad() {
		let prozess = katalog()
			.into_iter()
			.find(|p| p.key == "kov_kapazitaetsbuchung_tk_mgv")
			.expect("kov_kapazitaetsbuchung_tk_mgv");
		let erster = prozess.schritte.first().expect("erster Schritt");
		assert_eq!(erster.absender, rollen::KAPAZITAETSBUCHUNG_TK_MGV[0].slug());
		assert_eq!(erster.empfaenger, rollen::KAPAZITAETSBUCHUNG_TK_MGV[1].slug());
	}
}
