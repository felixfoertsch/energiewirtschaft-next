use chrono::{Datelike, Days, NaiveDate, Weekday};
use mako_types::sparte::Sparte;

use crate::feiertage::Feiertagskalender;

/// Returns true if the given date is a working day (Mon-Fri, not a holiday).
pub fn ist_werktag(datum: NaiveDate, kalender: &Feiertagskalender) -> bool {
	let wochentag = datum.weekday();
	wochentag != Weekday::Sat && wochentag != Weekday::Sun && !kalender.ist_feiertag(datum)
}

/// Advance `werktage` working days from `datum`, skipping weekends and holidays.
pub fn frist(
	datum: NaiveDate,
	werktage: u32,
	kalender: &Feiertagskalender,
	_sparte: Sparte,
) -> NaiveDate {
	let mut aktuell = datum;
	let mut verbleibend = werktage;
	while verbleibend > 0 {
		aktuell = aktuell + Days::new(1);
		if ist_werktag(aktuell, kalender) {
			verbleibend -= 1;
		}
	}
	aktuell
}

#[cfg(test)]
mod tests {
	use super::*;

	fn kalender_2025() -> Feiertagskalender {
		Feiertagskalender::bundesweit(2025)
	}

	#[test]
	fn ein_werktag_von_montag_ist_dienstag() {
		let k = kalender_2025();
		// 2025-03-03 is Monday
		let montag = NaiveDate::from_ymd_opt(2025, 3, 3).unwrap();
		let ergebnis = frist(montag, 1, &k, Sparte::Strom);
		assert_eq!(ergebnis, NaiveDate::from_ymd_opt(2025, 3, 4).unwrap());
	}

	#[test]
	fn ein_werktag_von_freitag_ist_montag() {
		let k = kalender_2025();
		// 2025-03-07 is Friday
		let freitag = NaiveDate::from_ymd_opt(2025, 3, 7).unwrap();
		let ergebnis = frist(freitag, 1, &k, Sparte::Strom);
		assert_eq!(ergebnis, NaiveDate::from_ymd_opt(2025, 3, 10).unwrap());
	}

	#[test]
	fn ein_werktag_von_donnerstag_vor_ostern_ist_dienstag() {
		let k = kalender_2025();
		// 2025-04-17 is Thursday before Easter (Karfreitag=18, Sat=19, Ostersonntag=20, Ostermontag=21)
		let donnerstag = NaiveDate::from_ymd_opt(2025, 4, 17).unwrap();
		let ergebnis = frist(donnerstag, 1, &k, Sparte::Strom);
		assert_eq!(ergebnis, NaiveDate::from_ymd_opt(2025, 4, 22).unwrap());
	}

	#[test]
	fn ec7_weihnachten_2025() {
		let k = kalender_2025();
		// 2025-12-24 is Wednesday, 25+26 are holidays (Thu+Fri), 27+28 weekend
		let heiligabend = NaiveDate::from_ymd_opt(2025, 12, 24).unwrap();
		let ergebnis = frist(heiligabend, 1, &k, Sparte::Strom);
		assert_eq!(ergebnis, NaiveDate::from_ymd_opt(2025, 12, 29).unwrap());
	}

	#[test]
	fn drei_werktage_von_montag_ist_donnerstag() {
		let k = kalender_2025();
		let montag = NaiveDate::from_ymd_opt(2025, 3, 3).unwrap();
		let ergebnis = frist(montag, 3, &k, Sparte::Strom);
		assert_eq!(ergebnis, NaiveDate::from_ymd_opt(2025, 3, 6).unwrap());
	}
}
