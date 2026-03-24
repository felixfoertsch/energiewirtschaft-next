# MaKo-Engine: Funktionale Marktkommunikation in Rust

**Datum:** 2026-03-24
**Status:** Entwurf
**Ziel-Formatversion:** FV2504 (aktuell gültig ab 04.04.2025). Formatversionsevolution wird über Enum-Varianten und Feature-Gates gelöst — kein Fork pro Version, sondern additive Erweiterung der Typen.
**Ziel:** Beweisbar korrekte, funktional modellierte Referenzimplementierung der deutschen energiewirtschaftlichen Marktkommunikation (Strom + Gas) in Rust — einsetzbar als produktives System auf Cloudflare Edge, FaaS oder WASM.

---

## 1  Kernprinzipien

- Alle Prozesse als pure Funktionen und Zustandsautomaten (Reducer)
- Algebraische Datentypen — ungültige Zustände sind nicht konstruierbar
- Composition über Decoration: Quittungsschicht wrappt Reducer, nicht umgekehrt
- Eigenes Rust-Typsystem, keine externen Dependencies auf BO4E/Python-Tooling
- Hochfrequenz-Ökosystem als Wissensbasis, nicht als Dependency
- Deployment-agnostisch: pure Functions kompilieren zu WASM, native, Lambda, Edge
- Systeme und Formate sind öffentlich, transportierte Inhalte sind vertraulich

---

## 2  Architektur

### 2.1  Schichtenmodell

```
┌─────────────────────────────────────────────────────────┐
│  Prozess-Schicht (State Machine / Reducer)               │
│  STROM: GPKE, WiM, MaBiS, MPES, RD 2.0, §14a, UBP     │
│  GAS:   GeLi Gas, GABi Gas, KoV, Nominierung            │
│  State × Event → Result<(State, Vec<Nachricht>), Fehler> │
├─────────────────────────────────────────────────────────┤
│  Quittungs-Schicht (Decorator, pure)                     │
│  CONTRL: Syntaxprüfung (spartenübergreifend)             │
│  APERAK: EBD-Entscheidungsbaum → accept/reject           │
├─────────────────────────────────────────────────────────┤
│  Mengenumrechnung Gas (pure)                             │
│  m³ × Zustandszahl × Brennwert → kWh                    │
├─────────────────────────────────────────────────────────┤
│  Kern-Typsystem                                          │
│  Marktrollen, Sparten, IDs, Nachrichten, Zeitmodell      │
│  Fristberechnung, Feiertagskalender, Gastag/Stromtag     │
│  Querschnittstypen: IFTSTA, PARTIN, UTILTS               │
├─────────────────────────────────────────────────────────┤
│  Codec (später)                                          │
│  EDIFACT ↔ interne Typen ↔ JSON                         │
├─────────────────────────────────────────────────────────┤
│  Transport-Schicht (IO, nicht-pure, später)               │
│  AS4 · S/MIME · REST-API                                 │
└─────────────────────────────────────────────────────────┘
```

### 2.2  Reducer als Kernabstraktion

Jede Kommunikationslinie ist ein Reducer. Der zentrale Trait:

```rust
/// Ergebnis eines Reducer-Schritts
struct ReducerOutput<S> {
	state: S,
	nachrichten: Vec<Nachricht>,  // Prozess-Nachrichten an andere Rollen
}

/// Fehler bei ungültigem Zustandsübergang
enum ProzessFehler {
	UngueltigerUebergang { state: String, event: String },
	Validierungsfehler(String),
	FristUeberschritten { frist: NaiveDate, eingang: NaiveDate },
}

/// Der Reducer-Trait — implementiert von jedem Prozess-Crate
trait Reducer {
	type State;
	type Event;

	fn reduce(state: Self::State, event: Self::Event) -> Result<ReducerOutput<Self::State>, ProzessFehler>;
}
```

- States und Events sind algebraische Datentypen (enums)
- Jeder State trägt nur die Daten, die in diesem Zustand relevant sind
- `ProzessFehler` ist ein gemeinsamer Typ in `mako-types`, erweiterbar per Enum-Variante
- `Nachricht` ist ein Enum in `mako-types` mit einer Variante pro typisierten Nachrichtentyp

### 2.3  Quittungsschicht als Decorator

CONTRL und APERAK gelten für jede EDIFACT-Nachricht. Die Quittungsschicht wrappt den Reducer.

Rückgabetyp des dekorierten Reducers trennt Prozess-Nachrichten von Quittungen:

```rust
struct DekorierterOutput<S> {
	state: S,
	nachrichten: Vec<Nachricht>,       // Prozess-Nachrichten (an Dritte)
	quittungen: Vec<Quittung>,         // CONTRL/APERAK zurück an Sender
}

struct Quittung {
	an: MarktpartnerId,                // Routing: zurück an den Sender
	typ: QuittungsTyp,                 // Contrl oder Aperak
	ergebnis: QuittungsErgebnis,       // Positiv oder Negativ(fehlercodes)
}
```

Ablauf:

1. Eingehende Nachricht → CONTRL-Syntaxprüfung (pure) → bei Fehler: `quittungen = [CONTRL-negativ]`, Reducer wird **nicht** aufgerufen, State bleibt unverändert
2. Bei CONTRL-ok → APERAK-Anwendungsprüfung (EBD-Logik, pure) → bei Fehler: `quittungen = [CONTRL-positiv, APERAK-negativ]`, Reducer wird **nicht** aufgerufen
3. Bei APERAK-ok → Reducer wird aufgerufen → `quittungen = [CONTRL-positiv, APERAK-positiv]`, `nachrichten` = Reducer-Output

Prozess-Reducer weiß nichts von Quittungen. Quittungsschicht weiß nichts vom Prozess. Quittungen sind an den Sender geroutet, Prozess-Nachrichten an die jeweiligen Empfänger.

---

## 3  Kern-Typsystem

### 3.1  Marktrollen

Enum mit allen MaKo-Rollen (Strom + Gas). Spartenspezifische Einschränkungen über das Typsystem:

- Spartenübergreifend: LF, LFN, LFA, NB, MSB, MDL, BKV
- Nur Strom: ÜNB, BIKO, EIV, BV, DP, ESA, AGG
- Nur Gas: FNB, MGV, TK, SSO, ENB, ANB

### 3.2  Newtype-IDs mit Validierung

- `MaLoId` — 11-stellig mit Prüfziffer, `::new() → Result<MaLoId, ValidationError>`
- `MeLoId` — 33-stellig
- `MarktpartnerId` — 13-stellig (BDEW-Codenummer)

Einmal validiert, immer gültig. Keine rohen Strings im System.

### 3.3  Nachrichten als typisierte Structs

Jede konkrete Nachricht ist ein eigenes Struct (z.B. `UtilmdAnmeldung`, `UtilmdBestaetigung`). Der Compiler erzwingt, dass ein Reducer nur gültige Nachrichten an der richtigen Stelle akzeptiert.

### 3.4  Querschnitts-Nachrichtentypen

In `mako-types` definiert, da sie prozessübergreifend verwendet werden:

- **IFTSTA** — Statusmeldungen zu Prozessschritten
- **PARTIN** — Marktpartner-Stammdaten (MP-ID, Kontaktdaten, Rollen)
- **UTILTS** — Zählzeitdefinitionen, Berechnungsformeln, Aufteilungsfaktoren

Diese haben keinen eigenen Reducer, sondern werden als Nachrichtentypen von mehreren Prozess-Crates referenziert.

### 3.5  Zeitmodell

- Sparte bestimmt Tagesbeginn: Strom = 00:00, Gas = 06:00 (Gastag)
- Fristberechnung: `frist(datum, werktage, kalender, sparte) → NaiveDate`
- Feiertagskalender als Daten, nicht als Code (ladbar, erweiterbar)
- Zeitumstellung: Gastag hat immer 24h, Stromtag hat 23 oder 25h

### 3.6  Formatversionierung

Ziel-Formatversion: **FV2504** (gültig ab 04.04.2025, inkl. LFW24).

Strategie für Versionsevolution:

- Neue Felder/Varianten werden additiv als Enum-Varianten oder optionale Felder ergänzt
- Veraltete Varianten werden mit `#[deprecated]` markiert, nicht gelöscht
- Der Codec (Phase 10) übernimmt das Mapping zwischen Formatversion und internem Modell
- Kein Fork pro Formatversion — ein Typsystem, ein Reducer, versionsbewusster Codec

---

## 4  Crate-Struktur

```
mako/
├── mako-types/          # Kern-Typsystem (Rollen, IDs, Nachrichten, Zeitmodell, IFTSTA, PARTIN, UTILTS)
├── mako-fristen/        # Fristberechnung, Feiertagskalender, Gastag/Stromtag
├── mako-gasumrechnung/  # Mengenumrechnung Gas: m³ × Zustandszahl × Brennwert → kWh
├── mako-quittung/       # CONTRL/APERAK Decorator
├── mako-gpke/           # GPKE-Reducer (LFW/LFW24, Lieferende, Stammdaten, Zuordnung, GDA)
├── mako-wim/            # WiM-Reducer
├── mako-mabis/          # MaBiS-Reducer
├── mako-mpes/           # MPES-Reducer
├── mako-geli/           # GeLi Gas-Reducer
├── mako-gabi/           # GABi Gas-Reducer
├── mako-kov/            # KoV-Reducer (inkl. Nominierung/Renominierung)
├── mako-rd2/            # Redispatch 2.0-Reducer
├── mako-14a/            # §14a-Reducer
├── mako-ubp/            # UBP-Reducer
├── mako-abrechnung/     # INVOIC/REMADV
├── mako-codec/          # EDIFACT ↔ interne Typen ↔ JSON
├── mako-testdata/       # Testdaten-Generator
└── mako-sim/            # Marktsimulation
```

### Dependency-Graph

```
mako-types          ← Basis, keine eigenen Dependencies
mako-fristen        ← mako-types
mako-gasumrechnung  ← mako-types
mako-quittung       ← mako-types, mako-fristen
mako-testdata       ← mako-types, mako-fristen
mako-gpke           ← mako-types, mako-fristen
mako-wim            ← mako-types, mako-fristen
mako-mabis          ← mako-types, mako-fristen
mako-mpes           ← mako-types, mako-fristen
mako-geli           ← mako-types, mako-fristen, mako-gasumrechnung
mako-gabi           ← mako-types, mako-fristen, mako-gasumrechnung
mako-kov            ← mako-types, mako-fristen, mako-gasumrechnung
mako-rd2            ← mako-types, mako-fristen
mako-14a            ← mako-types, mako-fristen
mako-ubp            ← mako-types, mako-fristen
mako-abrechnung     ← mako-types, mako-fristen
mako-codec          ← mako-types
mako-sim            ← mako-types, mako-fristen, mako-quittung, alle Prozess-Crates
```

Kein Prozess-Crate kennt ein anderes Prozess-Crate. Nur `mako-sim` hat die volle Abhängigkeit.

---

## 5  Test-Strategie

### 5.1  Test-Pyramide

- **Unit-Tests:** Typen, Validierung, Fristberechnung
- **Reducer-Tests:** Jeder Zustandsübergang (gültig + ungültig)
- **Integrationstests:** Voller Prozessdurchlauf inkl. Quittungsschicht

### 5.2  TDD-Rhythmus pro Kommunikationslinie

1. Tests schreiben: alle gültigen Übergänge + alle ungültigen Übergänge + Edge Cases
2. Reducer implementieren bis alle Tests grün
3. Testdaten-Generator erweitern
4. Integrationstest: voller Durchlauf mit Quittungsschicht

### 5.3  Testdaten

Eigener Generator (`mako-testdata`) weil 87% der EDIFACT-Testdaten nicht öffentlich verfügbar sind:

- Deterministisch generierte, fiktive IDs (MaLo, MeLo, MP-ID)
- Vollständige Nachrichten-Structs für jeden Prozessschritt
- Szenario-Fixtures: kompletter Durchlauf pro Kommunikationslinie
- Edge Cases als dedizierte Szenarien (Zeitumstellung, Gastag, gleichzeitige Anmeldung, Timeout, Grundversorgung, Feiertage über Jahreswechsel)

### 5.4  Edge Cases (Prio)

| # | Edge Case | Betrifft |
|---|-----------|----------|
| EC1 | Sommer-/Winterzeitumschaltung in Lastgängen | MSCONS |
| EC2 | Gastag (06:00) vs. Stromtag (00:00) | Alle DTM-Segmente |
| EC3 | UTC-Zeitumstellung in EDIFACT (303-Format) | Alle |
| EC4 | Gleichzeitige Anmeldung zweier LF für selbe MaLo | UTILMD |
| EC5 | Rückfall auf Grundversorgung (implizit) | GPKE-Zustandsautomat |
| EC6 | Fristablauf ohne Antwort (Timeout als Event) | Alle Prozesse |
| EC7 | Feiertagskalender über Jahreswechsel | Fristberechnung |
| EC8 | Maximale Anzahl LIN-Segmente | UTILMD |
| EC9 | Sonderzeichen im EDIFACT-Escape | Alle |
| EC10 | Leere optionale Segmente | Alle |

---

## 6  Implementierungsreihenfolge

### Phase 1: Fundament

| # | Aufgabe | Crate |
|---|---------|-------|
| 1.1 | Workspace + Cargo.toml Struktur | root |
| 1.2 | Marktrollen, Sparten, Nachrichtentypen als Enums | `mako-types` |
| 1.3 | Newtype-IDs mit Validierung | `mako-types` |
| 1.4 | Nachrichten-Structs für GPKE-LFW (inkl. LFW24) | `mako-types` |
| 1.5 | Querschnittstypen: IFTSTA, PARTIN, UTILTS Structs | `mako-types` |
| 1.6 | Reducer-Trait, ProzessFehler, Nachricht-Enum | `mako-types` |
| 1.7 | Feiertagskalender + Fristberechnung | `mako-fristen` |
| 1.8 | Gastag/Stromtag Zeitmodell | `mako-fristen` |

### Phase 2: Quittungsschicht

| # | Aufgabe | Crate |
|---|---------|-------|
| 2.1 | CONTRL-Prüfer | `mako-quittung` |
| 2.2 | APERAK-Prüfer (EBD, zunächst GPKE-LFW) | `mako-quittung` |
| 2.3 | Decorator mit getrenntem Routing (Quittungen vs. Prozessnachrichten) | `mako-quittung` |

### Phase 3: Erster Reducer — GPKE Lieferantenwechsel (LFW24)

Der Reducer modelliert den **post-LFW24-Prozess** (gültig ab 06.06.2025) als primären Pfad. Der pre-LFW24-Prozess wird nicht separat implementiert, da FV2504 die Ziel-Formatversion ist.

| # | Aufgabe | Crate |
|---|---------|-------|
| 3.1 | LfwState + LfwEvent Enums | `mako-gpke` |
| 3.2 | Tests: gültige Übergänge (Happy Path) | `mako-gpke` |
| 3.3 | Tests: ungültige Übergänge | `mako-gpke` |
| 3.4 | Tests: Edge Cases (EC4, EC5, EC6) | `mako-gpke` |
| 3.5 | Reducer implementieren | `mako-gpke` |
| 3.6 | Integration mit Quittungsschicht | `mako-gpke` |

### Phase 4: Testdaten-Generator

| # | Aufgabe | Crate |
|---|---------|-------|
| 4.1 | Test-ID-Generatoren | `mako-testdata` |
| 4.2 | UTILMD-Nachrichtengenerator | `mako-testdata` |
| 4.3 | CONTRL/APERAK-Paare | `mako-testdata` |
| 4.4 | Szenario-Fixtures GPKE-LFW | `mako-testdata` |

### Phase 5: Weitere GPKE-Prozesse

| # | Aufgabe | Crate |
|---|---------|-------|
| 5.1 | Lieferende/Abmeldung | `mako-gpke` |
| 5.2 | Stammdatenänderung | `mako-gpke` |
| 5.3 | Zuordnungsprozesse | `mako-gpke` |
| 5.4 | Geschäftsdatenanfrage | `mako-gpke` |
| 5.5 | Testdaten erweitern (MSCONS) | `mako-testdata` |

### Phase 6: WiM + UBP

| # | Aufgabe | Crate |
|---|---------|-------|
| 6.1 | MSB-Wechsel | `mako-wim` |
| 6.2 | Gerätewechsel | `mako-wim` |
| 6.3 | Zählwertübermittlung | `mako-wim` |
| 6.4 | Werte-Anfrage | `mako-wim` |
| 6.5 | Messprodukt-Bestellung | `mako-ubp` |
| 6.6 | Preisblatt-Veröffentlichung | `mako-ubp` |

### Phase 7: MaBiS + Abrechnung

| # | Aufgabe | Crate |
|---|---------|-------|
| 7.1 | Bilanzkreiszuordnung | `mako-mabis` |
| 7.2 | Bilanzierungsdaten | `mako-mabis` |
| 7.3 | Mehr-/Mindermengenabrechnung | `mako-mabis` |
| 7.4 | Clearinglisten | `mako-mabis` |
| 7.5 | INVOIC/REMADV | `mako-abrechnung` |

### Phase 8: MPES + RD 2.0 + §14a

| # | Aufgabe | Crate |
|---|---------|-------|
| 8.1 | MPES Erzeugungsanlagen | `mako-mpes` |
| 8.2 | RD 2.0 (XML-basiert) | `mako-rd2` |
| 8.3 | §14a steuerbare Verbraucher | `mako-14a` |

### Phase 9: Gas-Sparte

| # | Aufgabe | Crate |
|---|---------|-------|
| 9.1 | Mengenumrechnung Gas (m³ → kWh) | `mako-gasumrechnung` |
| 9.2 | GeLi Gas LFW | `mako-geli` |
| 9.3 | GeLi Gas Messwesen | `mako-geli` |
| 9.4 | GABi Gas Bilanzierung | `mako-gabi` |
| 9.5 | KoV Kapazitätsmanagement | `mako-kov` |
| 9.6 | Nominierung/Renominierung | `mako-kov` |

### Phase 10: Codec + Simulation

| # | Aufgabe | Crate |
|---|---------|-------|
| 10.1 | EDIFACT-Parser | `mako-codec` |
| 10.2 | EDIFACT → interne Typen | `mako-codec` |
| 10.3 | Interne Typen → EDIFACT | `mako-codec` |
| 10.4 | JSON-Serialisierung | `mako-codec` |
| 10.5 | Marktsimulation | `mako-sim` |

---

## 7  Guided Gates

Manuelle Verifikationsschritte nach Implementation:

- **GG-1:** `cargo test --workspace` — alle Tests grün, keine Warnungen
- **GG-2:** Jeder Reducer hat Tests für ALLE gültigen UND ungültigen Zustandsübergänge
- **GG-3:** Quittungsschicht ist als Decorator implementiert, nicht in Reducer eingebaut — Quittungen und Prozessnachrichten sind getrennt geroutet
- **GG-4:** Kein Prozess-Crate importiert ein anderes Prozess-Crate (Dependency-Graph wie in Sektion 4 dokumentiert)
- **GG-5:** Alle Newtype-IDs lehnen ungültige Eingaben ab (Tests mit Grenzwerten)
- **GG-6:** Fristberechnung hat Tests für: Normalfall, Wochenende, Feiertag, Jahreswechsel, Gastag-Offset
- **GG-7:** Edge Cases EC1–EC10 sind als dedizierte Tests implementiert (soweit die jeweilige Phase sie betrifft)
- **GG-8:** `cargo build --target wasm32-unknown-unknown` kompiliert für alle pure Crates (kein IO)
- **GG-9:** Testdaten-Generator erzeugt vollständige Nachrichten, keine Platzhalter
- **GG-10:** Ein GPKE-LFW Happy-Path-Szenario läuft komplett durch (Idle → Zugeordnet) inkl. Quittungen
- **GG-11:** Gas-Crates verwenden `mako-gasumrechnung` für alle Mengenkonvertierungen
- **GG-12:** Nominierung/Renominierung ist als Reducer in `mako-kov` implementiert mit eigenen Tests
