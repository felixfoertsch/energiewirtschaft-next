use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::querschnitt::PartinMarktpartner;
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

// ---------------------------------------------------------------------------
// 11. PARTIN Marktpartner-Stammdaten — NB -> LF
// ---------------------------------------------------------------------------

pub fn marktpartner_edi() -> String {
	let sender = test_mp_id(1); // NB
	let empfaenger = test_mp_id(0); // LF
	let mp = test_mp_id(2); // the partner being described

	format!(
		"UNB+UNOC:3+{sender}:500+{empfaenger}:500+260325:1200+00001'\
		 UNH+1+PARTIN:D:01B:UN:1.0e'\
		 BGM+Z34+DOK00001'\
		 DTM+137:20260325120000?+01:303'\
		 NAD+MS+{sender}::293'\
		 NAD+MR+{empfaenger}::293'\
		 NAD+DP+{mp}::293'\
		 CTA+IC+:Stadtwerke Musterstadt'\
		 RFF+ACD:Netzbetreiber'\
		 UNT+9+1'\
		 UNZ+1+00001'",
		sender = sender.as_str(),
		empfaenger = empfaenger.as_str(),
		mp = mp.as_str(),
	)
}

pub fn marktpartner_erwartet() -> Nachricht {
	let nb = test_mp_id(1);
	let lf = test_mp_id(0);

	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		pruef_id: None,
		payload: NachrichtenPayload::PartinMarktpartner(PartinMarktpartner {
			mp_id: test_mp_id(2),
			name: "Stadtwerke Musterstadt".to_string(),
			rolle: "Netzbetreiber".to_string(),
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
	fn parse_partin_marktpartner() {
		let edi = marktpartner_edi();
		let parsed = parse_nachricht(&edi).expect("parsing must succeed");
		assert_eq!(parsed, marktpartner_erwartet());
	}

	#[test]
	fn roundtrip_partin_marktpartner() {
		let parsed = parse_nachricht(&marktpartner_edi()).unwrap();
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap();
		assert_eq!(reparsed, parsed);
	}
}
