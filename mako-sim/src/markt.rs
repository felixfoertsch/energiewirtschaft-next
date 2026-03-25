use std::collections::HashMap;

use mako_types::nachricht::Nachricht;

use crate::agent::MarktAgent;

/// A simulated market with multiple agents.
#[derive(Debug, Clone)]
pub struct Markt {
	pub agenten: HashMap<String, MarktAgent>,
	pub event_log: Vec<Nachricht>,
}

impl Markt {
	pub fn new() -> Self {
		Self {
			agenten: HashMap::new(),
			event_log: vec![],
		}
	}

	/// Add an agent to the market.
	pub fn agent_hinzufuegen(&mut self, agent: MarktAgent) {
		self.agenten
			.insert(agent.id.as_str().to_string(), agent);
	}

	/// Route a message from sender to recipient.
	pub fn nachricht_zustellen(&mut self, nachricht: Nachricht) {
		self.event_log.push(nachricht.clone());
		let empfaenger_id = nachricht.empfaenger.as_str().to_string();
		if let Some(agent) = self.agenten.get_mut(&empfaenger_id) {
			agent.empfangen(nachricht);
		}
	}
}

impl Default for Markt {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::NaiveDate;
	use mako_types::gpke_nachrichten::{
		UtilmdAbmeldung, UtilmdAnmeldung, UtilmdBestaetigung, UtilmdZuordnung,
	};
	use mako_types::ids::{MaLoId, MarktpartnerId};
	use mako_types::nachricht::NachrichtenPayload;
	use mako_types::rolle::MarktRolle;

	use crate::agent::MarktAgent;

	fn lfn() -> MarktAgent {
		MarktAgent::new(
			MarktpartnerId::new("9900000000003").unwrap(),
			MarktRolle::LieferantNeu,
		)
	}

	fn lfa() -> MarktAgent {
		MarktAgent::new(
			MarktpartnerId::new("9900000000027").unwrap(),
			MarktRolle::LieferantAlt,
		)
	}

	fn nb() -> MarktAgent {
		MarktAgent::new(
			MarktpartnerId::new("9900000000010").unwrap(),
			MarktRolle::Netzbetreiber,
		)
	}

	fn malo() -> MaLoId {
		MaLoId::new("51238696788").unwrap()
	}

	fn mp_lfn() -> MarktpartnerId {
		MarktpartnerId::new("9900000000003").unwrap()
	}

	fn mp_nb() -> MarktpartnerId {
		MarktpartnerId::new("9900000000010").unwrap()
	}

	fn mp_lfa() -> MarktpartnerId {
		MarktpartnerId::new("9900000000027").unwrap()
	}

	#[test]
	fn create_markt_with_three_agents() {
		let mut markt = Markt::new();
		markt.agent_hinzufuegen(lfn());
		markt.agent_hinzufuegen(lfa());
		markt.agent_hinzufuegen(nb());
		assert_eq!(markt.agenten.len(), 3);
	}

	#[test]
	fn route_anmeldung_to_nb() {
		let mut markt = Markt::new();
		markt.agent_hinzufuegen(lfn());
		markt.agent_hinzufuegen(nb());

		let anmeldung = Nachricht {
			absender: mp_lfn(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: mp_nb(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: malo(),
				lieferant_neu: mp_lfn(),
				lieferbeginn: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
			}),
		};

		markt.nachricht_zustellen(anmeldung);

		let nb_agent = markt.agenten.get("9900000000010").unwrap();
		assert_eq!(nb_agent.posteingang.len(), 1);
		assert!(matches!(
			nb_agent.posteingang[0].payload,
			NachrichtenPayload::UtilmdAnmeldung(_)
		));
	}

	#[test]
	fn event_log_captures_all_messages() {
		let mut markt = Markt::new();
		markt.agent_hinzufuegen(lfn());
		markt.agent_hinzufuegen(nb());

		let msg = Nachricht {
			absender: mp_lfn(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: mp_nb(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: malo(),
				lieferant_neu: mp_lfn(),
				lieferbeginn: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
			}),
		};

		markt.nachricht_zustellen(msg);
		assert_eq!(markt.event_log.len(), 1);
	}

	#[test]
	fn message_to_unknown_agent_still_logged() {
		let mut markt = Markt::new();
		markt.agent_hinzufuegen(lfn());
		// NB not added

		let msg = Nachricht {
			absender: mp_lfn(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: mp_nb(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: malo(),
				lieferant_neu: mp_lfn(),
				lieferbeginn: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
			}),
		};

		markt.nachricht_zustellen(msg);
		assert_eq!(markt.event_log.len(), 1);
		// NB doesn't exist, so no agent received it
	}

	/// Full GPKE Lieferbeginn (LFW) simulation:
	/// 1. LFN -> NB: Anmeldung
	/// 2. NB -> LFN: Bestaetigung
	/// 3. NB -> LFA: Abmeldung
	/// 4. NB -> LFN: Zuordnung
	/// 5. NB -> LFA: Zuordnung
	#[test]
	fn gpke_lfw_full_simulation() {
		let mut markt = Markt::new();
		markt.agent_hinzufuegen(lfn());
		markt.agent_hinzufuegen(lfa());
		markt.agent_hinzufuegen(nb());

		let lieferbeginn = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

		// Step 1: LFN -> NB Anmeldung
		let anmeldung = Nachricht {
			absender: mp_lfn(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: mp_nb(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: malo(),
				lieferant_neu: mp_lfn(),
				lieferbeginn,
			}),
		};
		markt.nachricht_zustellen(anmeldung);

		// Step 2: NB -> LFN Bestaetigung
		let bestaetigung_lfn = Nachricht {
			absender: mp_nb(),
			absender_rolle: MarktRolle::Netzbetreiber,
			empfaenger: mp_lfn(),
			empfaenger_rolle: MarktRolle::LieferantNeu,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdBestaetigung(UtilmdBestaetigung {
				malo_id: malo(),
				bestaetigt_fuer: mp_lfn(),
				lieferbeginn,
			}),
		};
		markt.nachricht_zustellen(bestaetigung_lfn);

		// Step 3: NB -> LFA Abmeldung
		let abmeldung = Nachricht {
			absender: mp_nb(),
			absender_rolle: MarktRolle::Netzbetreiber,
			empfaenger: mp_lfa(),
			empfaenger_rolle: MarktRolle::LieferantAlt,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAbmeldung(UtilmdAbmeldung {
				malo_id: malo(),
				lieferant_alt: mp_lfa(),
				lieferende: lieferbeginn,
			}),
		};
		markt.nachricht_zustellen(abmeldung);

		// Step 4: NB -> LFN Zuordnung
		let zuordnung_lfn = Nachricht {
			absender: mp_nb(),
			absender_rolle: MarktRolle::Netzbetreiber,
			empfaenger: mp_lfn(),
			empfaenger_rolle: MarktRolle::LieferantNeu,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdZuordnung(UtilmdZuordnung {
				malo_id: malo(),
				zugeordnet_an: mp_lfn(),
				lieferbeginn,
			}),
		};
		markt.nachricht_zustellen(zuordnung_lfn);

		// Step 5: NB -> LFA Zuordnung
		let zuordnung_lfa = Nachricht {
			absender: mp_nb(),
			absender_rolle: MarktRolle::Netzbetreiber,
			empfaenger: mp_lfa(),
			empfaenger_rolle: MarktRolle::LieferantAlt,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdZuordnung(UtilmdZuordnung {
				malo_id: malo(),
				zugeordnet_an: mp_lfn(),
				lieferbeginn,
			}),
		};
		markt.nachricht_zustellen(zuordnung_lfa);

		// Verify final state
		assert_eq!(markt.event_log.len(), 5);

		let lfn_agent = markt.agenten.get("9900000000003").unwrap();
		assert_eq!(lfn_agent.posteingang.len(), 2); // Bestaetigung + Zuordnung

		let lfa_agent = markt.agenten.get("9900000000027").unwrap();
		assert_eq!(lfa_agent.posteingang.len(), 2); // Abmeldung + Zuordnung

		let nb_agent = markt.agenten.get("9900000000010").unwrap();
		assert_eq!(nb_agent.posteingang.len(), 1); // Anmeldung

		// Verify message types received
		assert!(matches!(
			lfn_agent.posteingang[0].payload,
			NachrichtenPayload::UtilmdBestaetigung(_)
		));
		assert!(matches!(
			lfn_agent.posteingang[1].payload,
			NachrichtenPayload::UtilmdZuordnung(_)
		));
		assert!(matches!(
			lfa_agent.posteingang[0].payload,
			NachrichtenPayload::UtilmdAbmeldung(_)
		));
		assert!(matches!(
			lfa_agent.posteingang[1].payload,
			NachrichtenPayload::UtilmdZuordnung(_)
		));
	}
}
