# Design: web-sw-cor24-basic UI

Batch-style "edit, run, see output" page. Simpler than the macrolisp REPL or
the p-code debugger — no split view, no trace, no memory inspector.

## Layout (single column, max-width ~960px)

```
┌──────────────────────────────────────────────────────────┐
│ web-sw-cor24-basic       [example ▼] [Run] [Reset] [Clr]│
├──────────────────────────────────────────────────────────┤
│ source (.bas)                                            │
│ ┌──────────────────────────────────────────────────────┐ │
│ │ PRINT "HELLO WORLD"                                   │ │
│ │ BYE                                                   │ │
│ └──────────────────────────────────────────────────────┘ │
│ ─ output ──────────────────── status: idle ───────────── │
│ ┌──────────────────────────────────────────────────────┐ │
│ │ HELLO WORLD                                           │ │
│ └──────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────┘
```

## Controls

- **Demo dropdown**: replaces source (and clears output) with the chosen
  bundled `.bas`. Listed in alphabetical order. Default = `hello`.
- **Run**: starts a batched run. While running, the button becomes **Stop**.
- **Reset**: reloads current demo from source, discarding edits. Disabled
  while running.
- **Clear**: clears the output panel. Does not reset the source.
- **Status line**: shows `idle | running… N instrs, T ms | done (N instrs, T ms) | halted | error: …`.

## Color / typography

Monospace everything in the source/output panels. System sans for chrome.
Dark theme by default (matches snobol4/macrolisp projects), single CSS file.

## Keyboard

- `Ctrl/Cmd+Enter` in source area: Run.
- `Esc` while running: Stop.

## Error display

Anything that fails before the emulator starts (e.g. p24 load error) is shown
in the output panel and the status line goes red. Halt for "out of budget"
shows status `halted (budget)` and surfaces an "Increase budget 4x" button
that bumps `max_instrs` 4x and re-runs.

## State machine

```
Idle ──Run──▶ Running ──tick (budget left)──▶ Running
                 │                                 │
                 │                       (vm halt │ emulator stop)
                 │                                 ▼
                 │                              Done
                 ├──Stop──▶ Stopped
                 └──error──▶ Error
```

## UART input model

BASIC is a REPL that reads lines via `sys 2` (GETC). The runner feeds the
entire BASIC program as UART input before starting execution:

1. Each line of source is queued as bytes + `\r`.
2. A final `\n` terminates input.
3. During each tick, queued bytes are drained into the UART while the RX
   buffer has space.
4. Output is collected from UART TX, with `>` prompts stripped.

## Output cleaning

The BASIC interpreter prints a `>` prompt before each line of input. These
are stripped from the displayed output, matching the behavior of
`run-basic.sh` (which pipes through `tr -d '>'`).

## Out of scope for v1

- Syntax highlighting (plain `<textarea>`).
- Interactive REPL with live input during execution.
- Saving snippets to localStorage.
- Sharing via URL hash.
- Trace / register / memory inspectors.
- INPUT statement support (would require live UART interaction during run).
