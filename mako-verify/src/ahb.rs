use mako_codec::edifact::segment::EdifactNachricht;

use crate::bericht::{AhbErgebnis, AhbFeldErgebnis, Urteil};
use crate::referenzdaten::{AhbZeile, Referenzdaten};

// ---------------------------------------------------------------------------
// AHB segment-presence validation (V1)
//
// For each AHB line that represents a segment header (data_element is None,
// segment_code is Some), we check whether the segment is present in the
// parsed EDIFACT message.
//
// ahb_expression semantics at segment level:
//   "Muss" / "M"           → segment must be present
//   "Kann" / "K" / "Soll" / "S" → optional, always passes
//   contains "["            → conditional, NichtPruefbar (deferred to Task 6)
//   "X"                    → segment must NOT be present
//
// Sub-lines (data_element is Some) with value_pool_entry set are checked
// as fixed-value assertions: the qualifier/value must appear somewhere in
// a matching segment's elements.
// ---------------------------------------------------------------------------

/// Validate an EDIFACT message against AHB rules (segment-presence, V1).
pub fn validiere_nachricht_ahb(
	edifact: &EdifactNachricht,
	pruefidentifikator: &str,
	nachrichtentyp: &str,
	refdata: &Referenzdaten,
) -> AhbErgebnis {
	let Some(ahb) = refdata.ahb(nachrichtentyp, pruefidentifikator) else {
		return AhbErgebnis {
			pruefidentifikator: pruefidentifikator.to_string(),
			nachrichtentyp: nachrichtentyp.to_string(),
			felder: vec![],
			urteil: Urteil::NichtPruefbar,
			zusammenfassung: Some(format!(
				"AHB-Dokument für {nachrichtentyp}/{pruefidentifikator} nicht gefunden"
			)),
		};
	};

	let mut felder: Vec<AhbFeldErgebnis> = Vec::new();

	// Identify segment-level header lines (data_element is None, segment_code is Some).
	// Skip UNH/UNT — these are envelope segments stripped by parse_interchange;
	// they are not part of EdifactNachricht.segmente.
	let segment_headers: Vec<&AhbZeile> = ahb
		.lines
		.iter()
		.filter(|z| {
			z.data_element.is_none()
				&& z.segment_code.is_some()
				&& !matches!(z.segment_code.as_deref(), Some("UNH" | "UNT"))
		})
		.collect();

	for zeile in &segment_headers {
		let seg_code = zeile.segment_code.as_deref().unwrap();
		let ahb_expr = zeile.ahb_expression.trim();
		let regel = klassifiziere_ausdruck(ahb_expr);

		match regel {
			AhbRegel::Muss => {
				let vorhanden = segment_vorhanden(edifact, seg_code);
				let urteil = if vorhanden {
					Urteil::Bestanden
				} else {
					Urteil::Fehlgeschlagen
				};
				felder.push(AhbFeldErgebnis {
					segment_code: Some(seg_code.to_string()),
					segment_group: zeile.segment_group_key.clone(),
					data_element: None,
					name: zeile.name.clone(),
					ahb_ausdruck: ahb_expr.to_string(),
					unser_wert: if vorhanden {
						Some("vorhanden".to_string())
					} else {
						None
					},
					erwarteter_wert: Some("Segment muss vorhanden sein".to_string()),
					urteil,
					details: if !vorhanden {
						Some(format!("Segment {seg_code} fehlt in der Nachricht"))
					} else {
						None
					},
				});
			}
			AhbRegel::Verboten => {
				let vorhanden = segment_vorhanden(edifact, seg_code);
				let urteil = if vorhanden {
					Urteil::Fehlgeschlagen
				} else {
					Urteil::Bestanden
				};
				felder.push(AhbFeldErgebnis {
					segment_code: Some(seg_code.to_string()),
					segment_group: zeile.segment_group_key.clone(),
					data_element: None,
					name: zeile.name.clone(),
					ahb_ausdruck: ahb_expr.to_string(),
					unser_wert: if vorhanden {
						Some("vorhanden".to_string())
					} else {
						None
					},
					erwarteter_wert: Some("Segment darf nicht vorhanden sein".to_string()),
					urteil,
					details: if vorhanden {
						Some(format!(
							"Segment {seg_code} ist vorhanden, obwohl es verboten ist"
						))
					} else {
						None
					},
				});
			}
			AhbRegel::Optional => {
				felder.push(AhbFeldErgebnis {
					segment_code: Some(seg_code.to_string()),
					segment_group: zeile.segment_group_key.clone(),
					data_element: None,
					name: zeile.name.clone(),
					ahb_ausdruck: ahb_expr.to_string(),
					unser_wert: if segment_vorhanden(edifact, seg_code) {
						Some("vorhanden".to_string())
					} else {
						None
					},
					erwarteter_wert: None,
					urteil: Urteil::Bestanden,
					details: None,
				});
			}
			AhbRegel::Bedingt => {
				felder.push(AhbFeldErgebnis {
					segment_code: Some(seg_code.to_string()),
					segment_group: zeile.segment_group_key.clone(),
					data_element: None,
					name: zeile.name.clone(),
					ahb_ausdruck: ahb_expr.to_string(),
					unser_wert: if segment_vorhanden(edifact, seg_code) {
						Some("vorhanden".to_string())
					} else {
						None
					},
					erwarteter_wert: None,
					urteil: Urteil::NichtPruefbar,
					details: Some(format!(
						"Bedingter Ausdruck '{ahb_expr}' — Bedingungsprüfung noch nicht implementiert"
					)),
				});
			}
		}
	}

	// Check fixed-value sub-lines: data_element is Some, value_pool_entry is Some,
	// and ahb_expression is "X" (meaning the value is required/fixed).
	// We verify that the value appears somewhere in matching segments.
	// Skip UNH/UNT sub-lines (envelope segments).
	let fixed_value_lines: Vec<&AhbZeile> = ahb
		.lines
		.iter()
		.filter(|z| {
			z.data_element.is_some()
				&& z.value_pool_entry.is_some()
				&& !matches!(z.segment_code.as_deref(), Some("UNH" | "UNT"))
				&& klassifiziere_ausdruck(z.ahb_expression.trim()) == AhbRegel::Verboten
		})
		.collect();

	// "X" on a data-element sub-line in AHB means the value is fixed/required,
	// not forbidden. We check that the value appears in a matching segment.
	for zeile in &fixed_value_lines {
		let seg_code = match zeile.segment_code.as_deref() {
			Some(c) => c,
			None => continue,
		};
		let erwarteter_wert = match zeile.value_pool_entry.as_deref() {
			Some(v) => v,
			None => continue,
		};

		// Skip long descriptive values — these are labels, not actual qualifiers
		if erwarteter_wert.len() > 20 {
			continue;
		}

		let gefunden = wert_in_segment(edifact, seg_code, erwarteter_wert);

		felder.push(AhbFeldErgebnis {
			segment_code: Some(seg_code.to_string()),
			segment_group: zeile.segment_group_key.clone(),
			data_element: zeile.data_element.clone(),
			name: zeile.name.clone(),
			ahb_ausdruck: "X".to_string(),
			unser_wert: if gefunden {
				Some(erwarteter_wert.to_string())
			} else {
				None
			},
			erwarteter_wert: Some(erwarteter_wert.to_string()),
			urteil: if gefunden {
				Urteil::Bestanden
			} else if segment_vorhanden(edifact, seg_code) {
				Urteil::Fehlgeschlagen
			} else {
				// segment itself is missing — the segment-level check already caught this
				Urteil::NichtPruefbar
			},
			details: if !gefunden && segment_vorhanden(edifact, seg_code) {
				Some(format!(
					"Wert '{erwarteter_wert}' nicht in Segment {seg_code} gefunden"
				))
			} else {
				None
			},
		});
	}

	// Also check sub-lines where ahb_expression contains "[" (conditional with value).
	// Skip UNH/UNT sub-lines (envelope segments).
	let conditional_value_lines: Vec<&AhbZeile> = ahb
		.lines
		.iter()
		.filter(|z| {
			z.data_element.is_some()
				&& z.value_pool_entry.is_some()
				&& !matches!(z.segment_code.as_deref(), Some("UNH" | "UNT"))
				&& z.ahb_expression.contains('[')
		})
		.collect();

	for zeile in &conditional_value_lines {
		let seg_code = match zeile.segment_code.as_deref() {
			Some(c) => c,
			None => continue,
		};

		felder.push(AhbFeldErgebnis {
			segment_code: Some(seg_code.to_string()),
			segment_group: zeile.segment_group_key.clone(),
			data_element: zeile.data_element.clone(),
			name: zeile.name.clone(),
			ahb_ausdruck: zeile.ahb_expression.clone(),
			unser_wert: None,
			erwarteter_wert: zeile.value_pool_entry.clone(),
			urteil: Urteil::NichtPruefbar,
			details: Some(
				"Bedingter Ausdruck — Bedingungsprüfung noch nicht implementiert".to_string(),
			),
		});
	}

	// Compute overall verdict
	let urteil = gesamt_urteil(&felder);

	let zusammenfassung = {
		let bestanden = felder.iter().filter(|f| f.urteil == Urteil::Bestanden).count();
		let fehler = felder
			.iter()
			.filter(|f| f.urteil == Urteil::Fehlgeschlagen)
			.count();
		let offen = felder
			.iter()
			.filter(|f| f.urteil == Urteil::NichtPruefbar)
			.count();
		Some(format!(
			"{} Prüfungen: {} bestanden, {} fehlgeschlagen, {} nicht prüfbar",
			felder.len(),
			bestanden,
			fehler,
			offen
		))
	};

	AhbErgebnis {
		pruefidentifikator: pruefidentifikator.to_string(),
		nachrichtentyp: nachrichtentyp.to_string(),
		felder,
		urteil,
		zusammenfassung,
	}
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AhbRegel {
	Muss,
	Optional,
	Verboten,
	Bedingt,
}

fn klassifiziere_ausdruck(expr: &str) -> AhbRegel {
	if expr.contains('[') {
		return AhbRegel::Bedingt;
	}
	match expr {
		"Muss" | "M" => AhbRegel::Muss,
		"X" => AhbRegel::Verboten,
		"Kann" | "K" | "Soll" | "S" => AhbRegel::Optional,
		_ => {
			// "Muss [10]" etc. with brackets would have been caught above.
			// Treat anything starting with "Muss" as Muss.
			if expr.starts_with("Muss") {
				AhbRegel::Muss
			} else if expr.starts_with("Kann") || expr.starts_with("Soll") {
				AhbRegel::Optional
			} else {
				// Unknown expression — treat as not checkable
				AhbRegel::Bedingt
			}
		}
	}
}

/// Check whether at least one segment with the given tag exists in the message.
fn segment_vorhanden(edifact: &EdifactNachricht, tag: &str) -> bool {
	edifact.segmente.iter().any(|s| s.tag == tag)
}

/// Check whether a value appears as a component in any element of a matching segment.
fn wert_in_segment(edifact: &EdifactNachricht, tag: &str, wert: &str) -> bool {
	edifact
		.segmente
		.iter()
		.filter(|s| s.tag == tag)
		.any(|s| {
			s.elements
				.iter()
				.any(|el| el.components.iter().any(|c| c == wert))
		})
}

/// Compute the overall verdict from individual field results.
fn gesamt_urteil(felder: &[AhbFeldErgebnis]) -> Urteil {
	if felder.iter().any(|f| f.urteil == Urteil::Fehlgeschlagen) {
		Urteil::Fehlgeschlagen
	} else if felder.iter().all(|f| f.urteil == Urteil::NichtPruefbar) {
		Urteil::NichtPruefbar
	} else {
		Urteil::Bestanden
	}
}
