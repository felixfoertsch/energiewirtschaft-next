use chrono::{Local, SecondsFormat};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rd2_quittung::{AcknowledgementDocument, AcknowledgementTyp};

pub fn erzeuge_ack(empfangene_nachricht: &Nachricht) -> Nachricht {
	let original_message_mrid = original_message_mrid(empfangene_nachricht);
	let received_at = Local::now().fixed_offset();

	Nachricht {
		absender: empfangene_nachricht.empfaenger.clone(),
		absender_rolle: empfangene_nachricht.empfaenger_rolle,
		empfaenger: empfangene_nachricht.absender.clone(),
		empfaenger_rolle: empfangene_nachricht.absender_rolle,
		pruef_id: None,
		payload: NachrichtenPayload::AcknowledgementDocument(AcknowledgementDocument {
			receiver_mrid: empfangene_nachricht.absender.clone(),
			sender_mrid: empfangene_nachricht.empfaenger.clone(),
			original_message_mrid,
			received_at,
			ack_typ: AcknowledgementTyp::Positiv,
			reason: None,
		}),
	}
}

fn original_message_mrid(nachricht: &Nachricht) -> String {
	let typ = nachricht.payload.typ();
	let raw = format!(
		"{}:{}:{}",
		nachricht.absender.as_str(),
		nachricht.empfaenger.as_str(),
		typ
	);
	raw.replace(':', "-")
}

pub fn received_at_iso(nachricht: &Nachricht) -> Option<String> {
	match &nachricht.payload {
		NachrichtenPayload::AcknowledgementDocument(ack) => {
			Some(ack.received_at.to_rfc3339_opts(SecondsFormat::Secs, true))
		}
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mako_types::gpke_nachrichten::UtilmdAnmeldung;
	use mako_types::ids::{MaLoId, MarktpartnerId};
	use mako_types::rolle::MarktRolle;

	#[test]
	fn erzeuge_ack_tauscht_sender_und_empfaenger() {
		let absender = MarktpartnerId::new("9900000000000").unwrap();
		let empfaenger = MarktpartnerId::new("9900000000001").unwrap();
		let nachricht = Nachricht {
			absender: absender.clone(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: empfaenger.clone(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696788").unwrap(),
				lieferant_neu: absender.clone(),
				lieferbeginn: chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
			}),
		};

		let ack = erzeuge_ack(&nachricht);

		assert_eq!(ack.absender, empfaenger);
		assert_eq!(ack.absender_rolle, MarktRolle::Netzbetreiber);
		assert_eq!(ack.empfaenger, absender);
		assert_eq!(ack.empfaenger_rolle, MarktRolle::LieferantNeu);
		match ack.payload {
			NachrichtenPayload::AcknowledgementDocument(payload) => {
				assert_eq!(payload.ack_typ, AcknowledgementTyp::Positiv);
				assert_eq!(payload.sender_mrid, nachricht.empfaenger);
				assert_eq!(payload.receiver_mrid, nachricht.absender);
			}
			other => panic!("unerwarteter Payload: {other:?}"),
		}
	}
}
