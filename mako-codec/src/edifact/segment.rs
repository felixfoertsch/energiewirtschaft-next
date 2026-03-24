/// A raw EDIFACT segment (e.g., UNH+1+UTILMD:D:11A:UN:2.7a')
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
	pub tag: String,
	pub elements: Vec<Element>,
}

/// A data element, composed of one or more components separated by `:`.
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
	pub components: Vec<String>,
}

/// An EDIFACT interchange (UNB..UNZ)
#[derive(Debug, Clone, PartialEq)]
pub struct Interchange {
	pub sender: String,
	pub empfaenger: String,
	pub datum: String,
	pub nachrichten: Vec<EdifactNachricht>,
}

/// A single message within an interchange (UNH..UNT)
#[derive(Debug, Clone, PartialEq)]
pub struct EdifactNachricht {
	pub typ: String,
	pub version: String,
	pub segmente: Vec<Segment>,
}
