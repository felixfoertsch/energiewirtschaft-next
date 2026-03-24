use mako_types::fehler::ProzessFehler;
use mako_types::reducer::ReducerOutput;

/// KoV 5.1: Kapazitätsbuchung (PRISMA-based)
/// Idle -> BuchungAngefragt -> Bestaetigt / Abgelehnt
#[derive(Debug, Clone, PartialEq)]
pub enum KapazitaetState {
	Idle,
	BuchungAngefragt {
		netzgebiet: String,
		kapazitaet_kwh_h: f64,
	},
	Bestaetigt {
		netzgebiet: String,
		kapazitaet_kwh_h: f64,
	},
	Abgelehnt {
		netzgebiet: String,
		grund: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum KapazitaetEvent {
	BuchungAngefragt {
		netzgebiet: String,
		kapazitaet_kwh_h: f64,
	},
	Bestaetigt,
	Abgelehnt { grund: String },
}

pub fn reduce(
	state: KapazitaetState,
	event: KapazitaetEvent,
) -> Result<ReducerOutput<KapazitaetState>, ProzessFehler> {
	match (state, event) {
		// 5.1.1: Idle + BuchungAngefragt -> BuchungAngefragt
		(
			KapazitaetState::Idle,
			KapazitaetEvent::BuchungAngefragt { netzgebiet, kapazitaet_kwh_h },
		) => Ok(ReducerOutput {
			state: KapazitaetState::BuchungAngefragt {
				netzgebiet,
				kapazitaet_kwh_h,
			},
			nachrichten: vec![],
		}),

		// 5.1.2: BuchungAngefragt + Bestaetigt -> Bestaetigt
		(
			KapazitaetState::BuchungAngefragt { netzgebiet, kapazitaet_kwh_h },
			KapazitaetEvent::Bestaetigt,
		) => Ok(ReducerOutput {
			state: KapazitaetState::Bestaetigt {
				netzgebiet,
				kapazitaet_kwh_h,
			},
			nachrichten: vec![],
		}),

		// 5.1.3: BuchungAngefragt + Abgelehnt -> Abgelehnt
		(
			KapazitaetState::BuchungAngefragt { netzgebiet, .. },
			KapazitaetEvent::Abgelehnt { grund },
		) => Ok(ReducerOutput {
			state: KapazitaetState::Abgelehnt { netzgebiet, grund },
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
