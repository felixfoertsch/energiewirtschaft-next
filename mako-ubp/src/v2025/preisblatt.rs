use chrono::NaiveDate;
use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{PricatPreisblatt, PreisPosition};
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// UBP 3.3 Preisblatt-Veröffentlichung states
#[derive(Debug, Clone, PartialEq)]
pub enum PreisblattState {
	Idle,
	Veroeffentlicht {
		herausgeber: MarktpartnerId,
		gueltig_ab: NaiveDate,
		positionen: Vec<PreisPosition>,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreisblattEvent {
	Veroeffentlichen(PricatPreisblatt),
}

pub fn reduce(
	state: PreisblattState,
	event: PreisblattEvent,
) -> Result<ReducerOutput<PreisblattState>, ProzessFehler> {
	match (state, event) {
		// 3.3.1: Idle + Veröffentlichen -> Veröffentlicht
		(PreisblattState::Idle, PreisblattEvent::Veroeffentlichen(pricat)) => {
			if pricat.positionen.is_empty() {
				return Err(ProzessFehler::Validierungsfehler(
					"Preisblatt muss mindestens eine Position enthalten".to_string(),
				));
			}
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let nachricht = Nachricht {
				absender: pricat.herausgeber.clone(),
				absender_rolle: MarktRolle::Messstellenbetreiber,
				empfaenger: nb,
				empfaenger_rolle: MarktRolle::Netzbetreiber,
				payload: NachrichtenPayload::PricatPreisblatt(PricatPreisblatt {
					herausgeber: pricat.herausgeber.clone(),
					gueltig_ab: pricat.gueltig_ab,
					positionen: pricat.positionen.clone(),
				}),
			};
			Ok(ReducerOutput {
				state: PreisblattState::Veroeffentlicht {
					herausgeber: pricat.herausgeber,
					gueltig_ab: pricat.gueltig_ab,
					positionen: pricat.positionen,
				},
				nachrichten: vec![nachricht],
			})
		}

		// Catch-all: cannot publish again from Veröffentlicht
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
