# Changelog

## 2026-04-09

### Demos

- Port `count`, `memdump`, `startrek` from upstream `sw-cor24-basic`
- Document new demos in `docs/demos.md` (startrek noted as non-interactive in current runner)

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
