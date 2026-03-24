use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::{InvoicRechnung, RechnungsTyp, RemadvZahlungsavis};
use mako_types::reducer::ReducerOutput;

/// Generic INVOIC -> REMADV pattern (covers 6.x):
/// Netznutzung, Messstellenbetrieb, MehrMindermengen,
/// Ausgleichsenergie Strom, Ausgleichsenergie Gas
///
/// Idle -> RechnungGesendet -> ZahlungsavisEmpfangen / Abgelehnt
#[derive(Debug, Clone, PartialEq)]
pub enum AbrechnungState {
	Idle,
	RechnungGesendet {
		rechnungsnummer: String,
		rechnungstyp: RechnungsTyp,
		gesamtbetrag_ct: i64,
	},
	ZahlungsavisEmpfangen {
		rechnungsnummer: String,
		betrag_ct: i64,
	},
	Abgelehnt {
		rechnungsnummer: String,
		grund: String,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum AbrechnungEvent {
	RechnungEmpfangen(InvoicRechnung),
	ZahlungsavisEmpfangen(RemadvZahlungsavis),
}

pub fn reduce(
	state: AbrechnungState,
	event: AbrechnungEvent,
) -> Result<ReducerOutput<AbrechnungState>, ProzessFehler> {
	match (state, event) {
		// Idle + RechnungEmpfangen -> RechnungGesendet
		(AbrechnungState::Idle, AbrechnungEvent::RechnungEmpfangen(r)) => {
			Ok(ReducerOutput {
				state: AbrechnungState::RechnungGesendet {
					rechnungsnummer: r.rechnungsnummer,
					rechnungstyp: r.rechnungstyp,
					gesamtbetrag_ct: r.gesamtbetrag_ct,
				},
				nachrichten: vec![],
			})
		}

		// RechnungGesendet + ZahlungsavisEmpfangen (akzeptiert) -> ZahlungsavisEmpfangen
		(
			AbrechnungState::RechnungGesendet { rechnungsnummer, .. },
			AbrechnungEvent::ZahlungsavisEmpfangen(avis),
		) if avis.akzeptiert => Ok(ReducerOutput {
			state: AbrechnungState::ZahlungsavisEmpfangen {
				rechnungsnummer,
				betrag_ct: avis.betrag_ct,
			},
			nachrichten: vec![],
		}),

		// RechnungGesendet + ZahlungsavisEmpfangen (abgelehnt) -> Abgelehnt
		(
			AbrechnungState::RechnungGesendet { rechnungsnummer, .. },
			AbrechnungEvent::ZahlungsavisEmpfangen(avis),
		) => Ok(ReducerOutput {
			state: AbrechnungState::Abgelehnt {
				rechnungsnummer,
				grund: avis
					.ablehnungsgrund
					.unwrap_or_else(|| "Kein Grund angegeben".to_string()),
			},
			nachrichten: vec![],
		}),

		// Catch-all: invalid transition
		(state, event) => Err(ProzessFehler::UngueltigerUebergang {
			state: format!("{state:?}"),
			event: format!("{event:?}"),
		}),
	}
}
