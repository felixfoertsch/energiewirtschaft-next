use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, UtilmdMsbWechselAnmeldung};
use mako_types::ids::{MeLoId, MarktpartnerId};
use mako_types::rolle::MarktRolle::*;

use super::msb_wechsel::{MsbWechselEvent, MsbWechselState, reduce};

fn melo() -> MeLoId {
	MeLoId::new("DE000000000000000000000000000000A").unwrap()
}
fn msb_neu_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}
fn msb_alt_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000028").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}
fn wechseldatum() -> NaiveDate {
	NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
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
	// v2020: MSB-centric Messwesen, first introduction of MSB as data hub
	let out = reduce(MsbWechselState::Idle, MsbWechselEvent::AnmeldungEmpfangen(anmeldung()))
		.expect("step 1");
	assert!(matches!(out.state, MsbWechselState::AnmeldungEingegangen { .. }));
	// MSB-Wechsel Anmeldung: MessstellenbetreiberNeu → Netzbetreiber
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender_rolle, MessstellenbetreiberNeu);
	assert_eq!(out.nachrichten[0].empfaenger_rolle, Netzbetreiber);

	let out = reduce(out.state, MsbWechselEvent::NbBestaetigt).expect("step 2");
	assert!(matches!(out.state, MsbWechselState::Bestaetigt { .. }));
	// MSB-Wechsel Bestätigung: Netzbetreiber → MessstellenbetreiberNeu
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender_rolle, Netzbetreiber);
	assert_eq!(out.nachrichten[0].empfaenger_rolle, MessstellenbetreiberNeu);
	assert_eq!(out.nachrichten[0].absender, nb_id());
	assert_eq!(out.nachrichten[0].empfaenger, msb_neu_id());

	let out = reduce(out.state, MsbWechselEvent::AbmeldungMsbAltInformiert).expect("step 3");
	assert!(matches!(out.state, MsbWechselState::AbmeldungInformiert { .. }));
	// MSB-Wechsel Abmeldung an alten MSB: Netzbetreiber → MessstellenbetreiberAlt
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender_rolle, Netzbetreiber);
	assert_eq!(out.nachrichten[0].empfaenger_rolle, MessstellenbetreiberAlt);
	assert_eq!(out.nachrichten[0].empfaenger, msb_alt_id());

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
fn abgelehnt_is_terminal() {
	let state = MsbWechselState::Abgelehnt {
		melo: melo(),
		grund: AblehnungsGrund::MaloUnbekannt,
	};
	let result = reduce(state, MsbWechselEvent::AnmeldungEmpfangen(anmeldung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
