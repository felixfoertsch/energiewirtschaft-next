use chrono::NaiveDate;

use mako_types::gpke_nachrichten::{
	MsconsLastgang, MsconsSchlussturnusmesswert, Messwert, MesswertStatus,
};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::ids::MaLoId;
use mako_types::rolle::MarktRolle;

use crate::ids::test_mp_id;

/// MSCONS Schlussturnusmesswert: NB -> LF
pub fn schlussturnusmesswert(malo_id: MaLoId, zaehlerstand: f64, stichtag: NaiveDate) -> Nachricht {
	let nb = test_mp_id(2);
	let lf = test_mp_id(1);
	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lf,
		empfaenger_rolle: MarktRolle::Lieferant,
		payload: NachrichtenPayload::MsconsSchlussturnusmesswert(MsconsSchlussturnusmesswert {
			malo_id,
			zaehlerstand,
			stichtag,
			einheit: "kWh".to_string(),
		}),
	}
}

/// MSCONS Lastgang (15-min intervals): MSB -> NB
///
/// Generates `days * 96` quarter-hour readings starting at midnight of `start_date`.
/// Each value oscillates around `base_kwh` using a simple sine pattern.
pub fn lastgang_15min(malo_id: MaLoId, start_date: NaiveDate, days: u32, base_kwh: f64) -> Nachricht {
	let msb = test_mp_id(3);
	let nb = test_mp_id(2);

	let intervals_per_day: u32 = 96;
	let total = days * intervals_per_day;
	let mut werte = Vec::with_capacity(total as usize);

	for i in 0..total {
		let minutes = i * 15;
		let zeitpunkt = start_date
			.and_hms_opt(0, 0, 0)
			.expect("valid midnight")
			+ chrono::Duration::minutes(minutes as i64);

		// Sine-based variation: +/- 20% of base
		let phase = (i as f64) / (intervals_per_day as f64) * std::f64::consts::TAU;
		let wert = base_kwh * (1.0 + 0.2 * phase.sin());

		werte.push(Messwert {
			zeitpunkt,
			wert,
			einheit: "kWh".to_string(),
			status: MesswertStatus::Gemessen,
		});
	}

	Nachricht {
		absender: msb,
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		payload: NachrichtenPayload::MsconsLastgang(MsconsLastgang {
			malo_id,
			werte,
			intervall_minuten: 15,
		}),
	}
}

/// MSCONS Zaehlerstand: MSB -> NB
pub fn zaehlerstand(malo_id: MaLoId, stand: f64, datum: NaiveDate) -> Nachricht {
	let msb = test_mp_id(3);
	let nb = test_mp_id(2);
	Nachricht {
		absender: msb,
		absender_rolle: MarktRolle::Messstellenbetreiber,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		payload: NachrichtenPayload::MsconsSchlussturnusmesswert(MsconsSchlussturnusmesswert {
			malo_id,
			zaehlerstand: stand,
			stichtag: datum,
			einheit: "kWh".to_string(),
		}),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ids::test_malo;

	#[test]
	fn schlussturnusmesswert_generates_valid_message() {
		let malo = test_malo(0);
		let stichtag = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
		let msg = schlussturnusmesswert(malo.clone(), 12345.6, stichtag);

		assert_eq!(msg.absender_rolle, MarktRolle::Netzbetreiber);
		assert_eq!(msg.empfaenger_rolle, MarktRolle::Lieferant);
		match &msg.payload {
			NachrichtenPayload::MsconsSchlussturnusmesswert(m) => {
				assert_eq!(m.malo_id, malo);
				assert!((m.zaehlerstand - 12345.6).abs() < f64::EPSILON);
				assert_eq!(m.stichtag, stichtag);
				assert_eq!(m.einheit, "kWh");
			}
			other => panic!("expected MsconsSchlussturnusmesswert, got {other:?}"),
		}
	}

	#[test]
	fn lastgang_generates_correct_number_of_intervals() {
		let malo = test_malo(1);
		let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
		let days = 3;
		let msg = lastgang_15min(malo, start, days, 0.5);

		match &msg.payload {
			NachrichtenPayload::MsconsLastgang(lg) => {
				// 96 intervals per day * 3 days = 288
				assert_eq!(lg.werte.len(), 96 * days as usize);
				assert_eq!(lg.intervall_minuten, 15);

				// First value starts at midnight
				let first = &lg.werte[0];
				assert_eq!(first.zeitpunkt, start.and_hms_opt(0, 0, 0).unwrap());

				// Last value starts at 23:45 on day 3
				let last = &lg.werte[lg.werte.len() - 1];
				let expected_last = NaiveDate::from_ymd_opt(2025, 1, 3)
					.unwrap()
					.and_hms_opt(23, 45, 0)
					.unwrap();
				assert_eq!(last.zeitpunkt, expected_last);

				// All values are positive
				assert!(lg.werte.iter().all(|w| w.wert > 0.0));
			}
			other => panic!("expected MsconsLastgang, got {other:?}"),
		}

		// Roles: MSB -> NB
		assert_eq!(msg.absender_rolle, MarktRolle::Messstellenbetreiber);
		assert_eq!(msg.empfaenger_rolle, MarktRolle::Netzbetreiber);
	}

	#[test]
	fn zaehlerstand_generates_valid_message() {
		let malo = test_malo(2);
		let datum = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
		let msg = zaehlerstand(malo.clone(), 99999.9, datum);

		assert_eq!(msg.absender_rolle, MarktRolle::Messstellenbetreiber);
		assert_eq!(msg.empfaenger_rolle, MarktRolle::Netzbetreiber);
		match &msg.payload {
			NachrichtenPayload::MsconsSchlussturnusmesswert(m) => {
				assert_eq!(m.malo_id, malo);
				assert!((m.zaehlerstand - 99999.9).abs() < f64::EPSILON);
				assert_eq!(m.stichtag, datum);
				assert_eq!(m.einheit, "kWh");
			}
			other => panic!("expected MsconsSchlussturnusmesswert, got {other:?}"),
		}
	}
}
