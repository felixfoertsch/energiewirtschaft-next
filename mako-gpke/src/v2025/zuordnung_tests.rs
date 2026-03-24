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

fn zuordnungsliste() -> UtilmdZuordnungsliste {
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
	let out = reduce(
		ZuordnungState::Idle,
		ZuordnungEvent::ListeEmpfangen(zuordnungsliste()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		ZuordnungState::ListeVersendet {
			eintraege: vec![ZuordnungsEintrag {
				malo_id: malo(),
				zugeordnet_an: lf_id(),
				gueltig_ab: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
			}],
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn versendet_plus_bestaetigt_transitions_to_bestaetigt() {
	let state = ZuordnungState::ListeVersendet {
		eintraege: vec![ZuordnungsEintrag {
			malo_id: malo(),
			zugeordnet_an: lf_id(),
			gueltig_ab: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
		}],
	};
	let out = reduce(state, ZuordnungEvent::Bestaetigt)
		.expect("should succeed");
	assert_eq!(out.state, ZuordnungState::Bestaetigt);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(
		ZuordnungState::Idle,
		ZuordnungEvent::ListeEmpfangen(zuordnungsliste()),
	)
	.expect("step 1");
	assert!(matches!(out.state, ZuordnungState::ListeVersendet { .. }));

	let out = reduce(out.state, ZuordnungEvent::Bestaetigt)
		.expect("step 2");
	assert_eq!(out.state, ZuordnungState::Bestaetigt);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(ZuordnungState::Idle, ZuordnungEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_cannot_receive_any_event() {
	let result = reduce(ZuordnungState::Bestaetigt, ZuordnungEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_cannot_receive_liste() {
	let result = reduce(
		ZuordnungState::Bestaetigt,
		ZuordnungEvent::ListeEmpfangen(zuordnungsliste()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
