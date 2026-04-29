# Changelog

## 2026-04-29 (later)

### UI

- Add a focus chain: on page load the demo `<select>` is focused;
  changing the demo focuses the **Run** button; clicking Run focuses
  the input field. Each transition uses a `Timeout(0)` deferred
  `.focus()` call so it runs after Yew commits the DOM update,
  fixing a flake where the input field wouldn't reliably take focus
  on Run-click and the user had to click the field manually before
  pressing Enter.
- Add `Session::echo_input(&str)` plus a wire-up in `Msg::InputSubmit`
  that appends the user's typed line to the output panel without a
  trailing newline. The COR24 VM has no TTY-style echo, so the
  browser host now provides it. With the demo formatted using
  `" - TRY HIGHER!"`-style responses, output reads as
  `YOUR GUESS ? 50 - TRY HIGHER!` on a single line per guess.

### Demos

- Rewrite `guess-random.bas` flow:
  - Print `RANDOMIZING...` *before* the seed loop (was: `--- GUESS
    THE NUMBER ---` printed first, which was misleading — the game
    proper hadn't started yet).
  - Print `--- GUESS THE NUMBER ---` and `I AM THINKING...` *after*
    the user presses Enter, so they appear once seeding is done.
  - INPUT prompt now reads `YOUR GUESS ? ` (the interpreter does
    not auto-print `? `, so the demo includes it in the literal).
  - Response strings prefixed with `" - "` so they sit on the same
    line as the echoed guess: `YOUR GUESS ? 50 - TRY HIGHER!`.

## 2026-04-29

### Runtime

- Implement syscall **id=9 (INKEY)** in the browser p-code runner:
  consume one byte from stdin if available, else return -1
  immediately. Never blocks. Mirrors the upstream `sys_inkey`
  shipped in `sw-cor24-pcode/vm/pvm.s` (id=9, dabe6f3) so the
  matching BASIC `INKEY` builtin (`sw-cor24-basic` 39c3271) actually
  works in this sandbox.
- Sync `assets/basic.p24` to the upstream rebuild that adds `INKEY`
  as keyword #37 (17956 → 18055 bytes). The upstream commit
  reshuffled the Pascal token layout (operator base `TP` 164 → 180,
  reserving room for up to 48 keywords), so this is a *required*
  pairing — old and new `.p24` images aren't interchangeable.
- Fix `runner.rs::new_with_mode` to skip its appended `RUN\n` when
  the source already ends with one. Every demo file conventionally
  ends with `RUN` so a human can paste the listing into a REPL and
  auto-run; the runner was double-stamping, leaving stray `RUN\n`
  bytes in stdin that INKEY would then consume as fake "user input"
  (and that earlier caused intermittent `?REDO` lines on `INPUT`).

### UI

- Add a **Help** button (rightmost in the chrome controls) that opens a
  modal with two tabs: a **User Guide** (getting-started, control flow,
  arrays, DATA/READ, an explicit "no built-in `RND` — roll your own
  LCG" section) and a **Reference** (statements, operator precedence,
  built-in functions, limits, runtime error codes). Closes via the
  ×, the overlay, or `Esc`.
- Reflow `.chrome` right padding 96 → 120px so the new Help button
  still clears the 80x80 GitHub-corner octocat plus its diagonal
  triangle at narrower viewport widths.
- Show the input row whenever an interactive demo is running, not
  just when the program is blocked at `INPUT`. Required for the
  INKEY-loop pattern, where the program polls non-blocking and
  needs the user to be able to deposit a byte mid-run. The same
  field continues to feed bytes into `INPUT` when the program asks
  for a line. Auto-focus on the field at run start.

### Demos

- Add `guess-random.bas` — interactive variant of `guess` that
  derives the target from **real reaction-time entropy** via
  `INKEY`. After draining any stale stdin bytes, R increments in a
  tight `IF INKEY < 0` loop until the user presses Enter; R is then
  passed through two LCG steps and reduced to 1..100. The target
  varies across runs because R depends on the exact instant the
  user hits Enter. No host clock is involved; the COR24 VM has none.

## 2026-04-25

### Demos

- Add six new feature demos vendored from `../sw-cor24-basic`, all
  registered alphabetically in `src/demos.rs`:
  - `bitwise-demo` — `BAND`/`BOR`/`BXOR`/`SHL`/`SHR` plus tagged-int
    helpers and byte packing.
  - `cont-demo` — `STOP` mid-program, then REPL-level `CONT` to resume.
  - `data-demo` — `DATA`/`READ`/`RESTORE`, including `RESTORE <line>`
    and negative literals.
  - `dim-demo` — `DIM` integer arrays, scalar/array namespace
    separation, expression subscripts.
  - `mod-demo` — FizzBuzz with the new `MOD` operator instead of the
    `(I/N)*N=I` workaround.
  - `on-demo` — `ON expr GOSUB` for O(1) bytecode-style dispatch.
- Re-sync `assets/basic.p24` (13508 → 17956 bytes) to pick up the
  upstream interpreter rebuild that adds the keywords used by these
  demos: `DIM`, `DATA`, `READ`, `RESTORE`, `ON`, `MOD`, `BAND`, `BOR`,
  `BXOR`, `SHL`, `SHR`, `CONT`.

### UI

- Add 96px right padding to the `.chrome` header row so the Run / Reset
  / Clear buttons no longer sit underneath the 80x80 GitHub-corner
  octocat at narrower viewport widths (Clear was unclickable on common
  laptop widths).

## 2026-04-24

### Demos

- Add `guess` — interactive guess-the-number demo vendored from
  `../sw-cor24-basic`. Exercises `INPUT`, `IF`/`THEN`/`GOTO`, and clean
  `BYE` exit.
- Skip upstream `blink.bas`: it `POKE`s a hardware LED address
  (`0xFF0000`) that traps on the host-side p-code interpreter the web
  sandbox runs on (the comment in the file itself notes this).

## 2026-04-15

### Demos

- Re-sync `examples/robot-chase.bas` from `../sw-cor24-basic` — upstream
  expanded the board from 12x12 to 16x16 (12 robots, 3 teleports, 4x4
  LRS regions). Also picks up the `PRINT ""`-vs-bare-`PRINT` hardening
  so rows render correctly on older interpreter builds.
- Re-sync `assets/basic.p24` from `../sw-cor24-basic` — picks up the
  PRINT double-newline fix required by the updated `robot-chase`.

## 2026-04-14

### Demos

- Add `robot-chase` — interactive 12x12 turn-based robot chase driven
  by `POKE`/`PEEK` as a 2D board, numpad-style move commands, teleport,
  LRS, and robot-vs-robot collision logic. Registered alphabetically in
  `src/demos.rs` between `memdump` and `startrek`.
- Add `trek-adventure` — interactive numeric-menu text adventure "Star
  Trek: Decaying Orbit". Exercises the BASIC interpreter's `IF ... AND
  ... THEN GOTO` compound-conditional path and multi-stage `INPUT`
  dispatch. Registered alphabetically in `src/demos.rs` after `startrek`.
- Re-sync `assets/basic.p24` and `examples/` from `../sw-cor24-basic` to
  pick up the latest interpreter (including fixes required by
  `trek-adventure`).
- Sync examples and `assets/basic.p24` from `../sw-cor24-basic` — picks up
  the STARTREK end-to-end fixes, CHR$ support in PRINT items, and clean
  REPL exit on Ctrl-D / EOF.

### Runner

- Remove the hidden 20M-instruction cap inside `Session::tick()` that
  silently stalled long-running sessions once their instruction count
  passed the threshold (no output, no halt, no status change).
- Interactive demos bypass the UI-level `max_instrs` budget so turn-based
  games (`startrek`) can run indefinitely without surprise `halted
  (budget)` errors.

### UI

- Two-column workspace: source panel on the left, output + input on the
  right. Each panel scrolls internally inside a viewport-bounded frame,
  instead of the whole page growing unboundedly as output arrives.
- Output panel auto-scrolls to the bottom after each render.
- Input field re-focuses after every iteration so typing in an
  interactive program (e.g. `startrek`) is uninterrupted.
- Collapses to a single column under 900px wide.

### Docs

- Rewrite `README.md` with explicit Intro / Overview / Build / Usage /
  Demos sections and a fresh interactive `startrek` screenshot
  (`images/screenshot-startrek.png`).

## 2026-04-09

### Demos

- Port `count`, `memdump`, `startrek` from upstream `sw-cor24-basic`
- Document new demos in `docs/demos.md`

### Runner

- Add interactive mode: GETC syscall pauses execution when its stdin buffer
  is empty (interactive mode only) by rewinding PC past the syscall, allowing
  the host to call `feed_input()` and resume.
- `Demo` gains an `interactive` flag; `startrek` is marked interactive.

### UI

- Show an input field beneath the output when the running session is
  awaiting user input; pressing Enter or clicking Send forwards the line
  to the BASIC interpreter and resumes execution.

### Fix

- Restore favicon by adding `data-trunk rel="copy-file"` directive in `index.html`

## 2026-04-08

### UI

- Add GitHub Octocat corner link (top-right triangle)
- Add footer with license, copyright, and links (COR24-TB, Blog, Discord, YouTube, demo docs, changes)

### Documentation

- Add `docs/demos.md` with demo program documentation
- Add `CHANGES.md`

### Build

- Update `build-pages.sh` to capture build host, ISO timestamp, and short commit SHA
