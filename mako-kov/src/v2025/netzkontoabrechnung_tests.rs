use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{InvoicRechnung, RechnungsTyp, RemadvZahlungsavis};
use mako_types::ids::MarktpartnerId;

use super::netzkontoabrechnung::{NetzkontoabrechnungEvent, NetzkontoabrechnungState, reduce};

fn rechnung() -> InvoicRechnung {
	InvoicRechnung {
		rechnungsnummer: "KOV-R-2025-001".to_string(),
		rechnungsdatum: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
		absender: MarktpartnerId::new("9900000000010").unwrap(),
		empfaenger: MarktpartnerId::new("9900000000003").unwrap(),
		positionen: vec![],
		gesamtbetrag_ct: 250000,
		rechnungstyp: RechnungsTyp::Netznutzung,
	}
}

fn zahlungsavis() -> RemadvZahlungsavis {
	RemadvZahlungsavis {
		referenz_rechnungsnummer: "KOV-R-2025-001".to_string(),
		zahlungsdatum: NaiveDate::from_ymd_opt(2025, 7, 15).unwrap(),
		betrag_ct: 250000,
		akzeptiert: true,
		ablehnungsgrund: None,
	}
}

// --- Happy path ---

#[test]
fn idle_plus_rechnung_transitions_to_gesendet() {
	let out = reduce(
		NetzkontoabrechnungState::Idle,
		NetzkontoabrechnungEvent::RechnungGesendet(rechnung()),
	)
	.expect("should succeed");
	assert!(matches!(out.state, NetzkontoabrechnungState::RechnungGesendet { .. }));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(
		NetzkontoabrechnungState::Idle,
		NetzkontoabrechnungEvent::RechnungGesendet(rechnung()),
	)
	.expect("step 1");

	let out = reduce(
		out.state,
		NetzkontoabrechnungEvent::ZahlungsavisEmpfangen(zahlungsavis()),
	)
	.expect("step 2");
	assert!(matches!(out.state, NetzkontoabrechnungState::ZahlungsavisEmpfangen { .. }));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_zahlungsavis() {
	let result = reduce(
		NetzkontoabrechnungState::Idle,
		NetzkontoabrechnungEvent::ZahlungsavisEmpfangen(zahlungsavis()),
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn zahlungsavis_is_terminal() {
	let state = NetzkontoabrechnungState::ZahlungsavisEmpfangen {
		rechnungsnummer: "KOV-R-2025-001".to_string(),
		akzeptiert: true,
	};
	let result = reduce(state, NetzkontoabrechnungEvent::RechnungGesendet(rechnung()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
