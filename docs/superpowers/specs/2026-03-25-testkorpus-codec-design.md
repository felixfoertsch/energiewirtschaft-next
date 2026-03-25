# MaKo-Testkorpus + Codec: Vollständige EDIFACT-Abdeckung

**Datum:** 2026-03-25
**Status:** Entwurf
**Vorgänger:** `2026-03-24-mako-engine-design.md` (Phasen 1–4 implementiert)
**Ziel:** Vollständiger Testkorpus für alle Nachrichtentypen + funktionaler EDIFACT-Codec (Parser + Serializer). Jede Nachricht durchläuft die gesamte Kommunikationskette: EDIFACT-String → Parser → Quittungsschicht → Reducer → Serializer → Roundtrip. Transport ist out of scope.

**Reprioritisierung:** Der Vorgänger-Spec platzierte den Codec in Phase 11. Wir ziehen ihn vor, weil der Testkorpus EDIFACT-Strings als Maßstab braucht — ohne funktionierenden Codec keine fachlich prüfbaren Testdaten. Die verbleibenden Reducer (Phasen 5–10) werden parallel oder danach gebaut; die Kommunikationsketten setzen voraus, dass die jeweiligen Reducer existieren.

**Bestandscode:** `mako-codec` existiert bereits mit einem Basis-Parser (`parser.rs`), Serializer-Stub (`serializer.rs`), und BDEW-Segmenttypen (`bdew_segmente.rs`). Dieses Spec beschreibt den Ausbau zum vollständigen Codec, nicht eine Neuentwicklung.

**Zählung der Nachrichtentypen:**
- `NachrichtenPayload` hat aktuell **43 Varianten** (Stand 2026-03-25)
- Hinzu kommen **3 Querschnittstypen** (IFTSTA, PARTIN, UTILTS) → 46 Payload-Varianten
- CONTRL und APERAK sind **Quittungstypen** (mako-quittung), keine Payload-Varianten — brauchen Fixtures aber keine Generatoren
- COMDIS, INSRPT, ORDCHG: **bewusst zurückgestellt** (Nischentypen, werden ergänzt wenn die Kerntypen stehen)
- Ergibt: **46 Payload-Fixtures + 4 Quittungs-Fixtures + 9 XML-Fixtures = 59 Fixtures gesamt**

**Formatversionen & Zukunftssicherheit:**
- Primäre Zielversion: **FV2504** (aktuell gültig, `docs/mako_aktuell_kostenfrei_25_03_2026/`)
- Zukünftig gültig: **FV2510** (`docs/mako_zukuenftig_kostenfrei_25_03_2026/`) — enthält einen **neuen XML-Typ "Kaskade"** (RD 2.0 Kaskadierung) sowie Versions-Bumps bei UTILMD Gas (G1.0a→G1.1), ORDRSP, PARTIN, REMADV, EBD (4.1→4.2), PID (3.2→3.3)
- Archivierte Versionen: `docs/2026_03_25_mako_*_archiviert/` (2.267 Dokumente, MaKo-Historie seit 2008) — relevant für historische Versionen (Phase 10), nicht für jetzt
- Fixtures werden zunächst für FV2504 gebaut; FV2510-Änderungen werden als separate Module ergänzt wenn die neue FV in Kraft tritt

---

## 1  Scope & Abgrenzung

### Im Scope

- EDIFACT-Lexer: `&str` → Roh-Segmente (nachrichtentypagnostisch)
- EDIFACT-Parser: Roh-Segmente → typisierte Rust-Structs (pro Nachrichtentyp)
- EDIFACT-Serializer: typisierte Rust-Structs → EDIFACT `String`
- XML-Parser/Serializer: XML → RD 2.0 Structs → XML (für Redispatch)
- Referenz-Fixtures: echte EDIFACT-Nachrichten als `&str`-Konstanten (59 Fixtures: 46 Payload + 4 Quittung + 9 XML)
- Generator: parametrisierte Erzeugung gültiger EDIFACT-Nachrichten (46 Payload-Generatoren)
- Fehler-Injektor: gezielte Korrumpierung gültiger Nachrichten (Syntax-, Anwendungs-, Fachfehler)
- Kommunikationsketten: vollständige Prozessdurchläufe als Sequenzen (15 Ketten)
- Erweiterung `NachrichtenPayload`: 3 neue Varianten (IFTSTA, PARTIN, UTILTS) in `mako-types`
- COMDIS, INSRPT, ORDCHG: bewusst zurückgestellt (Nischentypen, werden ergänzt wenn Kerntypen stehen)
- Roundtrip-Tests: `parse(serialize(x)) == x` für jede Fixture

### Außerhalb Scope

- Transportschicht (AS4, S/MIME, REST-API, CLS-Kanal)
- Verschlüsselung/Signatur
- Netzwerk-IO

---

## 2  Quellen

### 2.1  Primärquellen

| Quelle | Inhalt | Pfad/URL |
|--------|--------|----------|
| BDEW MaKo-Dokumente | MIG + AHB für alle 18 EDIFACT-Typen, XSDs für RD 2.0, EBDs, Codelisten | `docs/mako_aktuell_kostenfrei_25_03_2026/` |
| Hochfrequenz `edifact_mapper` | Echte .edi Fixture-Dateien (FV2504/2510/2604), MIG-XML-Schemas, TOML-Mappings pro PID | github.com/Hochfrequenz/edifact_mapper |
| Hochfrequenz `machine-readable_entscheidungsbaumdiagramme` | 1000+ EBDs als JSON-Entscheidungsbäume | github.com/Hochfrequenz/machine-readable_entscheidungsbaumdiagramme |
| Hochfrequenz `machine-readable_anwendungshandbuecher` | AHB flatahb JSON: Muss/Soll/Kann pro Feld + Bedingungen | github.com/Hochfrequenz/machine-readable_anwendungshandbuecher |
| Hochfrequenz `machine-readable_message-implementation-guide` | MIG-Segmenthierarchien als JSON/CSV | github.com/Hochfrequenz/machine-readable_message-implementation-guide |

### 2.2  Ergänzende Quellen

| Quelle | Inhalt |
|--------|--------|
| Hochfrequenz `EDILibrary` (C#) | Vollständige EDIFACT-Beispiele in Tests, GenericEDIWriter-Referenz |
| Hochfrequenz `bo4e-rust` | Rust-Typsystem für Energiemarkt-Objekte (crates.io) |
| Web-Beispiele | Echte UTILMD, INVOIC, APERAK aus B2B by Practice, Enerchy, Corrently |
| BNetzA-PDFs | MIG/AHB-Spezifikationen für alle Typen |

### 2.3  Verfügbare BDEW-Dokumente (lokal)

Vollständige MIG + AHB Paare für alle EDIFACT-Typen:

| Typ | MIG | AHB |
|-----|-----|-----|
| UTILMD Strom | S2.1 | 2.1 |
| UTILMD Gas | G1.0a | Gas 1.0a |
| MSCONS | 2.4c | 3.1f |
| INVOIC | 2.8e | 1.0 |
| REMADV | 2.9d | 1.0 |
| ORDERS | 1.4b | 1.1 |
| ORDRSP | 1.4a | 1.1 |
| ORDCHG | 1.1 | 1.0a |
| REQOTE | 1.3c | 1.1 |
| QUOTES | 1.3b | 1.1 |
| PRICAT | 2.0e | 2.0f |
| CONTRL | 2.0b | 1.0 |
| APERAK | 2.1i | 1.0 |
| IFTSTA | 2.0g | 2.0h |
| PARTIN | 1.0e | 1.0e |
| UTILTS | 1.1e | 1.0 |
| COMDIS | 1.0f | 1.0g |
| INSRPT | 1.1a | 1.1g |

XML RD 2.0 (jeweils FB + AWT + XSD): AcknowledgementDocument, ActivationDocument, Kostenblatt, NetworkConstraintDocument, PlannedResourceScheduleDocument, StatusRequest_MarketDocument, Stammdaten, Unavailability_MarketDocument. **Neu in FV2510:** Kaskade (Kaskadierung von Redispatch-Maßnahmen, FB 1.0 + AWT 1.0 + XSD 1.0).

Plus: EBD und Codelisten 4.1, Allgemeine Festlegungen 6.1b, PID 3.2, alle Codelisten (Artikelnummern, OBIS, Konfigurationen, Lokationsbündelstrukturen, Zeitreihentypen).

### 2.4  Lokale Dokumentenstruktur

| Ordner | Inhalt | Dokumente |
|--------|--------|-----------|
| `docs/mako_aktuell_kostenfrei_25_03_2026/` | Aktuell gültige FV2504 | 206 Dateien |
| `docs/mako_zukuenftig_kostenfrei_25_03_2026/` | Zukünftig gültige FV2510 | 204 Dateien |
| `docs/2026_03_25_mako_api_webdienste_aktuell_gueltig/` | API-Webdienste (aktuell) | 2 Dateien |
| `docs/2026_03_25_mako_*_archiviert/` (13 Ordner) | Archiv seit 2008 | 2.267 Dateien |

**Für die Implementation:** Fixtures werden gegen FV2504 gebaut (`mako_aktuell_kostenfrei_25_03_2026/`). Die MIG-Dokumente (PDF) enthalten die normative Segmentstruktur, die AHB-Dokumente die Feldanforderungen pro PID. FV2510-Dokumente dienen als Vorschau für den nächsten Versions-Sprung — insbesondere der neue XML-Typ **Kaskade** und die UTILMD Gas G1.1.

---

## 3  Architektur

### 3.1  Drei Schichten

```
┌──────────────────────────────────────────────────────┐
│  Schicht 3: Kommunikationsketten (Szenarien)          │
│  gpke_lfw_v2025_happy_path() → Vec<KettenSchritt>     │
│  Jeder Schritt: EDIFACT-String + erwarteter State     │
│  + erwartete Quittung + Roundtrip                     │
├──────────────────────────────────────────────────────┤
│  Schicht 2: Generator + Fehler-Injektor               │
│  erzeuge_utilmd_anmeldung(params) → EDIFACT-String    │
│  injiziere_fehler(msg, FehlerArt) → EDIFACT-String    │
├──────────────────────────────────────────────────────┤
│  Schicht 1: Referenz-Fixtures                         │
│  Pro Nachrichtentyp: EDIFACT &str + erwarteter Struct │
│  44 Payload-Varianten abgedeckt                       │
└──────────────────────────────────────────────────────┘
```

### 3.2  Crate-Struktur

```
mako-codec/                    # EDIFACT ↔ interne Typen ↔ JSON
├── Cargo.toml
└── src/
	├── lib.rs
	├── edifact/
	│   ├── mod.rs
	│   ├── lexer.rs           # UNA/Trennzeichen/Segment-Tokenizer
	│   ├── parser.rs          # Segmente → typisierte Structs (Dispatcher + pro-Typ Parser)
	│   ├── serializer.rs      # Typisierte Structs → EDIFACT-String
	│   └── segment.rs         # RohSegment-Datentyp
	├── xml/
	│   ├── mod.rs
	│   ├── parser.rs          # XML → RD 2.0 Structs
	│   └── serializer.rs      # RD 2.0 Structs → XML
	├── fehler.rs              # CodecFehler enum
	└── roundtrip.rs           # Roundtrip-Tests

mako-testdata/                 # Fixtures, Generator, Fehler-Injektor
├── Cargo.toml
└── src/
	├── lib.rs
	├── fixtures/              # Modul pro Nachrichtentyp
	│   ├── mod.rs
	│   ├── utilmd.rs          # UTILMD: EDIFACT &str + erwarteter Struct (13 Varianten)
	│   ├── mscons.rs          # MSCONS: Lastgang, Zählerstand, Aggregiert, etc.
	│   ├── invoic.rs          # INVOIC: Netznutzung, Messstellenbetrieb, etc.
	│   ├── remadv.rs          # REMADV: positiv, negativ
	│   ├── orders.rs          # ORDERS: Bestellung, WerteAnfrage
	│   ├── ordrsp.rs          # ORDRSP: Bestellantwort
	│   ├── reqote.rs          # REQOTE: Angebotsanfrage
	│   ├── quotes.rs          # QUOTES: Angebot
	│   ├── pricat.rs          # PRICAT: Preisblatt
	│   ├── contrl.rs          # CONTRL: positiv, negativ
	│   ├── aperak.rs          # APERAK: positiv, negativ
	│   ├── iftsta.rs          # IFTSTA: Statusmeldung
	│   ├── partin.rs          # PARTIN: Marktpartner-Stammdaten
	│   ├── utilts.rs          # UTILTS: Berechnungsformeln, Zählzeitdefinitionen
	│   └── xml/
	│       ├── mod.rs
	│       ├── activation.rs  # ActivationDocument
	│       ├── fahrplan.rs    # PlannedResourceScheduleDocument
	│       ├── stammdaten.rs  # RD Stammdaten
	│       ├── kostenblatt.rs # Kostenblatt
	│       ├── acknowledgement.rs
	│       ├── engpass.rs     # NetworkConstraintDocument
	│       ├── nichtverfuegbarkeit.rs # Unavailability_MarketDocument
	│       ├── statusrequest.rs
	│       └── kaskade.rs         # Kaskade (neu in FV2510)
	├── generator/
	│   ├── mod.rs
	│   ├── segmente.rs        # Segment-Bausteine (una, unb, unh, bgm, dtm, nad, ...)
	│   ├── edifact.rs         # Pro-Typ Generatoren (44 Stück)
	│   ├── xml.rs             # XML-Generatoren (8 Stück)
	│   └── params.rs          # Params-Structs mit Default
	├── fehler.rs              # FehlerArt enum + injiziere_fehler()
	├── ketten.rs              # Kommunikationsketten (15 Szenarien)
	├── ids.rs                 # Test-ID-Generatoren (existiert)
	├── quittungen.rs          # Quittungs-Fixtures (existiert)
	├── szenarien.rs           # Alt — wird durch ketten.rs ersetzt
	└── szenarien_historisch.rs
```

### 3.3  Dependency-Graph

```
mako-types           ← Basis (keine eigenen Dependencies)
mako-codec           ← mako-types
mako-testdata        ← mako-types, mako-codec, mako-fristen, mako-gpke, ...
                        (alle Prozess-Crates als dev-dependency für Ketten)
```

`mako-codec` hat **keine** Abhängigkeit auf `mako-testdata` (nur `dev-dependencies` für Roundtrip-Tests). `mako-testdata` hängt von `mako-codec` ab (braucht Parser/Serializer für Ketten-Runner).

---

## 4  EDIFACT-Codec

### 4.1  Lexer

Zeichenweise Tokenisierung des EDIFACT-Rohstrings. Nachrichtentypagnostisch.

**Bestandscode:** `mako-codec` hat bereits einen `Segment`-Typ in `segment.rs` und Parsing-Logik in `parser.rs`. Der bestehende Code wird zum Lexer refaktoriert — die vorhandene `Segment`-Struktur wird beibehalten und erweitert, nicht ersetzt.

```rust
/// UNA-Trennzeichen (konfigurierbar, Standard: :+.? ')
pub struct Trennzeichen {
	pub komponent: char,   // : (trennt Komponenten innerhalb Komposit-Datenelement)
	pub daten: char,       // + (trennt Datenelemente)
	pub dezimal: char,     // . (Dezimalpunkt)
	pub escape: char,      // ? (Escape-Zeichen)
	pub segment: char,     // ' (Segment-Ende)
}

/// Ein rohes Segment aus dem Lexer.
/// Erweitert den bestehenden Segment-Typ aus segment.rs.
pub struct RohSegment {
	pub tag: String,                        // z.B. "UNB", "DTM", "NAD"
	pub datenelemente: Vec<Vec<String>>,    // Äußere Vec = Datenelemente, innere Vec = Komponenten
}

/// Tokenisiert EDIFACT-Rohstring in Segmente.
/// Erkennt UNA (optionales Service-String-Advice), behandelt Escape-Sequenzen.
/// Fail-fast: bricht beim ersten Fehler ab (kein Recovery).
pub fn tokenize(input: &str) -> Result<(Trennzeichen, Vec<RohSegment>), CodecFehler>
```

### 4.2  Parser

Handgeschriebene Parser pro Nachrichtentyp. Kein generisches Schema-Loading — alles zur Kompilierzeit bekannt.

```rust
/// Dispatcher: erkennt Nachrichtentyp aus UNH-Segment + PID aus RFF+Z13,
/// routet zum richtigen Parser.
pub fn parse(input: &str) -> Result<Nachricht, CodecFehler>

/// Pro Nachrichtentyp eine parse-Funktion:
fn parse_utilmd_anmeldung(segmente: &[RohSegment]) -> Result<UtilmdAnmeldung, CodecFehler>
fn parse_mscons_lastgang(segmente: &[RohSegment]) -> Result<MsconsLastgang, CodecFehler>
fn parse_invoic(segmente: &[RohSegment]) -> Result<InvoicRechnung, CodecFehler>
// ... 44 Parser insgesamt
```

**Disambiguation:** Viele Payload-Varianten teilen denselben UNH-Nachrichtentyp (z.B. UTILMD deckt Anmeldung, Abmeldung, Stammdatenänderung, Bilanzkreiszuordnung, etc. ab). Der Dispatcher verwendet eine zweistufige Erkennung:

1. **UNH-Segment** → EDIFACT-Nachrichtentyp (UTILMD, MSCONS, INVOIC, ...)
2. **RFF+Z13-Segment** → Prüfidentifikator (44001 = Anmeldung, 44003 = Abmeldung, ...) + **BGM-Qualifier** (E01 = Anmeldung, E02 = Abmeldung, ...) als Fallback wenn kein PID vorhanden

Für MSCONS-Varianten (Lastgang vs. Zählerstand vs. Aggregiert) unterscheidet der Parser anhand des DTM-Qualifiers und der Segmentstruktur (Anzahl/Art der QTY-Segmente).

Designentscheidung: Handgeschrieben statt schema-basiert. Gründe:
- 46 Payload-Varianten, nicht tausende → überschaubarer Aufwand
- Lesbarer, testbarer, bessere Fehlermeldungen
- MIG/AHB definieren exakt welche Segmente in welcher Reihenfolge
- Kein Runtime-Schema-Loading → WASM-kompatibel

### 4.3  Serializer

Spiegelbildlich zum Parser. Pro Nachrichtentyp eine Serialisierungsfunktion.

```rust
pub fn serialize(nachricht: &Nachricht) -> String

fn serialize_utilmd_anmeldung(msg: &UtilmdAnmeldung, trz: &Trennzeichen) -> String
fn serialize_mscons_lastgang(msg: &MsconsLastgang, trz: &Trennzeichen) -> String
// ... 44 Serializer
```

### 4.4  XML (Redispatch 2.0)

Separates Modul für die 8 XML-Dokumenttypen. Validierung gegen die XSD-Schemas aus `docs/mako_aktuell_kostenfrei_25_03_2026/`.

```rust
pub fn parse_xml(input: &str) -> Result<Nachricht, CodecFehler>
pub fn serialize_xml(nachricht: &Nachricht) -> String
```

### 4.5  Fehlertypen

```rust
pub enum CodecFehler {
	// Lexer-Fehler
	KeinUnaOderUnb,
	UngueltigesTrennzeichen { position: usize },
	UnterbrocheneEscapeSequenz { position: usize },

	// Parser-Fehler
	SegmentFehlt { erwartet: String },
	UnbekannterNachrichtentyp { typ: String },
	UnbekannterPruefIdentifikator { code: String },
	FeldFehlt { segment: String, feld: String },
	UngueltigerWert { segment: String, feld: String, wert: String },
	UngueltigesFormat { segment: String, feld: String, erwartet: String },

	// XML-Fehler
	XmlParseFehler(String),
	XsdValidierungsFehler(String),
}
```

---

## 5  Referenz-Fixtures

Pro Nachrichtentyp mindestens eine vollständige, fachlich korrekte EDIFACT-Nachricht als `&str`-Konstante, plus der erwartete typisierte Struct nach dem Parsen.

### 5.1  Fixture-Struktur

```rust
/// GPKE 1.1.1: Anmeldung LFN → NB (PID 44001, FV2504)
pub const ANMELDUNG_LFW_EDI: &str = "UNA:+.? '...";

/// Der erwartete typisierte Struct nach dem Parsen
pub fn anmeldung_lfw_erwartet() -> Nachricht { /* ... */ }
```

### 5.2  Quellen pro Fixture

| Gruppe | Fixtures | Anzahl | Primärquelle |
|--------|----------|--------|--------------|
| GPKE | Anmeldung, Bestätigung, Abmeldung, Ablehnung, Zuordnung, LieferendeAbmeldung, LieferendeBestaetigung, Schlussturnusmesswert, Lastgang, Stammdatenaenderung, Zuordnungsliste, GDA-Anfrage, GDA-Antwort | 13 | edifact_mapper + UTILMD MIG S2.1 |
| WiM | MsbWechselAnmeldung, Geraetewechsel, WerteAnfrage | 3 | UTILMD MIG + ORDERS MIG |
| UBP | Angebotsanfrage, Angebot, Bestellung, Bestellantwort, Preisblatt | 5 | REQOTE/QUOTES/ORDERS/ORDRSP/PRICAT MIG |
| MaBiS | Bilanzkreiszuordnung, AggregierteZeitreihen, MehrMindermengen, Clearingliste | 4 | UTILMD/MSCONS MIG |
| Abrechnung | Rechnung, Zahlungsavis | 2 | INVOIC/REMADV MIG |
| MPES | AnmeldungErzeugung, EinspeiseMesswerte | 2 | UTILMD/MSCONS MIG |
| RD 2.0 (XML) | Stammdaten, Fahrplan, Aktivierung, Bestaetigung, Engpass, Nichtverfuegbarkeit, Kostenblatt, StatusRequest, Kaskade | 9 | XSD + AWT |
| §14a | SteuerbareVerbrauchseinrichtung, Steuersignal | 2 | UTILMD MIG + CLS-Spec |
| Gas | Nominierung, NominierungBestaetigung, Renominierung, Brennwert, Ausspeisepunkt | 5 | UTILMD/MSCONS MIG Gas |
| Querschnitt | IFTSTA-Statusmeldung, PARTIN-Marktpartner, UTILTS-Berechnungsformel | 3 | IFTSTA/PARTIN/UTILTS MIG |
| Quittung | CONTRL positiv, CONTRL negativ, APERAK positiv, APERAK negativ | 4 | CONTRL/APERAK MIG + Web-Beispiele |
| **Payload-Fixtures** | | **46** | (43 bestehende + 3 neue Querschnittstypen) |
| **Quittungs-Fixtures** | | **4** | (nicht in NachrichtenPayload) |
| **XML-Fixtures** | | **9** | (inkl. Kaskade aus FV2510) |
| **Gesamt** | | **59** | |

**Bewusst zurückgestellt:** COMDIS (Handelsunstimmigkeit), INSRPT (Prüfbericht), ORDCHG (Bestelländerung). Diese sind Nischentypen mit geringer Prozesspräsenz. MIG/AHB-Dokumente liegen vor; Fixtures werden ergänzt wenn die Kerntypen stehen.

### 5.3  Roundtrip-Test pro Fixture

```rust
#[test]
fn roundtrip_utilmd_anmeldung() {
	let parsed = parse(ANMELDUNG_LFW_EDI).unwrap();
	assert_eq!(parsed, anmeldung_lfw_erwartet());
	let serialized = serialize(&parsed);
	let reparsed = parse(&serialized).unwrap();
	assert_eq!(reparsed, parsed);
}
```

---

## 6  Generator

### 6.1  Segment-Bausteine

Pure functions die einzelne EDIFACT-Segmente erzeugen. Komponierbar.

```rust
fn una() -> String
fn unb(absender: &str, empfaenger: &str, datum: NaiveDate, referenz: &str) -> String
fn unh(referenz: &str, typ: &str, version: &str) -> String
fn bgm(dokumenttyp: &str, referenz: &str) -> String
fn dtm(qualifier: &str, datum: NaiveDate, format: DtmFormat) -> String
fn nad(qualifier: &str, mp_id: &MarktpartnerId) -> String
fn rff_pid(pid: &PruefIdentifikator) -> String
fn ide_malo(malo: &MaLoId) -> String
fn ide_melo(melo: &MeLoId) -> String
fn unt(segmentzahl: u32, referenz: &str) -> String
fn unz(nachrichtenzahl: u32, referenz: &str) -> String
```

### 6.2  Params-Structs

Jeder Nachrichtentyp hat einen eigenen Params-Struct mit `Default`:

```rust
pub struct AnmeldungParams {
	pub absender_mp: MarktpartnerId,
	pub empfaenger_mp: MarktpartnerId,
	pub malo_id: MaLoId,
	pub lieferbeginn: NaiveDate,
	pub datum: NaiveDate,
	pub referenz: String,
	pub dokument_nr: String,
}

impl Default for AnmeldungParams {
	fn default() -> Self { /* fachlich sinnvolle Defaults */ }
}
```

Struct-Update-Syntax für Varianten: `AnmeldungParams { lieferbeginn: ..., ..Default::default() }`.

### 6.3  Nachricht-Generatoren

46 Generatoren — einer pro Payload-Variante (43 bestehende + 3 neue Querschnittstypen):

```rust
pub fn erzeuge_utilmd_anmeldung(params: &AnmeldungParams) -> String
pub fn erzeuge_mscons_lastgang(params: &LastgangParams) -> String
pub fn erzeuge_invoic(params: &InvoicParams) -> String
// ... 46 insgesamt

// Convenience-Shorthand mit Defaults:
pub fn anmeldung() -> String { erzeuge_utilmd_anmeldung(&AnmeldungParams::default()) }
```

CONTRL/APERAK haben keine Generatoren — sie werden von `mako-quittung` erzeugt, nicht vom Testdaten-Generator.

---

## 7  Fehler-Injektor

### 7.1  FehlerArt-Katalog

```rust
pub enum FehlerArt {
	// Syntaxfehler (CONTRL-Ebene)
	AbsenderLeer,
	EmpfaengerLeer,
	SegmentFehlt(String),
	UngueltigesTrennzeichen,
	FalscheSegmentzahl,
	EscapeFehler,

	// Anwendungsfehler (APERAK-Ebene)
	FristUeberschritten { tage: i32 },
	UngueltigeMaLoId,
	UngueltigeMarktpartnerId,
	FalscheRolle,
	FalscherPruefIdentifikator,
	UnbekannterQualifier(String),

	// Fachliche Fehler (Reducer-Ebene)
	DoppelteAnmeldung,
	WiderspruchNachFrist,
}
```

### 7.2  Signatur

```rust
/// Pure function: injiziert Fehler, gibt neuen String zurück.
pub fn injiziere_fehler(edifact: &str, fehler: &FehlerArt) -> String
```

### 7.3  Fehler-Ebenen spiegeln Architektur

| Fehler-Ebene | Abgefangen durch | Reducer aufgerufen? |
|--------------|------------------|---------------------|
| Syntaxfehler | CONTRL (mako-quittung) | Nein |
| Anwendungsfehler | APERAK (mako-quittung) | Nein |
| Fachliche Fehler | Reducer (mako-gpke etc.) | Ja, gibt ProzessFehler |

---

## 8  Kommunikationsketten

### 8.1  Datenstruktur

```rust
pub struct KettenSchritt {
	pub edifact: String,
	pub nachricht: Nachricht,
	pub erwarteter_state: Box<dyn std::fmt::Debug>,  // Algebraischer State-Typ (pro Kette konkret)
	pub state_pruefer: fn(&dyn std::any::Any) -> bool,  // Typisierte State-Prüfung
	pub erwartete_quittung: Quittungsergebnis,
}

pub enum Quittungsergebnis {
	ContrlPositiv,
	ContrlNegativ,
	AperakPositiv,
	AperakNegativ { fehler_code: String },
	KeineQuittung,
}

pub struct Kette {
	pub name: &'static str,
	pub prozess: &'static str,
	pub version: MakoVersion,
	pub schritte: Vec<KettenSchritt>,
}
```

### 8.2  Ketten-Runner

```rust
/// Spielt eine Kette gegen die Engine ab:
/// Parser → Quittung → Reducer → Serializer → Roundtrip
pub fn pruefe_kette(kette: &Kette) -> Vec<KettenErgebnis>
```

### 8.3  Geplante Ketten (15)

| # | Kette | Schritte | Rollen | Prozess |
|---|-------|----------|--------|---------|
| 1 | GPKE LFW Happy Path | 7 | LFN, NB, LFA | GPKE |
| 2 | GPKE LFW Ablehnung durch LFA | 5 | LFN, NB, LFA | GPKE |
| 3 | GPKE LFW Fristüberschreitung | 3 | LFN, NB | GPKE |
| 4 | GPKE Lieferende | 4 | LF, NB, MSB | GPKE |
| 5 | GPKE Stammdatenänderung | 3 | NB, LF | GPKE |
| 6 | WiM MSB-Wechsel | 4 | MSB_neu, NB, MSB_alt | WiM |
| 7 | WiM Zählwertübermittlung | 3 | MSB, NB, LF | WiM |
| 8 | UBP Bestellung | 5 | LF, MSB | UBP |
| 9 | MaBiS Bilanzkreiszuordnung | 3 | LF, NB, BKV | MaBiS |
| 10 | Abrechnung Netznutzung | 3 | NB, LF | Abrechnung |
| 11 | RD 2.0 Abruf | 4 | ÜNB, NB, EIV | RD 2.0 |
| 12 | §14a Steuerung | 3 | NB, MSB | §14a |
| 13 | GeLi Gas LFW | 7 | LFN, NB, LFA | GeLi Gas |
| 14 | GABi Gas Nominierung | 4 | BKV, MGV | GABi Gas |
| 15 | KoV Brennwertmitteilung | 2 | FNB, NB | KoV |

### 8.4  Fachlicher Maßstab

Wenn alle 15 Ketten sauber durchlaufen — Parser, Quittung, Reducer, Serializer, Roundtrip — dann funktioniert die Engine für den jeweiligen Prozess. Die Ketten sind der Abnahmetest.

---

## 9  Implementierungsreihenfolge

### Phase A: Lexer + Basis-Parser (TDD)

| # | Aufgabe | Crate |
|---|---------|-------|
| A.1 | EDIFACT-Lexer: UNA, Trennzeichen, Escape, Segment-Tokenizer | mako-codec |
| A.2 | Erste Fixture: UTILMD Anmeldung (PID 44001) als `&str` | mako-testdata |
| A.3 | Parser für UTILMD Anmeldung (TDD: Fixture → Struct) | mako-codec |
| A.4 | Serializer für UTILMD Anmeldung (Struct → EDIFACT) | mako-codec |
| A.5 | Roundtrip-Test: parse → serialize → parse = identisch | mako-codec |

### Phase B: Alle EDIFACT-Fixtures + Parser (TDD)

| # | Aufgabe | Crate |
|---|---------|-------|
| B.1 | Fixtures: alle 13 GPKE-Varianten | mako-testdata |
| B.2 | Parser + Serializer: alle GPKE-Varianten | mako-codec |
| B.3 | Fixtures: WiM (3), UBP (5), MaBiS (4), Abrechnung (2) | mako-testdata |
| B.4 | Parser + Serializer: WiM, UBP, MaBiS, Abrechnung | mako-codec |
| B.5 | Fixtures: MPES (2), §14a (2), Gas (5), Quittung (4) | mako-testdata |
| B.6 | Parser + Serializer: MPES, §14a, Gas, Quittung | mako-codec |
| B.7 | Fixtures: IFTSTA, PARTIN, UTILTS (Querschnitt, 3 Stück) — NachrichtenPayload erweitern | mako-testdata + mako-types |
| B.8 | Parser + Serializer: Querschnitts-Typen | mako-codec |

### Phase C: XML (Redispatch 2.0)

| # | Aufgabe | Crate |
|---|---------|-------|
| C.1 | XML-Fixtures: alle 9 RD 2.0 Dokumenttypen (inkl. Kaskade) | mako-testdata |
| C.2 | XML-Parser + Serializer (XSD-validiert) | mako-codec |
| C.3 | XML-Roundtrip-Tests | mako-codec |

### Phase C2: Aufräumen

| # | Aufgabe | Crate |
|---|---------|-------|
| C2.1 | Bestehende `szenarien.rs` und `szenarien_historisch.rs` durch `ketten.rs` ersetzen | mako-testdata |
| C2.2 | Bestehende `mscons.rs` und `utilmd.rs` in `fixtures/` migrieren | mako-testdata |

### Phase D: Generator

| # | Aufgabe | Crate |
|---|---------|-------|
| D.1 | Segment-Bausteine (una, unb, unh, bgm, dtm, nad, ...) | mako-testdata |
| D.2 | Params-Structs mit Default (46 Stück) | mako-testdata |
| D.3 | Nachricht-Generatoren (46 Stück) | mako-testdata |
| D.4 | Generator-Tests: generiert → parse → Struct korrekt | mako-testdata |
| D.5 | XML-Generatoren (8 Stück) | mako-testdata |

### Phase E: Fehler-Injektor

| # | Aufgabe | Crate |
|---|---------|-------|
| E.1 | FehlerArt enum + injiziere_fehler() | mako-testdata |
| E.2 | Syntaxfehler-Tests (CONTRL erkennt) | mako-testdata |
| E.3 | Anwendungsfehler-Tests (APERAK erkennt) | mako-testdata |
| E.4 | Fachliche Fehler-Tests (Reducer erkennt) | mako-testdata |

### Phase F: Kommunikationsketten

**Voraussetzung:** Ketten setzen voraus, dass die jeweiligen Reducer funktional sind. Ketten für Prozesse deren Reducer noch nicht implementiert sind (z.B. RD 2.0, §14a) werden erst gebaut, wenn der Reducer steht. Ketten 1–5 (GPKE) können sofort gebaut werden.

| # | Aufgabe | Crate |
|---|---------|-------|
| F.1 | KettenSchritt, Kette, Ketten-Runner Datenstrukturen | mako-testdata |
| F.2 | Kette 1: GPKE LFW Happy Path (7 Schritte) | mako-testdata |
| F.3 | Kette 2-3: GPKE LFW Ablehnung + Fristüberschreitung | mako-testdata |
| F.4 | Ketten 4-5: GPKE Lieferende + Stammdaten | mako-testdata |
| F.5 | Ketten 6-8: WiM + UBP | mako-testdata |
| F.6 | Ketten 9-10: MaBiS + Abrechnung | mako-testdata |
| F.7 | Ketten 11-12: RD 2.0 + §14a | mako-testdata |
| F.8 | Ketten 13-15: Gas (GeLi, GABi, KoV) | mako-testdata |

---

## 10  Guided Gates

Manuelle Verifikationsschritte nach Implementation:

- **GG-1:** `cargo test --workspace` — alle Tests grün, keine Warnungen
- **GG-2:** Jede der 59 Fixtures parst korrekt zum erwarteten Struct
- **GG-3:** Jede Fixture besteht den Roundtrip-Test (parse → serialize → parse = identisch)
- **GG-4:** Alle 46 Payload-Generatoren erzeugen Nachrichten die der Parser akzeptiert
- **GG-5:** Generator-Nachrichten stimmen fachlich mit MIG/AHB überein (manuelle Prüfung Stichprobe)
- **GG-6:** Jede FehlerArt wird von der richtigen Schicht erkannt (Syntax → CONTRL, Anwendung → APERAK, Fachlich → Reducer)
- **GG-7:** Alle 15 Kommunikationsketten laufen sauber durch (Parser → Quittung → Reducer → Serializer)
- **GG-8:** Keine Prozess-Crate-Dependency auf mako-codec (Codec kennt Typen, nicht Reducer)
- **GG-9:** `cargo build --target wasm32-unknown-unknown` kompiliert für mako-codec (kein IO)
- **GG-10:** EDIFACT-Fixtures verwenden korrekte Formatversion (FV2504/S2.1 für Strom, G1.0a für Gas)
- **GG-11:** XML-Fixtures validieren gegen die mitgelieferten XSDs
- **GG-12:** Fehler-Injektor verändert das Original nicht (pure function)
- **GG-13:** Ketten-Runner prüft Roundtrip für jede ausgehende Nachricht
