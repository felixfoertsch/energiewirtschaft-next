//! Abrechnungs-Prozesskatalog für die Test-UI.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

use super::rollen;

pub fn katalog() -> Vec<ProzessDef> {
	let mut prozesse = Vec::new();

	for &(absender, empfaenger) in rollen::RECHNUNG_INVOIC {
		prozesse.push(ProzessDef::new(
			&format!("abrechnung_invoic_{}_{}", suffix(absender), suffix(empfaenger)),
			&format!("Rechnung {} → {}", absender.kuerzel(), empfaenger.kuerzel()),
			ProzessKategorie::Abrechnung,
			vec![SchrittDef::new(
				"Rechnung",
				absender,
				empfaenger,
				"InvoicRechnung",
				NachrichtenTyp::Invoic,
			)],
		));
	}

	for &(absender, empfaenger) in rollen::ZAHLUNGSAVIS_REMADV {
		prozesse.push(ProzessDef::new(
			&format!("abrechnung_remadv_{}_{}", suffix(absender), suffix(empfaenger)),
			&format!("Zahlungsavis {} → {}", absender.kuerzel(), empfaenger.kuerzel()),
			ProzessKategorie::Abrechnung,
			vec![SchrittDef::new(
				"Zahlungsavis",
				absender,
				empfaenger,
				"RemadvZahlungsavis",
				NachrichtenTyp::Remadv,
			)],
		));
	}

	prozesse
}

fn suffix(rolle: MarktRolle) -> &'static str {
	match rolle {
		MarktRolle::Rechnungsersteller => "rechnungsersteller",
		MarktRolle::Rechnungsempfaenger => "rechnungsempfaenger",
		MarktRolle::Messstellenbetreiber => "msb",
		MarktRolle::GrundzustaendigerMessstellenbetreiber => "gmsb",
		MarktRolle::WettbewerblicherMessstellenbetreiber => "wmsb",
		MarktRolle::Netzbetreiber => "nb",
		MarktRolle::Lieferant => "lf",
		MarktRolle::Bilanzkreisverantwortlicher => "bkv",
		MarktRolle::Marktgebietsverantwortlicher => "mgv",
		MarktRolle::Kapazitaetsnutzer => "kn",
		MarktRolle::Fernleitungsnetzbetreiber => "fnb",
		MarktRolle::Transportkunde => "tk",
		_ => "rolle",
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn katalog_enthaelt_eintraege_pro_rollentupel() {
		let prozesse = katalog();
		assert_eq!(
			prozesse.len(),
			rollen::RECHNUNG_INVOIC.len() + rollen::ZAHLUNGSAVIS_REMADV.len()
		);
		assert!(prozesse.iter().any(|p| p.key == "abrechnung_invoic_msb_lf"));
		assert!(prozesse.iter().any(|p| p.key == "abrechnung_remadv_lf_msb"));
	}

	#[test]
	fn invoic_msb_lf_nutzt_rollentupel() {
		let prozess = katalog()
			.into_iter()
			.find(|p| p.key == "abrechnung_invoic_msb_lf")
			.expect("abrechnung_invoic_msb_lf");
		let erster = prozess.schritte.first().expect("erster Schritt");
		assert_eq!(
			(erster.absender.as_str(), erster.empfaenger.as_str()),
			(rollen::RECHNUNG_INVOIC[1].0.slug(), rollen::RECHNUNG_INVOIC[1].1.slug())
		);
	}
}
