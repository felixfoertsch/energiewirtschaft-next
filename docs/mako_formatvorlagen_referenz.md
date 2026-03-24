# Formatvorlagen der Marktkommunikation

EDI@Energy · EDIFACT · XML · Übertragungswege

*Referenzdokument für Automatisierung und Maschinenlesbarkeit*

Stand: März 2026

Quellen: BDEW MaKo-Plattform (bdew-mako.de) · BNetzA Beschlusskammer 6 · EDI@Energy

> **Hinweis:** Die Formatdefinitionen und Prozesse sind öffentlich.
> **Die über diese Formate ausgetauschten Inhalte (Rechnungen, Stammdaten, Messwerte) sind vertraulich.**

---

## 1  EDIFACT-Nachrichtentypen

Die folgenden Nachrichtentypen bilden das Rückgrat der elektronischen Marktkommunikation in der deutschen Energiewirtschaft. Sie werden von der Projektgruppe EDI@Energy unter Federführung des BDEW erarbeitet und durch die BNetzA für alle Marktteilnehmer verbindlich festgelegt. Die Veröffentlichung erfolgt über die MaKo-Plattform (bdew-mako.de) und die BNetzA-Mitteilungen zu Datenformaten.

| Typ | Bezeichnung | Beschreibung / Verwendung |
|-----|-------------|---------------------------|
| **UTILMD** | Stammdaten | Übermittlung von Stammdaten zu Markt- und Messlokationen (An-/Abmeldung, Stammdatenänderung, Zuordnung). Zentraler Nachrichtentyp für GPKE, WiM, MPES. |
| **MSCONS** | Messwerte | Übermittlung von Zählerständen, Energiemengen und Lastgängen zwischen MSB/MDL, NB und LF. |
| **INVOIC** | Rechnung | Netznutzungs-, Energielieferungs- und Dienstleistungsrechnungen. Vertraulicher Inhalt, standardisiertes Format. |
| **REMADV** | Zahlungsavis | Zahlungsankündigung als Antwort auf eine INVOIC; referenziert Rechnungsnummer und Zahlungsbetrag. |
| **REQOTE** | Angebotsanfrage | Anfrage einer Konfiguration (z. B. Messprodukt, Schaltzeitdefinition) vom NB/LF an den MSB. |
| **QUOTES** | Angebot | Antwort des MSB auf eine REQOTE mit Preisangabe. |
| **ORDERS** | Bestellung | Bestellung einer Konfiguration oder eines Messprodukts; auch in MaBiS-Prozessen. |
| **ORDRSP** | Bestellbestätigung | Bestätigung oder Ablehnung einer ORDERS. |
| **ORDCHG** | Bestelländerung | Änderung einer bestehenden Bestellung. |
| **PRICAT** | Preiskatalog | Elektronisches Preisblatt (z. B. Preisblatt A des MSB, Netzentgeltpreisblätter). |
| **IFTSTA** | Statusmeldung | Übermittlung von Prüfergebnissen und Statuswerten zu Prozessschritten. |
| **CONTRL** | Syntaxprüfung | Automatische Empfangsbestätigung mit Ergebnis der Syntaxprüfung (UN/EDIFACT-Standard). |
| **APERAK** | Anwendungsprüfung | Ergebnis der AHB-Prüfung (Verarbeitbarkeitsprüfung); positiv oder mit Fehlercodes. |
| **PARTIN** | Marktpartner-Stammdaten | Austausch von Stammdaten der Marktpartner (MP-ID, Kontaktdaten, Marktrollen). |
| **UTILTS** | Berechnungsformel / Zeitreihentypen | Zählzeitdefinitionen, Berechnungsformeln, Aufteilungsfaktoren (z. B. für gemeinschaftliche Gebäudeversorgung §42b EnWG). |
| **HKNR-spezifische AHB** | Herkunftsnachweise | Datenformate für das Herkunftsnachweisregister (UBA) – Sonderprozess. |

---

## 2  XML-Formate (Redispatch 2.0)

Für den Redispatch 2.0 wurden eigene XML-basierte Datenformate definiert, die parallel zu den EDIFACT-Formaten existieren. Sie werden ebenfalls von der PG EDI@Energy erarbeitet und durch die BNetzA veröffentlicht. Die Formate sind abwärtskompatibel zu den Bestandsformaten der System Operation Guideline (SO GL) und des harmonisierten Aktivierungsprozesses (HAP).

| XML-Dokument | Beschreibung / Verwendung |
|--------------|---------------------------|
| **ActivationDocument** | Abruf- und Aktivierungsdaten für Redispatch-Maßnahmen; enthält Delta-/Sollwertanweisungen. |
| **PlannedResourceScheduleDocument** | Fahrplandaten (Planungsdaten) der technischen Ressourcen im Redispatch. |
| **Stammdaten (XML)** | Stammdaten der Steuerbaren und Technischen Ressourcen (TR-ID, SR-ID, Lokationszuordnung). |
| **Kostenblatt** | Kosteninformationen zu Redispatch-Maßnahmen. |
| **AcknowledgementDocument** | Bestätigungen und Quittungen für XML-Nachrichten im Redispatch-Prozess. |
| **NetworkConstraintDocument** | Engpassinformationen des Netzbetreibers. |
| **Unavailability_MarketDocument** | Nichtverfügbarkeitsmeldungen von Erzeugungsanlagen. |
| **StatusRequestMarketDocument** | Statusanfragen zu Redispatch-Prozessen. |
| **XSD-Dateien (informatorisch)** | XML-Schema-Definitionen zur maschinellen Validierung aller XML-Nachrichtentypen. |

---

## 3  Dokumenttypen je Nachrichtentyp

Für jeden Nachrichtentyp existieren typischerweise mehrere Begleitdokumente, die zusammen die vollständige Spezifikation ergeben. Für EDIFACT-Formate sind dies MIG und AHB, für XML-Formate AWT und FB. Die EBD gelten übergreifend.

| Kürzel | Bezeichnung | Beschreibung |
|--------|-------------|--------------|
| **MIG** | Message Implementation Guide | Technische Nachrichtenbeschreibung: definiert alle Segmente, Datenelemente, Qualifier und deren Struktur (Branching-Diagramm). Grundlage für die Implementierung in IT-Systemen. |
| **AHB** | Anwendungshandbuch | Fachliche Ausprägung der MIG: definiert je Prozessschritt, welche Felder Muss/Soll/Kann sind, welche Codes zu verwenden sind, und enthält Prüfidentifikatoren. |
| **EBD** | Entscheidungsbaum-Diagramme und Codelisten | Entscheidungslogik für Antwort-Nachrichten: systematische Prüfreihenfolge mit Antwortcodes bei Zustimmung/Ablehnung. |
| **AWT** | Anwendungstabelle (XML) | Pendant zum AHB für XML-Formate: fachliche Befüllungsregeln je Use-Case im Redispatch. |
| **FB** | Formatbeschreibung (XML) | Pendant zur MIG für XML-Formate: technische Struktur der XML-Nachricht. |

---

## 4  Prozessbeschreibungen (BNetzA-Festlegungen)

Die Prozessbeschreibungen werden von der BNetzA als rechtsverbindliche Festlegungen erlassen. Sie definieren die Abläufe, Fristen, Marktrollen und Geschäftsvorfälle der Marktkommunikation. Die EDI@Energy-Datenformate setzen diese Prozesse technisch um.

| Kürzel | Bezeichnung | Beschreibung / Aktueller Stand |
|--------|-------------|--------------------------------|
| **GPKE** | Geschäftsprozesse Kundenbelieferung Elektrizität | Lieferantenwechsel, Lieferbeginn/-ende, Stammdatenänderung, Zuordnungsprozesse, Konfigurationsbestellungen. Seit BK6-24-174 (06.06.2025) in vier Teilen: Teil 1 Einführung, Teil 2 Zuordnungsprozesse, Teil 3 Konfigurationen, Teil 4 Stammdatenprozesse (LFW24). |
| **WiM** | Wechselprozesse im Messwesen | MSB-Wechsel, Gerätewechsel, Zählwertübermittlung, Anfrage/Übermittlung von Werten (auch an ESA). |
| **MaBiS** | Marktregeln Bilanzkreisabrechnung Strom | Bilanzkreisabrechnung, Mehr-/Mindermengen, SLP-/RLM-Bilanzierung, Clearinglisten. Zukünftig: MaBiS Hub. |
| **MPES** | Marktprozesse Erzeugungsanlagen Strom | Zuordnungs- und Stammdatenprozesse für Erzeugungsanlagen. Wird sukzessive in GPKE integriert. |
| **GeLi Gas** | Geschäftsprozesse Lieferantenwechsel Gas | Pendant zur GPKE für die Gassparte. Aktuell: GeLi Gas 2.0; Regulierungsverfahren GeLi Gas 3.0 läuft. |
| **KoV** | Kooperationsvereinbarung Gas | Rahmenvertrag der Gasnetzbetreiber für Netzzugang, Bilanzierung und Kapazitätsmanagement Gas. |
| **LFW24** | Lieferantenwechsel in 24 Stunden | Festlegung BK6-22-024: beschleunigter werktäglicher LF-Wechsel und Neustrukturierung der Stammdatenprozesse. Gültig ab 04.04.2025 (IT) / 06.06.2025 (Prozesse). |
| **RD 2.0** | Redispatch 2.0 | Einbeziehung EE-/KWK-Anlagen ab 100 kW; Datenaustausch über XML-Formate zwischen ÜNB, VNB, BV, EIV. |
| **§14a EnWG** | Steuerbare Verbrauchseinrichtungen | Netzorientierte Steuerung von Wärmepumpen, Wallboxen etc. über CLS-Kanal/Steuerbox; eigene Prozesse und Datenformate. |
| **UBP** | Universalbestellprozess | Bestellung von Konfigurationen (Messprodukte, Schaltzeitdefinitionen, Leistungskurvendefinitionen) via REQOTE/QUOTES/ORDERS/ORDRSP. |

---

## 5  Übertragungswege

Die Marktkommunikation nutzt verschiedene Übertragungswege, die in den EDI@Energy-Regelungen spezifiziert sind. Seit April 2024 ist AS4 der Standard für EDIFACT-Nachrichten im Strom. Der Austausch erfolgt über BSI-zertifizierte Zertifikate mit Signatur und Verschlüsselung. Ab April 2025 kommen zusätzlich API-Webdienste (REST) hinzu.

| Kanal | Formate | Beschreibung |
|-------|---------|--------------|
| **AS4 (eDelivery)** | EDIFACT + XML | Neuer Standard-Übertragungsweg seit 04/2024 (Strom); basiert auf OASIS ebMS 3.0 / AS4-Profil. BSI-zertifizierte Zertifikate, signierte und verschlüsselte Übertragung. Drei Services: Test, Wechsel, Senden. |
| **E-Mail (S/MIME)** | EDIFACT | Historischer Übertragungsweg; S/MIME-verschlüsselt. Wird durch AS4 abgelöst, in Übergangsphase noch parallel nutzbar. |
| **AS2** | EDIFACT | Alternatives Protokoll (HTTP-basiert); ebenfalls durch AS4 abgelöst. |
| **API-Webdienste** | REST / JSON | Neuer Kanal ab 04.04.2025 für spezifische Prozesse (z. B. MaLo-ID-Ermittlung). Regelungen zum API-Webdienst 1.1. |
| **CLS-Kanal** | SMGW-intern | Steuerkanal über Smart Meter Gateway zur Ansteuerung steuerbarer Verbrauchseinrichtungen (§14a EnWG). |

---

## 6  Querschnittsdokumente und Regelwerke

Neben den nachrichtentypspezifischen MIGs und AHBs gibt es übergreifende Dokumente, die für alle Formate und Prozesse gelten. Diese bilden das gemeinsame Fundament der Marktkommunikation.

| Dokument | Beschreibung |
|----------|--------------|
| **Allgemeine Festlegungen (aktuell: Version 6.x)** | Übergreifende Regeln für alle EDIFACT- und XML-Nachrichten: Segmentstruktur, Namenskonventionen, Versionierung, Zeitangaben (UTC/MEZ/MESZ), MP-ID-Vergabe, Dateinamenkonventionen. |
| **Anwendungsübersicht der Prüfidentifikatoren** | Zuordnung aller Prüfidentifikatoren (Prüf-IDs) zu Prozessschritten und AHB-Anwendungsfällen über alle Nachrichtentypen hinweg. |
| **Entscheidungsbaum-Diagramme und Codelisten (EBD)** | Entscheidungslogik und Antwortcodes für Antwortnachrichten; spartenübergreifend (Strom und Gas). |
| **Codeliste der Artikelnummern und Artikel-ID** | Standardisierte Artikelbezeichnungen für Rechnungspositionen (INVOIC). |
| **Codeliste der OBIS-Kennzahlen und Medien** | Alle in EDI@Energy nutzbaren OBIS-Kennzahlen für Messwerte. |
| **Codeliste der Messprodukte** | Standardisierte Konfigurationsbezeichnungen für Messstellenbetrieb. |
| **Codeliste der Konfigurationen** | Konfigurationsprodukte für Schaltzeitdefinitionen und Leistungskurvendefinitionen. |
| **Regelungen zum Übertragungsweg (aktuell: 1.9)** | Technische Regeln für E-Mail/S/MIME und AS2: Zertifikate, Verschlüsselung, Signatur. |
| **Regelungen zum Übertragungsweg AS4 (aktuell: 2.4)** | Technische Regeln für AS4: Profil, Services, BSI-Zertifikate, Kommunikationsaufbau. |
| **AS4-Profil (aktuell: 1.1)** | Detailspezifikation des AS4-Profils für die Energiewirtschaft (Webservice-Endpoints, Header, Payloads). |
| **Regelungen zum API-Webdienst (aktuell: 1.1)** | Technische Spezifikation der neuen REST-API-Schnittstellen (z. B. MaLo-ID-Ermittlung). |
| **Regelungen zum Verzeichnisdienst (aktuell: 1.1)** | Zentraler Verzeichnisdienst für Zertifikate und Endpunkte der Marktteilnehmer. |
| **BDEW-Rollenmodell für die Marktkommunikation** | Definition aller Marktrollen und deren Kürzel (LF, NB, MSB, MDL, BKV, DP, ESA etc.). |
| **Feiertagskalender GPKE/GeLi Gas** | Jahreskalender der Werktage, relevant für alle Fristberechnungen in der Marktkommunikation. |

---

## 7  Versionierung und Konsultationsverfahren

Die EDI@Energy-Dokumente folgen einem festen Versionierungsschema (X.Yz) und einem halbjährlichen Konsultationsrhythmus. Die BNetzA veröffentlicht neue Versionen in nummerierten Mitteilungen (aktuell Nr. 48). Der typische Ablauf:

### 7.1  Konsultationsrhythmus

Die PG EDI@Energy erarbeitet Entwürfe → BNetzA veröffentlicht zur Konsultation (typischerweise 4 Wochen) → Konsultationssitzung mit Stellungnahmen → BNetzA veröffentlicht verbindlichen Stand. Umsetzungstermine sind typischerweise der 1. April und 1. Oktober eines Jahres.

### 7.2  Versionsnummern

X = Strukturänderung (neue/entfernte Segmente), Y = Textänderung (neue Qualifier, Codes), z = Fehlerkorrektur (Buchstabe). Beispiel: UTILMD Strom AHB 2.1 → Strukturversion 2, Textversion 1.

### 7.3  Vertraulichkeitshinweis

Die Formatdefinitionen (MIG, AHB, EBD, AWT, FB) sowie die Prozessbeschreibungen (GPKE, WiM, MaBiS etc.) sind öffentlich zugängliche Dokumente. Die über diese Formate ausgetauschten Inhalte – insbesondere Rechnungen (INVOIC), Zahlungsavise (REMADV), Stammdaten (UTILMD) und Messwerte (MSCONS) – sind dagegen vertrauliche Geschäftsdaten der beteiligten Marktteilnehmer.

---

## 8  Quellen und Bezugsadressen

Alle hier aufgeführten Formatvorlagen und Prozessbeschreibungen sind über die folgenden offiziellen Quellen kostenfrei abrufbar:

| Quelle | Beschreibung / URL |
|--------|---------------------|
| **BDEW MaKo-Plattform** | https://bdew-mako.de – zentrale Plattform mit allen EDI@Energy-Dokumenten, FAQ und Regelungen. Betrieben von Energie Codes und Services GmbH im Auftrag des BDEW. |
| **BNetzA Beschlusskammer 6 – Datenformate** | https://bundesnetzagentur.de → BK06 → Mitteilungen Datenformate – alle Konsultations- und Veröffentlichungsergebnisse mit verbindlichen EDI@Energy-Dokumenten. |
| **BNetzA Beschlusskammer 6 – GPKE** | https://bundesnetzagentur.de → BK06 → GPKE – aktuelle Prozessbeschreibungen (Teile 1–4), Lesefassungen, NNV, EDI-Vereinbarung. |
| **BNetzA Beschlusskammer 6 – MaKo 2022** | https://bundesnetzagentur.de → BK06 → MaKo 2022 – konsolidierte Lesefassungen GPKE, WiM, MPES, MaBiS. |
| **BDEW EDI@Energy-Seite** | https://bdew.de/service/edi-at-energy-dokumente/ – Einstiegsseite mit Verweis auf MaKo-Plattform und DVGW. |
| **DVGW Service & Consult** | https://dvgw-sc.de – Nachrichtentypen und Formate für die Gassparte (GeLi Gas, KoV). |
| **BDEW Anwendungshilfen** | https://bdew.de/service/anwendungshilfen/ – Umsetzungsfragenkataloge, Einführungsszenarien, Rollenmodell. |
| **BDEW Redispatch 2.0** | https://bdew.de/energie/redispatch-20-news/ – aktuelle Fehlerkorrekturen und Konsultationsergebnisse XML-Formate. |
