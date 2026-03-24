use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::UtilmdBilanzkreiszuordnung;
use mako_types::ids::MaLoId;
use mako_types::reducer::ReducerOutput;

use chrono::NaiveDate;

/// MaBiS 4.1: Bilanzkreiszuordnung
/// Idle -> ZuordnungGesendet -> Bestaetigt / Abgelehnt
#[derive(Debug, Clone, PartialEq)]
pub enum BilanzkreiszuordnungState {
	Idle,
	ZuordnungGesendet {
		malo: MaLoId,
		bilanzkreis: String,
		gueltig_ab: NaiveDate,
	},
	Bestaetigt {
		malo: MaLoId,
		bilanzkreis: String,
	},
	Abgelehnt {
		malo: MaLoId,
		grund: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum BilanzkreiszuordnungEvent {
	ZuordnungEmpfangen(UtilmdBilanzkreiszuordnung),
	Bestaetigt,
	Abgelehnt { grund: String },
}

pub fn reduce(
	state: BilanzkreiszuordnungState,
	event: BilanzkreiszuordnungEvent,
) -> Result<ReducerOutput<BilanzkreiszuordnungState>, ProzessFehler> {
	match (state, event) {
		// Idle + ZuordnungEmpfangen -> ZuordnungGesendet
		(
			BilanzkreiszuordnungState::Idle,
			BilanzkreiszuordnungEvent::ZuordnungEmpfangen(z),
		) => Ok(ReducerOutput {
			state: BilanzkreiszuordnungState::ZuordnungGesendet {
				malo: z.malo_id,
				bilanzkreis: z.bilanzkreis,
				gueltig_ab: z.gueltig_ab,
			},
			nachrichten: vec![],
		}),

		// ZuordnungGesendet + Bestaetigt -> Bestaetigt
		(
			BilanzkreiszuordnungState::ZuordnungGesendet { malo, bilanzkreis, .. },
			BilanzkreiszuordnungEvent::Bestaetigt,
		) => Ok(ReducerOutput {
			state: BilanzkreiszuordnungState::Bestaetigt { malo, bilanzkreis },
			nachrichten: vec![],
		}),

		// ZuordnungGesendet + Abgelehnt -> Abgelehnt
		(
			BilanzkreiszuordnungState::ZuordnungGesendet { malo, .. },
			BilanzkreiszuordnungEvent::Abgelehnt { grund },
		) => Ok(ReducerOutput {
			state: BilanzkreiszuordnungState::Abgelehnt { malo, grund },
			nachrichten: vec![],
		}),

		// Catch-all: invalid transition
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
