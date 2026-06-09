# SEIL Text Format

SEIL (`.seil`) is hand-writable executable IL for SEAM. Text, not Musi source, not binary instruction file. Musi compilers emit `.seil`; developers may author `.seil`; SEAM tooling assembles/loads `.seil` into internal binary image before execution.

SEIL text combines WAT/Lisp-like declarations with Forth/RPN-like bodies:

- module + metadata structure use parenthesized forms;
- procedure bodies use line-oriented stack instructions;
- source/program symbols preserved exactly;
- symbolic operands assemble to binary table indices;
- canonical disassembly emits symbols when stable names exist.

Project evidence: `grammar/seil.ebnf`, `LOCKED_LANGUAGE_DESIGN.md` sections 16-18.

## File Structure

```ebnf
seil-text ::= form-ws? module form-ws?
module    ::= "(" "module" ws module-name module-decl* form-ws? ")"
```

One `.seil` file contains exactly one `module`.

Example:

```text
(module math
  (asm math
    (ver 1 0 0 0)
    (runtime seam)
    (entry area))

  (sig area
    (in width f64)
    (in height f64)
    (out f64))

  (proc area area
    entry:
      ld.arg width
      ld.arg height
      mul
      ret
  ))
```

## Module Declarations

| Form     | Role                                           |
| -------- | ---------------------------------------------- |
| `asm`    | local assembly identity and load/link contract |
| `asmref` | referenced assembly identity and origin data   |
| `file`   | source/image metadata                          |
| `import` | imported module/native/foreign binding         |
| `export` | exported callable/value surface                |
| `type`   | typed value/reference/layout metadata          |
| `sig`    | callable stack input/output contract           |
| `global` | global storage declaration                     |
| `const`  | typed constant declaration                     |
| `proc`   | invokable declaration and optional body        |
| `ext`    | core extension row/opcode ownership            |
| `tool`   | optional non-semantic tool/source metadata     |

`asm` owns module entry metadata through `(entry symbol)`. Procedure entry labels are ordinary labels inside `proc` bodies.

Inside `ext`, `section` takes section-family id, row-kind id, then policy. It declares extension row kind hosted by existing binary section family. It does not create new binary section family. Policy = `required` or `skippable`.

## Procedures

Procedure has symbol + signature reference:

```text
(proc name sig-name
  optional metadata/forms...
  label:
    instruction operands...)
```

Procedure-local forms before instruction lines may declare locals, env cells, regions, extern/native origins, intrin/runtime origins, and metadata:

```text
(proc puts puts-sig
  (extern
    (abi c)
    (symbol "puts")))
```

Executable instructions appear directly inside `proc` after local forms. Body lines are labels, instructions, blanks, or `;` comments. Closing parentheses follow body line terminator; not part of instruction line.

## Signatures

Signatures use compact `sig`, `in`, and `out` forms:

```text
(sig distance
  (in a Point)
  (in b Point)
  (out f64))
```

Inputs may be named or anonymous. Outputs are ordered type entries. Verifier expands `inputs(S)` and `outputs(S)` from signature metadata.

## Instruction Bodies

Bodies are RPN stack assembly. Operands explicit; no implicit source-level loads.

```text
entry:
  ld.arg width
  ld.arg height
  mul
  ret
```

Correct scalar constants use locked opcode mnemonics:

```text
const.int i32 0
const.nat n64 10
const.flt f64 1.5
const.bit true
const.nil (ref Node)
```

## Symbols And Escaping

Directive words are SEIL syntax. Program symbols are exact logical symbols. Assemblers must not case-fold, dash-convert, Unicode-normalize, abbreviate, or rewrite symbols.

Simple symbols may be bare. Symbols colliding with directive-head positions or containing non-bare chars use backticks:

```text
`module`
`name-with-dash`
`weird space`
```

Inside escaped symbols, `` \` `` denotes literal backtick and `\\` denotes literal backslash. Newlines forbidden inside escaped symbols.

Strings use double quotes with backslash escapes for `"`, `\`, `n`, `r`, `t`, and `u{HEX}` Unicode scalar values. Canonical text emits shortest valid escapes and UTF-8 source text.

## Canonical Ordering

Canonical `.seil` declaration order:

1. `asm`
2. `asmref`
3. `file`
4. `import`
5. `type`
6. `sig`
7. `global`
8. `const`
9. `proc`
10. `export`
11. `ext`
12. `tool`

Within one class, canonical output preserves source/tool order when available; else sort by exact symbol byte order.

Metadata argument order is non-semantic. Tools wanting original order must store it in `tool` metadata.

## Assembler Obligations

Assembler must:

- resolve symbols to namespace-relative table indices;
- preserve exact source/program symbols;
- resolve body labels to body-local block metadata;
- resolve mnemonics to opcode ids from `seil_opcodes.def`;
- validate operands against opcode schemas;
- encode declarations into typed tables for SEAM loading;
- reject unknown executable semantics;
- keep tool metadata non-semantic.

## Unknowns

- Exact canonical whitespace policy not specified.
- Exact diagnostic wording for text parse/assemble failures not specified.
- Exact `tool` metadata schemas not specified.
