use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{AblehnungsGrund, Stammdatenfeld, UtilmdStammdatenaenderung};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::reducer::ReducerOutput;

/// GeLi Gas 2.3: Stammdatenänderung (Gas)
/// Same as GPKE stammdaten but for Gas MaLos.
#[derive(Debug, Clone, PartialEq)]
pub enum StammdatenState {
	Idle,
	AenderungGesendet {
		malo: MaLoId,
		initiator: MarktpartnerId,
		aenderungen: Vec<Stammdatenfeld>,
	},
	AenderungBestaetigt {
		malo: MaLoId,
	},
	Abgelehnt {
		malo: MaLoId,
		grund: AblehnungsGrund,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum StammdatenEvent {
	AenderungEingegangen(UtilmdStammdatenaenderung),
	AenderungBestaetigt,
	AenderungAbgelehnt { grund: AblehnungsGrund },
	FristUeberschritten,
}

fn malo_from_state(state: &StammdatenState) -> MaLoId {
	match state {
		StammdatenState::Idle => unreachable!("Idle has no MaLoId"),
		StammdatenState::AenderungGesendet { malo, .. }
		| StammdatenState::AenderungBestaetigt { malo, .. }
		| StammdatenState::Abgelehnt { malo, .. } => malo.clone(),
	}
}

pub fn reduce(
	state: StammdatenState,
	event: StammdatenEvent,
) -> Result<ReducerOutput<StammdatenState>, ProzessFehler> {
	match (state, event) {
		(StammdatenState::Idle, StammdatenEvent::AenderungEingegangen(a)) => Ok(ReducerOutput {
			state: StammdatenState::AenderungGesendet {
				malo: a.malo_id,
				initiator: a.initiator,
				aenderungen: a.aenderungen,
			},
			nachrichten: vec![],
		}),

		(
			StammdatenState::AenderungGesendet { malo, .. },
			StammdatenEvent::AenderungBestaetigt,
		) => Ok(ReducerOutput {
			state: StammdatenState::AenderungBestaetigt { malo },
			nachrichten: vec![],
		}),

		(
			StammdatenState::AenderungGesendet { malo, .. },
			StammdatenEvent::AenderungAbgelehnt { grund },
		) => Ok(ReducerOutput {
			state: StammdatenState::Abgelehnt { malo, grund },
			nachrichten: vec![],
		}),

		(
			ref s @ StammdatenState::AenderungGesendet { .. },
			StammdatenEvent::FristUeberschritten,
		) => {
			let malo = malo_from_state(s);
			Ok(ReducerOutput {
				state: StammdatenState::Abgelehnt {
					malo,
					grund: AblehnungsGrund::Fristverletzung,
				},
				nachrichten: vec![],
			})
		}

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
