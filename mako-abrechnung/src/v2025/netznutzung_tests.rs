use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	InvoicRechnung, RechnungsPosition, RechnungsTyp, RemadvZahlungsavis,
};
use mako_types::ids::MarktpartnerId;

use super::netznutzung::{AbrechnungEvent, AbrechnungState, reduce};

fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}
fn lf_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}
fn bkv_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}

fn netznutzung_rechnung() -> InvoicRechnung {
	InvoicRechnung {
		rechnungsnummer: "RE-NN-2025-001".to_string(),
		rechnungsdatum: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
		absender: nb_id(),
		empfaenger: lf_id(),
		positionen: vec![RechnungsPosition {
			bezeichnung: "Netznutzungsentgelt".to_string(),
			menge: 3500.0,
			einheit: "kWh".to_string(),
			einzelpreis_ct: 8,
			betrag_ct: 28000,
		}],
		gesamtbetrag_ct: 28000,
		rechnungstyp: RechnungsTyp::Netznutzung,
	}
}

fn msb_rechnung() -> InvoicRechnung {
	InvoicRechnung {
		rechnungsnummer: "RE-MSB-2025-001".to_string(),
		rechnungsdatum: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
		absender: nb_id(),
		empfaenger: lf_id(),
		positionen: vec![RechnungsPosition {
			bezeichnung: "Messstellenbetrieb".to_string(),
			menge: 12.0,
			einheit: "Monate".to_string(),
			einzelpreis_ct: 500,
			betrag_ct: 6000,
		}],
		gesamtbetrag_ct: 6000,
		rechnungstyp: RechnungsTyp::Messstellenbetrieb,
	}
}

fn ausgleichsenergie_rechnung() -> InvoicRechnung {
	InvoicRechnung {
		rechnungsnummer: "RE-AE-2025-001".to_string(),
		rechnungsdatum: NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
		absender: nb_id(),
		empfaenger: bkv_id(),
		positionen: vec![RechnungsPosition {
			bezeichnung: "Ausgleichsenergie".to_string(),
			menge: 1000.0,
			einheit: "kWh".to_string(),
			einzelpreis_ct: 12,
			betrag_ct: 12000,
		}],
		gesamtbetrag_ct: 12000,
		rechnungstyp: RechnungsTyp::Ausgleichsenergie,
	}
}

fn zahlungsavis_akzeptiert(rechnungsnummer: &str, betrag_ct: i64) -> RemadvZahlungsavis {
	RemadvZahlungsavis {
		referenz_rechnungsnummer: rechnungsnummer.to_string(),
		zahlungsdatum: NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(),
		betrag_ct,
		akzeptiert: true,
		ablehnungsgrund: None,
	}
}

fn zahlungsavis_abgelehnt(rechnungsnummer: &str) -> RemadvZahlungsavis {
	RemadvZahlungsavis {
		referenz_rechnungsnummer: rechnungsnummer.to_string(),
		zahlungsdatum: NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(),
		betrag_ct: 0,
		akzeptiert: false,
		ablehnungsgrund: Some("Rechnung fehlerhaft".to_string()),
	}
}

// --- Happy path: Netznutzung ---

#[test]
fn idle_plus_netznutzung_rechnung_transitions_to_gesendet() {
	let out = reduce(
		AbrechnungState::Idle,
		AbrechnungEvent::RechnungEmpfangen(netznutzung_rechnung()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		AbrechnungState::RechnungGesendet {
			rechnungsnummer: "RE-NN-2025-001".to_string(),
			rechnungstyp: RechnungsTyp::Netznutzung,
			gesamtbetrag_ct: 28000,
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn gesendet_plus_avis_akzeptiert_transitions_to_avis_empfangen() {
	let state = AbrechnungState::RechnungGesendet {
		rechnungsnummer: "RE-NN-2025-001".to_string(),
		rechnungstyp: RechnungsTyp::Netznutzung,
		gesamtbetrag_ct: 28000,
	};
	let out = reduce(
		state,
		AbrechnungEvent::ZahlungsavisEmpfangen(zahlungsavis_akzeptiert(
			"RE-NN-2025-001",
			28000,
		)),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		AbrechnungState::ZahlungsavisEmpfangen {
			rechnungsnummer: "RE-NN-2025-001".to_string(),
			betrag_ct: 28000,
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn gesendet_plus_avis_abgelehnt_transitions_to_abgelehnt() {
	let state = AbrechnungState::RechnungGesendet {
		rechnungsnummer: "RE-NN-2025-001".to_string(),
		rechnungstyp: RechnungsTyp::Netznutzung,
		gesamtbetrag_ct: 28000,
	};
	let out = reduce(
		state,
		AbrechnungEvent::ZahlungsavisEmpfangen(zahlungsavis_abgelehnt("RE-NN-2025-001")),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		AbrechnungState::Abgelehnt {
			rechnungsnummer: "RE-NN-2025-001".to_string(),
			grund: "Rechnung fehlerhaft".to_string(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_netznutzung_happy_path() {
	let out = reduce(
		AbrechnungState::Idle,
		AbrechnungEvent::RechnungEmpfangen(netznutzung_rechnung()),
	)
	.expect("step 1");
	assert!(matches!(
		out.state,
		AbrechnungState::RechnungGesendet { .. }
	));

	let out = reduce(
		out.state,
		AbrechnungEvent::ZahlungsavisEmpfangen(zahlungsavis_akzeptiert(
			"RE-NN-2025-001",
			28000,
		)),
	)
	.expect("step 2");
	assert!(matches!(
		out.state,
		AbrechnungState::ZahlungsavisEmpfangen { .. }
	));
}

// --- Messstellenbetrieb variant ---

#[test]
fn msb_rechnung_works() {
	let out = reduce(
		AbrechnungState::Idle,
		AbrechnungEvent::RechnungEmpfangen(msb_rechnung()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		AbrechnungState::RechnungGesendet {
			rechnungsnummer: "RE-MSB-2025-001".to_string(),
			rechnungstyp: RechnungsTyp::Messstellenbetrieb,
			gesamtbetrag_ct: 6000,
		}
	);
}

// --- Ausgleichsenergie variant ---

#[test]
fn ausgleichsenergie_rechnung_works() {
	let out = reduce(
		AbrechnungState::Idle,
		AbrechnungEvent::RechnungEmpfangen(ausgleichsenergie_rechnung()),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		AbrechnungState::RechnungGesendet {
			rechnungsnummer: "RE-AE-2025-001".to_string(),
			rechnungstyp: RechnungsTyp::Ausgleichsenergie,
			gesamtbetrag_ct: 12000,
		}
	);
}

// --- Rejection without grund ---

#[test]
fn rejection_without_grund_uses_default() {
	let state = AbrechnungState::RechnungGesendet {
		rechnungsnummer: "RE-NN-2025-001".to_string(),
		rechnungstyp: RechnungsTyp::Netznutzung,
		gesamtbetrag_ct: 28000,
	};
	let avis = RemadvZahlungsavis {
		referenz_rechnungsnummer: "RE-NN-2025-001".to_string(),
		zahlungsdatum: NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(),
		betrag_ct: 0,
		akzeptiert: false,
		ablehnungsgrund: None,
	};
	let out = reduce(state, AbrechnungEvent::ZahlungsavisEmpfangen(avis))
		.expect("should succeed");
	assert_eq!(
		out.state,
		AbrechnungState::Abgelehnt {
			rechnungsnummer: "RE-NN-2025-001".to_string(),
			grund: "Kein Grund angegeben".to_string(),
		}
	);
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_avis() {
	let result = reduce(
		AbrechnungState::Idle,
		AbrechnungEvent::ZahlungsavisEmpfangen(zahlungsavis_akzeptiert(
			"RE-NN-2025-001",
			28000,
		)),
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn avis_empfangen_is_terminal() {
	let state = AbrechnungState::ZahlungsavisEmpfangen {
		rechnungsnummer: "RE-NN-2025-001".to_string(),
		betrag_ct: 28000,
	};
	let result = reduce(
		state,
		AbrechnungEvent::RechnungEmpfangen(netznutzung_rechnung()),
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn abgelehnt_is_terminal() {
	let state = AbrechnungState::Abgelehnt {
		rechnungsnummer: "RE-NN-2025-001".to_string(),
		grund: "Rechnung fehlerhaft".to_string(),
	};
	let result = reduce(
		state,
		AbrechnungEvent::RechnungEmpfangen(netznutzung_rechnung()),
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}
