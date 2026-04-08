# PRD: web-sw-cor24-basic

## Goal

A zero-install, browser-based way to **write and run BASIC programs** on the
COR24 p-code virtual machine. Users land on a GitHub Pages URL and within
seconds can select a `.bas` example, hit Run, and see output.

## Audience

- Retro-computing enthusiasts curious about 1970s-style BASIC.
- Students or instructors wanting a no-setup BASIC sandbox.
- Anyone evaluating the COR24 / p-code / BASIC toolchain stack.

## Non-goals (initial release)

- Interactive REPL mode (BASIC's `>` prompt with live input) — v1 feeds the
  entire program as UART input and runs to completion.
- Loading external `.bas` files via filesystem.
- Live execution trace / register / memory viewers.
- Multi-user state, persistence, or accounts.

## User stories

1. *As a visitor* I see a BASIC source area pre-filled with `hello.bas`, a
   **demo dropdown**, a **Run** button, a **Reset** button, a **Clear**
   button, and an output area.
2. *As a visitor* I can pick from a **demo dropdown** whose entries match the
   examples in `../sw-cor24-basic/examples/`, listed in alphabetical order
   (`calc`, `hello`, ...). Selecting an entry replaces the source area with
   that bundled `.bas`.
3. *As a visitor* I click **Run** and within a few seconds see the BASIC
   program's UART output. Long-running programs surface a status indicator
   and can be stopped.
4. *As a visitor* I see instruction count and elapsed time after each run.
5. *As a developer* I can `trunk serve` locally and iterate on the UI.

## Success criteria for v1

- Live demo published at `https://<user>.github.io/web-sw-cor24-basic/`.
- All bundled examples run to completion in the browser and produce the same
  UART output as `./scripts/run-basic.sh` on the host.
- Cold-load to first successful Run on a typical laptop is under 5 seconds.
- Repository builds with `trunk build --release` from a clean checkout, given
  sibling repos `sw-cor24-basic` and `sw-cor24-emulator`.

## Key difference from sibling web-* projects

The BASIC interpreter is **p-code** (`.p24`), not native COR24 machine code.
It requires the p-code VM (`pvm.s`) running inside the COR24 emulator as an
intermediate layer. This adds one level of interpretation compared to
`web-sw-cor24-snobol4` (which loads native COR24 `.bin` directly).
