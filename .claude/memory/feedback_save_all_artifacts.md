---
name: Save all simulation artifacts to disk
description: When running simulations, persist every message/artifact — don't discard intermediate data
type: feedback
---

When running simulations: save everything. Every EDIFACT message, every log entry, every intermediate result must be on disk.

**Why:** User caught that the initial simulation discarded the raw EDIFACT strings after validation. The messages ARE the simulation — they need to be inspectable, replayable, and visible in the report.

**How to apply:** Simulation output goes to `mako-sim/simulation/` (gitignored). Every EDIFACT message saved as `.edi` file under `nachrichten/<kette>/<id>.edi`. Reports should embed raw message content where practical and link to files on disk otherwise.
