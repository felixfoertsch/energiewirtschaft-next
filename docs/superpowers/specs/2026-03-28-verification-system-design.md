# MaKo Verification System Design

## Problem

We built a functional model of German energy market communication (23 Rust crates, 580 tests, 10,526-message simulation). But we have no way for a domain expert (non-developer) to independently verify that our system produces correct EDIFACT messages, follows the right process decisions, and is interoperable with other systems.

## Solution: Hybrid Verification with External Reference Data

Build a verification system that uses official open-source reference data (AHB rules, EBD decision trees) as primary validator, and an independent third-party EDIFACT parser as cross-validator.

Three independent confirmations: our engine says X, the official rules say X is correct, and an independent parser agrees.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      mako-ui (React)                     │
│  Step-through mode  │  Batch report  │  Inline badges    │
└──────────────┬──────────────────────┬───────────────────┘
               │                      │
               ▼                      ▼
┌──────────────────────────┐  ┌───────────────────────────┐
│   OUR SYSTEM (existing)  │  │  VERIFICATION (new)       │
│                          │  │                           │
│  mako-cli                │  │  mako-verify (Rust)       │
│  mako-codec              │  │    Layer 1: AHB rules     │
│  mako-quittung           │  │    Layer 2: EBD trees     │
│  mako-gpke, mako-wim,   │  │    Layer 3: Codec interop │
│    ... (process reducers)│  │                           │
│  mako-testdata           │  │  Cross-validator (Node.js)│
│  mako-sim (simulation)   │  │    edifact-to-json-       │
│                          │  │    transformer (STROMDAO) │
│  → produces EDIFACT msgs │  │                           │
└────────────┬─────────────┘  └────────────┬──────────────┘
             │    messages flow to         │
             └────────────────────────────→┘
                                           │
                              ┌────────────▼──────────────┐
                              │  Reference data/          │
                              │  Hochfrequenz flat AHB    │
                              │  JSONs (FV2504)           │
                              │  EBD decision tree JSONs  │
                              │  (shipped in repo)        │
                              └───────────────────────────┘
```

**Flow:** Expert triggers a process step → our engine (mako-cli/codec/reducers) produces an EDIFACT message → mako-verify checks that message against the official rules → cross-validator independently parses it → UI shows the expert the combined results.

## Layer 1: Message Validation (AHB)

**Question answered:** "Is each EDIFACT message we produce correct according to the official AHB?"

**How it works:**

1. Our engine produces an EDIFACT message (e.g., UTILMD Anmeldung with Pruefidentifikator 11001)
2. mako-verify loads the flat AHB JSON for that Pruefidentifikator
3. Walks the message segment by segment and checks:
   - **Mandatory fields present** — AHB says "Muss" → field must exist
   - **Forbidden fields absent** — AHB says "X" → field must not exist
   - **Conditional fields correct** — AHB says `Muss [123] U [456]` → evaluate the condition expression given the message context
   - **Value pool correct** — AHB specifies allowed code values → check our value matches
   - **Format correct** — AHB specifies format constraints (e.g., `an..35`, `n..15`) → check length and type

**Expert view:** A table per message showing each segment/field, the AHB rule, our value, and pass/fail.

**Reference data:** Flat AHB JSONs from Hochfrequenz's `machine-readable_anwendungshandbuecher` repo. Covers FV2210–FV2510, our target is FV2504. Static JSON files, ~200 Pruefidentifikatoren. Shipped in repo under `mako-verify/reference-data/ahb/`.

**AHB JSON format (per Pruefidentifikator):**

```json
{
  "lines": [
    {
      "ahb_expression": "Muss",
      "conditions": "",
      "data_element": "0062",
      "guid": "...",
      "index": 2,
      "name": "Nachrichten-Referenznummer",
      "section_name": "Nachrichten-Kopfsegment",
      "segment_code": "UNH",
      "segment_group_key": null,
      "value_pool_entry": null
    }
  ]
}
```

## Layer 2: Process Validation (EBD)

**Question answered:** "Did our state machine make the right decisions at each step?"

**How it works:**

The official EBD (Entscheidungsbaumdiagramme) define exactly what a market role must check when receiving a message. For example, EBD E_0401 says when a Netzbetreiber receives a Lieferantenwechsel-Anmeldung:

1. "Is the MaLo-ID known?" → if no → reject with code A01
2. "Is the Lieferbeginn in the future?" → if no → reject with code A02
3. "Is there already an active supplier?" → if yes → send Abmeldung to LFA
4. ... etc.

Our reducers (mako-gpke, etc.) implement these decision trees. mako-verify compares:

1. Expert triggers a process step (e.g., NB receives Anmeldung)
2. Our reducer processes it → produces output (e.g., Bestätigung + Abmeldung an LFA)
3. mako-verify loads the EBD JSON for that step (e.g., `E_0401.json`)
4. Walks the decision tree with the actual message data → checks if our reducer followed the same path and arrived at the same conclusion

**Expert view:** A visual decision tree where each node shows the question, our system's answer, whether it matches the expected path, and the final outcome.

```
E_0401: Anmeldung prüfen (Netzbetreiber)
─────────────────────────────────────────
1. MaLo-ID bekannt?           → Ja  ✓
2. Lieferbeginn zulässig?     → Ja  ✓
3. Aktiver Lieferant vorhanden?→ Ja  ✓
   → Abmeldung an LFA senden  → ✓ (unser System hat das getan)
```

**Reference data:** EBD JSONs from Hochfrequenz's `machine-readable_entscheidungsbaumdiagramme` repo. Covers FV2304–FV2604. ~300 EBDs. Shipped in repo under `mako-verify/reference-data/ebd/`.

**EBD JSON format:**

```json
{
  "metadata": {
    "ebd_code": "E_0401",
    "ebd_name": "Anmeldung prüfen",
    "role": "NB",
    "section": "6.2.1: ..."
  },
  "rows": [
    {
      "step_number": "1",
      "description": "Ist die MaLo-ID bekannt?",
      "sub_rows": [
        { "check_result": { "result": false }, "result_code": "A01", "note": "MaLo unbekannt" },
        { "check_result": { "result": true, "subsequent_step_number": "2" } }
      ]
    }
  ]
}
```

## Layer 3: Codec Interoperability

**Question answered:** "Can other systems actually read what we produce, and can we read what they produce?"

**Two directions:**

### A) Our messages → their parser

Feed EDIFACT messages from mako-codec to STROMDAO's `edifact-to-json-transformer` (Node.js). Check:
- Parses without errors
- Extracted data (MaLo-ID, dates, roles, etc.) matches what we intended

### B) Their messages → our parser

Take test messages from STROMDAO's built-in generators (`generateTestUTILMD()`, `generateTestMSCONS()`, etc.) and feed them to mako-codec. Check:
- Our parser handles them without errors
- Roundtrip: parse → serialize → parse again — same result

**Expert view:** A comparison table showing field-by-field extraction from both parsers.

```
Message: 001_utilmd_anmeldung.edi
──────────────────────────────────
              Our parser    STROMDAO parser
MaLo-ID:     51238696788   51238696788      ✓
Lieferbeginn: 2026-07-15   2026-07-15       ✓
Absender:     9900000000000 9900000000000   ✓
Empfänger:    9900000000001 9900000000001   ✓
Roundtrip:    ✓             n/a
Parse OK:     ✓             ✓
```

**Runtime:** The Node.js cross-validator runs as a small sidecar process alongside the Express backend. Optional — the system works without it, but with it you get the independent second opinion.

**Reference:** STROMDAO `edifact-to-json-transformer` (npm: `edifact-json-transformer`). MIT license, zero runtime dependencies, supports UTILMD, MSCONS, ORDERS/ORDRSP, INVOIC/REMADV, APERAK/CONTRL.

## UI Integration

### Step-Through Mode

The expert picks a process (e.g., Lieferantenwechsel), then walks through it step by step:

1. See current state and available actions
2. Trigger the next step (e.g., "NB verarbeitet Anmeldung")
3. Our engine produces the output messages
4. Verification runs automatically on each produced message
5. Results appear inline — a verification panel next to the message detail:
   - **AHB check**: expandable table of field-by-field results
   - **EBD check**: visual decision tree with path highlighting
   - **Codec check**: cross-parser comparison (if sidecar running)
   - **Overall verdict**: green/yellow/red badge on the message

The expert can drill into any failure to understand exactly what went wrong.

### Batch Mode

The expert clicks "Simulation verifizieren" → runs the full Netzgebiet Rheinland simulation (or a subset) → all 10,526 messages get verified → produces a report:

- Summary: X of Y messages passed all three layers
- Breakdown by process type, by message type, by market role
- List of failures with drill-down
- Exportable as HTML (like the existing simulation report)

### Verification Badges

Every message in the message list gets a badge:

- **✓✓✓** — passed all three layers
- **✓✓○** — passed AHB + EBD, no cross-validation (sidecar not running)
- **✓✗✓** — AHB passed, EBD failed, codec passed → process logic issue
- **✗○○** — AHB failed → message structure issue

## New Components

### Rust Crates

**`mako-verify`** — new crate, the core verification engine:
- `ahb.rs` — loads flat AHB JSON, validates message fields against AHB rules
- `ebd.rs` — loads EBD JSON, walks decision trees, compares against reducer output
- `interop.rs` — prepares messages for cross-validation, compares extraction results
- `report.rs` — produces structured verification results (JSON)
- `lib.rs` — public API: `verify_nachricht()`, `verify_prozess_schritt()`, `verify_batch()`

**`mako-cli` extension** — new subcommand:
- `mako verify <datei>` — verify a single message (all three layers)
- `mako verify-batch <simulation-dir>` — verify all messages from a simulation run

### Node.js Sidecar

**`mako-ui/src/server/cross-validator.ts`** — wraps `edifact-json-transformer`:
- Express routes: `POST /api/cross-validate` (accepts raw EDIFACT, returns parsed JSON)
- `POST /api/cross-generate` (generates test messages from STROMDAO's generators)
- Comparison logic: field-by-field diff between our extraction and theirs

### React Components

**`VerificationPanel.tsx`** — main verification display:
- Tab layout: AHB | EBD | Codec
- AHB tab: field-by-field table with pass/fail
- EBD tab: visual decision tree
- Codec tab: cross-parser comparison

**`VerificationBadge.tsx`** — compact badge for message list:
- Three indicators (AHB/EBD/Codec), color-coded

**`BatchReport.tsx`** — batch verification results:
- Summary statistics
- Breakdown by process/message type/role
- Failure drill-down
- Export to HTML

**`EbdTree.tsx`** — decision tree visualization:
- Renders EBD as interactive tree
- Highlights the path our system took
- Shows pass/fail at each node

## Reference Data

Shipped in `mako-verify/reference-data/`:

```
mako-verify/reference-data/
├── ahb/
│   └── FV2504/
│       ├── UTILMD/
│       │   ├── 11001.json   (Anmeldung LFW Strom)
│       │   ├── 11002.json   (Bestätigung LFW)
│       │   └── ...
│       ├── MSCONS/
│       ├── ORDERS/
│       └── ...
└── ebd/
    └── FV2504/
        ├── E_0401.json
        ├── E_0402.json
        └── ...
```

**Source:** Downloaded from Hochfrequenz GitHub repos, committed to our repo. Updated when new format versions are published.

## Dependencies

### Required (always available)
- Flat AHB JSONs — static files, no runtime dependency
- EBD JSONs — static files, no runtime dependency
- mako-verify Rust crate — compiled with the workspace

### Optional (enhanced verification)
- Node.js + `edifact-json-transformer` npm package — for Layer 3 cross-validation
- Only needed if sidecar is running; system degrades gracefully without it

## Guided Gates

- GG-1: mako-verify can load flat AHB JSON for a Pruefidentifikator and list expected fields
- GG-2: mako-verify validates a known-good UTILMD Anmeldung → all fields pass
- GG-3: mako-verify validates a known-bad message (missing mandatory field) → correct failure reported
- GG-4: mako-verify loads an EBD JSON and walks the decision tree for a GPKE LFW Anmeldung
- GG-5: EBD validation matches our reducer output for a successful Lieferantenwechsel
- GG-6: Cross-validator parses our EDIFACT and extracts matching field values
- GG-7: Our parser handles STROMDAO-generated test messages without errors
- GG-8: Step-through mode in mako-ui shows verification results inline after processing a message
- GG-9: Batch mode runs verification on the full simulation and produces a summary report
- GG-10: Domain expert can identify a deliberately introduced error using only the UI
