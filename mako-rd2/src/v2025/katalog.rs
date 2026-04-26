//! Redispatch-2.0-Prozesskatalog für die Test-UI.
//!
//! Quelle der Wahrheit sind die Reducer in diesem Modul. Wire-Schritte
//! referenzieren nur Payload-Varianten, die der jeweilige Reducer tatsächlich
//! verarbeitet oder emittiert; reine Zustandsübergänge bleiben intern.

use mako_types::katalog::{NachrichtenTyp, ProzessDef, ProzessKategorie, SchrittDef};
use mako_types::rolle::MarktRolle;

pub fn katalog() -> Vec<ProzessDef> {
	vec![
		ProzessDef::new(
			"rd2_abruf",
			"Redispatch-Abruf",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::new(
					"Aktivierung senden",
					MarktRolle::AnfordernderNetzbetreiber,
					MarktRolle::Anschlussnetzbetreiber,
					"RdAktivierung",
					NachrichtenTyp::RdXml,
				),
				SchrittDef::new(
					"Weiterleitung",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::Einsatzverantwortlicher,
					"",
					NachrichtenTyp::Intern,
				),
				SchrittDef::new(
					"Quittierung",
					MarktRolle::Einsatzverantwortlicher,
					MarktRolle::BetreiberTechnischeRessource,
					"",
					NachrichtenTyp::Intern,
				),
			],
		),
		ProzessDef::new(
			"rd2_engpass",
			"Engpass-Meldung",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::new(
					"Engpass melden",
					MarktRolle::Netzbetreiber,
					MarktRolle::Anschlussnetzbetreiber,
					"RdEngpass",
					NachrichtenTyp::RdXml,
				),
				SchrittDef::new(
					"Bestätigen",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::Netzbetreiber,
					"",
					NachrichtenTyp::Intern,
				),
			],
		),
		ProzessDef::new(
			"rd2_fahrplan",
			"Fahrplan-Meldung",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::new(
					"Fahrplan senden",
					MarktRolle::Einsatzverantwortlicher,
					MarktRolle::Uebertragungsnetzbetreiber,
					"RdFahrplan",
					NachrichtenTyp::RdXml,
				),
				SchrittDef::new(
					"Weiterleitung",
					MarktRolle::DataProvider,
					MarktRolle::Anschlussnetzbetreiber,
					"",
					NachrichtenTyp::Intern,
				),
				SchrittDef::new(
					"Bestätigen",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::Einsatzverantwortlicher,
					"",
					NachrichtenTyp::Intern,
				),
			],
		),
		ProzessDef::new(
			"rd2_nichtverfuegbarkeit",
			"Nichtverfügbarkeitsmeldung",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::new(
					"Nichtverfügbarkeit melden",
					MarktRolle::Einsatzverantwortlicher,
					MarktRolle::DataProvider,
					"RdNichtverfuegbarkeit",
					NachrichtenTyp::RdXml,
				),
				SchrittDef::new(
					"Weiterleitung",
					MarktRolle::DataProvider,
					MarktRolle::Anschlussnetzbetreiber,
					"",
					NachrichtenTyp::Intern,
				),
			],
		),
		ProzessDef::new(
			"rd2_stammdaten",
			"Stammdaten-Austausch RD 2.0",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::new(
					"Stammdaten senden",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::DataProvider,
					"RdStammdaten",
					NachrichtenTyp::RdXml,
				),
				SchrittDef::new(
					"Weiterleitung",
					MarktRolle::DataProvider,
					MarktRolle::Anschlussnetzbetreiber,
					"",
					NachrichtenTyp::Intern,
				),
				SchrittDef::new(
					"Bestätigen",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::DataProvider,
					"",
					NachrichtenTyp::Intern,
				),
			],
		),
	]
}

#[cfg(test)]
mod tests {
	use chrono::NaiveDateTime;
	use mako_types::gpke_nachrichten::{
		RdAktivierung, RdEngpass, RdFahrplan, RdNichtverfuegbarkeit, RdStammdaten, RessourceTyp,
	};
	use mako_types::ids::MaLoId;
	use mako_types::nachricht::NachrichtenPayload;

	use super::*;
	use crate::v2025::abruf::{AbrufEvent, AbrufState, reduce as reduce_abruf};
	use crate::v2025::engpass::{EngpassEvent, EngpassState, reduce as reduce_engpass};
	use crate::v2025::fahrplan::{FahrplanEvent, FahrplanState, reduce as reduce_fahrplan};
	use crate::v2025::nichtverfuegbarkeit::{
		NichtverfuegbarkeitEvent, NichtverfuegbarkeitState, reduce as reduce_nichtverfuegbarkeit,
	};
	use crate::v2025::stammdaten::{
		StammdatenEvent, StammdatenState, reduce as reduce_stammdaten,
	};

	fn dt(s: &str) -> NaiveDateTime {
		NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
	}

	fn prozess(key: &str) -> ProzessDef {
		katalog()
			.into_iter()
			.find(|p| p.key == key)
			.expect("process exists")
	}

	#[test]
	fn katalog_deckt_alle_rd2_module_ab() {
		let keys: Vec<_> = katalog().into_iter().map(|p| p.key).collect();
		assert_eq!(
			keys,
			vec![
				"rd2_abruf",
				"rd2_engpass",
				"rd2_fahrplan",
				"rd2_nichtverfuegbarkeit",
				"rd2_stammdaten",
			]
		);
	}

	#[test]
	fn abruf_katalog_passt_zum_reducer_happy_path() {
		let p = prozess("rd2_abruf");
		assert_eq!(p.schritte.len(), 3);

		let out = reduce_abruf(
			AbrufState::Idle,
			AbrufEvent::AbrufGesendet(RdAktivierung {
				ressource_id: "TR-001".to_string(),
				sollwert_kw: 25.0,
				start: dt("2025-07-01 08:00:00"),
				ende: dt("2025-07-01 12:00:00"),
			}),
		)
		.expect("step 1");
		let msg = out.nachrichten.first().expect("wire message");
		assert_eq!(p.schritte[0].absender, msg.absender_rolle.slug());
		assert_eq!(p.schritte[0].empfaenger, msg.empfaenger_rolle.slug());
		assert_eq!(p.schritte[0].typ, "RdAktivierung");
		assert!(matches!(msg.payload, NachrichtenPayload::RdAktivierung(_)));

		let out = reduce_abruf(out.state, AbrufEvent::Weitergeleitet).expect("step 2");
		assert!(out.nachrichten.is_empty());
		let out = reduce_abruf(out.state, AbrufEvent::Quittiert).expect("step 3");
		assert!(out.nachrichten.is_empty());
	}

	#[test]
	fn engpass_katalog_passt_zum_reducer_happy_path() {
		let p = prozess("rd2_engpass");
		assert_eq!(p.schritte.len(), 2);

		let out = reduce_engpass(
			EngpassState::Idle,
			EngpassEvent::EngpassGemeldet(RdEngpass {
				netzgebiet: "Netz-Nord".to_string(),
				engpass_start: dt("2025-07-01 08:00:00"),
				engpass_ende: dt("2025-07-01 16:00:00"),
				betroffene_leistung_kw: 100.0,
			}),
		)
		.expect("step 1");
		let msg = out.nachrichten.first().expect("wire message");
		assert_eq!(p.schritte[0].absender, msg.absender_rolle.slug());
		assert_eq!(p.schritte[0].empfaenger, msg.empfaenger_rolle.slug());
		assert_eq!(p.schritte[0].typ, "RdEngpass");
		assert!(matches!(msg.payload, NachrichtenPayload::RdEngpass(_)));

		let out = reduce_engpass(out.state, EngpassEvent::Bestaetigt).expect("step 2");
		assert!(out.nachrichten.is_empty());
	}

	#[test]
	fn fahrplan_katalog_passt_zum_reducer_happy_path() {
		let p = prozess("rd2_fahrplan");
		assert_eq!(p.schritte.len(), 3);

		let out = reduce_fahrplan(
			FahrplanState::Idle,
			FahrplanEvent::FahrplanGesendet(RdFahrplan {
				ressource_id: "TR-001".to_string(),
				zeitreihe: vec![],
			}),
		)
		.expect("step 1");
		let msg = out.nachrichten.first().expect("wire message");
		assert_eq!(p.schritte[0].absender, msg.absender_rolle.slug());
		assert_eq!(p.schritte[0].empfaenger, msg.empfaenger_rolle.slug());
		assert_eq!(p.schritte[0].typ, "RdFahrplan");
		assert!(matches!(msg.payload, NachrichtenPayload::RdFahrplan(_)));

		let out = reduce_fahrplan(out.state, FahrplanEvent::Weitergeleitet).expect("step 2");
		assert!(out.nachrichten.is_empty());
		let out = reduce_fahrplan(out.state, FahrplanEvent::Bestaetigt).expect("step 3");
		assert!(out.nachrichten.is_empty());
	}

	#[test]
	fn nichtverfuegbarkeit_katalog_passt_zum_reducer_happy_path() {
		let p = prozess("rd2_nichtverfuegbarkeit");
		assert_eq!(p.schritte.len(), 2);

		let out = reduce_nichtverfuegbarkeit(
			NichtverfuegbarkeitState::Idle,
			NichtverfuegbarkeitEvent::Gemeldet(RdNichtverfuegbarkeit {
				ressource_id: "TR-001".to_string(),
				von: dt("2025-07-01 00:00:00"),
				bis: dt("2025-07-02 00:00:00"),
				grund: "Wartung".to_string(),
			}),
		)
		.expect("step 1");
		let msg = out.nachrichten.first().expect("wire message");
		assert_eq!(p.schritte[0].absender, msg.absender_rolle.slug());
		assert_eq!(p.schritte[0].empfaenger, msg.empfaenger_rolle.slug());
		assert_eq!(p.schritte[0].typ, "RdNichtverfuegbarkeit");
		assert!(matches!(
			msg.payload,
			NachrichtenPayload::RdNichtverfuegbarkeit(_)
		));

		let out = reduce_nichtverfuegbarkeit(out.state, NichtverfuegbarkeitEvent::Weitergeleitet)
			.expect("step 2");
		assert!(out.nachrichten.is_empty());
	}

	#[test]
	fn stammdaten_katalog_passt_zum_reducer_happy_path() {
		let p = prozess("rd2_stammdaten");
		assert_eq!(p.schritte.len(), 3);

		let out = reduce_stammdaten(
			StammdatenState::Idle,
			StammdatenEvent::StammdatenGesendet(RdStammdaten {
				ressource_id: "TR-001".to_string(),
				ressource_typ: RessourceTyp::TechnischeRessource,
				standort_malo: MaLoId::new("51238696788").unwrap(),
				installierte_leistung_kw: 50.0,
			}),
		)
		.expect("step 1");
		let msg = out.nachrichten.first().expect("wire message");
		assert_eq!(p.schritte[0].absender, msg.absender_rolle.slug());
		assert_eq!(p.schritte[0].empfaenger, msg.empfaenger_rolle.slug());
		assert_eq!(p.schritte[0].typ, "RdStammdaten");
		assert!(matches!(msg.payload, NachrichtenPayload::RdStammdaten(_)));

		let out = reduce_stammdaten(out.state, StammdatenEvent::Weitergeleitet).expect("step 2");
		assert!(out.nachrichten.is_empty());
		let out = reduce_stammdaten(out.state, StammdatenEvent::Bestaetigt).expect("step 3");
		assert!(out.nachrichten.is_empty());
	}
}
