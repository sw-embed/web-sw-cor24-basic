# BASIC Demos

This page documents the embedded demo programs included in the web UI.

## hello

A minimal "Hello World" program.

```basic
PRINT "HELLO WORLD"
BYE
```

Demonstrates the `PRINT` statement and the `BYE` command to exit the interpreter.

## factorial

Iterative factorial computation using `FOR`/`NEXT` loops.

```basic
10 REM ITERATIVE FACTORIAL OF N
20 LET N=7
30 LET F=1
40 FOR I=1 TO N
50 LET F=F*I
60 NEXT
70 PRINT N;"!=";F
80 END
```

Demonstrates:
- Numbered lines and `REM` comments
- `FOR`/`NEXT` loops
- Variable assignment with `LET`
- `END` statement to terminate

## fibonacci

Prints the first 10 Fibonacci numbers using iteration.

```basic
10 REM FIRST 10 FIBONACCI NUMBERS
20 LET A=0
30 LET B=1
40 FOR I=1 TO 10
50 PRINT A
60 LET C=A+B
70 LET A=B
80 LET B=C
90 NEXT
99 END
```

Demonstrates:
- `FOR`/`NEXT` loops
- Sequential variable swaps
- Numbered line program structure

## fizzbuzz

Classic FizzBuzz for numbers 1 through 15 using `GOTO` and divisibility checks.

```basic
10 REM FIZZBUZZ 1..15
20 FOR I=1 TO 15
30 IF (I/15)*15=I THEN GOTO 100
40 IF (I/3)*3=I THEN GOTO 200
50 IF (I/5)*5=I THEN GOTO 300
60 PRINT I
70 GOTO 90
90 NEXT
99 END
100 PRINT "FIZZBUZZ"
110 GOTO 90
200 PRINT "FIZZ"
210 GOTO 90
300 PRINT "BUZZ"
310 GOTO 90
```

Demonstrates:
- `IF`/`THEN` conditional branching
- `GOTO` for unconditional jumps
- Integer divisibility testing via `(I/N)*N=I`
- Multi-way branching pattern

## calc

Arithmetic expressions with variables and built-in functions.

```basic
PRINT "COR24 BASIC V1"
PRINT "2+3=";2+3
PRINT "7*6=";7*6
PRINT "100/7=";100/7
LET A=10
LET B=20
PRINT "A=";A
PRINT "B=";B
PRINT "A+B=";A+B
PRINT "(A+B)*2=";(A+B)*2
PRINT "ABS(-42)=";ABS(-42)
BYE
```

Demonstrates:
- Arithmetic operators: `+`, `-`, `*`, `/`
- Variable assignment with `LET`
- String concatenation with `;` in `PRINT`
- Built-in functions: `ABS()`

## count

Trivial counter from 1 to 10 — the simplest stored-mode program.

```basic
10 REM COUNT 1 TO 10
20 FOR I=1 TO 10
30 PRINT I
40 NEXT
50 END
```

Demonstrates the bare minimum `FOR`/`NEXT` loop with a numbered-line program.

## memdump

Pokes three bytes (`H`, `I`, `!`) into low memory and reads them back with `PEEK`.

```basic
10 REM POKE A SHORT MESSAGE INTO LOW MEMORY AND PEEK IT BACK
20 POKE 100,72
30 POKE 101,73
40 POKE 102,33
50 FOR I=100 TO 102
60 PRINT I;"=";PEEK(I)
70 NEXT
80 END
```

Demonstrates `POKE` and `PEEK` against the p-code VM's data memory.

## robot-chase

Turn-based robot-chase on a 16x16 grid — make the robots collide with each
other or walk into wreckage while you evade them. Inspired by the classic
BSD `robots` game.

Commands are numpad-style digits: `1`–`9` for the eight compass moves plus
`5=WAIT`, `0=TELEPORT` (three per game), `10=LRS` for a 4x4 regional
summary, and `99=RESIGN`.

Demonstrates:
- Using `POKE`/`PEEK` as a 2D array (256-cell board) plus three parallel
  arrays for robot X/Y/alive state (12 robots)
- A home-rolled linear-congruential PRNG seeded from the persistent
  variable `R` — the seed carries over across `RUN` invocations inside
  the same REPL so repeated games see different boards
- Multi-phase turn logic: player move → robot step → wreck collision →
  robot-robot collision → repaint

Like the other games, this demo is **interactive**. Type `99` at the
command prompt to resign.

## startrek

Classic Star Trek-style game — defend the Federation across an 8x8 galaxy.
Uses `INPUT`, `GOSUB`/`RETURN`, `PEEK`/`POKE` arrays in memory, and a
home-rolled PRNG.

This demo is **interactive**: it uses `INPUT` to read commands. When the
running program asks for input, an input field appears below the output;
type a command, press Enter (or Send), and execution resumes. The program
returns to the BASIC REPL when it ends — type `BYE` in the input field to
fully halt.

## trek-adventure

Numeric-menu text adventure — "Star Trek: Decaying Orbit". You awaken
alone on the bridge of the Enterprise with engines offline and orbit
decaying. Explore nine rooms, collect items, repair the warp relay, and
deal with Klingon boarders before the turn counter runs out.

All commands are numeric: top-level actions (`1=LOOK`, `3=GO`, `4=TAKE`,
`5=USE`, `6=THROW`, `7=FIRE`, …) and their targets (room numbers, item
numbers) are prompted one `INPUT` at a time — no string parsing required
from the interpreter.

Demonstrates:
- Multi-stage `INPUT` flow with a command/target sub-prompt pattern
- `IF`/`AND` compound conditions for room-and-item dispatch
- Deep `GOSUB`/`RETURN` nesting for rooms, inventory, events, and endings
- Integer state machine (room `R`, turns `T`, per-item flags) as the
  entire world model

Like `startrek`, this demo is **interactive**. Type `0` (QUIT) at the
command prompt to resign, or let the turn counter reach zero.
