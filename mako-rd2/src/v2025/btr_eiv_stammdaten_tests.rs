use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{RdStammdaten, RessourceTyp};
use mako_types::ids::MaLoId;
use mako_types::rolle::MarktRolle::*;

use super::btr_eiv_stammdaten::{BtrEivStammdatenEvent, BtrEivStammdatenState, reduce};

fn stammdaten_msg() -> RdStammdaten {
	RdStammdaten {
		ressource_id: "TR-001".to_string(),
		ressource_typ: RessourceTyp::TechnischeRessource,
		standort_malo: MaLoId::new("51238696788").unwrap(),
		installierte_leistung_kw: 50.0,
	}
}

#[test]
fn happy_path_idle_to_bestaetigt() {
	let out = reduce(
		BtrEivStammdatenState::Idle,
		BtrEivStammdatenEvent::StammdatenGesendet(stammdaten_msg()),
	)
	.expect("step 1");
	assert!(matches!(
		out.state,
		BtrEivStammdatenState::Gesendet { .. }
	));
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(
		out.nachrichten[0].absender_rolle,
		BetreiberTechnischeRessource
	);
	assert_eq!(out.nachrichten[0].empfaenger_rolle, Einsatzverantwortlicher);

	let out = reduce(out.state, BtrEivStammdatenEvent::Bestaetigt).expect("step 2");
	assert_eq!(
		out.state,
		BtrEivStammdatenState::Bestaetigt {
			ressource_id: "TR-001".to_string(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn idle_cannot_receive_bestaetigt() {
	let result = reduce(
		BtrEivStammdatenState::Idle,
		BtrEivStammdatenEvent::Bestaetigt,
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}
