use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{InvoicRechnung, RemadvZahlungsavis};
use mako_types::reducer::ReducerOutput;

/// KoV 5.2: Netzkontoabrechnung (INVOIC/REMADV)
/// Idle -> RechnungGesendet -> ZahlungsavisEmpfangen
#[derive(Debug, Clone, PartialEq)]
pub enum NetzkontoabrechnungState {
	Idle,
	RechnungGesendet {
		rechnungsnummer: String,
		gesamtbetrag_ct: i64,
	},
	ZahlungsavisEmpfangen {
		rechnungsnummer: String,
		akzeptiert: bool,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum NetzkontoabrechnungEvent {
	RechnungGesendet(InvoicRechnung),
	ZahlungsavisEmpfangen(RemadvZahlungsavis),
}

pub fn reduce(
	state: NetzkontoabrechnungState,
	event: NetzkontoabrechnungEvent,
) -> Result<ReducerOutput<NetzkontoabrechnungState>, ProzessFehler> {
	match (state, event) {
		// 5.2.1: Idle + RechnungGesendet -> RechnungGesendet
		(
			NetzkontoabrechnungState::Idle,
			NetzkontoabrechnungEvent::RechnungGesendet(r),
		) => Ok(ReducerOutput {
			state: NetzkontoabrechnungState::RechnungGesendet {
				rechnungsnummer: r.rechnungsnummer,
				gesamtbetrag_ct: r.gesamtbetrag_ct,
			},
			nachrichten: vec![],
		}),

		// 5.2.2: RechnungGesendet + ZahlungsavisEmpfangen -> ZahlungsavisEmpfangen
		(
			NetzkontoabrechnungState::RechnungGesendet { rechnungsnummer, .. },
			NetzkontoabrechnungEvent::ZahlungsavisEmpfangen(avis),
		) => Ok(ReducerOutput {
			state: NetzkontoabrechnungState::ZahlungsavisEmpfangen {
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
