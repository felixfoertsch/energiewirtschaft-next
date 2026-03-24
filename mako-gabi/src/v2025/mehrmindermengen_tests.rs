use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	InvoicRechnung, MsconsMehrMindermengen, RechnungsTyp, RemadvZahlungsavis,
};
use mako_types::ids::{MaLoId, MarktpartnerId};

use super::mehrmindermengen::{MehrMindermengenEvent, MehrMindermengenState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}

fn liste() -> MsconsMehrMindermengen {
	MsconsMehrMindermengen {
		malo_id: malo(),
		mehrmenge_kwh: 150.0,
		mindermenge_kwh: 50.0,
		abrechnungszeitraum_von: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
		abrechnungszeitraum_bis: NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
	}
}

fn rechnung() -> InvoicRechnung {
	InvoicRechnung {
		rechnungsnummer: "GAS-R-2025-001".to_string(),
		rechnungsdatum: NaiveDate::from_ymd_opt(2025, 7, 15).unwrap(),
		absender: MarktpartnerId::new("9900000000010").unwrap(),
		empfaenger: MarktpartnerId::new("9900000000003").unwrap(),
		positionen: vec![],
		gesamtbetrag_ct: 5000,
		rechnungstyp: RechnungsTyp::MehrMindermengen,
	}
}

fn zahlungsavis() -> RemadvZahlungsavis {
	RemadvZahlungsavis {
		referenz_rechnungsnummer: "GAS-R-2025-001".to_string(),
		zahlungsdatum: NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(),
		betrag_ct: 5000,
		akzeptiert: true,
		ablehnungsgrund: None,
	}
}

// --- Happy path ---

#[test]
fn idle_plus_liste_transitions_to_gesendet() {
	let out = reduce(MehrMindermengenState::Idle, MehrMindermengenEvent::ListeEmpfangen(liste()))
		.expect("should succeed");
	assert!(matches!(out.state, MehrMindermengenState::ListeGesendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(MehrMindermengenState::Idle, MehrMindermengenEvent::ListeEmpfangen(liste()))
		.expect("step 1");
	let out = reduce(out.state, MehrMindermengenEvent::RechnungGesendet(rechnung()))
		.expect("step 2");
	assert!(matches!(out.state, MehrMindermengenState::RechnungGesendet { .. }));

	let out = reduce(out.state, MehrMindermengenEvent::ZahlungsavisEmpfangen(zahlungsavis()))
		.expect("step 3");
	assert!(matches!(out.state, MehrMindermengenState::ZahlungsavisEmpfangen { .. }));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_rechnung() {
	let result = reduce(
		MehrMindermengenState::Idle,
		MehrMindermengenEvent::RechnungGesendet(rechnung()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn zahlungsavis_is_terminal() {
	let state = MehrMindermengenState::ZahlungsavisEmpfangen {
		malo: malo(),
		rechnungsnummer: "GAS-R-2025-001".to_string(),
		akzeptiert: true,
	};
	let result = reduce(state, MehrMindermengenEvent::ListeEmpfangen(liste()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
