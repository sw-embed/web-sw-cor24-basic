# web-sw-cor24-basic

Browser-based COR24 BASIC v1 sandbox. Write and run 1970s-style
line-numbered BASIC programs on the COR24 p-code virtual machine, directly
in your browser. Zero install.

**Live demo**: https://sw-embed.github.io/web-sw-cor24-basic/

Part of the [Software Wrighter COR24 Tools Project](https://sw-embed.github.io/web-sw-cor24-demos/#/).

![startrek screenshot](images/screenshot-startrek.png?ts=1776194114492)

## Intro

`web-sw-cor24-basic` is a zero-install browser sandbox for the
[COR24 BASIC v1](https://github.com/sw-embed/sw-cor24-basic) interpreter.
Pick a bundled demo from the dropdown — or paste your own BASIC program into
the editor — and run it in a p-code VM compiled to WebAssembly. The UI
supports `INPUT`-driven interactive programs, so REPL-style and turn-based
games like `startrek` work out of the box.

## Overview

The BASIC interpreter is Pascal compiled to COR24 p-code (`.p24`). In this
project the p-code runs on an in-browser p-code interpreter
([`pv24t`](../sw-cor24-pcode/tracer) ported to Rust/WASM — see
[`src/runner.rs`](src/runner.rs)) rather than on the COR24 hardware emulator,
so the only thing shipped to the browser is the `.p24` image and a small
loader.

```
Build time (sibling repo):
  sw-cor24-basic/src/basic.pas  --[p24p compiler]-->  build/basic.p24

Build time (this repo):
  ../sw-cor24-basic/build/basic.p24  --copy-->  assets/basic.p24
  examples/*.bas                         --embed--> Rust string table

Browser runtime:
  basic.p24     -> loaded by pa24r into a Rust p-code interpreter
  BASIC source  -> fed as stdin; auto-appended `RUN` for line-numbered progs
  INPUT         -> pauses execution; host resumes when user submits a line
  stdout bytes  -> streamed into the output panel (auto-scrolls)
```

- **Frontend**: Yew 0.21 (CSR), WASM
- **P-code loader**: [`pa24r`](../sw-cor24-pcode/assembler)
- **P-code interpreter**: in-tree port of [`pv24t`](../sw-cor24-pcode/tracer)
- **BASIC interpreter**: [`sw-cor24-basic`](../sw-cor24-basic) (Pascal -> p-code)

## Build

```bash
# 1. Build the BASIC interpreter p-code in the sibling repo
cd ../sw-cor24-basic && ./scripts/build-basic.sh

# 2. Copy the interpreter binary into this project
cp ../sw-cor24-basic/build/basic.p24 assets/

# 3. Dev server (http://localhost:9072/)
./scripts/serve.sh

# 4. Build the release bundle for GitHub Pages
./scripts/build-pages.sh
```

`build-pages.sh` holds an exclusive `dist/` lock (same lock as `serve.sh`) so
a running dev server cannot race the wasm-bindgen pipeline and leave empty
artifacts that manifest as SRI failures in the browser.

## Usage

1. Open the app (locally or the live demo link above).
2. Pick a demo from the **demo dropdown** (or paste your own BASIC into the
   source panel on the left).
3. Click **Run**. Output streams into the right-hand panel and auto-scrolls
   to the bottom.
4. If the program uses `INPUT`, an input row appears below the output —
   type your line and press Enter (or click Send). For long-running
   interactive programs (like `startrek`) there is no instruction budget:
   play as long as you like.
5. **Reset** reloads the current demo's source. **Clear** wipes the output
   panel. **Stop** halts a running program.

Keyboard shortcuts:

- **Ctrl/Cmd+Enter** — Run from the source panel
- **Esc** — Stop the running program
- **Enter** (in the input row) — submit an input line

## Demos

Bundled in [`examples/`](examples/) and registered in
[`src/demos.rs`](src/demos.rs). Full write-ups in
[`docs/demos.md`](docs/demos.md).

| Name        | Interactive | Summary |
|-------------|:-----------:|---------|
| `calc`      |             | Arithmetic, variables, `ABS`. |
| `count`     |             | Minimal `FOR`/`NEXT` loop, 1..10. |
| `factorial` |             | Iterative factorial with `FOR`/`NEXT`. |
| `fibonacci` |             | First 10 Fibonacci numbers. |
| `fizzbuzz`  |             | `IF`/`GOTO` branching, 1..15. |
| `hello`     |             | "HELLO WORLD" one-liner. |
| `memdump`   |             | `POKE` a short message, read it back with `PEEK`. |
| `startrek`  |      ✓      | Classic Star Trek game across an 8x8 galaxy. Uses `INPUT`, `GOSUB`/`RETURN`, `PEEK`/`POKE` as arrays, and a home-rolled PRNG. |

To add a new demo, see [`docs/adding-demos.md`](docs/adding-demos.md).

## Sibling repos

| Repo | Role |
|------|------|
| [sw-cor24-basic](https://github.com/sw-embed/sw-cor24-basic) | BASIC interpreter source |
| [sw-cor24-emulator](https://github.com/sw-embed/sw-cor24-emulator) | COR24 hardware emulator |
| [sw-cor24-pcode](https://github.com/sw-embed/sw-cor24-pcode) | P-code VM, assembler, linker |
| [web-sw-cor24-pcode](https://github.com/sw-embed/web-sw-cor24-pcode) | Browser p-code debugger (same pattern) |
| [web-sw-cor24-snobol4](https://github.com/sw-embed/web-sw-cor24-snobol4) | Browser SNOBOL4 sandbox (GH Pages pattern) |

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A. Wright

## License

MIT -- see [LICENSE](../sw-cor24-basic/LICENSE) for details.
