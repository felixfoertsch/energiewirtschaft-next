---
name: MaKo-Engine project context
description: Functional model of German energy market communication (Strom + Gas) in Rust — architecture decisions, current phase, key constraints
type: project
---

MaKo-Engine: beweisbar korrekte Referenzimplementierung der deutschen Marktkommunikation in Rust.

**Why:** Referenzimplementierung + produktives System. Pure functions → deployment-agnostisch (Cloudflare Edge, FaaS, WASM). Gesamte MaKo-Historie abbilden (v2017, v2020, v2022, v2025, zukünftige).

**How to apply:** Jede Kommunikationslinie ist ein `(State, Event) → Result<(State, Vec<Nachricht>), ProzessFehler>` Reducer. Quittungsschicht (CONTRL/APERAK) als generischer Decorator. Eigenes Rust-Typsystem, keine Dependencies auf BO4E/Python-Tooling. Hochfrequenz-Ökosystem als Wissensbasis.

**Current state (2026-03-24):**
- Spec: `docs/superpowers/specs/2026-03-24-mako-engine-design.md` (approved, 11 phases)
- Plan: `docs/superpowers/plans/2026-03-24-mako-foundation.md` (approved, 9 tasks for phases 1-4)
- Implementation: not started yet
- Next: execute foundation plan (Tasks 1-9)
