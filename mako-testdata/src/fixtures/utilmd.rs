use chrono::NaiveDate;

use mako_types::gpke_nachrichten::UtilmdAnmeldung;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
use mako_types::rolle::MarktRolle;

use crate::ids::{test_malo, test_mp_id};

/// Build a complete EDIFACT UTILMD Anmeldung string using test IDs.
/// Sender = test_mp_id(0) (LieferantNeu), Empfaenger = test_mp_id(1) (Netzbetreiber).
pub fn anmeldung_lfw_edi() -> String {
	let sender = test_mp_id(0);
	let empfaenger = test_mp_id(1);
	let malo = test_malo(0);

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILMD:D:11A:UN:S2.1'\
		 BGM+E01+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 IDE+24+{malo}'\
		 DTM+92:20260701:102'\
		 RFF+Z13:44001'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

/// The expected parsed Nachricht for the EDIFACT string from `anmeldung_lfw_edi()`.
pub fn anmeldung_lfw_erwartet() -> Nachricht {
	let sender = test_mp_id(0);
	let empfaenger = test_mp_id(1);

	Nachricht {
		absender: sender.clone(),
		absender_rolle: MarktRolle::LieferantNeu,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: Some(PruefIdentifikator::AnmeldungNn),
		payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: sender,
			lieferbeginn: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}),
	}
}

#[cfg(test)]
mod tests {
	use mako_codec::edifact::dispatch::parse_nachricht;

	use super::*;

	#[test]
	fn parse_anmeldung_lfw() {
		let edi = anmeldung_lfw_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		let erwartet = anmeldung_lfw_erwartet();
		assert_eq!(parsed, erwartet);
	}
}
