use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::Nachricht;
use mako_types::rolle::MarktRolle;

/// A simulated market participant that can send and receive messages.
#[derive(Debug, Clone)]
pub struct MarktAgent {
	pub id: MarktpartnerId,
	pub rolle: MarktRolle,
	pub posteingang: Vec<Nachricht>,
	pub postausgang: Vec<Nachricht>,
}

impl MarktAgent {
	pub fn new(id: MarktpartnerId, rolle: MarktRolle) -> Self {
		Self {
			id,
			rolle,
			posteingang: vec![],
			postausgang: vec![],
		}
	}

	/// Receive a message into the inbox.
	pub fn empfangen(&mut self, nachricht: Nachricht) {
		self.posteingang.push(nachricht);
	}

	/// Send a message (records it in the outbox).
	pub fn senden(&mut self, nachricht: Nachricht) {
		self.postausgang.push(nachricht);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::NaiveDate;
	use mako_types::gpke_nachrichten::UtilmdAnmeldung;
	use mako_types::ids::MaLoId;
	use mako_types::nachricht::NachrichtenPayload;

	#[test]
	fn agent_receives_message() {
		let mut agent = MarktAgent::new(
			MarktpartnerId::new("9900000000010").unwrap(),
			MarktRolle::Netzbetreiber,
		);
		assert!(agent.posteingang.is_empty());

		let nachricht = Nachricht {
			absender: MarktpartnerId::new("9900000000003").unwrap(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: MarktpartnerId::new("9900000000010").unwrap(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696788").unwrap(),
				lieferant_neu: MarktpartnerId::new("9900000000003").unwrap(),
				lieferbeginn: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
			}),
		};

		agent.empfangen(nachricht);
		assert_eq!(agent.posteingang.len(), 1);
	}

	#[test]
	fn agent_sends_message() {
		let mut agent = MarktAgent::new(
			MarktpartnerId::new("9900000000003").unwrap(),
			MarktRolle::LieferantNeu,
		);

		let nachricht = Nachricht {
			absender: MarktpartnerId::new("9900000000003").unwrap(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: MarktpartnerId::new("9900000000010").unwrap(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696788").unwrap(),
				lieferant_neu: MarktpartnerId::new("9900000000003").unwrap(),
				lieferbeginn: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
			}),
		};

		agent.senden(nachricht);
		assert_eq!(agent.postausgang.len(), 1);
	}
}
