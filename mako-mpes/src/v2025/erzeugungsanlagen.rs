use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{MsconsEinspeiseMesswerte, UtilmdAnmeldungErzeugung};
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;
use mako_types::rolle::MarktRolle::*;

pub const ERZEUGUNGSANLAGEN_ROLLENTUPEL: &[(MarktRolle, MarktRolle)] = &[
	(Netzbetreiber, BetreiberErzeugungsanlage),
	(BetreiberErzeugungsanlage, Direktvermarkter),
	(Direktvermarkter, BetreiberErzeugungsanlage),
	(Lieferant, Netzbetreiber),
	(Netzbetreiber, Lieferant),
	(Netzbetreiber, Uebertragungsnetzbetreiber),
	(Uebertragungsnetzbetreiber, Netzbetreiber),
	(Messstellenbetreiber, Netzbetreiber),
];

#[derive(Debug, Clone, PartialEq)]
pub enum ErzeugungsanlagenState {
	/// No active process
	Idle,
	/// MPES 5.1: Anmeldung received by NB
	AnmeldungEingegangen {
		malo: MaLoId,
		anlagenbetreiber: MarktpartnerId,
		nb: MarktpartnerId,
		eeg_anlage: bool,
		installierte_leistung_kw: f64,
	},
	/// MPES 5.2: NB has confirmed the registration
	Bestaetigt {
		malo: MaLoId,
		anlagenbetreiber: MarktpartnerId,
		nb: MarktpartnerId,
	},
	/// MPES 5.3: Zuordnung information sent to relevant parties
	ZuordnungInformiert {
		malo: MaLoId,
		anlagenbetreiber: MarktpartnerId,
		nb: MarktpartnerId,
	},
	/// MPES 5.4-5.6: Einspeise-Messwerte flow is active
	MesswerteAktiv {
		malo: MaLoId,
		anlagenbetreiber: MarktpartnerId,
	},
	/// Terminal failure
	Abgelehnt { malo: MaLoId, grund: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErzeugungsanlagenEvent {
	AnmeldungEmpfangen(UtilmdAnmeldungErzeugung),
	Bestaetigt,
	Abgelehnt { grund: String },
	ZuordnungInformiert,
	EinspeiseMesswerteEmpfangen(MsconsEinspeiseMesswerte),
}

pub fn reduce(
	state: ErzeugungsanlagenState,
	event: ErzeugungsanlagenEvent,
) -> Result<ReducerOutput<ErzeugungsanlagenState>, ProzessFehler> {
	match (state, event) {
		// 1. Idle + AnmeldungEmpfangen → AnmeldungEingegangen
		(ErzeugungsanlagenState::Idle, ErzeugungsanlagenEvent::AnmeldungEmpfangen(a)) => {
			let nb = MarktpartnerId::new("9900000000010").expect("valid NB id");
			Ok(ReducerOutput {
				state: ErzeugungsanlagenState::AnmeldungEingegangen {
					malo: a.malo_id,
					anlagenbetreiber: a.anlagenbetreiber,
					nb,
					eeg_anlage: a.eeg_anlage,
					installierte_leistung_kw: a.installierte_leistung_kw,
				},
				nachrichten: vec![],
			})
		}

		// 2. AnmeldungEingegangen + Bestaetigt → Bestaetigt
		(
			ErzeugungsanlagenState::AnmeldungEingegangen {
				malo,
				anlagenbetreiber,
				nb,
				..
			},
			ErzeugungsanlagenEvent::Bestaetigt,
		) => {
			let (absender_rolle, empfaenger_rolle) = ERZEUGUNGSANLAGEN_ROLLENTUPEL[0];
			let nachricht = Nachricht {
				absender: nb.clone(),
				absender_rolle,
				empfaenger: anlagenbetreiber.clone(),
				empfaenger_rolle,
				pruef_id: None,
				payload: NachrichtenPayload::UtilmdAnmeldungErzeugung(UtilmdAnmeldungErzeugung {
					malo_id: malo.clone(),
					anlagenbetreiber: anlagenbetreiber.clone(),
					eeg_anlage: true,
					installierte_leistung_kw: 0.0,
				}),
			};
			Ok(ReducerOutput {
				state: ErzeugungsanlagenState::Bestaetigt {
					malo,
					anlagenbetreiber,
					nb,
				},
				nachrichten: vec![nachricht],
			})
		}

		// 2b. AnmeldungEingegangen + Abgelehnt → Abgelehnt
		(
			ErzeugungsanlagenState::AnmeldungEingegangen { malo, .. },
			ErzeugungsanlagenEvent::Abgelehnt { grund },
		) => Ok(ReducerOutput {
			state: ErzeugungsanlagenState::Abgelehnt { malo, grund },
			nachrichten: vec![],
		}),

		// 3. Bestaetigt + ZuordnungInformiert → ZuordnungInformiert
		(
			ErzeugungsanlagenState::Bestaetigt {
				malo,
				anlagenbetreiber,
				nb,
			},
			ErzeugungsanlagenEvent::ZuordnungInformiert,
		) => Ok(ReducerOutput {
			state: ErzeugungsanlagenState::ZuordnungInformiert {
				malo,
				anlagenbetreiber,
				nb,
			},
			nachrichten: vec![],
		}),

		// 4. ZuordnungInformiert + EinspeiseMesswerteEmpfangen → MesswerteAktiv
		(
			ErzeugungsanlagenState::ZuordnungInformiert {
				malo,
				anlagenbetreiber,
				nb,
			},
			ErzeugungsanlagenEvent::EinspeiseMesswerteEmpfangen(m),
		) => {
			let msb = MarktpartnerId::new("9900000000027").expect("valid MSB id");
			let (absender_rolle, empfaenger_rolle) = ERZEUGUNGSANLAGEN_ROLLENTUPEL[7];
			let nachricht = Nachricht {
				absender: msb,
				absender_rolle,
				empfaenger: nb,
				empfaenger_rolle,
				pruef_id: None,
				payload: NachrichtenPayload::MsconsEinspeiseMesswerte(m),
			};
			Ok(ReducerOutput {
				state: ErzeugungsanlagenState::MesswerteAktiv {
					malo,
					anlagenbetreiber,
				},
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
