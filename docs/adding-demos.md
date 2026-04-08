# Adding BASIC Demos

## Files to update

1. Copy the `.bas` file to `examples/`
2. Register it in `src/demos.rs` (alphabetically in the `DEMOS` array)
3. Document it in `docs/demos.md`

## Critical: numbered vs immediate-mode programs

The COR24 BASIC interpreter has two execution modes:

- **Immediate mode** (no line numbers): Lines are executed as they are read.
  Examples: `hello.bas`, `calc.bas`.
- **Stored mode** (numbered lines): Lines are stored in memory, then executed
  only when a `RUN` command is issued. `END` and `STOP` return to immediate
  mode — they do **not** halt the interpreter. Only `BYE` exits cleanly.

The web runner feeds source as UART input followed by an EOT byte (`0x04`).
For stored-mode programs, the runner **must** detect numbered lines and
automatically append `RUN\nBYE\n` before the EOT marker. This logic lives in
`src/runner.rs` `Session::new()`.

**When adding a demo:**
- If the `.bas` file uses line numbers, verify the runner auto-appends
  `RUN\nBYE\n`.
- If the file uses `END`/`STOP` instead of `BYE`, ensure the runner handles
  it — `END` alone will cause the interpreter to block on input forever.
- Always test: the program must produce output and stop (status != 0 in the
  p-code VM).

## Build and deploy

```bash
bash scripts/build-pages.sh
git add examples/ src/ src/ docs/ pages/
git commit -m "add <name> demo"
git push
```

Do **not** run `trunk` directly — always use `scripts/build-pages.sh`.
