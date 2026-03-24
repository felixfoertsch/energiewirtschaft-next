use chrono::NaiveDate;

use mako_types::gpke_nachrichten::{UtilmdAbmeldung, UtilmdAnmeldung, UtilmdBestaetigung};
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::{test_malo, test_mp_id};

/// GPKE 1.1.1: Anmeldung LFN -> NB
pub fn anmeldung(lieferbeginn: NaiveDate) -> Nachricht {
	let lfn = test_mp_id(1);
	let nb = test_mp_id(2);
	Nachricht {
		absender: lfn.clone(),
		absender_rolle: MarktRolle::LieferantNeu,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: lfn,
			lieferbeginn,
		}),
	}
}

/// GPKE 1.1.2: Bestaetigung NB -> LFN
pub fn bestaetigung(empfaenger: MarktpartnerId, lieferbeginn: NaiveDate) -> Nachricht {
	let nb = test_mp_id(2);
	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: empfaenger.clone(),
		empfaenger_rolle: MarktRolle::LieferantNeu,
		payload: NachrichtenPayload::UtilmdBestaetigung(UtilmdBestaetigung {
			malo_id: test_malo(0),
			bestaetigt_fuer: empfaenger,
			lieferbeginn,
		}),
	}
}

/// GPKE 1.1.3: Abmeldung NB -> LFA
pub fn abmeldung(lfa: MarktpartnerId, lieferende: NaiveDate) -> Nachricht {
	let nb = test_mp_id(2);
	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lfa.clone(),
		empfaenger_rolle: MarktRolle::LieferantAlt,
		payload: NachrichtenPayload::UtilmdAbmeldung(UtilmdAbmeldung {
			malo_id: test_malo(0),
			lieferant_alt: lfa,
			lieferende,
		}),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn anmeldung_has_correct_roles() {
		let msg = anmeldung(NaiveDate::from_ymd_opt(2025, 7, 1).unwrap());
		assert_eq!(msg.absender_rolle, MarktRolle::LieferantNeu);
		assert_eq!(msg.empfaenger_rolle, MarktRolle::Netzbetreiber);
		assert!(matches!(msg.payload, NachrichtenPayload::UtilmdAnmeldung(_)));
	}

	#[test]
	fn bestaetigung_has_correct_roles() {
		let msg = bestaetigung(
			test_mp_id(1),
			NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
		);
		assert_eq!(msg.absender_rolle, MarktRolle::Netzbetreiber);
		assert_eq!(msg.empfaenger_rolle, MarktRolle::LieferantNeu);
		assert!(matches!(msg.payload, NachrichtenPayload::UtilmdBestaetigung(_)));
	}

	#[test]
	fn abmeldung_has_correct_roles() {
		let msg = abmeldung(
			test_mp_id(3),
			NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
		);
		assert_eq!(msg.absender_rolle, MarktRolle::Netzbetreiber);
		assert_eq!(msg.empfaenger_rolle, MarktRolle::LieferantAlt);
		assert!(matches!(msg.payload, NachrichtenPayload::UtilmdAbmeldung(_)));
	}
}
