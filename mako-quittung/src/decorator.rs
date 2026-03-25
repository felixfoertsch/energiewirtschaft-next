use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::nachricht::Nachricht;
use mako_types::reducer::ReducerOutput;

use crate::aperak::aperak_pruefen;
use crate::contrl::contrl_pruefen;
use crate::types::{DekorierterOutput, Quittung, QuittungsErgebnis, QuittungsTyp};

/// Wraps a reducer call with CONTRL and APERAK validation.
///
/// 1. CONTRL check: if negative, return immediately with state unchanged.
/// 2. APERAK check: if negative, return CONTRL-positiv + APERAK-negativ, state unchanged.
/// 3. Both positive: call reducer, return both positive receipts + reducer output.
pub fn mit_quittung<S, E, F>(
	nachricht: &Nachricht,
	state: S,
	event: E,
	stichtag: NaiveDate,
	reducer_fn: F,
) -> Result<DekorierterOutput<S>, ProzessFehler>
where
	S: Clone,
	F: FnOnce(S, E) -> Result<ReducerOutput<S>, ProzessFehler>,
{
	let absender = nachricht.absender.clone();

	// Step 1: CONTRL
	let contrl_ergebnis = contrl_pruefen(nachricht);
	if let QuittungsErgebnis::Negativ(_) = &contrl_ergebnis {
		return Ok(DekorierterOutput {
			state,
			nachrichten: Vec::new(),
			quittungen: vec![Quittung {
				an: absender,
				typ: QuittungsTyp::Contrl,
				ergebnis: contrl_ergebnis,
			}],
		});
	}

	let contrl_quittung = Quittung {
		an: absender.clone(),
		typ: QuittungsTyp::Contrl,
		ergebnis: QuittungsErgebnis::Positiv,
	};

	// Step 2: APERAK
	let aperak_ergebnis = aperak_pruefen(nachricht, stichtag);
	if let QuittungsErgebnis::Negativ(_) = &aperak_ergebnis {
		return Ok(DekorierterOutput {
			state,
			nachrichten: Vec::new(),
			quittungen: vec![
				contrl_quittung,
				Quittung {
					an: absender,
					typ: QuittungsTyp::Aperak,
					ergebnis: aperak_ergebnis,
				},
			],
		});
	}

	// Step 3: both passed, call reducer
	let output = reducer_fn(state, event)?;

	Ok(DekorierterOutput {
		state: output.state,
		nachrichten: output.nachrichten,
		quittungen: vec![
			contrl_quittung,
			Quittung {
				an: absender,
				typ: QuittungsTyp::Aperak,
				ergebnis: QuittungsErgebnis::Positiv,
			},
		],
	})
}

#[cfg(test)]
mod tests {
	use mako_types::gpke_nachrichten::UtilmdAnmeldung;
	use mako_types::ids::{MaLoId, MarktpartnerId};
	use mako_types::nachricht::{Nachricht, NachrichtenPayload};
	use mako_types::rolle::MarktRolle;

	use super::*;

	fn test_nachricht(lieferbeginn: NaiveDate) -> Nachricht {
		Nachricht {
			absender: MarktpartnerId::new("9900000000003").unwrap(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: MarktpartnerId::new("9900000000010").unwrap(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			pruef_id: None,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696788").unwrap(),
				lieferant_neu: MarktpartnerId::new("9900000000003").unwrap(),
				lieferbeginn,
			}),
		}
	}

	fn dummy_reducer(
		state: u32,
		event: &str,
	) -> Result<ReducerOutput<u32>, ProzessFehler> {
		let _ = event;
		Ok(ReducerOutput {
			state: state + 1,
			nachrichten: Vec::new(),
		})
	}

	#[test]
	fn valid_message_passes_through() {
		let stichtag = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();
		let nachricht = test_nachricht(
			NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
		);

		let result = mit_quittung(&nachricht, 0u32, "go", stichtag, dummy_reducer);
		let output = result.unwrap();

		assert_eq!(output.state, 1, "reducer should have been called");
		assert_eq!(output.quittungen.len(), 2);
		assert_eq!(output.quittungen[0].typ, QuittungsTyp::Contrl);
		assert_eq!(output.quittungen[0].ergebnis, QuittungsErgebnis::Positiv);
		assert_eq!(output.quittungen[1].typ, QuittungsTyp::Aperak);
		assert_eq!(output.quittungen[1].ergebnis, QuittungsErgebnis::Positiv);
	}

	#[test]
	fn aperak_failure_blocks_reducer() {
		let stichtag = NaiveDate::from_ymd_opt(2026, 3, 24).unwrap();
		// lieferbeginn in the past relative to stichtag
		let nachricht = test_nachricht(
			NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
		);

		let result = mit_quittung(&nachricht, 42u32, "go", stichtag, dummy_reducer);
		let output = result.unwrap();

		assert_eq!(output.state, 42, "state should be unchanged");
		assert!(output.nachrichten.is_empty(), "no process messages on failure");
		assert_eq!(output.quittungen.len(), 2);
		assert_eq!(output.quittungen[0].typ, QuittungsTyp::Contrl);
		assert_eq!(output.quittungen[0].ergebnis, QuittungsErgebnis::Positiv);
		assert_eq!(output.quittungen[1].typ, QuittungsTyp::Aperak);
		assert!(
			matches!(output.quittungen[1].ergebnis, QuittungsErgebnis::Negativ(_)),
			"APERAK should be negative"
		);
	}
}
