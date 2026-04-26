# Testkorpus Marktkommunikation – Quellen-Checkliste

Systematische Bestandsaufnahme öffentlich verfügbarer Beispiel- und Testdateien für alle Nachrichtentypen der deutschen energiewirtschaftlichen Marktkommunikation.

Stand: März 2026

**Legende:**
- ✅ = Öffentliche Beispieldaten oder Strukturdefinitionen verfügbar
- ⚠️ = Teilweise verfügbar (Snippets, Fragmente, oder nur Strukturdefinition ohne vollständige Beispielnachricht)
- ❌ = Keine öffentlichen Beispieldaten gefunden – muss selbst generiert werden
- 🔧 = Tooling vorhanden, mit dem Testdaten erzeugt werden können

---

## 1  Quellen-Register (Übergreifend)

### 1.1  Offizielle Regelwerke & Formatdefinitionen

| # | Quelle | URL | Inhalt |
|---|--------|-----|--------|
| Q1 | **BDEW MaKo-Plattform** | https://bdew-mako.de | Alle MIGs, AHBs, EBDs, Allgemeine Festlegungen – Formatdefinitionen (keine Beispielnachrichten) |
| Q2 | **BNetzA BK6 – Mitteilungen Datenformate** | https://www.bundesnetzagentur.de/DE/Beschlusskammern/BK06/BK6_83_Zug_Mess/835_mitteilungen_datenformate/ | Konsultationsfassungen aller EDIFACT/XML-Dokumente als PDF |
| Q3 | **ENTSO-E EDI Library** | https://www.entsoe.eu/publications/electronic-data-interchange-edi-library/ | XSD-Schemas für ActivationDocument, PlannedResourceSchedule etc. (Basis für RD 2.0 XML) |
| Q4 | **ENTSO-E GitLab XSD** | https://gitlab.entsoe.eu/transparency/xsd | XSD-Schemas der Transparency Platform |
| Q5 | **ENTSO-E Codelist ZIP** | https://www.entsoe.eu/Documents/EDI/Library/CodelistV71.zip | Aktuelle Codelisten als XSD |
| Q6 | **BDEW Anwendungshilfen** | https://www.bdew.de/service/anwendungshilfen/ | Umsetzungsfragenkataloge mit fachlichen Beispielen |
| Q7 | **BDEW Redispatch 2.0 News** | https://www.bdew.de/energie/redispatch-20-news/ | Fehlerkorrekturen, Konsultationsergebnisse XML |

### 1.2  Open-Source-Tooling & Repositories (Hochfrequenz-Ökosystem)

| # | Repository | URL | Relevanz für Testkorpus |
|---|-----------|-----|------------------------|
| T1 | **edi_energy_mirror** | https://github.com/Hochfrequenz/edi_energy_mirror | Mirror aller edi-energy.de-Dokumente (MIG/AHB als .docx/.pdf) – **Grundlage** |
| T2 | **edi_energy_scraper** | https://github.com/Hochfrequenz/edi_energy_scraper | Python-Tool zum automatischen Download/Mirror von bdew-mako.de |
| T3 | **kohlrahbi** | https://github.com/Hochfrequenz/kohlrahbi | AHB-Scraper: extrahiert Tabellen aus .docx → maschinenlesbare CSV/JSON |
| T4 | **machine-readable_anwendungshandbuecher** | https://github.com/Hochfrequenz/machine-readable_anwendungshandbuecher | Maschinenlesbare AHBs (JSON/CSV) – nicht mehr aktiv maintained, jetzt XML-basiert |
| T5 | **ahbicht** | https://github.com/Hochfrequenz/ahbicht | AHB-Bedingungsparser (Python) – evaluiert Muss/Kann-Ausdrücke |
| T6 | **mig_ahb_utility_stack (MAUS)** | https://github.com/Hochfrequenz/mig_ahb_utility_stack | Matching MIG ↔ AHB für Validierung |
| T7 | **EDILibrary** | https://github.com/Hochfrequenz/EDILibrary | C#/.NET EDIFACT-Library (Templates separat, via Hochfrequenz) |
| T8 | **digital_market_communication** | https://github.com/Hochfrequenz/digital_market_communication | Übersicht aller Hochfrequenz MaKo-Tools |
| T9 | **entscheidungsbaumdiagramme** | https://github.com/Hochfrequenz/entscheidungsbaumdiagramme | Maschinenlesbare EBDs (PlantUML, DOT, SVG) |
| T10 | **malo-id-generator** | https://github.com/Hochfrequenz/malo-id-generator | Generator für Test-MaLo-/MeLo-/SR-/TR-IDs |
| T11 | **BO4E-dotnet** | https://github.com/Hochfrequenz/BO4E-dotnet | Business Objects for Energy – JSON-basiertes Datenmodell |
| T12 | **fristenkalender_generator** | https://github.com/Hochfrequenz/fristenkalender_generator | Fristenkalender-Rohdaten (Werktage/Feiertage) |
| T13 | **mako_datetime_converter** | https://github.com/Hochfrequenz/mako_datetime_converter | Gastag/Stromtag, inklusive/exklusive Datumskonvertierung |

### 1.3  Drittanbieter-Tools mit Beispieldaten

| # | Tool / Repo | URL | Relevanz |
|---|------------|-----|----------|
| D1 | **edifact-to-json-transformer** (STROMDAO) | https://github.com/energychain/edifact-to-json-transformer | 🔧 Enthält `generateTestUTILMD()`, `generateTestMSCONS()`, `generateTestAPERAK()` |
| D2 | **msconsconverter** | https://github.com/vdmitriyev/msconsconverter | MSCONS→CSV Konverter mit Testdaten in `tests/data/` |
| D3 | **MSCONS Parser (Scala)** | https://github.com/derrickoswald/MSCONS | MSCONS-Reader mit Referenz auf BDEW-Dokumentation |
| D4 | **mscons-parser (Python)** | https://github.com/jay-christnach/mscons-parser | MSCONS-Parser (Luxemburg-Variante, BDEW 2.1a kompatibel) |
| D5 | **Willi-Mako Message Analyzer** | https://stromhaltig.de/wissen/artikel/edifact-message-analyzer | Online-Tool zur EDIFACT-Analyse – zeigt Beispiel-MSCONS |
| D6 | **B2B by Practice – Edipath-Doku** | https://b2bbp.next-level-help.org/b2b_cust_EDIPATH.html | Enthält vollständige UTILMD- und MSCONS-Beispiele |
| D7 | **Enerchy EDIFACT Guide** | https://enerchy.de/pillar/edifact | Strukturierte Erklärung mit Code-Snippets |
| D8 | **Corrently EDIFACT-Einführung** | https://corrently.io/books/energy-application-framework-de/page/edifact-nachrichten-in-der-energiewirtschaft-entwickler-einfuhrung | Entwickler-Tutorial mit UTILMD/MSCONS-Beispielen und npm-Paket |
| D9 | **EnBW EDIfact Editor** | https://www.enbw.com/energie/abwicklungsdienstleistungen/geschaeftskunden/enbw-praesentiert-edifact-editor | Kommerziell, 16 Editoren für alle Formate – Referenzimpl. |
| D10 | **J-EDI Viewer (regiocom)** | https://www.regiocom.com/j-edi-viewer | Kommerziell, enthält EDIFACT Message Builder für Testnachrichten |

---

## 2  EDIFACT-Nachrichtentypen – Testdaten-Status

### 2.1  UTILMD (Stammdaten)

| # | Use Case / Prüf-ID | Prozess | Beispieldaten? | Quelle |
|---|---------------------|---------|----------------|--------|
| 2.1.1 | Anmeldung LFW (E01, Prüf-ID 44001) | GPKE | ⚠️ Snippet | D6, D8, D1 |
| 2.1.2 | Abmeldung (E02) | GPKE | ⚠️ Snippet | D6 |
| 2.1.3 | Stammdatenänderung NB→LF (11109 ff.) | GPKE | ❌ | Nur AHB-Referenz in Q1 |
| 2.1.4 | Zuordnungsliste (E06) | GPKE | ❌ | – |
| 2.1.5 | Geschäftsdatenanfrage | GPKE | ❌ | – |
| 2.1.6 | MSB-Anmeldung/-Abmeldung | WiM | ❌ | – |
| 2.1.7 | Gerätewechsel-Mitteilung | WiM | ❌ | – |
| 2.1.8 | Bilanzkreiszuordnung | MaBiS | ❌ | – |
| 2.1.9 | Anmeldung Erzeugungsanlage | MPES | ❌ | – |
| 2.1.10 | Anmeldung §14a steuerbare Verbrauchseinrichtung | §14a | ❌ | – |
| 2.1.11 | Clearingliste | MaBiS | ❌ | – |
| 2.1.12 | LFW24 (24h-Lieferantenwechsel, ab 04.04.2025) | LFW24 | ❌ | Neuer Prozess, keine Testdaten im Umlauf |

**Gesamt UTILMD: 2/12 mit Snippets, 0 vollständig, 10 fehlen komplett.**

### 2.2  MSCONS (Messwerte)

| # | Use Case / Prüf-ID | Prozess | Beispieldaten? | Quelle |
|---|---------------------|---------|----------------|--------|
| 2.2.1 | Zählerstand (MRV/EMV/SMV) | WiM | ⚠️ Snippet | D2, D4, D5, D6 |
| 2.2.2 | Lastgang (15-min RLM) | WiM/MaBiS | ⚠️ Snippet | D2, Q2 (AHB-Beispiel Zeitumstellung) |
| 2.2.3 | Energiemenge (SLP) | WiM | ⚠️ Fragment | D5 |
| 2.2.4 | Schlussturnusmesswert | GPKE | ❌ | – |
| 2.2.5 | Aggregierte Zeitreihen NB→BKV | MaBiS | ❌ | – |
| 2.2.6 | Bilanzkreisabrechnungsdaten ÜNB→BKV | MaBiS | ❌ | – |
| 2.2.7 | Mehr-/Mindermengenliste | MaBiS | ❌ | – |
| 2.2.8 | Einspeise-Messwerte (EEG) | MPES | ❌ | – |
| 2.2.9 | Messwerte §14a steuerbare Verbrauchseinrichtung | §14a | ❌ | – |
| 2.2.10 | Sommer-/Winterzeitumschaltung (Edge Case) | Alle | ⚠️ Beschrieben | Q2 (AHB-Dokument, kein Roh-EDIFACT) |

**Gesamt MSCONS: 3/10 mit Snippets, 0 vollständig, 7 fehlen.**

### 2.3  INVOIC (Rechnung)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.3.1 | Netznutzungsrechnung (NB→LF) | Abrechnung | ❌ | – |
| 2.3.2 | Mehr-/Mindermengenrechnung | MaBiS | ❌ | – |
| 2.3.3 | Rechnung Messstellenbetrieb (MSB→LF) | WiM | ❌ | – |
| 2.3.4 | Ausgleichsenergie-Abrechnung (ÜNB→NB) | MaBiS | ❌ | – |
| 2.3.5 | Abschlagsrechnung | Abrechnung | ❌ | – |
| 2.3.6 | Stornorechnung | Abrechnung | ❌ | – |

**Gesamt INVOIC: 0/6, alle fehlen. Besonders heikel: Rechnungsinhalte sind vertraulich, selbst generierte Testdaten nötig.**

### 2.4  REMADV (Zahlungsavis)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.4.1 | Zahlungsavis positiv | Abrechnung | ❌ | – |
| 2.4.2 | Zahlungsavis negativ (Ablehnung) | Abrechnung | ❌ | – |

**Gesamt REMADV: 0/2.**

### 2.5  ORDERS / ORDRSP / ORDCHG (Bestellungen)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.5.1 | Bestellung Messprodukt (LF→MSB) | UBP | ❌ | – |
| 2.5.2 | Bestellbestätigung (MSB→LF) | UBP | ❌ | – |
| 2.5.3 | Bestelländerung | UBP | ❌ | – |
| 2.5.4 | Anforderung historischer Messwerte (LF→MSB) | WiM | ❌ | – |
| 2.5.5 | Anforderung Messwerte ESA→MSB | WiM | ❌ | – |

**Gesamt ORDERS-Familie: 0/5.**

### 2.6  REQOTE / QUOTES (Angebotsanfrage/-antwort)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.6.1 | Angebotsanfrage Messprodukt | UBP | ❌ | – |
| 2.6.2 | Angebot MSB | UBP | ❌ | – |
| 2.6.3 | Angebotsanfrage Schaltzeitdefinition | UBP | ❌ | – |

**Gesamt REQOTE/QUOTES: 0/3.**

### 2.7  PRICAT (Preiskatalog)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.7.1 | Preisblatt A MSB→NB/LF | UBP | ❌ | – |
| 2.7.2 | Netzentgelt-Preisblätter NB→LF | UBP | ❌ | – |

**Gesamt PRICAT: 0/2.**

### 2.8  IFTSTA (Statusmeldung)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.8.1 | Statusmeldung zu Prozessschritten | Querschnitt | ❌ | – |

**Gesamt IFTSTA: 0/1.**

### 2.9  CONTRL (Syntaxprüfung)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.9.1 | Positive Syntaxbestätigung | Quittung | ⚠️ Generator | D1 (`generateTestAPERAK`) erzeugt auch CONTRL-ähnliche Strukturen |
| 2.9.2 | Negative Syntaxbestätigung (Fehlercodes) | Quittung | ❌ | – |

**Gesamt CONTRL: 0–1/2.**

### 2.10  APERAK (Anwendungsprüfung)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.10.1 | Positive Anwendungsprüfung | Quittung | 🔧 Generator | D1 |
| 2.10.2 | Negative Anwendungsprüfung (EBD-Fehlercodes) | Quittung | ❌ | – |

**Gesamt APERAK: 0–1/2 (Generator vorhanden).**

### 2.11  PARTIN (Marktpartner-Stammdaten)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.11.1 | Marktpartner-Stammdaten (MP-ID, Rollen) | Querschnitt | ❌ | – |

**Gesamt PARTIN: 0/1.**

### 2.12  UTILTS (Berechnungsformel/Zeitreihentypen)

| # | Use Case | Prozess | Beispieldaten? | Quelle |
|---|----------|---------|----------------|--------|
| 2.12.1 | Zählzeitdefinitionen | Querschnitt | ❌ | – |
| 2.12.2 | Aufteilungsfaktoren §42b EnWG | Querschnitt | ❌ | – |

**Gesamt UTILTS: 0/2.**

---

## 3  XML-Nachrichtentypen (Redispatch 2.0) – Testdaten-Status

| # | XML-Dokument | Use Case | Beispieldaten? | Quelle |
|---|-------------|----------|----------------|--------|
| 3.1 | **ActivationDocument** | RD-Abruf/-Aktivierung | ✅ XSD | Q3 (ENTSO-E: `iec62325-451-7-activationdocument_v6_3.xsd`) |
| 3.2 | **PlannedResourceScheduleDocument** | Fahrpläne | ✅ XSD | Q3 (ENTSO-E: Schema "Planned resource schedule") |
| 3.3 | **AcknowledgementDocument** | Quittungen XML | ✅ XSD | Q3 (ENTSO-E: IEC 62325-451-1) |
| 3.4 | **Stammdaten (XML)** | TR/SR Stammdaten | ⚠️ Nur Formatbeschreibung | Q1/Q7 (BDEW-spezifisch, kein ENTSO-E-Standard) |
| 3.5 | **Kostenblatt** | RD-Kostenabrechnung | ⚠️ Nur Formatbeschreibung | Q1/Q7 |
| 3.6 | **NetworkConstraintDocument** | Engpassinfo | ✅ XSD | Q3 (ENTSO-E) |
| 3.7 | **Unavailability_MarketDocument** | Nichtverfügbarkeit | ✅ XSD | Q3 (ENTSO-E) |
| 3.8 | **StatusRequestMarketDocument** | Statusanfragen | ⚠️ | Q3 |

**Gesamt XML RD 2.0: XSD-Schemas für 4–5/8 über ENTSO-E verfügbar. Keine vollständigen Beispiel-Instanzdokumente mit energiewirtschaftlichem Inhalt öffentlich.**

---

## 4  Übertragungsweg-Artefakte

| # | Artefakt | Beispieldaten? | Quelle |
|---|---------|----------------|--------|
| 4.1 | AS4-Envelope (OASIS ebMS 3.0) | ⚠️ Generische OASIS-Beispiele | OASIS-Spezifikation |
| 4.2 | S/MIME-verschlüsselte EDIFACT-Mail | ❌ | – |
| 4.3 | REST-API MaLo-ID-Ermittlung (JSON) | ❌ | Regelwerk 1.1 auf bdew-mako.de |
| 4.4 | CLS-Kanal-Steuerbefehl | ❌ | BSI-TR-03109, proprietär |

---

## 5  Querschnittsdaten für Testinfrastruktur

| # | Datenbedarf | Verfügbar? | Quelle |
|---|------------|------------|--------|
| 5.1 | Feiertagskalender (Werktage MaKo) | ✅ | T12 (fristenkalender_generator) |
| 5.2 | Test-MaLo/MeLo/SR/TR-IDs | 🔧 | T10 (malo-id-generator) |
| 5.3 | Maschinenlesbare AHBs (JSON) | ✅ | T3/T4 (kohlrahbi / machine-readable AHBs) |
| 5.4 | Maschinenlesbare EBDs | ✅ | T9 (entscheidungsbaumdiagramme) |
| 5.5 | Codelisten (OBIS, Artikelnummern, Messprodukte) | ⚠️ | Q1 (PDF), teilweise in T4 |
| 5.6 | BDEW Marktpartnerdatenbank (MP-IDs) | ✅ | BDEW-Codenummerndatenbank (öffentlich abfragbar) |
| 5.7 | Gastag/Stromtag-Konvertierung | ✅ | T13 |
| 5.8 | BO4E Datenmodell (JSON) | ✅ | T11 |

---

## 6  Zusammenfassung & Priorisierung

### 6.1  Gesamtbild

| Kategorie | Typen gesamt | Verfügbar (Snippet+) | Fehlen komplett | Deckungsgrad |
|-----------|:------------:|:--------------------:|:---------------:|:------------:|
| EDIFACT Nachrichten | 47 Use Cases | 6 (Snippets/Generatoren) | 41 | **~13%** |
| XML RD 2.0 | 8 Typen | 5 (XSD nur) | 3 | **~63% (nur Schema)** |
| Übertragungsweg | 4 Artefakte | 0 | 4 | **0%** |
| Querschnittsdaten | 8 Bereiche | 7 | 1 | **~88%** |

### 6.2  Kritische Lücken (Prio 1 – sofort generieren)

1. **UTILMD Anmeldung/Abmeldung** – vollständige Nachrichten für GPKE-LFW inkl. LFW24
2. **MSCONS Lastgang + Zählerstand** – vollständige Nachrichten mit realistischen Zeitreihen
3. **INVOIC Netznutzungsrechnung** – mindestens ein vollständiges Rechnungsbeispiel
4. **REMADV** – positiv + negativ
5. **CONTRL + APERAK** – je positiv/negativ-Paar
6. **UTILMD Zuordnungslisten** – das MaKo-Rückgrat

### 6.3  Wichtige Lücken (Prio 2 – bald generieren)

7. **ORDERS/ORDRSP/REQOTE/QUOTES** – UBP-Prozess komplett
8. **PRICAT** – Preisblätter
9. **MaBiS-Zeitreihen** – aggregierte MSCONS NB→BKV, Fahrpläne BKV→ÜNB
10. **§14a-Nachrichten** – UTILMD + MSCONS für steuerbare Verbraucher
11. **MPES-Nachrichten** – Erzeugungsanlagen-Stammdaten + Einspeise-Messwerte

### 6.4  Ergänzende Lücken (Prio 3)

12. **PARTIN** – Marktpartner-Stammdaten
13. **UTILTS** – Berechnungsformeln, Zählzeitdefinitionen
14. **IFTSTA** – Statusmeldungen
15. **Alle XML RD 2.0 Instanzdokumente** – befüllte Beispiele auf Basis der XSDs
16. **REST-API Testfälle** – MaLo-ID-Ermittlung Request/Response

### 6.5  Edge Cases (für die Test-Suite essenziell)

| # | Edge Case | Betrifft | Priorität |
|---|-----------|----------|-----------|
| EC1 | Sommer-/Winterzeitumschaltung in Lastgängen | MSCONS | Hoch |
| EC2 | Gastag (06:00 Uhr) vs. Stromtag (00:00 Uhr) | Alle DTM-Segmente | Hoch |
| EC3 | UTC-Zeitumstellung in EDIFACT (303-Format) | Alle | Hoch |
| EC4 | Gleichzeitige Anmeldung zweier LF für selbe MaLo | UTILMD | Hoch |
| EC5 | Rückfall auf Grundversorgung (implizit, ohne Nachricht) | GPKE-Zustandsautomat | Mittel |
| EC6 | Fristablauf ohne Antwort (Timeout als Event) | Alle Prozesse | Mittel |
| EC7 | Feiertagskalender über Jahreswechsel | Fristberechnung | Mittel |
| EC8 | Maximale Anzahl LIN-Segmente (viele MaLos in einer UTILMD) | UTILMD | Mittel |
| EC9 | Sonderzeichen im EDIFACT-Escape (`?+`, `?:`, `?'`) | Alle | Niedrig |
| EC10 | Leere optionale Segmente / Conditional-Felder | Alle | Niedrig |

---

## 7  Empfohlene Vorgehensweise

### Phase 1: Strukturdefinitionen sichern
- [ ] `edi_energy_mirror` klonen (T1) → aktuelle MIGs/AHBs lokal
- [ ] `kohlrahbi` (T3) laufen lassen → maschinenlesbare AHBs als JSON/CSV
- [ ] EBDs aus T9 laden → Entscheidungslogik als Graphen
- [ ] ENTSO-E XSDs (Q3) herunterladen → XML-Validierungsbasis

### Phase 2: Vorhandene Snippets sammeln & normalisieren
- [ ] UTILMD-Snippet aus D6 (B2B by Practice) extrahieren und gegen aktuelle MIG validieren
- [ ] MSCONS-Snippets aus D2, D5 extrahieren
- [ ] `generateTestUTILMD()` / `generateTestMSCONS()` aus D1 evaluieren → Output gegen AHB prüfen
- [ ] Alle gesammelten Snippets in einheitliches Verzeichnis (`/corpus/raw/`)

### Phase 3: Fehlende Nachrichten generieren
- [ ] Generator bauen: AHB-JSON + EBD → vollständige EDIFACT-Beispielnachricht
- [ ] Pro Nachrichtentyp & Prüf-ID mindestens eine valide + eine invalide Nachricht
- [ ] XML RD 2.0: Instanzdokumente auf Basis der XSDs erzeugen
- [ ] Anonymisierte Testdaten verwenden (Test-MP-IDs, generierte MaLo-IDs via T10)

### Phase 4: Validierung & Edge Cases
- [ ] CONTRL/APERAK-Paare für jede Nachricht erzeugen
- [ ] Edge Cases (EC1–EC10) als dedizierte Testfälle implementieren
- [ ] Fristenberechnung gegen Feiertagskalender (T12) testen
- [ ] Gastag/Stromtag-Konvertierung (T13) in DTM-Segmenten verifizieren

---

## 8  Hinweis zur Vertraulichkeit

Die **Formatdefinitionen** (MIG, AHB, EBD, XSD) und **Prozessbeschreibungen** (GPKE, WiM, MaBiS) sind öffentlich zugängliche Dokumente. Alle in diesem Testkorpus erzeugten Nachrichten verwenden **fiktive** Stammdaten, Messwerte und Rechnungsbeträge. Echte Marktdaten (reale MP-IDs, MaLo-Zuordnungen, Rechnungsinhalte) dürfen **niemals** in den Testkorpus aufgenommen werden.
