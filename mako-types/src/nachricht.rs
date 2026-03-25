use serde::{Deserialize, Serialize};

use crate::gpke_nachrichten::*;
use crate::ids::MarktpartnerId;
use crate::pruefidentifikator::PruefIdentifikator;
use crate::rolle::MarktRolle;

/// Envelope for any MaKo message, carrying routing info and typed payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nachricht {
	pub absender: MarktpartnerId,
	pub absender_rolle: MarktRolle,
	pub empfaenger: MarktpartnerId,
	pub empfaenger_rolle: MarktRolle,
	pub pruef_id: Option<PruefIdentifikator>,
	pub payload: NachrichtenPayload,
}

/// Typed payload — one variant per concrete message type.
/// Extended as new message types are implemented.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NachrichtenPayload {
	UtilmdAnmeldung(UtilmdAnmeldung),
	UtilmdBestaetigung(UtilmdBestaetigung),
	UtilmdAbmeldung(UtilmdAbmeldung),
	UtilmdAblehnung(UtilmdAblehnung),
	UtilmdZuordnung(UtilmdZuordnung),
	UtilmdLieferendeAbmeldung(UtilmdLieferendeAbmeldung),
	UtilmdLieferendeBestaetigung(UtilmdLieferendeBestaetigung),
	MsconsSchlussturnusmesswert(MsconsSchlussturnusmesswert),
	MsconsLastgang(MsconsLastgang),
	UtilmdStammdatenaenderung(UtilmdStammdatenaenderung),
	UtilmdZuordnungsliste(UtilmdZuordnungsliste),
	UtilmdGeschaeftsdatenanfrage(UtilmdGeschaeftsdatenanfrage),
	UtilmdGeschaeftsdatenantwort(UtilmdGeschaeftsdatenantwort),
	// WiM
	UtilmdMsbWechselAnmeldung(UtilmdMsbWechselAnmeldung),
	UtilmdGeraetewechsel(UtilmdGeraetewechsel),
	OrdersWerteAnfrage(OrdersWerteAnfrage),
	// UBP
	ReqoteAngebotsanfrage(ReqoteAngebotsanfrage),
	QuotesAngebot(QuotesAngebot),
	OrdersBestellung(OrdersBestellung),
	OrdrspBestellantwort(OrdrspBestellantwort),
	PricatPreisblatt(PricatPreisblatt),
	// MaBiS
	UtilmdBilanzkreiszuordnung(UtilmdBilanzkreiszuordnung),
	MsconsAggregierteZeitreihen(MsconsAggregierteZeitreihen),
	MsconsMehrMindermengen(MsconsMehrMindermengen),
	UtilmdClearingliste(UtilmdClearingliste),
	// Abrechnung
	InvoicRechnung(InvoicRechnung),
	RemadvZahlungsavis(RemadvZahlungsavis),
	// MPES
	UtilmdAnmeldungErzeugung(UtilmdAnmeldungErzeugung),
	MsconsEinspeiseMesswerte(MsconsEinspeiseMesswerte),
	// Redispatch 2.0
	RdStammdaten(RdStammdaten),
	RdFahrplan(RdFahrplan),
	RdAktivierung(RdAktivierung),
	RdBestaetigung(RdBestaetigung),
	RdEngpass(RdEngpass),
	RdNichtverfuegbarkeit(RdNichtverfuegbarkeit),
	RdKostenblatt(RdKostenblatt),
	// §14a EnWG
	UtilmdSteuerbareVerbrauchseinrichtung(UtilmdSteuerbareVerbrauchseinrichtung),
	ClsSteuersignal(ClsSteuersignal),
	// Gas
	Nominierung(Nominierung),
	NominierungBestaetigung(NominierungBestaetigung),
	Renominierung(Renominierung),
	MsconsBrennwert(MsconsBrennwert),
	UtilmdAusspeisepunkt(UtilmdAusspeisepunkt),
	// Querschnitt
	IftstaStatusmeldung(crate::querschnitt::IftstaStatusmeldung),
	PartinMarktpartner(crate::querschnitt::PartinMarktpartner),
	UtiltsZaehlzeitdefinition(crate::querschnitt::UtiltsZaehlzeitdefinition),
}
