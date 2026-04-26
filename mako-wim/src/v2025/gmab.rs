use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::UtilmdGeschaeftsdatenanfrage;
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle::*;

/// WiM: gMSB-Inanspruchnahme
/// Lieferant oder Netzbetreiber beauftragen den grundzuständigen Messstellenbetreiber.
/// Idle → Beauftragt → Abgeschlossen
#[derive(Debug, Clone, PartialEq)]
pub enum GmabState {
	Idle,
	Beauftragt {
		malo: MaLoId,
		auftraggeber: MarktpartnerId,
	},
	Abgeschlossen {
		malo: MaLoId,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum GmabEvent {
	BeauftragungGesendet(UtilmdGeschaeftsdatenanfrage),
	BestaetigenEmpfangen,
}

fn gmsb() -> MarktpartnerId {
	MarktpartnerId::new("9900000000029").expect("valid gMSB id")
}

pub fn reduce(
	state: GmabState,
	event: GmabEvent,
) -> Result<ReducerOutput<GmabState>, ProzessFehler> {
	match (state, event) {
		// Idle + BeauftragungGesendet → Beauftragt (LF/NB → gMSB)
		(GmabState::Idle, GmabEvent::BeauftragungGesendet(anfrage)) => {
			let nachricht = Nachricht {
				absender: anfrage.anfragender.clone(),
				absender_rolle: Lieferant,
				empfaenger: gmsb(),
				empfaenger_rolle: GrundzustaendigerMessstellenbetreiber,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdGeschaeftsdatenanfrage(
					UtilmdGeschaeftsdatenanfrage {
						malo_id: anfrage.malo_id.clone(),
						anfragender: anfrage.anfragender.clone(),
					},
				),
			};
			Ok(ReducerOutput {
				state: GmabState::Beauftragt {
					malo: anfrage.malo_id,
					auftraggeber: anfrage.anfragender,
				},
				nachrichten: vec![nachricht],
			})
		}

		// Beauftragt + BestaetigenEmpfangen → Abgeschlossen
		(GmabState::Beauftragt { malo, .. }, GmabEvent::BestaetigenEmpfangen) => {
			Ok(ReducerOutput {
				state: GmabState::Abgeschlossen { malo },
				nachrichten: vec![],
			})
		}

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
