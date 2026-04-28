# AGENTS.md

This project uses **agentrail** for session-based saga/step orchestration.

**Quick start:** run `agentrail next`, then `agentrail begin`, do the work
in the step prompt, commit, then `agentrail complete`. Push the branch.

**For the full set of rules, read [CLAUDE.md](CLAUDE.md).** It contains
the agentrail session protocol, `.agentrail/` discipline (commit metadata
before `agentrail complete`), recovery (`audit`, `snapshot`), git hygiene,
and project-specific architecture / build / deploy notes. This file
exists so non-Claude agents (codex, opencode, GLM-5, etc.) know which
file to read.

When the briefing system rolls out (`agentrail instructions apply`), a
generated block will be stamped in this file between markers. Local
content here should remain minimal — keep all repo-specific rules in
CLAUDE.md so there is exactly one source of truth.
