//! UBP-Prozesskatalog für die Test-UI.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

use super::rollen;

pub fn katalog() -> Vec<ProzessDef> {
	let mut prozesse = Vec::new();

	for &(kaeufer, verkaeufer) in rollen::ANGEBOTSANFRAGE {
		prozesse.push(ProzessDef::new(
			&format!("ubp_angebotsanfrage_{}_{}", suffix(kaeufer), suffix(verkaeufer)),
			&format!("Angebotsanfrage {} → {}", kaeufer.kuerzel(), verkaeufer.kuerzel()),
			ProzessKategorie::Ubp,
			vec![
				SchrittDef::new(
					"Angebotsanfrage",
					kaeufer,
					verkaeufer,
					"ReqoteAngebotsanfrage",
					NachrichtenTyp::Reqote,
				),
				SchrittDef::new(
					"Angebot",
					verkaeufer,
					kaeufer,
					"QuotesAngebot",
					NachrichtenTyp::Quotes,
				),
			],
		));
	}

	for &(kaeufer, verkaeufer) in rollen::BESTELLUNG {
		prozesse.push(ProzessDef::new(
			&format!("ubp_bestellung_{}_{}", suffix(kaeufer), suffix(verkaeufer)),
			&format!("Bestellung {} → {}", kaeufer.kuerzel(), verkaeufer.kuerzel()),
			ProzessKategorie::Ubp,
			vec![
				SchrittDef::new(
					"Bestellung",
					kaeufer,
					verkaeufer,
					"OrdersBestellung",
					NachrichtenTyp::Orders,
				),
				SchrittDef::new(
					"Bestellantwort",
					verkaeufer,
					kaeufer,
					"OrdrspBestellantwort",
					NachrichtenTyp::Ordrsp,
				),
			],
		));
	}

	for &(herausgeber, empfaenger) in rollen::PREISBLATT {
		prozesse.push(ProzessDef::new(
			&format!("ubp_preisblatt_{}_{}", suffix(herausgeber), suffix(empfaenger)),
			&format!("Preisblatt {} → {}", herausgeber.kuerzel(), empfaenger.kuerzel()),
			ProzessKategorie::Ubp,
			vec![SchrittDef::new(
				"Preisblatt",
				herausgeber,
				empfaenger,
				"PricatPreisblatt",
				NachrichtenTyp::Pricat,
			)],
		));
	}

	prozesse
}

fn suffix(rolle: MarktRolle) -> &'static str {
	match rolle {
		MarktRolle::Lieferant => "lf",
		MarktRolle::Netzbetreiber => "nb",
		MarktRolle::Messstellenbetreiber => "msb",
		MarktRolle::GrundzustaendigerMessstellenbetreiber => "gmsb",
		MarktRolle::WettbewerblicherMessstellenbetreiber => "wmsb",
		MarktRolle::Energieserviceanbieter => "esa",
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
			rollen::ANGEBOTSANFRAGE.len() + rollen::BESTELLUNG.len() + rollen::PREISBLATT.len()
		);
		assert!(prozesse.iter().any(|p| p.key == "ubp_bestellung_lf_msb"));
		assert!(prozesse.iter().any(|p| p.key == "ubp_preisblatt_wmsb_nb"));
	}

	#[test]
	fn bestellung_lf_msb_nutzt_rollentupel() {
		let prozess = katalog()
			.into_iter()
			.find(|p| p.key == "ubp_bestellung_lf_msb")
			.expect("ubp_bestellung_lf_msb");
		let erster = prozess.schritte.first().expect("erster Schritt");
		assert_eq!(
			(erster.absender.as_str(), erster.empfaenger.as_str()),
			(rollen::BESTELLUNG[0].0.slug(), rollen::BESTELLUNG[0].1.slug())
		);
	}
}
