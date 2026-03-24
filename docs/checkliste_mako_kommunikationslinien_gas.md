# Checkliste: Kommunikationslinien der Marktkommunikation – Gassparte

Systematische Erfassung aller gerichteten Nachrichtenflüsse zwischen Marktrollen im Gasbereich

Stand: März 2026

---

## Lesehinweise

**Notation:** `Sender → Empfänger : NACHRICHTENTYP (Prozess)` beschreibt eine gerichtete Kommunikationslinie. Jede Linie ist eine Funktion: `f(Input-Nachricht, Kontext) → Output-Nachricht | Ø`.

**Quittungsschicht (gilt für ALLE Linien):** Jede EDIFACT-Nachricht erzeugt automatisch zwei Quittungen in Gegenrichtung:

```
Empfänger → Sender : CONTRL   -- Syntaxprüfung (pure, deterministisch)
Empfänger → Sender : APERAK   -- Anwendungsprüfung (EBD-Logik, deterministisch)
```

Diese Quittungsschicht wird in den Prozess-Tabellen **nicht** einzeln aufgeführt, ist aber für jede Linie implizit vorhanden und muss im funktionalen Modell als Middleware/Decorator implementiert werden.

**Vertraulichkeit:** Formate und Prozesse = öffentlich. Inhalte (Stammdaten, Messwerte, Rechnungen) = vertraulich.

**Bezug zur Strom-Checkliste:** Dieses Dokument ist das Pendant zur bestehenden Kommunikationslinien-Checkliste (Strom). Gleiche Architektur-Prinzipien, gleiche Notation, gleiche Schichtenarchitektur. Strukturelle Unterschiede zur Stromsparte werden explizit markiert.

---

## 0  Rollen-Register Gas (Kommunikationsteilnehmer)

Alle Rollen, die in der Gas-Marktkommunikation als Sender oder Empfänger auftreten. Rollen, die es im Strombereich nicht gibt oder die anders belegt sind, sind mit **(Gas-spezifisch)** markiert.

| Kürzel | Rolle | MaKo-aktiv in |
|--------|-------|---------------|
| **LF** | Lieferant (Gaslieferant) | GeLi Gas, GABi Gas, KoV |
| **LFN** | Lieferant neu (bei Wechsel) | GeLi Gas (LFW) |
| **LFA** | Lieferant alt (bei Wechsel) | GeLi Gas (LFW) |
| **NB** | Netzbetreiber (VNB Gas) | GeLi Gas, GABi Gas, KoV |
| **MSB** | Messstellenbetreiber | GeLi Gas (Messwesen) |
| **FNB** | Fernleitungsnetzbetreiber **(Gas-spezifisch)** | KoV, GABi Gas, Kapazitätsmanagement |
| **MGV** | Marktgebietsverantwortlicher **(Gas-spezifisch)** | GABi Gas, Bilanzierung. Seit 01.10.2021: THE (Trading Hub Europe) als einziger MGV. |
| **BKV** | Bilanzkreisverantwortlicher | GABi Gas |
| **SSO** | Speicherstellenbetreiber (Storage System Operator) **(Gas-spezifisch)** | KoV, Kapazitätsmanagement |
| **TK** | Transportkunde **(Gas-spezifisch)** | KoV (bucht Kapazitäten bei FNB/SSO) |
| **BG** | Bilanzgruppenverwalter **(Gas-spezifisch)** | GABi Gas (veraltet, Funktion geht in BKV auf) |
| **ESA** | Energieserviceanbieter | GeLi Gas (Werte-Anfrage) |

---

## 1  Strukturelle Unterschiede Strom ↔ Gas

Bevor die einzelnen Kommunikationslinien aufgeführt werden, sind die fundamentalen Unterschiede zwischen den Sparten festzuhalten. Diese beeinflussen die Prozess-Reducer und die Fristberechnung direkt.

### 1.1  Gastag vs. Stromtag

| Merkmal | Strom | Gas |
|---------|-------|-----|
| **Tagesbeginn** | 00:00 Uhr (MEZ/MESZ) | 06:00 Uhr (MEZ/MESZ) |
| **Tagesende** | 24:00 Uhr | 06:00 Uhr Folgetag |
| **Zeitumstellung** | Tag hat 23 oder 25 Stunden | Gastag hat immer 24 Stunden (06:00→06:00 überlappt die Umstellung) |
| **DTM-Segment** | Kalendertagbezug | Gastagbezug (06:00 Offset) |

**Architektur-Implikation:** Die Fristberechnung `frist(datum, n_werktage, feiertagskalender) → stichtag` muss um einen Sparten-Parameter ergänzt werden: `frist(datum, n_werktage, feiertagskalender, sparte) → stichtag`. Der Gastag-Offset (06:00) ist eine reine Konfiguration, keine Logik-Änderung.

### 1.2  Bilanzierung

| Merkmal | Strom (MaBiS) | Gas (GABi Gas) |
|---------|---------------|----------------|
| **Bilanzierungszeitraum** | 15 Minuten (Viertelstunde) | 1 Stunde (seit THE-Zusammenlegung) |
| **Bilanzkreisstruktur** | BKV ↔ ÜNB | BKV ↔ MGV (THE) |
| **Profilverfahren** | SLP (Standardlastprofil) | SLP Gas (temperaturabhängig) |
| **Lastgang** | RLM (Registrierende Leistungsmessung) | RLM Gas (stündlich) |
| **Ausgleichsenergie** | ÜNB stellt bereit | MGV (THE) stellt bereit |
| **Allokation** | Durch NB | Durch NB, mit Allokationsschlüssel nach KoV |
| **Mehr-/Mindermengen** | NB → LF | NB → LF (analog Strom) |

### 1.3  Netzstruktur

| Merkmal | Strom | Gas |
|---------|-------|-----|
| **Übertragungsnetz** | 4 ÜNBs (50Hertz, Amprion, TenneT, TransnetBW) | 16 FNBs (z. B. OGE, GASCADE, bayernets, terranets bw) |
| **Marktgebiet** | 4 Regelzonen | 1 Marktgebiet (THE seit 01.10.2021) |
| **Kapazitätsmanagement** | Implizit (Netznutzung über NB) | Explizit (Kapazitätsbuchung bei FNB/SSO via PRISMA) |
| **Speicher** | Kein Pendant | SSO mit Ein-/Ausspeicherkapazitäten |

### 1.4  Übertragungswege Gas

| Kanal | Format | Status | Anmerkung |
|-------|--------|--------|-----------|
| **E-Mail (S/MIME)** | EDIFACT | Noch Standard (Gas) | Gas hat AS4 noch nicht flächendeckend eingeführt |
| **AS4 (eDelivery)** | EDIFACT | In Einführung | Für Gas zeitversetzt zum Strom; GeLi Gas 3.0 wird AS4 voraussichtlich vorschreiben |
| **PRISMA-Plattform** | Proprietär / API | Aktiv | Kapazitätsbuchungen FNB/SSO – außerhalb der EDIFACT-MaKo |
| **THE-Portal** | Proprietär / API | Aktiv | Bilanzkreismanagement, Nominierungen – teilweise außerhalb EDIFACT |

**Architektur-Implikation:** Der Transport-Adapter muss für Gas auch S/MIME (legacy) und perspektivisch PRISMA/THE-APIs abbilden können. Das Schichtenmodell bleibt gleich, die Adapter-Vielfalt wächst.

---

## 2  GeLi Gas – Geschäftsprozesse Lieferantenwechsel Gas

Rechtliche Grundlage: GeLi Gas 2.0 (aktuell), BK7-Festlegung. GeLi Gas 3.0 im Regulierungsverfahren (voraussichtlich 2026).

### 2.1  Lieferantenwechsel (LFW Gas)

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion | Frist |
|---|--------|-----------|-----------|-------------------|-------|
| 2.1.1 | LFN | NB | UTILMD | Anmeldung zur Netznutzung Gas (Lieferbeginn) | 10 WT vor Lieferbeginn (GeLi Gas 2.0) |
| 2.1.2 | NB | LFN | UTILMD | Bestätigung der Anmeldung | 5 WT |
| 2.1.3 | NB | LFA | UTILMD | Abmeldung (Kündigung durch Neulieferant) | Zeitgleich mit 2.1.2 |
| 2.1.4 | LFA | NB | UTILMD | Bestätigung der Abmeldung / Widerspruch | 3 WT |
| 2.1.5 | NB | LFN | UTILMD | Zuordnungsbestätigung | Nach Ablauf Widerspruchsfrist |
| 2.1.6 | NB | LFA | UTILMD | Abmeldebestätigung | Parallel zu 2.1.5 |

**Vergleich zu Strom:** Die Fristen im Gasbereich sind deutlich länger als nach LFW24 (Strom: 1 WT). GeLi Gas 3.0 wird die Fristen voraussichtlich an den Strom-Standard angleichen.

### 2.2  Lieferende / Abmeldung Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 2.2.1 | LF | NB | UTILMD | Abmeldung (Lieferende Gas) |
| 2.2.2 | NB | LF | UTILMD | Bestätigung der Abmeldung |
| 2.2.3 | NB | LF | MSCONS | Schlussturnusmesswert (Gastag-bezogen) |

### 2.3  Stammdatenänderung Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 2.3.1 | NB | LF | UTILMD | Stammdatenänderung an MaLo Gas (NB-initiiert) |
| 2.3.2 | LF | NB | UTILMD | Stammdatenänderung (LF-initiiert) |
| 2.3.3 | NB | MSB | UTILMD | Stammdatenänderung an MeLo Gas |
| 2.3.4 | MSB | NB | UTILMD | Stammdatenänderung (MSB-initiiert) |
| 2.3.5 | MSB | LF | UTILMD | Stammdatenänderung (informatorisch) |

### 2.4  Zuordnungsprozesse Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 2.4.1 | NB | LF | UTILMD | Zuordnungsliste Gas (regelmäßig/ad hoc) |
| 2.4.2 | NB | MSB | UTILMD | Zuordnungsliste MSB Gas |

### 2.5  Geschäftsdatenanfrage Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 2.5.1 | LF | NB | UTILMD | Geschäftsdatenanfrage (MaLo-Daten Gas) |
| 2.5.2 | NB | LF | UTILMD | Antwort Geschäftsdatenanfrage |

---

## 3  Messwesen Gas

Im Gasbereich gelten die WiM-Prozesse analog zum Strom, jedoch mit Gas-spezifischen MIG/AHB-Ausprägungen (UTILMD Gas, MSCONS Gas). Die Nachrichtentypen sind identisch, die Segmentbefüllung unterscheidet sich.

### 3.1  Gerätewechsel Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 3.1.1 | MSB | NB | UTILMD | Mitteilung Gerätewechsel (Gaszähler) |
| 3.1.2 | NB | LF | UTILMD | Information über Gerätewechsel |
| 3.1.3 | MSB | NB | MSCONS | Ausbau-/Einbauzählerstand (Gastag-Bezug) |
| 3.1.4 | MSB | LF | MSCONS | Ausbau-/Einbauzählerstand |

### 3.2  Zählwertübermittlung Gas (Regelprozess)

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 3.2.1 | MSB | NB | MSCONS | Turnusmesswerte / Lastgang Gas (SLP/RLM stündlich) |
| 3.2.2 | MSB | LF | MSCONS | Turnusmesswerte / Lastgang Gas |
| 3.2.3 | NB | LF | MSCONS | Plausibilisierte/korrigierte Messwerte |

**Gas-Besonderheit:** Gasmesswerte werden in Betriebskubikmeter (m³) gemessen und über Zustandszahl (z-Zahl), Brennwert und Abrechnungsbrennwert in kWh umgerechnet. Diese Umrechnung erfolgt durch den NB und ist eine pure Funktion: `umrechnung(volumen_m3, zustandszahl, brennwert) → energie_kwh`.

### 3.3  Werte-Anfrage Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 3.3.1 | LF | MSB | ORDERS | Anforderung historischer Messwerte Gas |
| 3.3.2 | MSB | LF | MSCONS | Antwort mit Messwerten |
| 3.3.3 | ESA | MSB | ORDERS | Anforderung Messwerte (via §52 MsbG) |
| 3.3.4 | MSB | ESA | MSCONS | Antwort mit Messwerten |

---

## 4  GABi Gas – Bilanzierung Gas

Rechtliche Grundlage: GaBi Gas 2.0 (BNetzA BK7), KoV Anlage 4. THE als zentraler MGV seit 01.10.2021.

### 4.1  Bilanzkreiszuordnung Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 4.1.1 | LF | NB | UTILMD | Zuordnung MaLo Gas zu Bilanzkreis |
| 4.1.2 | NB | LF | UTILMD | Bestätigung Bilanzkreiszuordnung Gas |

### 4.2  Bilanzierungsdaten Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 4.2.1 | NB | BKV | MSCONS | Allokationsdaten (SLP-Synthese, RLM-Lastgänge stündlich) |
| 4.2.2 | NB | MGV (THE) | MSCONS | Netzbetreiber-Summenzeitreihen Gas |
| 4.2.3 | MGV (THE) | BKV | MSCONS | Bilanzkreisabrechnungsdaten Gas |

**Gas-Besonderheit:** Die Allokation im Gasbereich folgt den Allokationsregeln der KoV (Anlage 4). Für SLP-Entnahmestellen wird ein temperaturabhängiges Standardlastprofil (SLP Gas) verwendet, das auf der Allokationstemperatur basiert. Die Allokation ist eine pure Funktion: `allokation(entnahme_slp, temperatur, profil, brennwert) → zeitreihe_kwh`.

### 4.3  Nominierung / Renominierung

Nominierungen sind eine Gas-Spezifik ohne Pendant im Strombereich. Der BKV meldet dem MGV (THE) seinen geplanten Gastransport an.

| # | Sender | Empfänger | Kanal | Inhalt / Funktion |
|---|--------|-----------|-------|-------------------|
| 4.3.1 | BKV | MGV (THE) | THE-Portal / EDIG@S | Nominierung (geplante Gasmengen pro Stunde) |
| 4.3.2 | MGV (THE) | BKV | THE-Portal / EDIG@S | Bestätigung / Matching-Ergebnis |
| 4.3.3 | BKV | MGV (THE) | THE-Portal / EDIG@S | Renominierung (Anpassung bis 2h vor Lieferung) |

**Hinweis:** Nominierungen laufen aktuell über das THE-Portal und das EDIG@S-Format (europäischer Standard für Gasnominierungen), nicht über EDIFACT-MaKo. Für unser funktionales Modell ist dies ein eigener Transport-Adapter. Die Nominierungs-Logik selbst ist pure: `nominierung(bilanzkreis, zeitreihe_soll, matching_rules) → bestätigung | ablehnung`.

### 4.4  Mehr-/Mindermengenabrechnung Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 4.4.1 | NB | LF | MSCONS | Mehr-/Mindermengenliste Gas |
| 4.4.2 | NB | LF | INVOIC | Mehr-/Mindermengenrechnung Gas |
| 4.4.3 | LF | NB | REMADV | Zahlungsavis |

### 4.5  Clearinglisten Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 4.5.1 | NB | LF | UTILMD | Clearingliste Gas (Stammdatenabgleich) |
| 4.5.2 | LF | NB | UTILMD | Antwort Clearingliste Gas |

---

## 5  KoV – Kooperationsvereinbarung Gas (Netzzugang & Kapazitätsmanagement)

Rechtliche Grundlage: KoV XI (aktuelle Fassung), abgestimmt zwischen allen FNBs und VNBs unter Koordination von THE.

### 5.1  Kapazitätsbuchung

Kapazitätsbuchungen laufen über die PRISMA-Plattform (europäische Kapazitätsbuchungsplattform), nicht über EDIFACT. Für unser Modell ist dies ein eigener Adapter.

| # | Sender | Empfänger | Kanal | Inhalt / Funktion |
|---|--------|-----------|-------|-------------------|
| 5.1.1 | TK | FNB/SSO | PRISMA | Kapazitätsbuchung (Entry/Exit) |
| 5.1.2 | FNB/SSO | TK | PRISMA | Buchungsbestätigung |
| 5.1.3 | TK | FNB/SSO | PRISMA | Sekundärhandel / Kapazitätsübertragung |

### 5.2  Netzkontoabrechnung

Die Netzkontoabrechnung regelt den finanziellen Ausgleich zwischen VNBs und FNBs für die Nutzung des vorgelagerten Netzes.

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 5.2.1 | FNB | NB (VNB) | INVOIC | Netzkontoabrechnung |
| 5.2.2 | NB (VNB) | FNB | REMADV | Zahlungsavis Netzkontoabrechnung |

### 5.3  Brennwert- und Zustandszahl-Übermittlung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 5.3.1 | FNB/NB | LF | MSCONS | Brennwertmitteilung (monatlich/abrechnungsrelevant) |
| 5.3.2 | FNB | NB (VNB) | MSCONS | Zustandszahl / Abrechnungsbrennwert |

**Architektur-Implikation:** Brennwert und Zustandszahl sind Eingabeparameter für die Mengenumrechnung (m³ → kWh). Sie werden als Kontext-Daten im Reducer vorgehalten: immutable, versioniert, pro Zeitraum gültig.

### 5.4  Netzanschluss & Ausspeiseverträge

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 5.4.1 | NB | FNB | UTILMD | Anmeldung/Änderung Ausspeisepunkt |
| 5.4.2 | FNB | NB | UTILMD | Bestätigung Ausspeisepunkt |

---

## 6  Netznutzungsabrechnung Gas (INVOIC/REMADV-Prozesse)

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 6.1 | NB | LF | INVOIC | Netznutzungsrechnung Gas |
| 6.2 | LF | NB | REMADV | Zahlungsavis auf Netznutzungsrechnung |
| 6.3 | MSB | LF | INVOIC | Rechnung Messstellenbetrieb Gas |
| 6.4 | LF | MSB | REMADV | Zahlungsavis Messstellenbetrieb |
| 6.5 | MSB | NB | INVOIC | Rechnung Messstellenbetrieb (bei grundzust. MSB = NB) |
| 6.6 | NB | MSB | REMADV | Zahlungsavis |

---

## 7  Marktpartner-Stammdaten & Querschnitt Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 7.1 | Alle MP | Alle MP | PARTIN | Marktpartner-Stammdaten Gas (MP-ID, Kontaktdaten, Rollen) |
| 7.2 | NB | LF | UTILTS | Zählzeitdefinitionen Gas |
| 7.3 | NB | LF | IFTSTA | Statusmeldungen zu Prozessschritten Gas |

---

## 8  Datenformate Gas – Abgrenzung zu Strom

Die Gas-MaKo nutzt dieselben EDIFACT-Nachrichtentypen wie Strom, jedoch mit eigenen MIG/AHB-Ausprägungen. Die Formatdokumente werden von DVGW Service & Consult (DVGW-SC) gepflegt, nicht von der BDEW-MaKo-Plattform (die primär Strom bedient).

### 8.1  Nachrichtentypen Gas vs. Strom

| Nachrichtentyp | Strom-MIG/AHB | Gas-MIG/AHB | Unterschiede |
|----------------|---------------|-------------|-------------|
| **UTILMD** | UTILMD Strom | UTILMD Gas | Andere Segmentgruppen für Gas-MaLo (z. B. Brennwert, Zustandszahl, Druckstufe, Zählwerk-Typ m³/kWh) |
| **MSCONS** | MSCONS Strom | MSCONS Gas | Gastag-Offset (06:00), stündliche statt viertelstündliche RLM-Werte, Mengeneinheiten m³ und kWh |
| **INVOIC** | INVOIC Strom | INVOIC Gas | Gas-spezifische Artikelnummern (Brennwert-Positionen, Zustandszahl-Positionen) |
| **REMADV** | REMADV (spartenübergreifend) | REMADV (spartenübergreifend) | Identisch |
| **ORDERS** | ORDERS Strom | ORDERS Gas | Minimale Unterschiede |
| **PRICAT** | PRICAT Strom | PRICAT Gas | Gas-Netzentgelte (Arbeitspreis, Leistungspreis Gas-spezifisch) |
| **CONTRL** | Spartenübergreifend | Spartenübergreifend | Identisch |
| **APERAK** | Spartenübergreifend | Spartenübergreifend | Identisch |
| **PARTIN** | Spartenübergreifend | Spartenübergreifend | Identisch |
| **IFTSTA** | Spartenübergreifend | Spartenübergreifend | Identisch |

### 8.2  Quellen Gas-Formate

| Quelle | URL | Inhalt |
|--------|-----|--------|
| **DVGW Service & Consult** | https://dvgw-sc.de | MIGs, AHBs, EBDs für Gas-Nachrichtentypen |
| **bdew-mako.de** | https://bdew-mako.de | Querschnittsdokumente (Allgemeine Festlegungen, Codelisten) – gelten für Strom und Gas |
| **BNetzA BK7** | https://bundesnetzagentur.de → BK07 | Gas-spezifische Festlegungen (GeLi Gas, GABi Gas) |
| **THE** | https://www.tradinghub.eu | Bilanzkreisverträge, Nominierungsregeln, Allokationsregeln |
| **PRISMA** | https://platform.prisma-capacity.eu | Kapazitätsbuchungsplattform (außerhalb EDIFACT-MaKo) |

---

## 9  Funktionale Modellierung Gas – Erweiterung der Architektur

### 9.1  Spartenparameter

Die bestehende Architektur (Reducer, pure Functions, Transport-Adapter) wird nicht geändert, sondern um einen Spartenparameter erweitert:

```
type Sparte = Strom | Gas | Wasserstoff  -- zukunftssicher

type MaKoFn = (Nachricht, Kontext, Sparte) → (Nachricht | Ablehnung | Ø)

type Kontext = {
  stammdaten    : Map MaLoId Stammdaten
  kalender      : Feiertagskalender
  zuordnungen   : ZuordnungsListe
  sparte        : Sparte
  gaskontext    : Maybe GasKontext  -- nur bei Sparte = Gas
}

type GasKontext = {
  brennwerte      : Map (MaLoId, Zeitraum) Brennwert
  zustandszahlen  : Map (MaLoId, Zeitraum) Zustandszahl
  allokationsregeln : AllokationsRegel
  gastagOffset    : Duration  -- 06:00:00
}
```

### 9.2  Schichtenmodell mit Gas-Erweiterung

```
┌─────────────────────────────────────────────────┐
│  Prozess-Schicht (State Machine / Reducer)       │
│  STROM: GPKE-LFW, WiM, MaBiS, MPES, RD, §14a  │
│  GAS:   GeLi Gas, GABi Gas, KoV                 │
│  State × Event → State × [Nachricht]            │
├─────────────────────────────────────────────────┤
│  Validierungs-Schicht (Pure Functions)           │
│  CONTRL: parse → syntax check (spartenübergr.)  │
│  APERAK: EBD → accept/reject (Strom/Gas-EBDs)   │
├─────────────────────────────────────────────────┤
│  Serialisierung (Codec)                          │
│  EDIFACT ↔ internes Modell (BO4E)               │
│  Strom-MIG ↔ BO4E ↔ Gas-MIG (je Ausprägung)    │
├─────────────────────────────────────────────────┤
│  Mengenumrechnung Gas (Pure Functions)           │
│  m³ × Zustandszahl × Brennwert → kWh            │
│  (nur Gas-Sparte, kein Pendant in Strom)         │
├─────────────────────────────────────────────────┤
│  Transport-Schicht (IO-Monade / Effekte)         │
│  AS4 / S/MIME / REST-API / CLS-Kanal            │
│  + PRISMA-Adapter + THE-Portal-Adapter           │
│  Signatur, Verschlüsselung, Zertifikate         │
└─────────────────────────────────────────────────┘
```

### 9.3  Gas-spezifische Einschränkungen / Nicht-triviale Stellen

| Thema | Herausforderung | Lösungsansatz |
|-------|-----------------|---------------|
| **Gastag-Offset** | 06:00-Beginn verschiebt alle Zeitreihenzuordnungen | Offset als Konfiguration im Kontext; Zeitreihen intern normalisieren |
| **Brennwert-Versionierung** | Brennwert ändert sich monatlich, rückwirkende Korrekturen möglich | Brennwert als immutable, versioniertes Kontext-Datum |
| **m³ → kWh Umrechnung** | Muss vor Bilanzierung, nach Messung stattfinden | Eigene pure Schicht zwischen Codec und Prozess |
| **Nominierung ≠ EDIFACT** | EDIG@S/THE-Portal statt EDIFACT | Eigener Transport-Adapter; Nominierungs-Logik als Reducer |
| **PRISMA ≠ EDIFACT** | Kapazitätsbuchung über proprietäre Plattform | Adapter-Schicht; Buchung als eigener Prozess-Reducer |
| **16 FNBs statt 4 ÜNBs** | Mehr Marktpartner, komplexeres Routing | Routing als pure Funktion auf MP-ID + Netzgebiet |
| **Temperaturabhängige SLP** | Gas-SLP hängt von Allokationstemperatur ab | Temperatur als Kontext-Parameter; SLP-Berechnung als pure Funktion |
| **GeLi Gas 3.0 (im Kommen)** | Regulierungsverfahren läuft, Fristen ändern sich | Fristen als Konfiguration, nicht als Hard-Code |

---

## 10  Vollständigkeits-Checkliste Gas

### 10.1  Prozessdomänen Gas

| # | Domäne | Abschnitt | Linien erfasst | Vollständig? |
|---|--------|-----------|----------------|-------------|
| 1 | GeLi Gas (LFW, Abmeldung, Stammdaten, Zuordnung) | §2 | LFW, Abmeldung, Stammdaten, Zuordnung, GDA | ✅ |
| 2 | Messwesen Gas | §3 | Gerätewechsel, Zählwerte, Werte-Anfrage | ✅ |
| 3 | GABi Gas (Bilanzierung) | §4 | BK-Zuordnung, Allokation, Nominierung, MeMinMe, Clearing | ✅ |
| 4 | KoV (Kapazität, Netzkonten, Brennwert) | §5 | Kapazitätsbuchung, Netzkontoabrechnung, Brennwert | ✅ |
| 5 | Netznutzungsabrechnung Gas | §6 | INVOIC/REMADV alle Richtungen | ✅ |
| 6 | Querschnitt Gas | §7 | PARTIN, UTILTS, IFTSTA | ✅ |

### 10.2  Nachrichtentypen Gas

| Nachrichtentyp | Verwendet in Abschnitt(en) | Abgedeckt? |
|----------------|---------------------------|------------|
| UTILMD Gas | §2, §4, §5, §7 | ✅ |
| MSCONS Gas | §2, §3, §4, §5 | ✅ |
| INVOIC Gas | §4, §6 | ✅ |
| REMADV | §4, §5, §6 | ✅ |
| ORDERS Gas | §3 | ✅ |
| PRICAT Gas | (implizit in Abrechnung) | ⚠️ Preisblatt-Prozess Gas noch nicht detailliert |
| IFTSTA | §7 | ✅ |
| CONTRL | Quittungsschicht (alle) | ✅ |
| APERAK | Quittungsschicht (alle) | ✅ |
| PARTIN | §7 | ✅ |
| UTILTS | §7 | ✅ |
| EDIG@S (Nominierung) | §4.3 | ✅ (nicht-EDIFACT) |

### 10.3  Rollen-Kommunikationsmatrix Gas

| Sender ↓ / Empfänger → | LF | NB | MSB | FNB | MGV (THE) | BKV | SSO | ESA |
|---|---|---|---|---|---|---|---|---|
| **LF** | – | UTILMD, MSCONS, REMADV, ORDERS | ORDERS, REMADV | – | – | – | – | – |
| **NB** | UTILMD, MSCONS, INVOIC, PRICAT, UTILTS, IFTSTA | – | UTILMD | UTILMD, MSCONS | MSCONS | – | – | – |
| **MSB** | UTILMD, MSCONS, INVOIC | UTILMD, MSCONS, INVOIC | – | – | – | – | – | MSCONS |
| **FNB** | – | INVOIC, MSCONS | – | – | MSCONS | – | – | – |
| **MGV (THE)** | – | – | – | – | – | MSCONS, EDIG@S | – | – |
| **BKV** | – | – | – | – | MSCONS, EDIG@S | – | – | – |
| **SSO** | – | – | – | Kapazitätsdaten | – | – | – | – |
| **ESA** | – | – | ORDERS | – | – | – | – | – |

---

## 11  Nächste Schritte Gas

1. **Gas-MIG/AHB von DVGW-SC beschaffen.** UTILMD Gas, MSCONS Gas, INVOIC Gas MIGs und AHBs als Quelldateien sichern.
2. **Gas-EBDs extrahieren.** Entscheidungsbäume für GeLi-Gas-Prozesse maschinenlesbar machen (analog rebdhuhn für Strom).
3. **Gastag-Fristberechnung implementieren.** Pure Funktion mit 06:00-Offset als Konfigurationsparameter.
4. **Mengenumrechnung als eigene Schicht.** `(m³, Zustandszahl, Brennwert) → kWh` als pure Funktion.
5. **EDIG@S/THE-Adapter evaluieren.** Nominierungsformat (EDIG@S) und THE-Portal-API dokumentieren.
6. **PRISMA-Adapter evaluieren.** Kapazitätsbuchungs-API dokumentieren (außerhalb EDIFACT-Scope, aber Teil des Gesamtsystems).
7. **GeLi Gas 3.0 beobachten.** Regulierungsverfahren verfolgen; Fristen und Prozessänderungen als Konfigurationsupdate einplanen.
8. **Testkorpus Gas ergänzen.** UTILMD Gas, MSCONS Gas (mit Gastag-Offset), INVOIC Gas als Testdaten generieren.
