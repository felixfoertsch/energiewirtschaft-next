---
name: MaKo-Engine project context
description: Functional model of German energy market communication (Strom + Gas) in Rust — architecture decisions, current phase, key constraints
type: project
---

MaKo-Engine: beweisbar korrekte Referenzimplementierung der deutschen Marktkommunikation in Rust.

**Why:** Referenzimplementierung + produktives System. Pure functions → deployment-agnostisch (Cloudflare Edge, FaaS, WASM). Gesamte MaKo-Historie abbilden (v2017, v2020, v2022, v2025, zukünftige).

**How to apply:** Jede Kommunikationslinie ist ein `(State, Event) → Result<(State, Vec<Nachricht>), ProzessFehler>` Reducer. Quittungsschicht (CONTRL/APERAK) als generischer Decorator. Eigenes Rust-Typsystem, keine Dependencies auf BO4E/Python-Tooling. Hochfrequenz-Ökosystem als Wissensbasis.

**Current state (2026-03-29):**
- Foundation: 24 Crates (inkl. mako-verify), alle Prozess-Reducer (GPKE, WiM, MaBiS, etc.) als State Machines
- EDIFACT-Codec: vollständig (parse + serialize für alle 18 EDIFACT-Typen + 9 XML RD 2.0)
- Testkorpus: 59 Fixtures, 39 Generatoren, Fehler-Injektor, 15 Kommunikationsketten
- Rust-CLI (`mako-cli`): init, verarbeite, sende, status, verarbeite-alle, verifiziere, verifiziere-batch
- **641 Tests**, zero warnings

**MaKo-Simulator (2026-03-25):**
- Rust-CLI (`mako-cli`): init, verarbeite, sende, status, verarbeite-alle
- React Web-UI (`mako-ui`): Bun + Vite + React 19 + Tailwind v4 + shadcn/ui + Biome
  - Express Backend (Port 3001) + Vite Frontend (Port 5173)
  - 12 Komponenten: RollenTabs, MessageList, MessageDetail, StatusBadge, ProzessListe, AufgabenQueue, MessageForm, ProcessTimeline, VerifikationsBadge, VerifikationsPanel, EbdBaum, BatchBericht
  - 17 Prozessdefinitionen, 6 Rollen

**Verifikationssystem (2026-03-29):**
- `mako-verify` Crate: dreischichtige Qualitätssicherung mit externen Referenzdaten
  - Schicht 1 (AHB): Segment-Präsenz-Validierung gegen offizielle Flat-AHB-JSONs (474 Dateien, FV2504)
  - Schicht 2 (EBD): Ergebnisorientierter Vergleich gegen Entscheidungsbaumdiagramme (345 JSONs, FV2604)
  - Schicht 3 (Interop): Schlüsselfeld-Extraktion für Kreuzvalidierung, STROMDAO edifact-json-transformer als Node.js-Sidecar
- Referenzdaten: Hochfrequenz `machine-readable_anwendungshandbuecher` (AHB) + `machine-readable_entscheidungsbaumdiagramme` (EBD) im Repo unter `mako-verify/referenzdaten/`
- AHB-Bedingungsausdrücke: Parser für ∧/∨/⊻/UB-Operatoren + Dreiwert-Logik-Auswertung
- CLI: `mako verifiziere <datei>` + `mako verifiziere-batch <verzeichnis>` → JSON-Ausgabe
- UI: Verifizieren-Button pro Nachricht, VerifikationsPanel (AHB/EBD/Codec-Tabs), BatchBericht-Modal mit "Simulation verifizieren"-Button
- V1-Einschränkungen: AHB prüft Segment-Präsenz (nicht Feldwerte), EBD vergleicht Endergebnis (nicht jeden Schritt), Bedingungen mit externem Zustand als "unbestimmt" markiert

**Full Market Simulation "Netzgebiet Rheinland Q3 2026" (2026-03-28):**
- `mako-sim/src/bin/simulate.rs`: 92-day simulation, 7 named Teilnehmer, 10,526 EDIFACT messages
- Scenario: normal operations → Rheinstrom AG Insolvenz (Aug 15) → recovery + iMSys push
- Every message through real mako-codec parse/serialize, mako-quittung CONTRL/APERAK, mako-sim Markt routing
- Results: 10,502/10,526 parse OK (24 intentional error injections), 10,495 APERAK OK, full roundtrip
- All 10,525 raw EDIFACT files saved to `mako-sim/simulation/nachrichten/<kette>/<id>.edi` (56 MB)
- `mako-sim/simulation/simulation_log.json` (7.6 MB): full log with timestamps, narratives
- `build_report.py` + `mako-sim/simulation/report_template.html` → `simulation_report.html`
  - Light/dark mode (system preference + toggle, persisted to localStorage)
  - 7 tabs: Chronik, Zeitstrahl, Teilnehmer, Nachrichtenfluss, Nachrichten (searchable table with raw EDIFACT), Wochenberichte, Statistik
  - 45 EDIFACT messages embedded inline for sequence diagram chains
- `mako-sim/simulation/` is gitignored (generated output)
- Run: `cargo run -p mako-sim --bin mako-simulate` then `python3 build_report.py`

**Known limitations / next steps:**
- CLI dispatch (`mako-cli/src/verarbeite.rs`) only wires up `gpke_lfw` reducer — other processes mapped in event_mapping but not dispatched
- MSCONS parser doesn't handle DTM format 303 (timezone offset) — DTM+137 in MSCONS messages uses workaround (valid hours only)
- `test_malo(idx)` takes u8 → max 255 MaLos
- mako-ui not yet connected to simulation data
- Simulation is deterministic (seed 20260701) and reproducible
- Verifikation V2: Feldwert-Vergleich (Data-Element-Mapping), EBD-Schrittweise Auswertung, ahbicht-Integration

**Key resources:**
- BDEW MaKo-Dokumente: `docs/mako_aktuell_kostenfrei_25_03_2026/` (FV2504, 206 Dateien)
- Hochfrequenz machine-readable AHBs: `mako-verify/referenzdaten/ahb/FV2504/` (474 JSONs)
- Hochfrequenz machine-readable EBDs: `mako-verify/referenzdaten/ebd/FV2604/` (345 JSONs)
