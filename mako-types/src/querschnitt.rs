use serde::{Deserialize, Serialize};

/// IFTSTA status message (placeholder, cross-cutting)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Iftsta {
	pub status_code: String,
	pub beschreibung: String,
}

/// PARTIN market partner master data (placeholder, cross-cutting)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Partin {
	pub mp_id: String,
	pub name: String,
}

/// UTILTS calculation formulas / metering time definitions (placeholder)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Utilts {
	pub formel_id: String,
}
