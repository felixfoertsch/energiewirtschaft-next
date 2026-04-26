pub mod referenzdaten;
pub mod ahb;
pub mod ahb_ausdruck;
pub mod ebd;
pub mod interop;
pub mod bericht;

use std::path::Path;

use mako_codec::edifact::parser::parse_interchange;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};

use crate::ahb::validiere_nachricht_ahb;
use crate::bericht::{BatchErgebnis, Urteil, VerifikationsErgebnis};
use crate::ebd::vergleiche_ergebnis;
use crate::interop::extrahiere_schluesselfelder;
use crate::referenzdaten::Referenzdaten;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parse raw EDIFACT, run AHB validation (Layer 1), and extract key fields
/// (Layer 3 prep). Returns a combined `VerifikationsErgebnis`.
pub fn verifiziere_nachricht(
	edifact_roh: &str,
	refdata: &Referenzdaten,
) -> VerifikationsErgebnis {
	// parse the interchange
	let interchange = match parse_interchange(edifact_roh) {
		Ok(ic) => ic,
		Err(e) => {
			return VerifikationsErgebnis {
				datei: String::new(),
				nachrichtentyp: String::new(),
				pruefidentifikator: None,
				ahb: None,
				ebd: None,
				interop: None,
				gesamt_urteil: Urteil::NichtPruefbar,
				fehler: Some(format!("EDIFACT-Parse fehlgeschlagen: {e}")),
			};
		}
	};

	if interchange.nachrichten.is_empty() {
		return VerifikationsErgebnis {
			datei: String::new(),
			nachrichtentyp: String::new(),
			pruefidentifikator: None,
			ahb: None,
			ebd: None,
			interop: None,
			gesamt_urteil: Urteil::NichtPruefbar,
			fehler: Some("Interchange enthält keine Nachrichten".to_string()),
		};
	}

	let edifact_msg = &interchange.nachrichten[0];
	let nachrichtentyp = edifact_msg.typ.clone();

	// extract pruefidentifikator from RFF+Z13 segment
	let pruefidentifikator = edifact_msg
		.segmente
		.iter()
		.find(|s| {
			s.tag == "RFF"
				&& s.elements
					.first()
					.and_then(|e| e.components.first())
					.map(|c| c == "Z13")
					.unwrap_or(false)
		})
		.and_then(|s| {
			s.elements
				.first()
				.and_then(|e| e.components.get(1))
				.cloned()
		});

	// Layer 1: AHB validation
	let ahb = pruefidentifikator.as_ref().map(|pi| {
		validiere_nachricht_ahb(edifact_msg, pi, &nachrichtentyp, refdata)
	});

	// Layer 3: typed parse + key-field extraction. Without a third-party
	// reference parser to compare against, we still report whether OUR parser
	// accepted the message and which key fields it surfaced — this turns
	// "no third-party available" into observable data instead of a missing tab.
	let interop = match mako_codec::edifact::dispatch::parse_nachricht(edifact_roh) {
		Ok(n) => {
			let felder = extrahiere_schluesselfelder(&n);
			let mut feldvergleiche: Vec<crate::bericht::InteropFeldVergleich> = felder
				.into_iter()
				.map(|(feld, unser_wert)| crate::bericht::InteropFeldVergleich {
					feld,
					unser_wert: Some(unser_wert),
					drittanbieter_wert: None,
					stimmt_ueberein: false,
				})
				.collect();
			feldvergleiche.sort_by(|a, b| a.feld.cmp(&b.feld));
			Some(crate::bericht::InteropErgebnis {
				parse_ok_unser: true,
				parse_ok_drittanbieter: false,
				roundtrip_ok: false,
				feldvergleiche,
				urteil: Urteil::NichtPruefbar,
			})
		}
		Err(_) => Some(crate::bericht::InteropErgebnis {
			parse_ok_unser: false,
			parse_ok_drittanbieter: false,
			roundtrip_ok: false,
			feldvergleiche: Vec::new(),
			urteil: Urteil::Fehlgeschlagen,
		}),
	};

	// compute overall verdict from AHB result
	let gesamt_urteil = match &ahb {
		Some(a) => a.urteil,
		None => Urteil::NichtPruefbar,
	};

	VerifikationsErgebnis {
		datei: String::new(),
		nachrichtentyp,
		pruefidentifikator,
		ahb,
		ebd: None,
		interop,
		gesamt_urteil,
		fehler: None,
	}
}

/// Run Layer 2 (EBD comparison) for a process step.
///
/// Determines the answer code from output messages:
/// - If any output is a `UtilmdAblehnung`, extracts the rejection reason as antwortcode.
/// - Otherwise, assumes acceptance (no antwortcode).
///
/// Compares against EBD valid outcomes and returns a result with EBD filled in.
pub fn verifiziere_prozess_schritt(
	_eingabe: &Nachricht,
	ausgabe: &[Nachricht],
	ebd_code: &str,
	refdata: &Referenzdaten,
) -> VerifikationsErgebnis {
	let (unser_antwortcode, unser_beschreibung) = bestimme_antwortcode(ausgabe);

	let ebd_ergebnis = vergleiche_ergebnis(
		ebd_code,
		unser_antwortcode.as_deref(),
		unser_beschreibung.as_deref(),
		refdata,
	);

	let gesamt_urteil = ebd_ergebnis.urteil;

	VerifikationsErgebnis {
		datei: String::new(),
		nachrichtentyp: String::new(),
		pruefidentifikator: None,
		ahb: None,
		ebd: Some(ebd_ergebnis),
		interop: None,
		gesamt_urteil,
		fehler: None,
	}
}

/// Verify all `.edi` files in a directory tree and aggregate results.
pub fn verifiziere_batch(
	sim_verzeichnis: &Path,
	refdata: &Referenzdaten,
) -> BatchErgebnis {
	let edi_dateien = sammle_edi_dateien(sim_verzeichnis);

	let ergebnisse: Vec<VerifikationsErgebnis> = edi_dateien
		.iter()
		.map(|pfad| match std::fs::read_to_string(pfad) {
			Ok(inhalt) => {
				let mut ergebnis = verifiziere_nachricht(&inhalt, refdata);
				ergebnis.datei = pfad.display().to_string();
				ergebnis
			}
			Err(e) => VerifikationsErgebnis {
				datei: pfad.display().to_string(),
				nachrichtentyp: String::new(),
				pruefidentifikator: None,
				ahb: None,
				ebd: None,
				interop: None,
				gesamt_urteil: Urteil::NichtPruefbar,
				fehler: Some(format!("Datei nicht lesbar: {e}")),
			},
		})
		.collect();

	BatchErgebnis::aus_ergebnissen(ergebnisse)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Determine the answer code from output messages.
///
/// Scans for rejection payloads (`UtilmdAblehnung`) and extracts the rejection
/// reason as a string code. If no rejection is found, returns `None` (acceptance).
fn bestimme_antwortcode(ausgabe: &[Nachricht]) -> (Option<String>, Option<String>) {
	for nachricht in ausgabe {
		match &nachricht.payload {
			NachrichtenPayload::UtilmdAblehnung(abl) => {
				let code = format!("{:?}", abl.grund);
				return (Some(code), Some("Ablehnung".to_string()));
			}
			_ => {}
		}
	}
	// no rejection found — acceptance path
	(None, Some("Zustimmung".to_string()))
}

/// Recursively collect all `.edi` files in a directory tree.
fn sammle_edi_dateien(verzeichnis: &Path) -> Vec<std::path::PathBuf> {
	let mut dateien = Vec::new();
	sammle_edi_rekursiv(verzeichnis, &mut dateien);
	dateien.sort();
	dateien
}

fn sammle_edi_rekursiv(verzeichnis: &Path, dateien: &mut Vec<std::path::PathBuf>) {
	let Ok(eintraege) = std::fs::read_dir(verzeichnis) else {
		return;
	};

	for eintrag in eintraege.flatten() {
		let pfad = eintrag.path();
		if pfad.is_dir() {
			sammle_edi_rekursiv(&pfad, dateien);
		} else if pfad.extension().and_then(|e| e.to_str()) == Some("edi") {
			dateien.push(pfad);
		}
	}
}
