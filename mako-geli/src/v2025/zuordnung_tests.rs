use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{UtilmdZuordnungsliste, ZuordnungsEintrag};
use mako_types::ids::{MaLoId, MarktpartnerId};

use super::zuordnung::{ZuordnungEvent, ZuordnungState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn lf_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn liste() -> UtilmdZuordnungsliste {
	UtilmdZuordnungsliste {
		eintraege: vec![ZuordnungsEintrag {
			malo_id: malo(),
			zugeordnet_an: lf_id(),
			gueltig_ab: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
		}],
	}
}

// --- Happy path ---

#[test]
fn idle_plus_liste_transitions_to_versendet() {
	let out = reduce(ZuordnungState::Idle, ZuordnungEvent::ListeEmpfangen(liste()))
		.expect("should succeed");
	assert!(matches!(out.state, ZuordnungState::ListeVersendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(ZuordnungState::Idle, ZuordnungEvent::ListeEmpfangen(liste()))
		.expect("step 1");
	let out = reduce(out.state, ZuordnungEvent::Bestaetigt).expect("step 2");
	assert_eq!(out.state, ZuordnungState::Bestaetigt);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(ZuordnungState::Idle, ZuordnungEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_is_terminal() {
	let result = reduce(ZuordnungState::Bestaetigt, ZuordnungEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
