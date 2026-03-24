use chrono::NaiveDateTime;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	MsconsEinspeiseMesswerte, Messwert, MesswertStatus, UtilmdAnmeldungErzeugung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::NachrichtenPayload;
use mako_types::rolle::MarktRolle;

use super::erzeugungsanlagen::{
	ErzeugungsanlagenEvent, ErzeugungsanlagenState, reduce,
};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn betreiber_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}

fn anmeldung() -> UtilmdAnmeldungErzeugung {
	UtilmdAnmeldungErzeugung {
		malo_id: malo(),
		anlagenbetreiber: betreiber_id(),
		eeg_anlage: true,
		installierte_leistung_kw: 10.0,
	}
}

fn messwerte() -> MsconsEinspeiseMesswerte {
	MsconsEinspeiseMesswerte {
		malo_id: malo(),
		werte: vec![Messwert {
			zeitpunkt: NaiveDateTime::parse_from_str(
				"2025-07-01 00:00:00",
				"%Y-%m-%d %H:%M:%S",
			)
			.unwrap(),
			wert: 42.0,
			einheit: "kWh".to_string(),
			status: MesswertStatus::Gemessen,
		}],
	}
}

// --- Happy path ---

#[test]
fn idle_plus_anmeldung_transitions_to_eingegangen() {
	let out = reduce(
		ErzeugungsanlagenState::Idle,
		ErzeugungsanlagenEvent::AnmeldungEmpfangen(anmeldung()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		ErzeugungsanlagenState::AnmeldungEingegangen {
			malo: malo(),
			anlagenbetreiber: betreiber_id(),
			nb: nb_id(),
			eeg_anlage: true,
			installierte_leistung_kw: 10.0,
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn eingegangen_plus_bestaetigt_sends_message() {
	let state = ErzeugungsanlagenState::AnmeldungEingegangen {
		malo: malo(),
		anlagenbetreiber: betreiber_id(),
		nb: nb_id(),
		eeg_anlage: true,
		installierte_leistung_kw: 10.0,
	};
	let out = reduce(state, ErzeugungsanlagenEvent::Bestaetigt).expect("should succeed");
	assert_eq!(
		out.state,
		ErzeugungsanlagenState::Bestaetigt {
			malo: malo(),
			anlagenbetreiber: betreiber_id(),
			nb: nb_id(),
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	let msg = &out.nachrichten[0];
	assert_eq!(msg.absender, nb_id());
	assert_eq!(msg.absender_rolle, MarktRolle::Netzbetreiber);
	assert_eq!(msg.empfaenger, betreiber_id());
	assert_eq!(msg.empfaenger_rolle, MarktRolle::BetreiberErzeugungsanlage);
	assert!(matches!(msg.payload, NachrichtenPayload::UtilmdAnmeldungErzeugung(_)));
}

#[test]
fn bestaetigt_plus_zuordnung_informiert() {
	let state = ErzeugungsanlagenState::Bestaetigt {
		malo: malo(),
		anlagenbetreiber: betreiber_id(),
		nb: nb_id(),
	};
	let out = reduce(state, ErzeugungsanlagenEvent::ZuordnungInformiert)
		.expect("should succeed");
	assert_eq!(
		out.state,
		ErzeugungsanlagenState::ZuordnungInformiert {
			malo: malo(),
			anlagenbetreiber: betreiber_id(),
			nb: nb_id(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn zuordnung_informiert_plus_messwerte() {
	let state = ErzeugungsanlagenState::ZuordnungInformiert {
		malo: malo(),
		anlagenbetreiber: betreiber_id(),
		nb: nb_id(),
	};
	let out = reduce(
		state,
		ErzeugungsanlagenEvent::EinspeiseMesswerteEmpfangen(messwerte()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		ErzeugungsanlagenState::MesswerteAktiv {
			malo: malo(),
			anlagenbetreiber: betreiber_id(),
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::MsconsEinspeiseMesswerte(_)
	));
}

#[test]
fn full_happy_path() {
	let out = reduce(
		ErzeugungsanlagenState::Idle,
		ErzeugungsanlagenEvent::AnmeldungEmpfangen(anmeldung()),
	)
	.expect("step 1");
	assert!(matches!(out.state, ErzeugungsanlagenState::AnmeldungEingegangen { .. }));

	let out = reduce(out.state, ErzeugungsanlagenEvent::Bestaetigt).expect("step 2");
	assert!(matches!(out.state, ErzeugungsanlagenState::Bestaetigt { .. }));
	assert_eq!(out.nachrichten.len(), 1);

	let out = reduce(out.state, ErzeugungsanlagenEvent::ZuordnungInformiert)
		.expect("step 3");
	assert!(matches!(out.state, ErzeugungsanlagenState::ZuordnungInformiert { .. }));

	let out = reduce(
		out.state,
		ErzeugungsanlagenEvent::EinspeiseMesswerteEmpfangen(messwerte()),
	)
	.expect("step 4");
	assert!(matches!(out.state, ErzeugungsanlagenState::MesswerteAktiv { .. }));
}

// --- Rejection ---

#[test]
fn eingegangen_plus_abgelehnt() {
	let state = ErzeugungsanlagenState::AnmeldungEingegangen {
		malo: malo(),
		anlagenbetreiber: betreiber_id(),
		nb: nb_id(),
		eeg_anlage: true,
		installierte_leistung_kw: 10.0,
	};
	let out = reduce(
		state,
		ErzeugungsanlagenEvent::Abgelehnt {
			grund: "Technische Gründe".to_string(),
		},
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		ErzeugungsanlagenState::Abgelehnt {
			malo: malo(),
			grund: "Technische Gründe".to_string(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_bestaetigt() {
	let result = reduce(ErzeugungsanlagenState::Idle, ErzeugungsanlagenEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_cannot_receive_anmeldung() {
	let state = ErzeugungsanlagenState::Bestaetigt {
		malo: malo(),
		anlagenbetreiber: betreiber_id(),
		nb: nb_id(),
	};
	let result = reduce(
		state,
		ErzeugungsanlagenEvent::AnmeldungEmpfangen(anmeldung()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn messwerte_aktiv_cannot_receive_any_event() {
	let state = ErzeugungsanlagenState::MesswerteAktiv {
		malo: malo(),
		anlagenbetreiber: betreiber_id(),
	};
	let result = reduce(state, ErzeugungsanlagenEvent::Bestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_cannot_receive_any_event() {
	let state = ErzeugungsanlagenState::Abgelehnt {
		malo: malo(),
		grund: "test".to_string(),
	};
	let result = reduce(
		state,
		ErzeugungsanlagenEvent::AnmeldungEmpfangen(anmeldung()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
