use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{
	InvoicRechnung, MsconsMehrMindermengen, RemadvZahlungsavis,
};
use mako_types::ids::MaLoId;
use mako_types::reducer::ReducerOutput;

/// GABi Gas 4.4: Mehr-/Mindermengen (Gas)
/// Same pattern as MaBiS.
/// Idle -> ListeGesendet -> RechnungGesendet -> ZahlungsavisEmpfangen
#[derive(Debug, Clone, PartialEq)]
pub enum MehrMindermengenState {
	Idle,
	ListeGesendet {
		malo: MaLoId,
		mehrmenge_kwh: f64,
		mindermenge_kwh: f64,
		zeitraum_von: NaiveDate,
		zeitraum_bis: NaiveDate,
	},
	RechnungGesendet {
		malo: MaLoId,
		rechnungsnummer: String,
	},
	ZahlungsavisEmpfangen {
		malo: MaLoId,
		rechnungsnummer: String,
		akzeptiert: bool,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum MehrMindermengenEvent {
	ListeEmpfangen(MsconsMehrMindermengen),
	RechnungGesendet(InvoicRechnung),
	ZahlungsavisEmpfangen(RemadvZahlungsavis),
}

pub fn reduce(
	state: MehrMindermengenState,
	event: MehrMindermengenEvent,
) -> Result<ReducerOutput<MehrMindermengenState>, ProzessFehler> {
	match (state, event) {
		(
			MehrMindermengenState::Idle,
			MehrMindermengenEvent::ListeEmpfangen(liste),
		) => Ok(ReducerOutput {
			state: MehrMindermengenState::ListeGesendet {
				malo: liste.malo_id,
				mehrmenge_kwh: liste.mehrmenge_kwh,
				mindermenge_kwh: liste.mindermenge_kwh,
				zeitraum_von: liste.abrechnungszeitraum_von,
				zeitraum_bis: liste.abrechnungszeitraum_bis,
			},
			nachrichten: vec![],
		}),

		(
			MehrMindermengenState::ListeGesendet { malo, .. },
			MehrMindermengenEvent::RechnungGesendet(rechnung),
		) => Ok(ReducerOutput {
			state: MehrMindermengenState::RechnungGesendet {
				malo,
				rechnungsnummer: rechnung.rechnungsnummer,
			},
			nachrichten: vec![],
		}),

		(
			MehrMindermengenState::RechnungGesendet { malo, rechnungsnummer },
			MehrMindermengenEvent::ZahlungsavisEmpfangen(avis),
		) => Ok(ReducerOutput {
			state: MehrMindermengenState::ZahlungsavisEmpfangen {
				malo,
				rechnungsnummer,
				akzeptiert: avis.akzeptiert,
			},
			nachrichten: vec![],
		}),

		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
