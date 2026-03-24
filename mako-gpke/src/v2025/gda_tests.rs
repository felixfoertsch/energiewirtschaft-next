use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	AblehnungsGrund, Stammdatenfeld, UtilmdGeschaeftsdatenanfrage, UtilmdGeschaeftsdatenantwort,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::NachrichtenPayload;

use super::gda::{GdaEvent, GdaState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn lf_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}

fn anfrage() -> UtilmdGeschaeftsdatenanfrage {
	UtilmdGeschaeftsdatenanfrage {
		malo_id: malo(),
		anfragender: lf_id(),
	}
}

fn antwort() -> UtilmdGeschaeftsdatenantwort {
	UtilmdGeschaeftsdatenantwort {
		malo_id: malo(),
		stammdaten: vec![Stammdatenfeld {
			feld: "Jahresverbrauch".to_string(),
			alter_wert: None,
			neuer_wert: "4200".to_string(),
		}],
	}
}

// --- Happy path ---

#[test]
fn idle_plus_anfrage_transitions_to_gesendet() {
	let out = reduce(GdaState::Idle, GdaEvent::AnfrageEingegangen(anfrage()))
		.expect("should succeed");
	assert_eq!(
		out.state,
		GdaState::AnfrageGesendet {
			malo: malo(),
			anfragender: lf_id(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn gesendet_plus_antwort_transitions_to_beantwortet() {
	let state = GdaState::AnfrageGesendet {
		malo: malo(),
		anfragender: lf_id(),
	};
	let out = reduce(state, GdaEvent::AntwortEmpfangen(antwort()))
		.expect("should succeed");
	assert_eq!(
		out.state,
		GdaState::Beantwortet {
			malo: malo(),
			stammdaten: vec![Stammdatenfeld {
				feld: "Jahresverbrauch".to_string(),
				alter_wert: None,
				neuer_wert: "4200".to_string(),
			}],
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	let msg = &out.nachrichten[0];
	assert_eq!(msg.absender, nb_id());
	assert_eq!(msg.empfaenger, lf_id());
	assert!(matches!(
		msg.payload,
		NachrichtenPayload::UtilmdGeschaeftsdatenantwort(_)
	));
}

#[test]
fn full_happy_path() {
	let out = reduce(GdaState::Idle, GdaEvent::AnfrageEingegangen(anfrage()))
		.expect("step 1");
	assert!(matches!(out.state, GdaState::AnfrageGesendet { .. }));

	let out = reduce(out.state, GdaEvent::AntwortEmpfangen(antwort()))
		.expect("step 2");
	assert!(matches!(out.state, GdaState::Beantwortet { .. }));
	assert_eq!(out.nachrichten.len(), 1);
}

// --- Rejection ---

#[test]
fn rejection_from_anfrage_gesendet() {
	let state = GdaState::AnfrageGesendet {
		malo: malo(),
		anfragender: lf_id(),
	};
	let out = reduce(
		state,
		GdaEvent::Abgelehnt {
			grund: AblehnungsGrund::MaloUnbekannt,
		},
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		GdaState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::MaloUnbekannt,
		}
	);
	assert!(out.nachrichten.is_empty());
}

// --- Timeout ---

#[test]
fn timeout_from_anfrage_gesendet() {
	let state = GdaState::AnfrageGesendet {
		malo: malo(),
		anfragender: lf_id(),
	};
	let out = reduce(state, GdaEvent::FristUeberschritten)
		.expect("should succeed");
	assert_eq!(
		out.state,
		GdaState::Abgelehnt {
			malo: malo(),
			grund: AblehnungsGrund::Fristverletzung,
		}
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_antwort() {
	let result = reduce(GdaState::Idle, GdaEvent::AntwortEmpfangen(antwort()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn beantwortet_cannot_receive_any_event() {
	let state = GdaState::Beantwortet {
		malo: malo(),
		stammdaten: vec![],
	};
	let result = reduce(state, GdaEvent::AnfrageEingegangen(anfrage()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_cannot_receive_any_event() {
	let state = GdaState::Abgelehnt {
		malo: malo(),
		grund: AblehnungsGrund::MaloUnbekannt,
	};
	let result = reduce(state, GdaEvent::AnfrageEingegangen(anfrage()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn idle_cannot_timeout() {
	let result = reduce(GdaState::Idle, GdaEvent::FristUeberschritten);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
