use mako_types::nachricht::Nachricht;

use crate::types::{FehlerCode, QuittungsErgebnis};

/// Syntax-level validation (CONTRL): checks that sender and recipient IDs are non-empty.
pub fn contrl_pruefen(nachricht: &Nachricht) -> QuittungsErgebnis {
	if nachricht.absender.as_str().is_empty() {
		return QuittungsErgebnis::Negativ(FehlerCode::AbsenderLeer);
	}
	if nachricht.empfaenger.as_str().is_empty() {
		return QuittungsErgebnis::Negativ(FehlerCode::EmpfaengerLeer);
	}
	QuittungsErgebnis::Positiv
}

#[cfg(test)]
mod tests {
	use chrono::NaiveDate;

	use mako_types::gpke_nachrichten::UtilmdAnmeldung;
	use mako_types::ids::{MaLoId, MarktpartnerId};
	use mako_types::nachricht::{Nachricht, NachrichtenPayload};
	use mako_types::rolle::MarktRolle;

	use super::*;

	fn test_nachricht() -> Nachricht {
		Nachricht {
			absender: MarktpartnerId::new("9900000000003").unwrap(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: MarktpartnerId::new("9900000000010").unwrap(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696788").unwrap(),
				lieferant_neu: MarktpartnerId::new("9900000000003").unwrap(),
				lieferbeginn: NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
			}),
		}
	}

	#[test]
	fn valid_message_passes_contrl() {
		let nachricht = test_nachricht();
		let ergebnis = contrl_pruefen(&nachricht);
		assert_eq!(ergebnis, QuittungsErgebnis::Positiv);
	}
}
