# MaKo Verification System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a three-layer verification system (AHB message validation, EBD process validation, codec cross-validation) that lets domain experts confirm MaKo-Engine correctness through the web UI.

**Architecture:** New `mako-verify` Rust crate loads official reference data (flat AHB JSON, EBD JSON from Hochfrequenz open-source repos) and validates messages/processes against them. Optional Node.js sidecar runs STROMDAO's `edifact-to-json-transformer` as independent cross-validator. Results shown in mako-ui via inline badges and drill-down panels.

**Tech Stack:** Rust 2024 (mako-verify crate), TypeScript/React 19 (UI components), Node.js (cross-validator sidecar), serde_json (reference data parsing), clap (CLI extension)

**Spec:** `docs/superpowers/specs/2026-03-28-verification-system-design.md`

---

## File Structure

### New Crate: `mako-verify/`

| File | Responsibility |
|------|---------------|
| `mako-verify/Cargo.toml` | Crate config, depends on mako-types, mako-codec, mako-testdata (dev), serde, serde_json |
| `mako-verify/src/lib.rs` | Public API: `verifiziere_nachricht()`, `verifiziere_prozess_schritt()`, `verifiziere_batch()` |
| `mako-verify/src/referenzdaten.rs` | Load + cache AHB/EBD JSON files from disk |
| `mako-verify/src/ahb.rs` | Layer 1: AHB field-by-field validation (Muss/X/Wertpool/Format) |
| `mako-verify/src/ahb_ausdruck.rs` | AHB condition expression parser (boolean logic + static conditions) |
| `mako-verify/src/ebd.rs` | Layer 2: EBD outcome-oriented comparison |
| `mako-verify/src/interop.rs` | Layer 3: prepare messages for cross-validation, compare extracted fields |
| `mako-verify/src/bericht.rs` | Structured verification results (JSON-serializable) |
| `mako-verify/referenzdaten/ahb/FV2504/UTILMD/*.json` | Flat AHB reference data (downloaded from Hochfrequenz) |
| `mako-verify/referenzdaten/ahb/FV2504/MSCONS/*.json` | Flat AHB reference data |
| `mako-verify/referenzdaten/ebd/FV2504/*.json` | EBD decision tree reference data |
| `mako-verify/tests/ahb_tests.rs` | Layer 1 integration tests |
| `mako-verify/tests/ebd_tests.rs` | Layer 2 integration tests |
| `mako-verify/tests/interop_tests.rs` | Layer 3 integration tests |

### Modified: `mako-cli/`

| File | Change |
|------|--------|
| `mako-cli/Cargo.toml` | Add `mako-verify` dependency |
| `mako-cli/src/main.rs` | Add `Verifiziere` and `VerifiziereBatch` command variants |
| `mako-cli/src/verifiziere.rs` | New file: CLI handler for single message verification |
| `mako-cli/src/verifiziere_batch.rs` | New file: CLI handler for batch verification |

### Modified: `mako-ui/`

| File | Change |
|------|--------|
| `mako-ui/package.json` | Add `edifact-json-transformer` dependency |
| `mako-ui/src/server/index.ts` | Add `/api/verifiziere/` and `/api/verifiziere-batch` routes |
| `mako-ui/src/server/kreuzvalidator.ts` | New file: STROMDAO cross-validator wrapper |
| `mako-ui/src/lib/types.ts` | Add verification result types |
| `mako-ui/src/lib/api.ts` | Add verification API calls |
| `mako-ui/src/components/VerifikationsBadge.tsx` | New file: compact 3-indicator badge |
| `mako-ui/src/components/VerifikationsPanel.tsx` | New file: tabbed detail panel (AHB/EBD/Codec) |
| `mako-ui/src/components/EbdBaum.tsx` | New file: decision tree visualization |
| `mako-ui/src/components/BatchBericht.tsx` | New file: batch report with summary + drill-down |
| `mako-ui/src/components/MessageList.tsx` | Add VerifikationsBadge next to StatusBadge |
| `mako-ui/src/components/MessageDetail.tsx` | Add VerifikationsPanel section |
| `mako-ui/src/App.tsx` | Add batch verification button + BatchBericht modal |

---

## Task 1: Download and Commit Reference Data

**Files:**
- Create: `mako-verify/referenzdaten/ahb/FV2504/UTILMD/*.json` (flat AHB files)
- Create: `mako-verify/referenzdaten/ebd/FV2504/*.json` (EBD files)

- [ ] **Step 1: Clone Hochfrequenz AHB repo and copy FV2504 data**

```bash
cd /tmp
git clone --depth 1 https://github.com/Hochfrequenz/machine-readable_anwendungshandbuecher.git
```

Copy the FV2504 (or latest available FV) flatahb directory. The repo structure is `FV{YYMM}/{MESSAGE_TYPE}/flatahb/{pruefidentifikator}.json`. We need all message types:

```bash
mkdir -p mako-verify/referenzdaten/ahb/FV2504
# Copy each message type's flatahb directory
for msg_type in UTILMD MSCONS ORDERS ORDRSP QUOTES REQOTE INVOIC REMADV PRICAT IFTSTA PARTIN UTILTS INSRPT COMDIS; do
    src="/tmp/machine-readable_anwendungshandbuecher/FV2504/${msg_type}/flatahb"
    if [ -d "$src" ]; then
        mkdir -p "mako-verify/referenzdaten/ahb/FV2504/${msg_type}"
        cp "$src"/*.json "mako-verify/referenzdaten/ahb/FV2504/${msg_type}/"
    fi
done
```

If FV2504 doesn't exist yet, use the latest available (FV2410 or FV2504).

- [ ] **Step 2: Clone Hochfrequenz EBD repo and copy FV2504 data**

```bash
cd /tmp
git clone --depth 1 https://github.com/Hochfrequenz/machine-readable_entscheidungsbaumdiagramme.git
mkdir -p mako-verify/referenzdaten/ebd/FV2504
cp /tmp/machine-readable_entscheidungsbaumdiagramme/FV2504/*.json mako-verify/referenzdaten/ebd/FV2504/
```

Again, use latest available FV if FV2504 doesn't exist yet.

- [ ] **Step 3: Verify reference data is present**

```bash
ls mako-verify/referenzdaten/ahb/FV2504/UTILMD/ | head -10
# Expected: 11001.json, 11002.json, 11003.json, ... (or 44001.json etc.)
ls mako-verify/referenzdaten/ebd/FV2504/ | head -10
# Expected: E_0003.json, E_0401.json, E_0402.json, ...
```

- [ ] **Step 4: Commit reference data**

```bash
git add mako-verify/referenzdaten/
git commit -m "add Hochfrequenz AHB + EBD reference data (FV2504)"
```

---

## Task 2: Scaffold `mako-verify` Crate

**Files:**
- Create: `mako-verify/Cargo.toml`
- Create: `mako-verify/src/lib.rs`
- Modify: `Cargo.toml` (workspace root)

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "mako-verify"
edition.workspace = true
version.workspace = true
publish = false

[dependencies]
mako-types = { path = "../mako-types" }
mako-codec = { path = "../mako-codec" }
serde = { workspace = true }
serde_json = { workspace = true }

[dev-dependencies]
mako-testdata = { path = "../mako-testdata" }
```

- [ ] **Step 2: Create `src/lib.rs` with module declarations**

```rust
pub mod referenzdaten;
pub mod ahb;
pub mod ahb_ausdruck;
pub mod ebd;
pub mod interop;
pub mod bericht;
```

- [ ] **Step 3: Create placeholder modules**

Each module (`referenzdaten.rs`, `ahb.rs`, `ahb_ausdruck.rs`, `ebd.rs`, `interop.rs`, `bericht.rs`) starts as an empty file.

- [ ] **Step 4: Add to workspace**

In root `Cargo.toml`, add `"mako-verify"` to `workspace.members`.

- [ ] **Step 5: Verify it compiles**

```bash
cargo check -p mako-verify
```

Expected: compiles with no errors.

- [ ] **Step 6: Commit**

```bash
git add mako-verify/ Cargo.toml Cargo.lock
git commit -m "scaffold mako-verify crate"
```

---

## Task 3: Reference Data Loader (`referenzdaten.rs`)

**Files:**
- Create: `mako-verify/src/referenzdaten.rs`
- Test: `mako-verify/tests/ahb_tests.rs` (first test)

- [ ] **Step 1: Write failing test — load AHB for a known Pruefidentifikator**

In `mako-verify/tests/ahb_tests.rs`:

```rust
use mako_verify::referenzdaten::Referenzdaten;

#[test]
fn laedt_ahb_fuer_bekannten_pruefidentifikator() {
	let refdata = Referenzdaten::laden("referenzdaten");
	// Use a PI that exists in the downloaded data (e.g., 44001 for UTILMD or whatever is available)
	let ahb = refdata.ahb("UTILMD", "44001");
	assert!(ahb.is_some(), "AHB für 44001 sollte geladen werden");
	let ahb = ahb.unwrap();
	assert!(!ahb.lines.is_empty(), "AHB sollte Zeilen enthalten");
	// First line should be UNH segment
	assert_eq!(ahb.lines[0].segment_code.as_deref(), Some("UNH"));
}
```

- [ ] **Step 2: Run test — verify it fails**

```bash
cargo test -p mako-verify --test ahb_tests
```

Expected: FAIL (module not implemented)

- [ ] **Step 3: Define AHB data model and loader**

In `mako-verify/src/referenzdaten.rs`:

```rust
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct AhbDatei {
	pub meta: Option<AhbMeta>,
	pub lines: Vec<AhbZeile>,
}

#[derive(Debug, Deserialize)]
pub struct AhbMeta {
	pub description: Option<String>,
	pub direction: Option<String>,
	pub pruefidentifikator: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AhbZeile {
	pub ahb_expression: Option<String>,
	pub conditions: Option<String>,
	pub data_element: Option<String>,
	pub name: Option<String>,
	pub section_name: Option<String>,
	pub segment_code: Option<String>,
	pub segment_group_key: Option<String>,
	pub segment_id: Option<String>,
	pub value_pool_entry: Option<String>,
	pub guid: Option<String>,
	pub index: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct EbdDatei {
	pub metadata: EbdMetadata,
	pub multi_step_instructions: Option<Vec<EbdMultiStepInstruction>>,
	pub rows: Vec<EbdZeile>,
}

#[derive(Debug, Deserialize)]
pub struct EbdMetadata {
	pub chapter: Option<String>,
	pub ebd_code: String,
	pub ebd_name: String,
	pub remark: Option<String>,
	pub role: Option<String>,
	pub section: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EbdMultiStepInstruction {
	pub first_step_number_affected: String,
	pub instruction_text: String,
}

#[derive(Debug, Deserialize)]
pub struct EbdZeile {
	pub description: String,
	pub step_number: String,
	pub sub_rows: Vec<EbdSubRow>,
	pub use_cases: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct EbdSubRow {
	pub check_result: EbdCheckResult,
	pub note: Option<String>,
	pub result_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EbdCheckResult {
	pub result: bool,
	pub subsequent_step_number: Option<String>,
}

pub struct Referenzdaten {
	basis: PathBuf,
	ahb_cache: HashMap<String, AhbDatei>,
	ebd_cache: HashMap<String, EbdDatei>,
}

impl Referenzdaten {
	pub fn laden(basis: impl AsRef<Path>) -> Self {
		Self {
			basis: basis.as_ref().to_path_buf(),
			ahb_cache: HashMap::new(),
			ebd_cache: HashMap::new(),
		}
	}

	pub fn ahb(&mut self, nachrichtentyp: &str, pruefidentifikator: &str) -> Option<&AhbDatei> {
		let key = format!("{}/{}", nachrichtentyp, pruefidentifikator);
		if !self.ahb_cache.contains_key(&key) {
			let pfad = self.basis
				.join("ahb")
				.join("FV2504")
				.join(nachrichtentyp)
				.join(format!("{}.json", pruefidentifikator));
			if let Ok(inhalt) = fs::read_to_string(&pfad) {
				if let Ok(ahb) = serde_json::from_str::<AhbDatei>(&inhalt) {
					self.ahb_cache.insert(key.clone(), ahb);
				}
			}
		}
		self.ahb_cache.get(&key)
	}

	pub fn ebd(&mut self, ebd_code: &str) -> Option<&EbdDatei> {
		if !self.ebd_cache.contains_key(ebd_code) {
			let pfad = self.basis
				.join("ebd")
				.join("FV2504")
				.join(format!("{}.json", ebd_code));
			if let Ok(inhalt) = fs::read_to_string(&pfad) {
				if let Ok(ebd) = serde_json::from_str::<EbdDatei>(&inhalt) {
					self.ebd_cache.insert(ebd_code.to_string(), ebd);
				}
			}
		}
		self.ebd_cache.get(ebd_code)
	}
}
```

- [ ] **Step 4: Run test — verify it passes**

```bash
cargo test -p mako-verify --test ahb_tests
```

Expected: PASS

- [ ] **Step 5: Write test — load EBD for a known code**

In `mako-verify/tests/ebd_tests.rs`:

```rust
use mako_verify::referenzdaten::Referenzdaten;

#[test]
fn laedt_ebd_fuer_bekannten_code() {
	let mut refdata = Referenzdaten::laden("referenzdaten");
	// Use an EBD code that exists in the downloaded data
	let ebd = refdata.ebd("E_0003");
	assert!(ebd.is_some(), "EBD E_0003 sollte geladen werden");
	let ebd = ebd.unwrap();
	assert_eq!(ebd.metadata.ebd_code, "E_0003");
	assert!(!ebd.rows.is_empty(), "EBD sollte Zeilen enthalten");
}
```

- [ ] **Step 6: Run test — verify it passes**

```bash
cargo test -p mako-verify --test ebd_tests
```

Expected: PASS

- [ ] **Step 7: Write test — missing reference data returns None**

```rust
#[test]
fn fehlende_referenzdaten_gibt_none() {
	let mut refdata = Referenzdaten::laden("referenzdaten");
	assert!(refdata.ahb("UTILMD", "99999").is_none());
	assert!(refdata.ebd("E_9999").is_none());
}
```

- [ ] **Step 8: Run test — verify it passes**

```bash
cargo test -p mako-verify
```

Expected: all 3 tests PASS

- [ ] **Step 9: Commit**

```bash
git add mako-verify/
git commit -m "add reference data loader with AHB + EBD deserialization"
```

---

## Task 4: Result Types (`bericht.rs`)

**Files:**
- Create: `mako-verify/src/bericht.rs`

- [ ] **Step 1: Define verification result types**

```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Urteil {
	Bestanden,
	Fehlgeschlagen,
	NichtPruefbar,
}

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

#[derive(Debug, Clone, Serialize)]
pub struct AhbErgebnis {
	pub pruefidentifikator: String,
	pub nachrichtentyp: String,
	pub felder: Vec<AhbFeldErgebnis>,
	pub urteil: Urteil,
	pub zusammenfassung: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EbdAusgang {
	pub ebd_code: String,
	pub schritt: String,
	pub beschreibung: String,
	pub antwortcode: Option<String>,
	pub notiz: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EbdErgebnis {
	pub ebd_code: String,
	pub ebd_name: String,
	pub rolle: Option<String>,
	pub unser_ergebnis: String,
	pub gueltige_ausgaenge: Vec<EbdAusgang>,
	pub urteil: Urteil,
	pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InteropFeldVergleich {
	pub feld: String,
	pub unser_wert: String,
	pub drittanbieter_wert: String,
	pub stimmt_ueberein: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct InteropErgebnis {
	pub parse_ok_unser: bool,
	pub parse_ok_drittanbieter: Option<bool>,
	pub roundtrip_ok: bool,
	pub feldvergleiche: Vec<InteropFeldVergleich>,
	pub urteil: Urteil,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifikationsErgebnis {
	pub datei: String,
	pub nachrichtentyp: String,
	pub pruefidentifikator: Option<String>,
	pub ahb: Option<AhbErgebnis>,
	pub ebd: Option<EbdErgebnis>,
	pub interop: Option<InteropErgebnis>,
	pub gesamt_urteil: Urteil,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchErgebnis {
	pub gesamt: usize,
	pub bestanden: usize,
	pub fehlgeschlagen: usize,
	pub nicht_pruefbar: usize,
	pub ergebnisse: Vec<VerifikationsErgebnis>,
}

impl BatchErgebnis {
	pub fn zusammenfassung(&self) -> String {
		format!(
			"{} von {} bestanden, {} fehlgeschlagen, {} nicht prüfbar",
			self.bestanden, self.gesamt, self.fehlgeschlagen, self.nicht_pruefbar
		)
	}
}
```

- [ ] **Step 2: Verify it compiles**

```bash
cargo check -p mako-verify
```

Expected: compiles

- [ ] **Step 3: Commit**

```bash
git add mako-verify/src/bericht.rs
git commit -m "add verification result types"
```

---

## Task 5: AHB Field Validation — Core (`ahb.rs`)

**Files:**
- Create: `mako-verify/src/ahb.rs`
- Test: `mako-verify/tests/ahb_tests.rs` (extend)

- [ ] **Step 1: Write failing test — validate a known-good message**

In `mako-verify/tests/ahb_tests.rs`:

```rust
use mako_verify::ahb::validiere_nachricht_ahb;
use mako_verify::bericht::Urteil;
use mako_verify::referenzdaten::Referenzdaten;
use mako_codec::edifact::dispatch::parse_nachricht;
use mako_codec::edifact::segment::EdifactNachricht;
use mako_codec::edifact::parser::parse_edifact;

#[test]
fn known_good_utilmd_besteht_ahb_validierung() {
	let edi = mako_testdata::fixtures::utilmd::anmeldung_lfw_edi();
	let edifact = parse_edifact(&edi).unwrap();
	let nachricht = parse_nachricht(&edi).unwrap();
	let mut refdata = Referenzdaten::laden("referenzdaten");
	// Use the pruefidentifikator from the fixture
	let pi = nachricht.pruef_id.as_ref().unwrap().code().to_string();
	let ergebnis = validiere_nachricht_ahb(&edifact, &pi, "UTILMD", &mut refdata);
	assert_eq!(ergebnis.urteil, Urteil::Bestanden,
		"Known-good Nachricht sollte AHB bestehen: {}", ergebnis.zusammenfassung);
}
```

Note: the exact pruefidentifikator code used in the fixture must match a file in the reference data. If the fixture uses a PI not in the downloaded data, adapt the test to use one that is. The implementer should check `nachricht.pruef_id` and verify the corresponding JSON exists.

- [ ] **Step 2: Run test — verify it fails**

```bash
cargo test -p mako-verify --test ahb_tests known_good
```

Expected: FAIL (function not defined)

- [ ] **Step 3: Implement AHB validation**

In `mako-verify/src/ahb.rs`:

```rust
use crate::bericht::{AhbErgebnis, AhbFeldErgebnis, Urteil};
use crate::referenzdaten::{AhbZeile, Referenzdaten};
use mako_codec::edifact::segment::EdifactNachricht;

pub fn validiere_nachricht_ahb(
	edifact: &EdifactNachricht,
	pruefidentifikator: &str,
	nachrichtentyp: &str,
	refdata: &mut Referenzdaten,
) -> AhbErgebnis {
	let ahb = match refdata.ahb(nachrichtentyp, pruefidentifikator) {
		Some(ahb) => ahb,
		None => return AhbErgebnis {
			pruefidentifikator: pruefidentifikator.to_string(),
			nachrichtentyp: nachrichtentyp.to_string(),
			felder: vec![],
			urteil: Urteil::NichtPruefbar,
			zusammenfassung: format!("Keine Referenzdaten für PI {}", pruefidentifikator),
		},
	};

	let mut felder = Vec::new();
	let mut hat_fehler = false;

	for zeile in &ahb.lines {
		let ausdruck = zeile.ahb_expression.as_deref().unwrap_or("");
		// Skip lines that are section headers (no segment_code)
		let Some(segment_code) = zeile.segment_code.as_deref() else {
			continue;
		};

		let ergebnis = pruefe_feld(edifact, &zeile, segment_code, ausdruck);
		if ergebnis.urteil == Urteil::Fehlgeschlagen {
			hat_fehler = true;
		}
		felder.push(ergebnis);
	}

	let bestanden = felder.iter().filter(|f| f.urteil == Urteil::Bestanden).count();
	let fehlgeschlagen = felder.iter().filter(|f| f.urteil == Urteil::Fehlgeschlagen).count();
	let nicht_pruefbar = felder.iter().filter(|f| f.urteil == Urteil::NichtPruefbar).count();

	AhbErgebnis {
		pruefidentifikator: pruefidentifikator.to_string(),
		nachrichtentyp: nachrichtentyp.to_string(),
		felder,
		urteil: if hat_fehler { Urteil::Fehlgeschlagen } else { Urteil::Bestanden },
		zusammenfassung: format!("{} bestanden, {} fehlgeschlagen, {} nicht prüfbar",
			bestanden, fehlgeschlagen, nicht_pruefbar),
	}
}

fn pruefe_feld(
	edifact: &EdifactNachricht,
	zeile: &AhbZeile,
	segment_code: &str,
	ausdruck: &str,
) -> AhbFeldErgebnis {
	let name = zeile.name.clone().unwrap_or_default();
	let data_element = zeile.data_element.as_deref();
	let expected_value = zeile.value_pool_entry.as_deref();

	// Find matching segment(s) in the EDIFACT message
	let matching_segments: Vec<_> = edifact.segmente.iter()
		.filter(|s| s.tag == segment_code)
		.collect();

	let feld_vorhanden = !matching_segments.is_empty();

	// Check the actual value if we have a data_element to look for
	let actual_value = if let Some(de) = data_element {
		finde_wert(edifact, segment_code, de)
	} else {
		None
	};

	match ausdruck {
		"Muss" | "M" => {
			if !feld_vorhanden {
				return AhbFeldErgebnis {
					segment_code: Some(segment_code.to_string()),
					segment_group: zeile.segment_group_key.clone(),
					data_element: data_element.map(|s| s.to_string()),
					name,
					ahb_ausdruck: ausdruck.to_string(),
					unser_wert: None,
					erwarteter_wert: expected_value.map(|s| s.to_string()),
					urteil: Urteil::Fehlgeschlagen,
					details: Some(format!("Pflichtfeld fehlt: {}/{}", segment_code,
						data_element.unwrap_or("?"))),
				};
			}
			if let Some(expected) = expected_value {
				if let Some(actual) = &actual_value {
					if actual != expected {
						return AhbFeldErgebnis {
							segment_code: Some(segment_code.to_string()),
							segment_group: zeile.segment_group_key.clone(),
							data_element: data_element.map(|s| s.to_string()),
							name,
							ahb_ausdruck: ausdruck.to_string(),
							unser_wert: actual_value,
							erwarteter_wert: Some(expected.to_string()),
							urteil: Urteil::Fehlgeschlagen,
							details: Some(format!("Wert '{}' erwartet, '{}' gefunden", expected, actual)),
						};
					}
				}
			}
			AhbFeldErgebnis {
				segment_code: Some(segment_code.to_string()),
				segment_group: zeile.segment_group_key.clone(),
				data_element: data_element.map(|s| s.to_string()),
				name,
				ahb_ausdruck: ausdruck.to_string(),
				unser_wert: actual_value,
				erwarteter_wert: expected_value.map(|s| s.to_string()),
				urteil: Urteil::Bestanden,
				details: None,
			}
		}
		"X" => {
			if feld_vorhanden {
				AhbFeldErgebnis {
					segment_code: Some(segment_code.to_string()),
					segment_group: zeile.segment_group_key.clone(),
					data_element: data_element.map(|s| s.to_string()),
					name,
					ahb_ausdruck: ausdruck.to_string(),
					unser_wert: actual_value,
					erwarteter_wert: None,
					urteil: Urteil::Fehlgeschlagen,
					details: Some(format!("Verbotenes Feld vorhanden: {}", segment_code)),
				}
			} else {
				AhbFeldErgebnis {
					segment_code: Some(segment_code.to_string()),
					segment_group: zeile.segment_group_key.clone(),
					data_element: data_element.map(|s| s.to_string()),
					name,
					ahb_ausdruck: ausdruck.to_string(),
					unser_wert: None,
					erwarteter_wert: None,
					urteil: Urteil::Bestanden,
					details: None,
				}
			}
		}
		expr if expr.contains('[') => {
			// Conditional expression — delegate to ahb_ausdruck module later
			// For now: mark as NichtPruefbar
			AhbFeldErgebnis {
				segment_code: Some(segment_code.to_string()),
				segment_group: zeile.segment_group_key.clone(),
				data_element: data_element.map(|s| s.to_string()),
				name,
				ahb_ausdruck: ausdruck.to_string(),
				unser_wert: actual_value,
				erwarteter_wert: expected_value.map(|s| s.to_string()),
				urteil: Urteil::NichtPruefbar,
				details: Some("Bedingungsausdruck — noch nicht auswertbar".to_string()),
			}
		}
		_ => {
			// "Kann", "Soll", "K", or unknown — presence is optional
			AhbFeldErgebnis {
				segment_code: Some(segment_code.to_string()),
				segment_group: zeile.segment_group_key.clone(),
				data_element: data_element.map(|s| s.to_string()),
				name,
				ahb_ausdruck: ausdruck.to_string(),
				unser_wert: actual_value,
				erwarteter_wert: expected_value.map(|s| s.to_string()),
				urteil: Urteil::Bestanden,
				details: None,
			}
		}
	}
}

fn finde_wert(edifact: &EdifactNachricht, segment_code: &str, data_element: &str) -> Option<String> {
	// This is a simplified lookup — the real implementation needs to map
	// data_element IDs to segment+element positions based on EDIFACT structure.
	// For V1, we search for the segment and return the first non-empty element.
	// The implementer should refine this based on the actual data_element → position mapping.
	for segment in &edifact.segmente {
		if segment.tag == segment_code {
			for element in &segment.elements {
				for component in &element.components {
					if !component.is_empty() {
						return Some(component.clone());
					}
				}
			}
		}
	}
	None
}
```

**Important note for implementer:** The `finde_wert` function above is a placeholder. The real mapping from data_element IDs (like "2380", "3039") to specific positions in the EDIFACT segment requires knowledge of the EDIFACT service segment definitions. The implementer should study the existing `mako-codec/src/edifact/bdew_segmente.rs` for how segments are structured and map data elements to `segment.elements[n].components[m]` positions. Start with the simplified version and refine as tests reveal mismatches.

- [ ] **Step 4: Run test — verify it passes**

```bash
cargo test -p mako-verify --test ahb_tests known_good
```

Expected: PASS (or identify which fields need adjustment in the lookup logic)

- [ ] **Step 5: Write test — known-bad message (missing mandatory field)**

```rust
#[test]
fn fehlende_pflichtfelder_werden_erkannt() {
	// Use mako-testdata's error injector to create a bad message,
	// or manually construct an EDIFACT string with a missing segment.
	// The exact approach depends on what mako_testdata::fehler provides.
	let edi = mako_testdata::fixtures::utilmd::anmeldung_lfw_edi();
	// Remove a mandatory segment (e.g., strip out NAD+MS line)
	let bad_edi = edi.lines()
		.filter(|line| !line.starts_with("NAD+MS"))
		.collect::<Vec<_>>()
		.join("\n");
	// Parse what we can from the raw segments
	let edifact = mako_codec::edifact::parser::parse_edifact(&bad_edi).unwrap();
	let mut refdata = Referenzdaten::laden("referenzdaten");
	let ergebnis = validiere_nachricht_ahb(&edifact, "44001", "UTILMD", &mut refdata);
	assert_eq!(ergebnis.urteil, Urteil::Fehlgeschlagen);
	let fehler: Vec<_> = ergebnis.felder.iter()
		.filter(|f| f.urteil == Urteil::Fehlgeschlagen)
		.collect();
	assert!(!fehler.is_empty(), "Sollte fehlende Pflichtfelder erkennen");
}
```

Note: the exact approach to creating a bad message depends on how EDIFACT segments are delimited in the raw string (typically `'` as segment terminator). The implementer should adapt based on the actual fixture format.

- [ ] **Step 6: Run test — verify it passes**

```bash
cargo test -p mako-verify --test ahb_tests fehlende
```

Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add mako-verify/src/ahb.rs mako-verify/tests/ahb_tests.rs
git commit -m "add AHB field validation (Muss/X/Kann/conditional placeholder)"
```

---

## Task 6: AHB Condition Expression Parser (`ahb_ausdruck.rs`)

**Files:**
- Create: `mako-verify/src/ahb_ausdruck.rs`
- Test: inline unit tests

- [ ] **Step 1: Write failing test — parse simple Muss expression**

In `mako-verify/src/ahb_ausdruck.rs` (at bottom, in `#[cfg(test)]`):

```rust
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parst_einfaches_muss() {
		let ergebnis = parse_ahb_ausdruck("Muss");
		assert_eq!(ergebnis, AhbAusdruck::Muss);
	}

	#[test]
	fn parst_bedingung_mit_und() {
		let ergebnis = parse_ahb_ausdruck("Muss [556] ∧ [559]");
		assert_eq!(ergebnis, AhbAusdruck::Bedingt {
			basis: Box::new(AhbAusdruck::Muss),
			bedingung: Bedingung::Und(
				Box::new(Bedingung::Ref(556)),
				Box::new(Bedingung::Ref(559)),
			),
		});
	}

	#[test]
	fn parst_bedingung_mit_oder() {
		let ergebnis = parse_ahb_ausdruck("Kann [931] ∨ [932]");
		assert_eq!(ergebnis, AhbAusdruck::Bedingt {
			basis: Box::new(AhbAusdruck::Kann),
			bedingung: Bedingung::Oder(
				Box::new(Bedingung::Ref(931)),
				Box::new(Bedingung::Ref(932)),
			),
		});
	}
}
```

- [ ] **Step 2: Run tests — verify they fail**

```bash
cargo test -p mako-verify ahb_ausdruck
```

Expected: FAIL

- [ ] **Step 3: Implement expression parser**

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum AhbAusdruck {
	Muss,
	Soll,
	Kann,
	X,
	Bedingt {
		basis: Box<AhbAusdruck>,
		bedingung: Bedingung,
	},
	Unbekannt(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Bedingung {
	Ref(u32),
	Und(Box<Bedingung>, Box<Bedingung>),
	Oder(Box<Bedingung>, Box<Bedingung>),
	XOder(Box<Bedingung>, Box<Bedingung>),
	Nicht(Box<Bedingung>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BedingungsZustand {
	Wahr,
	Falsch,
	Unbestimmt,
}

pub fn parse_ahb_ausdruck(input: &str) -> AhbAusdruck {
	let input = input.trim();
	match input {
		"Muss" | "M" => AhbAusdruck::Muss,
		"Soll" | "S" => AhbAusdruck::Soll,
		"Kann" | "K" => AhbAusdruck::Kann,
		"X" => AhbAusdruck::X,
		s if s.contains('[') => {
			// Try to parse as "Basis [conditions]"
			let (basis_str, rest) = if s.starts_with("Muss ") {
				("Muss", &s[5..])
			} else if s.starts_with("Soll ") {
				("Soll", &s[5..])
			} else if s.starts_with("Kann ") {
				("Kann", &s[5..])
			} else if s.starts_with("X ") {
				("X", &s[2..])
			} else {
				return AhbAusdruck::Unbekannt(s.to_string());
			};
			let basis = Box::new(parse_ahb_ausdruck(basis_str));
			match parse_bedingung(rest.trim()) {
				Some(bed) => AhbAusdruck::Bedingt { basis, bedingung: bed },
				None => AhbAusdruck::Unbekannt(s.to_string()),
			}
		}
		other => AhbAusdruck::Unbekannt(other.to_string()),
	}
}

fn parse_bedingung(input: &str) -> Option<Bedingung> {
	let input = input.trim();

	// Check for binary operators (∧ = und, ∨ = oder, ⊻ = xoder)
	// Also support legacy: U = und, O = oder, X = xoder
	if let Some(pos) = find_operator(input, &["∧", " U "]) {
		let (left, right) = split_at_operator(input, pos, &["∧", " U "]);
		let l = parse_bedingung(left)?;
		let r = parse_bedingung(right)?;
		return Some(Bedingung::Und(Box::new(l), Box::new(r)));
	}
	if let Some(pos) = find_operator(input, &["∨", " O "]) {
		let (left, right) = split_at_operator(input, pos, &["∨", " O "]);
		let l = parse_bedingung(left)?;
		let r = parse_bedingung(right)?;
		return Some(Bedingung::Oder(Box::new(l), Box::new(r)));
	}
	if let Some(pos) = find_operator(input, &["⊻", " X "]) {
		let (left, right) = split_at_operator(input, pos, &["⊻", " X "]);
		let l = parse_bedingung(left)?;
		let r = parse_bedingung(right)?;
		return Some(Bedingung::XOder(Box::new(l), Box::new(r)));
	}

	// Single reference: [NNN]
	if input.starts_with('[') && input.ends_with(']') {
		let inner = &input[1..input.len()-1];
		if let Ok(num) = inner.parse::<u32>() {
			return Some(Bedingung::Ref(num));
		}
	}

	None
}

fn find_operator(input: &str, operators: &[&str]) -> Option<usize> {
	for op in operators {
		if let Some(pos) = input.find(op) {
			return Some(pos);
		}
	}
	None
}

fn split_at_operator<'a>(input: &'a str, pos: usize, operators: &[&str]) -> (&'a str, &'a str) {
	for op in operators {
		if input[pos..].starts_with(op) {
			return (input[..pos].trim(), input[pos + op.len()..].trim());
		}
	}
	unreachable!()
}

pub fn auswerten(bedingung: &Bedingung, zustaende: &dyn Fn(u32) -> BedingungsZustand) -> BedingungsZustand {
	match bedingung {
		Bedingung::Ref(nr) => zustaende(*nr),
		Bedingung::Und(l, r) => {
			let lz = auswerten(l, zustaende);
			let rz = auswerten(r, zustaende);
			match (lz, rz) {
				(BedingungsZustand::Wahr, BedingungsZustand::Wahr) => BedingungsZustand::Wahr,
				(BedingungsZustand::Falsch, _) | (_, BedingungsZustand::Falsch) => BedingungsZustand::Falsch,
				_ => BedingungsZustand::Unbestimmt,
			}
		}
		Bedingung::Oder(l, r) => {
			let lz = auswerten(l, zustaende);
			let rz = auswerten(r, zustaende);
			match (lz, rz) {
				(BedingungsZustand::Wahr, _) | (_, BedingungsZustand::Wahr) => BedingungsZustand::Wahr,
				(BedingungsZustand::Falsch, BedingungsZustand::Falsch) => BedingungsZustand::Falsch,
				_ => BedingungsZustand::Unbestimmt,
			}
		}
		Bedingung::XOder(l, r) => {
			let lz = auswerten(l, zustaende);
			let rz = auswerten(r, zustaende);
			match (lz, rz) {
				(BedingungsZustand::Wahr, BedingungsZustand::Falsch) |
				(BedingungsZustand::Falsch, BedingungsZustand::Wahr) => BedingungsZustand::Wahr,
				(BedingungsZustand::Wahr, BedingungsZustand::Wahr) |
				(BedingungsZustand::Falsch, BedingungsZustand::Falsch) => BedingungsZustand::Falsch,
				_ => BedingungsZustand::Unbestimmt,
			}
		}
		Bedingung::Nicht(inner) => {
			match auswerten(inner, zustaende) {
				BedingungsZustand::Wahr => BedingungsZustand::Falsch,
				BedingungsZustand::Falsch => BedingungsZustand::Wahr,
				BedingungsZustand::Unbestimmt => BedingungsZustand::Unbestimmt,
			}
		}
	}
}
```

- [ ] **Step 4: Run tests — verify they pass**

```bash
cargo test -p mako-verify ahb_ausdruck
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add mako-verify/src/ahb_ausdruck.rs
git commit -m "add AHB condition expression parser with boolean evaluation"
```

---

## Task 7: EBD Outcome Comparison (`ebd.rs`)

**Files:**
- Create: `mako-verify/src/ebd.rs`
- Test: `mako-verify/tests/ebd_tests.rs` (extend)

- [ ] **Step 1: Write failing test — extract valid outcomes from an EBD**

In `mako-verify/tests/ebd_tests.rs`:

```rust
use mako_verify::ebd::extrahiere_ausgaenge;
use mako_verify::referenzdaten::Referenzdaten;

#[test]
fn extrahiert_gueltige_ausgaenge_aus_ebd() {
	let mut refdata = Referenzdaten::laden("referenzdaten");
	let ebd = refdata.ebd("E_0003").unwrap();
	let ausgaenge = extrahiere_ausgaenge(ebd);
	// E_0003 should have at least one terminal outcome with a result_code
	assert!(!ausgaenge.is_empty(), "EBD sollte Ausgänge haben");
	// Each outcome should have a step number and either a result_code or "Ende"
	for ausgang in &ausgaenge {
		assert!(!ausgang.schritt.is_empty());
	}
}
```

- [ ] **Step 2: Run test — verify it fails**

```bash
cargo test -p mako-verify --test ebd_tests extrahiert
```

Expected: FAIL

- [ ] **Step 3: Implement EBD outcome extraction + comparison**

In `mako-verify/src/ebd.rs`:

```rust
use crate::bericht::{EbdAusgang, EbdErgebnis, Urteil};
use crate::referenzdaten::{EbdDatei, Referenzdaten};

pub fn extrahiere_ausgaenge(ebd: &EbdDatei) -> Vec<EbdAusgang> {
	let mut ausgaenge = Vec::new();
	for zeile in &ebd.rows {
		for sub_row in &zeile.sub_rows {
			// Terminal outcomes: have a result_code (rejection) or
			// subsequent_step_number is None or "Ende" (acceptance)
			let ist_terminal = sub_row.result_code.is_some()
				|| sub_row.check_result.subsequent_step_number.is_none()
				|| sub_row.check_result.subsequent_step_number.as_deref() == Some("Ende");

			if ist_terminal {
				ausgaenge.push(EbdAusgang {
					ebd_code: ebd.metadata.ebd_code.clone(),
					schritt: zeile.step_number.clone(),
					beschreibung: zeile.description.clone(),
					antwortcode: sub_row.result_code.clone(),
					notiz: sub_row.note.clone(),
				});
			}
		}
	}
	ausgaenge
}

pub fn vergleiche_ergebnis(
	ebd_code: &str,
	unser_antwortcode: Option<&str>,
	unser_ergebnis_beschreibung: &str,
	refdata: &mut Referenzdaten,
) -> EbdErgebnis {
	let ebd = match refdata.ebd(ebd_code) {
		Some(ebd) => ebd,
		None => return EbdErgebnis {
			ebd_code: ebd_code.to_string(),
			ebd_name: String::new(),
			rolle: None,
			unser_ergebnis: unser_ergebnis_beschreibung.to_string(),
			gueltige_ausgaenge: vec![],
			urteil: Urteil::NichtPruefbar,
			details: Some(format!("Keine Referenzdaten für EBD {}", ebd_code)),
		},
	};

	let gueltige_ausgaenge = extrahiere_ausgaenge(ebd);

	// Check if our result_code matches any terminal outcome
	let stimmt_ueberein = match unser_antwortcode {
		Some(code) => gueltige_ausgaenge.iter().any(|a| a.antwortcode.as_deref() == Some(code)),
		None => {
			// No rejection code = acceptance path
			// Check if there's a terminal outcome without a result_code (success path)
			gueltige_ausgaenge.iter().any(|a| a.antwortcode.is_none())
		}
	};

	EbdErgebnis {
		ebd_code: ebd_code.to_string(),
		ebd_name: ebd.metadata.ebd_name.clone(),
		rolle: ebd.metadata.role.clone(),
		unser_ergebnis: unser_ergebnis_beschreibung.to_string(),
		gueltige_ausgaenge,
		urteil: if stimmt_ueberein { Urteil::Bestanden } else { Urteil::Fehlgeschlagen },
		details: if stimmt_ueberein {
			Some("Unser Ergebnis ist ein gültiger EBD-Ausgang".to_string())
		} else {
			Some(format!("Antwortcode {:?} ist kein gültiger Ausgang von {}",
				unser_antwortcode, ebd_code))
		},
	}
}
```

- [ ] **Step 4: Run tests — verify they pass**

```bash
cargo test -p mako-verify --test ebd_tests
```

Expected: PASS

- [ ] **Step 5: Write test — outcome comparison for acceptance**

```rust
#[test]
fn akzeptanz_ist_gueltiger_ausgang() {
	let mut refdata = Referenzdaten::laden("referenzdaten");
	// Use an EBD that has a success path (no result_code)
	let ergebnis = mako_verify::ebd::vergleiche_ergebnis(
		"E_0003",
		None, // acceptance = no rejection code
		"Bestätigung gesendet",
		&mut refdata,
	);
	// This depends on E_0003 actually having a success path
	// Adapt the assertion based on what E_0003 contains
	assert!(ergebnis.urteil == Urteil::Bestanden || ergebnis.urteil == Urteil::NichtPruefbar);
}
```

- [ ] **Step 6: Run test — verify it passes**

```bash
cargo test -p mako-verify --test ebd_tests akzeptanz
```

- [ ] **Step 7: Commit**

```bash
git add mako-verify/src/ebd.rs mako-verify/tests/ebd_tests.rs
git commit -m "add EBD outcome extraction, comparison against reducer results"
```

---

## Task 8: Interop Preparation (`interop.rs`)

**Files:**
- Create: `mako-verify/src/interop.rs`
- Test: `mako-verify/tests/interop_tests.rs`

- [ ] **Step 1: Write failing test — extract key fields from a parsed Nachricht**

In `mako-verify/tests/interop_tests.rs`:

```rust
use mako_verify::interop::extrahiere_schluesselfelder;
use mako_codec::edifact::dispatch::parse_nachricht;

#[test]
fn extrahiert_schluesselfelder_aus_utilmd() {
	let edi = mako_testdata::fixtures::utilmd::anmeldung_lfw_edi();
	let nachricht = parse_nachricht(&edi).unwrap();
	let felder = extrahiere_schluesselfelder(&nachricht);
	assert!(felder.contains_key("absender"));
	assert!(felder.contains_key("empfaenger"));
	// UTILMD Anmeldung should have a malo_id
	assert!(felder.contains_key("malo_id") || felder.contains_key("absender_rolle"));
}
```

- [ ] **Step 2: Run test — verify it fails**

```bash
cargo test -p mako-verify --test interop_tests
```

Expected: FAIL

- [ ] **Step 3: Implement key field extraction**

In `mako-verify/src/interop.rs`:

```rust
use std::collections::HashMap;
use mako_types::nachricht::Nachricht;
use crate::bericht::{InteropErgebnis, InteropFeldVergleich, Urteil};

pub fn extrahiere_schluesselfelder(nachricht: &Nachricht) -> HashMap<String, String> {
	let mut felder = HashMap::new();
	felder.insert("absender".to_string(), nachricht.absender.to_string());
	felder.insert("empfaenger".to_string(), nachricht.empfaenger.to_string());
	felder.insert("absender_rolle".to_string(), format!("{:?}", nachricht.absender_rolle));
	felder.insert("empfaenger_rolle".to_string(), format!("{:?}", nachricht.empfaenger_rolle));
	if let Some(pi) = &nachricht.pruef_id {
		felder.insert("pruefidentifikator".to_string(), pi.code().to_string());
	}
	// Extract payload-specific fields based on NachrichtenPayload variant
	// The implementer should add match arms for the key variants (UtilmdAnmeldung, etc.)
	// to extract malo_id, lieferbeginn, etc.
	felder
}

pub fn vergleiche_felder(
	unsere: &HashMap<String, String>,
	drittanbieter: &HashMap<String, String>,
) -> InteropErgebnis {
	let mut vergleiche = Vec::new();
	let mut alle_stimmen = true;

	for (feld, unser_wert) in unsere {
		if let Some(deren_wert) = drittanbieter.get(feld) {
			let stimmt = unser_wert == deren_wert;
			if !stimmt {
				alle_stimmen = false;
			}
			vergleiche.push(InteropFeldVergleich {
				feld: feld.clone(),
				unser_wert: unser_wert.clone(),
				drittanbieter_wert: deren_wert.clone(),
				stimmt_ueberein: stimmt,
			});
		}
	}

	InteropErgebnis {
		parse_ok_unser: true,
		parse_ok_drittanbieter: Some(true),
		roundtrip_ok: true, // set externally
		feldvergleiche: vergleiche,
		urteil: if alle_stimmen { Urteil::Bestanden } else { Urteil::Fehlgeschlagen },
	}
}
```

- [ ] **Step 4: Run test — verify it passes**

```bash
cargo test -p mako-verify --test interop_tests
```

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add mako-verify/src/interop.rs mako-verify/tests/interop_tests.rs
git commit -m "add interop key field extraction, comparison logic"
```

---

## Task 9: Public API (`lib.rs`)

**Files:**
- Modify: `mako-verify/src/lib.rs`

- [ ] **Step 1: Write the public API wiring**

```rust
pub mod referenzdaten;
pub mod ahb;
pub mod ahb_ausdruck;
pub mod ebd;
pub mod interop;
pub mod bericht;

use bericht::{VerifikationsErgebnis, BatchErgebnis, Urteil};
use mako_codec::edifact::dispatch::parse_nachricht;
use mako_codec::edifact::parser::parse_edifact;
use referenzdaten::Referenzdaten;
use std::path::Path;

pub fn verifiziere_nachricht(
	edifact_roh: &str,
	refdata: &mut Referenzdaten,
) -> VerifikationsErgebnis {
	// Parse with our codec
	let edifact = match parse_edifact(edifact_roh) {
		Ok(e) => e,
		Err(err) => return VerifikationsErgebnis {
			datei: String::new(),
			nachrichtentyp: String::new(),
			pruefidentifikator: None,
			ahb: None,
			ebd: None,
			interop: None,
			gesamt_urteil: Urteil::Fehlgeschlagen,
		},
	};

	let nachricht = parse_nachricht(edifact_roh).ok();
	let nachrichtentyp = edifact.typ.clone();
	let pruefidentifikator = nachricht.as_ref()
		.and_then(|n| n.pruef_id.as_ref())
		.map(|pi| pi.code().to_string());

	// Layer 1: AHB validation
	let ahb_ergebnis = pruefidentifikator.as_ref().map(|pi| {
		ahb::validiere_nachricht_ahb(&edifact, pi, &nachrichtentyp, refdata)
	});

	// Layer 2: EBD — requires process context, not available for single-message verification
	// EBD comparison is done via verifiziere_prozess_schritt()

	// Layer 3: Interop — field extraction for cross-validation
	let interop_felder = nachricht.as_ref().map(|n| interop::extrahiere_schluesselfelder(n));

	let gesamt = match &ahb_ergebnis {
		Some(ahb) => ahb.urteil.clone(),
		None => Urteil::NichtPruefbar,
	};

	VerifikationsErgebnis {
		datei: String::new(),
		nachrichtentyp,
		pruefidentifikator,
		ahb: ahb_ergebnis,
		ebd: None,
		interop: None,
		gesamt_urteil: gesamt,
	}
}

pub fn verifiziere_batch(
	sim_verzeichnis: &Path,
	refdata: &mut Referenzdaten,
) -> BatchErgebnis {
	let mut ergebnisse = Vec::new();
	// Walk all .edi files in the simulation directory
	if let Ok(entries) = std::fs::read_dir(sim_verzeichnis) {
		for entry in entries.flatten() {
			let pfad = entry.path();
			if pfad.is_dir() {
				// Recurse into subdirectories (nachrichten/<kette>/)
				if let Ok(sub_entries) = std::fs::read_dir(&pfad) {
					for sub_entry in sub_entries.flatten() {
						let sub_pfad = sub_entry.path();
						if sub_pfad.extension().and_then(|e| e.to_str()) == Some("edi") {
							if let Ok(inhalt) = std::fs::read_to_string(&sub_pfad) {
								let mut erg = verifiziere_nachricht(&inhalt, refdata);
								erg.datei = sub_pfad.display().to_string();
								ergebnisse.push(erg);
							}
						}
					}
				}
			} else if pfad.extension().and_then(|e| e.to_str()) == Some("edi") {
				if let Ok(inhalt) = std::fs::read_to_string(&pfad) {
					let mut erg = verifiziere_nachricht(&inhalt, refdata);
					erg.datei = pfad.display().to_string();
					ergebnisse.push(erg);
				}
			}
		}
	}

	let gesamt = ergebnisse.len();
	let bestanden = ergebnisse.iter().filter(|e| e.gesamt_urteil == Urteil::Bestanden).count();
	let fehlgeschlagen = ergebnisse.iter().filter(|e| e.gesamt_urteil == Urteil::Fehlgeschlagen).count();
	let nicht_pruefbar = ergebnisse.iter().filter(|e| e.gesamt_urteil == Urteil::NichtPruefbar).count();

	BatchErgebnis { gesamt, bestanden, fehlgeschlagen, nicht_pruefbar, ergebnisse }
}
```

- [ ] **Step 2: Verify it compiles**

```bash
cargo check -p mako-verify
```

- [ ] **Step 3: Commit**

```bash
git add mako-verify/src/lib.rs
git commit -m "wire up public verification API"
```

---

## Task 10: CLI Commands (`verifiziere`, `verifiziere-batch`)

**Files:**
- Create: `mako-cli/src/verifiziere.rs`
- Create: `mako-cli/src/verifiziere_batch.rs`
- Modify: `mako-cli/src/main.rs`
- Modify: `mako-cli/Cargo.toml`

- [ ] **Step 1: Add mako-verify dependency to mako-cli**

In `mako-cli/Cargo.toml`, add under `[dependencies]`:

```toml
mako-verify = { path = "../mako-verify" }
```

- [ ] **Step 2: Add command variants to main.rs**

In the `Commands` enum in `mako-cli/src/main.rs`, add:

```rust
Verifiziere {
	datei: String,
	#[arg(long, default_value = "referenzdaten")]
	referenzdaten: String,
},
VerifiziereBatch {
	verzeichnis: String,
	#[arg(long, default_value = "referenzdaten")]
	referenzdaten: String,
},
```

Add match arms:

```rust
Commands::Verifiziere { datei, referenzdaten } => verifiziere::run(&datei, &referenzdaten),
Commands::VerifiziereBatch { verzeichnis, referenzdaten } => verifiziere_batch::run(&verzeichnis, &referenzdaten),
```

Add `mod verifiziere;` and `mod verifiziere_batch;` at the top.

- [ ] **Step 3: Implement `verifiziere.rs`**

```rust
use mako_verify::referenzdaten::Referenzdaten;

pub fn run(datei: &str, referenzdaten_pfad: &str) -> Result<(), Box<dyn std::error::Error>> {
	let inhalt = std::fs::read_to_string(datei)?;
	let mut refdata = Referenzdaten::laden(referenzdaten_pfad);
	let ergebnis = mako_verify::verifiziere_nachricht(&inhalt, &mut refdata);
	let json = serde_json::to_string_pretty(&ergebnis)?;
	println!("{}", json);
	Ok(())
}
```

- [ ] **Step 4: Implement `verifiziere_batch.rs`**

```rust
use mako_verify::referenzdaten::Referenzdaten;
use std::path::Path;

pub fn run(verzeichnis: &str, referenzdaten_pfad: &str) -> Result<(), Box<dyn std::error::Error>> {
	let mut refdata = Referenzdaten::laden(referenzdaten_pfad);
	let ergebnis = mako_verify::verifiziere_batch(Path::new(verzeichnis), &mut refdata);
	println!("{}", ergebnis.zusammenfassung());
	let json = serde_json::to_string_pretty(&ergebnis)?;
	// Write batch report to file
	let report_path = Path::new(verzeichnis).join("verifikation.json");
	std::fs::write(&report_path, &json)?;
	println!("Bericht geschrieben: {}", report_path.display());
	Ok(())
}
```

- [ ] **Step 5: Verify it compiles**

```bash
cargo check -p mako-cli
```

- [ ] **Step 6: Test CLI with a fixture**

```bash
# Generate a test .edi file from a fixture, then verify it
cargo run -p mako-cli -- verifiziere test_message.edi --referenzdaten mako-verify/referenzdaten
```

The implementer should create a small helper or use an existing test fixture .edi file.

- [ ] **Step 7: Commit**

```bash
git add mako-cli/
git commit -m "add verifiziere, verifiziere-batch CLI commands"
```

---

## Task 11: STROMDAO Cross-Validator Spike

**Files:**
- Modify: `mako-ui/package.json`
- Create: `mako-ui/src/server/kreuzvalidator.ts`

- [ ] **Step 1: Install edifact-json-transformer**

```bash
cd mako-ui
bun add edifact-json-transformer
```

- [ ] **Step 2: Create kreuzvalidator.ts**

```typescript
import type { Request, Response } from "express";
import express from "express";

// Dynamic import to handle if package isn't installed
let EdifactTransformer: unknown = null;

try {
	const mod = await import("edifact-json-transformer");
	EdifactTransformer = mod.EdifactTransformer;
} catch {
	console.warn("edifact-json-transformer nicht installiert — Kreuzvalidierung deaktiviert");
}

export function isAvailable(): boolean {
	return EdifactTransformer !== null;
}

export function kreuzvalidiere(edifactRoh: string): {
	ok: boolean;
	felder?: Record<string, string>;
	fehler?: string;
} {
	if (!EdifactTransformer) {
		return { ok: false, fehler: "Kreuzvalidator nicht verfügbar" };
	}
	try {
		// @ts-expect-error dynamic import
		const transformer = new EdifactTransformer({ enableAHBValidation: true });
		const result = transformer.transform(edifactRoh);
		// Extract key fields from the transformer output
		// The exact field paths depend on the transformer's JSON output structure
		// The implementer should inspect result.metadata and result.body to build this mapping
		const felder: Record<string, string> = {};
		if (result?.metadata?.message_type) felder["nachrichtentyp"] = result.metadata.message_type;
		if (result?.metadata?.sender) felder["absender"] = result.metadata.sender;
		if (result?.metadata?.receiver) felder["empfaenger"] = result.metadata.receiver;
		// Add more field extraction as the output structure is understood
		return { ok: true, felder };
	} catch (e) {
		return { ok: false, fehler: String(e) };
	}
}

export function registerRoutes(app: ReturnType<typeof express>) {
	app.post("/api/kreuzvalidiere", (req: Request, res: Response) => {
		const { edifact } = req.body;
		if (!edifact) {
			res.status(400).json({ ok: false, fehler: "edifact fehlt" });
			return;
		}
		const ergebnis = kreuzvalidiere(edifact);
		res.json(ergebnis);
	});

	app.get("/api/kreuzvalidator-status", (_req: Request, res: Response) => {
		res.json({ verfuegbar: isAvailable() });
	});
}
```

- [ ] **Step 3: Register routes in index.ts**

In `mako-ui/src/server/index.ts`, add near the top:

```typescript
import { registerRoutes as registerKreuzvalidator } from "./kreuzvalidator.ts";
```

And before the `app.listen()` call:

```typescript
registerKreuzvalidator(app);
```

- [ ] **Step 4: Test manually**

```bash
# Start the server
bun run server
# In another terminal, send a test EDIFACT message
curl -X POST http://localhost:3001/api/kreuzvalidiere \
  -H "Content-Type: application/json" \
  -d '{"edifact": "UNB+UNOC:3+..."}'
```

Check if it returns parsed fields or an error. Document the actual output structure.

- [ ] **Step 5: Check cross-validator status endpoint**

```bash
curl http://localhost:3001/api/kreuzvalidator-status
# Expected: {"verfuegbar": true} or {"verfuegbar": false}
```

- [ ] **Step 6: Commit**

```bash
git add mako-ui/
git commit -m "add STROMDAO cross-validator sidecar"
```

---

## Task 12: Backend Verification API Routes

**Files:**
- Modify: `mako-ui/src/server/index.ts`

- [ ] **Step 1: Add verification routes**

Add these routes to `index.ts`:

```typescript
app.get("/api/verifiziere/:rolle/:box/:datei", (req: Request, res: Response) => {
	const rolle = param(req, "rolle");
	const box_ = param(req, "box");
	const datei = param(req, "datei");
	const filePath = join(MARKT, rolle, box_, datei);
	if (!existsSync(filePath)) {
		res.status(404).json({ error: "not found" });
		return;
	}
	try {
		const output = cli([
			"verifiziere",
			filePath,
			"--referenzdaten",
			resolve("../mako-verify/referenzdaten"),
		]);
		const ergebnis = JSON.parse(output);
		res.json(ergebnis);
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});

app.post("/api/verifiziere-batch", (req: Request, res: Response) => {
	const { verzeichnis } = req.body;
	const dir = verzeichnis || resolve("../mako-sim/simulation/nachrichten");
	try {
		const output = cli([
			"verifiziere-batch",
			dir,
			"--referenzdaten",
			resolve("../mako-verify/referenzdaten"),
		]);
		const reportPath = join(dir, "verifikation.json");
		if (existsSync(reportPath)) {
			const report = JSON.parse(readFileSync(reportPath, "utf-8"));
			res.json(report);
		} else {
			res.json({ ok: true, ausgabe: output });
		}
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});
```

- [ ] **Step 2: Verify it compiles**

```bash
cd mako-ui && bun run server
# Should start without errors
```

- [ ] **Step 3: Commit**

```bash
git add mako-ui/src/server/index.ts
git commit -m "add verification API routes to Express backend"
```

---

## Task 13: React Types and API Client

**Files:**
- Modify: `mako-ui/src/lib/types.ts`
- Modify: `mako-ui/src/lib/api.ts`

- [ ] **Step 1: Add verification types**

In `mako-ui/src/lib/types.ts`:

```typescript
export type Urteil = "Bestanden" | "Fehlgeschlagen" | "NichtPruefbar";

export interface AhbFeldErgebnis {
	segment_code: string | null;
	segment_group: string | null;
	data_element: string | null;
	name: string;
	ahb_ausdruck: string;
	unser_wert: string | null;
	erwarteter_wert: string | null;
	urteil: Urteil;
	details: string | null;
}

export interface AhbErgebnis {
	pruefidentifikator: string;
	nachrichtentyp: string;
	felder: AhbFeldErgebnis[];
	urteil: Urteil;
	zusammenfassung: string;
}

export interface EbdAusgang {
	ebd_code: string;
	schritt: string;
	beschreibung: string;
	antwortcode: string | null;
	notiz: string | null;
}

export interface EbdErgebnis {
	ebd_code: string;
	ebd_name: string;
	rolle: string | null;
	unser_ergebnis: string;
	gueltige_ausgaenge: EbdAusgang[];
	urteil: Urteil;
	details: string | null;
}

export interface InteropFeldVergleich {
	feld: string;
	unser_wert: string;
	drittanbieter_wert: string;
	stimmt_ueberein: boolean;
}

export interface InteropErgebnis {
	parse_ok_unser: boolean;
	parse_ok_drittanbieter: boolean | null;
	roundtrip_ok: boolean;
	feldvergleiche: InteropFeldVergleich[];
	urteil: Urteil;
}

export interface VerifikationsErgebnis {
	datei: string;
	nachrichtentyp: string;
	pruefidentifikator: string | null;
	ahb: AhbErgebnis | null;
	ebd: EbdErgebnis | null;
	interop: InteropErgebnis | null;
	gesamt_urteil: Urteil;
}

export interface BatchErgebnis {
	gesamt: number;
	bestanden: number;
	fehlgeschlagen: number;
	nicht_pruefbar: number;
	ergebnisse: VerifikationsErgebnis[];
}
```

- [ ] **Step 2: Add API methods**

In `mako-ui/src/lib/api.ts`, add to the `api` object:

```typescript
verifiziere: (rolle: string, box: string, datei: string) =>
	get<VerifikationsErgebnis>(`/verifiziere/${rolle}/${box}/${datei}`),
verifiziereBatch: (verzeichnis?: string) =>
	post<BatchErgebnis>("/verifiziere-batch", { verzeichnis }),
kreuzvalidatorStatus: () =>
	get<{ verfuegbar: boolean }>("/kreuzvalidator-status"),
```

Add imports at the top:

```typescript
import type { ..., VerifikationsErgebnis, BatchErgebnis } from "./types.ts";
```

- [ ] **Step 3: Commit**

```bash
git add mako-ui/src/lib/
git commit -m "add verification types, API client methods"
```

---

## Task 14: VerifikationsBadge Component

**Files:**
- Create: `mako-ui/src/components/VerifikationsBadge.tsx`
- Modify: `mako-ui/src/components/MessageList.tsx`

- [ ] **Step 1: Create VerifikationsBadge**

```tsx
import { Badge } from "@/components/ui/badge";
import type { Urteil } from "@/lib/types";

interface VerifikationsBadgeProps {
	ahb: Urteil | null;
	ebd: Urteil | null;
	interop: Urteil | null;
}

function urteilZuSymbol(urteil: Urteil | null): string {
	if (urteil === null) return "○";
	switch (urteil) {
		case "Bestanden": return "✓";
		case "Fehlgeschlagen": return "✗";
		case "NichtPruefbar": return "○";
	}
}

function urteilZuFarbe(urteil: Urteil | null): string {
	if (urteil === null) return "text-muted-foreground";
	switch (urteil) {
		case "Bestanden": return "text-green-600 dark:text-green-400";
		case "Fehlgeschlagen": return "text-red-600 dark:text-red-400";
		case "NichtPruefbar": return "text-yellow-600 dark:text-yellow-400";
	}
}

export function VerifikationsBadge({ ahb, ebd, interop }: VerifikationsBadgeProps) {
	return (
		<span className="inline-flex gap-0.5 font-mono text-xs" title="AHB / EBD / Codec">
			<span className={urteilZuFarbe(ahb)}>{urteilZuSymbol(ahb)}</span>
			<span className={urteilZuFarbe(ebd)}>{urteilZuSymbol(ebd)}</span>
			<span className={urteilZuFarbe(interop)}>{urteilZuSymbol(interop)}</span>
		</span>
	);
}
```

- [ ] **Step 2: Integrate into MessageList**

In `MessageList.tsx`, import `VerifikationsBadge` and add it to each message card. The implementer should add state to track verification results per message (fetched lazily or on demand), and render the badge next to `StatusBadge`.

This requires wiring up a `useEffect` or on-click fetch of `/api/verifiziere/:rolle/:box/:datei` for each message. For V1, fetch on message selection rather than eagerly for all messages.

- [ ] **Step 3: Commit**

```bash
git add mako-ui/src/components/VerifikationsBadge.tsx mako-ui/src/components/MessageList.tsx
git commit -m "add VerifikationsBadge component, integrate in MessageList"
```

---

## Task 15: VerifikationsPanel Component

**Files:**
- Create: `mako-ui/src/components/VerifikationsPanel.tsx`
- Modify: `mako-ui/src/components/MessageDetail.tsx`

- [ ] **Step 1: Create VerifikationsPanel**

```tsx
import { useState } from "react";
import type { VerifikationsErgebnis, Urteil } from "@/lib/types";

interface VerifikationsPanelProps {
	ergebnis: VerifikationsErgebnis;
}

function urteilFarbe(urteil: Urteil): string {
	switch (urteil) {
		case "Bestanden": return "text-green-600 dark:text-green-400";
		case "Fehlgeschlagen": return "text-red-600 dark:text-red-400";
		case "NichtPruefbar": return "text-yellow-600 dark:text-yellow-400";
	}
}

export function VerifikationsPanel({ ergebnis }: VerifikationsPanelProps) {
	const [aktiveTab, setAktiveTab] = useState<"ahb" | "ebd" | "codec">("ahb");

	return (
		<div className="space-y-2">
			<div className="flex gap-2">
				{(["ahb", "ebd", "codec"] as const).map((tab) => (
					<button
						key={tab}
						type="button"
						onClick={() => setAktiveTab(tab)}
						className={`px-3 py-1 rounded text-sm ${aktiveTab === tab
							? "bg-primary text-primary-foreground"
							: "bg-muted text-muted-foreground"}`}
					>
						{tab.toUpperCase()}
					</button>
				))}
			</div>

			{aktiveTab === "ahb" && ergebnis.ahb && (
				<div className="space-y-1">
					<p className={`text-sm font-medium ${urteilFarbe(ergebnis.ahb.urteil)}`}>
						{ergebnis.ahb.zusammenfassung}
					</p>
					<table className="w-full text-xs">
						<thead>
							<tr className="border-b text-left">
								<th className="py-1">Segment</th>
								<th>Feld</th>
								<th>AHB</th>
								<th>Wert</th>
								<th>Ergebnis</th>
							</tr>
						</thead>
						<tbody>
							{ergebnis.ahb.felder.map((feld, i) => (
								<tr key={i} className="border-b border-muted">
									<td className="py-0.5 font-mono">{feld.segment_code}</td>
									<td>{feld.name}</td>
									<td className="font-mono">{feld.ahb_ausdruck}</td>
									<td className="font-mono">{feld.unser_wert ?? "—"}</td>
									<td className={urteilFarbe(feld.urteil)}>
										{feld.urteil === "Bestanden" ? "✓" : feld.urteil === "Fehlgeschlagen" ? "✗" : "○"}
										{feld.details && <span className="ml-1 text-muted-foreground">{feld.details}</span>}
									</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>
			)}

			{aktiveTab === "ebd" && ergebnis.ebd && (
				<div className="space-y-1">
					<p className="text-sm font-medium">{ergebnis.ebd.ebd_name}</p>
					<p className={`text-sm ${urteilFarbe(ergebnis.ebd.urteil)}`}>
						Unser Ergebnis: {ergebnis.ebd.unser_ergebnis}
					</p>
					{ergebnis.ebd.details && (
						<p className="text-xs text-muted-foreground">{ergebnis.ebd.details}</p>
					)}
				</div>
			)}

			{aktiveTab === "codec" && ergebnis.interop && (
				<div className="space-y-1">
					<table className="w-full text-xs">
						<thead>
							<tr className="border-b text-left">
								<th className="py-1">Feld</th>
								<th>Unser Parser</th>
								<th>STROMDAO</th>
								<th>Match</th>
							</tr>
						</thead>
						<tbody>
							{ergebnis.interop.feldvergleiche.map((v, i) => (
								<tr key={i} className="border-b border-muted">
									<td className="py-0.5">{v.feld}</td>
									<td className="font-mono">{v.unser_wert}</td>
									<td className="font-mono">{v.drittanbieter_wert}</td>
									<td>{v.stimmt_ueberein ? "✓" : "✗"}</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>
			)}

			{aktiveTab === "ahb" && !ergebnis.ahb && (
				<p className="text-sm text-muted-foreground">Keine AHB-Referenzdaten verfügbar</p>
			)}
			{aktiveTab === "ebd" && !ergebnis.ebd && (
				<p className="text-sm text-muted-foreground">EBD-Vergleich erfordert Prozesskontext</p>
			)}
			{aktiveTab === "codec" && !ergebnis.interop && (
				<p className="text-sm text-muted-foreground">Kreuzvalidator nicht aktiv</p>
			)}
		</div>
	);
}
```

- [ ] **Step 2: Integrate into MessageDetail**

In `MessageDetail.tsx`, add a "Verifizieren" button that fetches `/api/verifiziere/:rolle/:box/:datei` and displays the `VerifikationsPanel` with the result. Add state:

```tsx
const [verifikation, setVerifikation] = useState<VerifikationsErgebnis | null>(null);
const [verifiziereLaeuft, setVerifiziereLaeuft] = useState(false);
```

Add a button in the UI (after the existing "Verarbeiten" button):

```tsx
<Button
	variant="outline"
	size="sm"
	disabled={verifiziereLaeuft}
	onClick={async () => {
		setVerifiziereLaeuft(true);
		try {
			const erg = await api.verifiziere(rolle, box, datei);
			setVerifikation(erg);
		} finally {
			setVerifiziereLaeuft(false);
		}
	}}
>
	{verifiziereLaeuft ? "Verifiziere..." : "Verifizieren"}
</Button>
```

And render the panel:

```tsx
{verifikation && <VerifikationsPanel ergebnis={verifikation} />}
```

- [ ] **Step 3: Commit**

```bash
git add mako-ui/src/components/VerifikationsPanel.tsx mako-ui/src/components/MessageDetail.tsx
git commit -m "add VerifikationsPanel with AHB/EBD/Codec tabs, integrate in MessageDetail"
```

---

## Task 16: EBD Tree Visualization

**Files:**
- Create: `mako-ui/src/components/EbdBaum.tsx`

- [ ] **Step 1: Create EbdBaum component**

```tsx
import type { EbdAusgang } from "@/lib/types";

interface EbdBaumProps {
	ebd_name: string;
	rolle: string | null;
	ausgaenge: EbdAusgang[];
	unser_antwortcode: string | null;
}

export function EbdBaum({ ebd_name, rolle, ausgaenge, unser_antwortcode }: EbdBaumProps) {
	return (
		<div className="space-y-2">
			<p className="text-sm font-medium">{ebd_name} {rolle && `(${rolle})`}</p>
			<div className="space-y-1 text-xs font-mono">
				{ausgaenge.map((a, i) => {
					const istUnser = unser_antwortcode
						? a.antwortcode === unser_antwortcode
						: a.antwortcode === null;
					return (
						<div
							key={i}
							className={`flex items-start gap-2 p-1 rounded ${istUnser ? "bg-green-50 dark:bg-green-950 border border-green-300 dark:border-green-700" : ""}`}
						>
							<span className="shrink-0 w-8 text-right text-muted-foreground">
								{a.schritt}.
							</span>
							<div>
								<span>{a.beschreibung}</span>
								{a.antwortcode && (
									<span className="ml-2 text-red-600 dark:text-red-400">→ {a.antwortcode}</span>
								)}
								{!a.antwortcode && (
									<span className="ml-2 text-green-600 dark:text-green-400">→ Weiter</span>
								)}
								{a.notiz && (
									<p className="text-muted-foreground mt-0.5">{a.notiz}</p>
								)}
								{istUnser && (
									<span className="ml-2 text-green-600 dark:text-green-400 font-bold">← unser Ergebnis</span>
								)}
							</div>
						</div>
					);
				})}
			</div>
		</div>
	);
}
```

- [ ] **Step 2: Integrate into VerifikationsPanel EBD tab**

Replace the simple EBD section in `VerifikationsPanel.tsx` with:

```tsx
import { EbdBaum } from "./EbdBaum";

// In the ebd tab:
{aktiveTab === "ebd" && ergebnis.ebd && (
	<EbdBaum
		ebd_name={ergebnis.ebd.ebd_name}
		rolle={ergebnis.ebd.rolle}
		ausgaenge={ergebnis.ebd.gueltige_ausgaenge}
		unser_antwortcode={/* extract from ergebnis.ebd.unser_ergebnis */}
	/>
)}
```

- [ ] **Step 3: Commit**

```bash
git add mako-ui/src/components/EbdBaum.tsx mako-ui/src/components/VerifikationsPanel.tsx
git commit -m "add EBD decision tree visualization"
```

---

## Task 17: Batch Report Component + Button

**Files:**
- Create: `mako-ui/src/components/BatchBericht.tsx`
- Modify: `mako-ui/src/App.tsx`

- [ ] **Step 1: Create BatchBericht component**

```tsx
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import type { BatchErgebnis, VerifikationsErgebnis } from "@/lib/types";

interface BatchBerichtProps {
	ergebnis: BatchErgebnis;
	onClose: () => void;
}

export function BatchBericht({ ergebnis, onClose }: BatchBerichtProps) {
	const [filter, setFilter] = useState<"alle" | "fehler" | "bestanden">("alle");

	const gefiltert = ergebnis.ergebnisse.filter((e) => {
		if (filter === "fehler") return e.gesamt_urteil === "Fehlgeschlagen";
		if (filter === "bestanden") return e.gesamt_urteil === "Bestanden";
		return true;
	});

	return (
		<div className="fixed inset-0 bg-background/80 backdrop-blur-sm z-50 flex items-center justify-center">
			<div className="bg-background border rounded-lg shadow-lg w-[90vw] max-w-4xl max-h-[80vh] flex flex-col">
				<div className="p-4 border-b flex justify-between items-center">
					<div>
						<h2 className="text-lg font-semibold">Verifikationsbericht</h2>
						<p className="text-sm text-muted-foreground">
							{ergebnis.bestanden} bestanden, {ergebnis.fehlgeschlagen} fehlgeschlagen, {ergebnis.nicht_pruefbar} nicht prüfbar von {ergebnis.gesamt} Nachrichten
						</p>
					</div>
					<Button variant="ghost" size="sm" onClick={onClose}>Schließen</Button>
				</div>
				<div className="p-4 flex gap-2">
					{(["alle", "fehler", "bestanden"] as const).map((f) => (
						<Button
							key={f}
							variant={filter === f ? "default" : "outline"}
							size="sm"
							onClick={() => setFilter(f)}
						>
							{f === "alle" ? `Alle (${ergebnis.gesamt})` :
							 f === "fehler" ? `Fehler (${ergebnis.fehlgeschlagen})` :
							 `Bestanden (${ergebnis.bestanden})`}
						</Button>
					))}
				</div>
				<ScrollArea className="flex-1 p-4">
					<table className="w-full text-sm">
						<thead>
							<tr className="border-b text-left">
								<th className="py-2">Datei</th>
								<th>Typ</th>
								<th>PI</th>
								<th>AHB</th>
								<th>EBD</th>
								<th>Codec</th>
								<th>Gesamt</th>
							</tr>
						</thead>
						<tbody>
							{gefiltert.map((e, i) => (
								<tr key={i} className="border-b border-muted">
									<td className="py-1 font-mono text-xs max-w-[200px] truncate">{e.datei}</td>
									<td>{e.nachrichtentyp}</td>
									<td>{e.pruefidentifikator ?? "—"}</td>
									<td>{urteilSymbol(e.ahb?.urteil)}</td>
									<td>{urteilSymbol(e.ebd?.urteil)}</td>
									<td>{urteilSymbol(e.interop?.urteil)}</td>
									<td>{urteilSymbol(e.gesamt_urteil)}</td>
								</tr>
							))}
						</tbody>
					</table>
				</ScrollArea>
			</div>
		</div>
	);
}

function urteilSymbol(urteil?: string | null): string {
	if (!urteil) return "○";
	switch (urteil) {
		case "Bestanden": return "✓";
		case "Fehlgeschlagen": return "✗";
		case "NichtPruefbar": return "○";
		default: return "?";
	}
}
```

- [ ] **Step 2: Add batch verification button to App.tsx**

Add state and a button in the header area:

```tsx
const [batchErgebnis, setBatchErgebnis] = useState<BatchErgebnis | null>(null);
const [batchLaeuft, setBatchLaeuft] = useState(false);

// In the header:
<Button
	variant="outline"
	size="sm"
	disabled={batchLaeuft}
	onClick={async () => {
		setBatchLaeuft(true);
		try {
			const erg = await api.verifiziereBatch();
			setBatchErgebnis(erg);
		} finally {
			setBatchLaeuft(false);
		}
	}}
>
	{batchLaeuft ? "Verifiziere..." : "Simulation verifizieren"}
</Button>

// At the bottom of the component:
{batchErgebnis && (
	<BatchBericht ergebnis={batchErgebnis} onClose={() => setBatchErgebnis(null)} />
)}
```

- [ ] **Step 3: Commit**

```bash
git add mako-ui/src/components/BatchBericht.tsx mako-ui/src/App.tsx
git commit -m "add BatchBericht modal, 'Simulation verifizieren' button"
```

---

## Task 18: Integration Test — Full Pipeline

**Files:**
- Test: `mako-verify/tests/ahb_tests.rs` (extend)

- [ ] **Step 1: Write end-to-end test**

```rust
#[test]
fn vollstaendige_verifikation_einer_nachricht() {
	let edi = mako_testdata::fixtures::utilmd::anmeldung_lfw_edi();
	let mut refdata = Referenzdaten::laden("referenzdaten");
	let ergebnis = mako_verify::verifiziere_nachricht(&edi, &mut refdata);
	// Should produce a result (even if NichtPruefbar due to PI mismatch)
	println!("Ergebnis: {}", serde_json::to_string_pretty(&ergebnis).unwrap());
	// At minimum, the parse should succeed
	assert_ne!(ergebnis.nachrichtentyp, "", "Nachrichtentyp sollte erkannt werden");
}
```

- [ ] **Step 2: Run test**

```bash
cargo test -p mako-verify --test ahb_tests vollstaendige
```

Expected: PASS

- [ ] **Step 3: Run all tests in the workspace to ensure nothing is broken**

```bash
cargo test --workspace
```

Expected: all 580+ tests PASS

- [ ] **Step 4: Commit**

```bash
git add mako-verify/tests/
git commit -m "add end-to-end verification test"
```

---

## Task 19: Final Smoke Test

- [ ] **Step 1: Build everything**

```bash
cargo build --workspace
```

- [ ] **Step 2: Run the simulation**

```bash
cargo run -p mako-sim --bin mako-simulate
```

- [ ] **Step 3: Run batch verification on simulation output**

```bash
cargo run -p mako-cli -- verifiziere-batch mako-sim/simulation/nachrichten --referenzdaten mako-verify/referenzdaten
```

Check output: how many pass/fail/not-verifiable?

- [ ] **Step 4: Start the UI and test manually**

```bash
cd mako-ui
bun run server &
bunx vite --host &
```

Open in browser, select a role, view a message, click "Verifizieren", inspect the result.

- [ ] **Step 5: Document results**

Write findings (pass rates, common failures, adjustment needs) as a comment in the commit or as notes for the next iteration.

- [ ] **Step 6: Final commit**

```bash
git add mako-verify/ mako-cli/ mako-ui/
git commit -m "complete verification system V1: AHB validation, EBD comparison, cross-validator, UI integration"
```

---

## Corrections & Additions (from plan review)

The following corrections address issues found during plan review. Implementers should apply these alongside the corresponding tasks above.

### Correction A: `Urteil` must derive `Eq` (applies to Task 4)

In `bericht.rs`, change the derive on `Urteil`:

```rust
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Urteil {
```

### Correction B: `Referenzdaten::laden` takes format version parameter (applies to Task 3)

Change the constructor to accept a format version:

```rust
pub fn laden(basis: impl AsRef<Path>, format_version: &str) -> Self {
	Self {
		basis: basis.as_ref().to_path_buf(),
		format_version: format_version.to_string(),
		ahb_cache: HashMap::new(),
		ebd_cache: HashMap::new(),
	}
}
```

And use `self.format_version` instead of hardcoded `"FV2504"` in path construction. All call sites become `Referenzdaten::laden("referenzdaten", "FV2504")`.

### Correction C: Add `UB` operator to condition parser (applies to Task 6)

In `parse_bedingung`, add before the single-reference check:

```rust
// UB = sofern nicht (unless)
if let Some(pos) = find_operator(input, &[" UB "]) {
	let (left, right) = split_at_operator(input, pos, &[" UB "]);
	let l = parse_bedingung(left)?;
	let r = parse_bedingung(right)?;
	// "A UB B" means "A unless B" = A AND NOT B
	return Some(Bedingung::Und(Box::new(l), Box::new(Bedingung::Nicht(Box::new(r)))));
}
```

### Correction D: AHB validation uses segment-presence approach for V1 (applies to Task 5)

Replace the `finde_wert` placeholder with a segment-presence approach. For V1, we validate:
- **Segment presence**: Is the required segment (e.g., NAD+MS) present?
- **Value pool at qualifier level**: Does the segment qualifier match? (e.g., NAD+**MS** checks the first element)
- **Exact value comparison only when `value_pool_entry` maps to a qualifier** (e.g., BGM first element = "E01")

Do NOT attempt full data-element-to-position mapping in V1. Mark field-level value comparisons as `NichtPruefbar` with detail "Feldwert-Prüfung erfordert Data-Element-Mapping (V2)".

Replace `finde_wert` with:

```rust
fn segment_vorhanden(edifact: &EdifactNachricht, segment_code: &str, qualifier: Option<&str>) -> bool {
	edifact.segmente.iter().any(|s| {
		if s.tag != segment_code {
			return false;
		}
		match qualifier {
			Some(q) => s.elements.first()
				.and_then(|e| e.components.first())
				.map(|c| c == q)
				.unwrap_or(false),
			None => true,
		}
	})
}

fn qualifier_aus_segment_code(segment_code_mit_qualifier: &str) -> (&str, Option<&str>) {
	// "NAD+MS" → ("NAD", Some("MS"))
	// "DTM+137" → ("DTM", Some("137"))
	// "UNH" → ("UNH", None)
	if let Some((tag, qual)) = segment_code_mit_qualifier.split_once('+') {
		(tag, Some(qual))
	} else {
		(segment_code_mit_qualifier, None)
	}
}
```

Update `pruefe_feld` to use `segment_vorhanden` instead of direct segment iteration. The `"Muss"` check becomes:

```rust
"Muss" | "M" => {
	let (tag, qualifier) = qualifier_aus_segment_code(segment_code);
	if !segment_vorhanden(edifact, tag, qualifier) {
		// ... Fehlgeschlagen with "Pflichtfeld fehlt"
	}
	// Value pool check: only if value_pool_entry is set and represents a qualifier
	if let Some(expected) = expected_value {
		// Check if this is a simple qualifier match (e.g., "UTILMD", "E01", "137")
		if expected.len() <= 10 && expected.chars().all(|c| c.is_alphanumeric()) {
			// This is likely a code value — check segment qualifier
			// For V1, mark as Bestanden if segment is present (qualifier already checked above)
		}
	}
	// ... Bestanden
}
```

Adjust test expectations in Task 5: the known-good test should now focus on segment presence (which will reliably pass), not on value extraction (which requires V2 mapping).

---

## Task 9b: Implement `verifiziere_prozess_schritt` (NEW — Critical)

**Files:**
- Modify: `mako-verify/src/lib.rs`
- Test: `mako-verify/tests/ebd_tests.rs`

This task adds the missing public API function that connects Layer 2 (EBD) to the verification flow.

- [ ] **Step 1: Write failing test — GPKE LFW step through EBD**

In `mako-verify/tests/ebd_tests.rs`:

```rust
use mako_verify::bericht::Urteil;
use mako_verify::referenzdaten::Referenzdaten;
use mako_types::nachricht::Nachricht;

#[test]
fn gpke_lfw_happy_path_ist_gueltiger_ebd_ausgang() {
	// Build a minimal scenario: NB receives Anmeldung, produces Bestätigung
	let eingabe_edi = mako_testdata::fixtures::utilmd::anmeldung_lfw_edi();
	let eingabe = mako_codec::edifact::dispatch::parse_nachricht(&eingabe_edi).unwrap();

	// Simulate the reducer output: Bestätigung + Abmeldung an LFA
	// Use existing fixtures or construct manually
	let ausgabe_edi = mako_testdata::fixtures::utilmd::bestaetigung_lfw_edi();
	let ausgabe = mako_codec::edifact::dispatch::parse_nachricht(&ausgabe_edi).unwrap();

	let mut refdata = Referenzdaten::laden("referenzdaten", "FV2504");

	// E_0401 is the EBD for "NB prüft Anmeldung" — adapt code if different EBD
	let ergebnis = mako_verify::verifiziere_prozess_schritt(
		&eingabe,
		&[ausgabe],
		"E_0401", // or whichever EBD applies
		&mut refdata,
	);

	assert!(
		ergebnis.ebd.is_some(),
		"EBD-Ergebnis sollte vorhanden sein"
	);
	let ebd = ergebnis.ebd.unwrap();
	assert_eq!(ebd.urteil, Urteil::Bestanden,
		"Bestätigung sollte gültiger Ausgang sein: {:?}", ebd.details);
}
```

Note: The implementer must check which EBD code corresponds to the LFW Anmeldung step and adapt the test. Also verify that `bestaetigung_lfw_edi()` exists as a fixture — if not, construct a minimal one or use a different fixture pair.

- [ ] **Step 2: Run test — verify it fails**

```bash
cargo test -p mako-verify --test ebd_tests gpke_lfw
```

Expected: FAIL (function not defined)

- [ ] **Step 3: Implement `verifiziere_prozess_schritt` in `lib.rs`**

Add to `mako-verify/src/lib.rs`:

```rust
pub fn verifiziere_prozess_schritt(
	eingabe: &mako_types::nachricht::Nachricht,
	ausgabe: &[mako_types::nachricht::Nachricht],
	ebd_code: &str,
	refdata: &mut Referenzdaten,
) -> VerifikationsErgebnis {
	// Layer 1: Validate each output message against AHB
	let erste_ausgabe = ausgabe.first();
	let nachrichtentyp = erste_ausgabe
		.map(|n| format!("{:?}", n.payload).split('(').next().unwrap_or("").to_string())
		.unwrap_or_default();
	let pruefidentifikator = erste_ausgabe
		.and_then(|n| n.pruef_id.as_ref())
		.map(|pi| pi.code().to_string());

	// Layer 2: EBD outcome comparison
	// Determine our answer code from the output messages
	let unser_antwortcode = bestimme_antwortcode(ausgabe);
	let unser_beschreibung = beschreibe_ausgabe(ausgabe);

	let ebd_ergebnis = ebd::vergleiche_ergebnis(
		ebd_code,
		unser_antwortcode.as_deref(),
		&unser_beschreibung,
		refdata,
	);

	let gesamt = ebd_ergebnis.urteil.clone();

	VerifikationsErgebnis {
		datei: String::new(),
		nachrichtentyp,
		pruefidentifikator,
		ahb: None, // AHB checked per-message via verifiziere_nachricht
		ebd: Some(ebd_ergebnis),
		interop: None,
		gesamt_urteil: gesamt,
	}
}

fn bestimme_antwortcode(ausgabe: &[mako_types::nachricht::Nachricht]) -> Option<String> {
	// Check if any output message is a rejection (Ablehnung)
	for nachricht in ausgabe {
		match &nachricht.payload {
			mako_types::nachricht::NachrichtenPayload::UtilmdAblehnung(_) => {
				// Extract rejection code if available
				// The implementer should check what fields UtilmdAblehnung carries
				return Some("A01".to_string()); // placeholder — extract real code
			}
			_ => {}
		}
	}
	None // No rejection = acceptance path
}

fn beschreibe_ausgabe(ausgabe: &[mako_types::nachricht::Nachricht]) -> String {
	if ausgabe.is_empty() {
		return "Keine Ausgabe".to_string();
	}
	ausgabe.iter()
		.map(|n| format!("{:?}", n.payload).split('(').next().unwrap_or("?").to_string())
		.collect::<Vec<_>>()
		.join(" + ")
}
```

- [ ] **Step 4: Run test — verify it passes**

```bash
cargo test -p mako-verify --test ebd_tests gpke_lfw
```

Expected: PASS (adapt EBD code if needed)

- [ ] **Step 5: Commit**

```bash
git add mako-verify/src/lib.rs mako-verify/tests/ebd_tests.rs
git commit -m "implement verifiziere_prozess_schritt, wire EBD layer"
```

---

## Task 12b: Add Step-Verification API Route (NEW — Critical)

**Files:**
- Modify: `mako-ui/src/server/index.ts`

This task adds the missing API route that calls `verifiziere_prozess_schritt` after a process step.

- [ ] **Step 1: Add `/api/verifiziere-schritt` route**

In `mako-ui/src/server/index.ts`, add:

```typescript
app.post("/api/verifiziere-schritt", (req: Request, res: Response) => {
	const { rolle, datei, ebd_code } = req.body;
	if (!rolle || !datei) {
		res.status(400).json({ ok: false, error: "rolle und datei erforderlich" });
		return;
	}
	const filePath = join(MARKT, rolle, "inbox", datei);
	try {
		// First process the message
		const verarbeitungsAusgabe = cli(["verarbeite", filePath, "--markt", MARKT]);
		// Then verify the step
		// The CLI verifiziere command handles single-message AHB.
		// For EBD, we need to call verifiziere with process context.
		// V1: run verifiziere on the processed message and return combined result
		const output = cli([
			"verifiziere",
			filePath,
			"--referenzdaten",
			resolve("../mako-verify/referenzdaten"),
		]);
		const ergebnis = JSON.parse(output);
		res.json({ ...ergebnis, verarbeitung: verarbeitungsAusgabe });
	} catch (e) {
		res.status(500).json({ ok: false, error: String(e) });
	}
});
```

- [ ] **Step 2: Add API method in `api.ts`**

```typescript
verifizereSchritt: (rolle: string, datei: string, ebd_code?: string) =>
	post<VerifikationsErgebnis & { verarbeitung?: string }>("/verifiziere-schritt", { rolle, datei, ebd_code }),
```

- [ ] **Step 3: Wire into "Verarbeiten" button in MessageDetail**

Update the "Verarbeiten" button's `onClick` to also trigger step verification and display results:

```tsx
onClick={async () => {
	// Existing verarbeite call
	await api.verarbeite(rolle, datei);
	// Also verify
	const erg = await api.verifiziere(rolle, box, datei);
	setVerifikation(erg);
	onVerarbeitet?.();
}}
```

- [ ] **Step 4: Commit**

```bash
git add mako-ui/src/server/index.ts mako-ui/src/lib/api.ts mako-ui/src/components/MessageDetail.tsx
git commit -m "add step verification route, auto-verify on Verarbeiten"
```

---

## Task 18b: Batch Verification Test (NEW — Major)

**Files:**
- Test: `mako-verify/tests/ahb_tests.rs`

- [ ] **Step 1: Write batch verification test**

```rust
use std::fs;
use std::path::Path;

#[test]
fn batch_verifikation_ueber_verzeichnis() {
	// Create a temp directory with a few .edi files
	let temp = std::env::temp_dir().join("mako_verify_batch_test");
	let _ = fs::remove_dir_all(&temp);
	fs::create_dir_all(temp.join("sub")).unwrap();

	// Write two fixture files
	let edi1 = mako_testdata::fixtures::utilmd::anmeldung_lfw_edi();
	fs::write(temp.join("001.edi"), &edi1).unwrap();
	fs::write(temp.join("sub/002.edi"), &edi1).unwrap();

	let mut refdata = mako_verify::referenzdaten::Referenzdaten::laden(
		"referenzdaten", "FV2504"
	);
	let ergebnis = mako_verify::verifiziere_batch(&temp, &mut refdata);

	assert_eq!(ergebnis.gesamt, 2);
	assert!(ergebnis.bestanden + ergebnis.fehlgeschlagen + ergebnis.nicht_pruefbar == 2);

	// Cleanup
	let _ = fs::remove_dir_all(&temp);
}
```

- [ ] **Step 2: Run test**

```bash
cargo test -p mako-verify --test ahb_tests batch
```

Expected: PASS

- [ ] **Step 3: Add recursive directory walking**

If the test reveals that nested directories aren't traversed properly, add a recursive helper in `lib.rs`:

```rust
fn sammle_edi_dateien(verzeichnis: &Path) -> Vec<std::path::PathBuf> {
	let mut dateien = Vec::new();
	if let Ok(entries) = std::fs::read_dir(verzeichnis) {
		for entry in entries.flatten() {
			let pfad = entry.path();
			if pfad.is_dir() {
				dateien.extend(sammle_edi_dateien(&pfad));
			} else if pfad.extension().and_then(|e| e.to_str()) == Some("edi") {
				dateien.push(pfad);
			}
		}
	}
	dateien
}
```

- [ ] **Step 4: Commit**

```bash
git add mako-verify/
git commit -m "add batch verification test, recursive directory traversal"
```

---

## Task 18c: BatchErgebnis Unit Test (NEW — Major)

- [ ] **Step 1: Add unit test for zusammenfassung**

In `mako-verify/src/bericht.rs`, add at the bottom:

```rust
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn zusammenfassung_format() {
		let batch = BatchErgebnis {
			gesamt: 10,
			bestanden: 7,
			fehlgeschlagen: 2,
			nicht_pruefbar: 1,
			ergebnisse: vec![],
		};
		assert_eq!(
			batch.zusammenfassung(),
			"7 von 10 bestanden, 2 fehlgeschlagen, 1 nicht prüfbar"
		);
	}
}
```

- [ ] **Step 2: Run test**

```bash
cargo test -p mako-verify bericht
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add mako-verify/src/bericht.rs
git commit -m "add BatchErgebnis zusammenfassung test"
```

---

## V1 Scope Notes

- **"Soll" expressions** are treated identically to "Kann" (optional, always Bestanden) for V1. This is a pragmatic simplification — "Soll" implies strong recommendation but not a hard requirement.
- **Field-level value comparison** in AHB validation is limited to segment-presence and qualifier checks for V1. Full data-element-to-position mapping (comparing extracted values like dates, IDs) requires a mapping table and is deferred to V2.
- **`bestimme_antwortcode`** in `verifiziere_prozess_schritt` extracts rejection codes from output messages. The implementer must check which `NachrichtenPayload` variants carry rejection codes and extract them properly. The placeholder in Task 9b returns a hardcoded "A01" — this must be replaced with real extraction logic.
