use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::querschnitt::UtiltsZaehlzeitdefinition;
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

// ---------------------------------------------------------------------------
// 12. UTILTS Zaehlzeitdefinition — NB -> LF
// ---------------------------------------------------------------------------

pub fn zaehlzeitdefinition_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(0); // LF

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+UTILTS:D:01B:UN:1.1e'\
		 BGM+Z08+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 RFF+Z13:HT-NT-2026'\
		 IMD+F++:::Hochtarif/Niedertarif Umschaltzeiten'\
		 CCI+Z30::ZRTyp-HT-NT'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
	)
}

pub fn zaehlzeitdefinition_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(0);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::UtiltsZaehlzeitdefinition(UtiltsZaehlzeitdefinition {
			formel_id: "HT-NT-2026".to_string(),
			bezeichnung: "Hochtarif/Niedertarif Umschaltzeiten".to_string(),
			zeitreihen_typ: "ZRTyp-HT-NT".to_string(),
		}),
	}
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use mako_codec::edifact::dispatch::{parse_nachricht, serialize_nachricht};

	use super::*;

	#[test]
	fn parse_utilts_zaehlzeitdefinition() {
		let edi = zaehlzeitdefinition_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, zaehlzeitdefinition_erwartet());
	}

	#[test]
	fn roundtrip_utilts_zaehlzeitdefinition() {
		let parsed = parse_nachricht(&zaehlzeitdefinition_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed).expect("serialize");
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
