use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdFahrplan;
use mako_types::ids::MarktpartnerId;

use super::fahrplan::{FahrplanEvent, FahrplanState, reduce};

fn ressource_id() -> String {
	"TR-001".to_string()
}
fn absender_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}

fn fahrplan_msg() -> RdFahrplan {
	RdFahrplan {
		ressource_id: ressource_id(),
		zeitreihe: vec![],
	}
}

#[test]
fn happy_path_idle_to_bestaetigt() {
	let out = reduce(FahrplanState::Idle, FahrplanEvent::FahrplanGesendet(fahrplan_msg()))
		.expect("step 1");
	assert!(matches!(out.state, FahrplanState::FahrplanGesendet { .. }));
	assert_eq!(out.nachrichten.len(), 1);

	let out = reduce(out.state, FahrplanEvent::Weitergeleitet).expect("step 2");
	assert!(matches!(out.state, FahrplanState::Weitergeleitet { .. }));

	let out = reduce(out.state, FahrplanEvent::Bestaetigt).expect("step 3");
	assert_eq!(
		out.state,
		FahrplanState::Bestaetigt {
			ressource_id: ressource_id(),
		}
	);
}

#[test]
fn rejection_from_weitergeleitet() {
	let state = FahrplanState::Weitergeleitet {
		ressource_id: ressource_id(),
	};
	let out = reduce(
		state,
		FahrplanEvent::Abgelehnt {
			grund: "Zeitreihe ungültig".to_string(),
		},
	)
	.expect("should succeed");
	assert!(matches!(out.state, FahrplanState::Abgelehnt { .. }));
}

#[test]
fn idle_cannot_receive_bestaetigt() {
	let result = reduce(FahrplanState::Idle, FahrplanEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_is_terminal() {
	let state = FahrplanState::Bestaetigt {
		ressource_id: ressource_id(),
	};
	let result = reduce(state, FahrplanEvent::Weitergeleitet);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
