# Zeitplan Marktkommunikation – Meilensteine & Ausblick

Vergangene und kommende regulatorische Änderungen der Marktkommunikation

Stand: März 2026

---

## 1  Vergangene Meilensteine

| Datum | Meilenstein | Kategorie | Beschreibung |
|-------|------------|-----------|--------------|
| **01.10.2017** | MaKo 2017 (Interimsmodell) | Prozesse & Formate | Einführung MaLo/MeLo-Modell, MSB als neue Marktrolle nach MsbG. Beginn der Trennung von Netz und Messwesen. |
| **01.12.2019** | MaKo 2020 – IT-Umsetzung | Prozesse & Formate | IT-seitige Umstellung auf neue Prozesse: GPKE, WiM, MaBiS, MPES in aktualisierter Fassung. MSB wird zentrale Datendrehscheibe für Messwerte. |
| **01.02.2020** | MaKo 2020 – Prozessstart | Prozesse & Formate | Fachlicher Start der MaKo 2020-Prozesse für alle Marktteilnehmer. |
| **01.10.2021** | Redispatch 2.0 | Netzbetrieb | Einbeziehung von EE- und KWK-Anlagen ab 100 kW in den Redispatch. Neue XML-basierte Datenformate für Fahrplan, Abruf, Stammdaten. |
| **01.07.2022** | EEG-Umlage auf null | Regulierung | Abschaffung der EEG-Umlage; seit 2023 endgültig entfallen. Vereinfachung der Abrechnungsprozesse. |
| **01.10.2023** | MaKo 2022 – Prozessstart | Prozesse & Formate | Erweiterte Netzzugangsprozesse, aktualisierte GPKE, WiM, MaBiS. Neue Stammdatenprozesse. |
| **01.04.2024** | AS4 Pflicht (Strom) | Übertragungswege | AS4 (eDelivery) wird Standard-Übertragungsweg für EDIFACT- und XML-Nachrichten im Strombereich. E-Mail/S/MIME und AS2 werden abgelöst. |
| **01.01.2025** | §14a EnWG – Steuerbare Verbraucher | Regulierung | Neue Regelung für netzorientierte Steuerung von Wärmepumpen, Wallboxen etc. Eigene Prozesse und Datenformate über CLS-Kanal/Steuerbox. |

---

## 2  Aktuelle Phase (2025)

| Datum | Meilenstein | Kategorie | Beschreibung |
|-------|------------|-----------|--------------|
| **04.04.2025** | LFW24 – IT-Umsetzung | Prozesse & Formate | IT-seitige Umstellung auf den 24h-Lieferantenwechsel. Erster API-Webdienst (MaLo-ID-Ermittlung via REST/JSON). |
| **06.06.2025** | LFW24 – Prozessstart | Prozesse & Formate | Fachlicher Start: Lieferantenwechsel in 24 Stunden (werktäglich). GPKE in vier Teilen (BK6-24-174). Neustrukturierung aller Stammdatenprozesse. |
| **Q3/2025** | AS4 Version 2.4 | Übertragungswege | Aktualisierte AS4-Regelungen mit erweiterten Services. |

---

## 3  Geplante Meilensteine (2026–2029)

| Datum | Meilenstein | Kategorie | Beschreibung |
|-------|------------|-----------|--------------|
| **Q1/2026** | MaBiS-Hub Festlegung | Prozesse & Formate | Abschluss Festlegungsverfahren BK6-24-210. Zentrale Plattform für Bilanzkreisabrechnung und Stammdatenaustausch. |
| **2026** | GeLi Gas 3.0 | Prozesse & Formate | Überarbeitete Geschäftsprozesse Lieferantenwechsel Gas. Regulierungsverfahren läuft. |
| **Q2/2026** | Weitere API-Webdienste | Übertragungswege | Erweiterung der REST-API-Prozesse über MaLo-ID-Ermittlung hinaus. |
| **01.04.2028** | MaBiS-Hub – IT-Start | Infrastruktur | Beginn der Einführungsphase des MaBiS-Hubs (BDEW-Einführungsszenario). |
| **01.10.2029** | MaBiS-Hub – Vollbetrieb | Infrastruktur | Ende der Einführungsphase. Hub als zentrale Plattform im Regelbetrieb. |

---

## 4  Langfristvision (~2032)

| Datum | Meilenstein | Kategorie | Beschreibung |
|-------|------------|-----------|--------------|
| **~2032** | Effiziente MaKo-Landschaft | Infrastruktur | BNetzA-Vision: vollständig API-basierte Marktkommunikation. EDIFACT endgültig abgelöst. Zentrale Hub-Technologien statt bilateralem Nachrichtenaustausch. |
| **~2032** | Wasserstoff-Integration | Regulierung | WasABi (BK7-24-01-014) und WaKandA (BK7-24-01-015): Marktkommunikation für Wasserstoff als dritte Sparte, strukturell analog zu GaBi Gas / KARLA Gas. |

---

## 5  Einordnung für unser Projekt

**Kurzfristig (2025–2026):** LFW24 und der erste API-Webdienst sind die unmittelbar relevanten Änderungen. Unser System muss sowohl die alten EDIFACT-Prozesse als auch die neuen REST-APIs unterstützen.

**Mittelfristig (2027–2029):** Das MaBiS-Hub wird die Architektur der Bilanzkreisabrechnung fundamental verändern – von bilateralen NB↔BKV-Nachrichten zu einer zentralen Plattform. Das betrifft direkt unsere Reducer-Architektur für MaBiS (Abschnitt 4 der Kommunikationslinien-Checkliste).

**Langfristig (~2032):** Die vollständige API-Basierung validiert unseren Ansatz. Die funktionale Schichtenarchitektur mit austauschbaren Transport-Adaptern (AS4 → REST → Hub-API) ist genau die richtige Vorbereitung.

**Wasserstoff:** Die Beschlüsse WasABi und WaKandA von November 2025 zeigen, dass eine dritte Sparte kommt. Wer die Architektur jetzt spartenübergreifend aufsetzt, kann Wasserstoff mit minimalem Aufwand integrieren.
