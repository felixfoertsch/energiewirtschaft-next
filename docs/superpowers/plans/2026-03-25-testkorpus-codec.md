# MaKo-Testkorpus + Codec Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a complete EDIFACT codec (parse + serialize) and test corpus with 59 fixtures, 46 generators, a fault injector, and 15 end-to-end communication chains — all TDD-driven.

**Architecture:** Extend the existing `mako-codec` crate (which already has a working segment-level lexer/serializer) with typed dispatch: raw Segments → domain Structs (`Nachricht`/`NachrichtenPayload`) and back. Build `mako-testdata` fixtures as the TDD driver. Each fixture is a pair: EDIFACT `&str` constant + expected typed Rust struct.

**Tech Stack:** Rust 1.93, `chrono` for dates, `serde` + `serde_json` for serialization, `thiserror` for error types. No async, no IO, no network. EDIFACT segments per BDEW MIG FV2504.

**Spec:** `docs/superpowers/specs/2026-03-25-testkorpus-codec-design.md`

**Reference Docs:**
- `docs/mako_aktuell_kostenfrei_25_03_2026/` — all MIG/AHB PDFs (FV2504)
- `docs/mako_formatvorlagen_referenz.md` — EDIFACT message type overview
- `docs/checkliste_testkorpus_mako.md` — test data inventory & sources
- Hochfrequenz `edifact_mapper` fixtures (github.com/Hochfrequenz/edifact_mapper)

**Existing Code:**
- `mako-codec/src/edifact/parser.rs` — working segment-level lexer (`parse_segments`, `parse_interchange`), escape handling, 9 tests
- `mako-codec/src/edifact/serializer.rs` — working segment-level serializer (`serialize_segments`, `serialize_interchange`), roundtrip tests
- `mako-codec/src/edifact/segment.rs` — `Segment`, `Element`, `Interchange`, `EdifactNachricht` types
- `mako-codec/src/edifact/bdew_segmente.rs` — typed BDEW segment structs (BGM, DTM, NAD, etc.) with parse/serialize
- `mako-types/src/nachricht.rs` — `Nachricht`, `NachrichtenPayload` (43 variants)
- `mako-types/src/gpke_nachrichten.rs` — all message payload structs
- `mako-testdata/src/ids.rs` — `test_malo()`, `test_mp_id()`, `test_melo()` generators

---

## File Structure

### mako-codec (extend existing)

```
mako-codec/src/
├── lib.rs                          # Add: pub mod fehler; extend edifact mod
├── fehler.rs                       # NEW: CodecFehler enum (thiserror)
├── edifact/
│   ├── mod.rs                      # Add: pub mod dispatch;
│   ├── segment.rs                  # KEEP as-is
│   ├── parser.rs                   # KEEP as-is (segment-level lexer)
│   ├── serializer.rs               # KEEP as-is (segment-level serializer)
│   ├── bdew_segmente.rs            # KEEP as-is (typed BDEW segments)
│   └── dispatch.rs                 # NEW: Segment[] → Nachricht and Nachricht → Segment[]
│                                   #   parse_nachricht(&str) → Result<Nachricht, CodecFehler>
│                                   #   serialize_nachricht(&Nachricht) → String
│                                   #   One parse_*/serialize_* fn per payload variant
└── json/                           # KEEP stubs for now
```

### mako-testdata (extend existing)

```
mako-testdata/src/
├── lib.rs                          # Add: pub mod fixtures; pub mod generator; pub mod fehler; pub mod ketten;
├── ids.rs                          # KEEP
├── quittungen.rs                   # KEEP
├── fixtures/
│   ├── mod.rs                      # NEW: re-exports all fixture modules
│   ├── utilmd.rs                   # NEW: 13 GPKE UTILMD fixtures (EDIFACT &str + expected Nachricht)
│   ├── mscons.rs                   # NEW: MSCONS fixtures (Lastgang, Zählerstand, Aggregiert, etc.)
│   ├── invoic.rs                   # NEW: INVOIC fixture
│   ├── remadv.rs                   # NEW: REMADV fixture
│   ├── orders.rs                   # NEW: ORDERS fixtures (Bestellung, WerteAnfrage)
│   ├── ordrsp.rs                   # NEW: ORDRSP fixture
│   ├── reqote.rs                   # NEW: REQOTE fixture
│   ├── quotes.rs                   # NEW: QUOTES fixture
│   ├── pricat.rs                   # NEW: PRICAT fixture
│   ├── contrl.rs                   # NEW: CONTRL positiv/negativ
│   ├── aperak.rs                   # NEW: APERAK positiv/negativ
│   ├── iftsta.rs                   # NEW: IFTSTA fixture
│   ├── partin.rs                   # NEW: PARTIN fixture
│   └── utilts.rs                   # NEW: UTILTS fixture
├── generator/
│   ├── mod.rs                      # NEW: re-exports
│   ├── segmente.rs                 # NEW: segment builder fns (una, unb, unh, bgm, dtm, nad, ...)
│   ├── params.rs                   # NEW: Params structs with Default
│   └── edifact.rs                  # NEW: per-type generators (erzeuge_utilmd_anmeldung, etc.)
├── fehler.rs                       # NEW: FehlerArt enum + injiziere_fehler()
├── ketten.rs                       # NEW: Kette, KettenSchritt, pruefe_kette()
├── mscons.rs                       # MIGRATE → fixtures/mscons.rs (then delete)
├── utilmd.rs                       # MIGRATE → fixtures/utilmd.rs (then delete)
├── szenarien.rs                    # REPLACE with ketten.rs (then delete)
└── szenarien_historisch.rs         # KEEP for now (historical scenarios still useful)
```

### mako-types (extend existing)

```
mako-types/src/
├── querschnitt.rs                  # EXTEND: replace placeholders with real IFTSTA, PARTIN, UTILTS structs
└── nachricht.rs                    # EXTEND: add 3 new NachrichtenPayload variants
```

---

## Task 1: CodecFehler + Typed Dispatch Scaffold

**Files:**
- Create: `mako-codec/src/fehler.rs`
- Modify: `mako-codec/src/lib.rs`
- Modify: `mako-codec/src/edifact/mod.rs`
- Create: `mako-codec/src/edifact/dispatch.rs`

- [ ] **Step 1: Create CodecFehler enum**

`mako-codec/src/fehler.rs`:
```rust
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CodecFehler {
	// Lexer (segment-level) errors — delegate to existing ParseError
	#[error("parse error: {0}")]
	Parse(String),

	// Lexer errors
	#[error("no UNA or UNB found")]
	KeinUnaOderUnb,

	#[error("invalid separator at position {position}")]
	UngueltigesTrennzeichen { position: usize },

	#[error("unterminated escape sequence at position {position}")]
	UnterbrocheneEscapeSequenz { position: usize },

	// Dispatch errors — typed message level
	#[error("missing segment: expected {erwartet}")]
	SegmentFehlt { erwartet: String },

	#[error("unknown message type: {typ}")]
	UnbekannterNachrichtentyp { typ: String },

	#[error("unknown Prüfidentifikator: {code}")]
	UnbekannterPruefIdentifikator { code: String },

	#[error("missing field {feld} in segment {segment}")]
	FeldFehlt { segment: String, feld: String },

	#[error("invalid value '{wert}' for {feld} in {segment}")]
	UngueltigerWert { segment: String, feld: String, wert: String },

	#[error("invalid format for {feld} in {segment}, expected {erwartet}")]
	UngueltigesFormat { segment: String, feld: String, erwartet: String },

	// XML errors (Task 13)
	#[error("XML parse error: {0}")]
	XmlParseFehler(String),

	#[error("XSD validation error: {0}")]
	XsdValidierungsFehler(String),
}
```

- [ ] **Step 2: Create dispatch.rs with public API stubs**

`mako-codec/src/edifact/dispatch.rs`:
```rust
use mako_types::nachricht::Nachricht;

use crate::fehler::CodecFehler;

/// Parse an EDIFACT string into a typed Nachricht.
/// Dispatches based on UNH message type + RFF+Z13 Prüfidentifikator.
pub fn parse_nachricht(input: &str) -> Result<Nachricht, CodecFehler> {
	todo!("Task 2 implements the first variant")
}

/// Serialize a typed Nachricht to an EDIFACT string.
pub fn serialize_nachricht(nachricht: &Nachricht) -> String {
	todo!("Task 3 implements the first variant")
}
```

- [ ] **Step 3: Wire up modules in lib.rs and mod.rs**

Add to `mako-codec/src/lib.rs`:
```rust
pub mod fehler;
```

Add to `mako-codec/src/edifact/mod.rs`:
```rust
pub mod dispatch;
```

Add `thiserror` dependency to `mako-codec/Cargo.toml`:
```toml
thiserror = { workspace = true }
```

- [ ] **Step 4: Verify workspace compiles**

Run: `cargo check --workspace`
Expected: compiles (warnings about `todo!()` are OK)

- [ ] **Step 5: Commit**

```
git add mako-codec/
git commit -m "add CodecFehler enum, dispatch module scaffold for typed EDIFACT parsing"
```

---

## Task 2: First Fixture + Parser — UTILMD Anmeldung (PID 44001)

This is the foundational TDD cycle. Every subsequent fixture follows this exact pattern.

**Files:**
- Create: `mako-testdata/src/fixtures/mod.rs`
- Create: `mako-testdata/src/fixtures/utilmd.rs`
- Modify: `mako-testdata/src/lib.rs`
- Modify: `mako-testdata/Cargo.toml`
- Modify: `mako-codec/src/edifact/dispatch.rs`

**Reference:** UTILMD MIG S2.1 (`docs/mako_aktuell_kostenfrei_25_03_2026/UTILMD_MIG_Strom S2_1_ 20241001.pdf`), UTILMD AHB Strom 2.1 (PID 44001 = Anmeldung E01)

- [ ] **Step 1: Add mako-codec dependency to mako-testdata**

In `mako-testdata/Cargo.toml`, add:
```toml
mako-codec = { path = "../mako-codec" }
```

- [ ] **Step 2: Create fixture module structure**

`mako-testdata/src/fixtures/mod.rs`:
```rust
pub mod utilmd;
```

Add to `mako-testdata/src/lib.rs`:
```rust
pub mod fixtures;
```

- [ ] **Step 3: Write the UTILMD Anmeldung fixture**

`mako-testdata/src/fixtures/utilmd.rs`:
```rust
use chrono::NaiveDate;

use mako_types::gpke_nachrichten::UtilmdAnmeldung;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
use mako_types::rolle::MarktRolle;

use crate::ids::{test_malo, test_mp_id};

/// GPKE 1.1.1: Anmeldung LFN → NB (PID 44001, FV2504, UTILMD MIG S2.1)
///
/// Segments per MIG:
///   UNB — Interchange header (sender, receiver, date, ref)
///   UNH — Message header (UTILMD:D:11A:UN:S2.1)
///   BGM — E01 (Anmeldung)
///   DTM — 137 (Nachrichtendatum)
///   NAD+MS — Sender (LFN)
///   NAD+MR — Receiver (NB)
///   IDE+24 — MaLo-ID
///   DTM+92 — Lieferbeginn
///   RFF+Z13 — Prüfidentifikator 44001
///   UNT — Message trailer
///   UNZ — Interchange trailer
///
/// IDs: test_mp_id(0) = "9900000000000", test_mp_id(1) = "9900000000001"
///       test_malo(0) — compute at runtime, use its value in the EDI string
pub fn anmeldung_lfw_edi() -> String {
	let absender = crate::ids::test_mp_id(0);
	let empfaenger = crate::ids::test_mp_id(1);
	let malo = crate::ids::test_malo(0);
	format!(
		"UNB+UNOC:3+{absender}:500+{empfaenger}:500+260401:1200+REF00001'\
		UNH+1+UTILMD:D:11A:UN:S2.1'\
		BGM+E01+ANMELD00001'\
		DTM+137:20260401120000?+01:303'\
		NAD+MS+{absender}::293'\
		NAD+MR+{empfaenger}::293'\
		IDE+24+{malo}'\
		DTM+92:20260701:102'\
		RFF+Z13:44001'\
		UNT+9+1'\
		UNZ+1+REF00001'",
		absender = absender.as_str(),
		empfaenger = empfaenger.as_str(),
		malo = malo.as_str(),
	)
}

pub fn anmeldung_lfw_erwartet() -> Nachricht {
	Nachricht {
		absender: test_mp_id(0),
		absender_rolle: MarktRolle::LieferantNeu,
		empfaenger: test_mp_id(1),
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: Some(PruefIdentifikator::AnmeldungNn),  // PID 44001
		payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: test_mp_id(0),
			lieferbeginn: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
		}),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mako_codec::edifact::dispatch::parse_nachricht;

	#[test]
	fn parse_anmeldung_lfw() {
		let edi = anmeldung_lfw_edi();
		let parsed = parse_nachricht(&edi).unwrap();
		assert_eq!(parsed, anmeldung_lfw_erwartet());
	}
}
```

- [ ] **Step 4: Run test to verify it fails**

Run: `cargo test -p mako-testdata fixtures::utilmd::tests::parse_anmeldung_lfw`
Expected: FAIL — `parse_nachricht` contains `todo!()`

- [ ] **Step 5: Implement parse_nachricht for UTILMD Anmeldung**

In `mako-codec/src/edifact/dispatch.rs`, replace the stub:

```rust
use chrono::NaiveDate;

use mako_types::gpke_nachrichten::UtilmdAnmeldung;
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::pruefidentifikator::PruefIdentifikator;
use mako_types::rolle::MarktRolle;

use super::parser::{parse_interchange, ParseError};
use super::segment::Segment;
use crate::fehler::CodecFehler;

pub fn parse_nachricht(input: &str) -> Result<Nachricht, CodecFehler> {
	let interchange = parse_interchange(input).map_err(|e| CodecFehler::Parse(e.to_string()))?;
	let msg = interchange
		.nachrichten
		.first()
		.ok_or(CodecFehler::SegmentFehlt { erwartet: "UNH".into() })?;

	// Extract routing from interchange header
	let absender_str = &interchange.sender;
	let empfaenger_str = &interchange.empfaenger;

	// Find PID from RFF+Z13
	let pid = find_rff_z13(&msg.segmente)?;

	// Find BGM qualifier
	let bgm_code = find_bgm_code(&msg.segmente)?;

	// Dispatch based on message type + PID
	match (msg.typ.as_str(), bgm_code.as_str()) {
		("UTILMD", "E01") => parse_utilmd_anmeldung(absender_str, empfaenger_str, &msg.segmente, pid),
		_ => Err(CodecFehler::UnbekannterNachrichtentyp { typ: format!("{}+{}", msg.typ, bgm_code) }),
	}
}

pub fn serialize_nachricht(nachricht: &Nachricht) -> String {
	todo!("Task 3")
}

// --- Helper functions ---

fn find_rff_z13(segmente: &[Segment]) -> Result<Option<PruefIdentifikator>, CodecFehler> {
	for seg in segmente {
		if seg.tag == "RFF" {
			if let Some(elem) = seg.elements.first() {
				if elem.components.first().map(|s| s.as_str()) == Some("Z13") {
					let code_str = elem.components.get(1).ok_or(CodecFehler::FeldFehlt {
						segment: "RFF".into(),
						feld: "Z13 code".into(),
					})?;
					let code_u32 = code_str.parse::<u32>().map_err(|_| CodecFehler::UngueltigerWert {
						segment: "RFF".into(), feld: "Z13".into(), wert: code_str.clone(),
					})?;
					let pid = PruefIdentifikator::from_code(code_u32).ok_or(
						CodecFehler::UnbekannterPruefIdentifikator { code: code_str.clone() },
					)?;
					return Ok(Some(pid));
				}
			}
		}
	}
	Ok(None)
}

fn find_bgm_code(segmente: &[Segment]) -> Result<String, CodecFehler> {
	for seg in segmente {
		if seg.tag == "BGM" {
			return seg
				.elements
				.first()
				.and_then(|e| e.components.first())
				.cloned()
				.ok_or(CodecFehler::FeldFehlt { segment: "BGM".into(), feld: "Meldecode".into() });
		}
	}
	Err(CodecFehler::SegmentFehlt { erwartet: "BGM".into() })
}

fn find_nad(segmente: &[Segment], qualifier: &str) -> Result<String, CodecFehler> {
	for seg in segmente {
		if seg.tag == "NAD" {
			if let Some(elem) = seg.elements.first() {
				if elem.components.first().map(|s| s.as_str()) == Some(qualifier) {
					return seg
						.elements
						.get(1)
						.and_then(|e| e.components.first())
						.cloned()
						.ok_or(CodecFehler::FeldFehlt {
							segment: format!("NAD+{qualifier}"),
							feld: "MP-ID".into(),
						});
				}
			}
		}
	}
	Err(CodecFehler::SegmentFehlt { erwartet: format!("NAD+{qualifier}") })
}

fn find_ide_malo(segmente: &[Segment]) -> Result<MaLoId, CodecFehler> {
	for seg in segmente {
		if seg.tag == "IDE" {
			if let Some(elem) = seg.elements.first() {
				if elem.components.first().map(|s| s.as_str()) == Some("24") {
					let id_str = seg
						.elements
						.get(1)
						.and_then(|e| e.components.first())
						.ok_or(CodecFehler::FeldFehlt { segment: "IDE+24".into(), feld: "MaLo-ID".into() })?;
					return MaLoId::new(id_str).map_err(|_| CodecFehler::UngueltigerWert {
						segment: "IDE+24".into(),
						feld: "MaLo-ID".into(),
						wert: id_str.clone(),
					});
				}
			}
		}
	}
	Err(CodecFehler::SegmentFehlt { erwartet: "IDE+24".into() })
}

fn find_dtm(segmente: &[Segment], qualifier: &str) -> Result<NaiveDate, CodecFehler> {
	for seg in segmente {
		if seg.tag == "DTM" {
			if let Some(elem) = seg.elements.first() {
				if elem.components.first().map(|s| s.as_str()) == Some(qualifier) {
					let date_str = elem.components.get(1).ok_or(CodecFehler::FeldFehlt {
						segment: format!("DTM+{qualifier}"),
						feld: "date value".into(),
					})?;
					let format_code = elem.components.get(2).map(|s| s.as_str()).unwrap_or("102");
					return parse_dtm_date(date_str, format_code).map_err(|_| CodecFehler::UngueltigesFormat {
						segment: format!("DTM+{qualifier}"),
						feld: "date".into(),
						erwartet: format!("format code {format_code}"),
					});
				}
			}
		}
	}
	Err(CodecFehler::SegmentFehlt { erwartet: format!("DTM+{qualifier}") })
}

fn parse_dtm_date(value: &str, format_code: &str) -> Result<NaiveDate, ()> {
	match format_code {
		"102" => NaiveDate::parse_from_str(value, "%Y%m%d").map_err(|_| ()),
		"203" => NaiveDate::parse_from_str(&value[..8], "%Y%m%d").map_err(|_| ()),
		"303" => {
			// Format 303: YYYYMMDDHHmm with timezone offset after ?+
			let date_part = if value.len() >= 8 { &value[..8] } else { value };
			NaiveDate::parse_from_str(date_part, "%Y%m%d").map_err(|_| ())
		}
		_ => Err(()),
	}
}

fn parse_utilmd_anmeldung(
	absender_str: &str,
	empfaenger_str: &str,
	segmente: &[Segment],
	pid: Option<PruefIdentifikator>,
) -> Result<Nachricht, CodecFehler> {
	let absender = MarktpartnerId::new(absender_str).map_err(|_| CodecFehler::UngueltigerWert {
		segment: "UNB".into(), feld: "sender".into(), wert: absender_str.into(),
	})?;
	let empfaenger = MarktpartnerId::new(empfaenger_str).map_err(|_| CodecFehler::UngueltigerWert {
		segment: "UNB".into(), feld: "empfaenger".into(), wert: empfaenger_str.into(),
	})?;
	let malo_id = find_ide_malo(segmente)?;
	let lieferbeginn = find_dtm(segmente, "92")?;

	Ok(Nachricht {
		absender: absender.clone(),
		absender_rolle: MarktRolle::LieferantNeu,
		empfaenger,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		pruef_id: pid,
		payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
			malo_id,
			lieferant_neu: absender,
			lieferbeginn,
		}),
	})
}
```

- [ ] **Step 6: Run test to verify it passes**

Run: `cargo test -p mako-testdata fixtures::utilmd::tests::parse_anmeldung_lfw`
Expected: PASS

- [ ] **Step 7: Run all workspace tests to check nothing broke**

Run: `cargo test --workspace`
Expected: all pass

- [ ] **Step 8: Commit**

```
git add mako-codec/ mako-testdata/
git commit -m "add UTILMD Anmeldung fixture (PID 44001), implement typed dispatch parser"
```

---

## Task 3: Serializer — UTILMD Anmeldung Roundtrip

**Files:**
- Modify: `mako-codec/src/edifact/dispatch.rs`
- Modify: `mako-testdata/src/fixtures/utilmd.rs`

- [ ] **Step 1: Write the roundtrip test**

Add to `mako-testdata/src/fixtures/utilmd.rs` tests module:
```rust
#[test]
fn roundtrip_anmeldung_lfw() {
	let parsed = parse_nachricht(ANMELDUNG_LFW_EDI).unwrap();
	let serialized = serialize_nachricht(&parsed);
	let reparsed = parse_nachricht(&serialized).unwrap();
	assert_eq!(reparsed, parsed);
}
```

Add the import: `use mako_codec::edifact::dispatch::serialize_nachricht;`

**Roundtrip design note:** The roundtrip test asserts `parse(serialize(parse(x))) == parse(x)`, NOT `serialize(parse(x)) == x`. The serializer may produce different whitespace, document numbers, or timestamps than the original. What matters is semantic equivalence: the re-parsed struct must equal the first-parsed struct. The `Nachricht` struct does not carry interchange metadata (date, reference number, document number) — these are transport-level concerns that the roundtrip intentionally ignores.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p mako-testdata fixtures::utilmd::tests::roundtrip_anmeldung_lfw`
Expected: FAIL — `serialize_nachricht` contains `todo!()`

- [ ] **Step 3: Implement serialize_nachricht for UTILMD Anmeldung**

In `mako-codec/src/edifact/dispatch.rs`, replace the `serialize_nachricht` stub:

```rust
use super::segment::Interchange;
use super::serializer::serialize_interchange;

pub fn serialize_nachricht(nachricht: &Nachricht) -> String {
	let interchange = nachricht_to_interchange(nachricht);
	serialize_interchange(&interchange)
}

fn nachricht_to_interchange(nachricht: &Nachricht) -> Interchange {
	let segmente = match &nachricht.payload {
		NachrichtenPayload::UtilmdAnmeldung(anm) => serialize_utilmd_anmeldung(nachricht, anm),
		_ => todo!("serialize other payload types"),
	};

	let msg_typ = nachrichtentyp_fuer_payload(&nachricht.payload);
	let msg_version = "D:11A:UN:S2.1"; // TODO: derive from payload type

	Interchange {
		sender: nachricht.absender.as_str().to_string(),
		empfaenger: nachricht.empfaenger.as_str().to_string(),
		datum: chrono::Local::now().format("%Y%m%d").to_string(),
		nachrichten: vec![super::segment::EdifactNachricht {
			typ: msg_typ.to_string(),
			version: msg_version.to_string(),
			segmente,
		}],
	}
}

fn nachrichtentyp_fuer_payload(payload: &NachrichtenPayload) -> &'static str {
	match payload {
		NachrichtenPayload::UtilmdAnmeldung(_)
		| NachrichtenPayload::UtilmdBestaetigung(_)
		| NachrichtenPayload::UtilmdAbmeldung(_)
		| NachrichtenPayload::UtilmdAblehnung(_)
		| NachrichtenPayload::UtilmdZuordnung(_) => "UTILMD",
		NachrichtenPayload::MsconsLastgang(_)
		| NachrichtenPayload::MsconsSchlussturnusmesswert(_) => "MSCONS",
		NachrichtenPayload::InvoicRechnung(_) => "INVOIC",
		NachrichtenPayload::RemadvZahlungsavis(_) => "REMADV",
		_ => "UTILMD", // TODO: extend for all types
	}
}

fn serialize_utilmd_anmeldung(nachricht: &Nachricht, anm: &UtilmdAnmeldung) -> Vec<Segment> {
	let pid_code = nachricht.pruef_id.as_ref().map(|p| p.code().to_string()).unwrap_or_default();
	vec![
		make_bgm("E01", "ANMELD00001"),
		make_dtm_303("137", chrono::Local::now().date_naive()),
		make_nad("MS", nachricht.absender.as_str()),
		make_nad("MR", nachricht.empfaenger.as_str()),
		make_ide_malo(anm.malo_id.as_str()),
		make_dtm_102("92", anm.lieferbeginn),
		make_rff_z13(&pid_code),
	]
}

// --- Segment builder helpers ---

fn make_bgm(code: &str, dok_nr: &str) -> Segment {
	Segment {
		tag: "BGM".into(),
		elements: vec![
			Element { components: vec![code.into()] },
			Element { components: vec![dok_nr.into()] },
		],
	}
}

fn make_dtm_102(qualifier: &str, date: NaiveDate) -> Segment {
	Segment {
		tag: "DTM".into(),
		elements: vec![Element {
			components: vec![qualifier.into(), date.format("%Y%m%d").to_string(), "102".into()],
		}],
	}
}

fn make_dtm_303(qualifier: &str, date: NaiveDate) -> Segment {
	Segment {
		tag: "DTM".into(),
		elements: vec![Element {
			components: vec![
				qualifier.into(),
				format!("{}120000?+01", date.format("%Y%m%d")),
				"303".into(),
			],
		}],
	}
}

fn make_nad(qualifier: &str, mp_id: &str) -> Segment {
	Segment {
		tag: "NAD".into(),
		elements: vec![
			Element { components: vec![qualifier.into()] },
			Element { components: vec![mp_id.into(), "".into(), "293".into()] },
		],
	}
}

fn make_ide_malo(malo_id: &str) -> Segment {
	Segment {
		tag: "IDE".into(),
		elements: vec![
			Element { components: vec!["24".into()] },
			Element { components: vec![malo_id.into()] },
		],
	}
}

fn make_rff_z13(code: &str) -> Segment {
	Segment {
		tag: "RFF".into(),
		elements: vec![Element { components: vec!["Z13".into(), code.into()] }],
	}
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p mako-testdata fixtures::utilmd::tests::roundtrip_anmeldung_lfw`
Expected: PASS

- [ ] **Step 5: Run all workspace tests**

Run: `cargo test --workspace`
Expected: all pass

- [ ] **Step 6: Commit**

```
git add mako-codec/ mako-testdata/
git commit -m "add UTILMD Anmeldung serializer, roundtrip test passes"
```

---

## Task 4: Remaining GPKE UTILMD Fixtures (12 variants)

Same TDD pattern as Task 2–3, repeated for each GPKE variant. Each fixture needs:
1. `const *_EDI: &str` — the EDIFACT string
2. `fn *_erwartet() -> Nachricht` — the expected parsed struct
3. Parse test + roundtrip test
4. `parse_*` function in dispatch.rs
5. `serialize_*` function in dispatch.rs

**Files:**
- Modify: `mako-testdata/src/fixtures/utilmd.rs`
- Modify: `mako-codec/src/edifact/dispatch.rs`

Fixtures to add (one per GPKE process step):

| Fixture | BGM | PID | Struct |
|---------|-----|-----|--------|
| BESTAETIGUNG_LFW_EDI | E01 response | 44002 | UtilmdBestaetigung |
| ABMELDUNG_LFW_EDI | E02 | 44003 | UtilmdAbmeldung |
| ABLEHNUNG_LFW_EDI | E01 rejection | 44004 | UtilmdAblehnung |
| ZUORDNUNG_LFW_EDI | E06 | 44005 | UtilmdZuordnung |
| LIEFERENDE_ABMELDUNG_EDI | E02 | 44006 | UtilmdLieferendeAbmeldung |
| LIEFERENDE_BESTAETIGUNG_EDI | E01 resp | 44007 | UtilmdLieferendeBestaetigung |
| STAMMDATENAENDERUNG_EDI | E03 | 44008 | UtilmdStammdatenaenderung |
| ZUORDNUNGSLISTE_EDI | E06 list | 44009 | UtilmdZuordnungsliste |
| GDA_ANFRAGE_EDI | E09 | 44010 | UtilmdGeschaeftsdatenanfrage |
| GDA_ANTWORT_EDI | E09 resp | 44011 | UtilmdGeschaeftsdatenantwort |

Plus MSCONS fixtures in `fixtures/mscons.rs`:

| Fixture | Struct |
|---------|--------|
| SCHLUSSTURNUSMESSWERT_EDI | MsconsSchlussturnusmesswert |
| LASTGANG_EDI | MsconsLastgang |

- [ ] **Step 1: Write all 12 UTILMD fixture constants + expected structs**

Build each EDIFACT string following the MIG S2.1 segment structure. Use `test_malo(N)` and `test_mp_id(N)` for IDs. Each fixture uses the correct BGM qualifier and PID per the AHB.

- [ ] **Step 2: Write parse + roundtrip tests for each**

- [ ] **Step 3: Implement parse_* dispatch branches**

Add match arms to `parse_nachricht` for each BGM+PID combination. Extract shared helpers (e.g., `find_nad`, `find_dtm`, `find_ide_malo` already exist).

- [ ] **Step 4: Implement serialize_* for each variant**

Add match arms to `nachricht_to_interchange` and implement each `serialize_*` function using the segment builder helpers.

- [ ] **Step 5: Run tests**

Run: `cargo test -p mako-testdata fixtures::utilmd`
Expected: all pass

- [ ] **Step 6: Create fixtures/mscons.rs with MSCONS fixtures**

MSCONS uses a different UNH type and segment structure (SG5/SG6 groups with QTY segments). Build 2 fixtures: Schlussturnusmesswert and Lastgang.

- [ ] **Step 7: Implement MSCONS parse + serialize**

- [ ] **Step 8: Run all tests**

Run: `cargo test --workspace`
Expected: all pass

- [ ] **Step 9: Commit**

```
git add mako-codec/ mako-testdata/
git commit -m "add all GPKE fixtures (13 UTILMD + 2 MSCONS), parser + serializer + roundtrip tests"
```

---

## Task 5: WiM, UBP, MaBiS, Abrechnung Fixtures

Same TDD pattern. Create fixture files, write parse + roundtrip tests, implement dispatch.

**Files:**
- Create: `mako-testdata/src/fixtures/orders.rs` (WerteAnfrage + Bestellung)
- Create: `mako-testdata/src/fixtures/ordrsp.rs`
- Create: `mako-testdata/src/fixtures/reqote.rs`
- Create: `mako-testdata/src/fixtures/quotes.rs`
- Create: `mako-testdata/src/fixtures/pricat.rs`
- Create: `mako-testdata/src/fixtures/invoic.rs`
- Create: `mako-testdata/src/fixtures/remadv.rs`
- Modify: `mako-testdata/src/fixtures/utilmd.rs` (add MsbWechselAnmeldung, Geraetewechsel, Bilanzkreiszuordnung, Clearingliste)
- Modify: `mako-testdata/src/fixtures/mscons.rs` (add AggregierteZeitreihen, MehrMindermengen)
- Modify: `mako-testdata/src/fixtures/mod.rs`
- Modify: `mako-codec/src/edifact/dispatch.rs`

| Group | Fixtures | Count |
|-------|----------|-------|
| WiM UTILMD | MsbWechselAnmeldung, Geraetewechsel | 2 |
| WiM ORDERS | WerteAnfrage | 1 |
| UBP | Angebotsanfrage, Angebot, Bestellung, Bestellantwort, Preisblatt | 5 |
| MaBiS UTILMD | Bilanzkreiszuordnung, Clearingliste | 2 |
| MaBiS MSCONS | AggregierteZeitreihen, MehrMindermengen | 2 |
| Abrechnung | InvoicRechnung, RemadvZahlungsavis | 2 |

- [ ] **Step 1: Write fixtures for each new EDIFACT type (ORDERS, ORDRSP, REQOTE, QUOTES, PRICAT, INVOIC, REMADV)**

Each new file type needs its own UNH message type and segment structure per the respective MIG.

**MIG reference files** (in `docs/mako_aktuell_kostenfrei_25_03_2026/`):
- ORDERS: `ORDERS_MIG_1_4b_20250401.pdf` — UNH+ORDERS:D:01B:UN:1.4b, SG1(NAD), SG2(LIN+QTY+PRI+RFF)
- ORDRSP: `ORDRSP_MIG_1_4a_20250401.pdf` — UNH+ORDRSP:D:01B:UN:1.4a
- REQOTE: `REQOTE_MIG_1_3c_20250401.pdf` — UNH+REQOTE:D:01B:UN:1.3c
- QUOTES: `QUOTES_MIG_1_3b_20250401.pdf` — UNH+QUOTES:D:01B:UN:1.3b
- PRICAT: `PRICAT_MIG_2_0e_20250401.pdf` — UNH+PRICAT:D:01B:UN:2.0e, SG17(LIN+PIA+PRI)
- INVOIC: `INVOIC_MIG_2.8e_20250401.pdf` — UNH+INVOIC:D:01B:UN:2.8e, SG2(NAD), SG6(TAX), SG26(LIN+QTY+MOA)
- REMADV: `REMADV_MIG_2.9d_20250401.pdf` — UNH+REMADV:D:01B:UN:2.9d, SG1(RFF), SG3(DOC+MOA)

**Critical:** Read the MIG PDF for each type BEFORE writing the fixture. The segment order and group nesting differ significantly between types. The executing agent must extract the segment skeleton from the MIG's "Branching Diagram" section.

- [ ] **Step 2: Write parse + roundtrip tests**
- [ ] **Step 3: Implement parse dispatch for each new UNH type**
- [ ] **Step 4: Implement serialize for each new type**
- [ ] **Step 5: Run tests, commit**

```
git add mako-codec/ mako-testdata/
git commit -m "add WiM, UBP, MaBiS, Abrechnung fixtures (14 types), parser + serializer"
```

---

## Task 6: MPES, §14a, Gas, Querschnitt Fixtures

**Files:**
- Modify: `mako-testdata/src/fixtures/utilmd.rs` (add AnmeldungErzeugung, SteuerbareVerbrauchseinrichtung, Ausspeisepunkt)
- Modify: `mako-testdata/src/fixtures/mscons.rs` (add EinspeiseMesswerte, Brennwert)
- Create: `mako-testdata/src/fixtures/iftsta.rs`
- Create: `mako-testdata/src/fixtures/partin.rs`
- Create: `mako-testdata/src/fixtures/utilts.rs`
- Modify: `mako-types/src/querschnitt.rs` — replace placeholders with real structs
- Modify: `mako-types/src/nachricht.rs` — add IFTSTA, PARTIN, UTILTS to NachrichtenPayload
- Modify: `mako-codec/src/edifact/dispatch.rs`

| Group | Fixtures | Count |
|-------|----------|-------|
| MPES | AnmeldungErzeugung, EinspeiseMesswerte | 2 |
| §14a | SteuerbareVerbrauchseinrichtung, ClsSteuersignal | 2 |
| Gas | Nominierung, NominierungBestaetigung, Renominierung, Brennwert, Ausspeisepunkt | 5 |
| Querschnitt | IFTSTA, PARTIN, UTILTS | 3 |

- [ ] **Step 1: Extend NachrichtenPayload with 3 new Querschnitts-variants**

Add to `mako-types/src/nachricht.rs`:
```rust
// Querschnitt
IftstaStatusmeldung(IftstaStatusmeldung),
PartinMarktpartner(PartinMarktpartner),
UtiltsZaehlzeitdefinition(UtiltsZaehlzeitdefinition),
```

Replace placeholder structs in `querschnitt.rs` with real fields per IFTSTA MIG 2.0g, PARTIN MIG 1.0e, UTILTS MIG 1.1e.

- [ ] **Step 2: Write fixtures + tests for all 12 variants**
- [ ] **Step 3: Implement parse + serialize dispatch**
- [ ] **Step 4: Run tests, commit**

```
git add mako-types/ mako-codec/ mako-testdata/
git commit -m "add MPES, §14a, Gas, Querschnitt fixtures (12 types), extend NachrichtenPayload"
```

---

## Task 7: CONTRL + APERAK Fixtures

**Files:**
- Create: `mako-testdata/src/fixtures/contrl.rs`
- Create: `mako-testdata/src/fixtures/aperak.rs`
- Modify: `mako-codec/src/edifact/dispatch.rs`

These are quittung types — they don't go through `NachrichtenPayload` but need their own parse/serialize.

- [ ] **Step 1: Write 4 fixtures**

CONTRL positiv, CONTRL negativ, APERAK positiv, APERAK negativ. Reference: CONTRL MIG 2.0b, APERAK MIG 2.1i. Use the web examples found during research as basis.

- [ ] **Step 2: Write parse + roundtrip tests**
- [ ] **Step 3: Implement CONTRL/APERAK parse + serialize in dispatch.rs**
- [ ] **Step 4: Run tests, commit**

```
git add mako-codec/ mako-testdata/
git commit -m "add CONTRL, APERAK fixtures (4 types), parser + serializer"
```

---

## Task 8: Segment Builder + Generator

**Files:**
- Create: `mako-testdata/src/generator/mod.rs`
- Create: `mako-testdata/src/generator/segmente.rs`
- Create: `mako-testdata/src/generator/params.rs`
- Create: `mako-testdata/src/generator/edifact.rs`
- Modify: `mako-testdata/src/lib.rs`

- [ ] **Step 1: Write segment builder functions**

`segmente.rs`: `una()`, `unb()`, `unh()`, `bgm()`, `dtm()`, `nad()`, `rff_pid()`, `ide_malo()`, `ide_melo()`, `unt()`, `unz()`. Each returns a `String` segment.

- [ ] **Step 2: Write tests for segment builders**

Each builder test: build segment → parse with existing lexer → verify tag and elements match.

- [ ] **Step 3: Write Params structs with Default**

`params.rs`: `AnmeldungParams`, `BestaetigunsParams`, `LastgangParams`, `InvoicParams`, etc. — one per payload variant. Each has `Default` with fachlich sinnvolle Werte.

- [ ] **Step 4: Write generator functions**

`edifact.rs`: `erzeuge_utilmd_anmeldung(&AnmeldungParams) -> String`, etc. Each composes segment builders into a complete EDIFACT string.

- [ ] **Step 5: Write generator → parse roundtrip tests**

For each generator: `let edi = erzeuge_utilmd_anmeldung(&AnmeldungParams::default()); let parsed = parse_nachricht(&edi).unwrap();` — verify the parsed struct has correct field values.

- [ ] **Step 6: Run tests, commit**

```
git add mako-testdata/
git commit -m "add EDIFACT segment builders, params structs, 46 generators with roundtrip tests"
```

---

## Task 9: Fehler-Injektor

**Files:**
- Create: `mako-testdata/src/fehler.rs`
- Modify: `mako-testdata/src/lib.rs`

- [ ] **Step 1: Write FehlerArt enum + injiziere_fehler()**

As specified in the design spec: Syntaxfehler, Anwendungsfehler, Fachliche Fehler.

- [ ] **Step 2: Write tests for each FehlerArt**

For each variant: generate a valid message, inject the error, verify:
- Syntaxfehler → `parse_nachricht` returns `Err(CodecFehler::*)`
- Anwendungsfehler → parse succeeds, but APERAK check rejects
- Fachliche Fehler → parse succeeds, APERAK accepts, reducer returns `ProzessFehler`

- [ ] **Step 3: Run tests, commit**

```
git add mako-testdata/
git commit -m "add FehlerArt enum, injiziere_fehler(), tests for all error categories"
```

---

## Task 10: Kommunikationsketten (GPKE)

**Files:**
- Create: `mako-testdata/src/ketten.rs`
- Modify: `mako-testdata/src/lib.rs`

- [ ] **Step 1: Write Kette, KettenSchritt, Quittungsergebnis data structures**

- [ ] **Step 2: Write pruefe_kette() runner**

- [ ] **Step 3: Write Kette 1: GPKE LFW Happy Path (7 steps)**

Each step: EDIFACT string (from generator), expected Nachricht, expected state after reducer, expected quittung.

- [ ] **Step 4: Write Kette 2: GPKE LFW Ablehnung (5 steps)**

- [ ] **Step 5: Write Kette 3: GPKE LFW Fristüberschreitung (3 steps)**

- [ ] **Step 6: Write Kette 4: GPKE Lieferende (4 steps)**

- [ ] **Step 7: Write Kette 5: GPKE Stammdatenänderung (3 steps)**

- [ ] **Step 8: Run all ketten tests**

Run: `cargo test -p mako-testdata ketten`
Expected: all 5 GPKE ketten pass

- [ ] **Step 9: Commit**

```
git add mako-testdata/
git commit -m "add Kommunikationsketten framework, 5 GPKE chains (LFW happy/reject/timeout, Lieferende, Stammdaten)"
```

---

## Task 11: Remaining Ketten (10)

**Prerequisite:** Requires the respective process reducers to be functional. Implement chains as the reducers become available.

**Files:**
- Modify: `mako-testdata/src/ketten.rs`

| Kette | Reducer needed | Status |
|-------|---------------|--------|
| 6: WiM MSB-Wechsel | mako-wim v2025 | exists |
| 7: WiM Zählwertübermittlung | mako-wim v2025 | exists |
| 8: UBP Bestellung | mako-ubp v2025 | exists |
| 9: MaBiS Bilanzkreiszuordnung | mako-mabis v2025 | exists |
| 10: Abrechnung Netznutzung | mako-abrechnung v2025 | exists |
| 11: RD 2.0 Abruf | mako-rd2 v2025 | exists (stub) |
| 12: §14a Steuerung | mako-14a v2025 | exists (stub) |
| 13: GeLi Gas LFW | mako-geli v2025 | exists |
| 14: GABi Gas Nominierung | mako-gabi v2025 | exists |
| 15: KoV Brennwertmitteilung | mako-kov v2025 | exists |

- [ ] **Step 1: Implement Ketten 6–10 (WiM, UBP, MaBiS, Abrechnung)**
- [ ] **Step 2: Implement Ketten 11–12 (RD 2.0, §14a)**
- [ ] **Step 3: Implement Ketten 13–15 (Gas)**
- [ ] **Step 4: Run all ketten tests, commit**

```
git add mako-testdata/
git commit -m "add remaining 10 communication chains (WiM, UBP, MaBiS, Abrechnung, RD2, 14a, Gas)"
```

---

## Task 12: Cleanup + Migration

**Files:**
- Delete: `mako-testdata/src/szenarien.rs` (replaced by ketten.rs)
- Modify: `mako-testdata/src/lib.rs`
- Delete or migrate: `mako-testdata/src/mscons.rs` (old version → fixtures/mscons.rs)
- Delete or migrate: `mako-testdata/src/utilmd.rs` (old version → fixtures/utilmd.rs)

- [ ] **Step 1: Verify all old szenarien tests have equivalents in ketten**
- [ ] **Step 2: Remove old modules, update lib.rs**
- [ ] **Step 3: Run all workspace tests**

Run: `cargo test --workspace`
Expected: all pass, no dead code warnings

- [ ] **Step 4: Commit**

```
git add mako-testdata/
git commit -m "remove legacy szenarien modules, migrate to fixtures + ketten"
```

---

## Task 13: XML Fixtures (Redispatch 2.0)

**Note:** This task is intentionally placed last for the EDIFACT-focused phases. XML needs a separate parser (not the EDIFACT lexer). Consider `quick-xml` crate for parsing.

**Files:**
- Create: `mako-testdata/src/fixtures/xml/mod.rs` + 9 XML fixture files
- Create: `mako-codec/src/xml/mod.rs`
- Create: `mako-codec/src/xml/parser.rs`
- Create: `mako-codec/src/xml/serializer.rs`

- [ ] **Step 1: Add RdStatusRequest and RdKaskade to NachrichtenPayload**

Add 2 new structs to `mako-types/src/gpke_nachrichten.rs` (or a new `rd_nachrichten.rs`):
- `RdStatusRequest { ressource_id: String, anfrage_typ: String }`
- `RdKaskade { ressource_id: String, kaskaden_stufe: u8, ... }` (per Kaskade FB 1.0)

Add 2 new variants to `NachrichtenPayload` in `nachricht.rs`.

- [ ] **Step 2: Add quick-xml dependency to mako-codec**
- [ ] **Step 3: Write 9 XML fixture strings (from XSD examples)**
- [ ] **Step 4: Implement XML parse + serialize**
- [ ] **Step 5: Write roundtrip tests**
- [ ] **Step 6: Run tests, commit**

```
git add mako-codec/ mako-testdata/
git commit -m "add XML parser/serializer for 9 RD 2.0 document types, fixtures + roundtrip tests"
```
