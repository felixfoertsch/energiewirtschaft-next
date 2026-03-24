use chrono::NaiveDateTime;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdEngpass;
use mako_types::ids::MarktpartnerId;

use super::engpass::{EngpassEvent, EngpassState, reduce};

fn dt(s: &str) -> NaiveDateTime {
	NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
}

fn engpass_msg() -> RdEngpass {
	RdEngpass {
		netzgebiet: "Netz-Nord".to_string(),
		engpass_start: dt("2025-07-01 08:00:00"),
		engpass_ende: dt("2025-07-01 16:00:00"),
		betroffene_leistung_kw: 100.0,
	}
}

#[test]
fn happy_path_idle_to_bestaetigt() {
	let out = reduce(EngpassState::Idle, EngpassEvent::EngpassGemeldet(engpass_msg()))
		.expect("step 1");
	assert!(matches!(out.state, EngpassState::EngpassGemeldet { .. }));
	assert_eq!(out.nachrichten.len(), 1);

	let out = reduce(out.state, EngpassEvent::Bestaetigt).expect("step 2");
	assert_eq!(
		out.state,
		EngpassState::Bestaetigt {
			netzgebiet: "Netz-Nord".to_string(),
		}
	);
}

#[test]
fn idle_cannot_receive_bestaetigt() {
	let result = reduce(EngpassState::Idle, EngpassEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_is_terminal() {
	let state = EngpassState::Bestaetigt {
		netzgebiet: "Netz-Nord".to_string(),
	};
	let result = reduce(state, EngpassEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
