use std::fmt;

use serde::{Deserialize, Serialize};

use crate::fehler::ValidationError;

/// Marktlokations-ID (11 digits, last digit = check digit per Luhn algorithm)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaLoId(String);

impl MaLoId {
	pub fn new(value: &str) -> Result<Self, ValidationError> {
		if value.len() != 11 {
			return Err(ValidationError::InvalidLength {
				expected: 11,
				actual: value.len(),
			});
		}
		if !value.chars().all(|c| c.is_ascii_digit()) {
			return Err(ValidationError::InvalidCharacters);
		}
		let expected = luhn_check_digit(&value[..10]);
		let actual = value.chars().last().unwrap();
		if actual != expected {
			return Err(ValidationError::InvalidCheckDigit { expected, actual });
		}
		Ok(Self(value.to_string()))
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for MaLoId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

/// Marktpartner-ID (13 digits, BDEW Codenummer)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MarktpartnerId(String);

impl MarktpartnerId {
	pub fn new(value: &str) -> Result<Self, ValidationError> {
		if value.len() != 13 {
			return Err(ValidationError::InvalidLength {
				expected: 13,
				actual: value.len(),
			});
		}
		if !value.chars().all(|c| c.is_ascii_digit()) {
			return Err(ValidationError::InvalidCharacters);
		}
		Ok(Self(value.to_string()))
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for MarktpartnerId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

/// Messlokations-ID (33 characters: "DE" + 31 alphanumeric)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MeLoId(String);

impl MeLoId {
	pub fn new(value: &str) -> Result<Self, ValidationError> {
		if value.len() != 33 {
			return Err(ValidationError::InvalidLength {
				expected: 33,
				actual: value.len(),
			});
		}
		if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
			return Err(ValidationError::InvalidCharacters);
		}
		Ok(Self(value.to_string()))
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for MeLoId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

/// Luhn check digit calculation (modulus 10, weights 1 and 2 alternating from right)
fn luhn_check_digit(digits: &str) -> char {
	let sum: u32 = digits
		.chars()
		.rev()
		.enumerate()
		.map(|(i, c)| {
			let d = c.to_digit(10).unwrap();
			if i % 2 == 0 {
				let doubled = d * 2;
				if doubled > 9 { doubled - 9 } else { doubled }
			} else {
				d
			}
		})
		.sum();
	let check = (10 - (sum % 10)) % 10;
	char::from_digit(check, 10).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn valid_malo_id() {
		// luhn_check_digit("5123869678") == '8', so valid ID is 51238696788
		let result = MaLoId::new("51238696788");
		assert!(result.is_ok());
		assert_eq!(result.unwrap().as_str(), "51238696788");
	}

	#[test]
	fn malo_id_wrong_length() {
		let result = MaLoId::new("1234");
		assert_eq!(
			result,
			Err(ValidationError::InvalidLength {
				expected: 11,
				actual: 4
			})
		);
	}

	#[test]
	fn malo_id_non_digit() {
		let result = MaLoId::new("5123869678A");
		assert_eq!(result, Err(ValidationError::InvalidCharacters));
	}

	#[test]
	fn malo_id_wrong_check_digit() {
		// correct check digit is '8', so '2' should fail
		let result = MaLoId::new("51238696782");
		assert!(matches!(result, Err(ValidationError::InvalidCheckDigit { .. })));
	}

	#[test]
	fn valid_marktpartner_id() {
		let result = MarktpartnerId::new("9900000000003");
		assert!(result.is_ok());
	}

	#[test]
	fn marktpartner_id_wrong_length() {
		let result = MarktpartnerId::new("12345");
		assert_eq!(
			result,
			Err(ValidationError::InvalidLength {
				expected: 13,
				actual: 5
			})
		);
	}

	#[test]
	fn valid_melo_id() {
		let id = "DE000000000000000000000000000000A";
		let result = MeLoId::new(id);
		assert!(result.is_ok());
	}

	#[test]
	fn melo_id_wrong_length() {
		let result = MeLoId::new("DE00");
		assert_eq!(
			result,
			Err(ValidationError::InvalidLength {
				expected: 33,
				actual: 4
			})
		);
	}

	#[test]
	fn luhn_known_values() {
		assert_eq!(luhn_check_digit("5123869678"), '8');
	}
}
