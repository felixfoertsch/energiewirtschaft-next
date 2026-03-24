use mako_types::ids::{MaLoId, MarktpartnerId, MeLoId};

/// Luhn check digit calculation (modulus 10, weights 1 and 2 alternating from right).
/// Duplicated from mako-types to avoid making it public there.
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

/// Generate a deterministic, valid 11-digit MaLo-ID for the given index.
pub fn test_malo(index: u8) -> MaLoId {
	let base = format!("{:010}", 5123869670u64 + index as u64);
	let check = luhn_check_digit(&base);
	let full = format!("{base}{check}");
	MaLoId::new(&full).expect("test_malo must produce a valid MaLoId")
}

/// Generate a deterministic, valid 13-digit Marktpartner-ID for the given index.
pub fn test_mp_id(index: u8) -> MarktpartnerId {
	let id = format!("{:013}", 9900000000000u64 + index as u64);
	MarktpartnerId::new(&id).expect("test_mp_id must produce a valid MarktpartnerId")
}

/// Generate a deterministic, valid 33-char MeLo-ID for the given index.
pub fn test_melo(index: u8) -> MeLoId {
	let id = format!("DE{:031}", index);
	MeLoId::new(&id).expect("test_melo must produce a valid MeLoId")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn generates_valid_malos() {
		for i in 0..10 {
			let id = test_malo(i);
			assert_eq!(id.as_str().len(), 11);
			// re-validate through constructor
			assert!(MaLoId::new(id.as_str()).is_ok());
		}
	}

	#[test]
	fn malo_is_deterministic() {
		assert_eq!(test_malo(3).as_str(), test_malo(3).as_str());
	}

	#[test]
	fn malo_different_indices_differ() {
		assert_ne!(test_malo(0).as_str(), test_malo(1).as_str());
	}

	#[test]
	fn generates_valid_mp_ids() {
		for i in 0..10 {
			let id = test_mp_id(i);
			assert_eq!(id.as_str().len(), 13);
			assert!(MarktpartnerId::new(id.as_str()).is_ok());
		}
	}

	#[test]
	fn mp_id_is_deterministic() {
		assert_eq!(test_mp_id(5).as_str(), test_mp_id(5).as_str());
	}

	#[test]
	fn mp_id_different_indices_differ() {
		assert_ne!(test_mp_id(0).as_str(), test_mp_id(1).as_str());
	}

	#[test]
	fn generates_valid_melos() {
		for i in 0..10 {
			let id = test_melo(i);
			assert_eq!(id.as_str().len(), 33);
			assert!(MeLoId::new(id.as_str()).is_ok());
		}
	}

	#[test]
	fn melo_is_deterministic() {
		assert_eq!(test_melo(7).as_str(), test_melo(7).as_str());
	}

	#[test]
	fn melo_different_indices_differ() {
		assert_ne!(test_melo(0).as_str(), test_melo(1).as_str());
	}
}
