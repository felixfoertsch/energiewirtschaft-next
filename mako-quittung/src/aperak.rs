use chrono::NaiveDate;

use mako_types::nachricht::{Nachricht, NachrichtenPayload};

use crate::types::{FehlerCode, QuittungsErgebnis};

/// Business-level validation (APERAK): for UtilmdAnmeldung, checks that
/// lieferbeginn is strictly after the given stichtag. Other payload types
/// pass through without additional checks.
pub fn aperak_pruefen(nachricht: &Nachricht, stichtag: NaiveDate) -> QuittungsErgebnis {
	match &nachricht.payload {
		NachrichtenPayload::UtilmdAnmeldung(anmeldung) => {
			if anmeldung.lieferbeginn <= stichtag {
				return QuittungsErgebnis::Negativ(FehlerCode::LieferbeginnInVergangenheit);
			}
			QuittungsErgebnis::Positiv
		}
		_ => QuittungsErgebnis::Positiv,
	}
}

#[cfg(test)]
mod tests {
	use mako_types::gpke_nachrichten::UtilmdAnmeldung;
	use mako_types::ids::{MaLoId, MarktpartnerId};
	use mako_types::nachricht::{Nachricht, NachrichtenPayload};
	use mako_types::rolle::MarktRolle;

	use super::*;

	fn nachricht_mit_lieferbeginn(lieferbeginn: NaiveDate) -> Nachricht {
		Nachricht {
			absender: MarktpartnerId::new("9900000000003").unwrap(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: MarktpartnerId::new("9900000000010").unwrap(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696788").unwrap(),
				lieferant_neu: MarktpartnerId::new("9900000000003").unwrap(),
				lieferbeginn,
			}),
		}
	}

	#[test]
	fn future_lieferbeginn_passes_aperak() {
		let stichtag = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();
		let nachricht = nachricht_mit_lieferbeginn(
			NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
		);
		let ergebnis = aperak_pruefen(&nachricht, stichtag);
		assert_eq!(ergebnis, QuittungsErgebnis::Positiv);
	}

	#[test]
	fn past_lieferbeginn_fails_aperak() {
		let stichtag = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();
		let nachricht = nachricht_mit_lieferbeginn(
			NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
		);
		let ergebnis = aperak_pruefen(&nachricht, stichtag);
		assert_eq!(
			ergebnis,
			QuittungsErgebnis::Negativ(FehlerCode::LieferbeginnInVergangenheit)
		);
	}
}
