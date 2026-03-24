use serde::{Deserialize, Serialize};

use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::Nachricht;

/// Whether this is a CONTRL (syntax-level) or APERAK (business-level) receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuittungsTyp {
	Contrl,
	Aperak,
}

/// Outcome of a receipt check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuittungsErgebnis {
	Positiv,
	Negativ(FehlerCode),
}

/// Coded reason for a negative receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FehlerCode {
	AbsenderLeer,
	EmpfaengerLeer,
	LieferbeginnInVergangenheit,
}

/// A receipt (CONTRL or APERAK) to be sent back to a market partner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quittung {
	pub an: MarktpartnerId,
	pub typ: QuittungsTyp,
	pub ergebnis: QuittungsErgebnis,
}

/// Wraps a reducer's output with the receipts generated during validation.
#[derive(Debug, Clone, PartialEq)]
pub struct DekorierterOutput<S> {
	pub state: S,
	pub nachrichten: Vec<Nachricht>,
	pub quittungen: Vec<Quittung>,
}
