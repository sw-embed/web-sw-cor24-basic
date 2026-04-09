# Changelog

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
