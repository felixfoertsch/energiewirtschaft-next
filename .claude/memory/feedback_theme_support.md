---
name: Always support light and dark mode
description: Every web page/report must have both light and dark mode with system preference default and toggle
type: feedback
---

Every web page must support light mode AND dark mode. Default to system preference, with a manual toggle persisted to localStorage.

**Why:** User explicitly requested this and had it added to AGENTS-web-stack.md as a permanent rule.

**How to apply:** For Tailwind projects, use the `class` strategy with `dark:` variant. For standalone HTML (like simulation reports), use CSS custom properties with `[data-theme="dark"]` selectors. Always add a toggle button. Never ship a dark-only or light-only page.
