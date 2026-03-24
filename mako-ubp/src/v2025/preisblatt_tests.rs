use chrono::NaiveDate;
use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{PreisPosition, PricatPreisblatt};
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::NachrichtenPayload;

use super::preisblatt::{PreisblattEvent, PreisblattState, reduce};

fn msb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}
fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}

fn pricat() -> PricatPreisblatt {
	PricatPreisblatt {
		herausgeber: msb_id(),
		gueltig_ab: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
		positionen: vec![
			PreisPosition {
				bezeichnung: "Grundpreis iMSys".to_string(),
				preis_ct: 200.0,
				einheit: "ct/Monat".to_string(),
			},
			PreisPosition {
				bezeichnung: "Messentgelt".to_string(),
				preis_ct: 50.0,
				einheit: "ct/Monat".to_string(),
			},
		],
	}
}

// --- Happy path ---

#[test]
fn idle_plus_veroeffentlichen() {
	let out = reduce(PreisblattState::Idle, PreisblattEvent::Veroeffentlichen(pricat()))
		.expect("should succeed");
	match &out.state {
		PreisblattState::Veroeffentlicht { herausgeber, positionen, .. } => {
			assert_eq!(herausgeber, &msb_id());
			assert_eq!(positionen.len(), 2);
		}
		other => panic!("expected Veroeffentlicht, got {other:?}"),
	}
	assert_eq!(out.nachrichten.len(), 1);
	assert_eq!(out.nachrichten[0].absender, msb_id());
	assert_eq!(out.nachrichten[0].empfaenger, nb_id());
	assert!(matches!(
		out.nachrichten[0].payload,
		NachrichtenPayload::PricatPreisblatt(_)
	));
}

// --- Validation ---

#[test]
fn empty_positionen_rejected() {
	let empty = PricatPreisblatt {
		herausgeber: msb_id(),
		gueltig_ab: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
		positionen: vec![],
	};
	let result = reduce(PreisblattState::Idle, PreisblattEvent::Veroeffentlichen(empty));
	assert!(matches!(result, Err(ProzessFehler::Validierungsfehler(_))));
}

// --- Invalid transitions ---

#[test]
fn veroeffentlicht_cannot_publish_again() {
	let state = PreisblattState::Veroeffentlicht {
		herausgeber: msb_id(),
		gueltig_ab: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),
		positionen: vec![PreisPosition {
			bezeichnung: "Test".to_string(),
			preis_ct: 100.0,
			einheit: "ct/Monat".to_string(),
		}],
	};
	let result = reduce(state, PreisblattEvent::Veroeffentlichen(pricat()));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
