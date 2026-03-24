# MaKo-Engine Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the foundational Rust workspace with type system, deadline calculation, receipt layer, version infrastructure, and the first working reducer (GPKE Lieferantenwechsel v2025) — fully test-driven.

**Architecture:** Cargo workspace with 5 crates: `mako-types` (algebraic types, IDs, traits), `mako-fristen` (deadline calculation, holiday calendar), `mako-quittung` (CONTRL/APERAK decorator), `mako-gpke` (first reducer), `mako-testdata` (test data generator). All process logic is pure functions. No IO.

**Tech Stack:** Rust 1.93, `chrono` for dates, `serde` + `serde_json` for serialization, `thiserror` for error types. No async, no IO, no network.

**Spec:** `docs/superpowers/specs/2026-03-24-mako-engine-design.md`

**Reference Docs:**
- `docs/checkliste_mako_kommunikationslinien.md` — GPKE LFW process steps (Section 1.1)
- `docs/mako_formatvorlagen_referenz.md` — EDIFACT message types
- `docs/checkliste_testkorpus_mako.md` — test data inventory

---

## File Structure

```
mako/
├── Cargo.toml                          # Workspace root
├── mako-types/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                      # Re-exports all modules
│       ├── sparte.rs                   # Sparte enum (Strom, Gas)
│       ├── rolle.rs                    # MarktRolle enum (all market roles)
│       ├── ids.rs                      # MaLoId, MeLoId, MarktpartnerId newtypes
│       ├── nachricht.rs               # Nachricht enum, typed message structs
│       ├── gpke_nachrichten.rs         # GPKE-specific UTILMD message structs
│       ├── querschnitt.rs             # IFTSTA, PARTIN, UTILTS placeholder structs
│       ├── fehler.rs                   # ProzessFehler, ValidationError
│       ├── reducer.rs                  # Reducer trait, ReducerOutput
│       └── version.rs                 # MakoVersion enum, VersionDispatcher trait
├── mako-fristen/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── feiertage.rs               # Feiertagskalender (data, not code)
│       ├── frist.rs                   # frist() pure function
│       └── zeitmodell.rs              # Gastag/Stromtag offset logic
├── mako-quittung/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── contrl.rs                  # CONTRL syntax check (pure)
│       ├── aperak.rs                  # APERAK application check (EBD, pure)
│       ├── decorator.rs               # mit_quittung() decorator
│       └── types.rs                   # Quittung, QuittungsTyp, QuittungsErgebnis, DekorierterOutput
├── mako-gpke/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── v2025/
│           ├── mod.rs
│           ├── lfw.rs                 # LfwState, LfwEvent, reduce()
│           └── lfw_tests.rs           # All LFW tests (happy path, invalid, edge cases)
├── mako-testdata/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── ids.rs                     # Test ID generators
│       ├── utilmd.rs                  # UTILMD message generators
│       ├── quittungen.rs             # CONTRL/APERAK pair generators
│       └── szenarien.rs              # Full scenario fixtures (GPKE LFW)
```

---

## Task 1: Workspace Setup

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `mako-types/Cargo.toml`
- Create: `mako-types/src/lib.rs`
- Create: `mako-fristen/Cargo.toml`
- Create: `mako-fristen/src/lib.rs`
- Create: `mako-quittung/Cargo.toml`
- Create: `mako-quittung/src/lib.rs`
- Create: `mako-gpke/Cargo.toml`
- Create: `mako-gpke/src/lib.rs`
- Create: `mako-testdata/Cargo.toml`
- Create: `mako-testdata/src/lib.rs`

- [ ] **Step 1: Create workspace Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
	"mako-types",
	"mako-fristen",
	"mako-quittung",
	"mako-gpke",
	"mako-testdata",
]

[workspace.package]
edition = "2024"
license = "MIT OR Apache-2.0"
repository = "https://github.com/felixfoertsch/energiewirtschaft-next"

[workspace.dependencies]
chrono = { version = "0.4", default-features = false, features = ["serde"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
```

- [ ] **Step 2: Create mako-types crate**

`mako-types/Cargo.toml`:
```toml
[package]
name = "mako-types"
version = "0.1.0"
edition.workspace = true

[dependencies]
chrono = { workspace = true }
serde = { workspace = true }
thiserror = { workspace = true }
```

`mako-types/src/lib.rs`:
```rust
pub mod sparte;
pub mod rolle;
pub mod ids;
pub mod fehler;
pub mod gpke_nachrichten;
pub mod nachricht;
pub mod querschnitt;
pub mod reducer;
pub mod version;
```

- [ ] **Step 3: Create mako-fristen crate**

`mako-fristen/Cargo.toml`:
```toml
[package]
name = "mako-fristen"
version = "0.1.0"
edition.workspace = true

[dependencies]
chrono = { workspace = true }
mako-types = { path = "../mako-types" }
```

`mako-fristen/src/lib.rs`:
```rust
pub mod feiertage;
pub mod frist;
pub mod zeitmodell;
```

- [ ] **Step 4: Create mako-quittung crate**

`mako-quittung/Cargo.toml`:
```toml
[package]
name = "mako-quittung"
version = "0.1.0"
edition.workspace = true

[dependencies]
mako-types = { path = "../mako-types" }
mako-fristen = { path = "../mako-fristen" }
```

`mako-quittung/src/lib.rs`:
```rust
pub mod types;
pub mod contrl;
pub mod aperak;
pub mod decorator;
```

- [ ] **Step 5: Create mako-gpke crate**

`mako-gpke/Cargo.toml`:
```toml
[package]
name = "mako-gpke"
version = "0.1.0"
edition.workspace = true

[dependencies]
mako-types = { path = "../mako-types" }
mako-fristen = { path = "../mako-fristen" }

[dev-dependencies]
mako-testdata = { path = "../mako-testdata" }
mako-quittung = { path = "../mako-quittung" }
```

`mako-gpke/src/lib.rs`:
```rust
pub mod v2025;
```

- [ ] **Step 6: Create mako-testdata crate**

`mako-testdata/Cargo.toml`:
```toml
[package]
name = "mako-testdata"
version = "0.1.0"
edition.workspace = true

[dependencies]
mako-types = { path = "../mako-types" }
mako-fristen = { path = "../mako-fristen" }
```

`mako-testdata/src/lib.rs` (only `ids` initially — other modules added when their dependencies exist):
```rust
pub mod ids;
```

- [ ] **Step 7: Create stub modules so workspace compiles**

Each module file referenced in `lib.rs` needs to exist, even if empty. Create all files listed in the file structure with empty content or a single comment.

- [ ] **Step 8: Verify workspace compiles**

Run: `cargo check --workspace`
Expected: compiles with no errors (warnings about unused modules are OK)

- [ ] **Step 9: Commit**

```bash
git add -A
git commit -m "scaffold cargo workspace with 5 crates: types, fristen, quittung, gpke, testdata"
```

---

## Task 2: Sparte and MarktRolle Enums

**Files:**
- Create: `mako-types/src/sparte.rs`
- Create: `mako-types/src/rolle.rs`

- [ ] **Step 1: Write test for Sparte**

Add to `mako-types/src/sparte.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sparte {
	Strom,
	Gas,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sparte_display_and_equality() {
		assert_ne!(Sparte::Strom, Sparte::Gas);
		assert_eq!(Sparte::Strom, Sparte::Strom);
	}

	#[test]
	fn sparte_serializes_to_json() {
		let json = serde_json::to_string(&Sparte::Strom).unwrap();
		assert_eq!(json, "\"Strom\"");
		let json = serde_json::to_string(&Sparte::Gas).unwrap();
		assert_eq!(json, "\"Gas\"");
	}
}
```

- [ ] **Step 2: Write MarktRolle enum**

Add to `mako-types/src/rolle.rs`:
```rust
use serde::{Deserialize, Serialize};

use crate::sparte::Sparte;

/// All market roles in German energy market communication.
/// Roles are tagged with which Sparte(n) they participate in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarktRolle {
	// spartenuebergreifend
	Lieferant,
	LieferantNeu,
	LieferantAlt,
	Netzbetreiber,
	Messstellenbetreiber,
	Messdienstleister,
	Bilanzkreisverantwortlicher,

	// nur Strom
	Uebertragungsnetzbetreiber,
	Bilanzkoordinator,
	Einsatzverantwortlicher,
	BetreiberErzeugungsanlage,
	Direktvermarkter,
	Energieserviceanbieter,
	Aggregator,

	// nur Gas
	Fernleitungsnetzbetreiber,
	Marktgebietsverantwortlicher,
	Transportkunde,
	Speicherstellenbetreiber,
	Einspeisenetzbetreiber,
	Ausspeisenetzbetreiber,
}

impl MarktRolle {
	/// Returns the Sparten in which this role participates.
	pub fn sparten(&self) -> &'static [Sparte] {
		use MarktRolle::*;
		match self {
			Lieferant | LieferantNeu | LieferantAlt | Netzbetreiber
			| Messstellenbetreiber | Messdienstleister
			| Bilanzkreisverantwortlicher => &[Sparte::Strom, Sparte::Gas],

			Uebertragungsnetzbetreiber | Bilanzkoordinator
			| Einsatzverantwortlicher | BetreiberErzeugungsanlage
			| Direktvermarkter | Energieserviceanbieter | Aggregator => &[Sparte::Strom],

			Fernleitungsnetzbetreiber | Marktgebietsverantwortlicher
			| Transportkunde | Speicherstellenbetreiber
			| Einspeisenetzbetreiber | Ausspeisenetzbetreiber => &[Sparte::Gas],
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lieferant_is_spartenuebergreifend() {
		let sparten = MarktRolle::Lieferant.sparten();
		assert!(sparten.contains(&Sparte::Strom));
		assert!(sparten.contains(&Sparte::Gas));
	}

	#[test]
	fn uebertragungsnetzbetreiber_is_strom_only() {
		let sparten = MarktRolle::Uebertragungsnetzbetreiber.sparten();
		assert!(sparten.contains(&Sparte::Strom));
		assert!(!sparten.contains(&Sparte::Gas));
	}

	#[test]
	fn fernleitungsnetzbetreiber_is_gas_only() {
		let sparten = MarktRolle::Fernleitungsnetzbetreiber.sparten();
		assert!(!sparten.contains(&Sparte::Strom));
		assert!(sparten.contains(&Sparte::Gas));
	}
}
```

- [ ] **Step 3: Add serde_json dev-dependency to mako-types**

In `mako-types/Cargo.toml`, add:
```toml
[dev-dependencies]
serde_json = { workspace = true }
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p mako-types`
Expected: all tests pass

- [ ] **Step 5: Commit**

```bash
git add mako-types/
git commit -m "add Sparte, MarktRolle enums with sparten() method, tests"
```

---

## Task 3: Newtype IDs with Validation

**Files:**
- Create: `mako-types/src/ids.rs`
- Create: `mako-types/src/fehler.rs`

- [ ] **Step 1: Write ValidationError**

`mako-types/src/fehler.rs`:
```rust
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValidationError {
	#[error("invalid length: expected {expected}, got {actual}")]
	InvalidLength { expected: usize, actual: usize },

	#[error("invalid check digit: expected {expected}, got {actual}")]
	InvalidCheckDigit { expected: char, actual: char },

	#[error("invalid characters: only digits allowed")]
	InvalidCharacters,
}

/// Process-level errors for reducers
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ProzessFehler {
	#[error("invalid state transition: {state} cannot handle {event}")]
	UngueltigerUebergang { state: String, event: String },

	#[error("validation error: {0}")]
	Validierungsfehler(String),

	#[error("deadline exceeded: deadline was {frist}, received on {eingang}")]
	FristUeberschritten { frist: String, eingang: String },
}
```

- [ ] **Step 2: Write MaLoId with validation and tests**

`mako-types/src/ids.rs`:

MaLo-IDs are 11 digits. The last digit is a check digit calculated using the Luhn algorithm (modulus 10, weighted 1/2).

```rust
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::fehler::ValidationError;

/// Marktlokations-ID (11 digits, last digit = check digit per Luhn algorithm)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaLoId(String);

impl MaLoId {
	pub fn new(value: &str) -> Result<Self, ValidationError> {
		if value.len() != 11 {
			return Err(ValidationError::InvalidLength {
				expected: 11,
				actual: value.len(),
			});
		}
		if !value.chars().all(|c| c.is_ascii_digit()) {
			return Err(ValidationError::InvalidCharacters);
		}
		let expected = luhn_check_digit(&value[..10]);
		let actual = value.chars().last().unwrap();
		if actual != expected {
			return Err(ValidationError::InvalidCheckDigit { expected, actual });
		}
		Ok(Self(value.to_string()))
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for MaLoId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

/// Marktpartner-ID (13 digits, BDEW Codenummer)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MarktpartnerId(String);

impl MarktpartnerId {
	pub fn new(value: &str) -> Result<Self, ValidationError> {
		if value.len() != 13 {
			return Err(ValidationError::InvalidLength {
				expected: 13,
				actual: value.len(),
			});
		}
		if !value.chars().all(|c| c.is_ascii_digit()) {
			return Err(ValidationError::InvalidCharacters);
		}
		Ok(Self(value.to_string()))
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for MarktpartnerId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

/// Messlokations-ID (33 characters: "DE" + 31 alphanumeric)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MeLoId(String);

impl MeLoId {
	pub fn new(value: &str) -> Result<Self, ValidationError> {
		if value.len() != 33 {
			return Err(ValidationError::InvalidLength {
				expected: 33,
				actual: value.len(),
			});
		}
		if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
			return Err(ValidationError::InvalidCharacters);
		}
		Ok(Self(value.to_string()))
	}

	pub fn as_str(&self) -> &str {
		&self.0
	}
}

impl fmt::Display for MeLoId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

/// Luhn check digit calculation (modulus 10, weights 1 and 2 alternating from right)
fn luhn_check_digit(digits: &str) -> char {
	let sum: u32 = digits
		.chars()
		.rev()
		.enumerate()
		.map(|(i, c)| {
			let d = c.to_digit(10).unwrap();
			if i % 2 == 0 {
				let doubled = d * 2;
				if doubled > 9 { doubled - 9 } else { doubled }
			} else {
				d
			}
		})
		.sum();
	let check = (10 - (sum % 10)) % 10;
	char::from_digit(check, 10).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn valid_malo_id() {
		// 51238696781 is a known valid test MaLo-ID
		let result = MaLoId::new("51238696781");
		assert!(result.is_ok());
		assert_eq!(result.unwrap().as_str(), "51238696781");
	}

	#[test]
	fn malo_id_wrong_length() {
		let result = MaLoId::new("1234");
		assert_eq!(
			result,
			Err(ValidationError::InvalidLength {
				expected: 11,
				actual: 4
			})
		);
	}

	#[test]
	fn malo_id_non_digit() {
		let result = MaLoId::new("5123869678A");
		assert_eq!(result, Err(ValidationError::InvalidCharacters));
	}

	#[test]
	fn malo_id_wrong_check_digit() {
		// change last digit from 1 to 2
		let result = MaLoId::new("51238696782");
		assert!(matches!(result, Err(ValidationError::InvalidCheckDigit { .. })));
	}

	#[test]
	fn valid_marktpartner_id() {
		let result = MarktpartnerId::new("9900000000003");
		assert!(result.is_ok());
	}

	#[test]
	fn marktpartner_id_wrong_length() {
		let result = MarktpartnerId::new("12345");
		assert_eq!(
			result,
			Err(ValidationError::InvalidLength {
				expected: 13,
				actual: 5
			})
		);
	}

	#[test]
	fn valid_melo_id() {
		let id = "DE000000000000000000000000000000A";
		let result = MeLoId::new(id);
		assert!(result.is_ok());
	}

	#[test]
	fn melo_id_wrong_length() {
		let result = MeLoId::new("DE00");
		assert_eq!(
			result,
			Err(ValidationError::InvalidLength {
				expected: 33,
				actual: 4
			})
		);
	}

	#[test]
	fn luhn_known_values() {
		assert_eq!(luhn_check_digit("5123869678"), '1');
	}
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p mako-types`
Expected: all tests pass

- [ ] **Step 4: Commit**

```bash
git add mako-types/
git commit -m "add MaLoId, MeLoId, MarktpartnerId newtypes with Luhn validation, tests"
```

---

## Task 4: Reducer Trait, ProzessFehler, MakoVersion

**Files:**
- Create: `mako-types/src/reducer.rs`
- Create: `mako-types/src/version.rs`

- [ ] **Step 1: Write Reducer trait and ReducerOutput**

`mako-types/src/reducer.rs`:
```rust
use crate::fehler::ProzessFehler;
use crate::nachricht::Nachricht;

/// Output of a single reducer step.
#[derive(Debug, Clone, PartialEq)]
pub struct ReducerOutput<S> {
	pub state: S,
	pub nachrichten: Vec<Nachricht>,
}

/// The core trait every process crate implements.
pub trait Reducer {
	type State;
	type Event;

	fn reduce(
		state: Self::State,
		event: Self::Event,
	) -> Result<ReducerOutput<Self::State>, ProzessFehler>;
}
```

- [ ] **Step 2: Write MakoVersion enum and VersionDispatcher trait**

`mako-types/src/version.rs`:
```rust
use serde::{Deserialize, Serialize};

/// All MaKo format versions / epochs.
/// Each process crate has a module per version (e.g. `gpke::v2025`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MakoVersion {
	/// MaKo 2017: MaLo/MeLo model, MSB as new role
	V2017,
	/// MaKo 2020: New GPKE, WiM, MaBiS, MPES
	V2020,
	/// MaKo 2022: Extended network access processes
	V2022,
	/// FV2504/LFW24: 24h supplier switch, API web services
	V2025,
}

impl MakoVersion {
	pub fn gueltig_ab(&self) -> &'static str {
		match self {
			Self::V2017 => "2017-10-01",
			Self::V2020 => "2020-02-01",
			Self::V2022 => "2023-10-01",
			Self::V2025 => "2025-06-06",
		}
	}
}

/// Trait for version-aware dispatching.
/// Each process crate can implement this to route to the correct versioned reducer.
pub trait VersionDispatcher {
	type State;
	type Event;
	type Output;
	type Error;

	fn dispatch(
		version: MakoVersion,
		state: Self::State,
		event: Self::Event,
	) -> Result<Self::Output, Self::Error>;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn versions_are_ordered() {
		assert!(MakoVersion::V2017 < MakoVersion::V2020);
		assert!(MakoVersion::V2020 < MakoVersion::V2022);
		assert!(MakoVersion::V2022 < MakoVersion::V2025);
	}

	#[test]
	fn gueltig_ab_dates() {
		assert_eq!(MakoVersion::V2025.gueltig_ab(), "2025-06-06");
	}
}
```

- [ ] **Step 3: Write Nachricht enum stub**

`mako-types/src/nachricht.rs`:
```rust
use serde::{Deserialize, Serialize};

use crate::gpke_nachrichten::*;
use crate::ids::MarktpartnerId;
use crate::rolle::MarktRolle;

/// Envelope for any MaKo message, carrying routing info and typed payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nachricht {
	pub absender: MarktpartnerId,
	pub absender_rolle: MarktRolle,
	pub empfaenger: MarktpartnerId,
	pub empfaenger_rolle: MarktRolle,
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
}
```

- [ ] **Step 4: Write GPKE message structs**

`mako-types/src/gpke_nachrichten.rs`:
```rust
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::ids::{MaLoId, MarktpartnerId};

/// UTILMD Anmeldung: LFN -> NB (GPKE 1.1.1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdAnmeldung {
	pub malo_id: MaLoId,
	pub lieferant_neu: MarktpartnerId,
	pub lieferbeginn: NaiveDate,
}

/// UTILMD Bestaetigung: NB -> LFN (GPKE 1.1.2) or NB -> LFA (GPKE 1.1.6)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdBestaetigung {
	pub malo_id: MaLoId,
	pub bestaetigt_fuer: MarktpartnerId,
	pub lieferbeginn: NaiveDate,
}

/// UTILMD Abmeldung: NB -> LFA (GPKE 1.1.3)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdAbmeldung {
	pub malo_id: MaLoId,
	pub lieferant_alt: MarktpartnerId,
	pub lieferende: NaiveDate,
}

/// UTILMD Ablehnung: LFA -> NB (GPKE 1.1.4, rejection case)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdAblehnung {
	pub malo_id: MaLoId,
	pub grund: AblehnungsGrund,
}

/// UTILMD Zuordnung: NB -> LFN / NB -> LFA (GPKE 1.1.5 / 1.1.6)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilmdZuordnung {
	pub malo_id: MaLoId,
	pub zugeordnet_an: MarktpartnerId,
	pub lieferbeginn: NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AblehnungsGrund {
	Fristverletzung,
	MaloUnbekannt,
	KeinVertrag,
	Sonstiges(String),
}
```

- [ ] **Step 5: Write querschnitt.rs stubs**

`mako-types/src/querschnitt.rs`:
```rust
use serde::{Deserialize, Serialize};

/// IFTSTA status message (placeholder, cross-cutting)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Iftsta {
	pub status_code: String,
	pub beschreibung: String,
}

/// PARTIN market partner master data (placeholder, cross-cutting)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Partin {
	pub mp_id: String,
	pub name: String,
}

/// UTILTS calculation formulas / metering time definitions (placeholder)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Utilts {
	pub formel_id: String,
}
```

- [ ] **Step 6: Run tests**

Run: `cargo test -p mako-types`
Expected: all tests pass

- [ ] **Step 7: Verify workspace compiles**

Run: `cargo check --workspace`
Expected: no errors

- [ ] **Step 8: Commit**

```bash
git add mako-types/
git commit -m "add Reducer trait, MakoVersion enum, Nachricht envelope, GPKE message structs"
```

---

## Task 5: Feiertagskalender and Fristberechnung

**Files:**
- Create: `mako-fristen/src/feiertage.rs`
- Create: `mako-fristen/src/frist.rs`
- Create: `mako-fristen/src/zeitmodell.rs`

- [ ] **Step 1: Write failing tests for Feiertage**

`mako-fristen/src/feiertage.rs`:
```rust
use chrono::NaiveDate;
use std::collections::BTreeSet;

/// Holiday calendar — data, not code. Loadable, extendable.
#[derive(Debug, Clone)]
pub struct Feiertagskalender {
	feiertage: BTreeSet<NaiveDate>,
}

impl Feiertagskalender {
	pub fn new(feiertage: Vec<NaiveDate>) -> Self {
		Self {
			feiertage: feiertage.into_iter().collect(),
		}
	}

	pub fn ist_feiertag(&self, datum: NaiveDate) -> bool {
		self.feiertage.contains(&datum)
	}

	/// German federal holidays for a given year.
	/// These are the holidays that apply nationwide for MaKo deadline calculation.
	pub fn bundesweit(jahr: i32) -> Self {
		let d = |m: u32, t: u32| NaiveDate::from_ymd_opt(jahr, m, t).unwrap();
		let ostern = easter(jahr);

		let mut tage = vec![
			d(1, 1),   // Neujahr
			d(5, 1),   // Tag der Arbeit
			d(10, 3),  // Tag der Deutschen Einheit
			d(12, 25), // 1. Weihnachtsfeiertag
			d(12, 26), // 2. Weihnachtsfeiertag
			// bewegliche Feiertage
			ostern + chrono::Days::new(0),  // Ostersonntag (not all states, but MaKo uses it)
			ostern - chrono::Days::new(2),  // Karfreitag
			ostern + chrono::Days::new(1),  // Ostermontag
			ostern + chrono::Days::new(39), // Christi Himmelfahrt
			ostern + chrono::Days::new(49), // Pfingstsonntag
			ostern + chrono::Days::new(50), // Pfingstmontag
		];

		// Reformationstag (31.10.) is a federal holiday only for some states,
		// but MaKo treats it as a non-working day nationwide since MaKo 2020.
		tage.push(d(10, 31));

		Self::new(tage)
	}
}

/// Compute Easter Sunday using the Anonymous Gregorian algorithm.
fn easter(year: i32) -> NaiveDate {
	let a = year % 19;
	let b = year / 100;
	let c = year % 100;
	let d = b / 4;
	let e = b % 4;
	let f = (b + 8) / 25;
	let g = (b - f + 1) / 3;
	let h = (19 * a + b - d - g + 15) % 30;
	let i = c / 4;
	let k = c % 4;
	let l = (32 + 2 * e + 2 * i - h - k) % 7;
	let m = (a + 11 * h + 22 * l) / 451;
	let month = (h + l - 7 * m + 114) / 31;
	let day = (h + l - 7 * m + 114) % 31 + 1;
	NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn easter_2025() {
		assert_eq!(easter(2025), NaiveDate::from_ymd_opt(2025, 4, 20).unwrap());
	}

	#[test]
	fn easter_2026() {
		assert_eq!(easter(2026), NaiveDate::from_ymd_opt(2026, 4, 5).unwrap());
	}

	#[test]
	fn karfreitag_2025_is_holiday() {
		let kal = Feiertagskalender::bundesweit(2025);
		let karfreitag = NaiveDate::from_ymd_opt(2025, 4, 18).unwrap();
		assert!(kal.ist_feiertag(karfreitag));
	}

	#[test]
	fn normal_monday_is_not_holiday() {
		let kal = Feiertagskalender::bundesweit(2025);
		let monday = NaiveDate::from_ymd_opt(2025, 3, 10).unwrap();
		assert!(!kal.ist_feiertag(monday));
	}

	#[test]
	fn neujahr_is_holiday() {
		let kal = Feiertagskalender::bundesweit(2026);
		assert!(kal.ist_feiertag(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()));
	}

	#[test]
	fn weihnachten_is_holiday() {
		let kal = Feiertagskalender::bundesweit(2025);
		assert!(kal.ist_feiertag(NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()));
		assert!(kal.ist_feiertag(NaiveDate::from_ymd_opt(2025, 12, 26).unwrap()));
	}
}
```

- [ ] **Step 2: Run tests to verify they pass**

Run: `cargo test -p mako-fristen -- feiertage`
Expected: all pass

- [ ] **Step 3: Write Fristberechnung with tests**

`mako-fristen/src/frist.rs`:
```rust
use chrono::{Datelike, NaiveDate, Weekday};

use crate::feiertage::Feiertagskalender;
use mako_types::sparte::Sparte;

/// Calculates a MaKo deadline: starting from `datum`, advance `werktage` working days,
/// skipping weekends and holidays. Sparte matters for the Gastag offset but not for
/// the deadline calculation itself (deadlines are always in calendar days / working days).
pub fn frist(
	datum: NaiveDate,
	werktage: u32,
	kalender: &Feiertagskalender,
	_sparte: Sparte,
) -> NaiveDate {
	let mut remaining = werktage;
	let mut current = datum;
	while remaining > 0 {
		current = current.succ_opt().expect("date overflow");
		if ist_werktag(current, kalender) {
			remaining -= 1;
		}
	}
	current
}

/// A Werktag is Mon-Fri and not a holiday.
pub fn ist_werktag(datum: NaiveDate, kalender: &Feiertagskalender) -> bool {
	let weekday = datum.weekday();
	weekday != Weekday::Sat && weekday != Weekday::Sun && !kalender.ist_feiertag(datum)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn kal2025() -> Feiertagskalender {
		Feiertagskalender::bundesweit(2025)
	}

	#[test]
	fn one_werktag_from_monday() {
		// 2025-03-10 is Monday
		let start = NaiveDate::from_ymd_opt(2025, 3, 10).unwrap();
		let result = frist(start, 1, &kal2025(), Sparte::Strom);
		assert_eq!(result, NaiveDate::from_ymd_opt(2025, 3, 11).unwrap()); // Tuesday
	}

	#[test]
	fn one_werktag_from_friday_skips_weekend() {
		// 2025-03-14 is Friday
		let start = NaiveDate::from_ymd_opt(2025, 3, 14).unwrap();
		let result = frist(start, 1, &kal2025(), Sparte::Strom);
		assert_eq!(result, NaiveDate::from_ymd_opt(2025, 3, 17).unwrap()); // Monday
	}

	#[test]
	fn werktag_skips_holiday() {
		// Karfreitag 2025 = 18.04, Ostermontag = 21.04
		// Start: Thursday 17.04, 1 WT should land on Tuesday 22.04
		let start = NaiveDate::from_ymd_opt(2025, 4, 17).unwrap();
		let result = frist(start, 1, &kal2025(), Sparte::Strom);
		assert_eq!(result, NaiveDate::from_ymd_opt(2025, 4, 22).unwrap());
	}

	#[test]
	fn ec7_jahreswechsel() {
		// 2025-12-24 is Wednesday. 25+26 are holidays, 27+28 weekend, 01.01 holiday
		// 1 WT from 24.12 should land on 29.12 (Monday)
		let start = NaiveDate::from_ymd_opt(2025, 12, 24).unwrap();
		let result = frist(start, 1, &kal2025(), Sparte::Strom);
		assert_eq!(result, NaiveDate::from_ymd_opt(2025, 12, 29).unwrap());
	}

	#[test]
	fn three_werktage_normal_week() {
		let start = NaiveDate::from_ymd_opt(2025, 3, 10).unwrap(); // Monday
		let result = frist(start, 3, &kal2025(), Sparte::Strom);
		assert_eq!(result, NaiveDate::from_ymd_opt(2025, 3, 13).unwrap()); // Thursday
	}
}
```

- [ ] **Step 4: Write Zeitmodell with tests**

`mako-fristen/src/zeitmodell.rs`:
```rust
use chrono::NaiveTime;

use mako_types::sparte::Sparte;

/// Returns the start-of-day time for a given Sparte.
/// Strom: 00:00, Gas: 06:00 (Gastag convention)
pub fn tagesbeginn(sparte: Sparte) -> NaiveTime {
	match sparte {
		Sparte::Strom => NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
		Sparte::Gas => NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
	}
}

/// Duration of a day in hours for each Sparte.
/// Gastag always has 24 hours (06:00 to 06:00 next day, spanning DST changes).
/// Stromtag can have 23 or 25 hours during DST transitions.
pub fn tag_stunden(sparte: Sparte, ist_dst_umstellung: bool) -> u32 {
	match sparte {
		Sparte::Gas => 24, // always
		Sparte::Strom => {
			if ist_dst_umstellung {
				// caller must determine direction; this is a simplification
				// for now we return 24 as default, DST edge cases handled by EC1/EC3
				24
			} else {
				24
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn strom_starts_at_midnight() {
		let t = tagesbeginn(Sparte::Strom);
		assert_eq!(t, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
	}

	#[test]
	fn gas_starts_at_six() {
		let t = tagesbeginn(Sparte::Gas);
		assert_eq!(t, NaiveTime::from_hms_opt(6, 0, 0).unwrap());
	}

	#[test]
	fn gastag_always_24_hours() {
		assert_eq!(tag_stunden(Sparte::Gas, false), 24);
		assert_eq!(tag_stunden(Sparte::Gas, true), 24);
	}
}
```

- [ ] **Step 5: Run all fristen tests**

Run: `cargo test -p mako-fristen`
Expected: all tests pass

- [ ] **Step 6: Commit**

```bash
git add mako-fristen/
git commit -m "add Feiertagskalender (Easter algorithm, bundesweite Feiertage), frist(), Zeitmodell"
```

---

## Task 6: Quittungsschicht (CONTRL, APERAK, Decorator)

**Files:**
- Create: `mako-quittung/src/types.rs`
- Create: `mako-quittung/src/contrl.rs`
- Create: `mako-quittung/src/aperak.rs`
- Create: `mako-quittung/src/decorator.rs`

- [ ] **Step 1: Write quittung types**

`mako-quittung/src/types.rs`:
```rust
use serde::{Deserialize, Serialize};

use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::Nachricht;

/// Receipt sent back to the sender of the original message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quittung {
	pub an: MarktpartnerId,
	pub typ: QuittungsTyp,
	pub ergebnis: QuittungsErgebnis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuittungsTyp {
	Contrl,
	Aperak,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuittungsErgebnis {
	Positiv,
	Negativ(Vec<FehlerCode>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FehlerCode {
	pub code: String,
	pub text: String,
}

/// Output of a decorated reducer: separates process messages from receipts.
#[derive(Debug, Clone, PartialEq)]
pub struct DekorierterOutput<S> {
	pub state: S,
	pub nachrichten: Vec<Nachricht>,
	pub quittungen: Vec<Quittung>,
}
```

- [ ] **Step 2: Write CONTRL checker with tests**

`mako-quittung/src/contrl.rs`:
```rust
use mako_types::nachricht::Nachricht;

use crate::types::{FehlerCode, QuittungsErgebnis};

/// CONTRL syntax check — pure, deterministic.
/// For now: validates that the message has non-empty sender/recipient.
/// Will be extended with EDIFACT segment-level validation in the codec phase.
pub fn contrl_pruefen(nachricht: &Nachricht) -> QuittungsErgebnis {
	let mut fehler = Vec::new();

	if nachricht.absender.as_str().is_empty() {
		fehler.push(FehlerCode {
			code: "SYN001".to_string(),
			text: "Absender-ID leer".to_string(),
		});
	}

	if nachricht.empfaenger.as_str().is_empty() {
		fehler.push(FehlerCode {
			code: "SYN002".to_string(),
			text: "Empfaenger-ID leer".to_string(),
		});
	}

	if fehler.is_empty() {
		QuittungsErgebnis::Positiv
	} else {
		QuittungsErgebnis::Negativ(fehler)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mako_types::gpke_nachrichten::*;
	use mako_types::ids::*;
	use mako_types::nachricht::*;
	use mako_types::rolle::MarktRolle;

	fn test_nachricht() -> Nachricht {
		Nachricht {
			absender: MarktpartnerId::new("9900000000003").unwrap(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: MarktpartnerId::new("9900000000010").unwrap(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696781").unwrap(),
				lieferant_neu: MarktpartnerId::new("9900000000003").unwrap(),
				lieferbeginn: chrono::NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
			}),
		}
	}

	#[test]
	fn valid_message_passes_contrl() {
		let result = contrl_pruefen(&test_nachricht());
		assert_eq!(result, QuittungsErgebnis::Positiv);
	}
}
```

- [ ] **Step 3: Write APERAK checker with tests**

`mako-quittung/src/aperak.rs`:
```rust
use mako_types::nachricht::{Nachricht, NachrichtenPayload};

use crate::types::{FehlerCode, QuittungsErgebnis};

/// APERAK application-level check — EBD decision tree logic.
/// For GPKE LFW: validates that the Anmeldung has a future Lieferbeginn.
/// Will be extended with full EBD logic per process step.
pub fn aperak_pruefen(nachricht: &Nachricht, stichtag: chrono::NaiveDate) -> QuittungsErgebnis {
	let mut fehler = Vec::new();

	match &nachricht.payload {
		NachrichtenPayload::UtilmdAnmeldung(anmeldung) => {
			if anmeldung.lieferbeginn <= stichtag {
				fehler.push(FehlerCode {
					code: "EBD_A01".to_string(),
					text: "Lieferbeginn muss in der Zukunft liegen".to_string(),
				});
			}
		}
		_ => {
			// other message types: pass through for now
		}
	}

	if fehler.is_empty() {
		QuittungsErgebnis::Positiv
	} else {
		QuittungsErgebnis::Negativ(fehler)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mako_types::gpke_nachrichten::*;
	use mako_types::ids::*;
	use mako_types::nachricht::*;
	use mako_types::rolle::MarktRolle;

	fn anmeldung_nachricht(lieferbeginn: chrono::NaiveDate) -> Nachricht {
		Nachricht {
			absender: MarktpartnerId::new("9900000000003").unwrap(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: MarktpartnerId::new("9900000000010").unwrap(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696781").unwrap(),
				lieferant_neu: MarktpartnerId::new("9900000000003").unwrap(),
				lieferbeginn,
			}),
		}
	}

	#[test]
	fn future_lieferbeginn_passes() {
		let stichtag = chrono::NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
		let msg = anmeldung_nachricht(chrono::NaiveDate::from_ymd_opt(2025, 7, 1).unwrap());
		assert_eq!(aperak_pruefen(&msg, stichtag), QuittungsErgebnis::Positiv);
	}

	#[test]
	fn past_lieferbeginn_fails() {
		let stichtag = chrono::NaiveDate::from_ymd_opt(2025, 8, 1).unwrap();
		let msg = anmeldung_nachricht(chrono::NaiveDate::from_ymd_opt(2025, 7, 1).unwrap());
		let result = aperak_pruefen(&msg, stichtag);
		assert!(matches!(result, QuittungsErgebnis::Negativ(_)));
	}
}
```

- [ ] **Step 4: Write decorator with tests**

`mako-quittung/src/decorator.rs`:
```rust
use mako_types::fehler::ProzessFehler;
use mako_types::nachricht::Nachricht;
use mako_types::reducer::ReducerOutput;

use crate::aperak::aperak_pruefen;
use crate::contrl::contrl_pruefen;
use crate::types::{DekorierterOutput, Quittung, QuittungsErgebnis, QuittungsTyp};

/// Wraps a reducer with CONTRL + APERAK checks.
/// 1. CONTRL check -> if fail: return CONTRL-negativ, don't call reducer
/// 2. APERAK check -> if fail: return CONTRL-positiv + APERAK-negativ, don't call reducer
/// 3. Both pass -> call reducer, return CONTRL-positiv + APERAK-positiv + reducer output
pub fn mit_quittung<S, E, F>(
	nachricht: &Nachricht,
	state: S,
	event: E,
	stichtag: chrono::NaiveDate,
	reducer_fn: F,
) -> Result<DekorierterOutput<S>, ProzessFehler>
where
	F: FnOnce(S, E) -> Result<ReducerOutput<S>, ProzessFehler>,
	S: Clone,
{
	let sender = nachricht.absender.clone();

	// Step 1: CONTRL
	let contrl_ergebnis = contrl_pruefen(nachricht);
	if let QuittungsErgebnis::Negativ(codes) = &contrl_ergebnis {
		return Ok(DekorierterOutput {
			state,
			nachrichten: vec![],
			quittungen: vec![Quittung {
				an: sender,
				typ: QuittungsTyp::Contrl,
				ergebnis: QuittungsErgebnis::Negativ(codes.clone()),
			}],
		});
	}

	// Step 2: APERAK
	let aperak_ergebnis = aperak_pruefen(nachricht, stichtag);
	if let QuittungsErgebnis::Negativ(codes) = &aperak_ergebnis {
		return Ok(DekorierterOutput {
			state,
			nachrichten: vec![],
			quittungen: vec![
				Quittung {
					an: sender.clone(),
					typ: QuittungsTyp::Contrl,
					ergebnis: QuittungsErgebnis::Positiv,
				},
				Quittung {
					an: sender,
					typ: QuittungsTyp::Aperak,
					ergebnis: QuittungsErgebnis::Negativ(codes.clone()),
				},
			],
		});
	}

	// Step 3: Both passed -> call reducer
	let output = reducer_fn(state, event)?;

	Ok(DekorierterOutput {
		state: output.state,
		nachrichten: output.nachrichten,
		quittungen: vec![
			Quittung {
				an: sender.clone(),
				typ: QuittungsTyp::Contrl,
				ergebnis: QuittungsErgebnis::Positiv,
			},
			Quittung {
				an: sender,
				typ: QuittungsTyp::Aperak,
				ergebnis: QuittungsErgebnis::Positiv,
			},
		],
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use mako_types::gpke_nachrichten::*;
	use mako_types::ids::*;
	use mako_types::nachricht::*;
	use mako_types::rolle::MarktRolle;

	fn test_nachricht(lieferbeginn: chrono::NaiveDate) -> Nachricht {
		Nachricht {
			absender: MarktpartnerId::new("9900000000003").unwrap(),
			absender_rolle: MarktRolle::LieferantNeu,
			empfaenger: MarktpartnerId::new("9900000000010").unwrap(),
			empfaenger_rolle: MarktRolle::Netzbetreiber,
			payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
				malo_id: MaLoId::new("51238696781").unwrap(),
				lieferant_neu: MarktpartnerId::new("9900000000003").unwrap(),
				lieferbeginn,
			}),
		}
	}

	fn dummy_reducer(state: &str, _event: &str) -> Result<ReducerOutput<String>, ProzessFehler> {
		Ok(ReducerOutput {
			state: format!("{state}_processed"),
			nachrichten: vec![],
		})
	}

	#[test]
	fn decorator_passes_through_on_valid_message() {
		let stichtag = chrono::NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
		let msg = test_nachricht(chrono::NaiveDate::from_ymd_opt(2025, 7, 1).unwrap());

		let result = mit_quittung(&msg, "idle".to_string(), "event", stichtag, |s, _e| {
			dummy_reducer(&s, "")
		});

		let output = result.unwrap();
		assert_eq!(output.state, "idle_processed");
		assert_eq!(output.quittungen.len(), 2);
		assert_eq!(output.quittungen[0].typ, QuittungsTyp::Contrl);
		assert_eq!(output.quittungen[0].ergebnis, QuittungsErgebnis::Positiv);
		assert_eq!(output.quittungen[1].typ, QuittungsTyp::Aperak);
		assert_eq!(output.quittungen[1].ergebnis, QuittungsErgebnis::Positiv);
	}

	#[test]
	fn decorator_blocks_on_aperak_failure() {
		let stichtag = chrono::NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(); // after lieferbeginn
		let msg = test_nachricht(chrono::NaiveDate::from_ymd_opt(2025, 7, 1).unwrap());

		let result = mit_quittung(&msg, "idle".to_string(), "event", stichtag, |s, _e| {
			dummy_reducer(&s, "")
		});

		let output = result.unwrap();
		assert_eq!(output.state, "idle".to_string()); // state unchanged
		assert_eq!(output.nachrichten.len(), 0); // no process messages
		assert_eq!(output.quittungen.len(), 2);
		assert_eq!(output.quittungen[0].ergebnis, QuittungsErgebnis::Positiv); // CONTRL ok
		assert!(matches!(
			output.quittungen[1].ergebnis,
			QuittungsErgebnis::Negativ(_)
		)); // APERAK fail
	}
}
```

- [ ] **Step 5: Run all quittung tests**

Run: `cargo test -p mako-quittung`
Expected: all tests pass

- [ ] **Step 6: Commit**

```bash
git add mako-quittung/
git commit -m "add CONTRL/APERAK checkers, mit_quittung() decorator with separated routing"
```

---

## Task 7: GPKE Lieferantenwechsel v2025 Reducer

**Files:**
- Create: `mako-gpke/src/v2025/mod.rs`
- Create: `mako-gpke/src/v2025/lfw.rs`
- Create: `mako-gpke/src/v2025/lfw_tests.rs`

This is the core of the plan: the first reducer. **Strict TDD:** types first, then tests (red), then implementation (green).

- [ ] **Step 1: Write LfwState, LfwEvent enums AND reduce() signature (stub that always returns Err)**

`mako-gpke/src/v2025/mod.rs`:
```rust
pub mod lfw;

#[cfg(test)]
mod lfw_tests;
```

`mako-gpke/src/v2025/lfw.rs`:
```rust
use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::*;
use mako_types::ids::{MaLoId, MarktpartnerId};
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::reducer::ReducerOutput;
use mako_types::rolle::MarktRolle;

/// GPKE Lieferantenwechsel v2025 (post-LFW24) state machine.
///
/// Models the 6-step process from the Kommunikationslinien-Checkliste:
/// 1.1.1 LFN -> NB: Anmeldung
/// 1.1.2 NB -> LFN: Bestaetigung
/// 1.1.3 NB -> LFA: Abmeldung
/// 1.1.4 LFA -> NB: Bestaetigung/Ablehnung
/// 1.1.5 NB -> LFN: Zuordnungsbestaetigung
/// 1.1.6 NB -> LFA: Zuordnungsbestaetigung (Abmeldeliste)
#[derive(Debug, Clone, PartialEq)]
pub enum LfwState {
	/// No active process
	Idle,

	/// 1.1.1 received: NB is processing the Anmeldung
	AnmeldungEingegangen {
		malo: MaLoId,
		lfn: MarktpartnerId,
		nb: MarktpartnerId,
		lieferbeginn: NaiveDate,
	},

	/// 1.1.2 + 1.1.3 sent: waiting for LFA response
	AbmeldungAnLfaGesendet {
		malo: MaLoId,
		lfn: MarktpartnerId,
		lfa: MarktpartnerId,
		nb: MarktpartnerId,
		lieferbeginn: NaiveDate,
	},

	/// 1.1.4 received (confirmation): waiting for Widerspruchsfrist
	WiderspruchsfristLaeuft {
		malo: MaLoId,
		lfn: MarktpartnerId,
		lfa: MarktpartnerId,
		nb: MarktpartnerId,
		lieferbeginn: NaiveDate,
		frist_bis: NaiveDate,
	},

	/// 1.1.5 + 1.1.6 sent: process complete
	Zugeordnet {
		malo: MaLoId,
		lfn: MarktpartnerId,
		lieferbeginn: NaiveDate,
	},

	/// Process ended with rejection
	Abgelehnt {
		malo: MaLoId,
		grund: AblehnungsGrund,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum LfwEvent {
	/// 1.1.1: LFN sends Anmeldung to NB
	AnmeldungEmpfangen(UtilmdAnmeldung),

	/// 1.1.2 + 1.1.3: NB confirms to LFN and notifies LFA
	AnmeldungBestaetigt {
		lfa: MarktpartnerId,
	},

	/// 1.1.4: LFA confirms the Abmeldung
	LfaHatBestaetigt,

	/// 1.1.4: LFA rejects (Widerspruch)
	LfaHatAbgelehnt {
		grund: AblehnungsGrund,
	},

	/// Widerspruchsfrist expired without rejection
	WiderspruchsfristAbgelaufen,

	/// Timeout: no response within deadline (EC6)
	FristUeberschritten,
}

/// The pure reducer function.
/// STUB: Returns Err for all inputs. Implementation in Step 3.
pub fn reduce(
	state: LfwState,
	event: LfwEvent,
) -> Result<ReducerOutput<LfwState>, ProzessFehler> {
	Err(ProzessFehler::UngueltigerUebergang {
		state: format!("{state:?}"),
		event: format!("{event:?}"),
	})
}
```

- [ ] **Step 2: Write ALL tests (happy path, invalid transitions, edge cases EC4/EC5/EC6)**

These tests will all FAIL against the stub. That's correct — red phase of TDD.

`mako-gpke/src/v2025/lfw_tests.rs`:
```rust
use chrono::NaiveDate;

use mako_types::fehler::ProzessFehler;
use mako_types::gpke_nachrichten::*;
use mako_types::ids::*;
use mako_types::nachricht::NachrichtenPayload;

use super::lfw::*;

fn malo() -> MaLoId {
	MaLoId::new("51238696781").unwrap()
}

fn lfn_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000003").unwrap()
}

fn lfa_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000027").unwrap()
}

fn nb_id() -> MarktpartnerId {
	MarktpartnerId::new("9900000000010").unwrap()
}

fn lieferbeginn() -> NaiveDate {
	NaiveDate::from_ymd_opt(2025, 7, 1).unwrap()
}

fn anmeldung() -> UtilmdAnmeldung {
	UtilmdAnmeldung {
		malo_id: malo(),
		lieferant_neu: lfn_id(),
		lieferbeginn: lieferbeginn(),
	}
}

// ========== Happy Path ==========

#[test]
fn idle_plus_anmeldung_transitions_to_eingegangen() {
	let result = reduce(LfwState::Idle, LfwEvent::AnmeldungEmpfangen(anmeldung()));
	let output = result.unwrap();

	assert!(matches!(output.state, LfwState::AnmeldungEingegangen { .. }));
	if let LfwState::AnmeldungEingegangen { malo, lfn, .. } = &output.state {
		assert_eq!(malo, &malo());
		assert_eq!(lfn, &lfn_id());
	}
}

#[test]
fn eingegangen_plus_bestaetigt_sends_two_messages() {
	let state = LfwState::AnmeldungEingegangen {
		malo: malo(),
		lfn: lfn_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};

	let result = reduce(state, LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() });
	let output = result.unwrap();

	assert!(matches!(
		output.state,
		LfwState::AbmeldungAnLfaGesendet { .. }
	));
	assert_eq!(output.nachrichten.len(), 2);

	// First message: Bestaetigung to LFN
	assert!(matches!(
		output.nachrichten[0].payload,
		NachrichtenPayload::UtilmdBestaetigung(_)
	));

	// Second message: Abmeldung to LFA
	assert!(matches!(
		output.nachrichten[1].payload,
		NachrichtenPayload::UtilmdAbmeldung(_)
	));
}

#[test]
fn abmeldung_gesendet_plus_lfa_bestaetigt_transitions_to_widerspruchsfrist() {
	let state = LfwState::AbmeldungAnLfaGesendet {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};

	let result = reduce(state, LfwEvent::LfaHatBestaetigt);
	let output = result.unwrap();

	assert!(matches!(
		output.state,
		LfwState::WiderspruchsfristLaeuft { .. }
	));
}

#[test]
fn widerspruchsfrist_abgelaufen_transitions_to_zugeordnet() {
	let state = LfwState::WiderspruchsfristLaeuft {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
		frist_bis: NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
	};

	let result = reduce(state, LfwEvent::WiderspruchsfristAbgelaufen);
	let output = result.unwrap();

	assert!(matches!(output.state, LfwState::Zugeordnet { .. }));
	assert_eq!(output.nachrichten.len(), 2); // Zuordnung to LFN + LFA
}

#[test]
fn full_happy_path_idle_to_zugeordnet() {
	// Step 1: Anmeldung
	let out1 = reduce(LfwState::Idle, LfwEvent::AnmeldungEmpfangen(anmeldung())).unwrap();

	// Step 2: NB bestaetigt
	let out2 = reduce(out1.state, LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() }).unwrap();

	// Step 3: LFA bestaetigt
	let out3 = reduce(out2.state, LfwEvent::LfaHatBestaetigt).unwrap();

	// Step 4: Widerspruchsfrist abgelaufen
	let out4 = reduce(out3.state, LfwEvent::WiderspruchsfristAbgelaufen).unwrap();

	assert!(matches!(out4.state, LfwState::Zugeordnet { .. }));
	if let LfwState::Zugeordnet { malo, lfn, .. } = &out4.state {
		assert_eq!(malo, &malo());
		assert_eq!(lfn, &lfn_id());
	}
}

// ========== Rejection Path ==========

#[test]
fn lfa_ablehnung_transitions_to_abgelehnt() {
	let state = LfwState::AbmeldungAnLfaGesendet {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};

	let result = reduce(
		state,
		LfwEvent::LfaHatAbgelehnt {
			grund: AblehnungsGrund::KeinVertrag,
		},
	);
	let output = result.unwrap();

	assert!(matches!(output.state, LfwState::Abgelehnt { .. }));
	if let LfwState::Abgelehnt { grund, .. } = &output.state {
		assert_eq!(grund, &AblehnungsGrund::KeinVertrag);
	}
}

// ========== Invalid Transitions ==========

#[test]
fn idle_cannot_receive_bestaetigung() {
	let result = reduce(
		LfwState::Idle,
		LfwEvent::AnmeldungBestaetigt { lfa: lfa_id() },
	);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn zugeordnet_cannot_receive_any_event() {
	let state = LfwState::Zugeordnet {
		malo: malo(),
		lfn: lfn_id(),
		lieferbeginn: lieferbeginn(),
	};
	let result = reduce(state, LfwEvent::LfaHatBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn abgelehnt_cannot_receive_any_event() {
	let state = LfwState::Abgelehnt {
		malo: malo(),
		grund: AblehnungsGrund::Fristverletzung,
	};
	let result = reduce(state, LfwEvent::WiderspruchsfristAbgelaufen);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

#[test]
fn eingegangen_cannot_receive_lfa_bestaetigt() {
	let state = LfwState::AnmeldungEingegangen {
		malo: malo(),
		lfn: lfn_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};
	let result = reduce(state, LfwEvent::LfaHatBestaetigt);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

// ========== Edge Cases ==========

#[test]
fn ec6_timeout_from_anmeldung_eingegangen() {
	let state = LfwState::AnmeldungEingegangen {
		malo: malo(),
		lfn: lfn_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};

	let result = reduce(state, LfwEvent::FristUeberschritten);
	let output = result.unwrap();

	assert!(matches!(output.state, LfwState::Abgelehnt { .. }));
	if let LfwState::Abgelehnt { grund, .. } = &output.state {
		assert_eq!(grund, &AblehnungsGrund::Fristverletzung);
	}
}

#[test]
fn ec6_timeout_from_abmeldung_gesendet() {
	let state = LfwState::AbmeldungAnLfaGesendet {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};

	let result = reduce(state, LfwEvent::FristUeberschritten);
	let output = result.unwrap();

	assert!(matches!(output.state, LfwState::Abgelehnt { .. }));
}

#[test]
fn ec6_timeout_from_widerspruchsfrist() {
	let state = LfwState::WiderspruchsfristLaeuft {
		malo: malo(),
		lfn: lfn_id(),
		lfa: lfa_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
		frist_bis: NaiveDate::from_ymd_opt(2025, 6, 30).unwrap(),
	};

	let result = reduce(state, LfwEvent::FristUeberschritten);
	let output = result.unwrap();

	assert!(matches!(output.state, LfwState::Abgelehnt { .. }));
	if let LfwState::Abgelehnt { grund, .. } = &output.state {
		assert_eq!(grund, &AblehnungsGrund::Fristverletzung);
	}
}

// ========== EC4: Simultaneous Registration ==========

#[test]
fn ec4_second_anmeldung_while_process_running_is_invalid() {
	// A second Anmeldung for the same MaLo while a process is already running
	// must be rejected (state is not Idle).
	let state = LfwState::AnmeldungEingegangen {
		malo: malo(),
		lfn: lfn_id(),
		nb: nb_id(),
		lieferbeginn: lieferbeginn(),
	};

	let second_anmeldung = UtilmdAnmeldung {
		malo_id: malo(),
		lieferant_neu: MarktpartnerId::new("9900000000034").unwrap(), // different LF
		lieferbeginn: lieferbeginn(),
	};

	let result = reduce(state, LfwEvent::AnmeldungEmpfangen(second_anmeldung));
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}

// ========== EC5: Grundversorgung Fallback ==========

#[test]
fn ec5_grundversorgung_after_rejection_leaves_abgelehnt() {
	// When a LFW is rejected, the MaLo falls back to Grundversorgung.
	// In our model, Abgelehnt is the terminal state — the Grundversorgung
	// is an implicit market state, not a reducer state. The reducer correctly
	// ends in Abgelehnt; the calling system interprets this as Grundversorgung
	// if no other LF is assigned.
	let state = LfwState::Abgelehnt {
		malo: malo(),
		grund: AblehnungsGrund::KeinVertrag,
	};

	// No further transitions are possible from Abgelehnt
	let result = reduce(state, LfwEvent::WiderspruchsfristAbgelaufen);
	assert!(matches!(result, Err(ProzessFehler::UngueltigerUebergang { .. })));
}
```

- [ ] **Step 3: Run tests to verify they FAIL (red phase)**

Run: `cargo test -p mako-gpke`
Expected: ALL tests FAIL with `UngueltigerUebergang` (the stub returns Err for everything)

- [ ] **Step 4: Commit failing tests**

```bash
git add mako-gpke/
git commit -m "add GPKE LFW v2025 tests: happy path, rejection, EC4/EC5/EC6 (all red)"
```

- [ ] **Step 5: Implement reduce() — replace stub with full implementation**

Replace the stub `reduce()` function in `mako-gpke/src/v2025/lfw.rs` with the full match implementation. All match arms:

1. `(Idle, AnmeldungEmpfangen)` → `AnmeldungEingegangen`
2. `(AnmeldungEingegangen, AnmeldungBestaetigt)` → `AbmeldungAnLfaGesendet` + 2 messages
3. `(AbmeldungAnLfaGesendet, LfaHatBestaetigt)` → `WiderspruchsfristLaeuft`
4. `(AbmeldungAnLfaGesendet, LfaHatAbgelehnt)` → `Abgelehnt`
5. `(WiderspruchsfristLaeuft, WiderspruchsfristAbgelaufen)` → `Zugeordnet` + 2 messages
6. `(AnmeldungEingegangen | AbmeldungAnLfaGesendet | WiderspruchsfristLaeuft, FristUeberschritten)` → `Abgelehnt` (EC6 — all 3 waiting states)
7. `(_, _)` → `Err(UngueltigerUebergang)` (catches EC4, EC5 implicitly)

The full implementation code is provided in the spec design section 2.2. Ensure the EC6 timeout match covers all three waiting states (AnmeldungEingegangen, AbmeldungAnLfaGesendet, WiderspruchsfristLaeuft).

- [ ] **Step 6: Run tests to verify they PASS (green phase)**

Run: `cargo test -p mako-gpke`
Expected: ALL tests pass

- [ ] **Step 7: Verify entire workspace**

Run: `cargo test --workspace`
Expected: all tests across all crates pass

- [ ] **Step 8: Commit**

```bash
git add mako-gpke/
git commit -m "implement GPKE LFW v2025 reducer: all tests green"
```

---

## Task 8: Test Data Generator

**Files:**
- Create: `mako-testdata/src/ids.rs`
- Create: `mako-testdata/src/utilmd.rs`
- Create: `mako-testdata/src/quittungen.rs`
- Create: `mako-testdata/src/szenarien.rs`

- [ ] **Step 1: Write test ID generators**

`mako-testdata/src/ids.rs`:
```rust
use mako_types::ids::{MaLoId, MarktpartnerId, MeLoId};

/// Generate a deterministic test MaLo-ID. Index 0..99.
pub fn test_malo(index: u8) -> MaLoId {
	// base: 5123869678, compute check digit for each index
	let base = format!("512386{:04}", index);
	let check = luhn_check(&base);
	MaLoId::new(&format!("{base}{check}")).expect("test MaLo-ID must be valid")
}

/// Generate a deterministic test MarktpartnerId. Index 0..99.
pub fn test_mp_id(index: u8) -> MarktpartnerId {
	let id = format!("990000000{:04}", index);
	MarktpartnerId::new(&id).expect("test MP-ID must be valid")
}

/// Generate a deterministic test MeLoId.
pub fn test_melo(index: u8) -> MeLoId {
	let id = format!("DE{:031}", index);
	MeLoId::new(&id).expect("test MeLo-ID must be valid")
}

fn luhn_check(digits: &str) -> char {
	let sum: u32 = digits
		.chars()
		.rev()
		.enumerate()
		.map(|(i, c)| {
			let d = c.to_digit(10).unwrap();
			if i % 2 == 0 {
				let doubled = d * 2;
				if doubled > 9 { doubled - 9 } else { doubled }
			} else {
				d
			}
		})
		.sum();
	let check = (10 - (sum % 10)) % 10;
	char::from_digit(check, 10).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_malo_generates_valid_ids() {
		for i in 0..10 {
			let id = test_malo(i);
			assert_eq!(id.as_str().len(), 11);
		}
	}

	#[test]
	fn test_mp_id_generates_valid_ids() {
		for i in 0..10 {
			let id = test_mp_id(i);
			assert_eq!(id.as_str().len(), 13);
		}
	}

	#[test]
	fn test_melo_generates_valid_ids() {
		let id = test_melo(0);
		assert_eq!(id.as_str().len(), 33);
	}

	#[test]
	fn ids_are_deterministic() {
		assert_eq!(test_malo(0), test_malo(0));
		assert_ne!(test_malo(0), test_malo(1));
	}
}
```

- [ ] **Step 2: Write UTILMD generators**

`mako-testdata/src/utilmd.rs`:
```rust
use chrono::NaiveDate;

use mako_types::gpke_nachrichten::*;
use mako_types::ids::MarktpartnerId;
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::rolle::MarktRolle;

use crate::ids::{test_malo, test_mp_id};

/// Generate a UTILMD Anmeldung message (GPKE 1.1.1: LFN -> NB)
pub fn anmeldung(lieferbeginn: NaiveDate) -> Nachricht {
	let lfn = test_mp_id(1);
	let nb = test_mp_id(2);
	Nachricht {
		absender: lfn.clone(),
		absender_rolle: MarktRolle::LieferantNeu,
		empfaenger: nb,
		empfaenger_rolle: MarktRolle::Netzbetreiber,
		payload: NachrichtenPayload::UtilmdAnmeldung(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: lfn,
			lieferbeginn,
		}),
	}
}

/// Generate a UTILMD Bestaetigung message (GPKE 1.1.2: NB -> LFN)
pub fn bestaetigung(empfaenger: MarktpartnerId, lieferbeginn: NaiveDate) -> Nachricht {
	let nb = test_mp_id(2);
	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: empfaenger.clone(),
		empfaenger_rolle: MarktRolle::LieferantNeu,
		payload: NachrichtenPayload::UtilmdBestaetigung(UtilmdBestaetigung {
			malo_id: test_malo(0),
			bestaetigt_fuer: empfaenger,
			lieferbeginn,
		}),
	}
}

/// Generate a UTILMD Abmeldung message (GPKE 1.1.3: NB -> LFA)
pub fn abmeldung(lfa: MarktpartnerId, lieferende: NaiveDate) -> Nachricht {
	let nb = test_mp_id(2);
	Nachricht {
		absender: nb,
		absender_rolle: MarktRolle::Netzbetreiber,
		empfaenger: lfa.clone(),
		empfaenger_rolle: MarktRolle::LieferantAlt,
		payload: NachrichtenPayload::UtilmdAbmeldung(UtilmdAbmeldung {
			malo_id: test_malo(0),
			lieferant_alt: lfa,
			lieferende,
		}),
	}
}
```

- [ ] **Step 3: Write quittung generators**

`mako-testdata/src/quittungen.rs`:
```rust
use mako_types::ids::MarktpartnerId;

/// CONTRL/APERAK pair types are in mako-quittung.
/// This module provides convenience constructors for test scenarios.
/// Since mako-testdata doesn't depend on mako-quittung,
/// we provide the raw data that test code can use to construct Quittung instances.

/// Test receipt data (portable, no mako-quittung dependency)
#[derive(Debug, Clone)]
pub struct TestQuittungData {
	pub an: MarktpartnerId,
	pub ist_contrl: bool,
	pub ist_positiv: bool,
	pub fehler_code: Option<String>,
	pub fehler_text: Option<String>,
}

pub fn contrl_positiv(an: MarktpartnerId) -> TestQuittungData {
	TestQuittungData {
		an,
		ist_contrl: true,
		ist_positiv: true,
		fehler_code: None,
		fehler_text: None,
	}
}

pub fn contrl_negativ(an: MarktpartnerId, code: &str, text: &str) -> TestQuittungData {
	TestQuittungData {
		an,
		ist_contrl: true,
		ist_positiv: false,
		fehler_code: Some(code.to_string()),
		fehler_text: Some(text.to_string()),
	}
}

pub fn aperak_positiv(an: MarktpartnerId) -> TestQuittungData {
	TestQuittungData {
		an,
		ist_contrl: false,
		ist_positiv: true,
		fehler_code: None,
		fehler_text: None,
	}
}

pub fn aperak_negativ(an: MarktpartnerId, code: &str, text: &str) -> TestQuittungData {
	TestQuittungData {
		an,
		ist_contrl: false,
		ist_positiv: false,
		fehler_code: Some(code.to_string()),
		fehler_text: Some(text.to_string()),
	}
}
```

- [ ] **Step 4: Write scenario fixtures**

`mako-testdata/src/szenarien.rs`:
```rust
use chrono::NaiveDate;

use mako_gpke::v2025::lfw::{LfwEvent, LfwState};
use mako_types::gpke_nachrichten::*;

use crate::ids::{test_malo, test_mp_id};

/// Complete GPKE LFW v2025 happy path scenario.
/// Returns a vec of (event, expected_state_variant) tuples.
pub fn gpke_lfw_happy_path() -> Vec<LfwEvent> {
	let lieferbeginn = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

	vec![
		LfwEvent::AnmeldungEmpfangen(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: test_mp_id(1),
			lieferbeginn,
		}),
		LfwEvent::AnmeldungBestaetigt {
			lfa: test_mp_id(3),
		},
		LfwEvent::LfaHatBestaetigt,
		LfwEvent::WiderspruchsfristAbgelaufen,
	]
}

/// GPKE LFW v2025 rejection scenario (LFA rejects).
pub fn gpke_lfw_ablehnung() -> Vec<LfwEvent> {
	let lieferbeginn = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

	vec![
		LfwEvent::AnmeldungEmpfangen(UtilmdAnmeldung {
			malo_id: test_malo(0),
			lieferant_neu: test_mp_id(1),
			lieferbeginn,
		}),
		LfwEvent::AnmeldungBestaetigt {
			lfa: test_mp_id(3),
		},
		LfwEvent::LfaHatAbgelehnt {
			grund: AblehnungsGrund::KeinVertrag,
		},
	]
}

/// Run a scenario through the reducer, returning all intermediate states.
pub fn run_scenario(events: Vec<LfwEvent>) -> Vec<LfwState> {
	use mako_gpke::v2025::lfw::reduce;

	let mut states = vec![LfwState::Idle];
	let mut current = LfwState::Idle;

	for event in events {
		let output = reduce(current, event).expect("scenario step must not fail");
		current = output.state;
		states.push(current.clone());
	}

	states
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn happy_path_ends_in_zugeordnet() {
		let states = run_scenario(gpke_lfw_happy_path());
		assert!(matches!(states.last().unwrap(), LfwState::Zugeordnet { .. }));
	}

	#[test]
	fn ablehnung_ends_in_abgelehnt() {
		let states = run_scenario(gpke_lfw_ablehnung());
		assert!(matches!(states.last().unwrap(), LfwState::Abgelehnt { .. }));
	}

	#[test]
	fn happy_path_has_five_states() {
		// Idle -> Eingegangen -> AbmeldungGesendet -> Widerspruchsfrist -> Zugeordnet
		let states = run_scenario(gpke_lfw_happy_path());
		assert_eq!(states.len(), 5);
	}
}
```

- [ ] **Step 5: Update mako-testdata Cargo.toml and lib.rs**

`mako-testdata/Cargo.toml` (replace entire file):
```toml
[package]
name = "mako-testdata"
version = "0.1.0"
edition.workspace = true

[dependencies]
chrono = { workspace = true }
mako-types = { path = "../mako-types" }
mako-fristen = { path = "../mako-fristen" }
mako-gpke = { path = "../mako-gpke" }
```

`mako-testdata/src/lib.rs` (replace — now all modules have their dependencies):
```rust
pub mod ids;
pub mod utilmd;
pub mod quittungen;
pub mod szenarien;
```

- [ ] **Step 6: Run all tests**

Run: `cargo test --workspace`
Expected: ALL tests across all 5 crates pass

- [ ] **Step 7: Verify WASM target compiles for pure crates**

Run: `rustup target add wasm32-unknown-unknown && cargo build --target wasm32-unknown-unknown -p mako-types -p mako-fristen -p mako-gpke`
Expected: compiles (these crates have no IO)

- [ ] **Step 8: Commit**

```bash
git add mako-testdata/
git commit -m "add test data generator: ID generators, UTILMD builders, scenario fixtures for GPKE LFW"
```

---

## Task 9: Final Integration Test and Cleanup

**Files:**
- Create: `mako-gpke/tests/integration.rs`

- [ ] **Step 1: Write integration test — full LFW with quittungsschicht**

`mako-gpke/tests/integration.rs`:
```rust
use chrono::NaiveDate;

use mako_gpke::v2025::lfw::{reduce, LfwEvent, LfwState};
use mako_quittung::decorator::mit_quittung;
use mako_quittung::types::{QuittungsErgebnis, QuittungsTyp};
use mako_testdata::ids::{test_malo, test_mp_id};
use mako_testdata::utilmd;
use mako_types::gpke_nachrichten::UtilmdAnmeldung;

#[test]
fn gpke_lfw_v2025_full_integration_with_quittungen() {
	let stichtag = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
	let lieferbeginn = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

	// Step 1: LFN sends Anmeldung via decorated reducer
	let msg = utilmd::anmeldung(lieferbeginn);
	let event = LfwEvent::AnmeldungEmpfangen(UtilmdAnmeldung {
		malo_id: test_malo(0),
		lieferant_neu: test_mp_id(1),
		lieferbeginn,
	});

	let result = mit_quittung(&msg, LfwState::Idle, event, stichtag, reduce);
	let output = result.unwrap();

	// Verify: state transitioned
	assert!(matches!(
		output.state,
		LfwState::AnmeldungEingegangen { .. }
	));

	// Verify: CONTRL + APERAK positiv
	assert_eq!(output.quittungen.len(), 2);
	assert_eq!(output.quittungen[0].typ, QuittungsTyp::Contrl);
	assert_eq!(output.quittungen[0].ergebnis, QuittungsErgebnis::Positiv);
	assert_eq!(output.quittungen[1].typ, QuittungsTyp::Aperak);
	assert_eq!(output.quittungen[1].ergebnis, QuittungsErgebnis::Positiv);

	// Verify: quittungen routed back to sender
	assert_eq!(output.quittungen[0].an, test_mp_id(1));
}

#[test]
fn gpke_lfw_v2025_aperak_rejects_past_lieferbeginn() {
	let stichtag = NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(); // after lieferbeginn
	let lieferbeginn = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

	let msg = utilmd::anmeldung(lieferbeginn);
	let event = LfwEvent::AnmeldungEmpfangen(UtilmdAnmeldung {
		malo_id: test_malo(0),
		lieferant_neu: test_mp_id(1),
		lieferbeginn,
	});

	let result = mit_quittung(&msg, LfwState::Idle, event, stichtag, reduce);
	let output = result.unwrap();

	// State should NOT have transitioned
	assert_eq!(output.state, LfwState::Idle);

	// CONTRL positiv, APERAK negativ
	assert_eq!(output.quittungen[0].ergebnis, QuittungsErgebnis::Positiv);
	assert!(matches!(
		output.quittungen[1].ergebnis,
		QuittungsErgebnis::Negativ(_)
	));
}
```

- [ ] **Step 2: Run full workspace tests**

Run: `cargo test --workspace`
Expected: ALL tests pass, 0 failures

- [ ] **Step 3: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: no warnings

- [ ] **Step 4: Commit**

```bash
git add mako-gpke/tests/
git commit -m "add integration test: GPKE LFW v2025 with quittungsschicht, happy path + APERAK rejection"
```

- [ ] **Step 5: Final verification commit**

Run: `cargo test --workspace && cargo clippy --workspace -- -D warnings`
If both pass, the foundation is complete.
