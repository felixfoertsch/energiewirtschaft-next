use serde::{Deserialize, Serialize};

use crate::sparte::Sparte;

/// All market roles in German energy market communication.
/// Roles are tagged with which Sparte(n) they participate in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarktRolle {
	// spartenuebergreifend
	Lieferant,
	LieferantNeu,
	LieferantAlt,
	Netzbetreiber,
	Messstellenbetreiber,
	Messdienstleister,
	Bilanzkreisverantwortlicher,

	// nur Strom
	Uebertragungsnetzbetreiber,
	Bilanzkoordinator,
	Einsatzverantwortlicher,
	BetreiberErzeugungsanlage,
	Direktvermarkter,
	Energieserviceanbieter,
	Aggregator,

	// nur Gas
	Fernleitungsnetzbetreiber,
	Marktgebietsverantwortlicher,
	Transportkunde,
	Speicherstellenbetreiber,
	Einspeisenetzbetreiber,
	Ausspeisenetzbetreiber,
}

impl MarktRolle {
	/// Returns the Sparten in which this role participates.
	pub fn sparten(&self) -> &'static [Sparte] {
		use MarktRolle::*;
		match self {
			Lieferant | LieferantNeu | LieferantAlt | Netzbetreiber
			| Messstellenbetreiber | Messdienstleister
			| Bilanzkreisverantwortlicher => &[Sparte::Strom, Sparte::Gas],

			Uebertragungsnetzbetreiber | Bilanzkoordinator
			| Einsatzverantwortlicher | BetreiberErzeugungsanlage
			| Direktvermarkter | Energieserviceanbieter | Aggregator => &[Sparte::Strom],

			Fernleitungsnetzbetreiber | Marktgebietsverantwortlicher
			| Transportkunde | Speicherstellenbetreiber
			| Einspeisenetzbetreiber | Ausspeisenetzbetreiber => &[Sparte::Gas],
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
}
