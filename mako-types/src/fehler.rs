use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValidationError {
	#[error("invalid length: expected {expected}, got {actual}")]
	InvalidLength { expected: usize, actual: usize },

	#[error("invalid check digit: expected {expected}, got {actual}")]
	InvalidCheckDigit { expected: char, actual: char },

	#[error("invalid characters: only digits allowed")]
	InvalidCharacters,
}

/// Process-level errors for reducers
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ProzessFehler {
	#[error("invalid state transition: {state} cannot handle {event}")]
	UngueltigerUebergang { state: String, event: String },

	#[error("validation error: {0}")]
	Validierungsfehler(String),

	#[error("deadline exceeded: deadline was {frist}, received on {eingang}")]
	FristUeberschritten { frist: String, eingang: String },
}
