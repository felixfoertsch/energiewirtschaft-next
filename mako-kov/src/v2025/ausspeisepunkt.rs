use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::UtilmdAusspeisepunkt;
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::reducer::ReducerOutput;

/// KoV 5.4: Ausspeisepunkt (NB -> FNB)
/// Idle -> AnmeldungGesendet -> Bestaetigt
#[derive(Debug, Clone, PartialEq)]
pub enum AusspeisepunktState {
	Idle,
	AnmeldungGesendet {
		malo: MaLoId,
		nb: MarktpartnerId,
		fnb: MarktpartnerId,
	},
	Bestaetigt {
		malo: MaLoId,
	},
	Abgelehnt {
		malo: MaLoId,
		grund: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum AusspeisepunktEvent {
	AnmeldungEingegangen(UtilmdAusspeisepunkt),
	Bestaetigt,
	Abgelehnt { grund: String },
}

pub fn reduce(
	state: AusspeisepunktState,
	event: AusspeisepunktEvent,
) -> Result<ReducerOutput<AusspeisepunktState>, ProzessFehler> {
	match (state, event) {
		// 5.4.1: Idle + AnmeldungEingegangen -> AnmeldungGesendet
		(AusspeisepunktState::Idle, AusspeisepunktEvent::AnmeldungEingegangen(a)) => {
			Ok(ReducerOutput {
				state: AusspeisepunktState::AnmeldungGesendet {
					malo: a.malo_id,
					nb: a.nb,
					fnb: a.fnb,
				},
				nachrichten: vec![],
			})
		}

		// 5.4.2: AnmeldungGesendet + Bestaetigt -> Bestaetigt
		(
			AusspeisepunktState::AnmeldungGesendet { malo, .. },
			AusspeisepunktEvent::Bestaetigt,
		) => Ok(ReducerOutput {
			state: AusspeisepunktState::Bestaetigt { malo },
			nachrichten: vec![],
		}),

		// 5.4.3: AnmeldungGesendet + Abgelehnt -> Abgelehnt
		(
			AusspeisepunktState::AnmeldungGesendet { malo, .. },
			AusspeisepunktEvent::Abgelehnt { grund },
		) => Ok(ReducerOutput {
			state: AusspeisepunktState::Abgelehnt { malo, grund },
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
