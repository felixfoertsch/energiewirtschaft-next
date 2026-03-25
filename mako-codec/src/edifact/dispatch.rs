use mako_types::nachricht::Nachricht;

use crate::fehler::CodecFehler;

/// Parse an EDIFACT string into a typed Nachricht.
/// Dispatches based on UNH message type + RFF+Z13 Prüfidentifikator.
pub fn parse_nachricht(input: &str) -> Result<Nachricht, CodecFehler> {
	todo!("Task 2 implements the first variant")
}

/// Serialize a typed Nachricht to an EDIFACT string.
pub fn serialize_nachricht(nachricht: &Nachricht) -> String {
	todo!("Task 3 implements the first variant")
}
