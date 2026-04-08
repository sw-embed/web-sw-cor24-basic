# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`web-sw-cor24-basic` is a browser-based UI (Rust / Yew / WASM) that runs the
BASIC interpreter from `../sw-cor24-basic` on an in-browser COR24 p-code VM.

The BASIC interpreter is compiled Pascal → p-code (`.p24`), so it runs on the
p-code VM (not directly on the COR24 hardware emulator). This project follows
the same architecture as `../web-sw-cor24-pcode`: the p-code VM (`pvm.s`) is
assembled at build time, embedded in the WASM bundle, and loaded into
`EmulatorCore` at runtime alongside the BASIC interpreter `.p24` image.

A live demo is published to GitHub Pages from the committed `pages/` directory.

## CRITICAL: AgentRail Session Protocol (MUST follow exactly)

This project uses AgentRail. Every session follows this exact sequence:

### 1. START (do this FIRST, before anything else)
```bash
agentrail next
```
Read the output carefully. It tells you your current step, prompt, skill docs, and past trajectories.

### 2. BEGIN (immediately after reading the next output)
```bash
agentrail begin
```

### 3. WORK (do what the step prompt says)
Do NOT ask the user "want me to proceed?" or "shall I start?". The step prompt IS your instruction. Execute it.

### 4. COMMIT (after the work is done)
Commit your code changes with git.

### 5. COMPLETE (LAST thing, after committing)
```bash
agentrail complete --summary "what you accomplished" \
  --reward 1 \
  --actions "tools and approach used"
```
If the step failed: `--reward -1 --failure-mode "what went wrong"`
If the saga is finished: add `--done`

### 6. STOP (after complete, DO NOT continue working)
Do NOT make any further code changes after running agentrail complete.
Any changes after complete are untracked and invisible to the next session.
If you see more work to do, it belongs in the NEXT step, not this session.

Do NOT skip any of these steps. The next session depends on your trajectory recording.

## Architecture

```
Build time (out of band, in sibling repo):
  src/basic.pas  --[p24p compiler]-->  build/basic.p24

Build time (this repo, via build.rs):
  ../sw-cor24-pcode/vm/pvm.s  --[COR24 assembler]-->  asm/pvm.bin
  ../sw-cor24-basic/build/basic.p24  --copy-->  assets/basic.p24
  examples/*.bas                          --embed--> Rust string table

Browser runtime:
  pvm.s binary  -> load at 0x000000 into EmulatorCore
  basic.p24     -> load at 0x010000, relocate data refs
  BASIC source  -> feed as UART input (GETC)
  EmulatorCore.run_batch(...) until halt or budget
  UART TX bytes -> output panel
```

## Key difference from web-sw-cor24-snobol4

The SNOBOL4 interpreter is native COR24 machine code (`.bin`), loaded directly
into the emulator. The BASIC interpreter is p-code (`.p24`) — it requires the
p-code VM (`pvm.s`) to be running in the emulator first, then the `.p24` image
is loaded into the VM's code segment.

## Cross-Repo Context

All COR24 repos live under `~/github/sw-embed/` as siblings.

- `sw-cor24-emulator` — COR24 hardware emulator (EmulatorCore, Assembler)
- `sw-cor24-emulator/isa` — ISA types (cor24-isa)
- `sw-cor24-pcode/vm/pvm.s` — p-code VM in COR24 assembly
- `sw-cor24-pcode/assembler` — pa24r crate (load_p24, assemble)
- `sw-cor24-pcode/tracer` — pv24t (host-side p-code interpreter, reference)
- `sw-cor24-basic` — BASIC interpreter source + build scripts
- `web-sw-cor24-pcode` — browser p-code debugger (same pvm.s pattern)
- `web-sw-cor24-snobol4` — browser SNOBOL4 sandbox (GH Pages pattern)

## Build

```bash
# Ensure basic.p24 is built in the sibling repo
cd ../sw-cor24-basic && ./scripts/build-basic.sh

# Copy the interpreter binary
cp ../sw-cor24-basic/build/basic.p24 assets/

# Build and serve locally
trunk serve --release
```

## GitHub Pages Deployment

1. `trunk build --release` locally
2. Copy `dist/` contents into `pages/`
3. Commit `pages/` (including `.nojekyll`)
4. `.github/workflows/pages.yml` deploys on push to main
