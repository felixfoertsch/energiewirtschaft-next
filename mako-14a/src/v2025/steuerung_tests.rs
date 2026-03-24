use chrono::NaiveDateTime;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	ClsSteuersignal, MsconsEinspeiseMesswerte, Messwert, MesswertStatus,
	SteuerbarerGeraetetyp, Steuerungsart, UtilmdSteuerbareVerbrauchseinrichtung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};

use super::steuerung::{SteuerungEvent, SteuerungState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn anmelder_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}

fn dt(s: &str) -> NaiveDateTime {
	NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
}

fn anmeldung() -> UtilmdSteuerbareVerbrauchseinrichtung {
	UtilmdSteuerbareVerbrauchseinrichtung {
		malo_id: malo(),
		geraetetyp: SteuerbarerGeraetetyp::Waermepumpe,
		max_leistung_kw: 11.0,
	}
}

fn steuersignal() -> ClsSteuersignal {
	ClsSteuersignal {
		malo_id: malo(),
		steuerung: Steuerungsart::Dimmung { prozent: 60 },
		zeitpunkt: dt("2025-07-01 14:00:00"),
	}
}

fn messwerte() -> MsconsEinspeiseMesswerte {
	MsconsEinspeiseMesswerte {
		malo_id: malo(),
		werte: vec![Messwert {
			zeitpunkt: dt("2025-07-01 15:00:00"),
			wert: 6.6,
			einheit: "kW".to_string(),
			status: MesswertStatus::Gemessen,
		}],
	}
}

// --- Happy path: full lifecycle ---

#[test]
fn full_lifecycle_idle_to_gesteuert() {
	// Step 1: Idle → Angemeldet
	let out = reduce(
		SteuerungState::Idle,
		SteuerungEvent::AnmeldungEmpfangen(anmeldung()),
	)
	.expect("step 1");
	assert!(matches!(out.state, SteuerungState::Angemeldet { .. }));
	assert_eq!(out.nachrichten.len(), 1);

	// Step 2: Angemeldet → Konfiguriert
	let out = reduce(out.state, SteuerungEvent::KonfigurationGesendet)
		.expect("step 2");
	assert!(matches!(out.state, SteuerungState::Konfiguriert { .. }));
	assert!(out.nachrichten.is_empty());

	// Step 3: Konfiguriert → Aktiv
	let out = reduce(
		out.state,
		SteuerungEvent::SteuersignalGesendet(steuersignal()),
	)
	.expect("step 3");
	assert!(matches!(out.state, SteuerungState::Aktiv { .. }));
	assert_eq!(out.nachrichten.len(), 1);

	// Step 4: Aktiv → Gesteuert
	let out = reduce(
		out.state,
		SteuerungEvent::MesswerteEmpfangen(messwerte()),
	)
	.expect("step 4");
	assert_eq!(
		out.state,
		SteuerungState::Gesteuert {
			malo: malo(),
			nb: nb_id(),
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
}

#[test]
fn idle_plus_anmeldung() {
	let out = reduce(
		SteuerungState::Idle,
		SteuerungEvent::AnmeldungEmpfangen(anmeldung()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		SteuerungState::Angemeldet {
			malo: malo(),
			anmelder: anmelder_id(),
			nb: nb_id(),
		}
	);
	assert_eq!(out.nachrichten.len(), 1);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_konfiguration() {
	let result = reduce(
		SteuerungState::Idle,
		SteuerungEvent::KonfigurationGesendet,
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn angemeldet_cannot_receive_steuersignal() {
	let state = SteuerungState::Angemeldet {
		malo: malo(),
		anmelder: anmelder_id(),
		nb: nb_id(),
	};
	let result = reduce(
		state,
		SteuerungEvent::SteuersignalGesendet(steuersignal()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn konfiguriert_cannot_receive_messwerte() {
	let state = SteuerungState::Konfiguriert {
		malo: malo(),
		anmelder: anmelder_id(),
		nb: nb_id(),
	};
	let result = reduce(
		state,
		SteuerungEvent::MesswerteEmpfangen(messwerte()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn gesteuert_is_terminal() {
	let state = SteuerungState::Gesteuert {
		malo: malo(),
		nb: nb_id(),
	};
	let result = reduce(
		state,
		SteuerungEvent::AnmeldungEmpfangen(anmeldung()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn idle_cannot_receive_messwerte() {
	let result = reduce(
		SteuerungState::Idle,
		SteuerungEvent::MesswerteEmpfangen(messwerte()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
