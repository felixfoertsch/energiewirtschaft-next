---
name: Use the real engine, don't build parallel systems
description: When simulating or testing, always use the actual system components — never build separate mock implementations
type: feedback
---

When building simulations, tests, or demos: use the real system. Don't build a parallel toy implementation.

**Why:** User corrected me when I initially proposed a standalone simulation. The value is in exercising the actual mako-codec, mako-quittung, mako-sim, mako-testdata generators — not in a separate mock.

**How to apply:** The simulation binary (`mako-sim/src/bin/simulate.rs`) orchestrates scenarios but every EDIFACT message flows through the real parse/validate/serialize/route pipeline. New features should extend the real crates, not create workarounds.
