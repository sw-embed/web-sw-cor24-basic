# BASIC Demos

This page documents the embedded demo programs included in the web UI.

## hello

A minimal "Hello World" program.

```basic
PRINT "HELLO WORLD"
BYE
```

Demonstrates the `PRINT` statement and the `BYE` command to exit the interpreter.

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
