# MaKo-Engine: Funktionale Marktkommunikation für Strom und Gas

**Projektziel:** Entwicklung eines vollständig maschinenlesbaren, funktional modellierten Systems für die deutsche energiewirtschaftliche Marktkommunikation – Strom, Gas und perspektivisch Wasserstoff.

**Kernprinzipien:**

- Alle Prozesse als pure Funktionen und Zustandsautomaten (Reducer)
- Alle Dokumentation in Plain-Text-Formaten (Markdown, JSON, YAML, TOML)
- Systeme und Formate sind öffentlich, transportierte Inhalte sind vertraulich
- Spartenübergreifende Architektur mit austauschbaren Transport-Adaptern
- Vollständige Automatisierbarkeit als Designziel

**Stand:** März 2026

---

## Inhaltsverzeichnis

| # | Dokument | Datei | Inhalt |
|---|----------|-------|--------|
| 1 | [Glossar der Energiewirtschaft](#1--glossar-der-deutschen-energiewirtschaft) | `glossar_energiewirtschaft.md` | Marktrollen, Regulierung, Marktbegriffe, Technik, IT, Akteure |
| 2 | [Referenz BNetzA](#2--bundesnetzagentur-bnetzaref) | `referenz_bundesnetzagentur.md` | Aufgaben, Zuständigkeiten, aktuelle Verfahren |
| 3 | [Referenz Energiewirtschaft Einfach](#3--referenz-youtube-kanal-energiewirtschaft-einfach) | `referenz_energiewirtschaft_einfach.md` | Thematische Gruppierung der Kanal-Inhalte als Wissensbasis |
| 4 | [MaKo-Formatvorlagen-Referenz](#4--mako-formatvorlagen-referenz) | `mako_formatvorlagen_referenz.md` | EDIFACT/XML-Nachrichtentypen, Prozessbeschreibungen, Übertragungswege, Quellen |
| 5 | [Zeitplan Marktkommunikation](#5--zeitplan-marktkommunikation) | `zeitplan_marktkommunikation.md` | Vergangene, aktuelle und geplante regulatorische Meilensteine |
| 6 | [Kommunikationslinien Strom](#6--kommunikationslinien-strom) | `checkliste_mako_kommunikationslinien.md` | Alle Nachrichtenflüsse Strom: GPKE, WiM, UBP, MaBiS, MPES, RD 2.0, §14a |
| 7 | [Kommunikationslinien Gas](#7--kommunikationslinien-gas) | `checkliste_mako_kommunikationslinien_gas.md` | Alle Nachrichtenflüsse Gas: GeLi Gas, GABi Gas, KoV, Nominierung |
| 8 | [Testkorpus-Checkliste](#8--testkorpus-marktkommunikation) | `checkliste_testkorpus_mako.md` | Bestandsaufnahme Testdaten, Quellen, Lücken, Aufbauplan |
| 9 | [Recherche gleichgesinnte Projekte](#9--recherche-gleichgesinnte-projekte) | `recherche_gleichgesinnte_projekte.md` | Hochfrequenz, BO4E, STROMDAO, Vergleichsmatrix |

---

## Projektübersicht

### Architektur

Das System ist als funktionale Schichtenarchitektur aufgebaut. Jede Schicht ist von der darunterliegenden entkoppelt; Seiteneffekte (IO) sind ausschließlich in der Transport-Schicht isoliert.

```
┌─────────────────────────────────────────────────────────┐
│  Prozess-Schicht (State Machine / Reducer)               │
│  STROM: GPKE, WiM, MaBiS, MPES, RD 2.0, §14a, UBP     │
│  GAS:   GeLi Gas, GABi Gas, KoV, Nominierung            │
│  State × Event → State × [Nachricht]                    │
├─────────────────────────────────────────────────────────┤
│  Validierungs-Schicht (Pure Functions)                   │
│  CONTRL: Syntaxprüfung (spartenübergreifend)             │
│  APERAK: EBD-Entscheidungsbaum → accept/reject           │
├─────────────────────────────────────────────────────────┤
│  Mengenumrechnung Gas (Pure Functions)                   │
│  m³ × Zustandszahl × Brennwert → kWh                    │
│  (nur Gas-Sparte; kein Pendant in Strom)                 │
├─────────────────────────────────────────────────────────┤
│  Serialisierung / Codec                                  │
│  EDIFACT ↔ internes Datenmodell (BO4E) ↔ XML/JSON       │
│  Je Sparte eigene MIG/AHB-Ausprägung                    │
├─────────────────────────────────────────────────────────┤
│  Transport-Schicht (IO / Effekte)                        │
│  AS4 · S/MIME · REST-API · CLS-Kanal                     │
│  PRISMA (Gas-Kapazitäten) · THE-Portal (Gas-Nominierung) │
│  Signatur, Verschlüsselung, Zertifikate                 │
└─────────────────────────────────────────────────────────┘
```

### Spartenabdeckung

| Sparte | Status | Prozessdomänen |
|--------|--------|----------------|
| **Strom** | ✅ Vollständig dokumentiert | GPKE, WiM, UBP, MaBiS, MPES, Netznutzungsabrechnung, RD 2.0, §14a EnWG, API-Webdienste |
| **Gas** | ✅ Vollständig dokumentiert | GeLi Gas, Messwesen Gas, GABi Gas, KoV, Netznutzungsabrechnung Gas |
| **Wasserstoff** | ⏳ Perspektivisch (~2032) | WasABi (BK7-24-01-014), WaKandA (BK7-24-01-015) – Architektur ist vorbereitet |

### Vertraulichkeitsmodell

| Kategorie | Beispiele | Sichtbarkeit |
|-----------|-----------|-------------|
| **Systeme & Formate** | MIG, AHB, EBD, Prozessbeschreibungen, dieses Projekt | Öffentlich |
| **Transportierte Inhalte** | Rechnungen (INVOIC), Stammdaten (UTILMD), Messwerte (MSCONS), Zahlungsavise (REMADV) | Vertraulich |
| **Testdaten** | Fiktive MP-IDs, generierte MaLo-IDs, synthetische Messwerte | Öffentlich (da fiktiv) |

---

## 1  Glossar der deutschen Energiewirtschaft

**Datei:** `glossar_energiewirtschaft.md`

Umfassendes Nachschlagewerk mit 7 Kapiteln:

| Kapitel | Inhalt | Einträge (ca.) |
|---------|--------|:--------------:|
| 1 – Marktrollen | ÜNB, VNB, MSB, LF, BKV, FNB, MGV, SSO etc. | 18 |
| 2 – Regulierung & Gesetze | EnWG, EEG, MsbG, GNDEW, GEG, RED II/III etc. | 20 |
| 3 – Markt- & Preisbegriffe | Bilanzkreis, Fahrplan, Regelenergie, MaKo, GPKE, GeLi Gas etc. | 25+ |
| 4 – Technische Begriffe | iMSys, SMGW, SLP, RLM, Prosumer, V2G etc. | 20+ |
| 5 – CO₂ & Emissionshandel | EU-ETS, nEHS, BEHG, CBAM, THG-Quote, GoO/HKN etc. | 10 |
| 6 – IT & Kommunikation | EDIFACT, AS4, MSCONS, UTILMD, MaLo, MeLo, SAP IS-U, CLS etc. | 20+ |
| 7 – Akteure & Institutionen | 50Hertz, Amprion, THE, BDEW, BNetzA, ENTSO-E/G etc. | 15+ |

---

## 2  Bundesnetzagentur (BNetzA) – Referenz

**Datei:** `referenz_bundesnetzagentur.md`

Die BNetzA ist die zentrale Regulierungsbehörde. Für unser Projekt ist insbesondere die Beschlusskammer 6 (BK6) relevant, die alle MaKo-Festlegungen erlässt.

| Aufgabenbereich | Projektrelevanz |
|-----------------|-----------------|
| Netzregulierung (ARegV, Erlösobergrenzen) | Indirekt (Netzentgelte in INVOIC) |
| **Marktkommunikation (BK6)** | **Direkt – erlässt GPKE, WiM, MaBiS, MPES, GeLi Gas, Datenformate** |
| Netzentwicklungsplanung | Nicht direkt |
| Ausschreibungen EE & KWK | Nicht direkt |
| Energiemarkt-Monitoring (SMARD) | Kontextwissen |
| Versorgungssicherheit | Nicht direkt |

Aktuelle Festlegungen: LFW24 (BK6-22-024), MaBiS-Hub (BK6-24-210). Gas: BK7 für GeLi Gas, GABi Gas.

---

## 3  Referenz: YouTube-Kanal „Energiewirtschaft Einfach"

**Datei:** `referenz_energiewirtschaft_einfach.md`

Thematisch gruppierte Inhaltsbasis mit 60 Episoden (Carsten Eckart, 2019–2026). Dient als Kontextwissen und Validierungsquelle für Prozessverständnis.

| Themenbereich | Episoden | Abdeckung |
|---------------|:--------:|-----------|
| Marktrollen & Struktur | 5 | ✅ Gut |
| MaKo & Datenformate | 6 | ✅ Sehr gut (Kernthema) |
| Smart Metering | 5 | ✅ Gut |
| Bilanzierung & Handel | 8 | ✅ Sehr gut |
| Erneuerbare & EEG | 6 | ✅ Gut |
| Netz & Netzbetrieb | 3 | ⚠️ Mittel |
| Sektorkopplung & H₂ | 5 | ⚠️ Mittel |
| Klima & CO₂ | 6 | ⚠️ Mittel |
| Dezentralisierung | 5 | ✅ Gut |
| Lageberichte | 11 | Kontextwissen |

---

## 4  MaKo-Formatvorlagen-Referenz

**Datei:** `mako_formatvorlagen_referenz.md`

Zentrale Referenz für alle Datenformate, Prozessbeschreibungen, Übertragungswege und Quellen der Marktkommunikation.

### 4.1  Dokumententypen der EDI@Energy-Welt

| Kürzel | Typ | Funktion |
|--------|-----|----------|
| MIG | Message Implementation Guide | Technische Struktur der EDIFACT-Nachricht |
| AHB | Anwendungshandbuch | Fachliche Befüllungsregeln je Prozessschritt |
| EBD | Entscheidungsbaum-Diagramme | Prüflogik für Antwort-Nachrichten |
| AWT | Anwendungstabelle (XML) | Pendant zum AHB für XML-Formate (RD 2.0) |
| FB | Formatbeschreibung (XML) | Pendant zur MIG für XML-Formate |

### 4.2  Prozessbeschreibungen (BNetzA-Festlegungen)

| Kürzel | Sparte | Beschreibung |
|--------|--------|-------------|
| GPKE | Strom | Kundenbelieferung Elektrizität (seit LFW24 in 4 Teilen) |
| WiM | Strom+Gas | Wechselprozesse im Messwesen |
| MaBiS | Strom | Bilanzkreisabrechnung Strom |
| MPES | Strom | Marktprozesse Erzeugungsanlagen |
| GeLi Gas | Gas | Geschäftsprozesse Lieferantenwechsel Gas |
| KoV | Gas | Kooperationsvereinbarung Gasnetzzugang |
| GABi Gas | Gas | Bilanzierung Gas |
| LFW24 | Strom | 24h-Lieferantenwechsel (ab 06.06.2025) |
| RD 2.0 | Strom | Redispatch 2.0 (XML-basiert) |
| §14a EnWG | Strom | Steuerbare Verbrauchseinrichtungen |
| UBP | Strom+Gas | Universalbestellprozess |

### 4.3  Übertragungswege

| Kanal | Sparte | Status |
|-------|--------|--------|
| AS4 (eDelivery) | Strom (Standard seit 04/2024), Gas (in Einführung) | Aktiv |
| S/MIME (E-Mail) | Gas (noch Standard), Strom (auslaufend) | Übergang |
| REST-API | Strom (ab 04/2025), Gas (perspektivisch) | Neu |
| CLS-Kanal | Strom (§14a) | Aktiv |
| PRISMA | Gas (Kapazitätsbuchung) | Aktiv (nicht-EDIFACT) |
| THE-Portal | Gas (Nominierung) | Aktiv (nicht-EDIFACT) |

### 4.4  Versionierung und Quellen

Alle Formatdokumente folgen dem Schema `[Nachrichtentyp] [Sparte] [Dokumenttyp] [Strukturversion].[Textversion]`. Offizielle Quellen: BDEW MaKo-Plattform (bdew-mako.de), BNetzA BK6/BK7, DVGW-SC (Gas), ENTSO-E (XML-Schemas).

---

## 5  Zeitplan Marktkommunikation

**Datei:** `zeitplan_marktkommunikation.md`

Chronologische Übersicht aller regulatorischen Meilensteine von 2017 bis ~2032.

### 5.1  Vergangene Meilensteine (2017–2024)

| Datum | Meilenstein |
|-------|------------|
| 10/2017 | MaKo 2017 – MaLo/MeLo-Modell, MSB als Marktrolle |
| 12/2019 | MaKo 2020 – IT-Umsetzung |
| 02/2020 | MaKo 2020 – Prozessstart |
| 10/2021 | Redispatch 2.0 |
| 10/2023 | MaKo 2022 – Prozessstart |
| 04/2024 | AS4 Pflicht (Strom) |
| 01/2025 | §14a EnWG – Steuerbare Verbraucher |

### 5.2  Aktuelle Phase (2025)

| Datum | Meilenstein |
|-------|------------|
| 04/2025 | LFW24 – IT-Umsetzung + erster API-Webdienst |
| 06/2025 | LFW24 – Prozessstart (24h-Lieferantenwechsel) |
| Q3/2025 | AS4 Version 2.4 |

### 5.3  Geplante Meilensteine (2026–2032)

| Datum | Meilenstein | Projektimplikation |
|-------|------------|-------------------|
| Q1/2026 | MaBiS-Hub Festlegung (BK6-24-210) | Architektur-Vorbereitung Hub-Anbindung |
| 2026 | GeLi Gas 3.0 | Gas-Reducer Fristen aktualisieren |
| Q2/2026 | Weitere API-Webdienste | REST-Adapter erweitern |
| 04/2028 | MaBiS-Hub IT-Start | Hub-Adapter implementieren |
| 10/2029 | MaBiS-Hub Vollbetrieb | MaBiS-Reducer auf Hub umstellen |
| ~2032 | EDIFACT endgültig abgelöst, vollständig API-basiert | Validierung unseres Ansatzes |
| ~2032 | Wasserstoff-Integration (WasABi/WaKandA) | Dritte Sparte einhängen |

---

## 6  Kommunikationslinien Strom

**Datei:** `checkliste_mako_kommunikationslinien.md`

Vollständige Erfassung aller gerichteten Nachrichtenflüsse der Stromsparte.

### 6.1  Prozessdomänen

| # | Domäne | Abschnitt | Linien | Status |
|---|--------|-----------|--------|--------|
| 1 | GPKE – Kundenbelieferung | §1 | LFW/LFW24, Abmeldung, Stammdaten, Zuordnung, GDA | ✅ |
| 2 | WiM – Messwesen | §2 | MSB-Wechsel, Gerätewechsel, Zählwerte, Werte-Anfrage | ✅ |
| 3 | UBP – Bestellprozesse | §3 | REQOTE/QUOTES/ORDERS/ORDRSP, PRICAT | ✅ |
| 4 | MaBiS – Bilanzierung | §4 | BK-Zuordnung, Bilanzierungsdaten, MeMinMe, Clearing | ✅ |
| 5 | MPES – Erzeugungsanlagen | §5 | Anmeldung EE, Messwerte, Zuordnung | ✅ |
| 6 | Netznutzungsabrechnung | §6 | INVOIC/REMADV alle Richtungen | ✅ |
| 7 | Redispatch 2.0 | §7 | Stammdaten, Fahrplan, Abruf, Engpass, Kosten | ✅ |
| 8 | §14a EnWG | §8 | Anmeldung, Steuerung, Messwerte | ✅ |
| 9 | Querschnitt | §9 | PARTIN, UTILTS, IFTSTA | ✅ |
| 10 | API-Webdienste | §10 | MaLo-ID-Ermittlung | ⚠️ Erweiterbar |

### 6.2  Nachrichtentypen Strom

16 Nachrichtentypen erfasst: UTILMD, MSCONS, INVOIC, REMADV, REQOTE, QUOTES, ORDERS, ORDRSP, ORDCHG, PRICAT, IFTSTA, CONTRL, APERAK, PARTIN, UTILTS, XML RD 2.0 (7 Typen). Alle vollständig abgedeckt.

### 6.3  Architektur-Prinzipien

Die Kommunikationslinien-Checkliste definiert die Architektur-Grundlage:

- Jede Kommunikationslinie als pure Funktion: `(Nachricht, Kontext) → (Nachricht | Ablehnung | Ø)`
- Prozesse als Zustandsautomaten (Reducer): `(Zustand, Nachricht) → (neuer Zustand, [Ausgabe-Nachrichten])`
- Fristen als pure Kalenderfunktionen: `frist(datum, n_werktage, feiertagskalender) → stichtag`
- Quittungsschicht (CONTRL/APERAK) als generische Middleware
- Transport-Adapter (AS4, REST, CLS) als austauschbare IO-Schicht

---

## 7  Kommunikationslinien Gas

**Datei:** `checkliste_mako_kommunikationslinien_gas.md`

Vollständige Erfassung aller gerichteten Nachrichtenflüsse der Gassparte – strukturell analog zur Strom-Checkliste.

### 7.1  Prozessdomänen

| # | Domäne | Abschnitt | Linien | Status |
|---|--------|-----------|--------|--------|
| 1 | GeLi Gas – Lieferantenwechsel | §2 | LFW, Abmeldung, Stammdaten, Zuordnung, GDA | ✅ |
| 2 | Messwesen Gas | §3 | Gerätewechsel, Zählwerte, Werte-Anfrage | ✅ |
| 3 | GABi Gas – Bilanzierung | §4 | BK-Zuordnung, Allokation, Nominierung, MeMinMe, Clearing | ✅ |
| 4 | KoV – Kapazität & Netzkonten | §5 | Kapazitätsbuchung, Netzkontoabrechnung, Brennwert | ✅ |
| 5 | Netznutzungsabrechnung Gas | §6 | INVOIC/REMADV alle Richtungen | ✅ |
| 6 | Querschnitt Gas | §7 | PARTIN, UTILTS, IFTSTA | ✅ |

### 7.2  Strukturelle Unterschiede Strom ↔ Gas

| Merkmal | Strom | Gas |
|---------|-------|-----|
| Tagesbeginn | 00:00 | 06:00 (Gastag) |
| Bilanzierung | 15 min, BKV↔ÜNB | 1 h, BKV↔MGV (THE) |
| Übertragungsnetz | 4 ÜNBs | 16 FNBs |
| Marktgebiet | 4 Regelzonen | 1 (THE) |
| Kapazitätsmanagement | Implizit | Explizit (PRISMA) |
| Nominierung | Nein | Ja (EDIG@S / THE-Portal) |
| Mengenumrechnung | Nein | m³ → kWh (Zustandszahl, Brennwert) |
| Standardlastprofil | SLP Strom | SLP Gas (temperaturabhängig) |

### 7.3  Architektur-Erweiterung durch Gas

Die Gas-Checkliste erweitert die Strom-Architektur um:

- **Spartenparameter** auf allen Funktionen: `Sparte = Strom | Gas | Wasserstoff`
- **GasKontext** mit Brennwerten, Zustandszahlen, Allokationsregeln, Gastag-Offset
- **Mengenumrechnungsschicht** als eigene pure Schicht zwischen Codec und Prozess
- **Zusätzliche Transport-Adapter** für PRISMA, THE-Portal und EDIG@S
- **Gas-spezifische Reducer** für GeLi Gas, GABi Gas, KoV und Nominierung

---

## 8  Testkorpus Marktkommunikation

**Datei:** `checkliste_testkorpus_mako.md`

Systematische Bestandsaufnahme aller öffentlich verfügbaren Beispiel- und Testdateien.

### 8.1  Gesamtbild Testdaten-Verfügbarkeit

| Kategorie | Typen gesamt | Verfügbar (Snippet+) | Fehlen komplett | Deckungsgrad |
|-----------|:------------:|:--------------------:|:---------------:|:------------:|
| EDIFACT Nachrichten | 47 Use Cases | 6 (Snippets/Generatoren) | 41 | ~13% |
| XML RD 2.0 | 8 Typen | 5 (XSD nur) | 3 | ~63% (Schema) |
| Übertragungsweg-Artefakte | 4 | 0 | 4 | 0% |
| Querschnittsdaten | 8 Bereiche | 7 | 1 | ~88% |

### 8.2  Kritische Lücken (Prio 1)

1. UTILMD Anmeldung/Abmeldung (vollständig, inkl. LFW24)
2. MSCONS Lastgang + Zählerstand (mit Zeitreihen)
3. INVOIC Netznutzungsrechnung (mindestens ein Beispiel)
4. REMADV positiv + negativ
5. CONTRL + APERAK (je positiv/negativ)
6. UTILMD Zuordnungslisten

### 8.3  Quellen-Ökosystem

| Kategorie | Quellen |
|-----------|---------|
| Offizielle Regelwerke | BDEW MaKo-Plattform, BNetzA BK6, ENTSO-E, DVGW-SC |
| Open-Source-Tooling | Hochfrequenz (kohlrahbi, rebdhuhn, edi_energy_mirror, transformer.bee, BO4E) |
| Testdaten-Generierung | malo-id-generator, fristenkalender_generator, Gastag/Stromtag-Konvertierung |

### 8.4  Aufbauplan

Phase 1 (Strukturdefinitionen sichern) → Phase 2 (vorhandene Snippets sammeln) → Phase 3 (fehlende Nachrichten generieren) → Phase 4 (Validierung & Edge Cases).

10 Edge Cases identifiziert, darunter: Sommer-/Winterzeitumstellung, paralleler MSB-Wechsel und LFW, Gastag-Überlappung, §14a-Steuerung bei SMGW-Ausfall.

---

## 9  Recherche gleichgesinnte Projekte

**Datei:** `recherche_gleichgesinnte_projekte.md`

### 9.1  Identifizierte Akteure

| Akteur | Relevanz | Schwerpunkt |
|--------|:--------:|-------------|
| Hochfrequenz Consulting | ⭐⭐⭐⭐⭐ | Scraper, Konverter, Workflow-Engine – größte Überlappung |
| BO4E e.V. | ⭐⭐⭐⭐ | Standardisiertes Datenmodell (JSON-Schemas) |
| STROMDAO | ⭐⭐ | Kleinerer Scope, EDIFACT→JSON |

### 9.2  Alleinstellungsmerkmal unseres Projekts

| Dimension | Andere | Wir |
|-----------|--------|-----|
| Funktionale Modellierung | ❌ | ✅ Pure Functions, Reducer, IO-Isolation |
| Plain-Text-Dokumentation | ❌ | ✅ Markdown, JSON, YAML |
| Gesamtarchitektur | ❌ (werkzeuggetrieben) | ✅ Durchgängiges Schichtenmodell |
| EBDs als ausführbare Funktionen | ❌ (nur Graphen) | ✅ Deterministische pure Functions |
| Spartenübergreifend (Strom+Gas+H₂) | ❌ | ✅ Von Anfang an |

### 9.3  Empfehlungen

1. Hochfrequenz kontaktieren (kohlrahbi, rebdhuhn, transformer.bee nutzen)
2. BO4E als internes Datenmodell evaluieren
3. DVGW-SC-Formate für Gas analog zum edi_energy_mirror inventarisieren

---

## 10  Gesamtstatus und nächste Schritte

### 10.1  Recherche-Status

| Bereich | Status | Anmerkung |
|---------|--------|-----------|
| Marktrollen & Glossar | ✅ Vollständig | 7 Kapitel, 100+ Einträge |
| Regulierung (BNetzA, Gesetze) | ✅ Vollständig | BK6, BK7, GBK erfasst |
| Formatvorlagen (MIG, AHB, EBD) | ✅ Vollständig | Strom + Gas, alle Quellen dokumentiert |
| Zeitplan | ✅ Vollständig | 2017–2032 |
| Kommunikationslinien Strom | ✅ Vollständig | 10 Domänen, 16 Nachrichtentypen, Rollen-Matrix |
| Kommunikationslinien Gas | ✅ Vollständig | 6 Domänen, Gas-Rollen-Matrix, Strukturunterschiede |
| Testkorpus | ✅ Inventarisiert | Lücken identifiziert, Aufbauplan definiert |
| Gleichgesinnte Projekte | ✅ Recherchiert | 3 Akteure, Vergleichsmatrix, Empfehlungen |
| Architektur | ✅ Definiert | Schichtenmodell, Spartenübergreifend, Reducer-Prinzip |

### 10.2  Nächste Schritte (Implementierung)

| # | Schritt | Beschreibung | Grundlage |
|---|---------|-------------|-----------|
| 1 | Formale Typdefinitionen | JSON Schema / TypeScript-Typ für jeden Nachrichtentyp (Strom + Gas) | Dok. 4 + 6 + 7 |
| 2 | EBD als Code | Alle Entscheidungsbäume als deterministische pure Functions | Dok. 4 (EBDs) |
| 3 | Prozess-Reducer Strom | GPKE, WiM, MaBiS, MPES, UBP, RD 2.0, §14a je als Reducer | Dok. 6 |
| 4 | Prozess-Reducer Gas | GeLi Gas, GABi Gas, KoV, Nominierung je als Reducer | Dok. 7 |
| 5 | Mengenumrechnung Gas | `(m³, Zustandszahl, Brennwert) → kWh` als pure Funktion | Dok. 7, §3.2 |
| 6 | Feiertagskalender | YAML/JSON-Daten + pure Fristberechnung mit Spartenparameter | Dok. 6 + 7 |
| 7 | Quittungs-Middleware | CONTRL/APERAK als generischer Wrapper | Dok. 6, §12 |
| 8 | Codec / Serialisierung | EDIFACT ↔ BO4E (Strom-MIG + Gas-MIG) | Dok. 4 + 9 |
| 9 | Transport-Adapter | AS4, S/MIME, REST-API, CLS, PRISMA, THE-Portal | Dok. 4 + 7 |
| 10 | Testkorpus aufbauen | Phasen 1–4 gem. Aufbauplan, inkl. Gas-Testdaten | Dok. 8 |

### 10.3  Offene Punkte (kein Blocker für Start)

| Thema | Beschreibung | Zeitrahmen |
|-------|-------------|-----------|
| GeLi Gas 3.0 | Regulierungsverfahren läuft; Fristen werden sich ändern | 2026 |
| MaBiS-Hub | Festlegung Q1/2026, IT-Start 2028 | 2028–2029 |
| Weitere API-Webdienste | Erweiterung über MaLo-ID-Ermittlung hinaus | Q2/2026 |
| PRICAT Gas (Preisblätter) | Noch nicht im Detail dokumentiert | Bei Bedarf |
| Wasserstoff (WasABi/WaKandA) | Festlegungsverfahren, ~2032 | Langfristig |
