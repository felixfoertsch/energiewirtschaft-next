# Checkliste: Kommunikationslinien der Marktkommunikation

Strom & Gas – Systematische Erfassung aller gerichteten Nachrichtenflüsse

Stand: März 2026

---

## Lesehinweise

**Notation:** `Sender → Empfänger : NACHRICHTENTYP (Prozess)` beschreibt eine gerichtete Kommunikationslinie. Jede Linie ist eine Funktion: `f(Input-Nachricht, Kontext) → Output-Nachricht | Ø`.

**Quittungsschicht (gilt für ALLE Linien beider Sparten):** Jede EDIFACT-Nachricht erzeugt automatisch zwei Quittungen in Gegenrichtung:

```
Empfänger → Sender : CONTRL   -- Syntaxprüfung (pure, deterministisch)
Empfänger → Sender : APERAK   -- Anwendungsprüfung (EBD-Logik, deterministisch)
```

Diese Quittungsschicht wird in den Prozess-Tabellen **nicht** einzeln aufgeführt, ist aber für jede Linie implizit vorhanden und muss im funktionalen Modell als Middleware/Decorator implementiert werden.

**Sparten-Kennzeichnung:** ⚡ = nur Strom, 🔥 = nur Gas, ⚡🔥 = spartenübergreifend identisch.

**Vertraulichkeit:** Formate und Prozesse = öffentlich. Inhalte (Stammdaten, Messwerte, Rechnungen, Nominierungen, Allokationen) = vertraulich.

**Öl:** Heizöl ist kein leitungsgebundener Energieträger, fällt nicht unter das EnWG und hat keine regulierte Marktkommunikation. Kein MaKo-System vorhanden.

---

## 0  Rollen-Register (Kommunikationsteilnehmer)

### 0.1  Spartenübergreifende Rollen ⚡🔥

| Kürzel | Rolle | MaKo-aktiv in |
|--------|-------|---------------|
| **LF** | Lieferant | GPKE, GeLi Gas, WiM, MaBiS, GaBi Gas, UBP, §14a |
| **LFN** | Lieferant neu (bei Wechsel) | GPKE (LFW), GeLi Gas (LFW) |
| **LFA** | Lieferant alt (bei Wechsel) | GPKE (LFW), GeLi Gas (LFW) |
| **NB** | Netzbetreiber (VNB/ÜNB bzw. VNB/FNB) | GPKE, GeLi Gas, WiM, MaBiS, GaBi Gas, MPES, RD 2.0, §14a, KoV |
| **MSB** | Messstellenbetreiber | WiM, GPKE, GeLi Gas, UBP, §14a |
| **MDL** | Messdienstleister | WiM (im Auftrag MSB) |
| **BKV** | Bilanzkreisverantwortlicher | MaBiS, GaBi Gas, KoV |

### 0.2  Nur Strom ⚡

| Kürzel | Rolle | MaKo-aktiv in |
|--------|-------|---------------|
| **ÜNB** | Übertragungsnetzbetreiber | MaBiS, RD 2.0 |
| **BIKO** | Bilanzkoordinator | MaBiS |
| **EIV** | Einsatzverantwortlicher | RD 2.0 |
| **BV** | Betreiber von Erzeugungsanlagen | MPES, RD 2.0 |
| **DP** | Direktvermarkter | MPES |
| **ESA** | Energieserviceanbieter | WiM (Werte-Anfrage) |
| **AGG** | Aggregator | §14a, Flex-Prozesse |

### 0.3  Nur Gas 🔥

| Kürzel | Rolle | MaKo-aktiv in |
|--------|-------|---------------|
| **FNB** | Fernleitungsnetzbetreiber | KoV, GaBi Gas, KARLA Gas |
| **MGV** | Marktgebietsverantwortlicher (= THE) | GaBi Gas, KoV, Nominierung |
| **TK** | Transportkunde | KoV, GaBi Gas (Nominierung, Kapazitätsbuchung) |
| **SSO** | Speicherstellenbetreiber | KoV, GaBi Gas (Ein-/Ausspeicherung) |
| **ENB** | Einspeisenetzbetreiber (Biogas-Kontext) | GaBi Gas (Biogas-Bilanzierung) |
| **ANB** | Ausspeisenetzbetreiber | GaBi Gas (Allokation, Bilanzierung) |

> **Hinweis Gas-Rollen:** Im Gasmarkt ist der „Transportkunde" (TK) oft identisch mit dem Lieferanten (LF) oder BKV – die KoV unterscheidet aber funktional. FNB und VNB agieren in der KoV als Ein- bzw. Ausspeisenetzbetreiber (ENB/ANB).

---

# TEIL A: STROM ⚡

---

## 1  GPKE – Kundenbelieferung Elektrizität ⚡

### 1.1  Lieferantenwechsel (LFW / LFW24)

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion | Frist |
|---|--------|-----------|-----------|-------------------|-------|
| 1.1.1 | LFN | NB | UTILMD | Anmeldung zur Netznutzung (Lieferbeginn) | Ab 04.04.2025: 1 WT vor Lieferbeginn |
| 1.1.2 | NB | LFN | UTILMD | Bestätigung der Anmeldung | 1 WT |
| 1.1.3 | NB | LFA | UTILMD | Abmeldung (Kündigung durch Neulieferant) | Zeitgleich mit 1.1.2 |
| 1.1.4 | LFA | NB | UTILMD | Bestätigung der Abmeldung / Ablehnung | 1 WT |
| 1.1.5 | NB | LFN | UTILMD | Zuordnungsbestätigung (Zuordnungsliste) | Nach Ablauf Widerspruchsfrist |
| 1.1.6 | NB | LFA | UTILMD | Zuordnungsbestätigung (Abmeldeliste) | Parallel zu 1.1.5 |

### 1.2  Lieferende / Abmeldung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 1.2.1 | LF | NB | UTILMD | Abmeldung (Lieferende) |
| 1.2.2 | NB | LF | UTILMD | Bestätigung der Abmeldung |
| 1.2.3 | NB | LF | MSCONS | Schlussturnusmesswert / Schlusszählerstand |

### 1.3  Stammdatenänderung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 1.3.1 | NB | LF | UTILMD | Stammdatenänderung an MaLo/MeLo (NB-initiiert) |
| 1.3.2 | LF | NB | UTILMD | Stammdatenänderung (LF-initiiert, z. B. Prognosegrundlage) |
| 1.3.3 | NB | MSB | UTILMD | Stammdatenänderung an MeLo |
| 1.3.4 | MSB | NB | UTILMD | Stammdatenänderung an MeLo (MSB-initiiert) |
| 1.3.5 | MSB | LF | UTILMD | Stammdatenänderung (informatorisch) |

### 1.4  Zuordnungsprozesse (GPKE Teil 2)

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 1.4.1 | NB | LF | UTILMD | Zuordnungsliste (regelmäßig/ad hoc) |
| 1.4.2 | NB | MSB | UTILMD | Zuordnungsliste MSB |
| 1.4.3 | MSB | NB | UTILMD | Zuordnungsliste MSB → NB |
| 1.4.4 | MSB | LF | UTILMD | Zuordnungsliste MSB → LF |

### 1.5  Geschäftsdatenanfrage

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 1.5.1 | LF | NB | UTILMD | Geschäftsdatenanfrage (MaLo-Daten) |
| 1.5.2 | NB | LF | UTILMD | Antwort Geschäftsdatenanfrage |

---

## 2  WiM – Wechselprozesse im Messwesen ⚡🔥

> WiM gilt für Strom und Gas. Seit WiM Gas 2.0 (BK7-19-001, veröffentlicht 08/2025) werden die Messwesenprozesse im Gas weitgehend an Strom angeglichen.

### 2.1  MSB-Wechsel

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 2.1.1 | MSB_neu | NB | UTILMD | Anmeldung MSB-Wechsel |
| 2.1.2 | NB | MSB_neu | UTILMD | Bestätigung / Ablehnung |
| 2.1.3 | NB | MSB_alt | UTILMD | Abmeldung (Information) |
| 2.1.4 | MSB_alt | NB | MSCONS | Schlusszählerstand |

### 2.2  Gerätewechsel

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 2.2.1 | MSB | NB | UTILMD | Mitteilung Gerätewechsel (Zählertausch) |
| 2.2.2 | NB | LF | UTILMD | Information über Gerätewechsel |
| 2.2.3 | MSB | NB | MSCONS | Ausbau-/Einbauzählerstand |
| 2.2.4 | MSB | LF | MSCONS | Ausbau-/Einbauzählerstand |

### 2.3  Zählwertübermittlung (Regelprozess)

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 2.3.1 | MSB | NB | MSCONS | Turnusmesswerte / Lastgang (SLP/RLM) |
| 2.3.2 | MSB | LF | MSCONS | Turnusmesswerte / Lastgang (SLP/RLM) |
| 2.3.3 | NB | LF | MSCONS | Plausibilisierte Messwerte (Ersatzwerte) |

### 2.4  Werte-Anfrage (auch ESA) ⚡

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 2.4.1 | LF | MSB | ORDERS | Anforderung historischer Messwerte |
| 2.4.2 | MSB | LF | MSCONS | Antwort mit Messwerten |
| 2.4.3 | ESA | MSB | ORDERS | Anforderung Messwerte (via §52 MsbG) |
| 2.4.4 | MSB | ESA | MSCONS | Antwort mit Messwerten |

---

## 3  UBP – Universalbestellprozess (Konfigurationen) ⚡🔥

### 3.1  Messprodukt-Bestellung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 3.1.1 | LF/NB | MSB | REQOTE | Angebotsanfrage Messprodukt |
| 3.1.2 | MSB | LF/NB | QUOTES | Angebot mit Preisangabe |
| 3.1.3 | LF/NB | MSB | ORDERS | Bestellung Messprodukt |
| 3.1.4 | MSB | LF/NB | ORDRSP | Bestätigung / Ablehnung |
| 3.1.5 | LF/NB | MSB | ORDCHG | Bestelländerung |

### 3.2  Schaltzeitdefinition / Leistungskurvendefinition

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 3.2.1 | LF/NB | MSB | REQOTE | Angebotsanfrage Schaltzeit-/Leistungskurvendefinition |
| 3.2.2 | MSB | LF/NB | QUOTES | Angebot |
| 3.2.3 | LF/NB | MSB | ORDERS | Bestellung |
| 3.2.4 | MSB | LF/NB | ORDRSP | Bestätigung |

### 3.3  Preisblatt-Veröffentlichung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 3.3.1 | MSB | NB | PRICAT | Preisblatt A (Messstellenbetrieb) |
| 3.3.2 | MSB | LF | PRICAT | Preisblatt A (Messstellenbetrieb) |
| 3.3.3 | NB | LF | PRICAT | Netzentgelt-Preisblätter ⚡ (Gas: noch nicht standardisiert) |

---

## 4  MaBiS – Bilanzkreisabrechnung Strom ⚡

### 4.1  Bilanzkreiszuordnung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 4.1.1 | LF | NB | UTILMD | Zuordnung MaLo zu Bilanzkreis |
| 4.1.2 | NB | LF | UTILMD | Bestätigung Bilanzkreiszuordnung |

### 4.2  Bilanzierungsdaten

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 4.2.1 | NB | BKV | MSCONS | Aggregierte Zeitreihen (SLP-Synthese, RLM-Lastgänge) |
| 4.2.2 | NB | ÜNB | MSCONS | Netzbetreiber-Summenzeitreihen |
| 4.2.3 | ÜNB/BIKO | BKV | MSCONS | Bilanzkreisabrechnungsdaten |
| 4.2.4 | BKV | ÜNB | MSCONS | Fahrpläne (Bilanzkreis-Soll) |

### 4.3  Mehr-/Mindermengenabrechnung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 4.3.1 | NB | LF | MSCONS | Mehr-/Mindermengenliste |
| 4.3.2 | NB | LF | INVOIC | Mehr-/Mindermengenrechnung |
| 4.3.3 | LF | NB | REMADV | Zahlungsavis (Antwort auf INVOIC) |

### 4.4  Clearinglisten

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 4.4.1 | NB | LF | UTILMD | Clearingliste (Stammdaten-Abgleich) |
| 4.4.2 | LF | NB | UTILMD | Antwort Clearingliste |

---

## 5  MPES – Marktprozesse Erzeugungsanlagen Strom ⚡

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 5.1 | BV/DP | NB | UTILMD | Anmeldung Erzeugungsanlage |
| 5.2 | NB | BV/DP | UTILMD | Bestätigung Zuordnung |
| 5.3 | NB | LF | UTILMD | Zuordnungsinformation EEG-Anlage |
| 5.4 | MSB | NB | MSCONS | Einspeise-Messwerte |
| 5.5 | NB | DP | MSCONS | Einspeise-Messwerte (zur Direktvermarktung) |
| 5.6 | NB | ÜNB | MSCONS | EEG-Einspeisezeitreihen |

---

## 6  Netznutzungsabrechnung (INVOIC/REMADV) ⚡🔥

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion | Sparte |
|---|--------|-----------|-----------|-------------------|--------|
| 6.1 | NB | LF | INVOIC | Netznutzungsrechnung | ⚡🔥 |
| 6.2 | LF | NB | REMADV | Zahlungsavis auf Netznutzungsrechnung | ⚡🔥 |
| 6.3 | MSB | LF | INVOIC | Rechnung Messstellenbetrieb | ⚡🔥 |
| 6.4 | LF | MSB | REMADV | Zahlungsavis Messstellenbetrieb | ⚡🔥 |
| 6.5 | MSB | NB | INVOIC | Rechnung Messstellenbetrieb (bei grundzust. MSB = NB: intern) | ⚡🔥 |
| 6.6 | NB | MSB | REMADV | Zahlungsavis | ⚡🔥 |
| 6.7 | ÜNB | NB | INVOIC | Ausgleichsenergie-Abrechnung | ⚡ |
| 6.8 | NB | ÜNB | REMADV | Zahlungsavis | ⚡ |
| 6.9 | MGV | BKV | INVOIC | Ausgleichsenergie-Abrechnung Gas | 🔥 |
| 6.10 | BKV | MGV | REMADV | Zahlungsavis | 🔥 |

---

## 7  Redispatch 2.0 (XML-Formate) ⚡

### 7.1  Stammdaten-Austausch

| # | Sender | Empfänger | Format | Inhalt / Funktion |
|---|--------|-----------|--------|-------------------|
| 7.1.1 | BV/EIV | NB (Anschluss-NB) | XML Stammdaten | Stammdaten Technische/Steuerbare Ressourcen |
| 7.1.2 | NB | ÜNB | XML Stammdaten | Weiterleitung aggregierter Stammdaten |
| 7.1.3 | NB/ÜNB | BV/EIV | AcknowledgementDocument | Bestätigung |

### 7.2  Fahrplan-Prozess

| # | Sender | Empfänger | Format | Inhalt / Funktion |
|---|--------|-----------|--------|-------------------|
| 7.2.1 | EIV/BV | NB | PlannedResourceScheduleDocument | Planungsdaten (Fahrplan) |
| 7.2.2 | NB | ÜNB | PlannedResourceScheduleDocument | Weiterleitung |
| 7.2.3 | ÜNB/NB | EIV/BV | AcknowledgementDocument | Bestätigung |

### 7.3  Abruf / Aktivierung

| # | Sender | Empfänger | Format | Inhalt / Funktion |
|---|--------|-----------|--------|-------------------|
| 7.3.1 | ÜNB | NB | ActivationDocument | Redispatch-Abruf (Sollwert) |
| 7.3.2 | NB | EIV/BV | ActivationDocument | Weiterleitung an Anlagenbetreiber |
| 7.3.3 | EIV/BV | NB | AcknowledgementDocument | Quittung / Umsetzungsbestätigung |
| 7.3.4 | NB | ÜNB | AcknowledgementDocument | Weiterleitung Quittung |

### 7.4  Engpass und Nichtverfügbarkeit

| # | Sender | Empfänger | Format | Inhalt / Funktion |
|---|--------|-----------|--------|-------------------|
| 7.4.1 | NB/ÜNB | ÜNB/NB | NetworkConstraintDocument | Engpassinformation |
| 7.4.2 | BV/EIV | NB | Unavailability_MarketDocument | Nichtverfügbarkeitsmeldung |
| 7.4.3 | NB | ÜNB | Unavailability_MarketDocument | Weiterleitung |

### 7.5  Kostenabrechnung RD 2.0

| # | Sender | Empfänger | Format | Inhalt / Funktion |
|---|--------|-----------|--------|-------------------|
| 7.5.1 | BV/EIV | NB | Kostenblatt (XML) | Kosteninformation Redispatch-Maßnahme |
| 7.5.2 | NB | ÜNB | Kostenblatt (XML) | Weiterleitung |

---

## 8  §14a EnWG – Steuerbare Verbrauchseinrichtungen ⚡

| # | Sender | Empfänger | Nachricht/Kanal | Inhalt / Funktion |
|---|--------|-----------|-----------------|-------------------|
| 8.1 | LF/AG | NB | UTILMD | Anmeldung steuerbare Verbrauchseinrichtung |
| 8.2 | NB | LF | UTILMD | Bestätigung / Zuordnung SR |
| 8.3 | NB | MSB | UTILMD | Konfiguration Steuerbox / CLS-Kanal |
| 8.4 | NB | SMGW/SR | CLS-Kanal | Steuersignal (Dimmung/Abschaltung) |
| 8.5 | MSB | NB | MSCONS | Messwerte steuerbare Verbrauchseinrichtung |
| 8.6 | NB | LF | MSCONS | Messwerte (nach Steuerung) |

---

# TEIL B: GAS 🔥

---

## 9  GeLi Gas – Lieferantenwechsel Gas 🔥

> GeLi Gas 2.0: Prozessänderungen ab 01.04.2026. GeLi Gas 3.0: Beschluss BK7-24-01-009 (09/2025), überführt GasNZV-Inhalte in BNetzA-Festlegung, im Wesentlichen inhaltsgleich zu GeLi Gas 2.0. LFW in 24h ist für Gas noch **nicht** vorgesehen.

### 9.1  Lieferantenwechsel

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion | Frist |
|---|--------|-----------|-----------|-------------------|-------|
| 9.1.1 | LFN | NB | UTILMD | Anmeldung Gaslieferung (Lieferbeginn) | GeLi Gas 2.0: Fristen analog Strom, aber kein 24h-LFW |
| 9.1.2 | NB | LFN | UTILMD | Bestätigung der Anmeldung | Gem. GeLi Gas |
| 9.1.3 | NB | LFA | UTILMD | Abmeldung (Kündigung durch Neulieferant) | Zeitgleich mit 9.1.2 |
| 9.1.4 | LFA | NB | UTILMD | Bestätigung / Ablehnung der Abmeldung | Gem. GeLi Gas |
| 9.1.5 | NB | LFN | UTILMD | Zuordnungsbestätigung | Nach Widerspruchsfrist |
| 9.1.6 | NB | LFA | UTILMD | Zuordnungsbestätigung (Abmeldeliste) | Parallel zu 9.1.5 |

### 9.2  Lieferende / Abmeldung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 9.2.1 | LF | NB | UTILMD | Abmeldung (Lieferende Gas) |
| 9.2.2 | NB | LF | UTILMD | Bestätigung der Abmeldung |
| 9.2.3 | NB | LF | MSCONS | Schlusszählerstand |

### 9.3  Stammdatenänderung Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 9.3.1 | NB | LF | UTILMD | Stammdatenänderung MaLo/MeLo Gas |
| 9.3.2 | LF | NB | UTILMD | Stammdatenänderung (LF-initiiert) |
| 9.3.3 | NB | MSB | UTILMD | Stammdatenänderung MeLo |
| 9.3.4 | MSB | NB | UTILMD | Stammdatenänderung (MSB-initiiert) |

### 9.4  Zuordnungsprozesse Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 9.4.1 | NB | LF | UTILMD | Zuordnungsliste Gas |
| 9.4.2 | NB | MSB | UTILMD | Zuordnungsliste MSB Gas |

---

## 10  GaBi Gas – Bilanzierung & Bilanzkreismanagement Gas 🔥

> Grundmodell der Ausgleichsleistungen und Bilanzierungsregeln im Gassektor. Datenformate über DVGW Service & Consult (DVGW-EDIFACT, Edig@s-basiert). GaBi Gas 2.1: Beschluss BK7-24-01-008, überführt GasNZV-Inhalte.

### 10.1  Gas-spezifische EDIFACT-Nachrichtentypen (DVGW)

| Typ | Bezeichnung | Beschreibung / Verwendung |
|-----|-------------|---------------------------|
| **NOMINT** | Nominierung | Nominierung von Gasmengen durch TK/BKV an NB/MGV (Ein-/Ausspeisung, VHP-Handel). Basiert auf ORDERS. |
| **NOMRES** | Nominierungsantwort | Bestätigung/Ablehnung einer Nominierung durch NB/MGV an TK/BKV. |
| **ALOCAT** | Allokation | Übermittlung allokierter Mengen (Ist-Mengen) an Ein-/Ausspeisepunkten. Von NB/MGV an TK/BKV. Basiert auf ORDRSP. |
| **IMBNOT** | Bilanzmeldung | Bilanz-/Imbalance-Meldung an BKV; enthält Bilanzkreissaldo und Toleranzen. Basiert auf ORDRSP. |
| **SSQNOT** | SLP-Allokation | Allokation synthetischer SLP-Mengen. Von ANB an TK/BKV. |
| **TRANOT** | Transportmeldung | Meldung über Gastransporte zwischen Netzbetreibern. |
| **TSIMSG** | Stammdaten Gas (Bilanzierung) | Stammdatenaustausch im GaBi-Kontext (Netzkonten, BK-Zuordnungen). Basiert auf UTILMD. |
| **DELORD** | Lieferauftrag | Kapazitätsbestellung/-änderung. |
| **DELRES** | Lieferantwort | Bestätigung/Ablehnung eines DELORD. |
| **SCHEDL** | Fahrplan Gas | Übermittlung von Gasfahrplänen. |
| **CHACAP** | Kapazitätsänderung | Kapazitätsänderungsmeldung. |
| **SLPASP** | SLP-Ausspeisezeitreihe | SLP-Prognosezeitreihen. |

> **Quittungsschicht Gas:** CONTRL und APERAK gelten identisch wie im Strom. Die DVGW-Nachrichtentypen nutzen seit 2009 die EDI@Energy CONTRL/APERAK-Logik.

### 10.2  Nominierungsprozess

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion | Frist |
|---|--------|-----------|-----------|-------------------|-------|
| 10.2.1 | TK/BKV | MGV (THE) | NOMINT | Nominierung am VHP (Virtueller Handelspunkt) | D-1, 14:00 Uhr (Day-Ahead) |
| 10.2.2 | MGV | TK/BKV | NOMRES | Bestätigung/Ablehnung Nominierung | Innerhalb Matching-Zyklus |
| 10.2.3 | TK/BKV | ENB/ANB | NOMINT | Nominierung an Ein-/Ausspeisepunkt | D-1, 14:00 Uhr |
| 10.2.4 | ENB/ANB | TK/BKV | NOMRES | Bestätigung Ein-/Ausspeise-Nominierung | D-1 |
| 10.2.5 | TK/BKV | MGV | NOMINT | Renominierung (untertägig) | Innerhalb Gastag (bis 2h vor Stunde) |
| 10.2.6 | MGV | TK/BKV | NOMRES | Bestätigung Renominierung | Innerhalb Zyklus |

> **Gastag:** Beginnt um 06:00 Uhr, endet 06:00 Uhr des Folgetages. Stundenwerte (24h, bei Zeitumstellung 23 oder 25).

### 10.3  Allokation

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion | Frist |
|---|--------|-----------|-----------|-------------------|-------|
| 10.3.1 | ANB | TK/BKV | ALOCAT | Allokation Ausspeisepunkt (RLM-Daten) | D+1 (vorläufig) |
| 10.3.2 | ENB | TK/BKV | ALOCAT | Allokation Einspeisepunkt | D+1 |
| 10.3.3 | ANB | TK/BKV | SSQNOT | Allokation SLP-Mengen | D+1 |
| 10.3.4 | MGV | BKV | ALOCAT | VHP-Allokation (Handelsnominierungen) | D+1 |
| 10.3.5 | ANB/ENB | TK/BKV | ALOCAT | Korrigierte Allokation | Bis M+29 WT |
| 10.3.6 | SSO | ENB/ANB | ALOCAT | Speicher-Allokation (Ein-/Ausspeicherung) | D+1 |

### 10.4  Bilanzkreis-Saldo & Imbalance

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 10.4.1 | MGV | BKV | IMBNOT | Täglicher Bilanzkreissaldo (vorläufig) |
| 10.4.2 | MGV | BKV | IMBNOT | Endgültiger Bilanzkreissaldo (nach Korrektur) |
| 10.4.3 | MGV | BKV | ALOCAT | Übertragung von Salden zwischen BK (bei Verbindung) |

### 10.5  Mehr-/Mindermengenabrechnung Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 10.5.1 | NB | LF | MSCONS | Mehr-/Mindermengenliste Gas |
| 10.5.2 | NB | LF | INVOIC | Mehr-/Mindermengenrechnung Gas |
| 10.5.3 | LF | NB | REMADV | Zahlungsavis |

### 10.6  Stammdaten Bilanzierung Gas

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 10.6.1 | TK/BKV | MGV | TSIMSG | Bilanzkreis-Stammdaten, Netzkontozuordnung |
| 10.6.2 | MGV | TK/BKV | TSIMSG | Bestätigung / Zuordnungsinformation |
| 10.6.3 | NB | MGV | TSIMSG | Netzkonto-Stammdaten, Kopplungspunkte |
| 10.6.4 | MGV | NB | TSIMSG | Bestätigung Netzkonto |

---

## 11  KoV / KARLA Gas – Kapazitätsmanagement & Netzzugang Gas 🔥

> Kooperationsvereinbarung Gas (KoV): Rahmenvertrag der Gasnetzbetreiber. KARLA Gas 2.0 (BK7-24-01-007): Kapazitätsregelungen und Abwicklung des Netzzugangs, überführt GasNZV-Inhalte.

### 11.1  Kapazitätsbuchung (Entry-Exit)

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 11.1.1 | TK | FNB/NB | DELORD | Kapazitätsbuchung (feste/unterbrechbare Kapazität) |
| 11.1.2 | FNB/NB | TK | DELRES | Bestätigung/Ablehnung Kapazitätsbuchung |
| 11.1.3 | TK | FNB/NB | CHACAP | Kapazitätsänderung / -übertragung |
| 11.1.4 | FNB/NB | TK | DELRES | Bestätigung Kapazitätsänderung |

### 11.2  Transportabwicklung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 11.2.1 | ENB/ANB | MGV | TRANOT | Transportmeldung (Netzübergänge, Kopplungspunkte) |
| 11.2.2 | MGV | ENB/ANB | TRANOT | Bestätigung / Information Transportmengen |
| 11.2.3 | NB | NB | SCHEDL | Gasfahrplan zwischen Netzbetreibern |

### 11.3  Deklaration & Zuordnung

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 11.3.1 | TK | NB | TSIMSG | Deklarationsmitteilung (BK-Zuordnung an Ausspeisepunkt) |
| 11.3.2 | NB | TK | TSIMSG | Bestätigung Deklaration |
| 11.3.3 | NB | MGV | TSIMSG | Weiterleitung Deklaration |
| 11.3.4 | MGV | NB | TSIMSG | Deklarationsclearing |

---

## 12  Biogas-Bilanzierung 🔥

> Sonderprozesse für Biogas-Bilanzkreise gemäß GaBi Gas und ZuBio-Festlegung (BK7-24-01-010).

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 12.1 | TK | ENB | TSIMSG | Zuordnung Einspeisepunkt zu Biogas-BK |
| 12.2 | ENB | MGV | TSIMSG | Bestätigung Biogas-Einspeisezuordnung |
| 12.3 | MGV | BKV | IMBNOT | Biogas-BK-Saldo (mit Flexibilitätsrahmen) |
| 12.4 | BKV | MGV | ALOCAT | Übertragung Saldo/Flexibilität (Rechnungs-BK) |

---

# TEIL C: QUERSCHNITT ⚡🔥

---

## 13  Marktpartner-Stammdaten & Querschnitt ⚡🔥

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 13.1 | Alle MP | Alle MP | PARTIN | Marktpartner-Stammdaten (MP-ID, Kontaktdaten, Rollen) |
| 13.2 | NB | LF | UTILTS | Zählzeitdefinitionen, Berechnungsformeln |
| 13.3 | NB | MSB | UTILTS | Berechnungsformeln, Aufteilungsfaktoren (§42b EnWG) |
| 13.4 | NB | LF | IFTSTA | Statusmeldungen zu Prozessschritten |

---

## 14  API-Webdienste (ab 04.04.2025) ⚡

| # | Sender | Empfänger | Kanal | Inhalt / Funktion |
|---|--------|-----------|-------|-------------------|
| 14.1 | LF | NB | REST-API | MaLo-ID-Ermittlung |
| 14.2 | NB | LF | REST-API | Antwort MaLo-ID |

> **Hinweis:** Weitere API-Prozesse in Planung. Gas nutzt ab 04/2025 ebenfalls AS4 für EDIFACT.

---

## 15  Übertragungsweg-Matrix ⚡🔥

| Kanal | Format | Status | Sparte | Verwendung |
|-------|--------|--------|--------|------------|
| **AS4** | EDIFACT + XML | Standard | ⚡ seit 04/2024, 🔥 seit 04/2025 | Alle EDIFACT- und XML-Nachrichten |
| **E-Mail (S/MIME)** | EDIFACT | Auslaufend | ⚡🔥 | Parallelbetrieb in Übergangsphase |
| **AS2** | EDIFACT | Abgelöst | ⚡🔥 | Durch AS4 ersetzt |
| **REST-API** | JSON | Neu ab 04/2025 | ⚡ | MaLo-ID-Ermittlung, weitere geplant |
| **CLS-Kanal** | SMGW-proprietär | Aktiv | ⚡ | §14a-Steuerung über Smart Meter Gateway |
| **DVGW-EDIFACT** | EDIFACT (Edig@s-basiert) | Aktiv | 🔥 | GaBi Gas / KoV-Nachrichtentypen via AS4/S/MIME |

---

## 16  Funktionale Modellierung – Architektur-Skizze

### 16.1  Grundprinzip

Jede Kommunikationslinie wird als **pure Funktion** abgebildet:

```
type MaKoFn = (Nachricht, Kontext) → (Nachricht | Ablehnung | Ø)
```

Wobei:

- **Nachricht**: Typisierter EDIFACT/XML-Record (UTILMD, MSCONS, INVOIC, NOMINT, ALOCAT, …)
- **Kontext**: Immutabler Snapshot (Stammdaten, Kalender, Zuordnungslisten, Netzkonten, Kapazitätsregister)
- **Ablehnung**: APERAK mit Fehlercode (determiniert durch EBD) bzw. NOMRES mit Ablehnungsgrund
- **Ø**: Kein Output (z. B. rein informatorische Nachricht)

### 16.2  Schichten

```
┌─────────────────────────────────────────────────────────┐
│  Prozess-Schicht (State Machine / Reducer)               │
│  GPKE-LFW, GeLi-LFW, WiM, MaBiS, GaBi Gas,            │
│  MPES, RD 2.0, §14a, KoV/KARLA, ZuBio                  │
│  State × Event → State × [Nachricht]                    │
├─────────────────────────────────────────────────────────┤
│  Validierungs-Schicht (Pure Functions)                   │
│  CONTRL: parse → syntax check                           │
│  APERAK: EBD-Entscheidungsbaum → accept/reject           │
│  NOMRES: Nominierungs-Matching → confirm/reject          │
├─────────────────────────────────────────────────────────┤
│  Serialisierung (Codec)                                  │
│  EDI@Energy-EDIFACT ↔ internes Modell ↔ XML/JSON        │
│  DVGW-EDIFACT (Edig@s) ↔ internes Modell                │
├─────────────────────────────────────────────────────────┤
│  Transport-Schicht (IO-Monade / Effekte)                 │
│  AS4 / S/MIME / REST-API / CLS-Kanal                    │
│  Signatur, Verschlüsselung, BSI-Zertifikate             │
└─────────────────────────────────────────────────────────┘
```

### 16.3  Warum das funktioniert (beide Sparten)

1. **EBDs sind pure Funktionen.** Jeder Entscheidungsbaum ist eine deterministische Abbildung: Eingangsdaten + Prüfschritte → Antwortcode. Kein Seiteneffekt. Gilt für Strom- und Gas-EBDs identisch.

2. **Nachrichten sind immutable.** Eine UTILMD oder NOMINT wird erzeugt, signiert, gesendet – nie mutiert. Jede Änderung erzeugt eine neue Nachricht.

3. **Prozesse sind Zustandsautomaten.** Ein Lieferantenwechsel (Strom wie Gas) hat endliche Zustände. Der Übergang ist: `(Zustand, Nachricht) → (neuer Zustand, [Ausgabe-Nachrichten])` – ein klassischer Reducer.

4. **Fristen sind pure Kalender-Funktionen.** `frist(datum, n_werktage, feiertagskalender) → stichtag` – deterministisch. Gas hat zusätzlich den Gastag (06:00–06:00) als Zeitbasis.

5. **Nominierung ist ein Matching-Problem.** `nomint_matching(BKV_nominierungen, NB_kapazitäten) → NOMRES[]` – deterministisch bei gegebenem Kontext, keine Seiteneffekte.

6. **Allokation ist eine pure Berechnung.** `allokation(messwerte, SLP_profile, brennwert, zustandszahl) → ALOCAT` – reine Datenverarbeitung.

7. **Seiteneffekte sind isoliert.** Nur die Transport-Schicht hat IO (AS4-Versand, Zertifikatsprüfung). Alles darüber ist pure.

### 16.4  Einschränkungen / Nicht-triviale Stellen

| Thema | Herausforderung | Lösungsansatz |
|-------|-----------------|---------------|
| **Timeout / Fristablauf** | Impliziter Zustandswechsel ohne Nachricht | Timer als Event im Reducer modellieren |
| **Parallelität** | Mehrere LF melden gleichzeitig an | Konflikterkennung als pure Funktion auf Zuordnungsliste |
| **Rückfall auf Grundversorgung** | Implizite Zuordnung ohne Nachricht | Default-Transition im Zustandsautomaten |
| **Gastag ≠ Kalendertag** | Gas-Bilanzierung auf 06:00–06:00-Basis | Eigener Zeittyp `GasDay` mit Konvertierung |
| **Brennwert & Zustandszahl** | Physikalische Umrechnung Gas (Nm³ → kWh) | Pure Funktion: `energie(volumen, brennwert, zustandszahl)` |
| **Renominierung** | Untertägige Änderungen von Nominierungen | State-Update im Nominierungs-Reducer mit Versionierung |
| **THE als zentraler Knoten** | MGV bündelt alle VHP-Nominierungen | Matching-Engine als pure Funktion; IO nur für Versand |
| **Redispatch-Kaskade (Strom)** | ÜNB → NB → EIV mehrstufig | Komposition von Reducern |
| **Biogas-BK-Flexibilität** | Saldenübertragung zwischen Biogas-BK | Eigener Reducer mit Flexibilitätslogik |
| **DVGW vs. EDI@Energy** | Zwei Formatquellen, eine Quittungsschicht | Gemeinsamer CONTRL/APERAK-Wrapper, separate Codecs |
| **GeLi Gas ≠ GPKE (Detail)** | Fristen, Prozessschritte teils abweichend | Eigener Prozess-Reducer, gleiche Architektur |

---

## 17  Vollständigkeits-Checkliste

### 17.1  Prozessdomänen

| # | Domäne | Abschnitt | Sparte | Vollständig? |
|---|--------|-----------|--------|-------------|
| 1 | GPKE (LFW, Stammdaten, Zuordnung, GDA) | §1 | ⚡ | ✅ |
| 2 | WiM (MSB-Wechsel, Gerätewechsel, Zählwerte) | §2 | ⚡🔥 | ✅ |
| 3 | UBP (REQOTE/QUOTES/ORDERS, PRICAT) | §3 | ⚡🔥 | ✅ |
| 4 | MaBiS (BK-Zuordnung, Bilanzierung, MeMinMe) | §4 | ⚡ | ✅ |
| 5 | MPES (Erzeugungsanlagen) | §5 | ⚡ | ✅ |
| 6 | Netznutzungsabrechnung (INVOIC/REMADV) | §6 | ⚡🔥 | ✅ |
| 7 | Redispatch 2.0 (XML) | §7 | ⚡ | ✅ |
| 8 | §14a EnWG (Steuerbare Verbraucher) | §8 | ⚡ | ✅ |
| 9 | GeLi Gas (LFW, Stammdaten, Zuordnung) | §9 | 🔥 | ✅ |
| 10 | GaBi Gas (Nominierung, Allokation, Bilanzierung) | §10 | 🔥 | ✅ |
| 11 | KoV/KARLA Gas (Kapazität, Transport, Deklaration) | §11 | 🔥 | ✅ |
| 12 | Biogas-Bilanzierung (ZuBio) | §12 | 🔥 | ✅ |
| 13 | Querschnitt (PARTIN, UTILTS, IFTSTA) | §13 | ⚡🔥 | ✅ |
| 14 | API-Webdienste | §14 | ⚡ | ⚠️ Erweiterbar |

### 17.2  Nachrichtentypen – Vollständigkeitsmatrix

| Nachrichtentyp | Quelle | Sparte | Verwendet in | Abgedeckt? |
|----------------|--------|--------|-------------|------------|
| UTILMD | EDI@Energy | ⚡🔥 | §1, §2, §4, §5, §8, §9, §13 | ✅ |
| MSCONS | EDI@Energy | ⚡🔥 | §1, §2, §4, §5, §8, §10 | ✅ |
| INVOIC | EDI@Energy | ⚡🔥 | §4, §6 | ✅ |
| REMADV | EDI@Energy | ⚡🔥 | §4, §6 | ✅ |
| REQOTE | EDI@Energy | ⚡🔥 | §3 | ✅ |
| QUOTES | EDI@Energy | ⚡🔥 | §3 | ✅ |
| ORDERS | EDI@Energy | ⚡🔥 | §2, §3 | ✅ |
| ORDRSP | EDI@Energy | ⚡🔥 | §3 | ✅ |
| ORDCHG | EDI@Energy | ⚡🔥 | §3 | ✅ |
| PRICAT | EDI@Energy | ⚡(🔥 geplant) | §3 | ✅ |
| IFTSTA | EDI@Energy | ⚡🔥 | §13 | ✅ |
| CONTRL | EDI@Energy | ⚡🔥 | Quittungsschicht | ✅ |
| APERAK | EDI@Energy | ⚡🔥 | Quittungsschicht | ✅ |
| PARTIN | EDI@Energy | ⚡🔥 | §13 | ✅ |
| UTILTS | EDI@Energy | ⚡🔥 | §13 | ✅ |
| XML RD 2.0 (7 Typen) | EDI@Energy | ⚡ | §7 | ✅ |
| NOMINT | DVGW | 🔥 | §10 | ✅ |
| NOMRES | DVGW | 🔥 | §10 | ✅ |
| ALOCAT | DVGW | 🔥 | §10, §11, §12 | ✅ |
| IMBNOT | DVGW | 🔥 | §10, §12 | ✅ |
| SSQNOT | DVGW | 🔥 | §10 | ✅ |
| TRANOT | DVGW | 🔥 | §11 | ✅ |
| TSIMSG | DVGW | 🔥 | §10, §11, §12 | ✅ |
| DELORD | DVGW | 🔥 | §11 | ✅ |
| DELRES | DVGW | 🔥 | §11 | ✅ |
| SCHEDL | DVGW | 🔥 | §11 | ✅ |
| CHACAP | DVGW | 🔥 | §11 | ✅ |
| SLPASP | DVGW | 🔥 | §10 | ✅ |

### 17.3  Rollen-Kommunikationsmatrix (Strom)

Sender ↓ / Empfänger → | LF | NB | MSB | ÜNB | BKV | EIV | BV | DP | ESA |
|---|---|---|---|---|---|---|---|---|---|
| **LF** | – | UTILMD, MSCONS, REMADV, ORDERS, REQOTE | ORDERS, REQOTE, REMADV | – | – | – | – | – | – |
| **NB** | UTILMD, MSCONS, INVOIC, PRICAT, UTILTS, IFTSTA | – | UTILMD, UTILTS | MSCONS, XML | – | XML | XML | – | – |
| **MSB** | UTILMD, MSCONS, INVOIC, PRICAT | UTILMD, MSCONS, INVOIC | – | – | – | – | – | – | MSCONS |
| **ÜNB** | – | XML, INVOIC | – | – | MSCONS | XML | – | – | – |
| **BKV** | – | – | – | MSCONS | – | – | – | – | – |
| **EIV** | – | XML | – | – | – | – | – | – | – |
| **BV** | – | UTILMD, XML | – | – | – | – | – | – | – |
| **DP** | – | UTILMD | – | – | – | – | – | – | – |
| **ESA** | – | – | ORDERS | – | – | – | – | – | – |

### 17.4  Rollen-Kommunikationsmatrix (Gas)

Sender ↓ / Empfänger → | LF | NB (ANB/ENB) | MSB | MGV (THE) | BKV/TK | FNB | SSO |
|---|---|---|---|---|---|---|---|
| **LF** | – | UTILMD, REMADV | ORDERS, REQOTE, REMADV | – | – | – | – |
| **NB (ANB/ENB)** | UTILMD, MSCONS, INVOIC | SCHEDL | UTILMD | TSIMSG, TRANOT | ALOCAT, SSQNOT | – | – |
| **MSB** | UTILMD, MSCONS, INVOIC | UTILMD, MSCONS | – | – | – | – | – |
| **MGV (THE)** | – | TSIMSG, TRANOT | – | – | NOMRES, ALOCAT, IMBNOT, TSIMSG, INVOIC | – | – |
| **BKV/TK** | – | NOMINT, TSIMSG | – | NOMINT, TSIMSG, ALOCAT, REMADV | – | DELORD, CHACAP | – |
| **FNB** | – | – | – | – | DELRES | – | – |
| **SSO** | – | ALOCAT | – | – | – | – | – |

---

## 18  Nächste Schritte

1. **Formale Typdefinitionen.** Für jeden Nachrichtentyp (EDI@Energy + DVGW) ein Schema (JSON Schema / TypeScript-Typ) erstellen, abgeleitet aus MIG/AHB bzw. DVGW-Nachrichtenbeschreibungen.
2. **EBD als Code.** Alle Entscheidungsbäume als deterministische Funktionen implementieren (Strom und Gas).
3. **Prozess-Reducer.** Pro Domäne einen Reducer mit expliziten Zuständen und Transitionen:
   - GPKE-LFW, GeLi-LFW (strukturell ähnlich, parametrisch unterschiedlich)
   - MaBiS, GaBi Gas (Bilanzierung als Zeitreihen-Verarbeitung)
   - KoV-Nominierung (Matching-Engine)
   - RD 2.0 (Kaskaden-Reducer)
4. **Zeitmodell.** Zwei Zeitbasen: Kalendertag (Strom, 00:00–00:00) und Gastag (06:00–06:00). Pure Konvertierungsfunktion.
5. **Feiertagskalender.** Als reine Daten (YAML/JSON), Fristberechnung als pure Funktion.
6. **Brennwert-/Zustandszahl-Berechnung.** Pure Funktion für Gas-Energieermittlung (DVGW G 685).
7. **Quittungs-Middleware.** CONTRL/APERAK als generischer Wrapper – identisch für EDI@Energy und DVGW-Nachrichten.
8. **Transport-Adapter.** AS4, REST-API, CLS als austauschbare IO-Adapter.
9. **DVGW-Codec.** Separater Codec für DVGW-EDIFACT (Edig@s-basiert) neben dem EDI@Energy-Codec.
10. **Wasserstoff-Vorbereitung.** BNetzA-Festlegungen WasABi (BK7-24-01-014) und WaKandA (BK7-24-01-015) beschlossen 11/2025 – strukturell analog zu GaBi Gas / KARLA Gas. Frühzeitig als dritte Sparte vorsehen.
