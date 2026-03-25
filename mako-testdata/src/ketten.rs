//! Kommunikationsketten: end-to-end communication chain tests.
//!
//! Each Kette represents a sequence of EDIFACT messages in a business process.
//! The runner parses each message, verifies payload expectations, and roundtrips
//! through serialize -> re-parse to ensure codec consistency.

use mako_codec::edifact::dispatch::{parse_nachricht, serialize_nachricht};
use mako_types::nachricht::Nachricht;

/// A single step in a communication chain.
pub struct KettenSchritt {
	/// The EDIFACT string for this step.
	pub edifact: String,
	/// Human-readable description of the step.
	pub beschreibung: &'static str,
	/// Predicate that validates the parsed Nachricht.
	pub payload_pruefer: fn(&Nachricht) -> bool,
}

/// A full communication chain: a named sequence of steps.
pub struct Kette {
	pub name: &'static str,
	pub schritte: Vec<KettenSchritt>,
}

/// Run a chain: parse each EDIFACT string, verify payload, roundtrip outgoing.
pub fn pruefe_kette(kette: &Kette) {
	for (i, schritt) in kette.schritte.iter().enumerate() {
		// 1. Parse EDIFACT
		let parsed = parse_nachricht(&schritt.edifact).unwrap_or_else(|e| {
			panic!(
				"Kette '{}' Schritt {}: parse failed: {e}",
				kette.name, i
			)
		});

		// 2. Verify payload
		assert!(
			(schritt.payload_pruefer)(&parsed),
			"Kette '{}' Schritt {} ({}): payload check failed",
			kette.name,
			i,
			schritt.beschreibung
		);

		// 3. Roundtrip: serialize -> re-parse -> compare
		let serialized = serialize_nachricht(&parsed);
		let reparsed = parse_nachricht(&serialized).unwrap_or_else(|e| {
			panic!(
				"Kette '{}' Schritt {}: roundtrip parse failed: {e}",
				kette.name, i
			)
		});
		assert_eq!(
			parsed, reparsed,
			"Kette '{}' Schritt {} ({}): roundtrip mismatch",
			kette.name, i, schritt.beschreibung
		);
	}
}

// ===========================================================================
// GPKE Chain builders
// ===========================================================================

use chrono::NaiveDate;

use crate::generator::edifact::*;
use crate::generator::params::*;
use crate::ids::{test_malo, test_mp_id};
use mako_types::nachricht::NachrichtenPayload;

/// Chain 1: GPKE LFW Happy Path
///
/// 5 EDIFACT steps (steps 4 and 5 from the spec are non-EDIFACT):
/// 1. LFN -> NB: Anmeldung (UTILMD E01, PID 44001)
/// 2. NB -> LFN: Bestaetigung (UTILMD, PID 44002)
/// 3. NB -> LFA: Abmeldung (UTILMD E02, PID 44004)
/// (4. LFA -> NB: implicit accept -- no EDIFACT)
/// (5. Widerspruchsfrist abgelaufen -- internal, no EDIFACT)
/// 6. NB -> LFN: Zuordnung (UTILMD E06, PID 44005)
/// 7. NB -> LFA: Zuordnung (UTILMD E06, PID 44006)
pub fn gpke_lfw_happy_path() -> Kette {
	let malo = test_malo(0);
	let lfn = test_mp_id(0);
	let nb = test_mp_id(1);
	let lfa = test_mp_id(2);
	let lieferbeginn = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();

	Kette {
		name: "GPKE LFW Happy Path",
		schritte: vec![
			// 1. LFN -> NB: Anmeldung
			KettenSchritt {
				edifact: erzeuge_utilmd_anmeldung(&AnmeldungParams {
					sender: lfn.clone(),
					empfaenger: nb.clone(),
					malo_id: malo.clone(),
					lieferbeginn,
				}),
				beschreibung: "1. LFN -> NB: Anmeldung (PID 44001)",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdAnmeldung(_)),
			},
			// 2. NB -> LFN: Bestaetigung
			KettenSchritt {
				edifact: erzeuge_utilmd_bestaetigung(&BestaetigungParams {
					sender: nb.clone(),
					empfaenger: lfn.clone(),
					malo_id: malo.clone(),
					lieferbeginn,
				}),
				beschreibung: "2. NB -> LFN: Bestaetigung (PID 44002)",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::UtilmdBestaetigung(_))
				},
			},
			// 3. NB -> LFA: Abmeldung
			KettenSchritt {
				edifact: erzeuge_utilmd_abmeldung(&AbmeldungParams {
					sender: nb.clone(),
					empfaenger: lfa.clone(),
					malo_id: malo.clone(),
					lieferende: lieferbeginn,
				}),
				beschreibung: "3. NB -> LFA: Abmeldung (PID 44004)",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdAbmeldung(_)),
			},
			// Steps 4+5 are non-EDIFACT (implicit accept, Widerspruchsfrist)
			// 6. NB -> LFN: Zuordnung
			KettenSchritt {
				edifact: erzeuge_utilmd_zuordnung(&ZuordnungParams {
					sender: nb.clone(),
					empfaenger: lfn.clone(),
					malo_id: malo.clone(),
					lieferbeginn,
				}),
				beschreibung: "6. NB -> LFN: Zuordnung (PID 44005)",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdZuordnung(_)),
			},
			// 7. NB -> LFA: Zuordnung
			KettenSchritt {
				edifact: erzeuge_utilmd_zuordnung(&ZuordnungParams {
					sender: nb.clone(),
					empfaenger: lfa.clone(),
					malo_id: malo.clone(),
					lieferbeginn,
				}),
				beschreibung: "7. NB -> LFA: Zuordnung (PID 44006)",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdZuordnung(_)),
			},
		],
	}
}

/// Chain 2: GPKE LFW Ablehnung durch LFA
///
/// 1. LFN -> NB: Anmeldung
/// 2. NB -> LFN: Bestaetigung
/// 3. NB -> LFA: Abmeldung
/// 4. LFA -> NB: Ablehnung (UTILMD, rejection with AblehnungsGrund)
pub fn gpke_lfw_ablehnung() -> Kette {
	let malo = test_malo(0);
	let lfn = test_mp_id(0);
	let nb = test_mp_id(1);
	let lfa = test_mp_id(2);
	let lieferbeginn = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();

	Kette {
		name: "GPKE LFW Ablehnung durch LFA",
		schritte: vec![
			KettenSchritt {
				edifact: erzeuge_utilmd_anmeldung(&AnmeldungParams {
					sender: lfn.clone(),
					empfaenger: nb.clone(),
					malo_id: malo.clone(),
					lieferbeginn,
				}),
				beschreibung: "1. LFN -> NB: Anmeldung",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdAnmeldung(_)),
			},
			KettenSchritt {
				edifact: erzeuge_utilmd_bestaetigung(&BestaetigungParams {
					sender: nb.clone(),
					empfaenger: lfn.clone(),
					malo_id: malo.clone(),
					lieferbeginn,
				}),
				beschreibung: "2. NB -> LFN: Bestaetigung",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::UtilmdBestaetigung(_))
				},
			},
			KettenSchritt {
				edifact: erzeuge_utilmd_abmeldung(&AbmeldungParams {
					sender: nb.clone(),
					empfaenger: lfa.clone(),
					malo_id: malo.clone(),
					lieferende: lieferbeginn,
				}),
				beschreibung: "3. NB -> LFA: Abmeldung",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdAbmeldung(_)),
			},
			KettenSchritt {
				edifact: erzeuge_utilmd_ablehnung(&AblehnungParams {
					sender: lfa.clone(),
					empfaenger: nb.clone(),
					malo_id: malo.clone(),
				}),
				beschreibung: "4. LFA -> NB: Ablehnung",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdAblehnung(_)),
			},
		],
	}
}

/// Chain 3: GPKE LFW Fristueberschreitung
///
/// 1. LFN -> NB: Anmeldung
/// (Frist ueberschritten -- no response, internal event only)
pub fn gpke_lfw_fristueberschreitung() -> Kette {
	let malo = test_malo(0);
	let lfn = test_mp_id(0);
	let nb = test_mp_id(1);
	let lieferbeginn = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();

	Kette {
		name: "GPKE LFW Fristueberschreitung",
		schritte: vec![KettenSchritt {
			edifact: erzeuge_utilmd_anmeldung(&AnmeldungParams {
				sender: lfn.clone(),
				empfaenger: nb.clone(),
				malo_id: malo.clone(),
				lieferbeginn,
			}),
			beschreibung: "1. LFN -> NB: Anmeldung (danach Fristueberschreitung, kein EDIFACT)",
			payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdAnmeldung(_)),
		}],
	}
}

/// Chain 4: GPKE Lieferende
///
/// 1. LF -> NB: Lieferende-Abmeldung (UTILMD E02)
/// 2. NB -> LF: Lieferende-Bestaetigung (UTILMD E01)
/// 3. MSB -> NB: Schlussturnusmesswert (MSCONS)
pub fn gpke_lieferende() -> Kette {
	let malo = test_malo(0);
	let lf = test_mp_id(3);
	let nb = test_mp_id(1);
	let msb = test_mp_id(4);
	let lieferende = NaiveDate::from_ymd_opt(2026, 9, 30).unwrap();

	Kette {
		name: "GPKE Lieferende",
		schritte: vec![
			KettenSchritt {
				edifact: erzeuge_utilmd_lieferende_abmeldung(&LieferendeAbmeldungParams {
					sender: lf.clone(),
					empfaenger: nb.clone(),
					malo_id: malo.clone(),
					lieferende,
				}),
				beschreibung: "1. LF -> NB: Lieferende-Abmeldung",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::UtilmdLieferendeAbmeldung(_))
				},
			},
			KettenSchritt {
				edifact: erzeuge_utilmd_lieferende_bestaetigung(&LieferendeBestaetigungParams {
					sender: nb.clone(),
					empfaenger: lf.clone(),
					malo_id: malo.clone(),
					lieferende,
				}),
				beschreibung: "2. NB -> LF: Lieferende-Bestaetigung",
				payload_pruefer: |n| {
					matches!(
						n.payload,
						NachrichtenPayload::UtilmdLieferendeBestaetigung(_)
					)
				},
			},
			KettenSchritt {
				edifact: erzeuge_mscons_schlussturnusmesswert(&SchlussturnusmesswertParams {
					sender: msb.clone(),
					empfaenger: nb.clone(),
					malo_id: malo.clone(),
					stichtag: lieferende,
					zaehlerstand: 12345.6,
					einheit: "kWh".to_string(),
				}),
				beschreibung: "3. MSB -> NB: Schlussturnusmesswert",
				payload_pruefer: |n| {
					matches!(
						n.payload,
						NachrichtenPayload::MsconsSchlussturnusmesswert(_)
					)
				},
			},
		],
	}
}

/// Chain 5: GPKE Stammdatenaenderung
///
/// 1. NB -> LF: Stammdatenaenderung (UTILMD E03, PID 44112)
/// (2. Bestaetigung via APERAK -- not a Nachricht step)
pub fn gpke_stammdatenaenderung() -> Kette {
	let malo = test_malo(0);
	let nb = test_mp_id(1);
	let lf = test_mp_id(3);

	Kette {
		name: "GPKE Stammdatenaenderung",
		schritte: vec![KettenSchritt {
			edifact: erzeuge_utilmd_stammdatenaenderung(&StammdatenaenderungParams {
				sender: nb.clone(),
				empfaenger: lf.clone(),
				malo_id: malo.clone(),
				felder: vec![
					("Spannungsebene".to_string(), "Niederspannung".to_string()),
					("Netzgebiet".to_string(), "Berlin".to_string()),
				],
			}),
			beschreibung: "1. NB -> LF: Stammdatenaenderung (PID 44112, danach APERAK-Bestaetigung)",
			payload_pruefer: |n| {
				matches!(n.payload, NachrichtenPayload::UtilmdStammdatenaenderung(_))
			},
		}],
	}
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn kette_gpke_lfw_happy_path() {
		pruefe_kette(&gpke_lfw_happy_path());
	}

	#[test]
	fn kette_gpke_lfw_ablehnung() {
		pruefe_kette(&gpke_lfw_ablehnung());
	}

	#[test]
	fn kette_gpke_lfw_fristueberschreitung() {
		pruefe_kette(&gpke_lfw_fristueberschreitung());
	}

	#[test]
	fn kette_gpke_lieferende() {
		pruefe_kette(&gpke_lieferende());
	}

	#[test]
	fn kette_gpke_stammdatenaenderung() {
		pruefe_kette(&gpke_stammdatenaenderung());
	}

	/// Verify that the LFW reducer produces correct state transitions
	/// using the same data as the happy path chain.
	#[test]
	fn gpke_lfw_reducer_matches_chain() {
		use mako_gpke::v2025::lfw::{reduce, LfwEvent, LfwState};
		use mako_types::gpke_nachrichten::AblehnungsGrund;

		let kette = gpke_lfw_happy_path();

		// Step 1: parse Anmeldung, feed to reducer
		let anmeldung_parsed = parse_nachricht(&kette.schritte[0].edifact).unwrap();
		if let NachrichtenPayload::UtilmdAnmeldung(anm) = anmeldung_parsed.payload {
			let output = reduce(LfwState::Idle, LfwEvent::AnmeldungEmpfangen(anm)).unwrap();
			assert!(matches!(
				output.state,
				LfwState::AnmeldungEingegangen { .. }
			));

			// Step 2: AnmeldungBestaetigt with LFA
			let lfa = test_mp_id(2);
			let output =
				reduce(output.state, LfwEvent::AnmeldungBestaetigt { lfa }).unwrap();
			assert!(matches!(
				output.state,
				LfwState::AbmeldungAnLfaGesendet { .. }
			));
			// Reducer should produce 2 outgoing messages (Bestaetigung + Abmeldung)
			assert_eq!(output.nachrichten.len(), 2);
			// Verify payload types of the outgoing messages
			assert!(matches!(
				output.nachrichten[0].payload,
				NachrichtenPayload::UtilmdBestaetigung(_)
			));
			assert!(matches!(
				output.nachrichten[1].payload,
				NachrichtenPayload::UtilmdAbmeldung(_)
			));

			// Step 3: LFA confirms (implicit)
			let output = reduce(output.state, LfwEvent::LfaHatBestaetigt).unwrap();
			assert!(matches!(
				output.state,
				LfwState::WiderspruchsfristLaeuft { .. }
			));

			// Step 4: Widerspruchsfrist expires
			let output =
				reduce(output.state, LfwEvent::WiderspruchsfristAbgelaufen).unwrap();
			assert!(matches!(output.state, LfwState::Zugeordnet { .. }));
			// Reducer should produce 2 Zuordnung messages
			assert_eq!(output.nachrichten.len(), 2);
			for nachricht in &output.nachrichten {
				assert!(matches!(
					nachricht.payload,
					NachrichtenPayload::UtilmdZuordnung(_)
				));
			}
		} else {
			panic!("expected UtilmdAnmeldung payload");
		}

		// Separately: test Fristueberschreitung path using the same Anmeldung
		let anmeldung_parsed = parse_nachricht(&kette.schritte[0].edifact).unwrap();
		if let NachrichtenPayload::UtilmdAnmeldung(anm) = anmeldung_parsed.payload {
			let output = reduce(LfwState::Idle, LfwEvent::AnmeldungEmpfangen(anm)).unwrap();
			let output = reduce(output.state, LfwEvent::FristUeberschritten).unwrap();
			assert!(matches!(
				output.state,
				LfwState::Abgelehnt {
					grund: AblehnungsGrund::Fristverletzung,
					..
				}
			));
		} else {
			panic!("expected UtilmdAnmeldung payload");
		}
	}

	/// Verify that the Lieferende reducer produces correct state transitions
	/// using data consistent with chain 4.
	#[test]
	fn gpke_lieferende_reducer_matches_chain() {
		use mako_gpke::v2025::lieferende::{reduce, LieferendeEvent, LieferendeState};

		let kette = gpke_lieferende();

		// Step 1: parse Lieferende-Abmeldung
		let abmeldung_parsed = parse_nachricht(&kette.schritte[0].edifact).unwrap();
		if let NachrichtenPayload::UtilmdLieferendeAbmeldung(abm) = abmeldung_parsed.payload {
			let output = reduce(
				LieferendeState::Idle,
				LieferendeEvent::AbmeldungEingegangen(abm),
			)
			.unwrap();
			assert!(matches!(
				output.state,
				LieferendeState::AbmeldungGesendet { .. }
			));

			// Step 2: parse Lieferende-Bestaetigung, feed to reducer
			let bestaetigung_parsed = parse_nachricht(&kette.schritte[1].edifact).unwrap();
			if let NachrichtenPayload::UtilmdLieferendeBestaetigung(best) =
				bestaetigung_parsed.payload
			{
				let output = reduce(
					output.state,
					LieferendeEvent::AbmeldungBestaetigt(best),
				)
				.unwrap();
				assert!(matches!(
					output.state,
					LieferendeState::Bestaetigt { .. }
				));

				// Step 3: parse Schlussturnusmesswert
				let messwert_parsed = parse_nachricht(&kette.schritte[2].edifact).unwrap();
				if let NachrichtenPayload::MsconsSchlussturnusmesswert(mw) =
					messwert_parsed.payload
				{
					let output = reduce(
						output.state,
						LieferendeEvent::SchlussturnusmesswertEmpfangen(mw),
					)
					.unwrap();
					assert!(matches!(
						output.state,
						LieferendeState::Abgeschlossen { .. }
					));
				} else {
					panic!("expected MsconsSchlussturnusmesswert payload");
				}
			} else {
				panic!("expected UtilmdLieferendeBestaetigung payload");
			}
		} else {
			panic!("expected UtilmdLieferendeAbmeldung payload");
		}
	}

	/// Verify that the Stammdaten reducer produces correct state transitions
	/// using data consistent with chain 5.
	#[test]
	fn gpke_stammdaten_reducer_matches_chain() {
		use mako_gpke::v2025::stammdaten::{reduce, StammdatenEvent, StammdatenState};

		let kette = gpke_stammdatenaenderung();

		let parsed = parse_nachricht(&kette.schritte[0].edifact).unwrap();
		if let NachrichtenPayload::UtilmdStammdatenaenderung(sd) = parsed.payload {
			let output = reduce(
				StammdatenState::Idle,
				StammdatenEvent::AenderungEingegangen(sd),
			)
			.unwrap();
			assert!(matches!(
				output.state,
				StammdatenState::AenderungGesendet { .. }
			));

			// Simulate APERAK confirmation
			let output =
				reduce(output.state, StammdatenEvent::AenderungBestaetigt).unwrap();
			assert!(matches!(
				output.state,
				StammdatenState::AenderungBestaetigt { .. }
			));
		} else {
			panic!("expected UtilmdStammdatenaenderung payload");
		}
	}
}
