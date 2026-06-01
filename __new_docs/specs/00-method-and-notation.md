# 00. Method and Notation

Status: normative.

## Design center

Musi is an expression-first language with visible consequences. If syntax changes phase, storage duration, write authority, representation visibility, existential packaging, failure, suspension, or control flow, that consequence must be visible in the source form or type.

## One obvious Musi-native way

A semantic role should not have two equally supported surface forms.

Examples:

```text
binding introduction       let
body result                final expression without semicolon
discarding                 semicolon
runtime/value conditional  when ... else
runtime/value guard        when
constraint context         where
loop exit                  exit
next loop iteration        next
suspension                 yield
```

## Normative example markers

This pack uses marker lines to make formal examples readable.

```musi
let item : Text := "hello";
         ----    -------
         type    value position
```

Markers are not part of Musi. They annotate the previous line.

## Validity labels

Examples labeled **valid** are accepted under this source specification.

Examples labeled **invalid** are rejected by a conforming parser, resolver, type checker, or constraint checker.

## Source language only

This pack specifies source syntax and source-level semantics. It does not specify bytecode, VM instruction layout, GC object headers, or FFI ABI lowering. Those are implementation/runtime specifications, not source-language grammar.
