# Recherche: Gleichgesinnte Projekte – Maschinenlesbare Marktkommunikation

Bestehende Open-Source-Projekte und Akteure mit ähnlichem Ansatz

Stand: März 2026

---

## 1  Hochfrequenz Consulting GmbH (Berlin)

**Website:** https://www.hochfrequenz.de  
**GitHub:** https://github.com/Hochfrequenz  
**Relevanz:** ⭐⭐⭐⭐⭐ — Am weitesten fortgeschritten, größte Überlappung mit unserem Ansatz.

### 1.1  Projektlandschaft

| Repository | Beschreibung | Relevanz für uns |
|-----------|--------------|-----------------|
| **kohlrahbi** | AHB-Scraper → maschinenlesbare AHBs als JSON/CSV | Serialisierungs-Schicht (Codec) |
| **rebdhuhn** | EBD-Scraper → Entscheidungsbäume als Graphen | Validierungs-Schicht (APERAK-Logik) |
| **ebdamame** | EBD-Tabellen aus edi-energy.de extrahieren | Vorverarbeitung für rebdhuhn |
| **edi_energy_mirror** | Automatisierter Mirror aller edi-energy.de-Dokumente | Quelldaten für alles |
| **fundamend** | MIG maschinenlesbar machen | Serialisierungs-Schicht |
| **mig_ahb_utility_stack** | Parser für MIG/AHB-Strukturen | Tooling |
| **transformer.bee** | EDIFACT ↔ BO4E (JSON) Konverter | Codec: EDIFACT ↔ internes Modell |
| **mako.bee** | Workflow-Engine (ELSA-basiert) | Prozess-Schicht (aber kein funktionaler Ansatz) |
| **malo-id-generator** | Generiert valide MaLo-IDs für Tests | Test-Suite |
| **BO4E-python / BO4E-dotnet** | Business Objects for Energy – typisiertes Datenmodell | Datenmodell-Schicht |

### 1.2  Einordnung

Hochfrequenz arbeitet pragmatisch-werkzeuggetrieben: Scraper, Konverter, Workflow-Engine. Der Ansatz ist bottom-up (einzelne Tools für einzelne Probleme), nicht top-down (Gesamtarchitektur). Die Tools sind hochwertig und produktiv einsetzbar, aber es fehlt das architektonische Gesamtbild einer funktionalen Schichtenarchitektur.

**Empfehlung:** Kontakt aufnehmen. Die maschinenlesbaren EBDs, AHBs und MIGs könnten uns monatelange Arbeit ersparen.

---

## 2  BO4E e.V. (Business Objects for Energy)

**Website:** https://www.bo4e.de  
**GitHub:** https://github.com/bo4e  
**Relevanz:** ⭐⭐⭐⭐ — Standardisiertes Datenmodell, das als internes Modell in unserer Codec-Schicht dienen könnte.

### 2.1  Was BO4E ist

Ein offener Standard für typisierte Geschäftsobjekte der Energiewirtschaft. Definiert als JSON-Schemas Objekte wie `Marktlokation`, `Messlokation`, `Vertrag`, `Rechnung`, `Zählerstand` etc. Wird von Hochfrequenz, Robotron, und anderen Akteuren genutzt und weiterentwickelt.

### 2.2  Einordnung

BO4E ist das fehlende Bindeglied zwischen EDIFACT (extern) und unserem internen Datenmodell. Der `transformer.bee` von Hochfrequenz schlägt genau diese Brücke: `EDIFACT ↔ BO4E ↔ internes System`. Für unsere Architektur wäre BO4E ein Kandidat für die Codec-Schicht (`EDIFACT ↔ internes Datenmodell ↔ XML/JSON`).

---

## 3  STROMDAO GmbH

**GitHub:** https://github.com/STROMDAO  
**Relevanz:** ⭐⭐ — Kleinerer Scope, aber interessante Ansätze.

### 3.1  Relevante Projekte

| Repository | Beschreibung |
|-----------|--------------|
| **edifact-to-json** | Einfacher EDIFACT→JSON-Konverter |
| **mako-process-engine** | MaKo-Prozess-Engine (kleiner als mako.bee) |

### 3.2  Einordnung

Kleineres Team, weniger umfangreich als Hochfrequenz. Der edifact-to-json-Konverter ist ein guter Startpunkt, aber nicht so ausgereift wie transformer.bee. Die Prozess-Engine hat eine eigene Struktur, die nicht direkt kompatibel mit unserem funktionalen Ansatz ist.

---

## 4  Regulatorische Bestätigung

Die Branche bewegt sich in dieselbe Richtung wie unser Projekt:

| Signal | Beschreibung |
|--------|--------------|
| **API-Webdienste (ab 04/2025)** | BNetzA und BDEW führen REST/JSON als Übertragungsweg ein. Erste Swagger-Spezifikation für MaLo-ID-Ermittlung. |
| **Maschinenlesbare MIG/AHB (seit 2024)** | BDEW bietet erstmals maschinenlesbare Spezifikationen an (zuvor nur PDF/Word). |
| **MaBiS-Hub (~2028)** | Zentralisierung der Bilanzkreisabrechnung über Hub-Technologie – API-basiert. |
| **EDIFACT-Ablösung (~2032)** | BNetzA-Langfristvision: vollständig API-basierte Marktkommunikation. |

---

## 5  Vergleichsmatrix

| Dimension | Hochfrequenz | STROMDAO | BO4E | Unser Ansatz |
|-----------|-------------|----------|------|-------------|
| EBD als Code | ✅ ebdamame + rebdhuhn | ❌ | ❌ | ✅ geplant |
| EDIFACT↔JSON | ✅ transformer.bee | ✅ edifact-to-json | ❌ | ✅ Codec-Schicht |
| Typisiertes Datenmodell | ✅ BO4E-dotnet/Python | ⚠️ eigene Struktur | ✅ JSON Schemas | ✅ geplant |
| MIG/AHB maschinenlesbar | ✅ fundamend, kohlrahbi | ❌ | ❌ | ✅ Ziel |
| Prozess als State Machine | ⚠️ mako.bee (ELSA) | ⚠️ mako-process-engine | ❌ | ✅ Reducer-Architektur |
| Funktionale Modellierung | ❌ | ❌ | ❌ | ✅ **Kernidee** |
| Plain-Text (Markdown/YAML) | ❌ | ❌ | ✅ JSON Schema | ✅ **Kernprinzip** |
| Transport-Adapter (AS4/API) | ⚠️ teilweise | ❌ | ❌ | ✅ geplant |
| Gesamtarchitektur | ❌ werkzeuggetrieben | ❌ | ❌ | ✅ **Schichtenmodell** |

---

## 6  Alleinstellungsmerkmal unseres Projekts

Niemand, den wir finden konnten, hat die gesamte Kommunikationsstruktur als gerichteten Graphen von pure Functions modelliert. Unser Ansatz unterscheidet sich durch:

1. **Konsequent funktionale Modellierung** — pure Functions, Reducer, IO-Isolation
2. **Dokumentation in Plain-Text-Formaten** — Markdown, JSON, YAML statt Word/PDF
3. **Architektonisches Gesamtbild** — nicht einzelne Tools, sondern eine durchgängige Schichtenarchitektur
4. **EBDs als deterministische Funktionen** — nicht nur als Graphen visualisiert, sondern als ausführbare pure Functions

---

## 7  Empfehlung

1. **Hochfrequenz kontaktieren.** Die Überlappung ist so groß, dass Zusammenarbeit oder gegenseitige Nutzung der Open-Source-Tools offensichtlich Sinn ergibt.
2. **BO4E als internes Datenmodell evaluieren.** Statt ein eigenes Schema zu definieren, auf den bestehenden Standard aufbauen.
3. **kohlrahbi + rebdhuhn sofort nutzen.** Die maschinenlesbaren AHBs und EBDs sind die Grundlage für unsere Validierungs- und Codec-Schicht.
4. **transformer.bee als Referenz studieren.** Die EDIFACT↔BO4E-Konvertierung ist genau unsere Codec-Schicht.
