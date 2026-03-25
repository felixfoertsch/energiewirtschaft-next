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

*Sektionen 7–9 (CLI, Simulation, UI) folgen nach Implementierung in Task 10.*
