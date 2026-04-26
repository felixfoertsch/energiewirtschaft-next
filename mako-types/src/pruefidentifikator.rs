use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gpke_nachrichten::RechnungsTyp;
use crate::nachricht::NachrichtenPayload;

/// BDEW Pruefidentifikator (RFF+Z13) -- identifies the specific business use case
/// of an EDIFACT message. Each message in MaKo carries exactly one Pruefidentifikator
/// that determines which AHB rules and EBD decision trees apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum PruefIdentifikator {
	// === GPKE (Strom-Lieferantenwechsel) ===
	/// 44001: Anmeldung Netznutzung (UTILMD, GPKE 1.1.1)
	AnmeldungNn,
	/// 44002: Bestaetigung Anmeldung (APERAK)
	AnmeldungBestaetigung,
	/// 44003: Ablehnung Anmeldung (APERAK)
	AnmeldungAblehnung,
	/// 44004: Abmeldung Netznutzung (UTILMD, GPKE 1.2.1)
	AbmeldungNn,
	/// 44005: Bestaetigung Abmeldung (APERAK)
	AbmeldungBestaetigung,
	/// 44006: Ablehnung Abmeldung (APERAK)
	AbmeldungAblehnung,
	/// 44016: Kuendigungsmitteilung an Altlieferant (UTILMD, GPKE 1.1.3)
	Kuendigungsmitteilung,
	/// 44017: Bestaetigung Kuendigung (APERAK)
	KuendigungBestaetigung,
	/// 44018: Ablehnung/Widerspruch Kuendigung (APERAK)
	KuendigungAblehnung,
	/// 44112: Stammdatenaenderung (UTILMD, GPKE 1.3)
	Stammdatenaenderung,
	/// 44123: Bilanzierungsrelevante Aenderung (UTILMD)
	BilanzierungsrelevanteAenderung,

	// === Messwerte ===
	/// 13002: Zaehlerstand (MSCONS)
	Zaehlerstand,
	/// 13006: Messwert Storno (MSCONS)
	MesswertStorno,
	/// 13007: Gasbeschaffenheit (MSCONS)
	Gasbeschaffenheit,
	/// 13008: Lastgang (MSCONS)
	Lastgang,
	/// 13009: Energiemenge (MSCONS)
	Energiemenge,

	// === WiM (Messwesen) ===
	/// 17009: Geraetewechsel (ORDERS)
	Geraetewechsel,
	/// 17101: Anfrage Stammdaten MaLo (ORDERS)
	AnfrageStammdatenMalo,
	/// 17102: Anfrage Werte (ORDERS)
	AnfrageWerte,
	/// 19015: Bestaetigung Geraetewechsel (ORDRSP)
	GeraetewechselBestaetigung,
	/// 19101: Ablehnung Stammdatenanfrage (ORDRSP)
	StammdatenanfrageAblehnung,
	/// 19102: Ablehnung Werteanfrage (ORDRSP)
	WerteanfrageAblehnung,

	// === IFTSTA (Status) ===
	/// 21039: Auftragsstatus Sperren (IFTSTA)
	AuftragsstatusSperren,
	/// 21040: Info Entsperrauftrag (IFTSTA)
	InfoEntsperrauftrag,

	// === Rechnungen ===
	/// 31001: Abschlagsrechnung (INVOIC)
	Abschlagsrechnung,
	/// 31002: Netznutzungsrechnung (INVOIC)
	Netznutzungsrechnung,
	/// 31003: Rechnung Messstellenbetrieb (INVOIC)
	RechnungMessstellenbetrieb,
	/// 31004: Stornorechnung (INVOIC)
	Stornorechnung,
	/// 33001: Zahlungsavis positiv (REMADV)
	ZahlungsavisPositiv,
	/// 33002: Zahlungsavis negativ / Ablehnung (REMADV)
	ZahlungsavisNegativ,
}

impl PruefIdentifikator {
	/// Returns the numeric code as used in RFF+Z13 segments.
	pub fn code(&self) -> u32 {
		match self {
			Self::AnmeldungNn => 44001,
			Self::AnmeldungBestaetigung => 44002,
			Self::AnmeldungAblehnung => 44003,
			Self::AbmeldungNn => 44004,
			Self::AbmeldungBestaetigung => 44005,
			Self::AbmeldungAblehnung => 44006,
			Self::Kuendigungsmitteilung => 44016,
			Self::KuendigungBestaetigung => 44017,
			Self::KuendigungAblehnung => 44018,
			Self::Stammdatenaenderung => 44112,
			Self::BilanzierungsrelevanteAenderung => 44123,
			Self::Zaehlerstand => 13002,
			Self::MesswertStorno => 13006,
			Self::Gasbeschaffenheit => 13007,
			Self::Lastgang => 13008,
			Self::Energiemenge => 13009,
			Self::Geraetewechsel => 17009,
			Self::AnfrageStammdatenMalo => 17101,
			Self::AnfrageWerte => 17102,
			Self::GeraetewechselBestaetigung => 19015,
			Self::StammdatenanfrageAblehnung => 19101,
			Self::WerteanfrageAblehnung => 19102,
			Self::AuftragsstatusSperren => 21039,
			Self::InfoEntsperrauftrag => 21040,
			Self::Abschlagsrechnung => 31001,
			Self::Netznutzungsrechnung => 31002,
			Self::RechnungMessstellenbetrieb => 31003,
			Self::Stornorechnung => 31004,
			Self::ZahlungsavisPositiv => 33001,
			Self::ZahlungsavisNegativ => 33002,
		}
	}

	/// Parse a numeric code back to a PruefIdentifikator.
	pub fn from_code(code: u32) -> Option<Self> {
		match code {
			44001 => Some(Self::AnmeldungNn),
			44002 => Some(Self::AnmeldungBestaetigung),
			44003 => Some(Self::AnmeldungAblehnung),
			44004 => Some(Self::AbmeldungNn),
			44005 => Some(Self::AbmeldungBestaetigung),
			44006 => Some(Self::AbmeldungAblehnung),
			44016 => Some(Self::Kuendigungsmitteilung),
			44017 => Some(Self::KuendigungBestaetigung),
			44018 => Some(Self::KuendigungAblehnung),
			44112 => Some(Self::Stammdatenaenderung),
			44123 => Some(Self::BilanzierungsrelevanteAenderung),
			13002 => Some(Self::Zaehlerstand),
			13006 => Some(Self::MesswertStorno),
			13007 => Some(Self::Gasbeschaffenheit),
			13008 => Some(Self::Lastgang),
			13009 => Some(Self::Energiemenge),
			17009 => Some(Self::Geraetewechsel),
			17101 => Some(Self::AnfrageStammdatenMalo),
			17102 => Some(Self::AnfrageWerte),
			19015 => Some(Self::GeraetewechselBestaetigung),
			19101 => Some(Self::StammdatenanfrageAblehnung),
			19102 => Some(Self::WerteanfrageAblehnung),
			21039 => Some(Self::AuftragsstatusSperren),
			21040 => Some(Self::InfoEntsperrauftrag),
			31001 => Some(Self::Abschlagsrechnung),
			31002 => Some(Self::Netznutzungsrechnung),
			31003 => Some(Self::RechnungMessstellenbetrieb),
			31004 => Some(Self::Stornorechnung),
			33001 => Some(Self::ZahlungsavisPositiv),
			33002 => Some(Self::ZahlungsavisNegativ),
			_ => None,
		}
	}

	/// Best-effort lookup from a typed payload to its RFF+Z13 identifier.
	///
	/// Some implemented payloads do not have a dedicated enum variant yet; those
	/// return `None` so callers can still serialize messages that have no known
	/// Prüfidentifikator in this crate.
	pub fn for_payload(payload: &NachrichtenPayload) -> Option<Self> {
		match payload {
			NachrichtenPayload::UtilmdAnmeldung(_) => Some(Self::AnmeldungNn),
			NachrichtenPayload::UtilmdBestaetigung(_) => Some(Self::AnmeldungBestaetigung),
			NachrichtenPayload::UtilmdAbmeldung(_) => Some(Self::Kuendigungsmitteilung),
			NachrichtenPayload::UtilmdAblehnung(_) => Some(Self::AnmeldungAblehnung),
			NachrichtenPayload::UtilmdLieferendeAbmeldung(_) => Some(Self::AbmeldungNn),
			NachrichtenPayload::UtilmdLieferendeBestaetigung(_) => {
				Some(Self::AbmeldungBestaetigung)
			}
			NachrichtenPayload::UtilmdStammdatenaenderung(_) => Some(Self::Stammdatenaenderung),
			NachrichtenPayload::UtilmdBilanzkreiszuordnung(_) => {
				Some(Self::BilanzierungsrelevanteAenderung)
			}
			NachrichtenPayload::MsconsSchlussturnusmesswert(_) => Some(Self::Zaehlerstand),
			NachrichtenPayload::MsconsLastgang(_) => Some(Self::Lastgang),
			NachrichtenPayload::MsconsBrennwert(_) => Some(Self::Gasbeschaffenheit),
			NachrichtenPayload::MsconsEinspeiseMesswerte(_) => Some(Self::Energiemenge),
			NachrichtenPayload::UtilmdGeraetewechsel(_) => Some(Self::Geraetewechsel),
			NachrichtenPayload::OrdersWerteAnfrage(_) => Some(Self::AnfrageWerte),
			NachrichtenPayload::InvoicRechnung(rechnung) => match rechnung.rechnungstyp {
				RechnungsTyp::Netznutzung => Some(Self::Netznutzungsrechnung),
				RechnungsTyp::Messstellenbetrieb => Some(Self::RechnungMessstellenbetrieb),
				RechnungsTyp::MehrMindermengen => Some(Self::Abschlagsrechnung),
				RechnungsTyp::Ausgleichsenergie => Some(Self::Stornorechnung),
			},
			NachrichtenPayload::RemadvZahlungsavis(avis) => {
				if avis.akzeptiert {
					Some(Self::ZahlungsavisPositiv)
				} else {
					Some(Self::ZahlungsavisNegativ)
				}
			}
			_ => None,
		}
	}

	/// Returns the process this identifier belongs to.
	pub fn prozess(&self) -> &'static str {
		match self {
			Self::AnmeldungNn
			| Self::AnmeldungBestaetigung
			| Self::AnmeldungAblehnung
			| Self::AbmeldungNn
			| Self::AbmeldungBestaetigung
			| Self::AbmeldungAblehnung
			| Self::Kuendigungsmitteilung
			| Self::KuendigungBestaetigung
			| Self::KuendigungAblehnung
			| Self::Stammdatenaenderung
			| Self::BilanzierungsrelevanteAenderung => "GPKE",

			Self::Zaehlerstand
			| Self::MesswertStorno
			| Self::Gasbeschaffenheit
			| Self::Lastgang
			| Self::Energiemenge => "Messwerte",

			Self::Geraetewechsel
			| Self::AnfrageStammdatenMalo
			| Self::AnfrageWerte
			| Self::GeraetewechselBestaetigung
			| Self::StammdatenanfrageAblehnung
			| Self::WerteanfrageAblehnung => "WiM",

			Self::AuftragsstatusSperren | Self::InfoEntsperrauftrag => "IFTSTA",

			Self::Abschlagsrechnung
			| Self::Netznutzungsrechnung
			| Self::RechnungMessstellenbetrieb
			| Self::Stornorechnung
			| Self::ZahlungsavisPositiv
			| Self::ZahlungsavisNegativ => "Abrechnung",
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// All variants for iteration in tests.
	const ALL_VARIANTS: &[PruefIdentifikator] = &[
		PruefIdentifikator::AnmeldungNn,
		PruefIdentifikator::AnmeldungBestaetigung,
		PruefIdentifikator::AnmeldungAblehnung,
		PruefIdentifikator::AbmeldungNn,
		PruefIdentifikator::AbmeldungBestaetigung,
		PruefIdentifikator::AbmeldungAblehnung,
		PruefIdentifikator::Kuendigungsmitteilung,
		PruefIdentifikator::KuendigungBestaetigung,
		PruefIdentifikator::KuendigungAblehnung,
		PruefIdentifikator::Stammdatenaenderung,
		PruefIdentifikator::BilanzierungsrelevanteAenderung,
		PruefIdentifikator::Zaehlerstand,
		PruefIdentifikator::MesswertStorno,
		PruefIdentifikator::Gasbeschaffenheit,
		PruefIdentifikator::Lastgang,
		PruefIdentifikator::Energiemenge,
		PruefIdentifikator::Geraetewechsel,
		PruefIdentifikator::AnfrageStammdatenMalo,
		PruefIdentifikator::AnfrageWerte,
		PruefIdentifikator::GeraetewechselBestaetigung,
		PruefIdentifikator::StammdatenanfrageAblehnung,
		PruefIdentifikator::WerteanfrageAblehnung,
		PruefIdentifikator::AuftragsstatusSperren,
		PruefIdentifikator::InfoEntsperrauftrag,
		PruefIdentifikator::Abschlagsrechnung,
		PruefIdentifikator::Netznutzungsrechnung,
		PruefIdentifikator::RechnungMessstellenbetrieb,
		PruefIdentifikator::Stornorechnung,
		PruefIdentifikator::ZahlungsavisPositiv,
		PruefIdentifikator::ZahlungsavisNegativ,
	];

	#[test]
	fn code_and_from_code_round_trip() {
		for &variant in ALL_VARIANTS {
			let code = variant.code();
			let parsed = PruefIdentifikator::from_code(code)
				.unwrap_or_else(|| panic!("from_code({code}) returned None"));
			assert_eq!(variant, parsed, "round-trip failed for code {code}");
		}
	}

	#[test]
	fn from_code_returns_none_for_unknown() {
		assert_eq!(PruefIdentifikator::from_code(0), None);
		assert_eq!(PruefIdentifikator::from_code(99999), None);
		assert_eq!(PruefIdentifikator::from_code(12345), None);
	}

	#[test]
	fn gpke_variants_return_gpke_prozess() {
		let gpke_variants = [
			PruefIdentifikator::AnmeldungNn,
			PruefIdentifikator::AnmeldungBestaetigung,
			PruefIdentifikator::AnmeldungAblehnung,
			PruefIdentifikator::AbmeldungNn,
			PruefIdentifikator::AbmeldungBestaetigung,
			PruefIdentifikator::AbmeldungAblehnung,
			PruefIdentifikator::Kuendigungsmitteilung,
			PruefIdentifikator::KuendigungBestaetigung,
			PruefIdentifikator::KuendigungAblehnung,
			PruefIdentifikator::Stammdatenaenderung,
			PruefIdentifikator::BilanzierungsrelevanteAenderung,
		];
		for variant in gpke_variants {
			assert_eq!(
				variant.prozess(),
				"GPKE",
				"{variant:?} should belong to GPKE"
			);
		}
	}

	#[test]
	fn json_serialization_round_trip() {
		for &variant in ALL_VARIANTS {
			let json = serde_json::to_string(&variant)
				.unwrap_or_else(|e| panic!("serialize {variant:?}: {e}"));
			let deserialized: PruefIdentifikator = serde_json::from_str(&json)
				.unwrap_or_else(|e| panic!("deserialize {variant:?} from {json}: {e}"));
			assert_eq!(variant, deserialized, "JSON round-trip failed for {variant:?}");
		}
	}
}
