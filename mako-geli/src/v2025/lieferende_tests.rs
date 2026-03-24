use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, MsconsSchlussturnusmesswert, UtilmdLieferendeAbmeldung,
	UtilmdLieferendeBestaetigung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::NachrichtenPayload;

use super::lieferende::{LieferendeEvent, LieferendeState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn lf_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}
fn lieferende() -> NaiveDate {
	NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
}

fn abmeldung() -> UtilmdLieferendeAbmeldung {
	UtilmdLieferendeAbmeldung {
		malo_id: malo(),
		lieferant: lf_id(),
		lieferende: lieferende(),
	}
}

// --- Happy path ---

#[test]
fn idle_plus_abmeldung_transitions_to_gesendet() {
	let out = reduce(LieferendeState::Idle, LieferendeEvent::AbmeldungEingegangen(abmeldung()))
		.expect("should succeed");
	assert!(matches!(out.state, LieferendeState::AbmeldungGesendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn gesendet_plus_bestaetigt_sends_message() {
	let state = LieferendeState::AbmeldungGesendet {
		malo: malo(),
		lf: lf_id(),
		nb: nb_id(),
		lieferende: lieferende(),
	};
	let out = reduce(
		state,
		LieferendeEvent::AbmeldungBestaetigt(UtilmdLieferendeBestaetigung {
			malo_id: malo(),
			lieferende: lieferende(),
		}),
	)
	.expect("should succeed");
	assert!(matches!(out.state, LieferendeState::Bestaetigt { .. }));
	assert_eq!(out.nachrichten.len(), 1);
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::UtilmdLieferendeBestaetigung(_)
	));
}

#[test]
fn full_happy_path() {
	let out = reduce(LieferendeState::Idle, LieferendeEvent::AbmeldungEingegangen(abmeldung()))
		.expect("step 1");

	let out = reduce(
		out.state,
		LieferendeEvent::AbmeldungBestaetigt(UtilmdLieferendeBestaetigung {
			malo_id: malo(),
			lieferende: lieferende(),
		}),
	)
	.expect("step 2");
	assert!(matches!(out.state, LieferendeState::Bestaetigt { .. }));

	let out = reduce(
		out.state,
		LieferendeEvent::SchlussturnusmesswertEmpfangen(MsconsSchlussturnusmesswert {
			malo_id: malo(),
			zaehlerstand: 12345.678,
			stichtag: lieferende(),
			einheit: "m3".to_string(),
		}),
	)
	.expect("step 3");
	assert!(matches!(out.state, LieferendeState::Abgeschlossen { .. }));
}

// --- Rejection ---

#[test]
fn rejection_from_gesendet() {
	let state = LieferendeState::AbmeldungGesendet {
		malo: malo(),
		lf: lf_id(),
		nb: nb_id(),
		lieferende: lieferende(),
	};
	let out = reduce(state, LieferendeEvent::Abgelehnt { grund: AblehnungsGrund::KeinVertrag })
		.expect("should succeed");
	assert!(matches!(out.state, LieferendeState::Abgelehnt { .. }));
}

// --- Timeout ---

#[test]
fn timeout_from_gesendet() {
	let state = LieferendeState::AbmeldungGesendet {
		malo: malo(),
		lf: lf_id(),
		nb: nb_id(),
		lieferende: lieferende(),
	};
	let out = reduce(state, LieferendeEvent::FristUeberschritten).expect("should succeed");
	assert_eq!(
		out.state,
		LieferendeState::Abgelehnt { malo: malo(), grund: AblehnungsGrund::Fristverletzung }
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(
		LieferendeState::Idle,
		LieferendeEvent::AbmeldungBestaetigt(UtilmdLieferendeBestaetigung {
			malo_id: malo(),
			lieferende: lieferende(),
		}),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgeschlossen_is_terminal() {
	let state = LieferendeState::Abgeschlossen { malo: malo(), zaehlerstand: 100.0 };
	let result = reduce(state, LieferendeEvent::FristUeberschritten);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
