use chrono::NaiveDate;
use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, UtilmdMsbWechselAnmeldung};
use mako_types::ids::{MeLoId, MarktpartnerId};
use mako_types::nachricht::NachrichtenPayload;

use super::msb_wechsel::{MsbWechselEvent, MsbWechselState, reduce};

fn melo() -> MeLoId {
	MeLoId::new("DE000000000000000000000000000000A").unwrap()
}
fn msb_neu_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}
fn wechseldatum() -> NaiveDate {
	NaiveDate::from_ymd_opt(2025, 7, 1).unwrap()
}

fn anmeldung() -> UtilmdMsbWechselAnmeldung {
	UtilmdMsbWechselAnmeldung {
		melo_id: melo(),
		msb_neu: msb_neu_id(),
		wechseldatum: wechseldatum(),
	}
}

// --- Happy path ---

#[test]
fn idle_plus_anmeldung_transitions_to_eingegangen() {
	let out = reduce(MsbWechselState::Idle, MsbWechselEvent::AnmeldungEmpfangen(anmeldung()))
		.expect("should succeed");
	assert_eq!(
		out.state,
		MsbWechselState::AnmeldungEingegangen {
			melo: melo(),
			msb_neu: msb_neu_id(),
			wechseldatum: wechseldatum(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn eingegangen_plus_bestaetigt_transitions() {
	let state = MsbWechselState::AnmeldungEingegangen {
		melo: melo(),
		msb_neu: msb_neu_id(),
		wechseldatum: wechseldatum(),
	};
	let out = reduce(state, MsbWechselEvent::NbBestaetigt).expect("should succeed");
	assert_eq!(
		out.state,
		MsbWechselState::Bestaetigt {
			melo: melo(),
			msb_neu: msb_neu_id(),
			wechseldatum: wechseldatum(),
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender, nb_id());
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::UtilmdMsbWechselAnmeldung(_)
	));
}

#[test]
fn bestaetigt_plus_abmeldung_informiert() {
	let state = MsbWechselState::Bestaetigt {
		melo: melo(),
		msb_neu: msb_neu_id(),
		wechseldatum: wechseldatum(),
	};
	let out = reduce(state, MsbWechselEvent::AbmeldungMsbAltInformiert).expect("should succeed");
	assert!(matches!(out.state, MsbWechselState::AbmeldungInformiert { .. }));
}

#[test]
fn abmeldung_informiert_plus_zaehlerstand_completes() {
	let state = MsbWechselState::AbmeldungInformiert {
		melo: melo(),
		msb_neu: msb_neu_id(),
		wechseldatum: wechseldatum(),
	};
	let out = reduce(
		state,
		MsbWechselEvent::SchlusszaehlerstandEmpfangen { zaehlerstand: 12345.0 },
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		MsbWechselState::Abgeschlossen {
			melo: melo(),
			schlusszaehlerstand: 12345.0,
		}
	);
}

#[test]
fn full_happy_path() {
	let out = reduce(MsbWechselState::Idle, MsbWechselEvent::AnmeldungEmpfangen(anmeldung()))
		.expect("step 1");
	let out = reduce(out.state, MsbWechselEvent::NbBestaetigt).expect("step 2");
	let out = reduce(out.state, MsbWechselEvent::AbmeldungMsbAltInformiert).expect("step 3");
	let out = reduce(
		out.state,
		MsbWechselEvent::SchlusszaehlerstandEmpfangen { zaehlerstand: 99.9 },
	)
	.expect("step 4");
	assert!(matches!(out.state, MsbWechselState::Abgeschlossen { .. }));
}

// --- Rejection ---

#[test]
fn rejection_from_eingegangen() {
	let state = MsbWechselState::AnmeldungEingegangen {
		melo: melo(),
		msb_neu: msb_neu_id(),
		wechseldatum: wechseldatum(),
	};
	let out = reduce(
		state,
		MsbWechselEvent::NbAbgelehnt {
			grund: AblehnungsGrund::MaloUnbekannt,
		},
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		MsbWechselState::Abgelehnt {
			melo: melo(),
			grund: AblehnungsGrund::MaloUnbekannt,
		}
	);
}

// --- Timeout ---

#[test]
fn timeout_from_eingegangen() {
	let state = MsbWechselState::AnmeldungEingegangen {
		melo: melo(),
		msb_neu: msb_neu_id(),
		wechseldatum: wechseldatum(),
	};
	let out = reduce(state, MsbWechselEvent::FristUeberschritten).expect("should succeed");
	assert_eq!(
		out.state,
		MsbWechselState::Abgelehnt {
			melo: melo(),
			grund: AblehnungsGrund::Fristverletzung,
		}
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_bestaetigen() {
	let result = reduce(MsbWechselState::Idle, MsbWechselEvent::NbBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgeschlossen_cannot_receive_event() {
	let state = MsbWechselState::Abgeschlossen {
		melo: melo(),
		schlusszaehlerstand: 100.0,
	};
	let result = reduce(state, MsbWechselEvent::NbBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_cannot_receive_event() {
	let state = MsbWechselState::Abgelehnt {
		melo: melo(),
		grund: AblehnungsGrund::MaloUnbekannt,
	};
	let result = reduce(state, MsbWechselEvent::AnmeldungEmpfangen(anmeldung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
