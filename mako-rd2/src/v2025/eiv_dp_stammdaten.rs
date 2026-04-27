//! Stammdatenmeldung EIV → DP → ANB (FB Stammdaten 1.4a, DocType Z02/Z03).
//!
//! Der Einsatzverantwortliche bündelt die Stammdaten der ihm vertraglich
//! zugeordneten technischen Ressourcen (kommen i. d. R. zuvor per
//! `rd2_btr_eiv_stammdaten` von den BTRs) und meldet sie dem Data Provider.
//! Der DP konsolidiert die Daten und leitet sie an den Anschlussnetzbetreiber
//! weiter, damit dort Engpass- und Abrufbearbeitung darauf aufsetzen kann.

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::RdStammdaten;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle::*;

#[derive(Debug, Clone, PartialEq)]
pub enum EivDpStammdatenState {
	Idle,
	GesendetVomEiv {
		ressource_id: String,
	},
	WeitergeleitetAnAnb {
		ressource_id: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum EivDpStammdatenEvent {
	StammdatenAnDpGesendet(RdStammdaten),
	DpAnAnbWeitergeleitet(RdStammdaten),
}

pub fn reduce(
	state: EivDpStammdatenState,
	event: EivDpStammdatenEvent,
) -> Result<ReducerOutput<EivDpStammdatenState>, ProzessFehler> {
	match (state, event) {
		(EivDpStammdatenState::Idle, EivDpStammdatenEvent::StammdatenAnDpGesendet(sd)) => {
			// MP-IDs entsprechen mako-cli/src/init.rs::ROLLEN — Index 18 = EIV, 20 = DP.
			let absender = MarktpartnerId::new("9900000000018").expect("valid id");
			let empfaenger = MarktpartnerId::new("9900000000020").expect("valid id");
			let nachricht = Nachricht {
				absender,
				absender_rolle: Einsatzverantwortlicher,
				empfaenger,
				empfaenger_rolle: DataProvider,
				pruef_id: None,
				payload: NachrichtenPayload::RdStammdaten(sd.clone()),
			};
			Ok(ReducerOutput {
				state: EivDpStammdatenState::GesendetVomEiv {
					ressource_id: sd.ressource_id,
				},
				nachrichten: vec![nachricht],
			})
		}

		(
			EivDpStammdatenState::GesendetVomEiv { ressource_id },
			EivDpStammdatenEvent::DpAnAnbWeitergeleitet(sd),
		) => {
			// MP-IDs: Index 20 = DP, 15 = ANB.
			let absender = MarktpartnerId::new("9900000000020").expect("valid id");
			let empfaenger = MarktpartnerId::new("9900000000015").expect("valid id");
			let nachricht = Nachricht {
				absender,
				absender_rolle: DataProvider,
				empfaenger,
				empfaenger_rolle: Anschlussnetzbetreiber,
				pruef_id: None,
				payload: NachrichtenPayload::RdStammdaten(sd),
			};
			Ok(ReducerOutput {
				state: EivDpStammdatenState::WeitergeleitetAnAnb { ressource_id },
				nachrichten: vec![nachricht],
			})
		}

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}

#[cfg(test)]
mod tests {
	use mako_types::gpke_nachrichten::RessourceTyp;
	use mako_types::ids::MaLoId;

	use super::*;

	fn sd() -> RdStammdaten {
		RdStammdaten {
			ressource_id: "TR-WIND-001".to_string(),
			ressource_typ: RessourceTyp::TechnischeRessource,
			standort_malo: MaLoId::new("51238696788").unwrap(),
			installierte_leistung_kw: 3000.0,
		}
	}

	#[test]
	fn happy_path_eiv_dp_anb() {
		let out = reduce(
			EivDpStammdatenState::Idle,
			EivDpStammdatenEvent::StammdatenAnDpGesendet(sd()),
		)
		.expect("step 1");
		let msg = out.nachrichten.first().expect("wire 1");
		assert_eq!(msg.absender_rolle, Einsatzverantwortlicher);
		assert_eq!(msg.empfaenger_rolle, DataProvider);

		let out = reduce(
			out.state,
			EivDpStammdatenEvent::DpAnAnbWeitergeleitet(sd()),
		)
		.expect("step 2");
		let msg = out.nachrichten.first().expect("wire 2");
		assert_eq!(msg.absender_rolle, DataProvider);
		assert_eq!(msg.empfaenger_rolle, Anschlussnetzbetreiber);
	}

	#[test]
	fn idle_kann_keine_weiterleitung() {
		let result = reduce(
			EivDpStammdatenState::Idle,
			EivDpStammdatenEvent::DpAnAnbWeitergeleitet(sd()),
		);
		assert!(matches!(
			result,
			Err(ProzessFehler::UngueltigerUebergang { .. })
		));
	}
}
