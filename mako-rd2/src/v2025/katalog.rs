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
				SchrittDef::with_erklaerung(
					"Aktivierung senden",
					MarktRolle::AnfordernderNetzbetreiber,
					MarktRolle::Anschlussnetzbetreiber,
					"RdAktivierung",
					NachrichtenTyp::RdXml,
					"Der anfordernde Netzbetreiber meldet dem Anschlussnetzbetreiber einen konkreten Redispatch-Bedarf. Der Anschlussnetzbetreiber braucht diese Aktivierung, um die betroffenen technischen Ressourcen in seinem Netzgebiet operativ anzusteuern.",
				),
				SchrittDef::with_erklaerung(
					"Weiterleitung",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::Einsatzverantwortlicher,
					"",
					NachrichtenTyp::Intern,
					"Der Anschlussnetzbetreiber leitet die Aktivierung an den Einsatzverantwortlichen weiter und übersetzt den Netzbedarf in eine anlagenbezogene Anforderung. Der Einsatzverantwortliche koordiniert daraus den konkreten Einsatz der Ressource.",
				),
				SchrittDef::with_erklaerung(
					"Quittierung",
					MarktRolle::Einsatzverantwortlicher,
					MarktRolle::BetreiberTechnischeRessource,
					"",
					NachrichtenTyp::Intern,
					"Der Einsatzverantwortliche informiert den Betreiber der technischen Ressource über die angenommene Aktivierung. Damit weiß der Betreiber, welche operative Maßnahme für seine Anlage ausgelöst wurde.",
				),
			],
		),
		ProzessDef::new(
			"rd2_eiv_dp_stammdaten",
			"Stammdaten EIV→DP→ANB",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::with_erklaerung(
					"Stammdaten an DP senden",
					MarktRolle::Einsatzverantwortlicher,
					MarktRolle::DataProvider,
					"RdStammdaten",
					NachrichtenTyp::RdXml,
					"Der Einsatzverantwortliche bündelt die von seinen BTRs gemeldeten Anlagenstammdaten und schickt sie dem Data Provider. Der DP ist im RD-2.0-Datenraum die zentrale Sammelstelle: er konsolidiert die Stammdaten aller EIVs und stellt sie den berechtigten Netzbetreibern in einer abgestimmten Sicht bereit.",
				),
				SchrittDef::with_erklaerung(
					"DP an ANB weiterleiten",
					MarktRolle::DataProvider,
					MarktRolle::Anschlussnetzbetreiber,
					"RdStammdaten",
					NachrichtenTyp::RdXml,
					"Der DP leitet die konsolidierten Anlagenstammdaten an den Anschlussnetzbetreiber weiter. Erst damit kennt der ANB die im Netzgebiet verfügbaren Ressourcen vollständig und kann sie in Engpassbearbeitung und Abrufkaskade einbeziehen.",
				),
			],
		),
		ProzessDef::new(
			"rd2_btr_eiv_stammdaten",
			"Stammdaten BTR→EIV",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::with_erklaerung(
					"Stammdaten senden",
					MarktRolle::BetreiberTechnischeRessource,
					MarktRolle::Einsatzverantwortlicher,
					"RdStammdaten",
					NachrichtenTyp::RdXml,
					"Der BTR hat eine kleine regelbare Anlage und meldet seinem EIV die installierte Leistung, den Anlagentyp und den Standort. Der EIV ist die zentrale Verbindung der Anlage in die Marktkommunikation und braucht diese Stammdaten, um den operativen Einsatz zu organisieren.",
				),
				SchrittDef::with_erklaerung(
					"Bestätigen",
					MarktRolle::Einsatzverantwortlicher,
					MarktRolle::BetreiberTechnischeRessource,
					"",
					NachrichtenTyp::Intern,
					"Der EIV bestätigt dem BTR den Empfang der Stammdaten und übernimmt damit die Verantwortung, die Anlage in die Marktkommunikation einzubinden. Erst nach dieser Bestätigung gilt die Anlage als angebunden.",
				),
			],
		),
		ProzessDef::new(
			"rd2_engpass",
			"Engpass-Meldung",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::with_erklaerung(
					"Engpass melden",
					MarktRolle::Netzbetreiber,
					MarktRolle::Anschlussnetzbetreiber,
					"RdEngpass",
					NachrichtenTyp::RdXml,
					"Der Netzbetreiber meldet einen Engpass an den Anschlussnetzbetreiber, dessen Netzgebiet oder Anlagen betroffen sind. Der Anschlussnetzbetreiber braucht diese Information, um die relevanten Ressourcen, Stammdaten und Einsatzverantwortlichen zuzuordnen.",
				),
				SchrittDef::with_erklaerung(
					"Bestätigen",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::Netzbetreiber,
					"",
					NachrichtenTyp::Intern,
					"Der Anschlussnetzbetreiber bestätigt, dass er den Engpass fachlich verarbeitet und in seine Redispatch-Koordination übernommen hat. Der meldende Netzbetreiber erhält damit Rückmeldung, dass der Engpass nicht unbearbeitet bleibt.",
				),
			],
		),
		ProzessDef::new(
			"rd2_fahrplan",
			"Fahrplan-Meldung",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::with_erklaerung(
					"Fahrplan senden",
					MarktRolle::Einsatzverantwortlicher,
					MarktRolle::DataProvider,
					"RdFahrplan",
					NachrichtenTyp::RdXml,
					"Der Einsatzverantwortliche übermittelt den geplanten Einsatz seiner steuerbaren Ressource an den Data Provider. Der DP ist im RD-2.0-Datenraum der zentrale Empfänger für Planungsdaten und konsolidiert sie für alle berechtigten Netzbetreiber.",
				),
				SchrittDef::with_erklaerung(
					"Weiterleitung",
					MarktRolle::DataProvider,
					MarktRolle::Anschlussnetzbetreiber,
					"",
					NachrichtenTyp::Intern,
					"Der DP tritt als Mittelsmann zwischen EIV und ANB auf und konsolidiert die Fahrplandaten aus dem RD-2.0-Datenraum. Der Anschlussnetzbetreiber erhält dadurch eine abgestimmte Sicht auf die für sein Netz relevanten Fahrpläne.",
				),
				SchrittDef::with_erklaerung(
					"Bestätigen",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::Einsatzverantwortlicher,
					"",
					NachrichtenTyp::Intern,
					"Der Anschlussnetzbetreiber bestätigt dem Einsatzverantwortlichen, dass die Fahrplandaten für die Netzführung verwertbar vorliegen. Damit kann der Einsatzverantwortliche den gemeldeten Plan als Grundlage weiterer Redispatch-Kommunikation verwenden.",
				),
			],
		),
		ProzessDef::new(
			"rd2_nichtverfuegbarkeit",
			"Nichtverfügbarkeitsmeldung",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::with_erklaerung(
					"Nichtverfügbarkeit melden",
					MarktRolle::Einsatzverantwortlicher,
					MarktRolle::DataProvider,
					"RdNichtverfuegbarkeit",
					NachrichtenTyp::RdXml,
					"Der Einsatzverantwortliche meldet dem DP, dass eine Ressource zeitweise nicht oder nur eingeschränkt verfügbar ist. Der DP konsolidiert als Mittelsmann zwischen EIV und ANB diese Verfügbarkeitsinformation mit den übrigen Datenflüssen im RD-2.0-Datenraum.",
				),
				SchrittDef::with_erklaerung(
					"Weiterleitung",
					MarktRolle::DataProvider,
					MarktRolle::Anschlussnetzbetreiber,
					"",
					NachrichtenTyp::Intern,
					"Der DP leitet die konsolidierte Nichtverfügbarkeit an den Anschlussnetzbetreiber weiter. Der Anschlussnetzbetreiber braucht diese Information, damit er keine nicht verfügbare Ressource für Engpassmaßnahmen einplant.",
				),
			],
		),
		ProzessDef::new(
			"rd2_stammdaten",
			"Stammdaten-Austausch RD 2.0",
			ProzessKategorie::Rd2,
			vec![
				SchrittDef::with_erklaerung(
					"Stammdaten senden",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::DataProvider,
					"RdStammdaten",
					NachrichtenTyp::RdXml,
					"Der Anschlussnetzbetreiber leitet die Anlagenstammdaten an den Data Provider weiter. Der DP ist der zentrale Mittelsmann im RD-2.0-Datenraum: er konsolidiert die Stammdaten aller Anlagen seines Bilanzierungsgebiets und stellt sie für Abrufe und Engpassmeldungen bereit.",
				),
				SchrittDef::with_erklaerung(
					"Weiterleitung",
					MarktRolle::DataProvider,
					MarktRolle::Anschlussnetzbetreiber,
					"",
					NachrichtenTyp::Intern,
					"Der DP gleicht die eingegangenen Stammdaten mit den bereits konsolidierten Datenflüssen zwischen EIV und ANB ab und stellt dem Anschlussnetzbetreiber die abgestimmte Sicht bereit. So arbeitet der Anschlussnetzbetreiber mit denselben Stammdaten wie der übrige RD-2.0-Datenraum.",
				),
				SchrittDef::with_erklaerung(
					"Bestätigen",
					MarktRolle::Anschlussnetzbetreiber,
					MarktRolle::DataProvider,
					"",
					NachrichtenTyp::Intern,
					"Der Anschlussnetzbetreiber bestätigt dem DP, dass die konsolidierten Stammdaten fachlich akzeptiert sind. Der DP kann die Daten danach als verbindliche Grundlage für weitere Abrufe, Fahrpläne und Engpassmeldungen verwenden.",
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
	use crate::v2025::btr_eiv_stammdaten::{
		BtrEivStammdatenEvent, BtrEivStammdatenState, reduce as reduce_btr_eiv_stammdaten,
	};
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
				"rd2_eiv_dp_stammdaten",
				"rd2_btr_eiv_stammdaten",
				"rd2_engpass",
				"rd2_fahrplan",
				"rd2_nichtverfuegbarkeit",
				"rd2_stammdaten",
			]
		);
	}

	#[test]
	fn btr_eiv_stammdaten_katalog_passt_zum_reducer_happy_path() {
		let p = prozess("rd2_btr_eiv_stammdaten");
		assert_eq!(p.schritte.len(), 2);

		let out = reduce_btr_eiv_stammdaten(
			BtrEivStammdatenState::Idle,
			BtrEivStammdatenEvent::StammdatenGesendet(RdStammdaten {
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

		let out =
			reduce_btr_eiv_stammdaten(out.state, BtrEivStammdatenEvent::Bestaetigt).expect("step 2");
		assert!(out.nachrichten.is_empty());
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
