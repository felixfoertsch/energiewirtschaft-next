use chrono::{Days, NaiveDate};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Feiertagskalender {
	feiertage: BTreeSet<NaiveDate>,
}

impl Feiertagskalender {
	pub fn new(feiertage: Vec<NaiveDate>) -> Self {
		Self {
			feiertage: feiertage.into_iter().collect(),
		}
	}

	pub fn ist_feiertag(&self, datum: NaiveDate) -> bool {
		self.feiertage.contains(&datum)
	}

	/// German federal holidays for a given year.
	pub fn bundesweit(jahr: i32) -> Self {
		let mut tage = Vec::new();

		// Fixed holidays
		tage.push(NaiveDate::from_ymd_opt(jahr, 1, 1).unwrap()); // Neujahr
		tage.push(NaiveDate::from_ymd_opt(jahr, 5, 1).unwrap()); // Tag der Arbeit
		tage.push(NaiveDate::from_ymd_opt(jahr, 10, 3).unwrap()); // Tag der Deutschen Einheit
		tage.push(NaiveDate::from_ymd_opt(jahr, 10, 31).unwrap()); // Reformationstag
		tage.push(NaiveDate::from_ymd_opt(jahr, 12, 25).unwrap()); // 1. Weihnachtstag
		tage.push(NaiveDate::from_ymd_opt(jahr, 12, 26).unwrap()); // 2. Weihnachtstag

		// Easter-based holidays
		let ostern = ostersonntag(jahr);
		tage.push(ostern - Days::new(2)); // Karfreitag
		tage.push(ostern); // Ostersonntag
		tage.push(ostern + Days::new(1)); // Ostermontag
		tage.push(ostern + Days::new(39)); // Christi Himmelfahrt
		tage.push(ostern + Days::new(49)); // Pfingstsonntag
		tage.push(ostern + Days::new(50)); // Pfingstmontag

		Self::new(tage)
	}
}

/// Anonymous Gregorian Easter algorithm.
fn ostersonntag(jahr: i32) -> NaiveDate {
	let a = jahr % 19;
	let b = jahr / 100;
	let c = jahr % 100;
	let d = b / 4;
	let e = b % 4;
	let f = (b + 8) / 25;
	let g = (b - f + 1) / 3;
	let h = (19 * a + b - d - g + 15) % 30;
	let i = c / 4;
	let k = c % 4;
	let l = (32 + 2 * e + 2 * i - h - k) % 7;
	let m = (a + 11 * h + 22 * l) / 451;
	let month = (h + l - 7 * m + 114) / 31;
	let day = (h + l - 7 * m + 114) % 31 + 1;
	NaiveDate::from_ymd_opt(jahr, month as u32, day as u32).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ostern_2025() {
		assert_eq!(ostersonntag(2025), NaiveDate::from_ymd_opt(2025, 4, 20).unwrap());
	}

	#[test]
	fn ostern_2026() {
		assert_eq!(ostersonntag(2026), NaiveDate::from_ymd_opt(2026, 4, 5).unwrap());
	}

	#[test]
	fn karfreitag_2025_ist_feiertag() {
		let kalender = Feiertagskalender::bundesweit(2025);
		let karfreitag = NaiveDate::from_ymd_opt(2025, 4, 18).unwrap();
		assert!(kalender.ist_feiertag(karfreitag));
	}

	#[test]
	fn normaler_montag_ist_kein_feiertag() {
		let kalender = Feiertagskalender::bundesweit(2025);
		let montag = NaiveDate::from_ymd_opt(2025, 3, 3).unwrap();
		assert!(!kalender.ist_feiertag(montag));
	}

	#[test]
	fn neujahr_ist_feiertag() {
		let kalender = Feiertagskalender::bundesweit(2025);
		let neujahr = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
		assert!(kalender.ist_feiertag(neujahr));
	}

	#[test]
	fn weihnachten_ist_feiertag() {
		let kalender = Feiertagskalender::bundesweit(2025);
		assert!(kalender.ist_feiertag(NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()));
		assert!(kalender.ist_feiertag(NaiveDate::from_ymd_opt(2025, 12, 26).unwrap()));
	}
}
