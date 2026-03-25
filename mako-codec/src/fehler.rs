use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CodecFehler {
	// Lexer errors
	#[error("no UNA or UNB found")]
	KeinUnaOderUnb,

	#[error("invalid separator at position {position}")]
	UngueltigesTrennzeichen { position: usize },

	#[error("unterminated escape sequence at position {position}")]
	UnterbrocheneEscapeSequenz { position: usize },

	// Parse error delegation
	#[error("parse error: {0}")]
	Parse(String),

	// Dispatch errors — typed message level
	#[error("missing segment: expected {erwartet}")]
	SegmentFehlt { erwartet: String },

	#[error("unknown message type: {typ}")]
	UnbekannterNachrichtentyp { typ: String },

	#[error("unknown Prüfidentifikator: {code}")]
	UnbekannterPruefIdentifikator { code: String },

	#[error("missing field {feld} in segment {segment}")]
	FeldFehlt { segment: String, feld: String },

	#[error("invalid value '{wert}' for {feld} in {segment}")]
	UngueltigerWert { segment: String, feld: String, wert: String },

	#[error("invalid format for {feld} in {segment}, expected {erwartet}")]
	UngueltigesFormat { segment: String, feld: String, erwartet: String },

	// XML errors (Task 13)
	#[error("XML parse error: {0}")]
	XmlParseFehler(String),

	#[error("XSD validation error: {0}")]
	XsdValidierungsFehler(String),
}
