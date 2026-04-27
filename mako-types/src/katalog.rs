//! Process catalog descriptors.
//!
//! Each crate that implements MaKo processes (mako-gpke, mako-rd2, mako-wim,
//! ...) exposes a `pub fn katalog() -> Vec<ProzessDef>` that lists the
//! processes it implements together with their step sequences. The aggregator
//! crate `mako-katalog` collects all of them and the CLI emits the union as
//! JSON for the test frontend.
//!
//! Why a hand-authored catalog instead of deriving it from the reducers: a
//! reducer encodes transitions, not a linear human-readable timeline. The
//! catalog is the timeline a domain expert wants to see in the UI. We keep
//! it next to the implementation so drift is visible in the same diff.

use serde::{Deserialize, Serialize};

use crate::rolle::MarktRolle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProzessKategorie {
	Gpke,
	Wim,
	Ubp,
	MaBis,
	Abrechnung,
	Rd2,
	Para14a,
	GeliGas,
	GabiGas,
	KoV,
	Mpes,
}

impl ProzessKategorie {
	/// Display label for the UI grouping header.
	pub fn label(&self) -> &'static str {
		match self {
			Self::Gpke => "GPKE",
			Self::Wim => "WiM",
			Self::Ubp => "UBP",
			Self::MaBis => "MaBiS",
			Self::Abrechnung => "Abrechnung",
			Self::Rd2 => "RD 2.0",
			Self::Para14a => "§14a",
			Self::GeliGas => "GeLi Gas",
			Self::GabiGas => "GABi Gas",
			Self::KoV => "KoV",
			Self::Mpes => "MPES",
		}
	}
}

/// EDIFACT/XML message type a step is carried over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NachrichtenTyp {
	Utilmd,
	Mscons,
	Invoic,
	Remadv,
	Reqote,
	Quotes,
	Orders,
	Ordrsp,
	Pricat,
	/// Redispatch 2.0 XML payloads.
	RdXml,
	/// CLS steering signal (binary/proprietary).
	Cls,
	/// Process-internal step without a wire message (e.g. waiting on a Frist).
	Intern,
}

impl NachrichtenTyp {
	pub fn label(&self) -> &'static str {
		match self {
			Self::Utilmd => "UTILMD",
			Self::Mscons => "MSCONS",
			Self::Invoic => "INVOIC",
			Self::Remadv => "REMADV",
			Self::Reqote => "REQOTE",
			Self::Quotes => "QUOTES",
			Self::Orders => "ORDERS",
			Self::Ordrsp => "ORDRSP",
			Self::Pricat => "PRICAT",
			Self::RdXml => "XML",
			Self::Cls => "CLS",
			Self::Intern => "",
		}
	}
}

/// One observable step in a process timeline.
///
/// `absender`/`empfaenger` are stored as filesystem slugs (matching
/// `mako-cli init`'s role directory names) so the wire format aligns 1:1
/// with `/api/rollen` and the test UI doesn't need any role-name mapping.
/// `typ` references the payload variant from the corresponding
/// `*_nachrichten` enum if applicable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchrittDef {
	pub name: String,
	pub erklaerung: String,
	pub absender: String,
	pub empfaenger: String,
	/// Symbolic payload variant (e.g. `"UtilmdAnmeldung"`). For `Intern`
	/// steps this is `""`.
	pub typ: String,
	pub nachrichten_typ: NachrichtenTyp,
}

impl SchrittDef {
	/// Convenience constructor — takes &str for ergonomic call sites.
	pub fn new(
		name: &str,
		absender: MarktRolle,
		empfaenger: MarktRolle,
		typ: &str,
		nachrichten_typ: NachrichtenTyp,
	) -> Self {
		Self::with_erklaerung(
			name,
			absender,
			empfaenger,
			typ,
			nachrichten_typ,
			&standard_erklaerung(name, absender, empfaenger, typ, nachrichten_typ),
		)
	}

	/// Constructor for hand-authored domain explanations.
	pub fn with_erklaerung(
		name: &str,
		absender: MarktRolle,
		empfaenger: MarktRolle,
		typ: &str,
		nachrichten_typ: NachrichtenTyp,
		erklaerung: &str,
	) -> Self {
		Self {
			name: name.to_string(),
			erklaerung: erklaerung.to_string(),
			absender: absender.slug().to_string(),
			empfaenger: empfaenger.slug().to_string(),
			typ: typ.to_string(),
			nachrichten_typ,
		}
	}
}

fn standard_erklaerung(
	name: &str,
	absender: MarktRolle,
	empfaenger: MarktRolle,
	typ: &str,
	nachrichten_typ: NachrichtenTyp,
) -> String {
	let absender = absender.kuerzel();
	let empfaenger = empfaenger.kuerzel();
	let payload = if typ.is_empty() {
		"den internen Prozessstatus".to_string()
	} else {
		format!("die fachlichen Daten aus {typ}")
	};
	let kanal = nachrichten_typ.label();
	let kanal = if kanal.is_empty() {
		"prozessintern"
	} else {
		kanal
	};
	if absender == "DP" || empfaenger == "DP" {
		return format!(
			"Der DP tritt als Mittelsmann zwischen EIV und ANB auf und konsolidiert die Datenflüsse im RD-2.0-Datenraum. In diesem Schritt übergibt {absender} {payload} an {empfaenger}, damit alle Beteiligten mit derselben fachlichen Sicht arbeiten."
		);
	}
	match nachrichten_typ {
		NachrichtenTyp::Intern => format!(
			"{absender} klärt mit {empfaenger} den fachlichen Status für „{name}“. Der interne Schritt steuert, ob der Prozess fortgeführt, bestätigt oder abgelehnt wird."
		),
		NachrichtenTyp::Invoic => format!(
			"{absender} stellt {empfaenger} die abrechnungsrelevante Forderung. Die Rechnung macht den fachlichen Anspruch zahlbar und ordnet ihn dem Marktprozess zu."
		),
		NachrichtenTyp::Remadv => format!(
			"{absender} avisiert {empfaenger} die Zahlung zu einer offenen Forderung. {empfaenger} kann den Zahlungseingang dadurch eindeutig zuordnen."
		),
		NachrichtenTyp::Mscons => format!(
			"{absender} übermittelt {empfaenger} die benötigten Mess- oder Bilanzierungswerte. Diese Werte bilden die Grundlage für Plausibilisierung, Bilanzierung oder Abrechnung."
		),
		NachrichtenTyp::Reqote => format!(
			"{absender} fordert bei {empfaenger} ein Angebot für eine energiewirtschaftliche Leistung an. Die Anfrage legt den Leistungsbedarf vor einer Bestellung fest."
		),
		NachrichtenTyp::Quotes => format!(
			"{absender} übermittelt {empfaenger} Konditionen und Leistungsdetails. {empfaenger} kann auf dieser Grundlage die Bestellung fachlich entscheiden."
		),
		NachrichtenTyp::Orders => format!(
			"{absender} bestellt bei {empfaenger} eine definierte Marktleistung. Die Bestellung schafft die Grundlage für Ausführung, Nachweis und Abrechnung."
		),
		NachrichtenTyp::Ordrsp => format!(
			"{absender} bestätigt oder lehnt die Bestellung gegenüber {empfaenger} ab. {empfaenger} erkennt dadurch, ob die Leistung erbracht wird."
		),
		NachrichtenTyp::Pricat => format!(
			"{absender} stellt {empfaenger} die gültigen Preise für standardisierte Leistungen bereit. Das Preisblatt schafft die Grundlage für Angebot, Bestellung und Abrechnung."
		),
		NachrichtenTyp::RdXml => format!(
			"{absender} übermittelt {empfaenger} {payload} für Redispatch 2.0. Der Schritt stellt sicher, dass Netzbedarf, Einsatzplanung oder Ressourcendaten im RD-2.0-Prozess fachlich verfügbar sind."
		),
		NachrichtenTyp::Cls => format!(
			"{absender} sendet {empfaenger} ein Steuerungssignal für eine steuerbare Verbrauchseinrichtung. Der Schritt löst die technische Umsetzung der netzdienlichen Steuerung aus."
		),
		NachrichtenTyp::Utilmd => format!(
			"{absender} übermittelt {empfaenger} {payload} per {kanal}. Der Schritt hält Stammdaten, Zuordnungen oder Geschäftsdaten zwischen den Marktpartnern synchron."
		),
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProzessDef {
	/// Stable identifier (e.g. `"gpke_lfw"`). Used as React key.
	pub key: String,
	pub name: String,
	pub kategorie: ProzessKategorie,
	pub schritte: Vec<SchrittDef>,
}

impl ProzessDef {
	pub fn new(
		key: &str,
		name: &str,
		kategorie: ProzessKategorie,
		schritte: Vec<SchrittDef>,
	) -> Self {
		Self {
			key: key.to_string(),
			name: name.to_string(),
			kategorie,
			schritte,
		}
	}

	/// Returns true if `rolle_slug` participates as sender or recipient in
	/// any step. The slug is the filesystem identifier (`MarktRolle::slug`).
	pub fn beteiligt(&self, rolle_slug: &str) -> bool {
		self.schritte
			.iter()
			.any(|s| s.absender == rolle_slug || s.empfaenger == rolle_slug)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn beteiligt_findet_absender_und_empfaenger() {
		let p = ProzessDef::new(
			"demo",
			"Demo",
			ProzessKategorie::Gpke,
			vec![SchrittDef::new(
				"x",
				MarktRolle::LieferantNeu,
				MarktRolle::Netzbetreiber,
				"UtilmdAnmeldung",
				NachrichtenTyp::Utilmd,
			)],
		);
		assert!(p.beteiligt("lieferant_neu"));
		assert!(p.beteiligt("netzbetreiber"));
		assert!(!p.beteiligt("marktgebietsverantwortlicher"));
	}

	#[test]
	fn kategorie_serialisiert_snake_case() {
		let json = serde_json::to_string(&ProzessKategorie::Rd2).unwrap();
		assert_eq!(json, "\"rd2\"");
		let json = serde_json::to_string(&ProzessKategorie::GeliGas).unwrap();
		assert_eq!(json, "\"geli_gas\"");
	}

	#[test]
	fn schritt_serialisiert_erklaerung() {
		let schritt = SchrittDef::with_erklaerung(
			"x",
			MarktRolle::LieferantNeu,
			MarktRolle::Netzbetreiber,
			"UtilmdAnmeldung",
			NachrichtenTyp::Utilmd,
			"Fachlicher Zweck.",
		);
		let json = serde_json::to_value(schritt).unwrap();
		assert_eq!(json["erklaerung"], "Fachlicher Zweck.");
	}
}
