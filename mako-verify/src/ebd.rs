use crate::bericht::{EbdAusgang, EbdErgebnis, Urteil};
use crate::referenzdaten::{EbdDokument, Referenzdaten};

/// Extract all terminal outcomes from an EBD decision tree.
///
/// A terminal outcome is a sub_row that either:
/// - has a `result_code` (explicit rejection or acceptance code), or
/// - has no `subsequent_step_number` (implicit terminal, no further steps)
///
/// Sub_rows that point to another step (non-None, non-"Ende" subsequent_step_number)
/// without a result_code are intermediate and skipped.
pub fn extrahiere_ausgaenge(ebd: &EbdDokument) -> Vec<EbdAusgang> {
	let mut ausgaenge = Vec::new();

	for zeile in &ebd.rows {
		for unterzeile in &zeile.sub_rows {
			let ist_terminal = unterzeile.result_code.is_some() || {
				let step = &unterzeile.check_result.subsequent_step_number;
				match step {
					None => true,
					Some(s) if s.eq_ignore_ascii_case("ende") => true,
					Some(s) if s.is_empty() => true,
					_ => false,
				}
			};

			if ist_terminal {
				ausgaenge.push(EbdAusgang {
					ebd_code: ebd.metadata.ebd_code.clone(),
					schritt: zeile.step_number.clone(),
					beschreibung: zeile.description.clone(),
					antwortcode: unterzeile.result_code.clone(),
					notiz: unterzeile.note.clone(),
				});
			}
		}
	}

	ausgaenge
}

/// Compare our reducer's output against the valid EBD outcomes.
///
/// - If the EBD reference data is missing, returns `NichtPruefbar`.
/// - If `unser_antwortcode` matches a terminal result_code, returns `Bestanden`.
/// - If `unser_antwortcode` is None, checks for an acceptance path (terminal with no result_code).
/// - Otherwise returns `Fehlgeschlagen`.
pub fn vergleiche_ergebnis(
	ebd_code: &str,
	unser_antwortcode: Option<&str>,
	unser_beschreibung: Option<&str>,
	refdata: &Referenzdaten,
) -> EbdErgebnis {
	let Some(dok) = refdata.ebd(ebd_code) else {
		return EbdErgebnis {
			ebd_code: ebd_code.to_string(),
			ebd_name: String::new(),
			rolle: String::new(),
			unser_ergebnis: None,
			gueltige_ausgaenge: Vec::new(),
			urteil: Urteil::NichtPruefbar,
			details: Some(format!("Keine Referenzdaten für EBD {ebd_code} vorhanden")),
		};
	};

	let gueltige_ausgaenge = extrahiere_ausgaenge(&dok);

	let unser_ergebnis = EbdAusgang {
		ebd_code: ebd_code.to_string(),
		schritt: String::new(),
		beschreibung: unser_beschreibung.unwrap_or_default().to_string(),
		antwortcode: unser_antwortcode.map(|s| s.to_string()),
		notiz: None,
	};

	let urteil = match unser_antwortcode {
		Some(code) => {
			// check if our code matches any terminal result_code
			if gueltige_ausgaenge
				.iter()
				.any(|a| a.antwortcode.as_deref() == Some(code))
			{
				Urteil::Bestanden
			} else {
				Urteil::Fehlgeschlagen
			}
		}
		None => {
			// acceptance path: look for a terminal without a result_code
			// (sub_row with subsequent_step_number == None or "Ende" and no result_code)
			let hat_akzeptanz = dok.rows.iter().any(|zeile| {
				zeile.sub_rows.iter().any(|u| {
					u.result_code.is_none() && {
						let step = &u.check_result.subsequent_step_number;
						match step {
							None => true,
							Some(s) if s.eq_ignore_ascii_case("ende") => true,
							Some(s) if s.is_empty() => true,
							_ => false,
						}
					}
				})
			});

			if hat_akzeptanz {
				Urteil::Bestanden
			} else {
				Urteil::Fehlgeschlagen
			}
		}
	};

	let details = match urteil {
		Urteil::Bestanden => None,
		Urteil::Fehlgeschlagen => {
			let codes: Vec<String> = gueltige_ausgaenge
				.iter()
				.filter_map(|a| a.antwortcode.clone())
				.collect();
			Some(format!(
				"Antwortcode {:?} ist kein gültiger Ausgang. Gültige Codes: {:?}",
				unser_antwortcode.unwrap_or("(Akzeptanz)"),
				codes,
			))
		}
		Urteil::NichtPruefbar => None,
	};

	EbdErgebnis {
		ebd_code: ebd_code.to_string(),
		ebd_name: dok.metadata.ebd_name.clone(),
		rolle: dok.metadata.role.clone(),
		unser_ergebnis: Some(unser_ergebnis),
		gueltige_ausgaenge,
		urteil,
		details,
	}
}
