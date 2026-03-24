use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::MsconsBrennwert;
use mako_types::reducer::ReducerOutput;

/// KoV 5.3: Brennwertmitteilung
/// Idle -> BrennwertMitgeteilt (one-shot notification)
#[derive(Debug, Clone, PartialEq)]
pub enum BrennwertState {
	Idle,
	BrennwertMitgeteilt {
		netzgebiet: String,
		brennwert_kwh_per_m3: f64,
		zustandszahl: f64,
		gueltig_ab: NaiveDate,
		gueltig_bis: NaiveDate,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum BrennwertEvent {
	BrennwertMitgeteilt(MsconsBrennwert),
}

pub fn reduce(
	state: BrennwertState,
	event: BrennwertEvent,
) -> Result<ReducerOutput<BrennwertState>, ProzessFehler> {
	match (state, event) {
		// 5.3.1: Idle + BrennwertMitgeteilt -> BrennwertMitgeteilt
		(BrennwertState::Idle, BrennwertEvent::BrennwertMitgeteilt(b)) => Ok(ReducerOutput {
			state: BrennwertState::BrennwertMitgeteilt {
				netzgebiet: b.netzgebiet,
				brennwert_kwh_per_m3: b.brennwert_kwh_per_m3,
				zustandszahl: b.zustandszahl,
				gueltig_ab: b.gueltig_ab,
				gueltig_bis: b.gueltig_bis,
			},
			nachrichten: vec![],
		}),

		// Catch-all: invalid transition (BrennwertMitgeteilt is terminal)
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
