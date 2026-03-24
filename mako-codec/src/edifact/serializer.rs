use super::segment::{Element, Interchange, Segment};

/// Escape special EDIFACT characters in a component value.
fn escape(value: &str) -> String {
	let mut result = String::with_capacity(value.len());
	for c in value.chars() {
		match c {
			'+' | ':' | '\'' | '?' => {
				result.push('?');
				result.push(c);
			}
			_ => result.push(c),
		}
	}
	result
}

/// Serialize a single element to its EDIFACT string representation.
fn serialize_element(element: &Element) -> String {
	element
		.components
		.iter()
		.map(|c| escape(c))
		.collect::<Vec<_>>()
		.join(":")
}

/// Serialize segments to an EDIFACT string.
pub fn serialize_segments(segments: &[Segment]) -> String {
	segments
		.iter()
		.map(|seg| {
			let mut parts = vec![seg.tag.clone()];
			for elem in &seg.elements {
				parts.push(serialize_element(elem));
			}
			parts.join("+")
		})
		.map(|s| format!("{s}'"))
		.collect()
}

/// Serialize a full interchange to an EDIFACT string.
pub fn serialize_interchange(interchange: &Interchange) -> String {
	let mut all_segments = Vec::new();

	// UNB header
	all_segments.push(Segment {
		tag: "UNB".to_string(),
		elements: vec![
			Element {
				components: vec!["UNOC".to_string(), "3".to_string()],
			},
			Element {
				components: vec![interchange.sender.clone(), "500".to_string()],
			},
			Element {
				components: vec![interchange.empfaenger.clone(), "500".to_string()],
			},
			Element {
				components: vec![interchange.datum.clone()],
			},
			Element {
				components: vec!["00001".to_string()],
			},
		],
	});

	let msg_count = interchange.nachrichten.len();

	for (i, msg) in interchange.nachrichten.iter().enumerate() {
		let ref_nr = format!("{}", i + 1);

		// UNH
		all_segments.push(Segment {
			tag: "UNH".to_string(),
			elements: vec![
				Element {
					components: vec![ref_nr.clone()],
				},
				Element {
					components: std::iter::once(msg.typ.clone())
						.chain(msg.version.split(':').map(String::from))
						.collect(),
				},
			],
		});

		// Body segments
		all_segments.extend(msg.segmente.iter().cloned());

		// UNT: segment count = body segments + UNH + UNT
		let seg_count = msg.segmente.len() + 2;
		all_segments.push(Segment {
			tag: "UNT".to_string(),
			elements: vec![
				Element {
					components: vec![format!("{seg_count}")],
				},
				Element {
					components: vec![ref_nr],
				},
			],
		});
	}

	// UNZ
	all_segments.push(Segment {
		tag: "UNZ".to_string(),
		elements: vec![
			Element {
				components: vec![format!("{msg_count}")],
			},
			Element {
				components: vec!["00001".to_string()],
			},
		],
	});

	serialize_segments(&all_segments)
}

#[cfg(test)]
mod tests {
	use crate::edifact::parser;
	use crate::edifact::segment::EdifactNachricht;

	use super::*;

	#[test]
	fn round_trip_single_segment() {
		let input = "UNH+1+UTILMD:D:11A:UN:2.7a'";
		let segments = parser::parse_segments(input).unwrap();
		let output = serialize_segments(&segments);
		assert_eq!(output, input);
	}

	#[test]
	fn round_trip_multiple_segments() {
		let input = "BGM+E01+12345+9'DTM+137:20250701:102'";
		let segments = parser::parse_segments(input).unwrap();
		let output = serialize_segments(&segments);
		assert_eq!(output, input);
	}

	#[test]
	fn round_trip_escape_characters() {
		let input = "NAD+MS+?+test::293'";
		let segments = parser::parse_segments(input).unwrap();
		let output = serialize_segments(&segments);
		assert_eq!(output, input);
	}

	#[test]
	fn round_trip_interchange() {
		let interchange = Interchange {
			sender: "9900000000003".to_string(),
			empfaenger: "9900000000010".to_string(),
			datum: "20250701".to_string(),
			nachrichten: vec![EdifactNachricht {
				typ: "UTILMD".to_string(),
				version: "D:11A:UN:2.7a".to_string(),
				segmente: vec![
					Segment {
						tag: "BGM".to_string(),
						elements: vec![
							Element {
								components: vec!["E01".to_string()],
							},
							Element {
								components: vec!["12345".to_string()],
							},
							Element {
								components: vec!["9".to_string()],
							},
						],
					},
					Segment {
						tag: "DTM".to_string(),
						elements: vec![Element {
							components: vec![
								"137".to_string(),
								"20250701".to_string(),
								"102".to_string(),
							],
						}],
					},
				],
			}],
		};

		let serialized = serialize_interchange(&interchange);
		let parsed_back = parser::parse_interchange(&serialized).unwrap();

		assert_eq!(parsed_back.sender, interchange.sender);
		assert_eq!(parsed_back.empfaenger, interchange.empfaenger);
		assert_eq!(parsed_back.nachrichten.len(), 1);
		assert_eq!(parsed_back.nachrichten[0].typ, "UTILMD");
		assert_eq!(parsed_back.nachrichten[0].version, "D:11A:UN:2.7a");
		assert_eq!(parsed_back.nachrichten[0].segmente.len(), 2);
		assert_eq!(parsed_back.nachrichten[0].segmente, interchange.nachrichten[0].segmente);
	}

	#[test]
	fn escape_special_characters() {
		let seg = Segment {
			tag: "FTX".to_string(),
			elements: vec![Element {
				components: vec!["text with + and : and ' inside".to_string()],
			}],
		};
		let output = serialize_segments(&[seg]);
		// All special chars should be escaped
		assert!(output.contains("?+"));
		assert!(output.contains("?:"));
		assert!(output.contains("?'"));

		// Round-trip must preserve the value
		let parsed = parser::parse_segments(&output).unwrap();
		assert_eq!(
			parsed[0].elements[0].components[0],
			"text with + and : and ' inside"
		);
	}
}
