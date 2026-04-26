use std::fmt;

use super::segment::{EdifactNachricht, Element, Interchange, Segment};

/// Errors that can occur during EDIFACT parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
	UnexpectedEnd,
	InvalidSegment(String),
	MissingSeparator,
	MissingSegment(String),
	InvalidQualifier(String),
	InvalidDate(String),
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseError::UnexpectedEnd => write!(f, "unexpected end of input"),
			ParseError::InvalidSegment(s) => write!(f, "invalid segment: {s}"),
			ParseError::MissingSeparator => write!(f, "missing separator"),
			ParseError::MissingSegment(s) => write!(f, "missing required segment: {s}"),
			ParseError::InvalidQualifier(s) => write!(f, "invalid qualifier: {s}"),
			ParseError::InvalidDate(s) => write!(f, "invalid date: {s}"),
		}
	}
}

impl std::error::Error for ParseError {}

/// Parse an EDIFACT string into segments.
/// Handles separators: ' (segment), + (element), : (component), ? (escape)
pub fn parse_segments(input: &str) -> Result<Vec<Segment>, ParseError> {
	let input = input.trim();
	if input.is_empty() {
		return Ok(vec![]);
	}

	let raw_segments = split_escaped(input, '\'');
	let mut segments = Vec::new();

	for raw in raw_segments {
		let raw = raw.trim();
		if raw.is_empty() {
			continue;
		}

		let segment = parse_single_segment(raw)?;
		segments.push(segment);
	}

	Ok(segments)
}

/// Parse a single segment string (without trailing ') into a Segment.
fn parse_single_segment(raw: &str) -> Result<Segment, ParseError> {
	let element_strs = split_escaped(raw, '+');
	if element_strs.is_empty() {
		return Err(ParseError::InvalidSegment(raw.to_string()));
	}

	let tag = unescape(&element_strs[0]);
	if tag.is_empty() {
		return Err(ParseError::InvalidSegment(raw.to_string()));
	}

	let elements: Vec<Element> = element_strs[1..]
		.iter()
		.map(|e| {
			let components = split_escaped(e, ':')
				.into_iter()
				.map(|c| unescape(&c))
				.collect();
			Element { components }
		})
		.collect();

	Ok(Segment { tag, elements })
}

/// Parse a full interchange (UNB..UNZ).
pub fn parse_interchange(input: &str) -> Result<Interchange, ParseError> {
	let segments = parse_segments(input)?;

	// Find UNB
	let unb = segments
		.first()
		.filter(|s| s.tag == "UNB")
		.ok_or(ParseError::InvalidSegment(
			"interchange must start with UNB".to_string(),
		))?;

	let sender = unb
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();
	let empfaenger = unb
		.elements
		.get(2)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();
	let datum = unb
		.elements
		.get(3)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();

	// UNB control reference (data element 5, index 4) — used to verify the
	// matching UNZ reference at the end of the interchange.
	let unb_reference = unb
		.elements
		.get(4)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();

	// Collect messages between UNH..UNT pairs.
	let mut nachrichten = Vec::new();
	let mut current_msg: Option<(String, String, Vec<Segment>)> = None;
	let mut unz_segment: Option<&Segment> = None;

	for seg in &segments[1..] {
		match seg.tag.as_str() {
			"UNH" => {
				if current_msg.is_some() {
					return Err(ParseError::InvalidSegment(
						"UNH without preceding UNT — previous message is unterminated"
							.to_string(),
					));
				}
				let typ = seg
					.elements
					.get(1)
					.and_then(|e| e.components.first())
					.cloned()
					.unwrap_or_default();
				let version = seg
					.elements
					.get(1)
					.map(|e| e.components[1..].join(":"))
					.unwrap_or_default();
				current_msg = Some((typ, version, Vec::new()));
			}
			"UNT" => {
				let (typ, version, segmente) = current_msg.take().ok_or_else(|| {
					ParseError::InvalidSegment("UNT without preceding UNH".to_string())
				})?;
				nachrichten.push(EdifactNachricht {
					typ,
					version,
					segmente,
				});
			}
			"UNZ" => {
				unz_segment = Some(seg);
				break;
			}
			_ => {
				if let Some((_, _, ref mut segs)) = current_msg {
					segs.push(seg.clone());
				}
			}
		}
	}

	if current_msg.is_some() {
		return Err(ParseError::MissingSegment("UNT".to_string()));
	}

	let unz = unz_segment.ok_or_else(|| ParseError::MissingSegment("UNZ".to_string()))?;

	// Validate UNZ count (data element 1) matches number of messages.
	let unz_count_str = unz
		.elements
		.first()
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();
	let unz_count: usize = unz_count_str.parse().map_err(|_| {
		ParseError::InvalidSegment(format!(
			"UNZ count is not a number: {unz_count_str:?}"
		))
	})?;
	if unz_count != nachrichten.len() {
		return Err(ParseError::InvalidSegment(format!(
			"UNZ count {unz_count} does not match {} messages in interchange",
			nachrichten.len()
		)));
	}

	// Validate UNZ control reference (data element 2) matches UNB reference.
	let unz_reference = unz
		.elements
		.get(1)
		.and_then(|e| e.components.first())
		.cloned()
		.unwrap_or_default();
	if !unb_reference.is_empty() && unz_reference != unb_reference {
		return Err(ParseError::InvalidSegment(format!(
			"UNZ reference {unz_reference:?} does not match UNB reference {unb_reference:?}"
		)));
	}

	Ok(Interchange {
		sender,
		empfaenger,
		datum,
		nachrichten,
	})
}

/// Split a string by a separator character, respecting ? as escape.
fn split_escaped(input: &str, separator: char) -> Vec<String> {
	let mut parts = Vec::new();
	let mut current = String::new();
	let mut chars = input.chars().peekable();

	while let Some(c) = chars.next() {
		if c == '?' {
			// Escape: next character is literal
			if let Some(next) = chars.next() {
				current.push('?');
				current.push(next);
			}
		} else if c == separator {
			parts.push(current);
			current = String::new();
		} else {
			current.push(c);
		}
	}

	parts.push(current);
	parts
}

/// Remove escape characters from a string.
fn unescape(input: &str) -> String {
	let mut result = String::new();
	let mut chars = input.chars().peekable();

	while let Some(c) = chars.next() {
		if c == '?' {
			if let Some(next) = chars.next() {
				result.push(next);
			}
		} else {
			result.push(c);
		}
	}

	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_unh_segment() {
		let segments = parse_segments("UNH+1+UTILMD:D:11A:UN:2.7a'").unwrap();
		assert_eq!(segments.len(), 1);
		let seg = &segments[0];
		assert_eq!(seg.tag, "UNH");
		assert_eq!(seg.elements.len(), 2);
		assert_eq!(seg.elements[0].components, vec!["1"]);
		assert_eq!(
			seg.elements[1].components,
			vec!["UTILMD", "D", "11A", "UN", "2.7a"]
		);
	}

	#[test]
	fn parse_bgm_segment() {
		let segments = parse_segments("BGM+E01+12345+9'").unwrap();
		assert_eq!(segments.len(), 1);
		let seg = &segments[0];
		assert_eq!(seg.tag, "BGM");
		assert_eq!(seg.elements.len(), 3);
		assert_eq!(seg.elements[0].components, vec!["E01"]);
		assert_eq!(seg.elements[1].components, vec!["12345"]);
		assert_eq!(seg.elements[2].components, vec!["9"]);
	}

	#[test]
	fn parse_dtm_segment() {
		let segments = parse_segments("DTM+137:20250701:102'").unwrap();
		assert_eq!(segments.len(), 1);
		let seg = &segments[0];
		assert_eq!(seg.tag, "DTM");
		assert_eq!(seg.elements.len(), 1);
		assert_eq!(
			seg.elements[0].components,
			vec!["137", "20250701", "102"]
		);
	}

	#[test]
	fn parse_escape_character() {
		let segments = parse_segments("NAD+MS+?+test::293'").unwrap();
		assert_eq!(segments.len(), 1);
		let seg = &segments[0];
		assert_eq!(seg.tag, "NAD");
		assert_eq!(seg.elements.len(), 2);
		assert_eq!(seg.elements[0].components, vec!["MS"]);
		// ?+ is escaped, so "+test" becomes a single component value containing "+"
		assert_eq!(seg.elements[1].components, vec!["+test", "", "293"]);
	}

	#[test]
	fn parse_multiple_segments() {
		let input = "UNH+1+UTILMD:D:11A:UN:2.7a'BGM+E01+12345+9'";
		let segments = parse_segments(input).unwrap();
		assert_eq!(segments.len(), 2);
		assert_eq!(segments[0].tag, "UNH");
		assert_eq!(segments[1].tag, "BGM");
	}

	#[test]
	fn parse_empty_input() {
		let segments = parse_segments("").unwrap();
		assert!(segments.is_empty());
	}

	#[test]
	fn parse_full_interchange() {
		let input = "\
UNB+UNOC:3+9900000000003:500+9900000000010:500+20250701:1200+00001'\
UNH+1+UTILMD:D:11A:UN:2.7a'\
BGM+E01+12345+9'\
DTM+137:20250701:102'\
UNT+4+1'\
UNZ+1+00001'";

		let interchange = parse_interchange(input).unwrap();
		assert_eq!(interchange.sender, "9900000000003");
		assert_eq!(interchange.empfaenger, "9900000000010");
		assert_eq!(interchange.datum, "20250701");
		assert_eq!(interchange.nachrichten.len(), 1);

		let msg = &interchange.nachrichten[0];
		assert_eq!(msg.typ, "UTILMD");
		assert_eq!(msg.version, "D:11A:UN:2.7a");
		// BGM and DTM are the body segments (UNH/UNT excluded)
		assert_eq!(msg.segmente.len(), 2);
		assert_eq!(msg.segmente[0].tag, "BGM");
		assert_eq!(msg.segmente[1].tag, "DTM");
	}

	#[test]
	fn error_on_invalid_interchange_missing_unb() {
		let input = "UNH+1+UTILMD:D:11A:UN:2.7a'UNT+2+1'";
		let result = parse_interchange(input);
		assert!(result.is_err());
	}

	#[test]
	fn error_on_invalid_segment() {
		let input = "+'";
		let result = parse_segments(input);
		assert!(matches!(result, Err(ParseError::InvalidSegment(_))));
	}
}
