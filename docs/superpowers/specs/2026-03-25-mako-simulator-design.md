# MaKo-Simulator: Web-UI + CLI für fachlichen Systemtest

**Datum:** 2026-03-25
**Status:** Entwurf
**Vorgänger:** `2026-03-25-testkorpus-codec-design.md` (implementiert, 566 Tests)
**Ziel:** Ein dateibasierter Simulator, der alle Kommunikationslinien der deutschen Marktkommunikation durchspielbar macht. Sachbearbeiter und Entwickler können jede Marktrolle manuell bedienen, Nachrichten erzeugen, senden und empfangen — mit vollständiger Transparenz über den Prozessverlauf.

---

## 1  Scope & Abgrenzung

### Im Scope

- **Bedienungsanleitung** (`docs/anleitung.md`) — erklärt das Gesamtprojekt (Architektur, Typsystem, Codec, Testkorpus, Simulation) auf Deutsch, für Entwickler und Ingenieure teilbar
- **Rust-CLI** (`mako-cli/`) — neues Crate im Workspace, verarbeitet Nachrichten auf der Festplatte (parse → CONTRL → APERAK → Reducer → Antwort)
- **Node/React Web-App** (`mako-ui/`) — shadcn-basiertes Interface zum Bedienen aller Marktrollen, liest/schreibt Dateien auf der Festplatte
- **Dateibasierte Transportschicht** — Ordnerstruktur `markt/{rolle}/inbox|outbox/` als Nachrichtenaustausch

### Voraussetzungen (vor Phase C)

- **Reducer Message-Generation** — die meisten Reducer geben aktuell `nachrichten: vec![]` zurück. Für den Simulator müssen zumindest die GPKE-Reducer (LFW, Lieferende, Stammdaten) vollständige ausgehende Nachrichten erzeugen. Dies wird in Phase B.0 als erstes adressiert.

### Beziehung zu `mako-sim`

`mako-sim` existiert bereits als Crate mit `MarktAgent` + `Markt` (agentenbasierte Simulation). `mako-cli` **ersetzt `mako-sim` nicht** — es ist ein anderes Werkzeug:
- `mako-sim`: programmatische Simulation (Rust-Code), für automatisierte Tests
- `mako-cli`: dateibasierte Simulation (CLI + Dateisystem), für manuellen fachlichen Test

`mako-cli` nutzt die gleichen Prozess-Crates, aber nicht `mako-sim`. Beide koexistieren.

### Außerhalb Scope

- Worker-Deployment (Cloudflare, FaaS)
- HTTP-API zwischen UI und Engine (kommt später, wenn Workers stehen)
- Produktions-UI (Design, Robustheit, Fehlerbehandlung)
- RD 2.0 automatische ACK-Erzeugung (separates Todo)

---

## 2  Architektur

### 2.1  Systemübersicht

```
┌─────────────────────────────────┐
│  Node/React Web-App (mako-ui)   │
│  - Tab pro Marktrolle            │
│  - Inbox/Outbox Browser          │
│  - Formular → JSON/EDIFACT       │
│  - Aufgaben-Queue                │
│  - Prozess-Timeline              │
│  - Nachrichten-Status (✓✓✓)     │
│  - fs.watch auf markt/           │
└────────────┬────────────────────┘
             │ liest/schreibt Dateien
             ▼
┌─────────────────────────────────┐
│  Dateisystem: markt/             │
│  ├── lieferant_neu/inbox|outbox  │
│  ├── netzbetreiber/inbox|outbox  │
│  ├── lieferant_alt/inbox|outbox  │
│  ├── messstellenbetreiber/...    │
│  └── log/2026-03-25.jsonl        │
└────────────┬────────────────────┘
             │ mako-cli verarbeite
             ▼
┌─────────────────────────────────┐
│  Rust-CLI (mako-cli)             │
│  - parse (EDIFACT/JSON → Typen)  │
│  - CONTRL/APERAK Prüfung         │
│  - Reducer (State Machine)       │
│  - serialize (Typen → EDIFACT)   │
│  - Dateien in outbox schreiben   │
└─────────────────────────────────┘
```

### 2.2  Ordnerstruktur

```
markt/
├── lieferant_neu/
│   ├── inbox/              ← eingehende Nachrichten
│   │   ├── 001_utilmd_bestaetigung.edi
│   │   └── 001_utilmd_bestaetigung.json   ← parallel: maschinenlesbar
│   ├── outbox/             ← ausgehende Nachrichten
│   │   └── 001_utilmd_anmeldung.edi
│   └── state.json          ← aktueller Reducer-State pro Prozess
├── netzbetreiber/
│   ├── inbox/
│   ├── outbox/
│   └── state.json
├── lieferant_alt/
│   ├── inbox/
│   ├── outbox/
│   └── state.json
├── messstellenbetreiber/
│   ├── inbox/
│   ├── outbox/
│   └── state.json
├── bilanzkreisverantwortlicher/
│   ├── inbox/
│   ├── outbox/
│   └── state.json
├── marktgebietsverantwortlicher/
│   ├── inbox/
│   ├── outbox/
│   └── state.json
└── log/
    └── 2026-03-25.jsonl    ← chronologisches Log aller Nachrichten
```

### 2.3  Nachrichtenfluss

1. **Erstellen:** Sachbearbeiter füllt Formular im UI aus → UI generiert JSON + EDIFACT → Datei in `rolle/outbox/`
2. **Senden:** UI kopiert Datei von `absender/outbox/` nach `empfaenger/inbox/` → Status: ✓✓ Zugestellt
3. **Verarbeiten:** `mako-cli verarbeite empfaenger/inbox/datei.edi` → CONTRL → APERAK → Reducer → Antworten in `empfaenger/outbox/`
4. **Quittungen:** CONTRL/APERAK-Dateien automatisch in `absender/inbox/` → Status: ✓ CONTRL ✓ APERAK
5. **Nächster Schritt:** UI zeigt "Aktion erforderlich" an, Sachbearbeiter wechselt Rolle

### 2.4  Nachrichten-Status (WhatsApp-Style)

Jede Nachricht durchläuft eine Statusfolge:

| Status | Symbol | Bedeutung | Wann |
|--------|--------|-----------|------|
| Erstellt | ✓ | Datei liegt in outbox | Nach Formular-Submit |
| Zugestellt | ✓✓ | Datei liegt in empfänger/inbox | Nach cp/mv |
| CONTRL positiv | ✓ CONTRL | Syntaxprüfung bestanden | Nach mako-cli |
| CONTRL negativ | ✗ CONTRL | Syntaxfehler, Prozess stoppt | Nach mako-cli |
| APERAK positiv | ✓ APERAK | Anwendungsprüfung bestanden | Nach mako-cli |
| APERAK negativ | ✗ APERAK | Anwendungsfehler, Prozess stoppt | Nach mako-cli |
| Verarbeitet | ✓✓✓ | Reducer hat Antwort erzeugt | Nach mako-cli |

Status wird in einer `.status.json`-Datei neben jeder Nachricht gespeichert:

```json
{
  "datei": "001_utilmd_anmeldung.edi",
  "erstellt": "2026-03-25T12:30:00",
  "zugestellt": "2026-03-25T12:30:01",
  "contrl": { "ergebnis": "positiv", "zeitpunkt": "2026-03-25T12:30:02" },
  "aperak": { "ergebnis": "positiv", "zeitpunkt": "2026-03-25T12:30:02" },
  "verarbeitet": "2026-03-25T12:30:03"
}
```

### 2.5  Prozess-Identifikation

Ein Prozess wird durch die **MaLo-ID** (oder MeLo-ID) identifiziert. Dateinamen folgen dem Schema `{seq}_{typ}_{malo_kurz}.edi`, z.B. `001_utilmd_anmeldung_51238.edi`. Die Sequenznummer (`{seq}`) ist eine monoton steigende Zahl pro Rolle.

`state.json` enthält eine Map von Prozess-Keys auf Reducer-States:

```json
{
  "gpke_lfw/51238696700": {
    "prozess": "gpke_lfw",
    "malo_id": "51238696700",
    "state": "AbmeldungAnLfaGesendet",
    "state_data": { "lfn": "9900000000000", "lfa": "9900000000002", "lieferbeginn": "2026-07-01" },
    "letzte_aktion": "2026-03-25T12:30:03"
  },
  "gpke_lieferende/51238696700": { ... }
}
```

### 2.6  Atomizität der Verarbeitung

`mako-cli verarbeite` führt alle Schritte atomar aus:
1. Nachricht parsen
2. CONTRL prüfen → Quittung sofort in `absender/inbox/` schreiben
3. APERAK prüfen → Quittung sofort in `absender/inbox/` schreiben
4. Reducer ausführen → State in `state.json` aktualisieren
5. Ausgehende Nachrichten in `outbox/` schreiben
6. `.status.json` der Originalnachricht aktualisieren

Dateioperationen sind nicht transaktional — bei Absturz können inkonsistente Zustände entstehen. Für den fachlichen Test ist das akzeptabel; `mako-cli init` setzt alles zurück.

---

## 3  Rust-CLI (`mako-cli`)

### 3.1  Befehle

```bash
# Ordnerstruktur initialisieren
mako-cli init markt/

# Nachricht verarbeiten (parse → CONTRL → APERAK → Reducer → outbox)
mako-cli verarbeite markt/netzbetreiber/inbox/001_utilmd_anmeldung.edi

# Nachricht senden (outbox → inbox des Empfängers)
mako-cli sende markt/ lieferant_neu netzbetreiber 001_utilmd_anmeldung.edi

# Alle unverarbeiteten Nachrichten verarbeiten
mako-cli verarbeite-alle markt/netzbetreiber/

# Status anzeigen
mako-cli status markt/
```

### 3.2  Verarbeitungslogik

```rust
fn verarbeite(pfad: &Path, markt_dir: &Path) -> Result<()> {
    // 1. Datei lesen (EDIFACT oder JSON)
    // 2. parse_nachricht() → Nachricht
    // 3. CONTRL prüfen → Quittung in absender/inbox/
    // 4. APERAK prüfen → Quittung in absender/inbox/
    // 5. State laden (state.json)
    // 6. Nachricht → Event (neues Mapping!)
    // 7. Reducer aufrufen → neuer State + ausgehende Nachrichten
    // 8. State speichern
    // 9. Ausgehende Nachrichten in outbox/ schreiben
    // 10. Status-Datei aktualisieren (.status.json)
}
```

### 3.3  Nachricht → Event Mapping

Neues Modul: bildet `Nachricht` → prozessspezifisches `Event` ab. Beispiel:

```rust
fn nachricht_to_lfw_event(nachricht: &Nachricht) -> Option<LfwEvent> {
    match &nachricht.payload {
        NachrichtenPayload::UtilmdAnmeldung(a) => Some(LfwEvent::AnmeldungEmpfangen(a.clone())),
        NachrichtenPayload::UtilmdAblehnung(a) => Some(LfwEvent::LfaHatAbgelehnt { grund: a.grund.clone() }),
        _ => None,
    }
}
```

### 3.4  Crate-Abhängigkeiten

```toml
[dependencies]
mako-types = { path = "../mako-types" }
mako-codec = { path = "../mako-codec" }
mako-quittung = { path = "../mako-quittung" }
mako-fristen = { path = "../mako-fristen" }
mako-gpke = { path = "../mako-gpke" }
mako-wim = { path = "../mako-wim" }
# ... alle Prozess-Crates
clap = { version = "4", features = ["derive"] }
serde_json = { workspace = true }
```

---

## 4  Node/React Web-App (`mako-ui`)

### 4.1  Tech-Stack

- React + Vite + TypeScript
- Tailwind CSS + shadcn/ui
- File-System Access: `chokidar` (Node.js fs.watch wrapper) über einen kleinen Express-Server
- CLI-Aufruf: Express-Server ruft `mako-cli` per `execFile()` auf (kein Shell, kein HTTP zur Engine)
- Kein Datenbank, kein State-Management-Framework — der Dateisystem-Ordner IST der State
- Konsistenz: Express-Server serialisiert CLI-Aufrufe (kein paralleles Schreiben)

### 4.2  Architektur

```
mako-ui/
├── package.json
├── vite.config.ts
├── src/
│   ├── App.tsx                 # Hauptlayout: Tabs + 3-Spalten
│   ├── components/
│   │   ├── RollenTabs.tsx      # Tab-Leiste mit Rollen + Badge-Counts
│   │   ├── AufgabenQueue.tsx   # Offene Aktionen mit Rollen-Sprung
│   │   ├── ProzessListe.tsx    # Kommunikationslinien-Navigation
│   │   ├── MessageList.tsx     # Inbox/Outbox mit Status-Checkmarks
│   │   ├── MessageDetail.tsx   # Nachricht anzeigen (JSON + EDIFACT)
│   │   ├── MessageForm.tsx     # Formular zum Erstellen
│   │   ├── EdifactPreview.tsx  # Live EDIFACT-Vorschau
│   │   ├── ProcessTimeline.tsx # Prozessverlauf-Leiste
│   │   └── StatusBadge.tsx     # ✓✓✓ Checkmark-Komponente
│   ├── lib/
│   │   ├── markt.ts            # Ordner lesen/schreiben, Status-Management
│   │   ├── prozesse.ts         # Prozess-Definitionen (Schritte, Rollen, Nachrichten)
│   │   └── types.ts            # TypeScript-Typen (Nachricht, Rolle, Status)
│   └── server/
│       └── api.ts              # Express-Server für fs-Zugriff + CLI-Aufruf
├── public/
└── tailwind.config.ts
```

### 4.3  UI-Layout

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚡ MaKo-Simulator          [3 offene Aufgaben] [GPKE LFW 3/6]  │
├──────────────────────────────────────────────────────────────────┤
│ LFN · 990...000 │ NB · 990...001 (2) │ LFA │ MSB │ BKV │ + │   │
├──────────┬───────────────────────────┬───────────────────────────┤
│ Aufgaben │  Inbox (3) | Outbox (1)   │  Nachricht erstellen      │
│          │                           │                           │
│ → NB:    │  ┌─ UTILMD Bestätigung ─┐ │  Prozessschritt: ▾        │
│   Abmeld.│  │ Von: NB  ✓✓ ✓C ✓A   │ │  MaLo-ID: [          ]   │
│          │  └──────────────────────┘ │  Empfänger: [        ]    │
│ → LFA:   │  ┌─ CONTRL positiv ────┐ │  Lieferbeginn: [     ]    │
│   Prüfung│  │ Von: NB  ✓✓         │ │                           │
│          │  └──────────────────────┘ │  [Formular|EDIFACT|Datei] │
│──────────│                           │                           │
│ Prozesse │  ┌─ ⚠ Aktion nötig ───┐ │  [ Senden → NB ]          │
│ GPKE     │  │ NB → LFA Abmeldung  │ │                           │
│  ● LFW   │  │ [→ Als NB handeln]  │ │  ▸ EDIFACT-Vorschau       │
│  ○ L.ende│  └──────────────────────┘ │                           │
│ WiM      │                           │                           │
│  ○ MSB   │                           │                           │
├──────────┴───────────────────────────┴───────────────────────────┤
│  GPKE LFW: [1.Anmeldung ✓✓] → [2.Bestätigung ✓✓] → [3.Abm ⏳] │
└──────────────────────────────────────────────────────────────────┘
```

### 4.4  Aufgaben-Queue

Die Queue zeigt **manuell erforderliche Aktionen** über alle Prozesse hinweg. Jede Aufgabe hat:
- Was zu tun ist ("Abmeldung an LFA senden")
- Welche Rolle handeln muss (NB, LFA, ...)
- Sprung-Link ("→ Zu NB wechseln") — wechselt Tab + selektiert den Prozess

Die Queue wird aus dem Prozess-State abgeleitet: wenn ein Reducer-State auf ein Event wartet, das eine manuelle Aktion erfordert, erscheint es in der Queue.

### 4.5  Rollen-Sprung

Klickbare Elemente die den Tab wechseln:
- Absender/Empfänger-Links in Nachrichten ("Von: **Netzbetreiber**" → klick → NB-Tab)
- Aufgaben-Queue-Einträge ("→ Zu NB wechseln")
- "Aktion erforderlich"-Karten ("→ Als NB handeln")
- Tab-Badges zeigen ungelesene Nachrichten pro Rolle

---

## 5  Bedienungsanleitung

`docs/anleitung.md` — eigenständiges Dokument, auf Deutsch, für Entwickler und Ingenieure teilbar.

### 5.1  Gliederung

1. **Projektübersicht** — Was ist die MaKo-Engine, warum, für wen
2. **Architektur** — Schichtenmodell, Crate-Struktur, Dependency-Graph
3. **Typsystem** — Rollen, IDs (MaLo/MeLo/MP-ID), Nachrichten, NachrichtenPayload
4. **Reducer-Konzept** — State × Event → (State, Nachrichten), Quittungsschicht als Decorator
5. **EDIFACT-Codec** — Lexer → Parser → Dispatch, Serializer, Roundtrip
6. **Testkorpus** — Fixtures (59), Generatoren (39), Fehler-Injektor, Kommunikationsketten (15)
7. **Simulator bedienen** — CLI-Befehle, Web-UI starten, Prozess durchspielen
8. **Prozess-Referenz** — jede Kommunikationslinie mit Schritten, Rollen, Nachrichten, Fristen
9. **Glossar** — Fachbegriffe kurz erklärt

---

## 6  Implementierungsreihenfolge

### Phase A: Bedienungsanleitung

| # | Aufgabe |
|---|---------|
| A.1 | `docs/anleitung.md` schreiben (Sektionen 1–6) |
| A.2 | Simulator-Sektionen (7–8) nach Phase B/C ergänzen |

### Phase B: Rust-CLI

| # | Aufgabe | Crate |
|---|---------|-------|
| B.0 | GPKE-Reducer Nachrichten-Erzeugung vervollständigen (LFW, Lieferende, Stammdaten müssen ausgehende Nachrichten produzieren) | mako-gpke |
| B.1 | `mako-cli` Crate anlegen, `clap`-Befehle scaffolden, zum Workspace hinzufügen | mako-cli |
| B.2 | `init` Befehl: Ordnerstruktur erstellen | mako-cli |
| B.3 | Nachricht → Event Mapping (pro Prozess) | mako-cli |
| B.4 | `verarbeite` Befehl: parse → CONTRL → APERAK → Reducer → outbox | mako-cli |
| B.5 | `sende` Befehl: Datei kopieren + Status-Update | mako-cli |
| B.6 | `status` Befehl: Überblick aller Rollen/Prozesse | mako-cli |
| B.7 | `verarbeite-alle` Befehl: Batch-Verarbeitung | mako-cli |

### Phase C: Node/React Web-App

| # | Aufgabe |
|---|---------|
| C.1 | Vite + React + Tailwind + shadcn scaffolden |
| C.2 | Express-Server für fs-Zugriff + CLI-Aufruf |
| C.3 | RollenTabs + ProzessListe + AufgabenQueue |
| C.4 | MessageList + StatusBadge (Inbox/Outbox mit Checkmarks) |
| C.5 | MessageForm + EdifactPreview (Nachricht erstellen) |
| C.6 | ProcessTimeline (Prozessverlauf) |
| C.7 | Rollen-Sprung (Tab-Wechsel per Klick) |
| C.8 | File-Watcher (chokidar) für Live-Updates |

### Phase D: Integration + Anleitung vervollständigen

| # | Aufgabe |
|---|---------|
| D.1 | End-to-End Test: GPKE LFW komplett durchspielen |
| D.2 | Anleitung Sektionen 7–9 (Simulator-Bedienung, Prozess-Referenz, Glossar) |

---

## 7  Offene Todos (nicht in diesem Scope)

- **RD 2.0 Reducer: automatische ACK-Erzeugung** — `mako-rd2` Reducer sollte `RdBestaetigung` in outbox legen wenn er Stammdaten/Fahrplan/Abruf empfängt
- **Worker-Deployment** — CLI-Befehle durch HTTP-Endpunkte ersetzen, ein Worker pro Rolle
- **Robustes Produktions-UI** — Fehlerbehandlung, Validierung, Design-Polish
- **Reducer Message-Generation** — viele Reducer geben `nachrichten: vec![]` zurück, müssen ausgehende Nachrichten erzeugen

---

## 8  Guided Gates

- **GG-1:** `cargo test --workspace` — alle Tests grün nach mako-cli
- **GG-2:** `mako-cli init markt/` erstellt korrekte Ordnerstruktur für alle Rollen
- **GG-3:** `mako-cli verarbeite` parst EDIFACT, erzeugt CONTRL/APERAK, führt Reducer aus, schreibt Dateien
- **GG-4:** Web-UI zeigt Inbox/Outbox korrekt an, aktualisiert sich bei Dateiänderungen
- **GG-5:** Ein GPKE LFW Happy Path kann komplett im UI durchgespielt werden (7 Schritte: 5 EDIFACT-Nachrichten + 2 interne Events)
- **GG-6:** Nachrichten-Status (✓ → ✓✓ → ✓CONTRL → ✓APERAK → ✓✓✓) wird korrekt angezeigt
- **GG-7:** Rollen-Sprung funktioniert (Klick auf Empfänger → Tab wechselt)
- **GG-8:** Aufgaben-Queue zeigt offene manuelle Aktionen über alle Prozesse
- **GG-9:** Bedienungsanleitung ist verständlich für Entwickler ohne MaKo-Vorwissen
- **GG-10:** EDIFACT-Dateien aus dem UI sind identisch zu den Testkorpus-Fixtures (fachlich korrekt)
