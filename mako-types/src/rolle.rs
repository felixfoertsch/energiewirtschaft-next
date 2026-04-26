use serde::{Deserialize, Serialize};

use crate::sparte::Sparte;

/// All market roles in German energy market communication.
/// Roles are tagged with which Sparte(n) they participate in.
///
/// Naming: full German role names. The official 2-4 letter shorthand
/// (NB, ÜNB, LF, BKV, MSB, BTR, EIV, DP, ANB, ...) appears in BNetzA AHB/EBD
/// reference data; we keep that mapping in `kuerzel()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarktRolle {
	// ---------------------------------------------------------------
	// spartenübergreifend (Strom + Gas)
	// ---------------------------------------------------------------
	Lieferant,
	LieferantNeu,
	LieferantAlt,
	/// Ersatz-/Grundversorger (E/G) — Lieferanten-Spezialisierung für die
	/// Ersatz- bzw. Grundversorgung gemäß EnWG §38 / §36.
	LieferantErsatzGrundversorgung,
	Netzbetreiber,
	NetzbetreiberAlt,
	NetzbetreiberNeu,
	Messstellenbetreiber,
	MessstellenbetreiberAlt,
	MessstellenbetreiberNeu,
	/// Grundzuständiger Messstellenbetreiber (gMSB) — MsbG §3 Nr. 4.
	GrundzustaendigerMessstellenbetreiber,
	Messdienstleister,
	Bilanzkreisverantwortlicher,
	/// Abstrakte Rolle in Abrechnungs-Prozessen (REMADV/INVOIC) — der
	/// Aussteller der Rechnung. Konkret meist NB/LF/MSB/MGV.
	Rechnungsersteller,
	/// Abstrakte Rolle in Abrechnungs-Prozessen — der Empfänger der Rechnung.
	Rechnungsempfaenger,

	// ---------------------------------------------------------------
	// nur Strom
	// ---------------------------------------------------------------
	Uebertragungsnetzbetreiber,
	Bilanzkoordinator,
	/// Anschlussnetzbetreiber (ANB) — fachliche NB-Spezialisierung in
	/// Redispatch 2.0 und MaBiS-Prozessen.
	Anschlussnetzbetreiber,
	/// Anfordernder Netzbetreiber (anfNB) — Redispatch-Anforderer.
	AnfordernderNetzbetreiber,
	/// Wettbewerblicher Messstellenbetreiber (wMSB) — MsbG §41.
	WettbewerblicherMessstellenbetreiber,
	Einsatzverantwortlicher,
	/// Betreiber der Technischen Ressource (BTR) — Redispatch 2.0,
	/// XML-Code A21 "Producer".
	BetreiberTechnischeRessource,
	/// Data Provider (DP) — Redispatch 2.0 Verteilerfunktion,
	/// XML-Code A39 "Data provider".
	DataProvider,
	BetreiberErzeugungsanlage,
	Direktvermarkter,
	Energieserviceanbieter,
	Aggregator,
	/// Ladepunktbetreiber (LPB) — Betreiber von Ladepunkten für Elektrofahrzeuge.
	Ladepunktbetreiber,
	/// Registerbetreiber Herkunftsnachweisregister (RB-HKNR / UBA).
	RegisterbetreiberHknr,

	// ---------------------------------------------------------------
	// nur Gas
	// ---------------------------------------------------------------
	Fernleitungsnetzbetreiber,
	Marktgebietsverantwortlicher,
	Transportkunde,
	/// Kapazitätsnutzer (KN) — Gas/KoV Kapazitätsabrechnung.
	Kapazitaetsnutzer,
	Speicherstellenbetreiber,
	Einspeisenetzbetreiber,
	Ausspeisenetzbetreiber,
}

impl MarktRolle {
	/// Returns the Sparten in which this role participates.
	pub fn sparten(&self) -> &'static [Sparte] {
		use MarktRolle::*;
		match self {
			// spartenübergreifend
			Lieferant
			| LieferantNeu
			| LieferantAlt
			| LieferantErsatzGrundversorgung
			| Netzbetreiber
			| NetzbetreiberAlt
			| NetzbetreiberNeu
			| Messstellenbetreiber
			| MessstellenbetreiberAlt
			| MessstellenbetreiberNeu
			| GrundzustaendigerMessstellenbetreiber
			| Messdienstleister
			| Bilanzkreisverantwortlicher
			| Rechnungsersteller
			| Rechnungsempfaenger => &[Sparte::Strom, Sparte::Gas],

			// nur Strom
			Uebertragungsnetzbetreiber
			| Bilanzkoordinator
			| Anschlussnetzbetreiber
			| AnfordernderNetzbetreiber
			| WettbewerblicherMessstellenbetreiber
			| Einsatzverantwortlicher
			| BetreiberTechnischeRessource
			| DataProvider
			| BetreiberErzeugungsanlage
			| Direktvermarkter
			| Energieserviceanbieter
			| Aggregator
			| Ladepunktbetreiber
			| RegisterbetreiberHknr => &[Sparte::Strom],

			// nur Gas
			Fernleitungsnetzbetreiber
			| Marktgebietsverantwortlicher
			| Transportkunde
			| Kapazitaetsnutzer
			| Speicherstellenbetreiber
			| Einspeisenetzbetreiber
			| Ausspeisenetzbetreiber => &[Sparte::Gas],
		}
	}

	/// Stable filesystem/wire slug — same identifiers `mako-cli init` uses
	/// for role directory names. Keep in sync with `mako-cli/src/init.rs::ROLLEN`.
	pub fn slug(&self) -> &'static str {
		use MarktRolle::*;
		match self {
			Lieferant => "lieferant",
			LieferantNeu => "lieferant_neu",
			LieferantAlt => "lieferant_alt",
			LieferantErsatzGrundversorgung => "lieferant_ersatz_grundversorgung",
			Netzbetreiber => "netzbetreiber",
			NetzbetreiberAlt => "netzbetreiber_alt",
			NetzbetreiberNeu => "netzbetreiber_neu",
			Anschlussnetzbetreiber => "anschlussnetzbetreiber",
			AnfordernderNetzbetreiber => "anfordernder_netzbetreiber",
			Uebertragungsnetzbetreiber => "uebertragungsnetzbetreiber",
			Fernleitungsnetzbetreiber => "fernleitungsnetzbetreiber",
			Messstellenbetreiber => "messstellenbetreiber",
			MessstellenbetreiberAlt => "messstellenbetreiber_alt",
			MessstellenbetreiberNeu => "messstellenbetreiber_neu",
			GrundzustaendigerMessstellenbetreiber => "grundzustaendiger_messstellenbetreiber",
			WettbewerblicherMessstellenbetreiber => "wettbewerblicher_messstellenbetreiber",
			Messdienstleister => "messdienstleister",
			Bilanzkreisverantwortlicher => "bilanzkreisverantwortlicher",
			Bilanzkoordinator => "bilanzkoordinator",
			Marktgebietsverantwortlicher => "marktgebietsverantwortlicher",
			BetreiberTechnischeRessource => "betreiber_technische_ressource",
			Einsatzverantwortlicher => "einsatzverantwortlicher",
			DataProvider => "data_provider",
			BetreiberErzeugungsanlage => "betreiber_erzeugungsanlage",
			Direktvermarkter => "direktvermarkter",
			Energieserviceanbieter => "energieserviceanbieter",
			Aggregator => "aggregator",
			Ladepunktbetreiber => "ladepunktbetreiber",
			RegisterbetreiberHknr => "registerbetreiber_hknr",
			Transportkunde => "transportkunde",
			Kapazitaetsnutzer => "kapazitaetsnutzer",
			Speicherstellenbetreiber => "speicherstellenbetreiber",
			Einspeisenetzbetreiber => "einspeisenetzbetreiber",
			Ausspeisenetzbetreiber => "ausspeisenetzbetreiber",
			Rechnungsersteller => "rechnungsersteller",
			Rechnungsempfaenger => "rechnungsempfaenger",
		}
	}

	/// Official BDEW/BNetzA shorthand used in AHB/EBD reference data.
	pub fn kuerzel(&self) -> &'static str {
		use MarktRolle::*;
		match self {
			Lieferant => "LF",
			LieferantNeu => "LFN",
			LieferantAlt => "LFA",
			LieferantErsatzGrundversorgung => "E/G",
			Netzbetreiber => "NB",
			NetzbetreiberAlt => "NBA",
			NetzbetreiberNeu => "NBN",
			Anschlussnetzbetreiber => "ANB",
			AnfordernderNetzbetreiber => "anfNB",
			Uebertragungsnetzbetreiber => "ÜNB",
			Fernleitungsnetzbetreiber => "FNB",
			Messstellenbetreiber => "MSB",
			MessstellenbetreiberAlt => "MSBA",
			MessstellenbetreiberNeu => "MSBN",
			GrundzustaendigerMessstellenbetreiber => "gMSB",
			WettbewerblicherMessstellenbetreiber => "wMSB",
			Messdienstleister => "MDL",
			Bilanzkreisverantwortlicher => "BKV",
			Bilanzkoordinator => "BIKO",
			Marktgebietsverantwortlicher => "MGV",
			BetreiberTechnischeRessource => "BTR",
			Einsatzverantwortlicher => "EIV",
			DataProvider => "DP",
			BetreiberErzeugungsanlage => "BEA",
			Direktvermarkter => "DV",
			Energieserviceanbieter => "ESA",
			Aggregator => "AGG",
			Ladepunktbetreiber => "LPB",
			RegisterbetreiberHknr => "RB-HKNR",
			Transportkunde => "TK",
			Kapazitaetsnutzer => "KN",
			Speicherstellenbetreiber => "SSB",
			Einspeisenetzbetreiber => "ENB",
			Ausspeisenetzbetreiber => "ANBG",
			Rechnungsersteller => "ReErst",
			Rechnungsempfaenger => "ReEmpf",
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lieferant_is_spartenuebergreifend() {
		let sparten = MarktRolle::Lieferant.sparten();
		assert!(sparten.contains(&Sparte::Strom));
		assert!(sparten.contains(&Sparte::Gas));
	}

	#[test]
	fn uebertragungsnetzbetreiber_is_strom_only() {
		let sparten = MarktRolle::Uebertragungsnetzbetreiber.sparten();
		assert!(sparten.contains(&Sparte::Strom));
		assert!(!sparten.contains(&Sparte::Gas));
	}

	#[test]
	fn fernleitungsnetzbetreiber_is_gas_only() {
		let sparten = MarktRolle::Fernleitungsnetzbetreiber.sparten();
		assert!(!sparten.contains(&Sparte::Strom));
		assert!(sparten.contains(&Sparte::Gas));
	}

	#[test]
	fn btr_is_strom_only() {
		// Betreiber der Technischen Ressource ist eine Redispatch-2.0-Rolle (Strom).
		let sparten = MarktRolle::BetreiberTechnischeRessource.sparten();
		assert_eq!(sparten, &[Sparte::Strom]);
	}

	#[test]
	fn data_provider_is_strom_only() {
		let sparten = MarktRolle::DataProvider.sparten();
		assert_eq!(sparten, &[Sparte::Strom]);
	}

	#[test]
	fn kapazitaetsnutzer_is_gas_only() {
		let sparten = MarktRolle::Kapazitaetsnutzer.sparten();
		assert_eq!(sparten, &[Sparte::Gas]);
	}

	#[test]
	fn rechnungsersteller_is_spartenuebergreifend() {
		// Abrechnung gibt es in beiden Sparten.
		let sparten = MarktRolle::Rechnungsersteller.sparten();
		assert!(sparten.contains(&Sparte::Strom));
		assert!(sparten.contains(&Sparte::Gas));
	}

	#[test]
	fn kuerzel_distinct_for_known_pairs() {
		// Sicherstellen, dass die offiziellen Abkürzungen pro Variante stimmen.
		assert_eq!(MarktRolle::BetreiberTechnischeRessource.kuerzel(), "BTR");
		assert_eq!(MarktRolle::Einsatzverantwortlicher.kuerzel(), "EIV");
		assert_eq!(MarktRolle::DataProvider.kuerzel(), "DP");
		assert_eq!(MarktRolle::Anschlussnetzbetreiber.kuerzel(), "ANB");
		assert_eq!(MarktRolle::AnfordernderNetzbetreiber.kuerzel(), "anfNB");
		assert_eq!(MarktRolle::GrundzustaendigerMessstellenbetreiber.kuerzel(), "gMSB");
		assert_eq!(MarktRolle::Bilanzkoordinator.kuerzel(), "BIKO");
		assert_eq!(MarktRolle::Marktgebietsverantwortlicher.kuerzel(), "MGV");
	}
}
