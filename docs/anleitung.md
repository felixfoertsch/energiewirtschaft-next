# MaKo-Engine — Bedienungsanleitung

> Sektionen 1–6. Sektionen 7–9 (CLI, Simulation, UI) folgen in Task 10.

---

## 1. Projektuebersicht

Die MaKo-Engine ist eine beweisbar korrekte Referenzimplementierung der deutschen Marktkommunikation fuer Strom und Gas in Rust.

**Ziel:** Jede Kommunikationslinie der Marktkommunikation als pure Function abbilden — deterministische Eingabe-Ausgabe-Beziehungen ohne Seiteneffekte. Das System ist deployment-agnostisch: es laeuft als WASM-Modul, auf Cloudflare Edge, in FaaS-Umgebungen oder als eingebettete Bibliothek. Die gesamte MaKo-Historie (Formatversionen 2017, 2020, 2022, 2025 und zukuenftige) wird parallel abgebildet, ohne dass aeltere Versionen entfernt werden.

**Warum Rust?** Das Typsystem erzwingt Korrektheit zur Kompilierzeit. Ungueltiger State ist nicht darstellbar. IDs werden einmal validiert und sind danach immer gueltig. Algebraische Datentypen (Enums mit Daten) bilden Geschaeftsprozesse als Zustandsmaschinen ab.

**Fuer wen?** Entwickler, Ingenieure und Fachexperten, die das System verstehen, testen und erweitern wollen. Die Engine dient sowohl als ausfuehrbares Nachschlagewerk fuer MaKo-Regeln als auch als produktives Backend fuer Marktkommunikation.

**Abgrenzung:** Die MaKo-Engine ist kein ERP-System und kein UI-Framework. Sie implementiert die Geschaeftslogik der Marktkommunikation. Transport (AS4, E-Mail) liegt ausserhalb des Scope.

---

## 2. Architektur

### Schichtenmodell

Die Engine folgt einem strikten Schichtenmodell. Jede Schicht hat eine klar definierte Aufgabe:

```
Eingehende EDIFACT-Nachricht
	→ Codec (EDIFACT/XML → interne Typen)
	→ Quittungsschicht (CONTRL/APERAK Decorator)
	→ Prozess-Reducer (State × Event → State' × Nachrichten)
	→ Codec (interne Typen → EDIFACT/XML)
Ausgehende EDIFACT-Nachricht
```

- **Codec-Schicht:** Wandelt EDIFACT-Strings und XML-Dokumente in typisierte Rust-Structs um und zurueck. Keine Geschaeftslogik.
- **Quittungsschicht:** Prueft Syntax (CONTRL) und Anwendungsregeln (APERAK) bevor der Reducer aufgerufen wird. Bei Fehler bleibt der State unveraendert.
- **Prozess-Schicht:** Jeder Geschaeftsprozess ist ein Reducer: `(State, Event) → Result<(State, Vec<Nachricht>), ProzessFehler>`.
- **Transport:** Nicht Teil der Engine. AS4, E-Mail, SFTP — das erledigt die umgebende Infrastruktur.

### Crate-Struktur

Der Workspace enthaelt 18 Crates:

| Crate | Beschreibung |
|---|---|
| `mako-types` | Kern-Typsystem: Rollen, IDs, Nachrichten, Reducer-Trait |
| `mako-fristen` | Fristberechnung, Feiertagskalender, Gastag/Stromtag |
| `mako-quittung` | CONTRL/APERAK Decorator |
| `mako-codec` | EDIFACT ↔ interne Typen, XML fuer RD 2.0 |
| `mako-gpke` | GPKE-Reducer (LFW, Lieferende, Stammdaten, Zuordnung, GDA) |
| `mako-wim` | WiM-Reducer (Messwesen) |
| `mako-ubp` | UBP-Reducer (ungeplante Beschaffungsprozesse) |
| `mako-mabis` | MaBiS-Reducer (Bilanzierung) |
| `mako-abrechnung` | Abrechnungs-Reducer (Netznutzung, MSB) |
| `mako-mpes` | MPES-Reducer (Erzeugung/Einspeisung) |
| `mako-rd2` | Redispatch 2.0-Reducer |
| `mako-14a` | §14a EnWG-Reducer (steuerbare Verbrauchseinrichtungen) |
| `mako-geli` | GeLi Gas-Reducer (Lieferantenwechsel Gas) |
| `mako-gabi` | GABi Gas-Reducer (Nominierung) |
| `mako-kov` | KoV-Reducer (Kooperationsvereinbarung Gas) |
| `mako-gasumrechnung` | Gasumrechnungs-Reducer |
| `mako-testdata` | Fixtures, Generatoren, Fehler-Injektor, Ketten |
| `mako-sim` | Marktsimulation (agentenbasiert) |

### Dependency-Graph

Kein Prozess-Crate kennt ein anderes Prozess-Crate. Alle Prozess-Crates haengen ausschliesslich von `mako-types` und `mako-fristen` ab. Nur `mako-sim` und `mako-testdata` haben Abhaengigkeiten auf alle Prozess-Crates und den Codec. Das verhindert zirkulaere Abhaengigkeiten und erlaubt isolierte Entwicklung.

---

## 3. Typsystem

Das Typsystem in `mako-types` bildet die gesamte Domäne der Marktkommunikation als algebraische Datentypen ab.

### MarktRolle

20 Rollen, gegliedert nach Spartenzugehoerigkeit:

- **Spartenuebergreifend (7):** Lieferant, LieferantNeu, LieferantAlt, Netzbetreiber, Messstellenbetreiber, Messdienstleister, Bilanzkreisverantwortlicher
- **Nur Strom (7):** Uebertragungsnetzbetreiber, Bilanzkoordinator, Einsatzverantwortlicher, BetreiberErzeugungsanlage, Direktvermarkter, Energieserviceanbieter, Aggregator
- **Nur Gas (6):** Fernleitungsnetzbetreiber, Marktgebietsverantwortlicher, Transportkunde, Speicherstellenbetreiber, Einspeisenetzbetreiber, Ausspeisenetzbetreiber

Jede Rolle kennt ihre Sparten ueber `rolle.sparten() → &[Sparte]`.

### Sparte

```rust
pub enum Sparte {
	Strom,
	Gas,
}
```

### IDs

Drei ID-Typen, die bei Konstruktion validiert werden — danach immer gueltig:

- **`MaLoId`** — Marktlokations-ID, 11 Stellen, letzte Stelle = Luhn-Pruefsumme
- **`MeLoId`** — Messlokations-ID, 33 Zeichen, beginnt mit `DE`
- **`MarktpartnerId`** — BDEW-Codenummer, 13 Stellen

```rust
let malo = MaLoId::new("51238696788")?;      // Luhn-validiert
let melo = MeLoId::new("DE00000000000000000000000000000A")?;
let mp   = MarktpartnerId::new("9900000000003")?;
```

Jeder Konstruktor gibt `Result<Self, ValidationError>` zurueck. Ungueltiger State ist nicht darstellbar.

### Nachricht

Der zentrale Envelope fuer jede MaKo-Nachricht:

```rust
pub struct Nachricht {
	pub absender: MarktpartnerId,
	pub absender_rolle: MarktRolle,
	pub empfaenger: MarktpartnerId,
	pub empfaenger_rolle: MarktRolle,
	pub pruef_id: Option<PruefIdentifikator>,
	pub payload: NachrichtenPayload,
}
```

### NachrichtenPayload

48 Varianten, gegliedert nach Prozess:

- **GPKE (13):** Anmeldung, Bestaetigung, Abmeldung, Ablehnung, Zuordnung, Lieferende (2), Stammdatenaenderung, Zuordnungsliste, Geschaeftsdatenanfrage/-antwort, MSB-Wechsel, Geraetewechsel
- **WiM (1):** WerteAnfrage (Orders)
- **UBP (5):** Angebotsanfrage, Angebot, Bestellung, Bestellantwort, Preisblatt
- **MaBiS (4):** Bilanzkreiszuordnung, Aggregierte Zeitreihen, Mehr-/Mindermengen, Clearingliste
- **Abrechnung (2):** Rechnung (Invoic), Zahlungsavis (Remadv)
- **MPES (2):** Anmeldung Erzeugung, Einspeise-Messwerte
- **RD 2.0 (9):** Stammdaten, Fahrplan, Aktivierung, Bestaetigung, Engpass, Nichtverfuegbarkeit, Kostenblatt, StatusRequest, Kaskade
- **§14a (2):** Steuerbare Verbrauchseinrichtung, Steuersignal
- **Gas (5):** Nominierung, Nominierung-Bestaetigung, Renominierung, Brennwert, Ausspeisepunkt
- **Querschnitt (3):** IFTSTA-Statusmeldung, PARTIN-Marktpartner, UTILTS-Zaehlzeitdefinition

### PruefIdentifikator

30 BDEW RFF+Z13 Codes, z.B. `AnmeldungNn` (44001), `Zaehlerstand` (13002), `Abschlagsrechnung` (31001). Jeder Identifier kennt seinen numerischen Code (`code() → u32`) und seinen Prozess (`prozess() → &str`).

### Reducer-Trait

```rust
pub trait Reducer {
	type State;
	type Event;

	fn reduce(
		state: Self::State,
		event: Self::Event,
	) -> Result<ReducerOutput<Self::State>, ProzessFehler>;
}
```

---

## 4. Reducer-Konzept

Jede Kommunikationslinie ist ein Reducer: eine pure Function, die den aktuellen Zustand und ein Ereignis entgegennimmt und einen neuen Zustand plus ausgehende Nachrichten zurueckgibt.

```
(State, Event) → Result<(State, Vec<Nachricht>), ProzessFehler>
```

### States und Events als algebraische Datentypen

States und Events sind Rust-Enums mit Daten. Jeder Enum-Variant traegt genau die Informationen, die in diesem Zustand relevant sind. Unmoegliche Zustaende sind nicht darstellbar.

### Beispiel: GPKE Lieferantenwechsel (LFW)

```
Idle
	→ AnmeldungEmpfangen → AnmeldungEingegangen
	→ AnmeldungBestaetigt → AbmeldungAnLfaGesendet
	→ LfaHatBestaetigt → WiderspruchsfristLaeuft
	→ WiderspruchsfristAbgelaufen → Zugeordnet (Terminal)
```

Bei Ablehnung oder Fristueberschreitung: `→ Abgelehnt` (Terminal).

Der Reducer erzeugt bei bestimmten Uebergaengen automatisch Nachrichten — z.B. sendet der NB nach Bestaetigung sowohl eine Bestaetigung an den neuen Lieferanten als auch eine Abmeldung an den alten.

### Quittungsschicht als Decorator

Die Quittungsschicht (`mako-quittung`) sitzt vor jedem Reducer:

1. **CONTRL:** Syntaxpruefung der eingehenden EDIFACT-Nachricht
2. **APERAK:** Anwendungspruefung (IDs gueltig, Pruefidentifikator bekannt)
3. **Reducer:** Fachliche Verarbeitung

Bei Fehler in Schicht 1 oder 2 wird der Reducer nicht aufgerufen und der State bleibt unveraendert. Stattdessen erzeugt die Schicht eine Fehlermeldung (CONTRL- oder APERAK-Nachricht).

### Versionierung

Jeder Prozess existiert in mehreren Formatversionen:

```rust
mako_gpke::v2025::lfw::reduce(state, event)
mako_gpke::v2022::lfw::reduce(state, event)
mako_gpke::v2020::lfw::reduce(state, event)
mako_gpke::v2017::lfw::reduce(state, event)
```

Alle Versionen existieren parallel. Das ermoeglicht Tests gegen historische Nachrichten und den Betrieb waehrend Uebergangsfristen.

---

## 5. EDIFACT-Codec

Der Codec (`mako-codec`) wandelt zwischen EDIFACT-Strings und typisierten Rust-Structs um. Fuer Redispatch 2.0 werden zusaetzlich 9 XML-Dokumenttypen unterstuetzt.

### Lexer: `parse_segments()`

Zeichenweise Tokenisierung des EDIFACT-Strings. Erkennt die vier Separatoren (`'` Segment, `+` Element, `:` Komponente, `?` Escape) und erzeugt `Vec<Segment>`. Jedes `Segment` besteht aus einem Tag und einer Liste von `Element`s, die wiederum aus Komponenten bestehen.

### Parser: `parse_interchange()`

Verarbeitet die UNB/UNH/UNT/UNZ-Struktur und erzeugt ein `Interchange` mit Sender, Empfaenger und einer Liste von `EdifactNachricht`s. Jede Nachricht traegt ihren Typ (z.B. `UTILMD`), ihre Version und die inneren Segmente.

### Typed Dispatch: `parse_nachricht()`

Erkennt den Nachrichtentyp aus dem UNH-Segment, den Pruefidentifikator aus RFF+Z13 und den BGM-Qualifier. Routet zum richtigen Parser:

```rust
match msg.typ.as_str() {
	"UTILMD" => parse_utilmd(...),
	"MSCONS" => parse_mscons(...),
	"ORDERS" => parse_orders(...),
	// ... 12 Nachrichtentypen total
	other => Err(CodecFehler::UnbekannterNachrichtentyp { .. }),
}
```

Das Ergebnis ist eine vollstaendig typisierte `Nachricht` mit validiertem Envelope und Payload.

### Serializer: `serialize_nachricht()`

Spiegelbildlich zum Parser. Nimmt eine typisierte `Nachricht` entgegen und erzeugt einen gueltig formatierten EDIFACT-String inkl. UNB/UNH/UNT/UNZ-Rahmen.

### XML: `parse_xml()` / `serialize_xml()`

Fuer Redispatch 2.0 existieren 9 XML-Dokumenttypen: Stammdaten, Fahrplan, Aktivierung, Bestaetigung, Engpass, Nichtverfuegbarkeit, Kostenblatt, StatusRequest, Kaskade. Basiert auf `quick-xml` mit Serde-Integration.

### Roundtrip-Garantie

Fuer alle 52 Fixtures gilt:

```
parse(serialize(parse(x))) == parse(x)
```

Jede Fixture wird automatisch gegen diese Eigenschaft getestet. Wenn der Roundtrip bricht, bricht der Build.

---

## 6. Testkorpus

Der Testkorpus (`mako-testdata`) ist das Rueckgrat der Korrektheitsnachweise. Er besteht aus vier Saeulen.

### 52 Fixtures

Jede Fixture besteht aus einem EDIFACT-`&str` (oder XML-String) und dem erwarteten Rust-Struct. Pro Fixture existieren zwei Tests: ein Parse-Test (EDIFACT → Struct, Vergleich mit Erwartung) und ein Roundtrip-Test (parse → serialize → re-parse, Vergleich).

- **39 Payload-Fixtures (EDIFACT):** Alle 39 EDIFACT-basierten NachrichtenPayload-Typen (UTILMD, MSCONS, ORDERS, ORDRSP, REQOTE, QUOTES, PRICAT, INVOIC, REMADV, IFTSTA, PARTIN, UTILTS)
- **4 Quittungs-Fixtures:** CONTRL positiv/negativ, APERAK positiv/negativ
- **9 XML-Fixtures:** Alle Redispatch 2.0-Dokumenttypen

### 39 Generatoren

Segment-Builder (pure Functions) plus Params-Structs mit `Default`-Implementierung. Jede `erzeuge_*`-Funktion nimmt einen Params-Struct und liefert einen gueltigen EDIFACT-String. Kurzform-Funktionen wie `anmeldung()` liefern sofort eine gueltige Nachricht mit Default-Parametern.

```rust
use mako_testdata::generator::edifact::anmeldung;

let edi: String = anmeldung(); // gueltige UTILMD-Anmeldung
```

Die Generatoren sind die Grundlage fuer den Fehler-Injektor und die Kommunikationsketten.

### Fehler-Injektor

Der `FehlerArt`-Enum definiert 8 Fehlervarianten auf drei Erkennungsschichten:

| Schicht | FehlerArt | Wirkung |
|---|---|---|
| Syntax (CONTRL) | `AbsenderLeer` | NAD+MS MP-ID leer |
| Syntax (CONTRL) | `EmpfaengerLeer` | NAD+MR MP-ID leer |
| Syntax (CONTRL) | `SegmentFehlt(tag)` | Segment komplett entfernt |
| Syntax (CONTRL) | `FalscheSegmentzahl` | UNT-Zaehler auf 99 |
| Anwendung (APERAK) | `UngueltigeMaLoId` | Luhn-Pruefsumme falsch |
| Anwendung (APERAK) | `UngueltigeMarktpartnerId` | 12 statt 13 Stellen |
| Anwendung (APERAK) | `FalscherPruefIdentifikator` | RFF+Z13 → 99999 |
| Fachlich (Reducer) | `FristInVergangenheit` | Lieferbeginn 2020-01-01 |

`injiziere_fehler()` ist eine pure Function — das Original bleibt unveraendert:

```rust
let valid = anmeldung();
let kaputt = injiziere_fehler(&valid, &FehlerArt::AbsenderLeer);
assert_ne!(valid, kaputt); // Original unveraendert
```

### 15 Kommunikationsketten

Vollstaendige Prozessdurchlaeufe, jeder als benannte Sequenz von EDIFACT-Nachrichten:

1. GPKE LFW Happy Path
2. GPKE LFW Ablehnung
3. GPKE LFW Fristueberschreitung
4. GPKE Lieferende
5. GPKE Stammdatenaenderung
6. WiM MSB-Wechsel
7. WiM Zaehlwertuebermittlung
8. UBP Bestellung
9. MaBiS Bilanzkreiszuordnung
10. Abrechnung Netznutzung
11. RD 2.0 Abruf
12. §14a Steuerung
13. GeLi Gas LFW
14. GABi Gas Nominierung
15. KoV Brennwertmitteilung

Jede Kette wird automatisch geprueft: Parse → Payload-Validierung → Roundtrip fuer jeden Schritt.

### Schnellstart

```bash
cargo test --workspace
```

Muss 566+ Tests gruen zeigen (39 Test-Suites). Bei Fehlern: der Build ist rot, die Fehlermeldung zeigt exakt die Fixture oder den Reducer-Uebergang, der bricht.

---

## 7. Simulator bedienen

### 7.1 CLI-Kommandos

Der MaKo-Simulator wird über die Kommandozeile (`mako-cli`) und ein Web-UI (`mako-ui`) bedient. Die CLI ist das Fundament — sie verarbeitet Nachrichten, das Web-UI ist die grafische Oberfläche darüber.

**Markt initialisieren:**

```bash
cargo build -p mako-cli
cargo run -p mako-cli -- init markt/
```

Erzeugt `markt/` mit sechs Rollen-Verzeichnissen (je `inbox/`, `outbox/`, `state.json`), `rollen.json` und `log/`.

**Nachricht verarbeiten:**

```bash
cargo run -p mako-cli -- verarbeite markt/netzbetreiber/inbox/001_UtilmdAnmeldung.json --markt markt
```

Pipeline: Datei lesen → CONTRL-Prüfung → APERAK-Prüfung → Reducer-Dispatch → ausgehende Nachrichten in `outbox/` schreiben → `.status.json` aktualisieren.

**Nachricht senden (Rolle → Rolle):**

```bash
cargo run -p mako-cli -- sende lieferant_neu netzbetreiber 001_UtilmdAnmeldung.json --markt markt
```

Kopiert die Datei von `lieferant_neu/outbox/` nach `netzbetreiber/inbox/`, setzt `zugestellt`-Zeitstempel in `.status.json`.

**Alle unverarbeiteten Nachrichten verarbeiten:**

```bash
cargo run -p mako-cli -- verarbeite-alle netzbetreiber --markt markt
```

**Status anzeigen:**

```bash
cargo run -p mako-cli -- status markt
```

Zeigt pro Rolle: Anzahl Inbox/Outbox-Nachrichten und aktive Prozesse mit aktuellem Zustand.

### 7.2 Web-UI starten

Das Web-UI besteht aus einem Express-Backend (Port 3001) und einem Vite-Frontend (Port 5173):

```bash
cd mako-ui
bun install           # einmalig
bun run server        # Terminal 1: Express-Backend
bun run dev           # Terminal 2: Vite-Devserver
```

Dann im Browser http://localhost:5173 öffnen.

### 7.3 UI-Aufbau

Das UI hat drei Spalten und eine Fußleiste:

| Bereich | Inhalt |
|---------|--------|
| **Links** | Aufgaben-Queue (offene Aktionen) + Prozessliste (nach Kategorie) |
| **Mitte** | Inbox/Outbox der aktiven Rolle mit Nachrichtenkarten |
| **Rechts** | Nachrichtendetail (JSON, EDIFACT, Status) oder Sendeformular |
| **Unten** | Prozess-Timeline: alle Schritte mit Absender→Empfänger |

**Rollen-Tabs:** Am oberen Rand. Jeder Tab zeigt den Rollennamen und ein Badge mit der Anzahl unverarbeiteter Inbox-Nachrichten.

**Workflow:**
1. Prozess in der linken Spalte auswählen (z.B. „Lieferantenwechsel")
2. Rechte Spalte zeigt Sendeformular mit MaLo-ID und Schritt-Auswahl
3. „Senden" schickt die Nachricht an den Empfänger
4. Tab des Empfängers wechseln → Nachricht erscheint in der Inbox
5. Nachricht anklicken → Detail-Ansicht mit „Verarbeiten"-Button
6. Nach Verarbeitung: CONTRL/APERAK in Absender-Inbox, Folgenachrichten in Outbox

### 7.4 GPKE Lieferantenwechsel — Schritt für Schritt

1. Tab „Lieferant Neu" → Prozess „Lieferantenwechsel" → Senden: Anmeldung → NB
2. Tab „Netzbetreiber" → Inbox: Anmeldung anklicken → „Verarbeiten"
3. NB Outbox: Bestätigung + Abmeldung erscheinen
4. „Senden" der Bestätigung → LFN, „Senden" der Abmeldung → LFA
5. Tab „Lieferant Alt" → Inbox: Abmeldung verarbeiten
6. Tab „Netzbetreiber" → Zuordnungen senden → LFN + LFA

---

## 8. Prozess-Referenz

### 8.1 GPKE Lieferantenwechsel (Strom)

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Anmeldung | LFN | NB | UTILMD (UtilmdAnmeldung) |
| 2 | Bestätigung | NB | LFN | UTILMD (UtilmdBestaetigung) |
| 3 | Abmeldung an LFA | NB | LFA | UTILMD (UtilmdAbmeldung) |
| 4 | Widerspruchsfrist | LFA | NB | (intern/Fristablauf) |
| 5 | Zuordnung an LFN | NB | LFN | UTILMD (UtilmdZuordnung) |
| 6 | Zuordnung an LFA | NB | LFA | UTILMD (UtilmdZuordnung) |

### 8.2 GPKE Lieferende (Strom)

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Abmeldung | LFA | NB | UTILMD (UtilmdLieferendeAbmeldung) |
| 2 | Bestätigung | NB | LFA | UTILMD (UtilmdLieferendeBestaetigung) |
| 3 | Schlussturnusmesswert | MSB | NB | MSCONS |

### 8.3 GPKE Stammdatenänderung

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Änderung senden | NB | LFN | UTILMD (UtilmdStammdatenaenderung) |
| 2 | Bestätigung/Ablehnung | LFN | NB | UTILMD |

### 8.4 GPKE Geschäftsdatenanfrage

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Anfrage | LFN | NB | UTILMD (UtilmdGeschaeftsdatenanfrage) |
| 2 | Antwort | NB | LFN | UTILMD (UtilmdGeschaeftsdatenantwort) |

### 8.5 WiM MSB-Wechsel

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Anmeldung MSB neu | MSB | NB | UTILMD (UtilmdMsbWechselAnmeldung) |
| 2 | Abmeldung MSB alt | NB | MSB | UTILMD (UtilmdAbmeldung) |
| 3 | Bestätigung | NB | MSB | UTILMD (UtilmdBestaetigung) |

### 8.6 UBP Bestellung Messprodukt

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Angebotsanfrage | LFN | MSB | REQOTE |
| 2 | Angebot | MSB | LFN | QUOTES |
| 3 | Bestellung | LFN | MSB | ORDERS |
| 4 | Bestellantwort | MSB | LFN | ORDRSP |

### 8.7 MaBiS Bilanzkreiszuordnung

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Zuordnung | LFN | NB | UTILMD (UtilmdBilanzkreiszuordnung) |
| 2 | Bestätigung | NB | LFN | UTILMD |

### 8.8 Abrechnung Netznutzung

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Rechnung | NB | LFN | INVOIC |
| 2 | Zahlungsavis | LFN | NB | REMADV |

### 8.9 RD 2.0 Redispatch-Abruf

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Aktivierung | NB | LFN | XML (RdAktivierung) |
| 2 | Quittierung | LFN | NB | XML (RdQuittung) |

### 8.10 §14a Steuerung

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Anmeldung SVE | NB | MSB | UTILMD |
| 2 | Steuersignal | NB | MSB | CLS |

### 8.11 GeLi Gas Lieferantenwechsel

Gleicher Ablauf wie GPKE LFW (Abschnitt 8.1), mit Gas-spezifischen Fristen (10WT/5WT/3WT) und Gastag-Bezug.

### 8.12 GABi Gas Nominierung

| # | Schritt | Absender | Empfänger | Nachrichtentyp |
|---|---------|----------|-----------|----------------|
| 1 | Nominierung | BKV | MGV | MSCONS |
| 2 | Bestätigung | MGV | BKV | MSCONS |

---

## 9. Glossar

| Abkürzung | Bedeutung |
|-----------|-----------|
| **MaLo** | Marktlokation — ein Entnahmepunkt im Stromnetz, identifiziert durch eine 11-stellige Nummer |
| **MeLo** | Messlokation — ein physischer Zählerstandort, identifiziert durch eine 33-stellige Nummer |
| **MP-ID** | Marktpartner-ID — 13-stellige Identifikationsnummer eines Marktteilnehmers (BDEW-Codenummer) |
| **Sparte** | Strom oder Gas — bestimmt Fristen, Formate und Prozessvarianten |
| **EDIFACT** | Electronic Data Interchange for Administration, Commerce and Transport — UN-Nachrichtenstandard für den B2B-Datenaustausch in der Energiewirtschaft |
| **UTILMD** | Utility Master Data — EDIFACT-Nachrichtentyp für Stammdaten (An-/Abmeldung, Zuordnung, Geschäftsdatenanfrage) |
| **MSCONS** | Metered Services Consumption — EDIFACT-Nachrichtentyp für Messwerte und Zeitreihen |
| **INVOIC** | Invoice — EDIFACT-Nachrichtentyp für Rechnungen |
| **REMADV** | Remittance Advice — EDIFACT-Nachrichtentyp für Zahlungsavise |
| **ORDERS** | Purchase Order — EDIFACT-Nachrichtentyp für Bestellungen |
| **ORDRSP** | Purchase Order Response — EDIFACT-Nachrichtentyp für Bestellantworten |
| **REQOTE** | Request for Quote — EDIFACT-Nachrichtentyp für Angebotsanfragen |
| **QUOTES** | Quote — EDIFACT-Nachrichtentyp für Angebote |
| **CONTRL** | Syntaxkontrollnachricht — automatische Quittung auf EDIFACT-Ebene (Syntax ok/fehlerhaft) |
| **APERAK** | Application Error and Acknowledgement — inhaltliche Quittung (Geschäftsregeln ok/verletzt) |
| **Reducer** | `(State, Event) → (State, Vec<Nachricht>)` — pure Zustandsübergangsfunktion pro Kommunikationslinie |
| **PID** | Prüfidentifikator — 5-stellige Nummer, identifiziert einen spezifischen EDIFACT-Nachrichteninhalt im AHB |
| **AHB** | Anwendungshandbuch — beschreibt, welche Segmente/Datenelemente für einen PID-Wert zu befüllen sind |
| **MIG** | Message Implementation Guide — technische Spezifikation eines EDIFACT-Nachrichtentyps |
| **EBD** | Entscheidungsbaum-Diagramm — beschreibt die Geschäftslogik für Prüfschritte (z.B. „Anmeldung akzeptieren oder ablehnen") |
| **GPKE** | Geschäftsprozesse zur Kundenbelieferung mit Elektrizität |
| **GeLi Gas** | Geschäftsprozesse Lieferantenwechsel Gas |
| **WiM** | Wechselprozesse im Messwesen |
| **UBP** | Übermittlung von Bestellprodukten |
| **MaBiS** | Marktregeln für die Durchführung der Bilanzkreisabrechnung Strom |
| **GABi Gas** | Geschäftsabwicklung Bilanzierung Gas |
| **KoV** | Kooperationsvereinbarung Gas |
| **RD 2.0** | Redispatch 2.0 — Engpassmanagement im Stromnetz |
| **LFN** | Lieferant Neu — der neue Stromlieferant in einem Wechselprozess |
| **LFA** | Lieferant Alt — der bisherige Stromlieferant |
| **NB** | Netzbetreiber — Verteilnetzbetreiber (VNB) |
| **MSB** | Messstellenbetreiber — verantwortlich für Zähler und Messwerte |
| **BKV** | Bilanzkreisverantwortlicher — verantwortlich für die Bilanzkreisabrechnung |
| **MGV** | Marktgebietsverantwortlicher — verantwortlich für ein Gasmarktgebiet |
| **FNB** | Fernleitungsnetzbetreiber — Betreiber des Gasfernleitungsnetzes |
