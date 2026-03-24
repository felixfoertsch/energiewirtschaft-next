use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	ClsSteuersignal, MsconsEinspeiseMesswerte, UtilmdSteuerbareVerbrauchseinrichtung,
};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

#[derive(Debug, Clone, PartialEq)]
pub enum SteuerungState {
	/// No active process
	Idle,
	/// §14a 8.1: Steuerbare Verbrauchseinrichtung registered
	Angemeldet {
		malo: MaLoId,
		anmelder: MarktpartnerId,
		nb: MarktpartnerId,
	},
	/// §14a 8.2-8.3: CLS configuration completed
	Konfiguriert {
		malo: MaLoId,
		anmelder: MarktpartnerId,
		nb: MarktpartnerId,
	},
	/// §14a 8.4: CLS channel active, ready for control signals
	Aktiv {
		malo: MaLoId,
		anmelder: MarktpartnerId,
		nb: MarktpartnerId,
	},
	/// §14a 8.4: A control signal has been applied
	Gesteuert {
		malo: MaLoId,
		nb: MarktpartnerId,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum SteuerungEvent {
	AnmeldungEmpfangen(UtilmdSteuerbareVerbrauchseinrichtung),
	KonfigurationGesendet,
	SteuersignalGesendet(ClsSteuersignal),
	MesswerteEmpfangen(MsconsEinspeiseMesswerte),
}

pub fn reduce(
	state: SteuerungState,
	event: SteuerungEvent,
) -> Result<ReducerOutput<SteuerungState>, ProzessFehler> {
	match (state, event) {
		// 1. Idle + AnmeldungEmpfangen → Angemeldet
		(SteuerungState::Idle, SteuerungEvent::AnmeldungEmpfangen(a)) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			let anmelder = MarktpartnerId::new("9900000000003").expect("valid id");
			let nachricht = Nachricht {
				absender: anmelder.clone(),
				absender_rolle: MarktRolle::Lieferant,
				empfaenger: nb.clone(),
				empfaenger_rolle: MarktRolle::Netzbetreiber,
				payload: NachrichtenPayload::UtilmdSteuerbareVerbrauchseinrichtung(a.clone()),
			};
			Ok(ReducerOutput {
				state: SteuerungState::Angemeldet {
					malo: a.malo_id,
					anmelder,
					nb,
				},
				nachrichten: vec![nachricht],
			})
		}

		// 2. Angemeldet + KonfigurationGesendet → Konfiguriert
		(
			SteuerungState::Angemeldet { malo, anmelder, nb },
			SteuerungEvent::KonfigurationGesendet,
		) => Ok(ReducerOutput {
			state: SteuerungState::Konfiguriert { malo, anmelder, nb },
			nachrichten: vec![],
		}),

		// 3. Konfiguriert + SteuersignalGesendet → Aktiv
		(
			SteuerungState::Konfiguriert { malo, anmelder, nb },
			SteuerungEvent::SteuersignalGesendet(signal),
		) => {
			let nachricht = Nachricht {
				absender: nb.clone(),
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: anmelder.clone(),
				empfaenger_rolle: MarktRolle::Lieferant,
				payload: NachrichtenPayload::ClsSteuersignal(signal),
			};
			Ok(ReducerOutput {
				state: SteuerungState::Aktiv { malo, anmelder, nb },
				nachrichten: vec![nachricht],
			})
		}

		// 4. Aktiv + MesswerteEmpfangen → Gesteuert
		(
			SteuerungState::Aktiv { malo, nb, .. },
			SteuerungEvent::MesswerteEmpfangen(m),
		) => {
			let nachricht = Nachricht {
				absender: nb.clone(),
				absender_rolle: MarktRolle::Netzbetreiber,
				empfaenger: nb.clone(),
				empfaenger_rolle: MarktRolle::Netzbetreiber,
				payload: NachrichtenPayload::MsconsEinspeiseMesswerte(m),
			};
			Ok(ReducerOutput {
				state: SteuerungState::Gesteuert { malo, nb },
				nachrichten: vec![nachricht],
			})
		}

		// Catch-all: invalid transition
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
