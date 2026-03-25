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
use crate::ids::{test_malo, test_melo, test_mp_id};
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
// WiM chains
// ===========================================================================

/// Chain 6: WiM MSB-Wechsel (3 EDIFACT steps)
///
/// 1. MSB_neu -> NB: MsbWechselAnmeldung (UTILMD E03)
/// 2. NB -> MSB_alt: Abmeldung notification (reuse UtilmdAbmeldung)
/// 3. NB -> MSB_neu: Bestaetigung
pub fn wim_msb_wechsel() -> Kette {
	let msb_neu = test_mp_id(3);
	let msb_alt = test_mp_id(4);
	let nb = test_mp_id(1);
	let melo = test_melo(0);
	let wechseldatum = NaiveDate::from_ymd_opt(2026, 8, 1).unwrap();

	Kette {
		name: "WiM MSB-Wechsel",
		schritte: vec![
			KettenSchritt {
				edifact: erzeuge_utilmd_msb_wechsel_anmeldung(&MsbWechselAnmeldungParams {
					sender: msb_neu.clone(),
					empfaenger: nb.clone(),
					melo_id: melo.clone(),
					wechseldatum,
				}),
				beschreibung: "1. MSB_neu -> NB: MsbWechselAnmeldung",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::UtilmdMsbWechselAnmeldung(_))
				},
			},
			KettenSchritt {
				edifact: erzeuge_utilmd_abmeldung(&AbmeldungParams {
					sender: nb.clone(),
					empfaenger: msb_alt.clone(),
					malo_id: test_malo(0),
					lieferende: wechseldatum,
				}),
				beschreibung: "2. NB -> MSB_alt: Abmeldung",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdAbmeldung(_)),
			},
			KettenSchritt {
				edifact: erzeuge_utilmd_bestaetigung(&BestaetigungParams {
					sender: nb.clone(),
					empfaenger: msb_neu.clone(),
					malo_id: test_malo(0),
					lieferbeginn: wechseldatum,
				}),
				beschreibung: "3. NB -> MSB_neu: Bestaetigung",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::UtilmdBestaetigung(_))
				},
			},
		],
	}
}

/// Chain 7: WiM Zählwertübermittlung (2 EDIFACT steps)
///
/// 1. MSB -> NB: Lastgang (MSCONS)
/// 2. NB -> LF: Lastgang forwarded (MSCONS)
pub fn wim_zaehlwertübermittlung() -> Kette {
	let msb = test_mp_id(3);
	let nb = test_mp_id(1);
	let lf = test_mp_id(0);
	let malo = test_malo(0);

	Kette {
		name: "WiM Zaehlwertuebermittlung",
		schritte: vec![
			KettenSchritt {
				edifact: erzeuge_mscons_lastgang(&LastgangParams {
					sender: msb.clone(),
					empfaenger: nb.clone(),
					malo_id: malo.clone(),
					werte: vec![
						("20260701000000".to_string(), "1.5".to_string(), "kWh".to_string()),
						("20260701001500".to_string(), "2.3".to_string(), "kWh".to_string()),
					],
				}),
				beschreibung: "1. MSB -> NB: Lastgang",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::MsconsLastgang(_)),
			},
			KettenSchritt {
				edifact: erzeuge_mscons_lastgang(&LastgangParams {
					sender: nb.clone(),
					empfaenger: lf.clone(),
					malo_id: malo.clone(),
					werte: vec![
						("20260701000000".to_string(), "1.5".to_string(), "kWh".to_string()),
						("20260701001500".to_string(), "2.3".to_string(), "kWh".to_string()),
					],
				}),
				beschreibung: "2. NB -> LF: Lastgang forwarded",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::MsconsLastgang(_)),
			},
		],
	}
}

// ===========================================================================
// UBP chains
// ===========================================================================

/// Chain 8: UBP Bestellung (4 EDIFACT steps)
///
/// 1. LF -> MSB: Angebotsanfrage (REQOTE)
/// 2. MSB -> LF: Angebot (QUOTES)
/// 3. LF -> MSB: Bestellung (ORDERS)
/// 4. MSB -> LF: Bestellantwort (ORDRSP, accepted)
pub fn ubp_bestellung() -> Kette {
	let lf = test_mp_id(0);
	let msb = test_mp_id(3);
	let melo = test_melo(0);

	Kette {
		name: "UBP Bestellung",
		schritte: vec![
			KettenSchritt {
				edifact: erzeuge_reqote_angebotsanfrage(&AngebotsanfrageParams {
					sender: lf.clone(),
					empfaenger: msb.clone(),
					melo_id: melo.clone(),
					produkt: "Intelligentes Messsystem".to_string(),
				}),
				beschreibung: "1. LF -> MSB: Angebotsanfrage",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::ReqoteAngebotsanfrage(_))
				},
			},
			KettenSchritt {
				edifact: erzeuge_quotes_angebot(&AngebotParams {
					sender: msb.clone(),
					empfaenger: lf.clone(),
					melo_id: melo.clone(),
					produkt: "Intelligentes Messsystem".to_string(),
					preis: "1250".to_string(),
				}),
				beschreibung: "2. MSB -> LF: Angebot",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::QuotesAngebot(_)),
			},
			KettenSchritt {
				edifact: erzeuge_orders_bestellung(&BestellungParams {
					sender: lf.clone(),
					empfaenger: msb.clone(),
					melo_id: melo.clone(),
					referenz_angebot: "ANG-2026-001".to_string(),
				}),
				beschreibung: "3. LF -> MSB: Bestellung",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::OrdersBestellung(_))
				},
			},
			KettenSchritt {
				edifact: erzeuge_ordrsp_bestellantwort(&BestellantwortParams {
					sender: msb.clone(),
					empfaenger: lf.clone(),
					melo_id: melo.clone(),
					status_code: "Z07".to_string(),
					grund: None,
				}),
				beschreibung: "4. MSB -> LF: Bestellantwort (accepted)",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::OrdrspBestellantwort(_))
				},
			},
		],
	}
}

// ===========================================================================
// MaBiS chains
// ===========================================================================

/// Chain 9: MaBiS Bilanzkreiszuordnung (2 EDIFACT steps)
///
/// 1. LF -> NB: Bilanzkreiszuordnung (UTILMD E01)
/// 2. NB -> LF: Bestaetigung
pub fn mabis_bilanzkreiszuordnung() -> Kette {
	let lf = test_mp_id(0);
	let nb = test_mp_id(1);
	let malo = test_malo(0);
	let gueltig_ab = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();

	Kette {
		name: "MaBiS Bilanzkreiszuordnung",
		schritte: vec![
			KettenSchritt {
				edifact: erzeuge_utilmd_bilanzkreiszuordnung(&BilanzkreiszuordnungParams {
					sender: lf.clone(),
					empfaenger: nb.clone(),
					malo_id: malo.clone(),
					bilanzkreis: "11XDE-BKTEST-X".to_string(),
					gueltig_ab,
				}),
				beschreibung: "1. LF -> NB: Bilanzkreiszuordnung",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::UtilmdBilanzkreiszuordnung(_))
				},
			},
			KettenSchritt {
				edifact: erzeuge_utilmd_bestaetigung(&BestaetigungParams {
					sender: nb.clone(),
					empfaenger: lf.clone(),
					malo_id: malo.clone(),
					lieferbeginn: gueltig_ab,
				}),
				beschreibung: "2. NB -> LF: Bestaetigung",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::UtilmdBestaetigung(_))
				},
			},
		],
	}
}

// ===========================================================================
// Abrechnung chains
// ===========================================================================

/// Chain 10: Abrechnung Netznutzung (2 EDIFACT steps)
///
/// 1. NB -> LF: Rechnung (INVOIC)
/// 2. LF -> NB: Zahlungsavis positiv (REMADV)
pub fn abrechnung_netznutzung() -> Kette {
	let nb = test_mp_id(1);
	let lf = test_mp_id(0);

	Kette {
		name: "Abrechnung Netznutzung",
		schritte: vec![
			KettenSchritt {
				edifact: erzeuge_invoic_rechnung(&RechnungParams {
					sender: nb.clone(),
					empfaenger: lf.clone(),
					rechnungsnummer: "RG-2026-0001".to_string(),
					rechnungsdatum: NaiveDate::from_ymd_opt(2026, 3, 15).unwrap(),
					positionen: vec![(
						"Netznutzung Q1".to_string(),
						"5000".to_string(),
						"kWh".to_string(),
						"250".to_string(),
						"12500".to_string(),
					)],
					gesamtbetrag: "12500".to_string(),
				}),
				beschreibung: "1. NB -> LF: Rechnung",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::InvoicRechnung(_)),
			},
			KettenSchritt {
				edifact: erzeuge_remadv_zahlungsavis(&ZahlungsavisParams {
					sender: lf.clone(),
					empfaenger: nb.clone(),
					referenz_rechnungsnummer: "RG-2026-0001".to_string(),
					zahlungsdatum: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
					betrag: "12500".to_string(),
					status_code: "Z06".to_string(),
				}),
				beschreibung: "2. LF -> NB: Zahlungsavis positiv",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::RemadvZahlungsavis(_))
				},
			},
		],
	}
}

// ===========================================================================
// RD 2.0 placeholder
// ===========================================================================

/// Chain 11: RD 2.0 Abruf — XML-based, implemented in Task 13.
pub fn rd2_abruf() -> Kette {
	// XML-based, implemented in Task 13
	Kette {
		name: "RD 2.0 Abruf",
		schritte: vec![],
	}
}

// ===========================================================================
// §14a chains
// ===========================================================================

/// Chain 12: §14a Steuerung (2 EDIFACT steps)
///
/// 1. NB -> MSB: SteuerbareVerbrauchseinrichtung (UTILMD E01)
/// 2. NB -> MSB: Steuersignal (UTILMD E04 / CLS)
pub fn steuerung_14a() -> Kette {
	let nb = test_mp_id(1);
	let msb = test_mp_id(3);
	let malo = test_malo(0);

	Kette {
		name: "§14a Steuerung",
		schritte: vec![
			KettenSchritt {
				edifact: erzeuge_utilmd_steuerbare_verbrauchseinrichtung(
					&SteuerbareVerbrauchseinrichtungParams {
						sender: nb.clone(),
						empfaenger: msb.clone(),
						malo_id: malo.clone(),
						geraetetyp: "Wallbox".to_string(),
						max_leistung_kw: "11".to_string(),
					},
				),
				beschreibung: "1. NB -> MSB: SteuerbareVerbrauchseinrichtung",
				payload_pruefer: |n| {
					matches!(
						n.payload,
						NachrichtenPayload::UtilmdSteuerbareVerbrauchseinrichtung(_)
					)
				},
			},
			KettenSchritt {
				edifact: erzeuge_utilmd_cls_steuersignal(&ClsSteuersignalParams {
					sender: nb.clone(),
					empfaenger: msb.clone(),
					malo_id: malo.clone(),
					steuerung_code: "Z08".to_string(),
					zeitpunkt: "20260701140000".to_string(),
				}),
				beschreibung: "2. NB -> MSB: Steuersignal",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::ClsSteuersignal(_)),
			},
		],
	}
}

// ===========================================================================
// GeLi Gas chains
// ===========================================================================

/// Chain 13: GeLi Gas LFW (5 EDIFACT steps)
///
/// Same EDIFACT structure as GPKE LFW, with Gas roles (LFN, LFA, NB).
/// Steps 4+5 from the spec are non-EDIFACT (Widerspruchsfrist).
/// 1. LFN -> NB: Anmeldung
/// 2. NB -> LFN: Bestaetigung
/// 3. NB -> LFA: Abmeldung
/// 4. NB -> LFN: Zuordnung
/// 5. NB -> LFA: Zuordnung
pub fn geli_gas_lfw() -> Kette {
	let malo = test_malo(0);
	let lfn = test_mp_id(0);
	let nb = test_mp_id(1);
	let lfa = test_mp_id(2);
	let lieferbeginn = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();

	Kette {
		name: "GeLi Gas LFW",
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
				edifact: erzeuge_utilmd_zuordnung(&ZuordnungParams {
					sender: nb.clone(),
					empfaenger: lfn.clone(),
					malo_id: malo.clone(),
					lieferbeginn,
				}),
				beschreibung: "4. NB -> LFN: Zuordnung",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdZuordnung(_)),
			},
			KettenSchritt {
				edifact: erzeuge_utilmd_zuordnung(&ZuordnungParams {
					sender: nb.clone(),
					empfaenger: lfa.clone(),
					malo_id: malo.clone(),
					lieferbeginn,
				}),
				beschreibung: "5. NB -> LFA: Zuordnung",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::UtilmdZuordnung(_)),
			},
		],
	}
}

// ===========================================================================
// GABi Gas chains
// ===========================================================================

/// Chain 14: GABi Gas Nominierung (3 EDIFACT steps)
///
/// 1. BKV -> MGV: Nominierung (MSCONS)
/// 2. MGV -> BKV: NominierungBestaetigung (MSCONS)
/// 3. BKV -> MGV: Renominierung (MSCONS)
pub fn gabi_gas_nominierung() -> Kette {
	let bkv = test_mp_id(4);
	let mgv = test_mp_id(5);
	let bilanzkreis = "11XDE-GASTEST-Y".to_string();

	Kette {
		name: "GABi Gas Nominierung",
		schritte: vec![
			KettenSchritt {
				edifact: erzeuge_mscons_nominierung(&NominierungParams {
					sender: bkv.clone(),
					empfaenger: mgv.clone(),
					bilanzkreis: bilanzkreis.clone(),
					werte: vec![
						("20260701060000".to_string(), "500".to_string(), "kWh".to_string()),
						("20260701070000".to_string(), "450".to_string(), "kWh".to_string()),
					],
				}),
				beschreibung: "1. BKV -> MGV: Nominierung",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::Nominierung(_)),
			},
			KettenSchritt {
				edifact: erzeuge_mscons_nominierung_bestaetigung(&NominierungBestaetigungParams {
					sender: mgv.clone(),
					empfaenger: bkv.clone(),
					bilanzkreis: bilanzkreis.clone(),
					status_code: "Z06".to_string(),
				}),
				beschreibung: "2. MGV -> BKV: NominierungBestaetigung",
				payload_pruefer: |n| {
					matches!(n.payload, NachrichtenPayload::NominierungBestaetigung(_))
				},
			},
			KettenSchritt {
				edifact: erzeuge_mscons_renominierung(&RenominierungParams {
					sender: bkv.clone(),
					empfaenger: mgv.clone(),
					bilanzkreis: bilanzkreis.clone(),
					werte: vec![(
						"20260701060000".to_string(),
						"520".to_string(),
						"kWh".to_string(),
					)],
				}),
				beschreibung: "3. BKV -> MGV: Renominierung",
				payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::Renominierung(_)),
			},
		],
	}
}

// ===========================================================================
// KoV chains
// ===========================================================================

/// Chain 15: KoV Brennwertmitteilung (1 EDIFACT step)
///
/// 1. FNB -> LF: Brennwert (MSCONS)
pub fn kov_brennwertmitteilung() -> Kette {
	let fnb = test_mp_id(6);
	let lf = test_mp_id(0);

	Kette {
		name: "KoV Brennwertmitteilung",
		schritte: vec![KettenSchritt {
			edifact: erzeuge_mscons_brennwert(&BrennwertParams {
				sender: fnb.clone(),
				empfaenger: lf.clone(),
				netzgebiet: "Netzgebiet-Nord".to_string(),
				brennwert: "11.42".to_string(),
				zustandszahl: "0.9635".to_string(),
				gueltig_ab: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
				gueltig_bis: NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
			}),
			beschreibung: "1. FNB -> LF: Brennwert",
			payload_pruefer: |n| matches!(n.payload, NachrichtenPayload::MsconsBrennwert(_)),
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

	#[test]
	fn kette_wim_msb_wechsel() {
		pruefe_kette(&wim_msb_wechsel());
	}

	#[test]
	fn kette_wim_zaehlwertübermittlung() {
		pruefe_kette(&wim_zaehlwertübermittlung());
	}

	#[test]
	fn kette_ubp_bestellung() {
		pruefe_kette(&ubp_bestellung());
	}

	#[test]
	fn kette_mabis_bilanzkreiszuordnung() {
		pruefe_kette(&mabis_bilanzkreiszuordnung());
	}

	#[test]
	fn kette_abrechnung_netznutzung() {
		pruefe_kette(&abrechnung_netznutzung());
	}

	#[test]
	fn kette_rd2_abruf() {
		// Placeholder: no EDIFACT steps, just verifies the function compiles and chain is empty.
		let kette = rd2_abruf();
		assert_eq!(kette.schritte.len(), 0);
	}

	#[test]
	fn kette_steuerung_14a() {
		pruefe_kette(&steuerung_14a());
	}

	#[test]
	fn kette_geli_gas_lfw() {
		pruefe_kette(&geli_gas_lfw());
	}

	#[test]
	fn kette_gabi_gas_nominierung() {
		pruefe_kette(&gabi_gas_nominierung());
	}

	#[test]
	fn kette_kov_brennwertmitteilung() {
		pruefe_kette(&kov_brennwertmitteilung());
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
