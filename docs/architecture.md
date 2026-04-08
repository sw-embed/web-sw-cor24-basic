# Architecture: web-sw-cor24-basic

## Decision: Rust/Yew + cor24-emulator + pvm.s + pre-built basic.p24

We do **not** rebuild the BASIC interpreter in the browser. The interpreter
is Pascal compiled to p-code (`.p24`) by `../sw-cor24-basic/scripts/build-basic.sh`.
The `.p24` file is embedded in the WASM bundle. At runtime the browser loads
the p-code VM (`pvm.s`) into the COR24 emulator, then loads the `.p24` image
into the VM's code segment and feeds BASIC source as UART input.

This follows the same pattern as `../web-sw-cor24-pcode`: the COR24 assembler
runs at build time to compile `pvm.s`, the resulting binary is embedded via
`include_bytes!`, and `EmulatorCore` provides the execution substrate.

### Why not port pv24t to a WASM library?

The `pv24t` tracer contains a self-contained Rust p-code VM (`~980 lines`).
Extracting it into a reusable crate would give faster per-instruction execution
since it avoids the COR24 instruction decode overhead. However:
- The `cor24-emulator` + `pvm.s` pattern is already battle-tested in
  `web-sw-cor24-pcode`.
- It keeps the runtime identical to what runs on host hardware.
- The WASM overhead of COR24 instruction decode is acceptable for interactive
  BASIC programs.

## Pipeline

```
Build time (out of band, in sibling repo):
  src/basic.pas  --[p24p compiler]-->  build/basic.p24

Build time (this repo, via build.rs):
  ../sw-cor24-pcode/vm/pvm.s  --[COR24 asm]-->  pvm.bin
  ../sw-cor24-basic/build/basic.p24  --embed-->  basic.p24
  examples/*.bas                          --embed-->  Rust string table

Browser runtime:
  pvm.s binary  -> load at 0x000000 into EmulatorCore
  basic.p24     -> load at 0x010000, relocate data refs
  BASIC source  -> feed line-by-line as UART input (GETC / sys 2)
  EmulatorCore.run_batch(...) until p-code VM halt or budget
  UART TX bytes -> output panel (strip '>' prompts)
```

## Stack

- **Frontend**: Yew 0.21 (CSR), `wasm-bindgen`, `web-sys`, `gloo`.
- **Build tool**: Trunk.
- **CPU**: `cor24-emulator` crate (path dep on `../sw-cor24-emulator`).
- **P-code VM**: `pvm.s` assembled at build time via `cor24-emulator::Assembler`.
- **P-code loader**: `pa24r` crate (path dep on `../sw-cor24-pcode/assembler`).
- **ISA types**: `cor24-isa` crate.
- **Interpreter binary**: pre-built `basic.p24` from `../sw-cor24-basic/build/`,
  embedded with `include_bytes!` via `build.rs`.
- **Examples**: bundled `.bas` files, embedded with `include_str!`.

## Project structure

```
web-sw-cor24-basic/
├── Cargo.toml
├── build.rs                 # Assembles pvm.s, copies basic.p24 to OUT_DIR
├── Trunk.toml
├── index.html
├── CLAUDE.md
├── AGENTS.md
├── asm/
│   └── pvm.s                # P-code VM in COR24 assembly (from sw-cor24-pcode)
├── assets/
│   └── basic.p24            # Pre-compiled BASIC interpreter
├── examples/
│   ├── calc.bas
│   └── hello.bas
├── src/
│   ├── main.rs              # Yew renderer entry
│   ├── lib.rs               # App component (dropdown, Run, Reset, Clear)
│   ├── config.rs            # Pre-assembled pvm.s binary + label addresses
│   ├── demos.rs             # Bundled .bas example registry
│   ├── runner.rs            # Session: pvm.s + basic.p24 in EmulatorCore
│   └── ui.css               # Dark theme
├── pages/                   # Committed Trunk dist output for GH Pages
│   └── .nojekyll
├── docs/
└── .github/workflows/pages.yml
```

## Component design

### `App` (lib.rs)
Top-level Yew component holding state: selected example index, source text,
output text, status string, running flag, instruction budget. Messages:
`SelectDemo`, `SourceChanged`, `Run`, `Tick`, `Stop`, `Reset`, `Clear`,
`IncreaseBudget`, `KeyDown`.

### `Session` (runner.rs)
Wraps `EmulatorCore` to run BASIC on the p-code VM:
1. Load `PVM_BINARY` at address 0x000000.
2. Load `BASIC_P24` at address 0x010000 with data relocation.
3. Initialize pvm.s (run init, patch vm_state, set PC to vm_loop).
4. Queue BASIC source lines as UART input (each line terminated with `\r`).
5. Run `run_batch()` in a tick loop, feeding UART bytes each tick.
6. Check p-code VM status after each batch (halt, trap, stall).
7. Collect and clean UART output (strip `>` prompts).

### `config.rs`
Provides `PVM_BINARY` (assembled pvm.s), `PVM_LABELS`, `label_addr()`,
and `BASIC_P24` — all generated at build time.

### `demos.rs`
Static table of bundled `.bas` examples with `include_str!`. Adding an
example is just dropping a `.bas` file and adding a `Demo` entry.

## GitHub Pages deployment

Same pattern as `../web-sw-cor24-pcode` and `../web-sw-cor24-macrolisp`:

1. Developer runs `scripts/build-pages.sh` which does `trunk build --release
   --public-url /web-sw-cor24-basic/` then rsyncs `dist/` into `pages/`
   (preserving `.nojekyll`).
2. `pages/` is committed and pushed.
3. `.github/workflows/pages.yml` uploads `./pages` as the Pages artifact on
   every push to `main`.

The `--public-url` flag is critical: GitHub Pages serves static files at
`/<repo>/`, so all asset references must be prefixed. Without it, the WASM
and CSS paths would point to `/` which 404s on GitHub Pages.

This avoids needing Rust/Trunk in CI.

## p-code VM initialization

The `Session::new` flow follows `web-sw-cor24-pcode/src/debugger.rs`:

1. Load `pvm.s` binary into EmulatorCore at address 0.
2. Write `sys halt` (0x60 0x00) at `code_seg` address.
3. Run 10K COR24 instructions (pvm.s boots, enters vm_loop, hits sys halt).
4. Clear boot output (PVM banner).
5. Soft reset emulator (preserves memory).
6. Set COR24 PC to `vm_loop`, FP to `vm_state`.
7. Patch `vm_state`: code_base = 0x010000, pc = 0, status = 0.
8. Feed BASIC source as UART input.

## Stack pointer

The runner disables the emulator's stack-bounds check
(`set_stack_bounds(0, 0)`) because the p-code VM manages its own stacks
outside the strict 8 KB EBR window.
