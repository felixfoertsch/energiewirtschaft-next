//! Error injection for test corpus.
//! Each FehlerArt injects a specific defect into a valid EDIFACT string
//! so tests can verify the correct detection layer catches it.

/// Catalog of injectable errors, mapped to the three detection layers.
#[derive(Debug, Clone)]
pub enum FehlerArt {
	// Syntax errors (CONTRL layer — parser fails)
	/// NAD+MS MP-ID → empty
	AbsenderLeer,
	/// NAD+MR MP-ID → empty
	EmpfaengerLeer,
	/// Remove named segment entirely
	SegmentFehlt(String),
	/// UNT count off by a large amount
	FalscheSegmentzahl,

	// Application errors (APERAK layer — parser succeeds but validation fails)
	/// IDE+24 value → invalid checksum
	UngueltigeMaLoId,
	/// NAD+MS value → 12 digits instead of 13
	UngueltigeMarktpartnerId,
	/// RFF+Z13 → 99999 (unknown PID)
	FalscherPruefIdentifikator,

	// Business errors (Reducer layer — parse + validate succeed, reducer rejects)
	/// DTM+92 lieferbeginn → 2020-01-01
	FristInVergangenheit,
}

/// Inject a specific error into a valid EDIFACT string.
/// Returns a new string with the error — the original is unchanged.
pub fn injiziere_fehler(edifact: &str, fehler: &FehlerArt) -> String {
	match fehler {
		FehlerArt::AbsenderLeer => {
			replace_segment_field(edifact, "NAD+MS+", |_| "NAD+MS+::293'".to_string())
		}
		FehlerArt::EmpfaengerLeer => {
			replace_segment_field(edifact, "NAD+MR+", |_| "NAD+MR+::293'".to_string())
		}
		FehlerArt::SegmentFehlt(tag) => remove_segment(edifact, tag),
		FehlerArt::FalscheSegmentzahl => {
			replace_segment_field(edifact, "UNT+", |seg| {
				// Replace real count with 99 to make UNT wrong.
				// seg starts at "UNT+", find the next '+' to get the count portion.
				if let Some(count_end) = seg[4..].find('+') {
					let rest = &seg[4 + count_end..];
					format!("UNT+99{rest}")
				} else {
					seg.to_string()
				}
			})
		}
		FehlerArt::UngueltigeMaLoId => {
			// 51238696781: 11 digits, fails Luhn (correct check digit is '8', not '1')
			replace_segment_field(edifact, "IDE+24+", |_| "IDE+24+51238696781'".to_string())
		}
		FehlerArt::UngueltigeMarktpartnerId => {
			// 12 digits — fails length validation
			replace_segment_field(edifact, "NAD+MS+", |_| {
				"NAD+MS+123456789012::293'".to_string()
			})
		}
		FehlerArt::FalscherPruefIdentifikator => {
			replace_segment_field(edifact, "RFF+Z13:", |_| "RFF+Z13:99999'".to_string())
		}
		FehlerArt::FristInVergangenheit => {
			replace_segment_field(edifact, "DTM+92:", |_| "DTM+92:20200101:102'".to_string())
		}
	}
}

// ---------------------------------------------------------------------------
// String-level helpers
// ---------------------------------------------------------------------------

/// Find the first segment whose raw text starts with `prefix`, apply
/// `transform` to the full segment string (from prefix to closing `'`),
/// and return a new EDIFACT string with the replacement in place.
/// If no matching segment is found, returns the original unchanged.
fn replace_segment_field<F>(edifact: &str, prefix: &str, transform: F) -> String
where
	F: FnOnce(&str) -> String,
{
	let Some(start) = edifact.find(prefix) else {
		return edifact.to_string();
	};
	// Find the segment terminator `'` starting from `start`.
	let Some(term_offset) = edifact[start..].find('\'') else {
		return edifact.to_string();
	};
	let end = start + term_offset + 1; // include the `'`
	let original_segment = &edifact[start..end];
	let replacement = transform(original_segment);
	format!("{}{}{}", &edifact[..start], replacement, &edifact[end..])
}

/// Remove the first segment whose tag matches `tag` exactly.
/// Segments are delimited by `'`. The tag is the text before the first `+`.
fn remove_segment(edifact: &str, tag: &str) -> String {
	// Build the prefix to search for: tag followed by `+` or a `'` (segment with no fields)
	let prefix_with_plus = format!("{tag}+");
	let prefix_standalone = format!("{tag}'");

	let prefix = if edifact.contains(&prefix_with_plus) {
		prefix_with_plus
	} else if edifact.contains(&prefix_standalone) {
		prefix_standalone.clone()
	} else {
		return edifact.to_string();
	};

	let Some(start) = edifact.find(&prefix) else {
		return edifact.to_string();
	};
	let Some(term_offset) = edifact[start..].find('\'') else {
		return edifact.to_string();
	};
	let end = start + term_offset + 1;
	format!("{}{}", &edifact[..start], &edifact[end..])
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use chrono::NaiveDate;
	use mako_codec::edifact::dispatch::parse_nachricht;
	use mako_types::nachricht::NachrichtenPayload;

	use crate::generator::edifact::anmeldung;

	use super::*;

	#[test]
	fn syntax_absender_leer() {
		let valid = anmeldung();
		let kaputt = injiziere_fehler(&valid, &FehlerArt::AbsenderLeer);
		// NAD+MS must now have an empty ID component
		assert!(kaputt.contains("NAD+MS+::293'"));
		// Parser should fail because MarktpartnerId::new("") returns Err
		let result = parse_nachricht(&kaputt);
		assert!(result.is_err());
	}

	#[test]
	fn syntax_empfaenger_leer() {
		let valid = anmeldung();
		let kaputt = injiziere_fehler(&valid, &FehlerArt::EmpfaengerLeer);
		assert!(kaputt.contains("NAD+MR+::293'"));
		let result = parse_nachricht(&kaputt);
		assert!(result.is_err());
	}

	#[test]
	fn syntax_segment_fehlt_bgm() {
		let valid = anmeldung();
		let kaputt = injiziere_fehler(&valid, &FehlerArt::SegmentFehlt("BGM".to_string()));
		assert!(!kaputt.contains("BGM+"));
		let result = parse_nachricht(&kaputt);
		assert!(result.is_err());
	}

	#[test]
	fn syntax_falsche_segmentzahl() {
		let valid = anmeldung();
		let kaputt = injiziere_fehler(&valid, &FehlerArt::FalscheSegmentzahl);
		// Injection changed the string
		assert_ne!(valid, kaputt);
		// UNT now carries count 99
		assert!(kaputt.contains("UNT+99+"));
	}

	#[test]
	fn anwendung_ungueltige_malo() {
		let valid = anmeldung();
		let kaputt = injiziere_fehler(&valid, &FehlerArt::UngueltigeMaLoId);
		assert!(kaputt.contains("IDE+24+51238696781'"));
		// MaLoId::new fails on wrong check digit → parse_nachricht returns Err
		let result = parse_nachricht(&kaputt);
		assert!(result.is_err());
	}

	#[test]
	fn anwendung_ungueltige_mp_id() {
		let valid = anmeldung();
		let kaputt = injiziere_fehler(&valid, &FehlerArt::UngueltigeMarktpartnerId);
		assert!(kaputt.contains("NAD+MS+123456789012::293'"));
		// 12 digits fails MarktpartnerId::new → parse_nachricht returns Err
		let result = parse_nachricht(&kaputt);
		assert!(result.is_err());
	}

	#[test]
	fn anwendung_falscher_pid() {
		let valid = anmeldung();
		let kaputt = injiziere_fehler(&valid, &FehlerArt::FalscherPruefIdentifikator);
		assert!(kaputt.contains("RFF+Z13:99999'"));
		// Unknown PID code falls through to BGM-based dispatch, so parsing still succeeds.
		// The pruef_id field is None because 99999 is not a known PruefIdentifikator.
		let result = parse_nachricht(&kaputt).expect("unbekannter PID parsebar via BGM fallback");
		assert!(
			result.pruef_id.is_none(),
			"unbekannter PID muss pruef_id = None ergeben"
		);
	}

	#[test]
	fn fachlich_frist_in_vergangenheit() {
		let valid = anmeldung();
		let kaputt = injiziere_fehler(&valid, &FehlerArt::FristInVergangenheit);
		assert!(kaputt.contains("DTM+92:20200101:102'"));
		// Must parse successfully — this is a valid EDIFACT message
		let result = parse_nachricht(&kaputt).expect("frist in vergangenheit muss parsebar sein");
		if let NachrichtenPayload::UtilmdAnmeldung(anm) = &result.payload {
			assert_eq!(anm.lieferbeginn, NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
		} else {
			panic!("erwartet UtilmdAnmeldung, erhalten: {:?}", result.payload);
		}
	}

	#[test]
	fn injiziere_fehler_preserves_original() {
		let valid = anmeldung();
		let original_clone = valid.clone();
		let _kaputt = injiziere_fehler(&valid, &FehlerArt::AbsenderLeer);
		// valid is unchanged — pure function
		assert_eq!(valid, original_clone);
		let reparsed = parse_nachricht(&valid).unwrap();
		assert!(matches!(reparsed.payload, NachrichtenPayload::UtilmdAnmeldung(_)));
	}

	#[test]
	fn segment_fehlt_keine_wirkung_bei_unbekanntem_tag() {
		let valid = anmeldung();
		let result = injiziere_fehler(&valid, &FehlerArt::SegmentFehlt("GIBTS_NICHT".to_string()));
		// No matching segment → string unchanged
		assert_eq!(valid, result);
	}

	#[test]
	fn replace_segment_field_keine_wirkung_bei_fehlendem_praefix() {
		let input = "UNB+TEST'BGM+E01+DOK00001'UNZ+1+00001'";
		let result = replace_segment_field(input, "GIBTS_NICHT+", |s| s.to_string());
		assert_eq!(input, result);
	}
}
