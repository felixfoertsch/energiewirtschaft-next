# MaKo-Verifikationssystem: Dreischichtige Qualitätssicherung mit externen Referenzdaten

**Datum:** 2026-03-28
**Status:** Entwurf
**Vorgänger:** `2026-03-25-mako-simulator-design.md` (implementiert, 580 Tests, 10.526-Nachrichten-Simulation)
**Ziel:** Ein Verifikationssystem, das die Korrektheit unserer MaKo-Engine unabhängig nachweist. Domänenexperten (Nicht-Entwickler) können über die Web-UI schrittweise oder im Batch prüfen, ob unsere EDIFACT-Nachrichten den offiziellen AHB-Regeln entsprechen, ob unsere Prozess-Reducer die richtigen Entscheidungen treffen (EBD), und ob andere Systeme unsere Nachrichten lesen können.

---

## 1  Scope & Abgrenzung

### Im Scope

- **`mako-verify` Crate** — neues Rust-Crate im Workspace, lädt offizielle Referenzdaten (AHB-JSON, EBD-JSON) und validiert unsere Nachrichten dagegen
- **Schicht 1: Nachrichtenvalidierung (AHB)** — Feld-für-Feld-Prüfung jeder EDIFACT-Nachricht gegen die offizielle Anwendungshandbuch-Tabelle des jeweiligen Prüfidentifikators
- **Schicht 2: Prozessvalidierung (EBD)** — Vergleich unserer Reducer-Entscheidungen mit den offiziellen Entscheidungsbaumdiagrammen
- **Schicht 3: Codec-Interoperabilität** — Kreuzvalidierung unserer Nachrichten mit einem unabhängigen Drittanbieter-Parser (STROMDAO `edifact-to-json-transformer`)
- **CLI-Erweiterung** — `mako verifiziere <datei>` und `mako verifiziere-batch <sim-verzeichnis>`
- **UI-Integration** — Schrittmodus (Verifikation nach jedem Prozessschritt) und Batch-Modus (gesamte Simulation) in mako-ui
- **Referenzdaten** — Flat-AHB-JSONs und EBD-JSONs aus dem Hochfrequenz-Ökosystem, im Repo mitgeliefert
- **Zielversion:** FV2504 (eine Formatversion gleichzeitig)

### Außerhalb Scope

- Vollständige AHB-Bedingungsauswertung (siehe Abschnitt 3.3 für den pragmatischen Ansatz)
- Semantische Interpretation von EBD-Fragen (siehe Abschnitt 4.3 für den Binding-Mechanismus)
- Automatisches Herunterladen/Aktualisieren der Referenzdaten von edi-energy.de
- BDEW-XML-basierte Validierung (erfordert kostenpflichtiges Abonnement, `fundamend`-Integration kommt ggf. später)
- AS4-Transportschicht-Tests (Schleupen Fakeserver — separates Thema)
- Produktionsreife der UI-Komponenten (Fehlerbehandlung, Robustheit)

### Beziehung zu bestehender Infrastruktur

`mako-verify` nutzt die bestehende `mako-testdata`-Infrastruktur (59 Fixtures, 39 Generatoren) für:
- Known-Good-Nachrichten als Referenz für Positivtests
- Known-Bad-Nachrichten (via `fehler.rs` Fehler-Injektor) für Negativtests
- Kommunikationsketten für Prozesstests

`mako-verify` nutzt `mako-codec` zum Parsen der Nachrichten, aber validiert unabhängig dagegen.

---

## 2  Architektur

### 2.1  Systemübersicht

```
┌─────────────────────────────────────────────────────────┐
│                      mako-ui (React)                     │
│  Schrittmodus  │  Batch-Bericht  │  Verifikations-Badges│
└──────────────┬──────────────────────┬───────────────────┘
               │                      │
               ▼                      ▼
┌──────────────────────────┐  ┌───────────────────────────┐
│   UNSER SYSTEM (besteht) │  │  VERIFIKATION (neu)       │
│                          │  │                           │
│  mako-cli                │  │  mako-verify (Rust)       │
│  mako-codec              │  │    Schicht 1: AHB-Regeln  │
│  mako-quittung           │  │    Schicht 2: EBD-Bäume   │
│  mako-gpke, mako-wim,   │  │    Schicht 3: Codec-Interop│
│    ... (Prozess-Reducer) │  │                           │
│  mako-testdata           │  │  Kreuzvalidator (Node.js) │
│  mako-sim (Simulation)   │  │    edifact-to-json-       │
│                          │  │    transformer (STROMDAO) │
│  → erzeugt EDIFACT-Msgs  │  │                           │
└────────────┬─────────────┘  └────────────┬──────────────┘
             │    Nachrichten fließen zu    │
             └────────────────────────────→┘
                                           │
                              ┌────────────▼──────────────┐
                              │  Referenzdaten/           │
                              │  Hochfrequenz Flat-AHB    │
                              │  JSONs (FV2504)           │
                              │  EBD-Entscheidungsbaum-   │
                              │  JSONs (FV2504)           │
                              │  (im Repo mitgeliefert)   │
                              └───────────────────────────┘
```

### 2.2  Datenfluss

1. Experte löst einen Prozessschritt aus (z.B. "NB verarbeitet Anmeldung")
2. Unser System (mako-cli/codec/reducer) erzeugt EDIFACT-Nachrichten
3. `mako-verify` prüft jede erzeugte Nachricht:
   - Schicht 1: Felder gegen AHB-Tabelle des Prüfidentifikators
   - Schicht 2: Reducer-Entscheidung gegen EBD-Baum
   - Schicht 3: Nachricht an Kreuzvalidator (falls aktiv)
4. Ergebnisse erscheinen in der UI (inline oder als Bericht)

---

## 3  Schicht 1: Nachrichtenvalidierung (AHB)

### 3.1  Fragestellung

"Ist jede EDIFACT-Nachricht, die wir erzeugen, korrekt gemäß dem offiziellen Anwendungshandbuch?"

### 3.2  Funktionsweise

1. Unsere Engine erzeugt eine EDIFACT-Nachricht (z.B. UTILMD Anmeldung mit Prüfidentifikator 11001)
2. `mako-verify` lädt die Flat-AHB-JSON für diesen Prüfidentifikator
3. Durchläuft die Nachricht Segment für Segment und prüft:
   - **Pflichtfelder vorhanden** — AHB sagt "Muss" → Feld muss existieren
   - **Verbotene Felder abwesend** — AHB sagt "X" → Feld darf nicht existieren
   - **Wertpool korrekt** — AHB gibt erlaubte Codewerte vor → unser Wert muss übereinstimmen
   - **Format korrekt** — AHB gibt Formatbeschränkungen vor (z.B. `an..35`, `n..15`) → Länge und Typ prüfen
   - **Bedingte Felder** — siehe 3.3

### 3.3  Pragmatischer Ansatz für Bedingungsausdrücke

AHB-Bedingungsausdrücke wie `Muss [123] U [456]` bestehen aus zwei Teilen:

1. **Boolesche Logik** — die Verknüpfung (`U`=und, `O`=oder, `X`=exklusiv-oder, `UB`=sofern nicht)
2. **Bedingungszustand** — ob Bedingung `[123]` wahr oder falsch ist (erfordert Domänenwissen über den Nachrichteninhalt)

**V1-Ansatz (im Scope):**

- Boolesche Logik parsen wir selbst (die Grammatik ist endlich und dokumentiert)
- Bedingungszustand: wir implementieren eine **statisch auswertbare Teilmenge** der Bedingungen — diejenigen, deren Zustand sich direkt aus der Nachricht ableiten lässt:
  - `[931]` "Wenn SG4 IDE+24 (MeLo-ID) vorhanden" → Segment-Existenzprüfung
  - `[932]` "Wenn SG4 LOC+172 (Lokation) vorhanden" → Segment-Existenzprüfung
  - `[492]` "Wenn Sparte = Strom" → Codewert-Prüfung in SG2 NAD
  - usw.
- Bedingungen, die externen Zustand erfordern (z.B. "Wenn Marktlokation in Niederspannung"), werden als **unbestimmt** markiert
- Unbestimmte Bedingungen → Feld wird als **gelb** (nicht prüfbar) statt grün/rot angezeigt

**Späterer Ausbau (außerhalb Scope):**

- Integration mit `ahbicht` (Python REST-API oder lokaler Aufruf) für vollständige Bedingungsauswertung
- Eigene Bedingungstabelle, die alle ~500 Bedingungsnummern auf programmatische Prüfungen abbildet

### 3.4  Referenzdaten

Flat-AHB-JSONs aus Hochfrequenz `machine-readable_anwendungshandbuecher`:

```
mako-verify/referenzdaten/ahb/FV2504/
├── UTILMD/
│   ├── 11001.json   (Anmeldung LFW Strom)
│   ├── 11002.json   (Bestätigung LFW)
│   ├── 11003.json   (Ablehnung LFW)
│   └── ...
├── MSCONS/
│   ├── 13002.json   (Schlussturnusmesswert)
│   └── ...
├── ORDERS/
├── INVOIC/
└── ...
```

Format pro Prüfidentifikator:

```json
{
  "lines": [
    {
      "ahb_expression": "Muss",
      "conditions": "",
      "data_element": "0062",
      "name": "Nachrichten-Referenznummer",
      "section_name": "Nachrichten-Kopfsegment",
      "segment_code": "UNH",
      "segment_group_key": null,
      "value_pool_entry": null
    }
  ]
}
```

### 3.5  Ansicht für den Experten

Tabelle pro Nachricht:

| Segment | Feld | AHB-Regel | Unser Wert | Ergebnis |
|---------|------|-----------|------------|----------|
| UNH | 0062 Nachrichten-Referenznr. | Muss | 00001 | ✓ |
| BGM | 1004 Dokumentennummer | Muss | 11001 | ✓ |
| DTM+137 | 2380 Datum/Zeit | Muss | 20260701 | ✓ |
| NAD+MS | 3039 MP-ID Absender | Muss | 9900000000000 | ✓ |
| SG4 IDE+24 | 7140 MeLo-ID | Muss [931] | DE00012345678901234567890123456 | ○ (Bedingung nicht auswertbar) |

Grün = bestanden, rot = fehlgeschlagen, gelb = bedingt/nicht prüfbar.

### 3.6  Verhalten bei fehlenden Referenzdaten

Wenn für einen Prüfidentifikator keine AHB-JSON existiert:
- Schicht 1 meldet "Keine Referenzdaten für PI {id}" als Warnung
- Nachricht wird nicht als fehlgeschlagen gewertet, sondern als **nicht prüfbar**
- Im Batch-Bericht erscheint sie in einer separaten Kategorie "Nicht verifizierbar"

---

## 4  Schicht 2: Prozessvalidierung (EBD)

### 4.1  Fragestellung

"Hat unser State-Machine-Reducer an jedem Schritt die richtige Entscheidung getroffen?"

### 4.2  Funktionsweise

Die offiziellen EBD definieren exakt, was eine Marktrolle beim Empfang einer Nachricht prüfen muss. Beispiel E_0401 (NB empfängt LFW-Anmeldung):

1. "Ist die MaLo-ID dem NB bekannt?" → falls nein → Ablehnung mit Code A01
2. "Ist das Lieferbeginn-Datum zulässig?" → falls nein → Ablehnung mit Code A02
3. "Liegt ein aktiver Lieferant vor?" → falls ja → Abmeldung an LFA senden
4. usw.

Unsere Reducer (mako-gpke, etc.) implementieren diese Entscheidungsbäume. `mako-verify` vergleicht das Ergebnis.

### 4.3  Binding-Mechanismus: Ergebnisorientierter Vergleich

Die EBD-Fragen sind natürlichsprachlich ("Ist die MaLo-ID bekannt?") und erfordern semantische Interpretation. Vollautomatisches Durchlaufen der EBD-Bäume mit realen Nachrichtendaten ist daher **außerhalb Scope für V1**.

**V1-Ansatz (im Scope): Ergebnisorientierter Vergleich**

Statt jeden EBD-Schritt automatisch auszuwerten, vergleichen wir das **Endergebnis** unseres Reducers mit den **möglichen Endergebnissen** des EBD:

1. Experte löst Prozessschritt aus (z.B. NB empfängt Anmeldung)
2. Unser Reducer erzeugt Output (z.B. Bestätigung + Abmeldung an LFA)
3. `mako-verify` prüft:
   - Ist unser Output ein **gültiges Endergebnis** des EBD? (z.B. "Bestätigung senden" ist ein valider Ausgang von E_0401)
   - Stimmt der **Antwortcode** überein? (z.B. unser Reducer lehnt ab mit A01 → E_0401 hat A01 als Ausgang für "MaLo unbekannt")
   - Sind die **erzeugten Folgenachrichten** konsistent mit dem EBD-Ausgang?

4. Der Experte sieht den **vollständigen EBD-Baum** zur Referenz und kann manuell nachvollziehen, ob der Pfad korrekt ist:

```
E_0401: Anmeldung prüfen (Netzbetreiber)
─────────────────────────────────────────
1. MaLo-ID bekannt?              [manuell prüfbar]
2. Lieferbeginn zulässig?        [manuell prüfbar]
3. Aktiver Lieferant vorhanden?  [manuell prüfbar]
   ───────────────────────────────────────────
   Unser Ergebnis: Bestätigung + Abmeldung an LFA
   EBD-Ausgang:    ✓ gültiger Ausgang (Schritt 3, Ja-Pfad)
```

**Späterer Ausbau (außerhalb Scope):**

- Pro EBD-Schritt eine programmatische Prüffunktion hinterlegen (manuelles Mapping von ~300 EBDs × N Schritte)
- Vollautomatisches Durchlaufen des Baums mit gebundenen Nachrichtendaten

### 4.4  Referenzdaten

EBD-JSONs aus Hochfrequenz `machine-readable_entscheidungsbaumdiagramme`:

```
mako-verify/referenzdaten/ebd/FV2504/
├── E_0401.json
├── E_0402.json
├── E_0453.json
└── ...
```

Format:

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
      "description": "Ist die MaLo-ID dem NB bekannt?",
      "sub_rows": [
        { "check_result": { "result": false }, "result_code": "A01", "note": "MaLo unbekannt" },
        { "check_result": { "result": true, "subsequent_step_number": "2" } }
      ]
    }
  ]
}
```

### 4.5  Verhalten bei fehlenden Referenzdaten

Analog zu Schicht 1: Warnung, nicht als Fehler gewertet, separate Kategorie im Bericht.

---

## 5  Schicht 3: Codec-Interoperabilität

### 5.1  Fragestellung

"Können andere Systeme unsere Nachrichten lesen, und können wir Nachrichten anderer Systeme lesen?"

### 5.2  Richtung A: Unsere Nachrichten → Drittanbieter-Parser

EDIFACT-Nachrichten aus mako-codec an STROMDAOs `edifact-to-json-transformer` (Node.js) senden:
- Parst fehlerfrei?
- Extrahierte Daten (MaLo-ID, Daten, Rollen, etc.) stimmen mit unserer Intention überein?

### 5.3  Richtung B: Drittanbieter-Nachrichten → Unser Parser

Test-Nachrichten aus STROMDAOs eingebauten Generatoren (`generateTestUTILMD()`, `generateTestMSCONS()`, etc.) an mako-codec senden:
- Unser Parser verarbeitet sie fehlerfrei?
- Roundtrip: parse → serialize → parse → identisches Ergebnis?

### 5.4  Bekannte Einschränkungen des Kreuzvalidators

`edifact-to-json-transformer` (npm: `edifact-json-transformer`, MIT-Lizenz):
- **Unterstützte Nachrichtentypen:** UTILMD, MSCONS, ORDERS/ORDRSP, INVOIC/REMADV, APERAK/CONTRL
- **Nicht unterstützt:** REQOTE, QUOTES, PRICAT, IFTSTA, PARTIN, UTILTS, RD 2.0 XML
- **Validierungstiefe:** Grundlegend (Prüfidentifikator-Existenz, Rollenvalidierung, ID-Formate) — kein vollständiger AHB-Auswerter
- **FV-Kompatibilität:** Nicht explizit FV2504-spezifisch; Grundstruktur sollte kompatibel sein, Details müssen in Phase A validiert werden

Nachrichten ohne Kreuzvalidator-Unterstützung werden als "nicht kreuzvalidierbar" markiert (gelb, nicht rot).

### 5.5  Ansicht für den Experten

Vergleichstabelle:

```
Nachricht: 001_utilmd_anmeldung.edi
──────────────────────────────────
              Unser Parser  STROMDAO-Parser
MaLo-ID:     51238696788   51238696788      ✓
Lieferbeginn: 2026-07-15   2026-07-15       ✓
Absender:     9900000000000 9900000000000   ✓
Empfänger:    9900000000001 9900000000001   ✓
Roundtrip:    ✓             n/a
Parse OK:     ✓             ✓
```

### 5.6  Laufzeitumgebung

Der Node.js-Kreuzvalidator läuft als kleiner Sidecar-Prozess neben dem Express-Backend. Optional — das System funktioniert ohne ihn, aber mit ihm erhält man die unabhängige zweite Meinung.

---

## 6  UI-Integration

### 6.1  Schrittmodus (detaillierte Exploration)

Der Experte wählt einen Prozess (z.B. Lieferantenwechsel) und geht Schritt für Schritt durch:

1. Aktuellen Zustand und verfügbare Aktionen sehen
2. Nächsten Schritt auslösen (z.B. "NB verarbeitet Anmeldung")
3. Unsere Engine erzeugt die Ausgangsnachrichten
4. Verifikation läuft automatisch auf jeder erzeugten Nachricht
5. Ergebnisse erscheinen inline — ein Verifikations-Panel neben der Nachrichtenansicht:
   - **AHB-Tab:** aufklappbare Tabelle mit Feld-für-Feld-Ergebnissen
   - **EBD-Tab:** Entscheidungsbaum mit Pfadhervorhebung und Ergebnisvergleich
   - **Codec-Tab:** Kreuzparser-Vergleich (falls Sidecar aktiv)
   - **Gesamturteil:** grün/gelb/rot Badge auf der Nachricht

### 6.2  Batch-Modus (Gesamtabdeckung)

Experte klickt "Simulation verifizieren" → führt die vollständige Netzgebiet-Rheinland-Simulation aus (oder Teilmenge) → alle Nachrichten werden verifiziert → Bericht:

- Zusammenfassung: X von Y Nachrichten haben alle drei Schichten bestanden
- Aufschlüsselung nach Prozesstyp, Nachrichtentyp, Marktrolle
- Liste der Fehler mit Drill-Down
- Exportierbar als HTML (wie der bestehende Simulationsbericht)

### 6.3  Verifikations-Badges

Jede Nachricht in der Nachrichtenliste erhält ein Badge:

- **✓✓✓** — alle drei Schichten bestanden
- **✓✓○** — AHB + EBD bestanden, keine Kreuzvalidierung (Sidecar nicht aktiv)
- **✓✗✓** — AHB bestanden, EBD fehlgeschlagen, Codec bestanden → Prozesslogik-Problem
- **✗○○** — AHB fehlgeschlagen → Nachrichtenstruktur-Problem
- **○○○** — keine Referenzdaten verfügbar

---

## 7  Neue Komponenten

### 7.1  Rust-Crate: `mako-verify`

```
mako-verify/
├── Cargo.toml
├── referenzdaten/
│   ├── ahb/FV2504/...        (Flat-AHB-JSONs)
│   └── ebd/FV2504/...        (EBD-JSONs)
├── src/
│   ├── lib.rs                 (öffentliche API)
│   ├── ahb.rs                 (Schicht 1: AHB-Validierung)
│   ├── ahb_ausdruck.rs        (Bedingungsausdrücke parsen + auswerten)
│   ├── ebd.rs                 (Schicht 2: EBD-Ergebnisvergleich)
│   ├── interop.rs             (Schicht 3: Vorbereitung für Kreuzvalidierung)
│   ├── bericht.rs             (strukturierte Ergebnisse als JSON)
│   └── referenzdaten.rs       (Laden/Caching der JSON-Referenzdaten)
└── tests/
    ├── ahb_tests.rs
    ├── ebd_tests.rs
    └── interop_tests.rs
```

Öffentliche API:

```rust
pub fn verifiziere_nachricht(nachricht: &Nachricht, referenzdaten: &Referenzdaten)
    -> VerifikationsErgebnis;

pub fn verifiziere_prozess_schritt(
    eingabe: &Nachricht,
    ausgabe: &[Nachricht],
    reducer_zustand: &serde_json::Value,
    referenzdaten: &Referenzdaten,
) -> ProzessVerifikationsErgebnis;

pub fn verifiziere_batch(
    sim_verzeichnis: &Path,
    referenzdaten: &Referenzdaten,
) -> BatchErgebnis;
```

### 7.2  CLI-Erweiterung

```
mako verifiziere <datei>                    # einzelne Nachricht, alle drei Schichten
mako verifiziere-batch <sim-verzeichnis>    # alle Nachrichten einer Simulation
```

### 7.3  Node.js-Kreuzvalidator

`mako-ui/src/server/kreuzvalidator.ts`:

- `POST /api/kreuzvalidiere` — akzeptiert rohes EDIFACT, gibt geparste JSON-Felder zurück
- `POST /api/kreuz-generiere` — erzeugt Testnachrichten aus STROMDAO-Generatoren
- Vergleichslogik: Feld-für-Feld-Diff zwischen unserer Extraktion und ihrer

### 7.4  React-Komponenten

- **`VerifikationsPanel.tsx`** — Hauptanzeige mit Tabs: AHB | EBD | Codec
- **`VerifikationsBadge.tsx`** — kompaktes Badge für Nachrichtenliste (drei Indikatoren, farbkodiert)
- **`BatchBericht.tsx`** — Batch-Verifikationsergebnisse mit Zusammenfassung und Drill-Down
- **`EbdBaum.tsx`** — Entscheidungsbaum-Visualisierung mit Pfadhervorhebung

---

## 8  Abhängigkeiten

### Erforderlich (immer verfügbar)

- Flat-AHB-JSONs — statische Dateien, keine Laufzeitabhängigkeit
- EBD-JSONs — statische Dateien, keine Laufzeitabhängigkeit
- `mako-verify` Rust-Crate — kompiliert mit dem Workspace
- `mako-testdata` — bestehende Fixtures und Generatoren für Test-Nachrichten

### Optional (erweiterte Verifikation)

- Node.js + `edifact-json-transformer` npm-Paket — für Schicht 3 Kreuzvalidierung
- Nur erforderlich wenn Sidecar läuft; System degradiert gracefully ohne

---

## 9  Implementierungsreihenfolge

### Phase A: Referenzdaten & Grundlagen

1. Referenzdaten herunterladen und im Repo ablegen (Flat-AHB-JSONs FV2504, EBD-JSONs FV2504)
2. `mako-verify` Crate anlegen mit Referenzdaten-Lader
3. STROMDAO `edifact-to-json-transformer` Kompatibilität prüfen (Spike: unsere Nachrichten parsen lassen, Abdeckung dokumentieren)

### Phase B: Schicht 1 — AHB-Validierung

4. AHB-Feldprüfung implementieren (Muss/Verboten/Wertpool/Format)
5. Bedingungsausdrücke parsen (boolesche Logik)
6. Statisch auswertbare Bedingungen implementieren (Segment-Existenz, Codewert-Prüfung)
7. CLI `mako verifiziere <datei>` für Schicht 1
8. Tests: Known-Good-Nachrichten (aus mako-testdata Fixtures) → alle Felder bestehen
9. Tests: Known-Bad-Nachrichten (via Fehler-Injektor) → korrekter Fehler erkannt

### Phase C: Schicht 2 — EBD-Prozessvalidierung

10. EBD-JSON-Lader und Datenmodell
11. Ergebnisorientierter Vergleich: Reducer-Output gegen EBD-Ausgänge
12. Tests: GPKE LFW Happy Path → Ergebnis stimmt mit EBD überein
13. Tests: GPKE LFW Ablehnung → Antwortcode stimmt mit EBD überein

### Phase D: Schicht 3 — Kreuzvalidierung

14. Node.js-Sidecar implementieren (kreuzvalidator.ts)
15. Richtung A: Unsere Nachrichten → STROMDAO-Parser → Feldvergleich
16. Richtung B: STROMDAO-Generatoren → Unser Parser → Roundtrip-Test
17. Tests: Bidirektionale Kompatibilität für UTILMD, MSCONS, ORDERS

### Phase E: UI-Integration

18. VerifikationsBadge in MessageList integrieren
19. VerifikationsPanel in MessageDetail integrieren
20. EbdBaum-Komponente
21. Batch-Modus: "Simulation verifizieren"-Button + BatchBericht
22. API-Routen: `/api/verifiziere/:rolle/:box/:datei`, `/api/verifiziere-batch`

### Phase F: Gesamttest

23. Vollständige Simulation (10.526 Nachrichten) durch alle drei Schichten
24. Bericht generieren, Ergebnisse prüfen
25. Deliberately eingeführten Fehler vom Experten finden lassen (GG-10)

---

## 10  Guided Gates

- **GG-1:** `mako-verify` lädt Flat-AHB-JSON für Prüfidentifikator 11001 und listet erwartete Felder auf
- **GG-2:** `mako-verify` validiert eine Known-Good UTILMD Anmeldung (aus mako-testdata Fixture) → alle Felder bestehen
- **GG-3:** `mako-verify` validiert eine Nachricht mit fehlendem Pflichtfeld (SG2 NAD+MS) → korrekter Fehler "Pflichtfeld fehlt: NAD+MS/3039"
- **GG-4:** `mako-verify` lädt EBD E_0401 und listet alle gültigen Ausgänge auf
- **GG-5:** EBD-Vergleich für GPKE LFW Happy Path: Reducer-Ergebnis "Bestätigung + Abmeldung an LFA" wird als gültiger E_0401-Ausgang erkannt
- **GG-6:** Kreuzvalidator parst eine unserer UTILMD-Nachrichten und extrahiert identische MaLo-ID, Lieferbeginn, Absender, Empfänger
- **GG-7:** Unser Parser verarbeitet eine STROMDAO-generierte Test-UTILMD fehlerfrei und übersteht den Roundtrip
- **GG-8:** Schrittmodus in mako-ui zeigt VerifikationsPanel mit AHB-Ergebnissen nach Verarbeitung einer Anmeldung
- **GG-9:** Batch-Modus verifiziert die vollständige Simulation und erzeugt einen Zusammenfassungsbericht mit Aufschlüsselung nach Prozesstyp
- **GG-10:** Domänenexperte identifiziert ein absichtlich fehlendes Pflichtfeld (SG6 IDE+24 MeLo-ID in einer UTILMD Anmeldung) über den AHB-Tab in der UI, ohne rohes EDIFACT lesen zu müssen
