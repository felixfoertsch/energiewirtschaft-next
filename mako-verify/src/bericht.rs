use serde::Serialize;

// ---------------------------------------------------------------------------
// Urteil — overall verdict for any verification check
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Urteil {
	Bestanden,
	Fehlgeschlagen,
	NichtPruefbar,
}

// ---------------------------------------------------------------------------
// AHB field-level result
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct AhbFeldErgebnis {
	pub segment_code: Option<String>,
	pub segment_group: Option<String>,
	pub data_element: Option<String>,
	pub name: String,
	pub ahb_ausdruck: String,
	pub unser_wert: Option<String>,
	pub erwarteter_wert: Option<String>,
	pub urteil: Urteil,
	pub details: Option<String>,
}

// ---------------------------------------------------------------------------
// AHB document-level result
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct AhbErgebnis {
	pub pruefidentifikator: String,
	pub nachrichtentyp: String,
	pub felder: Vec<AhbFeldErgebnis>,
	pub urteil: Urteil,
	pub zusammenfassung: Option<String>,
}

// ---------------------------------------------------------------------------
// EBD single outcome
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct EbdAusgang {
	pub ebd_code: String,
	pub schritt: String,
	pub beschreibung: String,
	pub antwortcode: Option<String>,
	pub notiz: Option<String>,
}

// ---------------------------------------------------------------------------
// EBD document-level result
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct EbdErgebnis {
	pub ebd_code: String,
	pub ebd_name: String,
	pub rolle: String,
	pub unser_ergebnis: Option<EbdAusgang>,
	pub gueltige_ausgaenge: Vec<EbdAusgang>,
	pub urteil: Urteil,
	pub details: Option<String>,
}

// ---------------------------------------------------------------------------
// Interop field comparison
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct InteropFeldVergleich {
	pub feld: String,
	pub unser_wert: Option<String>,
	pub drittanbieter_wert: Option<String>,
	pub stimmt_ueberein: bool,
}

// ---------------------------------------------------------------------------
// Interop result
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct InteropErgebnis {
	pub parse_ok_unser: bool,
	pub parse_ok_drittanbieter: bool,
	pub roundtrip_ok: bool,
	pub feldvergleiche: Vec<InteropFeldVergleich>,
	pub urteil: Urteil,
}

// ---------------------------------------------------------------------------
// Per-file verification result
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct VerifikationsErgebnis {
	pub datei: String,
	pub nachrichtentyp: String,
	pub pruefidentifikator: Option<String>,
	pub ahb: Option<AhbErgebnis>,
	pub ebd: Option<EbdErgebnis>,
	pub interop: Option<InteropErgebnis>,
	pub gesamt_urteil: Urteil,
	/// Error detail when verification could not be performed (parse failure,
	/// unreadable file, etc.). `None` for successful verifications.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fehler: Option<String>,
}

// ---------------------------------------------------------------------------
// Batch result across multiple files
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct BatchErgebnis {
	pub gesamt: usize,
	pub bestanden: usize,
	pub fehlgeschlagen: usize,
	pub nicht_pruefbar: usize,
	pub ergebnisse: Vec<VerifikationsErgebnis>,
}

impl BatchErgebnis {
	/// Compute a human-readable summary string.
	pub fn zusammenfassung(&self) -> String {
		format!(
			"{} geprüft, {} bestanden, {} fehlgeschlagen, {} nicht prüfbar",
			self.gesamt, self.bestanden, self.fehlgeschlagen, self.nicht_pruefbar,
		)
	}

	/// Build a `BatchErgebnis` from a list of individual results.
	pub fn aus_ergebnissen(ergebnisse: Vec<VerifikationsErgebnis>) -> Self {
		let gesamt = ergebnisse.len();
		let bestanden = ergebnisse
			.iter()
			.filter(|e| e.gesamt_urteil == Urteil::Bestanden)
			.count();
		let fehlgeschlagen = ergebnisse
			.iter()
			.filter(|e| e.gesamt_urteil == Urteil::Fehlgeschlagen)
			.count();
		let nicht_pruefbar = ergebnisse
			.iter()
			.filter(|e| e.gesamt_urteil == Urteil::NichtPruefbar)
			.count();

		Self {
			gesamt,
			bestanden,
			fehlgeschlagen,
			nicht_pruefbar,
			ergebnisse,
		}
	}
}
