use chrono::NaiveTime;
use mako_types::sparte::Sparte;

/// Returns the start-of-day time for the given Sparte.
/// Strom: 00:00, Gas: 06:00.
pub fn tagesbeginn(sparte: Sparte) -> NaiveTime {
	match sparte {
		Sparte::Strom => NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
		Sparte::Gas => NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
	}
}

/// Returns the number of hours in a day for the given Sparte.
/// Gas is always 24h. Strom defaults to 24h.
pub fn tag_stunden(sparte: Sparte, _ist_dst_umstellung: bool) -> u32 {
	match sparte {
		Sparte::Gas => 24,
		Sparte::Strom => 24,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn strom_beginnt_um_mitternacht() {
		assert_eq!(
			tagesbeginn(Sparte::Strom),
			NaiveTime::from_hms_opt(0, 0, 0).unwrap()
		);
	}

	#[test]
	fn gas_beginnt_um_sechs() {
		assert_eq!(
			tagesbeginn(Sparte::Gas),
			NaiveTime::from_hms_opt(6, 0, 0).unwrap()
		);
	}

	#[test]
	fn gastag_immer_24_stunden() {
		assert_eq!(tag_stunden(Sparte::Gas, false), 24);
		assert_eq!(tag_stunden(Sparte::Gas, true), 24);
	}
}
