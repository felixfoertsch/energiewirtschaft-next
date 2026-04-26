# Checkliste: Kommunikationslinien der Marktkommunikation

Systematische Erfassung aller gerichteten Nachrichtenflüsse zwischen Marktrollen

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

---

## 0  Rollen-Register (Kommunikationsteilnehmer)

Alle Rollen, die in der Marktkommunikation als Sender oder Empfänger auftreten.

| Kürzel | Rolle | MaKo-aktiv in |
|--------|-------|---------------|
| **LF** | Lieferant | GPKE, WiM, MaBiS, MPES, UBP, §14a |
| **LFN** | Lieferant neu (bei Wechsel) | GPKE (LFW) |
| **LFA** | Lieferant alt (bei Wechsel) | GPKE (LFW) |
| **NB** | Netzbetreiber (VNB/ÜNB) | GPKE, WiM, MaBiS, MPES, RD 2.0, §14a |
| **MSB** | Messstellenbetreiber | WiM, GPKE, UBP, §14a |
| **MDL** | Messdienstleister | WiM (im Auftrag MSB) |
| **ÜNB** | Übertragungsnetzbetreiber | MaBiS, RD 2.0 |
| **BKV** | Bilanzkreisverantwortlicher | MaBiS |
| **BIKO** | Bilanzkoordinator | MaBiS |
| **EIV** | Einsatzverantwortlicher | RD 2.0 |
| **BV** | Betreiber von Erzeugungsanlagen | MPES, RD 2.0 |
| **DP** | Direktvermarkter | MPES |
| **ESA** | Energieserviceanbieter | WiM (Werte-Anfrage) |
| **AGG** | Aggregator | §14a, Flex-Prozesse |

---

## 1  GPKE – Kundenbelieferung Elektrizität

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

## 2  WiM – Wechselprozesse im Messwesen

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

### 2.4  Werte-Anfrage (auch ESA)

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 2.4.1 | LF | MSB | ORDERS | Anforderung historischer Messwerte |
| 2.4.2 | MSB | LF | MSCONS | Antwort mit Messwerten |
| 2.4.3 | ESA | MSB | ORDERS | Anforderung Messwerte (via §52 MsbG) |
| 2.4.4 | MSB | ESA | MSCONS | Antwort mit Messwerten |

---

## 3  UBP – Universalbestellprozess (Konfigurationen)

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
| 3.3.3 | NB | LF | PRICAT | Netzentgelt-Preisblätter |

---

## 4  MaBiS – Bilanzkreisabrechnung Strom

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

## 5  MPES – Marktprozesse Erzeugungsanlagen Strom

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 5.1 | BV/DP | NB | UTILMD | Anmeldung Erzeugungsanlage |
| 5.2 | NB | BV/DP | UTILMD | Bestätigung Zuordnung |
| 5.3 | NB | LF | UTILMD | Zuordnungsinformation EEG-Anlage |
| 5.4 | MSB | NB | MSCONS | Einspeise-Messwerte |
| 5.5 | NB | DP | MSCONS | Einspeise-Messwerte (zur Direktvermarktung) |
| 5.6 | NB | ÜNB | MSCONS | EEG-Einspeisezeitreihen |

---

## 6  Netznutzungsabrechnung (INVOIC/REMADV-Prozesse)

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 6.1 | NB | LF | INVOIC | Netznutzungsrechnung |
| 6.2 | LF | NB | REMADV | Zahlungsavis auf Netznutzungsrechnung |
| 6.3 | MSB | LF | INVOIC | Rechnung Messstellenbetrieb |
| 6.4 | LF | MSB | REMADV | Zahlungsavis Messstellenbetrieb |
| 6.5 | MSB | NB | INVOIC | Rechnung Messstellenbetrieb (bei grundzust. MSB = NB: intern) |
| 6.6 | NB | MSB | REMADV | Zahlungsavis |
| 6.7 | ÜNB | NB | INVOIC | Ausgleichsenergie-Abrechnung |
| 6.8 | NB | ÜNB | REMADV | Zahlungsavis |

---

## 7  Redispatch 2.0 (XML-Formate)

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

## 8  §14a EnWG – Steuerbare Verbrauchseinrichtungen

| # | Sender | Empfänger | Nachricht/Kanal | Inhalt / Funktion |
|---|--------|-----------|-----------------|-------------------|
| 8.1 | LF/AG | NB | UTILMD | Anmeldung steuerbare Verbrauchseinrichtung |
| 8.2 | NB | LF | UTILMD | Bestätigung / Zuordnung SR |
| 8.3 | NB | MSB | UTILMD | Konfiguration Steuerbox / CLS-Kanal |
| 8.4 | NB | SMGW/SR | CLS-Kanal | Steuersignal (Dimmung/Abschaltung) |
| 8.5 | MSB | NB | MSCONS | Messwerte steuerbare Verbrauchseinrichtung |
| 8.6 | NB | LF | MSCONS | Messwerte (nach Steuerung) |

---

## 9  Marktpartner-Stammdaten & Querschnitt

| # | Sender | Empfänger | Nachricht | Inhalt / Funktion |
|---|--------|-----------|-----------|-------------------|
| 9.1 | Alle MP | Alle MP | PARTIN | Marktpartner-Stammdaten (MP-ID, Kontaktdaten, Rollen) |
| 9.2 | NB | LF | UTILTS | Zählzeitdefinitionen, Berechnungsformeln |
| 9.3 | NB | MSB | UTILTS | Berechnungsformeln, Aufteilungsfaktoren (§42b EnWG) |
| 9.4 | NB | LF | IFTSTA | Statusmeldungen zu Prozessschritten |

---

## 10  API-Webdienste (ab 04.04.2025)

| # | Sender | Empfänger | Kanal | Inhalt / Funktion |
|---|--------|-----------|-------|-------------------|
| 10.1 | LF | NB | REST-API | MaLo-ID-Ermittlung |
| 10.2 | NB | LF | REST-API | Antwort MaLo-ID |

> **Hinweis:** Weitere API-Prozesse sind in Planung. Das API-Regelwerk (Version 1.1) definiert die technischen Spezifikationen.

---

## 11  Übertragungsweg-Matrix

Jede Kommunikationslinie nutzt einen definierten Übertragungsweg. Für das funktionale Modell ist dies eine Transport-Schicht unterhalb der fachlichen Funktionen.

| Kanal | Format | Status | Verwendung |
|-------|--------|--------|------------|
| **AS4** | EDIFACT + XML | Standard seit 04/2024 | Alle EDIFACT- und XML-Nachrichten (Strom) |
| **E-Mail (S/MIME)** | EDIFACT | Auslaufend | Parallelbetrieb in Übergangsphase |
| **AS2** | EDIFACT | Abgelöst | Durch AS4 ersetzt |
| **REST-API** | JSON | Neu ab 04/2025 | MaLo-ID-Ermittlung, weitere Prozesse geplant |
| **CLS-Kanal** | SMGW-proprietär | Aktiv | §14a-Steuerung über Smart Meter Gateway |

---

## 12  Funktionale Modellierung – Architektur-Skizze

### 12.1  Grundprinzip

Jede Kommunikationslinie wird als **pure Funktion** abgebildet:

```
type MaKoFn = (Nachricht, Kontext) → (Nachricht | Ablehnung | Ø)
```

Wobei:

- **Nachricht**: Typisierter EDIFACT/XML-Record (UTILMD, MSCONS, INVOIC, …)
- **Kontext**: Immutabler Snapshot (Stammdaten, Kalender, Zuordnungslisten)
- **Ablehnung**: APERAK mit Fehlercode (determiniert durch EBD)
- **Ø**: Kein Output (z. B. rein informatorische Nachricht)

### 12.2  Schichten

```
┌─────────────────────────────────────────────────┐
│  Prozess-Schicht (State Machine / Reducer)       │
│  GPKE-LFW, WiM, MaBiS, MPES, RD 2.0, §14a     │
│  State × Event → State × [Nachricht]            │
├─────────────────────────────────────────────────┤
│  Validierungs-Schicht (Pure Functions)           │
│  CONTRL: parse → syntax check                   │
│  APERAK: EBD-Entscheidungsbaum → accept/reject  │
├─────────────────────────────────────────────────┤
│  Serialisierung (Codec)                          │
│  EDIFACT ↔ internes Datenmodell ↔ XML/JSON      │
├─────────────────────────────────────────────────┤
│  Transport-Schicht (IO-Monade / Effekte)         │
│  AS4 / S/MIME / REST-API / CLS-Kanal            │
│  Signatur, Verschlüsselung, Zertifikate         │
└─────────────────────────────────────────────────┘
```

### 12.3  Warum das funktioniert

1. **EBDs sind pure Funktionen.** Jeder Entscheidungsbaum ist eine deterministische Abbildung: Eingangsdaten + Prüfschritte → Antwortcode. Kein Seiteneffekt.

2. **Nachrichten sind immutable.** Eine UTILMD wird erzeugt, signiert, gesendet – nie mutiert. Jede Änderung erzeugt eine neue Nachricht.

3. **Prozesse sind Zustandsautomaten.** Ein Lieferantenwechsel hat endliche Zustände (angemeldet, bestätigt, zugeordnet, aktiv). Der Übergang ist: `(Zustand, Nachricht) → (neuer Zustand, [Ausgabe-Nachrichten])` – ein klassischer Reducer.

4. **Fristen sind pure Kalender-Funktionen.** `frist(datum, n_werktage, feiertagskalender) → stichtag` – kein Seiteneffekt, deterministisch.

5. **Seiteneffekte sind isoliert.** Nur die Transport-Schicht hat IO (AS4-Versand, Zertifikatsprüfung). Alles darüber ist pure.

### 12.4  Einschränkungen / Nicht-triviale Stellen

| Thema | Herausforderung | Lösungsansatz |
|-------|-----------------|---------------|
| **Timeout / Fristablauf** | Impliziter Zustandswechsel ohne Nachricht | Timer als Event im Reducer modellieren |
| **Parallelität** | Mehrere LF melden gleichzeitig an | Konflikterkennung als pure Funktion auf Zuordnungsliste |
| **Rückfall auf Grundversorgung** | Implizite Zuordnung ohne explizite Nachricht | Als Default-Transition im Zustandsautomaten |
| **Gas-Spezifika** | GeLi Gas, KoV haben teils andere Prozess-Logik | Eigener Prozess-Typ, gleiche Architektur |
| **Redispatch-Kaskade** | ÜNB → NB → EIV mehrstufig | Komposition von Reducern |

---

## 13  Vollständigkeits-Checkliste

### 13.1  Prozessdomänen

| # | Domäne | Abschnitt | Linien erfasst | Vollständig? |
|---|--------|-----------|----------------|-------------|
| 1 | GPKE (Strom) | §1 | LFW, Abmeldung, Stammdaten, Zuordnung, GDA | ✅ |
| 2 | WiM | §2 | MSB-Wechsel, Gerätewechsel, Zählwerte, Werte-Anfrage | ✅ |
| 3 | UBP | §3 | REQOTE/QUOTES/ORDERS/ORDRSP, PRICAT | ✅ |
| 4 | MaBiS | §4 | BK-Zuordnung, Bilanzierung, MeMinMe, Clearing | ✅ |
| 5 | MPES | §5 | Anmeldung EE, Messwerte, Zuordnung | ✅ |
| 6 | Netznutzungsabrechnung | §6 | INVOIC/REMADV alle Richtungen | ✅ |
| 7 | Redispatch 2.0 | §7 | Stammdaten, Fahrplan, Abruf, Engpass, Kosten | ✅ |
| 8 | §14a EnWG | §8 | Anmeldung, Steuerung, Messwerte | ✅ |
| 9 | Querschnitt | §9 | PARTIN, UTILTS, IFTSTA | ✅ |
| 10 | API-Webdienste | §10 | MaLo-ID-Ermittlung | ⚠️ Erweiterbar |

### 13.2  Nachrichtentypen

| Nachrichtentyp | Verwendet in Abschnitt(en) | Abgedeckt? |
|----------------|---------------------------|------------|
| UTILMD | §1, §2, §4, §5, §8, §9 | ✅ |
| MSCONS | §1, §2, §4, §5, §8 | ✅ |
| INVOIC | §4, §6 | ✅ |
| REMADV | §4, §6 | ✅ |
| REQOTE | §3 | ✅ |
| QUOTES | §3 | ✅ |
| ORDERS | §2, §3 | ✅ |
| ORDRSP | §3 | ✅ |
| ORDCHG | §3 | ✅ |
| PRICAT | §3 | ✅ |
| IFTSTA | §9 | ✅ |
| CONTRL | Quittungsschicht (alle) | ✅ |
| APERAK | Quittungsschicht (alle) | ✅ |
| PARTIN | §9 | ✅ |
| UTILTS | §9 | ✅ |
| XML RD 2.0 (7 Typen) | §7 | ✅ |

### 13.3  Rollen-Kommunikationsmatrix

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

---

## 14  Nächste Schritte

1. **Formale Typdefinitionen.** Für jeden Nachrichtentyp ein Schema (JSON Schema / TypeScript-Typ) erstellen, abgeleitet aus MIG/AHB.
2. **EBD als Code.** Alle Entscheidungsbäume als deterministische Funktionen implementieren.
3. **Prozess-Reducer.** Pro Domäne (GPKE, WiM, …) einen Reducer mit expliziten Zuständen und Transitionen.
4. **Feiertagskalender.** Als reine Daten (YAML/JSON) pflegen, Fristberechnung als pure Funktion.
5. **Quittungs-Middleware.** CONTRL/APERAK als generische Wrapper um jede eingehende Nachricht.
6. **Transport-Adapter.** AS4, REST-API, CLS als austauschbare IO-Adapter unterhalb der fachlichen Schicht.
7. **Gas-Erweiterung.** GeLi Gas, KoV analog zu GPKE/MaBiS modellieren.
