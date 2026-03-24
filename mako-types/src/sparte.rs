use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sparte {
	Strom,
	Gas,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sparte_display_and_equality() {
		assert_ne!(Sparte::Strom, Sparte::Gas);
		assert_eq!(Sparte::Strom, Sparte::Strom);
	}

	#[test]
	fn sparte_serializes_to_json() {
		let json = serde_json::to_string(&Sparte::Strom).unwrap();
		assert_eq!(json, "\"Strom\"");
		let json = serde_json::to_string(&Sparte::Gas).unwrap();
		assert_eq!(json, "\"Gas\"");
	}
}
