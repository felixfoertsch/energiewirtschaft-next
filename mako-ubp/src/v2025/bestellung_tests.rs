use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	OrdersBestellung, OrdrspBestellantwort, QuotesAngebot, ReqoteAngebotsanfrage,
};
use mako_types::ids::{MeLoId, MarktpartnerId};
use mako_types::nachricht::NachrichtenPayload;

use super::bestellung::{BestellungEvent, BestellungState, reduce};

fn melo() -> MeLoId {
	MeLoId::new("DE000000000000000000000000000000A").unwrap()
}
fn lf_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn msb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}

fn anfrage() -> ReqoteAngebotsanfrage {
	ReqoteAngebotsanfrage {
		melo_id: melo(),
		anfragender: lf_id(),
		produkt_beschreibung: "iMSys Standard".to_string(),
	}
}

fn angebot() -> QuotesAngebot {
	QuotesAngebot {
		melo_id: melo(),
		anbieter: msb_id(),
		preis_ct_pro_monat: 199.0,
		produkt_beschreibung: "iMSys Standard".to_string(),
	}
}

fn bestellung() -> OrdersBestellung {
	OrdersBestellung {
		melo_id: melo(),
		besteller: lf_id(),
		referenz_angebot: "ANG-2025-001".to_string(),
	}
}

fn antwort_positiv() -> OrdrspBestellantwort {
	OrdrspBestellantwort {
		melo_id: melo(),
		angenommen: true,
		grund: None,
	}
}

fn antwort_negativ() -> OrdrspBestellantwort {
	OrdrspBestellantwort {
		melo_id: melo(),
		angenommen: false,
		grund: Some("Kapazität erschöpft".to_string()),
	}
}

// --- Happy path ---

#[test]
fn idle_plus_anfrage() {
	let out = reduce(BestellungState::Idle, BestellungEvent::AnfrageGesendet(anfrage()))
		.expect("should succeed");
	assert!(matches!(out.state, BestellungState::AnfrageGesendet { .. }));
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender, lf_id());
	assert_eq!(out.nachrichten[0].empfaenger, msb_id());
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::ReqoteAngebotsanfrage(_)
	));
}

#[test]
fn anfrage_plus_angebot() {
	let state = BestellungState::AnfrageGesendet {
		melo: melo(),
		anfragender: lf_id(),
		produkt: "iMSys Standard".to_string(),
	};
	let out = reduce(state, BestellungEvent::AngebotEmpfangen(angebot())).expect("should succeed");
	match &out.state {
		BestellungState::AngebotErhalten { preis_ct_pro_monat, .. } => {
			assert!((preis_ct_pro_monat - 199.0).abs() < f64::EPSILON);
		}
		other => panic!("expected AngebotErhalten, got {other:?}"),
	}
}

#[test]
fn angebot_plus_bestellung() {
	let state = BestellungState::AngebotErhalten {
		melo: melo(),
		anfragender: lf_id(),
		anbieter: msb_id(),
		preis_ct_pro_monat: 199.0,
		produkt: "iMSys Standard".to_string(),
	};
	let out = reduce(state, BestellungEvent::BestellungGesendet(bestellung()))
		.expect("should succeed");
	assert!(matches!(out.state, BestellungState::Bestellt { .. }));
	assert_eq!(out.nachrichten.len(), 1);
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::OrdersBestellung(_)
	));
}

#[test]
fn bestellt_plus_positive_antwort() {
	let state = BestellungState::Bestellt {
		melo: melo(),
		besteller: lf_id(),
		referenz_angebot: "ANG-2025-001".to_string(),
	};
	let out = reduce(state, BestellungEvent::AntwortEmpfangen(antwort_positiv()))
		.expect("should succeed");
	assert_eq!(out.state, BestellungState::Bestaetigt { melo: melo() });
}

#[test]
fn bestellt_plus_negative_antwort() {
	let state = BestellungState::Bestellt {
		melo: melo(),
		besteller: lf_id(),
		referenz_angebot: "ANG-2025-001".to_string(),
	};
	let out = reduce(state, BestellungEvent::AntwortEmpfangen(antwort_negativ()))
		.expect("should succeed");
	assert_eq!(
		out.state,
		BestellungState::Abgelehnt {
			melo: melo(),
			grund: "Kapazität erschöpft".to_string(),
		}
	);
}

#[test]
fn full_happy_path() {
	let out = reduce(BestellungState::Idle, BestellungEvent::AnfrageGesendet(anfrage()))
		.expect("step 1");
	let out = reduce(out.state, BestellungEvent::AngebotEmpfangen(angebot())).expect("step 2");
	let out = reduce(out.state, BestellungEvent::BestellungGesendet(bestellung()))
		.expect("step 3");
	let out = reduce(out.state, BestellungEvent::AntwortEmpfangen(antwort_positiv()))
		.expect("step 4");
	assert!(matches!(out.state, BestellungState::Bestaetigt { .. }));
}

// --- Timeout ---

#[test]
fn timeout_from_anfrage_gesendet() {
	let state = BestellungState::AnfrageGesendet {
		melo: melo(),
		anfragender: lf_id(),
		produkt: "iMSys Standard".to_string(),
	};
	let out = reduce(state, BestellungEvent::FristUeberschritten).expect("should succeed");
	assert!(matches!(out.state, BestellungState::Abgelehnt { .. }));
}

#[test]
fn timeout_from_bestellt() {
	let state = BestellungState::Bestellt {
		melo: melo(),
		besteller: lf_id(),
		referenz_angebot: "ANG-2025-001".to_string(),
	};
	let out = reduce(state, BestellungEvent::FristUeberschritten).expect("should succeed");
	assert!(matches!(out.state, BestellungState::Abgelehnt { .. }));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_angebot() {
	let result = reduce(
		BestellungState::Idle,
		BestellungEvent::AngebotEmpfangen(angebot()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn bestaetigt_cannot_receive_event() {
	let state = BestellungState::Bestaetigt { melo: melo() };
	let result = reduce(state, BestellungEvent::AnfrageGesendet(anfrage()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_cannot_receive_event() {
	let state = BestellungState::Abgelehnt {
		melo: melo(),
		grund: "test".to_string(),
	};
	let result = reduce(state, BestellungEvent::AnfrageGesendet(anfrage()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
