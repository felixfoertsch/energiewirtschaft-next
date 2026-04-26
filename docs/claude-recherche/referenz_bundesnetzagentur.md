# Bundesnetzagentur (BNetzA) – Referenz Energiebereich

Zusammenfassung der Aufgaben, Zuständigkeiten und aktuellen Verfahren

Stand: März 2026

---

## 1  Überblick

| Merkmal | Detail |
|---------|--------|
| **Name** | Bundesnetzagentur für Elektrizität, Gas, Telekommunikation, Post und Eisenbahnen |
| **Kurzform** | BNetzA |
| **Rechtsform** | Bundesoberbehörde im Geschäftsbereich des BMWK |
| **Sitz** | Bonn |
| **Mitarbeiter** | ca. 3.000 an 46 Standorten |
| **Energiezuständigkeit seit** | 2005 (Novelle EnWG) |
| **Website** | https://www.bundesnetzagentur.de |

---

## 2  Kernaufgaben im Energiebereich

### 2.1  Netzregulierung

Die BNetzA kontrolliert und genehmigt die Netznutzungsentgelte und gewährleistet den diskriminierungsfreien Zugang zu Strom- und Gasnetzen. In der Anreizregulierung (ARegV) bestimmt die Behörde nicht das einzelne Netzentgelt, sondern setzt Erlösobergrenzen und überwacht die Einhaltung.

Seit dem EuGH-Urteil vom September 2021 wird die Unabhängigkeit der BNetzA massiv gestärkt: Regelungen, die bisher in der Anreizregulierungsverordnung und den Entgeltverordnungen vorgegeben waren, müssen nun direkt durch Festlegungen der unabhängigen Regulierungsbehörde ersetzt werden. Dafür wurde die **Große Beschlusskammer Energie (GBK)** eingerichtet.

### 2.2  Marktkommunikation (Beschlusskammer 6)

Die BK6 ist für unser Projekt die zentrale Instanz. Sie erlässt die rechtsverbindlichen Festlegungen zu:

- **GPKE** — Geschäftsprozesse Kundenbelieferung Elektrizität
- **WiM** — Wechselprozesse im Messwesen
- **MaBiS** — Marktregeln Bilanzkreisabrechnung Strom
- **MPES** — Marktprozesse Erzeugungsanlagen Strom
- **GeLi Gas** — Geschäftsprozesse Lieferantenwechsel Gas
- **Datenformate** — Verbindliche EDI@Energy-Dokumente (EDIFACT, XML)

Aktuelle Festlegungen: LFW24 (BK6-22-024), MaBiS-Hub (BK6-24-210).

### 2.3  Netzentwicklungsplanung

Die BNetzA prüft und bestätigt den Netzentwicklungsplan Strom (erstellt von den ÜNBs) und den Netzentwicklungsplan Gas und Wasserstoff (erstellt von den FNBs). Die Szenariorahmen bilden wahrscheinliche energiewirtschaftliche Entwicklungen ab.

### 2.4  Ausschreibungen EE & KWK

Durchführung der Ausschreibungen für Windenergie (on-/offshore), Solar, Biomasse und KWK-Anlagen gemäß EEG und KWKG.

### 2.5  Energiemarkt-Monitoring

Gemeinsam mit dem Bundeskartellamt überwacht die BNetzA den Energiemarkt. Die Transparenzdaten werden über die Plattform **SMARD** (Strommarktdaten) veröffentlicht.

### 2.6  Versorgungssicherheit

Überwachung der Versorgungssicherheit inkl. LNG-Anlagen und Gasspeicher (seit der Energiekrise 2022 deutlich ausgeweitet).

### 2.7  Weitere Zuständigkeiten

- **Entflechtung (Unbundling)** — Trennung von Netzbetrieb und Erzeugung/Vertrieb
- **Kohleausstieg** — Begleitung der Stilllegungsreihenfolge
- **E-Mobilität** — Überwachung der öffentlichen Ladeinfrastruktur
- **Stromspeicher** — Regulierung von Speicheranlagen
- **Smart Meter Rollout** — Zusammenarbeit mit BSI bei der Zertifizierung

---

## 3  Aktuelle Regulierungsverfahren (NEST-Prozess)

Der **NEST-Prozess** (Neufestlegung der Netzentgelt- und Anreizregulierung) ist das zentrale Reformpaket nach dem EuGH-Urteil:

| Verfahren | Kürzel | Gegenstand |
|-----------|--------|------------|
| Kapitalverzinsung & Produktivitätsfaktor | **RAMEN Strom/Gas** | Methoden der Kapitalverzinsung |
| Effizienzvergleich | **StromNEF / GasNEF** | SFA- und DEA-basierter Effizienzvergleich |
| Netzentgeltsystematik Strom | **AgNes** | Einspeiseentgelte, Kapazitätspreise |
| Netzentgeltsystematik Gas | **Sygne** | Überarbeitung der Gasnetzentgeltsystematik |
| Rückstellungen Gasnetz-Stilllegung | **Brücken** | Rückbau und Stilllegung von Erdgasnetzen |
| Kapazitätspreise Wasserstoff | **Kosmo** | Wasserstoff-Netzentgelte |
| Wasserstoff-Netzzugang | **WaKandA** | Grundmodell für H₂-Netzzugang |
| Qualitätsregulierung | – | SAIDI, ASIDI + neue Kennzahlen (Energiewendekompetenz, Digitalisierung) |

---

## 4  Zuständigkeitsabgrenzung

| Netzbetreiber | Zuständige Behörde |
|--------------|-------------------|
| Überregionale NB (> Landesgrenzen) | BNetzA |
| Regionale NB (innerhalb eines Landes) | Landesregulierungsbehörde |
| NB in BE, BB, HB, SH | BNetzA (per Organleihe) |

---

## 5  Relevanz für unser Projekt

| Thema | Relevanz |
|-------|----------|
| **BK6-Festlegungen** | Direkt — definieren alle Prozesse, die wir als Reducer modellieren |
| **Datenformat-Mitteilungen** | Direkt — verbindliche EDI@Energy-Versionen, Umsetzungstermine |
| **MaBiS-Hub (BK6-24-210)** | Strategisch — verändert die Bilanzierungs-Architektur fundamental |
| **LFW24 (BK6-22-024)** | Operativ — erster API-Webdienst, neue GPKE-Struktur |
| **NEST/AgNes** | Indirekt — Netzentgelt-Änderungen fließen in INVOIC-Inhalte |
| **WaKandA/Kosmo** | Perspektivisch — Wasserstoff als dritte Sparte |
| **SMARD** | Referenzdaten — Transparenzplattform für Marktdaten |

---

## 6  Quellen

| Quelle | URL |
|--------|-----|
| BNetzA Startseite | https://www.bundesnetzagentur.de |
| BK6 – Marktkommunikation | https://www.bundesnetzagentur.de → Fachthemen → Elektrizität und Gas → BK6 |
| SMARD | https://www.smard.de |
| Wikipedia | https://de.wikipedia.org/wiki/Bundesnetzagentur |
