# AGENTS.md — AgentRail instructions for web-sw-cor24-basic

This file tells AI agents how to use AgentRail with this project.

## What is AgentRail?

AgentRail is a session-based workflow manager that coordinates multi-step
development across sessions. Each session advances one step in a saga (plan).

## Session Protocol

Every session MUST follow this exact sequence:

### 1. Get your assignment
```bash
agentrail next
```
Read the output. It tells you: current step number, step prompt, plan context,
relevant skill docs, and past trajectories from previous sessions.

### 2. Begin the session
```bash
agentrail begin
```
This records the session start and locks the step.

### 3. Execute the step prompt
The step prompt IS your instruction. Do NOT ask the user "want me to proceed?"
or "shall I start?". Execute the step directly.

### 4. Commit your work
After making code changes, commit with git. Every modified repo gets a commit.

### 5. Record completion
```bash
agentrail complete --summary "what you accomplished" \
  --reward 1 \
  --actions "tools and approach used"
```
- Success: `--reward 1`
- Failure: `--reward -1 --failure-mode "what went wrong"`
- Saga finished: add `--done`

### 6. STOP
Do NOT make further code changes after `agentrail complete`.
Untracked changes are invisible to the next session.

## Other commands
```bash
agentrail status          # Current saga state
agentrail history         # All completed steps
agentrail plan            # View the plan
agentrail next            # Current step + context
```

## Project context

This is a Rust/Yew/WASM browser UI that runs a BASIC interpreter (compiled
to p-code `.p24`) on the COR24 p-code VM (`pvm.s`) inside the COR24 hardware
emulator. See `CLAUDE.md` for full architecture details.
