use schemars::{JsonSchema, schema::RootSchema};
use serde::{Deserialize, Serialize};

use crate::gpke_nachrichten::*;
use crate::ids::MarktpartnerId;
use crate::pruefidentifikator::PruefIdentifikator;
use crate::rd2_quittung::AcknowledgementDocument;
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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
	RdStatusRequest(RdStatusRequest),
	RdKaskade(RdKaskade),
	AcknowledgementDocument(AcknowledgementDocument),
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

macro_rules! payload_registry {
	($( $typ:literal => $variant:ident($payload_ty:ty) ),+ $(,)?) => {
		pub const NACHRICHTEN_PAYLOAD_TYPEN: &[&str] = &[
			$( $typ ),+
		];

		pub fn schema_for(typ: &str) -> Option<RootSchema> {
			match typ {
				$( $typ => Some(schemars::schema_for!($payload_ty)), )+
				_ => None,
			}
		}

		impl NachrichtenPayload {
			pub fn typ(&self) -> &'static str {
				match self {
					$( Self::$variant(_) => $typ, )+
				}
			}

			pub fn from_value_for_typ(
				typ: &str,
				fields: serde_json::Value,
			) -> Result<Option<Self>, serde_json::Error> {
				match typ {
					$(
						$typ => serde_json::from_value::<$payload_ty>(fields)
							.map(Self::$variant)
							.map(Some),
					)+
					_ => Ok(None),
				}
			}
		}
	};
}

payload_registry! {
	"UtilmdAnmeldung" => UtilmdAnmeldung(UtilmdAnmeldung),
	"UtilmdBestaetigung" => UtilmdBestaetigung(UtilmdBestaetigung),
	"UtilmdAbmeldung" => UtilmdAbmeldung(UtilmdAbmeldung),
	"UtilmdAblehnung" => UtilmdAblehnung(UtilmdAblehnung),
	"UtilmdZuordnung" => UtilmdZuordnung(UtilmdZuordnung),
	"UtilmdLieferendeAbmeldung" => UtilmdLieferendeAbmeldung(UtilmdLieferendeAbmeldung),
	"UtilmdLieferendeBestaetigung" => UtilmdLieferendeBestaetigung(UtilmdLieferendeBestaetigung),
	"MsconsSchlussturnusmesswert" => MsconsSchlussturnusmesswert(MsconsSchlussturnusmesswert),
	"MsconsLastgang" => MsconsLastgang(MsconsLastgang),
	"UtilmdStammdatenaenderung" => UtilmdStammdatenaenderung(UtilmdStammdatenaenderung),
	"UtilmdZuordnungsliste" => UtilmdZuordnungsliste(UtilmdZuordnungsliste),
	"UtilmdGeschaeftsdatenanfrage" => UtilmdGeschaeftsdatenanfrage(UtilmdGeschaeftsdatenanfrage),
	"UtilmdGeschaeftsdatenantwort" => UtilmdGeschaeftsdatenantwort(UtilmdGeschaeftsdatenantwort),
	"UtilmdMsbWechselAnmeldung" => UtilmdMsbWechselAnmeldung(UtilmdMsbWechselAnmeldung),
	"UtilmdGeraetewechsel" => UtilmdGeraetewechsel(UtilmdGeraetewechsel),
	"OrdersWerteAnfrage" => OrdersWerteAnfrage(OrdersWerteAnfrage),
	"ReqoteAngebotsanfrage" => ReqoteAngebotsanfrage(ReqoteAngebotsanfrage),
	"QuotesAngebot" => QuotesAngebot(QuotesAngebot),
	"OrdersBestellung" => OrdersBestellung(OrdersBestellung),
	"OrdrspBestellantwort" => OrdrspBestellantwort(OrdrspBestellantwort),
	"PricatPreisblatt" => PricatPreisblatt(PricatPreisblatt),
	"UtilmdBilanzkreiszuordnung" => UtilmdBilanzkreiszuordnung(UtilmdBilanzkreiszuordnung),
	"MsconsAggregierteZeitreihen" => MsconsAggregierteZeitreihen(MsconsAggregierteZeitreihen),
	"MsconsMehrMindermengen" => MsconsMehrMindermengen(MsconsMehrMindermengen),
	"UtilmdClearingliste" => UtilmdClearingliste(UtilmdClearingliste),
	"InvoicRechnung" => InvoicRechnung(InvoicRechnung),
	"RemadvZahlungsavis" => RemadvZahlungsavis(RemadvZahlungsavis),
	"UtilmdAnmeldungErzeugung" => UtilmdAnmeldungErzeugung(UtilmdAnmeldungErzeugung),
	"MsconsEinspeiseMesswerte" => MsconsEinspeiseMesswerte(MsconsEinspeiseMesswerte),
	"RdStammdaten" => RdStammdaten(RdStammdaten),
	"RdFahrplan" => RdFahrplan(RdFahrplan),
	"RdAktivierung" => RdAktivierung(RdAktivierung),
	"RdBestaetigung" => RdBestaetigung(RdBestaetigung),
	"RdEngpass" => RdEngpass(RdEngpass),
	"RdNichtverfuegbarkeit" => RdNichtverfuegbarkeit(RdNichtverfuegbarkeit),
	"RdKostenblatt" => RdKostenblatt(RdKostenblatt),
	"RdStatusRequest" => RdStatusRequest(RdStatusRequest),
	"RdKaskade" => RdKaskade(RdKaskade),
	"AcknowledgementDocument" => AcknowledgementDocument(AcknowledgementDocument),
	"UtilmdSteuerbareVerbrauchseinrichtung" => UtilmdSteuerbareVerbrauchseinrichtung(UtilmdSteuerbareVerbrauchseinrichtung),
	"ClsSteuersignal" => ClsSteuersignal(ClsSteuersignal),
	"Nominierung" => Nominierung(Nominierung),
	"NominierungBestaetigung" => NominierungBestaetigung(NominierungBestaetigung),
	"Renominierung" => Renominierung(Renominierung),
	"MsconsBrennwert" => MsconsBrennwert(MsconsBrennwert),
	"UtilmdAusspeisepunkt" => UtilmdAusspeisepunkt(UtilmdAusspeisepunkt),
	"IftstaStatusmeldung" => IftstaStatusmeldung(crate::querschnitt::IftstaStatusmeldung),
	"PartinMarktpartner" => PartinMarktpartner(crate::querschnitt::PartinMarktpartner),
	"UtiltsZaehlzeitdefinition" => UtiltsZaehlzeitdefinition(crate::querschnitt::UtiltsZaehlzeitdefinition),
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn schema_for_covers_all_payload_types() {
		for typ in NACHRICHTEN_PAYLOAD_TYPEN {
			assert!(schema_for(typ).is_some(), "missing schema for {typ}");
		}
	}
}
