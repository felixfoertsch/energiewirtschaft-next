use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, UtilmdMsbWechselAnmeldung};
use mako_types::ids::{MeLoId, MarktpartnerId};

use super::msb_wechsel::{MsbWechselEvent, MsbWechselState, reduce};

fn melo() -> MeLoId {
	MeLoId::new("DE000000000000000000000000000000A").unwrap()
}
fn msb_neu_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}
fn wechseldatum() -> NaiveDate {
	NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
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
fn full_happy_path() {
	let out = reduce(MsbWechselState::Idle, MsbWechselEvent::AnmeldungEmpfangen(anmeldung()))
		.expect("step 1");
	assert!(matches!(out.state, MsbWechselState::AnmeldungEingegangen { .. }));

	let out = reduce(out.state, MsbWechselEvent::NbBestaetigt).expect("step 2");
	assert!(matches!(out.state, MsbWechselState::Bestaetigt { .. }));
	assert_eq!(out.nachrichten.len(), 1);

	let out = reduce(out.state, MsbWechselEvent::AbmeldungMsbAltInformiert).expect("step 3");
	assert!(matches!(out.state, MsbWechselState::AbmeldungInformiert { .. }));

	let out = reduce(
		out.state,
		MsbWechselEvent::SchlusszaehlerstandEmpfangen { zaehlerstand: 99.9 },
	)
	.expect("step 4");
	assert!(matches!(out.state, MsbWechselState::Abgeschlossen { .. }));
}

// --- Invalid transition ---

#[test]
fn idle_cannot_bestaetigen() {
	let result = reduce(MsbWechselState::Idle, MsbWechselEvent::NbBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgeschlossen_is_terminal() {
	let state = MsbWechselState::Abgeschlossen {
		melo: melo(),
		schlusszaehlerstand: 100.0,
	};
	let result = reduce(state, MsbWechselEvent::NbBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
