# Implementation plan

The agentrail saga (`.agentrail/saga.toml`) is the source of truth for
ordering. This document is the human-readable overview.

## Phase 0 — Skeleton (DONE)

The project skeleton already exists: Cargo.toml, Trunk.toml, index.html,
build.rs, src/{config,demos,runner,lib,main}.rs, ui.css, examples, assets,
asm/pvm.s, .github/workflows/pages.yml, pages/.nojekyll.

## Phase 1 — Verify runner works

1. Ensure `cargo check` passes with all path deps resolving correctly.
2. Verify `trunk build --release` produces output in `dist/`.
3. Test with `trunk serve` — load the page, select "hello", click Run,
   verify UART output appears.

## Phase 2 — Fix runner integration

4. Debug and fix the p-code VM initialization flow (load pvm.s, init,
   patch vm_state, load basic.p24, feed UART input).
5. Verify "hello" example produces `HELLO WORLD` in output.
6. Verify "calc" example produces correct arithmetic output.
7. Clean output: strip `>` prompts, handle `READY` lines.

## Phase 3 — Polish UI

8. Ensure Run toggles to Stop while running, Reset reloads demo, Clear
   clears output.
9. Status line: instruction count, elapsed time, budget-exhausted affordance.
10. Ctrl/Cmd+Enter keyboard shortcut, Esc to stop.
11. Dark-theme CSS final pass — consistent with snobol4/macrolisp.

## Phase 4 — Ship

12. `trunk build --release` → copy `dist/` to `pages/`.
13. Verify `.github/workflows/pages.yml` deploys correctly.
14. Enable Pages in repo settings; verify live URL.
15. Update `README.md` with description and live link.

## Risks / open questions

- **p-code VM init correctness**: the init sequence (boot, sys halt, reset,
  patch vm_state) is borrowed from `web-sw-cor24-pcode` but may need
  adjustments for the BASIC interpreter's memory requirements.
- **UART feeding timing**: BASIC reads one character at a time via GETC.
  The batch-tick approach must drain the UART RX queue fast enough to
  avoid starvation (tight UART-wait loops).
- **Binary size**: `pvm.s` binary (~30 KB) + `basic.p24` (~8 KB) + WASM
  runtime should stay under a few MB.
- **INPUT statement**: BASIC's INPUT reads from UART during execution,
  requiring interactive UART feeding. Not supported in v1 batch mode.
