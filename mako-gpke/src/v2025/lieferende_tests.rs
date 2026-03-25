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
	NaiveDate::from_ymd_opt(2025, 9, 30).unwrap()
}

fn abmeldung() -> UtilmdLieferendeAbmeldung {
	UtilmdLieferendeAbmeldung {
		malo_id: malo(),
		lieferant: lf_id(),
		lieferende: lieferende(),
	}
}

fn bestaetigung() -> UtilmdLieferendeBestaetigung {
	UtilmdLieferendeBestaetigung {
		malo_id: malo(),
		lieferende: lieferende(),
	}
}

fn schlussturnusmesswert() -> MsconsSchlussturnusmesswert {
	MsconsSchlussturnusmesswert {
		malo_id: malo(),
		zaehlerstand: 12345.67,
		stichtag: lieferende(),
		einheit: "kWh".to_string(),
	}
}

// --- Happy path ---

#[test]
fn idle_plus_abmeldung_transitions_to_abmeldung_gesendet() {
	let out = reduce(LieferendeState::Idle, LieferendeEvent::AbmeldungEingegangen(abmeldung()))
		.expect("should succeed");
	assert_eq!(
		out.state,
		LieferendeState::AbmeldungGesendet {
			malo: malo(),
			lf: lf_id(),
			nb: nb_id(),
			lieferende: lieferende(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn abmeldung_gesendet_plus_bestaetigt_transitions_to_bestaetigt() {
	let state = LieferendeState::AbmeldungGesendet {
		malo: malo(),
		lf: lf_id(),
		nb: nb_id(),
		lieferende: lieferende(),
	};
	let out = reduce(state, LieferendeEvent::AbmeldungBestaetigt(bestaetigung()))
		.expect("should succeed");
	assert_eq!(
		out.state,
		LieferendeState::Bestaetigt {
			malo: malo(),
			lieferende: lieferende(),
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::UtilmdLieferendeBestaetigung(_)
	));
}

#[test]
fn bestaetigt_plus_messwert_transitions_to_abgeschlossen() {
	let state = LieferendeState::Bestaetigt {
		malo: malo(),
		lieferende: lieferende(),
	};
	let out = reduce(
		state,
		LieferendeEvent::SchlussturnusmesswertEmpfangen(schlussturnusmesswert()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		LieferendeState::Abgeschlossen {
			malo: malo(),
			zaehlerstand: 12345.67,
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path_idle_to_abgeschlossen() {
	// Step 1: Idle -> AbmeldungGesendet
	let out = reduce(LieferendeState::Idle, LieferendeEvent::AbmeldungEingegangen(abmeldung()))
		.expect("step 1");
	assert!(matches!(out.state, LieferendeState::AbmeldungGesendet { .. }));

	// Step 2: AbmeldungGesendet -> Bestaetigt
	let out = reduce(out.state, LieferendeEvent::AbmeldungBestaetigt(bestaetigung()))
		.expect("step 2");
	assert!(matches!(out.state, LieferendeState::Bestaetigt { .. }));
	assert_eq!(out.nachrichten.len(), 1);

	// Step 3: Bestaetigt -> Abgeschlossen
	let out = reduce(
		out.state,
		LieferendeEvent::SchlussturnusmesswertEmpfangen(schlussturnusmesswert()),
	)
	.expect("step 3");
	assert!(matches!(out.state, LieferendeState::Abgeschlossen { .. }));
}

// --- Rejection ---

#[test]
fn rejection_from_abmeldung_gesendet() {
	let state = LieferendeState::AbmeldungGesendet {
		malo: malo(),
		lf: lf_id(),
		nb: nb_id(),
		lieferende: lieferende(),
	};
	let out = reduce(
		state,
		LieferendeEvent::Abgelehnt {
			grund: AblehnungsGrund::MaloUnbekannt,
		},
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		LieferendeState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::MaloUnbekannt,
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	let msg = &out.nachrichten[0];
	assert_eq!(msg.absender, nb_id());
	assert_eq!(msg.empfaenger, lf_id());
	assert!(matches!(msg.payload, NachrichtenPayload::UtilmdAblehnung(_)));
}

// --- Timeout ---

#[test]
fn timeout_from_abmeldung_gesendet() {
	let state = LieferendeState::AbmeldungGesendet {
		malo: malo(),
		lf: lf_id(),
		nb: nb_id(),
		lieferende: lieferende(),
	};
	let out = reduce(state, LieferendeEvent::FristUeberschritten)
		.expect("should succeed");
	assert_eq!(
		out.state,
		LieferendeState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::Fristverletzung,
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	assert!(matches!(out.nachrichten[0].payload, NachrichtenPayload::UtilmdAblehnung(_)));
}

#[test]
fn timeout_from_bestaetigt() {
	let state = LieferendeState::Bestaetigt {
		malo: malo(),
		lieferende: lieferende(),
	};
	let out = reduce(state, LieferendeEvent::FristUeberschritten)
		.expect("should succeed");
	assert_eq!(
		out.state,
		LieferendeState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::Fristverletzung,
		}
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(
		LieferendeState::Idle,
		LieferendeEvent::AbmeldungBestaetigt(bestaetigung()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgeschlossen_cannot_receive_any_event() {
	let state = LieferendeState::Abgeschlossen {
		malo: malo(),
		zaehlerstand: 12345.67,
	};
	let result = reduce(state, LieferendeEvent::AbmeldungEingegangen(abmeldung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_cannot_receive_any_event() {
	let state = LieferendeState::Abgelehnt {
		malo: malo(),
		grund: AblehnungsGrund::MaloUnbekannt,
	};
	let result = reduce(state, LieferendeEvent::AbmeldungEingegangen(abmeldung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn idle_cannot_timeout() {
	let result = reduce(LieferendeState::Idle, LieferendeEvent::FristUeberschritten);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
