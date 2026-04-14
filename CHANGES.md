# Changelog

## 2026-04-14

### Demos

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
