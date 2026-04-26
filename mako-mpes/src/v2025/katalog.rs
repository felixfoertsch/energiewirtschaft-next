//! MPES-Prozesskatalog für die Test-UI.
//!
//! Die aktuelle Implementierung hat einen Reducer für Erzeugungsanlagen. Der
//! Katalog bildet deshalb einen Sammel-Prozess ab und trennt keine
//! Direktvermarktungsvariante ab, solange der Reducer keinen eigenen Ablauf
//! dafür enthält.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

pub fn katalog() -> Vec<ProzessDef> {
	vec![ProzessDef::new(
		"mpes_erzeugungsanlagen_anlage",
		"Stammdaten Erzeugungsanlagen",
		ProzessKategorie::Mpes,
		vec![
			SchrittDef::new(
				"Anmeldung empfangen",
				MarktRolle::BetreiberErzeugungsanlage,
				MarktRolle::Netzbetreiber,
				"UtilmdAnmeldungErzeugung",
				NachrichtenTyp::Utilmd,
			),
			SchrittDef::new(
				"Bestätigung senden",
				MarktRolle::Netzbetreiber,
				MarktRolle::BetreiberErzeugungsanlage,
				"UtilmdAnmeldungErzeugung",
				NachrichtenTyp::Utilmd,
			),
			SchrittDef::new(
				"Zuordnung informieren",
				MarktRolle::Netzbetreiber,
				MarktRolle::BetreiberErzeugungsanlage,
				"",
				NachrichtenTyp::Intern,
			),
			SchrittDef::new(
				"Einspeise-Messwerte senden",
				MarktRolle::Messstellenbetreiber,
				MarktRolle::Netzbetreiber,
				"MsconsEinspeiseMesswerte",
				NachrichtenTyp::Mscons,
			),
		],
	)]
}

#[cfg(test)]
mod tests {
	use chrono::NaiveDateTime;
	use mako_types::gpke_nachrichten::{
		Messwert, MesswertStatus, MsconsEinspeiseMesswerte, UtilmdAnmeldungErzeugung,
	};
	use mako_types::ids::{MaLoId, MarktpartnerId};
	use mako_types::nachricht::NachrichtenPayload;

	use super::*;
	use crate::v2025::erzeugungsanlagen::{
		ErzeugungsanlagenEvent, ErzeugungsanlagenState, reduce,
	};

	fn malo() -> MaLoId {
		MaLoId::new("51238696788").unwrap()
	}

	fn betreiber_id() -> MarktpartnerId {
		MarktpartnerId::new("9900000000003").unwrap()
	}

	fn nb_id() -> MarktpartnerId {
		MarktpartnerId::new("9900000000010").unwrap()
	}

	fn anmeldung() -> UtilmdAnmeldungErzeugung {
		UtilmdAnmeldungErzeugung {
			malo_id: malo(),
			anlagenbetreiber: betreiber_id(),
			eeg_anlage: true,
			installierte_leistung_kw: 10.0,
		}
	}

	fn messwerte() -> MsconsEinspeiseMesswerte {
		MsconsEinspeiseMesswerte {
			malo_id: malo(),
			werte: vec![Messwert {
				zeitpunkt: NaiveDateTime::parse_from_str(
					"2025-07-01 00:00:00",
					"%Y-%m-%d %H:%M:%S",
				)
				.unwrap(),
				wert: 42.0,
				einheit: "kWh".to_string(),
				status: MesswertStatus::Gemessen,
			}],
		}
	}

	#[test]
	fn katalog_enthaelt_sammelprozess_fuer_erzeugungsanlagen() {
		let katalog = katalog();
		assert_eq!(katalog.len(), 1);
		assert_eq!(katalog[0].key, "mpes_erzeugungsanlagen_anlage");
		assert_eq!(katalog[0].schritte.len(), 4);
	}

	#[test]
	fn erzeugungsanlagen_katalog_passt_zum_reducer_happy_path() {
		let p = katalog().remove(0);

		let out = reduce(
			ErzeugungsanlagenState::Idle,
			ErzeugungsanlagenEvent::AnmeldungEmpfangen(anmeldung()),
		)
		.expect("step 1");
		assert!(out.nachrichten.is_empty());
		assert_eq!(p.schritte[0].typ, "UtilmdAnmeldungErzeugung");
		assert_eq!(p.schritte[0].nachrichten_typ, NachrichtenTyp::Utilmd);

		let out = reduce(out.state, ErzeugungsanlagenEvent::Bestaetigt).expect("step 2");
		let msg = out.nachrichten.first().expect("wire message");
		assert_eq!(p.schritte[1].absender, msg.absender_rolle.slug());
		assert_eq!(p.schritte[1].empfaenger, msg.empfaenger_rolle.slug());
		assert_eq!(p.schritte[1].typ, "UtilmdAnmeldungErzeugung");
		assert!(matches!(
			msg.payload,
			NachrichtenPayload::UtilmdAnmeldungErzeugung(_)
		));

		let out = reduce(out.state, ErzeugungsanlagenEvent::ZuordnungInformiert).expect("step 3");
		assert!(out.nachrichten.is_empty());

		let out = reduce(
			out.state,
			ErzeugungsanlagenEvent::EinspeiseMesswerteEmpfangen(messwerte()),
		)
		.expect("step 4");
		let msg = out.nachrichten.first().expect("wire message");
		assert_eq!(p.schritte[3].absender, msg.absender_rolle.slug());
		assert_eq!(p.schritte[3].empfaenger, msg.empfaenger_rolle.slug());
		assert_eq!(p.schritte[3].typ, "MsconsEinspeiseMesswerte");
		assert!(matches!(
			msg.payload,
			NachrichtenPayload::MsconsEinspeiseMesswerte(_)
		));
	}

	#[test]
	fn anmeldung_und_bestaetigung_verwenden_erzeugungsanlagen_rollen() {
		let p = katalog().remove(0);
		assert_eq!(
			p.schritte[0].absender,
			MarktRolle::BetreiberErzeugungsanlage.slug()
		);
		assert_eq!(p.schritte[0].empfaenger, MarktRolle::Netzbetreiber.slug());
		assert_eq!(p.schritte[1].absender, MarktRolle::Netzbetreiber.slug());
		assert_eq!(
			p.schritte[1].empfaenger,
			MarktRolle::BetreiberErzeugungsanlage.slug()
		);
		assert_eq!(nb_id(), MarktpartnerId::new("9900000000010").unwrap());
	}
}
