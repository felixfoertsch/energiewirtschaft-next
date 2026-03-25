# MaKo-Simulator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a file-based MaKo simulator with a Rust CLI (parse → quittung → reducer → outbox), a React/shadcn Web-UI for manually operating all market roles, and a German-language system guide.

**Architecture:** The Rust CLI (`mako-cli`) processes EDIFACT/JSON files from `markt/{rolle}/inbox/` through the existing codec + quittung + reducer stack, writing results to `outbox/`. A React app (`mako-ui`) watches the filesystem and provides forms, inbox/outbox browsers, process timelines, and role-switching. The filesystem IS the transport layer.

**Tech Stack:** Rust (mako-cli: clap, existing mako-* crates), Node.js (mako-ui: React, Vite, Tailwind, shadcn/ui, Express, chokidar). EDIFACT per BDEW MIG FV2504.

**Spec:** `docs/superpowers/specs/2026-03-25-mako-simulator-design.md`

**Existing Code:**
- 566 tests passing, full EDIFACT codec (dispatch.rs: parse + serialize for all types)
- `mako-quittung`: CONTRL/APERAK decorator (`mit_quittung`)
- `mako-gpke/v2025`: LFW, Lieferende, Stammdaten, Zuordnung, GDA reducers (partially generate messages)
- `mako-testdata`: 59 fixtures, 39 generators, error injector, 15 communication chains
- `mako-sim`: agent-based simulation (separate from CLI, coexists)

---

## File Structure

### New: `mako-cli/` (Rust crate)

```
mako-cli/
├── Cargo.toml
└── src/
	├── main.rs                 # clap CLI entry point
	├── init.rs                 # `init` command: create markt/ directory structure
	├── verarbeite.rs           # `verarbeite` command: parse → CONTRL → APERAK → reducer → outbox
	├── sende.rs                # `sende` command: copy file from outbox to inbox
	├── status.rs               # `status` command: show overview of all roles/processes
	├── event_mapping.rs        # Nachricht → prozessspezifisches Event mapping
	└── state_store.rs          # Read/write state.json per role
```

### New: `mako-ui/` (Node/React app, separate project root)

```
mako-ui/
├── package.json
├── vite.config.ts
├── tailwind.config.ts
├── tsconfig.json
├── src/
│   ├── App.tsx                 # Main layout: tabs + 3-column grid
│   ├── main.tsx                # Entry point
│   ├── components/
│   │   ├── RollenTabs.tsx      # Tab bar with role names + badge counts
│   │   ├── AufgabenQueue.tsx   # Open tasks with role-switch links
│   │   ├── ProzessListe.tsx    # Communication line navigation (left sidebar)
│   │   ├── MessageList.tsx     # Inbox/Outbox message cards with status checkmarks
│   │   ├── MessageForm.tsx     # Form to create new messages (fields per process step)
│   │   ├── EdifactPreview.tsx  # Live EDIFACT preview (monospace, collapsible)
│   │   ├── ProcessTimeline.tsx # Step-by-step timeline bar (bottom)
│   │   └── StatusBadge.tsx     # ✓ ✓✓ ✓CONTRL ✓APERAK ✓✓✓ component
│   ├── lib/
│   │   ├── types.ts            # TypeScript types (Nachricht, Rolle, Status, Prozess)
│   │   ├── api.ts              # HTTP client for Express backend
│   │   ├── prozesse.ts         # Process definitions (steps, roles, message types per line)
│   │   └── rollen.ts           # Role definitions (name, mp_id, color)
│   └── server/
│       └── index.ts            # Express server: fs access + mako-cli invocation + chokidar
├── public/
│   └── index.html
└── components.json             # shadcn config
```

### New: `docs/anleitung.md`

German-language system guide (Markdown, ~2000 words).

### Modified: `mako-gpke/src/v2025/*.rs`

Fill in missing `nachrichten: vec![]` where a reducer transition should produce outgoing messages.

### Modified: root `Cargo.toml`

Add `"mako-cli"` to workspace members.

---

## Task 1: Bedienungsanleitung (Sektionen 1–6)

**Files:**
- Create: `docs/anleitung.md`

This is a documentation task — no code, no tests. The guide explains the existing system.

- [ ] **Step 1: Write the Anleitung**

`docs/anleitung.md` — auf Deutsch, Gliederung:

1. **Projektübersicht** — MaKo-Engine, Ziel (beweisbar korrekte Referenzimplementierung), warum Rust, warum funktional
2. **Architektur** — Schichtenmodell (Prozess → Quittung → Codec → Transport), Crate-Struktur (18 Crates), Dependency-Graph
3. **Typsystem** — Marktrollen (enum), IDs (MaLo/MeLo/MP-ID mit Validierung), Nachrichten (NachrichtenPayload, 46 Varianten), Reducer-Trait
4. **Reducer-Konzept** — `(State, Event) → Result<(State, Vec<Nachricht>), ProzessFehler>`, Quittungsschicht als Decorator (CONTRL → APERAK → Reducer), Zustandsautomaten pro Kommunikationslinie
5. **EDIFACT-Codec** — Lexer (Trennzeichen, Escape), Parser (Dispatch nach UNH + BGM + PID), Serializer, Roundtrip-Tests, XML für RD 2.0
6. **Testkorpus** — 59 Fixtures (EDIFACT `&str` + erwarteter Struct), 39 Generatoren (Params + Default), Fehler-Injektor (FehlerArt enum), 15 Kommunikationsketten

Reference the actual crate/file names. Include a `cargo test --workspace` command as "So prüft man ob alles funktioniert".

Sektionen 7–9 (Simulator, Prozess-Referenz, Glossar) kommen in Task 10 nach Phase C.

- [ ] **Step 2: Verify it renders correctly**

Open in a Markdown viewer, check all headings, code blocks, tables render.

- [ ] **Step 3: Commit**

```
git add docs/anleitung.md
git commit -m "add Bedienungsanleitung (Sektionen 1–6): Architektur, Typsystem, Codec, Testkorpus"
```

---

## Task 2: GPKE Reducer Message-Generation

**Files:**
- Modify: `mako-gpke/src/v2025/lfw.rs`
- Modify: `mako-gpke/src/v2025/lieferende.rs`
- Modify: `mako-gpke/src/v2025/stammdaten.rs`
- Modify: `mako-gpke/src/v2025/zuordnung.rs`
- Modify: `mako-gpke/src/v2025/gda.rs`

The GPKE LFW reducer already generates messages on steps 2 and 5. Other reducers and other LFW steps that should produce messages still return `nachrichten: vec![]`.

- [ ] **Step 1: Audit all GPKE reducers for missing message generation**

For each reducer, check every `nachrichten: vec![]` and determine if that transition should produce an outgoing message per the GPKE process description. Key gaps:

**LFW:**
- Step 1 (Idle → AnmeldungEingegangen): No outgoing message needed (NB receives, doesn't reply yet). ✓ correct.
- Step 3 (AbmeldungAnLfaGesendet → WiderspruchsfristLaeuft): LFA has confirmed, no outgoing message. ✓ correct.
- Step 4 (WiderspruchsfristLaeuft → Zugeordnet): already generates 2 Zuordnungen. ✓ correct.
- Step 5 (LfaHatAbgelehnt → Abgelehnt): should generate UtilmdAblehnung back to LFN.
- Step 6 (FristUeberschritten → Abgelehnt): should generate timeout notification.

**Lieferende:**
- Check each transition: AbmeldungEmpfangen → BestaetugungGesendet should generate UtilmdLieferendeBestaetigung.
- ZaehlerstandEmpfangen should acknowledge.

**Stammdaten:**
- AenderungGesendet → should generate confirmation/rejection message.

**GDA:**
- AnfrageEmpfangen → should generate UtilmdGeschaeftsdatenantwort.

- [ ] **Step 2: Implement missing message generation**

For each gap: add the correct `Nachricht` with proper `absender`, `empfaenger`, `absender_rolle`, `empfaenger_rolle`, `payload`. Use the existing patterns from LFW step 2 as template.

- [ ] **Step 3: Write/update tests for message generation**

For each changed transition: verify the `output.nachrichten` vec is non-empty and contains the correct payload type.

- [ ] **Step 4: Run tests**

Run: `cargo test --workspace`
Expected: all pass (existing + new)

- [ ] **Step 5: Commit**

```
git add mako-gpke/
git commit -m "complete GPKE reducer message generation (LFW, Lieferende, Stammdaten, GDA)"
```

---

## Task 3: mako-cli Scaffold + init Command

**Files:**
- Create: `mako-cli/Cargo.toml`
- Create: `mako-cli/src/main.rs`
- Create: `mako-cli/src/init.rs`
- Modify: `Cargo.toml` (workspace root — add `"mako-cli"` to members)

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "mako-cli"
version = "0.1.0"
edition.workspace = true

[dependencies]
chrono = { workspace = true }
clap = { version = "4", features = ["derive"] }
serde = { workspace = true }
serde_json = { workspace = true }
mako-types = { path = "../mako-types" }
mako-codec = { path = "../mako-codec" }
mako-quittung = { path = "../mako-quittung" }
mako-fristen = { path = "../mako-fristen" }
mako-gpke = { path = "../mako-gpke" }
mako-wim = { path = "../mako-wim" }
mako-ubp = { path = "../mako-ubp" }
mako-mabis = { path = "../mako-mabis" }
mako-abrechnung = { path = "../mako-abrechnung" }
mako-mpes = { path = "../mako-mpes" }
mako-rd2 = { path = "../mako-rd2" }
mako-14a = { path = "../mako-14a" }
mako-geli = { path = "../mako-geli" }
mako-gabi = { path = "../mako-gabi" }
mako-kov = { path = "../mako-kov" }
mako-gasumrechnung = { path = "../mako-gasumrechnung" }
```

- [ ] **Step 2: Create main.rs with clap commands**

```rust
use clap::{Parser, Subcommand};

mod init;

#[derive(Parser)]
#[command(name = "mako-cli", about = "MaKo-Simulator CLI")]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Erstellt die Markt-Ordnerstruktur
	Init {
		/// Pfad zum Markt-Verzeichnis
		#[arg(default_value = "markt")]
		pfad: String,
	},
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::Init { pfad } => init::run(&pfad),
	}
}
```

- [ ] **Step 3: Implement init command**

`mako-cli/src/init.rs`:
```rust
use std::fs;
use std::path::Path;

const ROLLEN: &[&str] = &[
	"lieferant_neu",
	"netzbetreiber",
	"lieferant_alt",
	"messstellenbetreiber",
	"bilanzkreisverantwortlicher",
	"marktgebietsverantwortlicher",
];

pub fn run(pfad: &str) {
	let base = Path::new(pfad);
	for rolle in ROLLEN {
		let inbox = base.join(rolle).join("inbox");
		let outbox = base.join(rolle).join("outbox");
		fs::create_dir_all(&inbox).expect("create inbox");
		fs::create_dir_all(&outbox).expect("create outbox");

		// Initialize empty state.json
		let state_path = base.join(rolle).join("state.json");
		if !state_path.exists() {
			fs::write(&state_path, "{}").expect("write state.json");
		}
	}

	// Create log directory
	fs::create_dir_all(base.join("log")).expect("create log dir");

	println!("Markt initialisiert in {pfad}/");
	for rolle in ROLLEN {
		println!("  {rolle}/inbox/ {rolle}/outbox/ {rolle}/state.json");
	}
	println!("  log/");
}
```

- [ ] **Step 4: Add mako-cli to workspace members**

Add `"mako-cli"` to the `members` array in root `Cargo.toml`.

- [ ] **Step 5: Write unit test for init**

```rust
#[cfg(test)]
mod tests {
	use super::*;
	use std::path::PathBuf;

	#[test]
	fn init_creates_all_directories() {
		let dir = std::env::temp_dir().join("mako-cli-test-init");
		let _ = std::fs::remove_dir_all(&dir);
		run(dir.to_str().unwrap());

		for rolle in ROLLEN {
			assert!(dir.join(rolle).join("inbox").is_dir());
			assert!(dir.join(rolle).join("outbox").is_dir());
			assert!(dir.join(rolle).join("state.json").exists());
		}
		assert!(dir.join("log").is_dir());

		std::fs::remove_dir_all(&dir).ok();
	}
}
```

- [ ] **Step 6: Verify it compiles and runs**

```bash
cargo build -p mako-cli
cargo test -p mako-cli
cargo run -p mako-cli -- init /tmp/test-markt
ls -la /tmp/test-markt/
```
Expected: 6 role directories with inbox/outbox/state.json + log/

- [ ] **Step 7: Commit**

```
git add mako-cli/ Cargo.toml
git commit -m "add mako-cli crate with init command, create markt/ directory structure"
```

---

## Task 4: Event Mapping + State Store

**Files:**
- Create: `mako-cli/src/event_mapping.rs`
- Create: `mako-cli/src/state_store.rs`

- [ ] **Step 1: Implement Nachricht → Event mapping**

`mako-cli/src/event_mapping.rs`:

Map each `NachrichtenPayload` variant to the corresponding process event. Return a tuple of `(process_key, boxed_reducer_call)` where process_key is `"{prozess}/{malo_id}"`.

```rust
use mako_types::nachricht::{Nachricht, NachrichtenPayload};
use mako_types::gpke_nachrichten::*;

/// Identifies which process and key a message belongs to.
pub struct ProzessZuordnung {
	pub prozess: String,       // e.g. "gpke_lfw"
	pub key: String,           // e.g. "gpke_lfw/51238696700"
	pub beschreibung: String,  // e.g. "GPKE Lieferantenwechsel"
}

pub fn zuordne_prozess(nachricht: &Nachricht) -> Option<ProzessZuordnung> {
	match &nachricht.payload {
		NachrichtenPayload::UtilmdAnmeldung(a) => Some(ProzessZuordnung {
			prozess: "gpke_lfw".into(),
			key: format!("gpke_lfw/{}", a.malo_id.as_str()),
			beschreibung: "GPKE Lieferantenwechsel".into(),
		}),
		NachrichtenPayload::UtilmdBestaetigung(b) => Some(ProzessZuordnung {
			prozess: "gpke_lfw".into(),
			key: format!("gpke_lfw/{}", b.malo_id.as_str()),
			beschreibung: "GPKE Lieferantenwechsel".into(),
		}),
		// ... map all GPKE payload variants
		// ... map WiM, UBP, MaBiS, etc.
		_ => None,
	}
}
```

- [ ] **Step 2: Implement state store**

`mako-cli/src/state_store.rs`:

Read/write `state.json` per role. Uses serde_json with a HashMap of process keys → serialized states.

```rust
use std::collections::HashMap;
use std::path::Path;

pub type StateMap = HashMap<String, serde_json::Value>;

pub fn load_state(rolle_dir: &Path) -> StateMap {
	let path = rolle_dir.join("state.json");
	if path.exists() {
		let content = std::fs::read_to_string(&path).expect("read state.json");
		serde_json::from_str(&content).unwrap_or_default()
	} else {
		HashMap::new()
	}
}

pub fn save_state(rolle_dir: &Path, state: &StateMap) {
	let path = rolle_dir.join("state.json");
	let content = serde_json::to_string_pretty(state).expect("serialize state");
	std::fs::write(path, content).expect("write state.json");
}
```

- [ ] **Step 3: Write tests for event mapping**

Test that each GPKE message type maps to the correct process key.

- [ ] **Step 4: Commit**

```
git add mako-cli/src/
git commit -m "add event mapping (Nachricht → process key) and state store (state.json read/write)"
```

---

## Task 5: `verarbeite` Command

**Files:**
- Create: `mako-cli/src/verarbeite.rs`
- Modify: `mako-cli/src/main.rs`

The core command: reads a file from inbox, parses it, runs CONTRL → APERAK → Reducer, writes results.

- [ ] **Step 1: Add Verarbeite command to clap**

Add to `Commands` enum:
```rust
/// Verarbeitet eine Nachricht (parse → CONTRL → APERAK → Reducer → outbox)
Verarbeite {
	/// Pfad zur Nachrichtendatei
	datei: String,
	/// Pfad zum Markt-Verzeichnis
	#[arg(long, default_value = "markt")]
	markt: String,
},
```

- [ ] **Step 2: Implement verarbeite command**

`mako-cli/src/verarbeite.rs`:

```rust
pub fn run(datei: &str, markt: &str) -> Result<(), Box<dyn std::error::Error>> {
	// 1. Read file (detect EDIFACT vs JSON by extension)
	let content = std::fs::read_to_string(datei)?;
	let nachricht = if datei.ends_with(".json") {
		serde_json::from_str(&content)?
	} else {
		mako_codec::edifact::dispatch::parse_nachricht(&content)?
	};

	// 2. Determine sender/receiver role directories
	let empfaenger_dir = find_rolle_dir(markt, &nachricht.empfaenger)?;
	let absender_dir = find_rolle_dir(markt, &nachricht.absender)?;

	// 3. CONTRL check
	let contrl = mako_quittung::contrl::contrl_pruefen(&nachricht);
	write_quittung(&absender_dir, "contrl", &contrl, &nachricht)?;

	// 4. If CONTRL negative, stop
	if matches!(contrl, QuittungsErgebnis::Negativ(_)) {
		update_status(datei, "contrl_negativ")?;
		return Ok(());
	}

	// 5. APERAK check
	let stichtag = chrono::Local::now().date_naive();
	let aperak = mako_quittung::aperak::aperak_pruefen(&nachricht, stichtag);
	write_quittung(&absender_dir, "aperak", &aperak, &nachricht)?;

	// 6. If APERAK negative, stop
	if matches!(aperak, QuittungsErgebnis::Negativ(_)) {
		update_status(datei, "aperak_negativ")?;
		return Ok(());
	}

	// 7. Map to process, load state, run reducer
	let zuordnung = event_mapping::zuordne_prozess(&nachricht)
		.ok_or("unknown process for this message type")?;

	let mut states = state_store::load_state(&empfaenger_dir);
	let (new_state, outgoing) = dispatch_reducer(&zuordnung, &states, &nachricht)?;
	states.insert(zuordnung.key.clone(), new_state);
	state_store::save_state(&empfaenger_dir, &states);

	// 8. Write outgoing messages to outbox + log
	for (i, msg) in outgoing.iter().enumerate() {
		let filename = format!("{:03}_{}.edi", next_seq(&empfaenger_dir.join("outbox")), payload_name(&msg.payload));
		let edi = mako_codec::edifact::dispatch::serialize_nachricht(msg);
		std::fs::write(empfaenger_dir.join("outbox").join(&filename), &edi)?;
		let json = serde_json::to_string_pretty(msg)?;
		std::fs::write(empfaenger_dir.join("outbox").join(filename.replace(".edi", ".json")), &json)?;
		log_nachricht(&base, msg, &filename)?;
	}

	// 9. Update .status.json
	update_status(datei, "verarbeitet")?;
	println!("Verarbeitet: {} → {} ausgehende Nachrichten", zuordnung.beschreibung, outgoing.len());
	Ok(())
}

/// Dispatch to the correct reducer based on process key.
fn dispatch_reducer(
	zuordnung: &event_mapping::ProzessZuordnung,
	states: &state_store::StateMap,
	nachricht: &Nachricht,
) -> Result<(serde_json::Value, Vec<Nachricht>), Box<dyn std::error::Error>> {
	match zuordnung.prozess.as_str() {
		"gpke_lfw" => {
			let state: mako_gpke::v2025::lfw::LfwState = states
				.get(&zuordnung.key)
				.map(|v| serde_json::from_value(v.clone()).unwrap_or(mako_gpke::v2025::lfw::LfwState::Idle))
				.unwrap_or(mako_gpke::v2025::lfw::LfwState::Idle);
			let event = event_mapping::to_lfw_event(nachricht)?;
			let output = mako_gpke::v2025::lfw::reduce(state, event)?;
			Ok((serde_json::to_value(&output.state)?, output.nachrichten))
		}
		"gpke_lieferende" => {
			// Same pattern: deserialize state, map event, reduce, serialize
			todo!("implement per-process dispatch")
		}
		// ... one arm per process type
		other => Err(format!("unknown process: {other}").into()),
	}
}
```

- [ ] **Step 3: Implement helper functions**

`find_rolle_dir` — search `markt/` for a directory whose `state.json` contains the given MP-ID, or match by directory naming convention.

`write_quittung` — serialize CONTRL/APERAK as EDIFACT, write to `absender_dir/inbox/`.

`update_status` — write/update `.status.json` beside the original file.

- [ ] **Step 4: Test end-to-end with a GPKE Anmeldung**

```bash
cargo run -p mako-cli -- init /tmp/test-markt

# Create a test message using the generator
# (or manually write an EDIFACT file)
echo 'UNB+UNOC:3+...' > /tmp/test-markt/netzbetreiber/inbox/001_anmeldung.edi

cargo run -p mako-cli -- verarbeite /tmp/test-markt/netzbetreiber/inbox/001_anmeldung.edi --markt /tmp/test-markt

# Check results
ls /tmp/test-markt/netzbetreiber/outbox/
ls /tmp/test-markt/lieferant_neu/inbox/  # should have CONTRL + APERAK
cat /tmp/test-markt/netzbetreiber/inbox/001_anmeldung.status.json
```

- [ ] **Step 5: Commit**

```
git add mako-cli/
git commit -m "add verarbeite command: parse → CONTRL → APERAK → reducer → outbox"
```

---

## Task 6: `sende` + `status` Commands

**Files:**
- Create: `mako-cli/src/sende.rs`
- Create: `mako-cli/src/status.rs`
- Modify: `mako-cli/src/main.rs`

- [ ] **Step 1: Implement sende command**

Copies file from `absender/outbox/{datei}` to `empfaenger/inbox/{datei}`, creates `.status.json` with `zugestellt` timestamp.

- [ ] **Step 2: Implement status command**

Reads all `state.json` files, counts inbox/outbox messages per role, prints overview:

```
Markt-Status (/tmp/test-markt):
  lieferant_neu     inbox: 2  outbox: 1  prozesse: gpke_lfw/51238 (AnmeldungEingegangen)
  netzbetreiber     inbox: 1  outbox: 2  prozesse: gpke_lfw/51238 (AbmeldungGesendet)
  lieferant_alt     inbox: 1  outbox: 0  prozesse: -
```

- [ ] **Step 3: Implement verarbeite-alle command**

Iterates over all `.edi` and `.json` files in `rolle/inbox/` that have no `.status.json` or no `verarbeitet` status. Calls `verarbeite::run()` for each.

- [ ] **Step 4: Implement log writing**

`log_nachricht(markt_dir, nachricht, filename)` — appends a JSONL line to `markt/log/YYYY-MM-DD.jsonl`:
```json
{"zeitpunkt":"2026-03-25T12:30:03","von":"lieferant_neu","an":"netzbetreiber","typ":"UtilmdAnmeldung","datei":"001_anmeldung.edi"}
```

Call from both `sende` (on delivery) and `verarbeite` (on processing + outgoing).

- [ ] **Step 5: Implement .status.json write logic**

`update_status(datei, status)` — creates/updates `{datei}.status.json`:
```json
{"erstellt":"...","zugestellt":"...","contrl":{"ergebnis":"positiv","zeitpunkt":"..."},"aperak":{"ergebnis":"positiv","zeitpunkt":"..."},"verarbeitet":"..."}
```

Each status field is added incrementally (existing fields preserved). `sende` sets `zugestellt`, `verarbeite` sets `contrl`/`aperak`/`verarbeitet`.

- [ ] **Step 6: Add commands to main.rs**

Add `Sende`, `Status`, `VerarbeiteAlle` to the `Commands` enum.

- [ ] **Step 7: Write unit tests**

- `test_init_creates_directories` — init in temp dir, verify all role dirs + inbox/outbox/state.json exist
- `test_sende_copies_file` — create file in outbox, sende, verify copy in inbox + .status.json with `zugestellt`
- `test_status_shows_overview` — init + place files, verify output contains role names and counts
- `test_verarbeite_alle_processes_inbox` — place 2 messages in inbox, verarbeite-alle, verify both have .status.json

- [ ] **Step 8: Run tests, commit**

```
git add mako-cli/
git commit -m "add sende, status, verarbeite-alle commands with log writing, .status.json, unit tests"
```

---

## Task 7: mako-ui Scaffold (React + Vite + shadcn)

**Files:**
- Create: `mako-ui/` (entire project scaffold)

- [ ] **Step 1: Create Vite + React + TypeScript project**

```bash
cd /path/to/workspace
npm create vite@latest mako-ui -- --template react-ts
cd mako-ui
npm install
```

- [ ] **Step 2: Add Tailwind + shadcn**

```bash
npm install -D tailwindcss @tailwindcss/vite
npx shadcn@latest init
```

Configure `tailwind.config.ts` and `components.json` for shadcn.

- [ ] **Step 3: Add Express + chokidar backend**

```bash
npm install express chokidar cors
npm install -D @types/express @types/cors tsx
```

Create `src/server/index.ts` — Express server on port 3001:
- `GET /api/rollen` — list role directories
- `GET /api/rollen/:rolle/inbox` — list inbox files
- `GET /api/rollen/:rolle/outbox` — list outbox files
- `GET /api/rollen/:rolle/state` — read state.json
- `GET /api/nachrichten/:rolle/:box/:datei` — read message file + status
- `POST /api/nachrichten/:rolle` — create message (write to outbox)
- `POST /api/sende` — invoke `mako-cli sende`
- `POST /api/verarbeite` — invoke `mako-cli verarbeite`
- `POST /api/verarbeite-alle` — invoke `mako-cli verarbeite-alle`
- `GET /api/events` — SSE stream for file changes (chokidar)

The Express server needs to know the `mako-cli` binary path. Configure via environment variable `MAKO_CLI_PATH` with default `../target/debug/mako-cli` (relative to mako-ui/). Before starting the UI, build the CLI: `cargo build -p mako-cli`.

- [ ] **Step 4: Add TypeScript types**

`src/lib/types.ts`:
```typescript
export interface Rolle {
  name: string;
  mp_id: string;
  verzeichnis: string;
}

export interface NachrichtMeta {
  datei: string;
  typ: string;         // UTILMD, MSCONS, INVOIC, ...
  absender: string;
  empfaenger: string;
  zeitpunkt: string;
  status: NachrichtenStatus;
}

export interface NachrichtenStatus {
  erstellt?: string;
  zugestellt?: string;
  contrl?: { ergebnis: 'positiv' | 'negativ'; zeitpunkt: string };
  aperak?: { ergebnis: 'positiv' | 'negativ'; zeitpunkt: string };
  verarbeitet?: string;
}

export interface ProzessSchritt {
  name: string;
  absender_rolle: string;
  empfaenger_rolle: string;
  nachrichtentyp: string;
  status: 'done' | 'current' | 'pending';
}
```

- [ ] **Step 5: Create basic App.tsx with tab layout**

Placeholder: tabs for roles, empty content areas.

- [ ] **Step 6: Verify dev server starts**

```bash
# Terminal 1: Express backend
npx tsx src/server/index.ts

# Terminal 2: Vite frontend
npm run dev
```

Open http://localhost:5173 — should show empty tab layout.

- [ ] **Step 7: Commit**

```
git add mako-ui/
git commit -m "scaffold mako-ui: Vite + React + Tailwind + shadcn + Express backend"
```

---

## Task 8: UI Components — RollenTabs + MessageList + StatusBadge

**Files:**
- Create: `mako-ui/src/components/RollenTabs.tsx`
- Create: `mako-ui/src/components/MessageList.tsx`
- Create: `mako-ui/src/components/MessageDetail.tsx`
- Create: `mako-ui/src/components/StatusBadge.tsx`
- Modify: `mako-ui/src/App.tsx`

- [ ] **Step 1: RollenTabs** — shadcn Tabs component, one tab per role from `/api/rollen`. Badge with unread count. Active tab stored in URL/state.

- [ ] **Step 2: MessageList** — fetches inbox/outbox from Express API. Renders message cards with:
- Type badge (UTILMD, MSCONS, INVOIC...)
- Absender/Empfänger as clickable links (→ switches tab)
- Key fields (MaLo-ID, Datum, etc.)
- StatusBadge row

- [ ] **Step 3: MessageDetail** — click a message in MessageList → shows full JSON content + raw EDIFACT + .status.json timeline. Absender/Empfänger as clickable links for role-switching.

- [ ] **Step 4: StatusBadge** — renders the WhatsApp-style checkmarks:
- ✓ Erstellt (gray)
- ✓✓ Zugestellt (gray)
- ✓ CONTRL (green/red)
- ✓ APERAK (green/red)
- ✓✓✓ Verarbeitet (green)

- [ ] **Step 5: Wire into App.tsx** — 3-column layout, MessageList in center column

- [ ] **Step 6: Test with real data** — run `mako-cli init`, place a test EDIFACT file, verify UI shows it

- [ ] **Step 7: Commit**

```
git add mako-ui/
git commit -m "add RollenTabs, MessageList, MessageDetail, StatusBadge components"
```

---

## Task 9: UI Components — MessageForm + EdifactPreview + ProzessListe + AufgabenQueue + ProcessTimeline

**Files:**
- Create: `mako-ui/src/components/MessageForm.tsx`
- Create: `mako-ui/src/components/EdifactPreview.tsx`
- Create: `mako-ui/src/components/ProzessListe.tsx`
- Create: `mako-ui/src/components/AufgabenQueue.tsx`
- Create: `mako-ui/src/components/ProcessTimeline.tsx`
- Create: `mako-ui/src/lib/prozesse.ts`
- Create: `mako-ui/src/lib/rollen.ts`
- Modify: `mako-ui/src/App.tsx`

- [ ] **Step 1: prozesse.ts** — define all communication lines with their steps:
```typescript
export const PROZESSE = {
  gpke_lfw: {
    name: "GPKE Lieferantenwechsel",
    schritte: [
      { name: "Anmeldung", absender: "lieferant_neu", empfaenger: "netzbetreiber", typ: "UtilmdAnmeldung" },
      { name: "Bestätigung", absender: "netzbetreiber", empfaenger: "lieferant_neu", typ: "UtilmdBestaetigung" },
      // ... all 7 steps
    ]
  },
  // ... all 15 processes
};
```

- [ ] **Step 2: rollen.ts** — define roles with display names, MP-IDs, colors

- [ ] **Step 3: ProzessListe** — left sidebar, grouped by category (GPKE, WiM, MaBiS, Gas...), highlight active process

- [ ] **Step 4: AufgabenQueue** — above ProzessListe. Derives open tasks from state.json: if a process is in a state that requires manual action from a different role, show it as a task with a "→ Zu {rolle} wechseln" link.

- [ ] **Step 5: MessageForm** — right panel. Dropdown for process step (filtered by current role). Dynamic fields based on step's message type. "Senden" button calls `POST /api/nachrichten/:rolle` then `POST /api/sende`.

- [ ] **Step 6: EdifactPreview** — collapsible monospace panel below form. Calls Express API to generate EDIFACT from form fields (Express calls `mako-cli` or uses the codec directly).

- [ ] **Step 7: ProcessTimeline** — bottom bar. Shows steps of the selected process with done/current/pending indicators.

- [ ] **Step 8: Wire everything into App.tsx 3-column layout**

- [ ] **Step 9: Commit**

```
git add mako-ui/
git commit -m "add MessageForm, EdifactPreview, ProzessListe, AufgabenQueue, ProcessTimeline"
```

---

## Task 10: Integration + Anleitung vervollständigen

**Files:**
- Modify: `docs/anleitung.md` — add Sektionen 7–9
- Modify: `mako-ui/src/server/index.ts` — SSE file watcher

- [ ] **Step 1: Add chokidar SSE endpoint**

Express endpoint `GET /api/events` that streams file changes as Server-Sent Events. The React app subscribes and re-fetches inbox/outbox on changes.

- [ ] **Step 2: End-to-end test: GPKE LFW Happy Path**

1. `mako-cli init markt/`
2. Open UI at http://localhost:5173
3. Select tab "Lieferant Neu"
4. Select "GPKE Lieferantenwechsel" in ProzessListe
5. Fill MessageForm with Anmeldung fields
6. Click "Senden → NB"
7. Switch to "Netzbetreiber" tab (or click task in Queue)
8. Verify message in inbox with ✓✓ Zugestellt
9. Click "Verarbeiten" (or auto-process)
10. Verify CONTRL + APERAK in LFN inbox
11. Verify NB has Bestätigung + Abmeldung in outbox
12. Continue through all 7 steps

Document any issues, fix, re-test.

- [ ] **Step 3: Anleitung Sektionen 7–9**

Add to `docs/anleitung.md`:

7. **Simulator bedienen** — `mako-cli init`, `mako-cli verarbeite`, `mako-cli sende`, `mako-cli status`. Web-UI starten (npm run dev + Express). Screenshot-Beschreibung des UI.
8. **Prozess-Referenz** — jede Kommunikationslinie mit Tabelle: Schritt, Absender, Empfänger, Nachrichtentyp, automatisch/manuell. Mindestens GPKE LFW, Lieferende, Stammdaten.
9. **Glossar** — MaLo, MeLo, MP-ID, Sparte, EDIFACT, UTILMD, MSCONS, CONTRL, APERAK, Reducer, PID, AHB, MIG, EBD.

- [ ] **Step 4: Final commit**

```
git add docs/anleitung.md mako-ui/
git commit -m "add SSE file watcher, complete Anleitung (Sektionen 7–9), end-to-end tested"
```
