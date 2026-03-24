use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	InvoicRechnung, MsconsMehrMindermengen, RechnungsPosition, RechnungsTyp,
	RemadvZahlungsavis,
};
use mako_types::ids::{MaLoId, MarktpartnerId};

use super::mehrmindermengen::{MehrMindermengenEvent, MehrMindermengenState, reduce};

fn malo() -> MaLoId {
	MaLoId::new("51238696788").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}
fn lf_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}

fn liste() -> MsconsMehrMindermengen {
	MsconsMehrMindermengen {
		malo_id: malo(),
		mehrmenge_kwh: 150.0,
		mindermenge_kwh: 30.0,
		abrechnungszeitraum_von: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
		abrechnungszeitraum_bis: NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
	}
}

fn rechnung() -> InvoicRechnung {
	InvoicRechnung {
		rechnungsnummer: "RE-2025-001".to_string(),
		rechnungsdatum: NaiveDate::from_ymd_opt(2025, 7, 15).unwrap(),
		absender: nb_id(),
		empfaenger: lf_id(),
		positionen: vec![RechnungsPosition {
			bezeichnung: "Mehrmenge".to_string(),
			menge: 150.0,
			einheit: "kWh".to_string(),
			einzelpreis_ct: 5,
			betrag_ct: 750,
		}],
		gesamtbetrag_ct: 750,
		rechnungstyp: RechnungsTyp::MehrMindermengen,
	}
}

fn zahlungsavis(akzeptiert: bool) -> RemadvZahlungsavis {
	RemadvZahlungsavis {
		referenz_rechnungsnummer: "RE-2025-001".to_string(),
		zahlungsdatum: NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(),
		betrag_ct: 750,
		akzeptiert,
		ablehnungsgrund: if akzeptiert {
			None
		} else {
			Some("Betrag inkorrekt".to_string())
		},
	}
}

// --- Happy path ---

#[test]
fn idle_plus_liste_transitions_to_gesendet() {
	let out = reduce(
		MehrMindermengenState::Idle,
		MehrMindermengenEvent::ListeEmpfangen(liste()),
	)
	.expect("should succeed");
	assert!(matches!(
		out.state,
		MehrMindermengenState::ListeGesendet { .. }
	));
	assert!(out.nachrichten.is_empty());
}

#[test]
fn gesendet_plus_rechnung_transitions_to_rechnung_gesendet() {
	let state = MehrMindermengenState::ListeGesendet {
		malo: malo(),
		mehrmenge_kwh: 150.0,
		mindermenge_kwh: 30.0,
		zeitraum_von: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
		zeitraum_bis: NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
	};
	let out = reduce(state, MehrMindermengenEvent::RechnungGesendet(rechnung()))
		.expect("should succeed");
	assert_eq!(
		out.state,
		MehrMindermengenState::RechnungGesendet {
			malo: malo(),
			rechnungsnummer: "RE-2025-001".to_string(),
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn rechnung_plus_avis_transitions_to_avis_empfangen() {
	let state = MehrMindermengenState::RechnungGesendet {
		malo: malo(),
		rechnungsnummer: "RE-2025-001".to_string(),
	};
	let out = reduce(
		state,
		MehrMindermengenEvent::ZahlungsavisEmpfangen(zahlungsavis(true)),
	)
	.expect("should succeed");
	assert_eq!(
		out.state,
		MehrMindermengenState::ZahlungsavisEmpfangen {
			malo: malo(),
			rechnungsnummer: "RE-2025-001".to_string(),
			akzeptiert: true,
		}
	);
	assert!(out.nachrichten.is_empty());
}

#[test]
fn full_happy_path() {
	let out = reduce(
		MehrMindermengenState::Idle,
		MehrMindermengenEvent::ListeEmpfangen(liste()),
	)
	.expect("step 1");

	let out = reduce(out.state, MehrMindermengenEvent::RechnungGesendet(rechnung()))
		.expect("step 2");

	let out = reduce(
		out.state,
		MehrMindermengenEvent::ZahlungsavisEmpfangen(zahlungsavis(true)),
	)
	.expect("step 3");

	assert!(matches!(
		out.state,
		MehrMindermengenState::ZahlungsavisEmpfangen { akzeptiert: true, .. }
	));
}

#[test]
fn rejection_path() {
	let out = reduce(
		MehrMindermengenState::Idle,
		MehrMindermengenEvent::ListeEmpfangen(liste()),
	)
	.expect("step 1");

	let out = reduce(out.state, MehrMindermengenEvent::RechnungGesendet(rechnung()))
		.expect("step 2");

	let out = reduce(
		out.state,
		MehrMindermengenEvent::ZahlungsavisEmpfangen(zahlungsavis(false)),
	)
	.expect("step 3");

	assert!(matches!(
		out.state,
		MehrMindermengenState::ZahlungsavisEmpfangen { akzeptiert: false, .. }
	));
}

// --- Invalid transitions ---

#[test]
fn idle_cannot_receive_rechnung() {
	let result = reduce(
		MehrMindermengenState::Idle,
		MehrMindermengenEvent::RechnungGesendet(rechnung()),
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn idle_cannot_receive_avis() {
	let result = reduce(
		MehrMindermengenState::Idle,
		MehrMindermengenEvent::ZahlungsavisEmpfangen(zahlungsavis(true)),
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn liste_gesendet_cannot_receive_avis() {
	let state = MehrMindermengenState::ListeGesendet {
		malo: malo(),
		mehrmenge_kwh: 150.0,
		mindermenge_kwh: 30.0,
		zeitraum_von: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
		zeitraum_bis: NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
	};
	let result = reduce(
		state,
		MehrMindermengenEvent::ZahlungsavisEmpfangen(zahlungsavis(true)),
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}

#[test]
fn avis_empfangen_is_terminal() {
	let state = MehrMindermengenState::ZahlungsavisEmpfangen {
		malo: malo(),
		rechnungsnummer: "RE-2025-001".to_string(),
		akzeptiert: true,
	};
	let result = reduce(
		state,
		MehrMindermengenEvent::ListeEmpfangen(liste()),
	);
	assert!(matches!(
		result,
		Err(ProzessFehler::UngueltigerUebergang { .. })
	));
}
