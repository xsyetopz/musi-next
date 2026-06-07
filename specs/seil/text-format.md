# SEIL Text Format

SEIL (`.seil`) is the hand-writable executable intermediate language for SEAM. It is text, not Musi source and not a binary instruction file. Musi compilers emit `.seil`; developers may author `.seil`; SEAM tooling assembles or loads `.seil` into an internal binary image before execution.

SEIL text combines WAT/Lisp-like declarations with Forth/RPN-like instruction bodies:

- module and metadata structure uses parenthesized forms;
- executable procedure bodies use line-oriented stack instructions;
- source/program symbols are preserved exactly;
- symbolic operands assemble to binary table indices in the internal image;
- canonical disassembly emits symbols when stable names exist.

Project evidence: `grammar/seil.ebnf`, `LOCKED_LANGUAGE_DESIGN.md` sections 16-18.

## File Structure

```ebnf
seil-text ::= form-ws? module form-ws?
module    ::= "(" "module" ws module-name module-decl* form-ws? ")"
```

One `.seil` file contains exactly one `module` declaration.

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

Inside `ext`, `section` takes two numeric operands before policy: section-family id, then row-kind id. It declares an extension row kind hosted by an existing binary section family; it does not create a new binary section family. Policy is `required` or `skippable`.

## Procedures

A procedure has a symbol and a signature reference:

```text
(proc name sig-name
  optional metadata/forms...
  label:
    instruction operands...)
```

Procedure-local forms before instruction lines may declare locals, environment cells, regions, extern/native origins, intrin/runtime origins, and metadata:

```text
(proc puts puts-sig
  (extern
    (abi c)
    (symbol "puts")))
```

Executable instructions appear directly inside the `proc` form after any local declaration forms. Body lines are labels, instructions, blank lines, or `;` comments. Closing parentheses follow a body line terminator; they are not part of an instruction line.

## Signatures

Signatures use compact `sig`, `in`, and `out` forms:

```text
(sig distance
  (in a Point)
  (in b Point)
  (out f64))
```

Inputs may be named or anonymous. Outputs are ordered type entries. The verifier expands `inputs(S)` and `outputs(S)` from signature metadata.

## Instruction Bodies

Instruction bodies are RPN stack assembly. Operands are explicit; SEIL has no implicit source-level loads.

```text
entry:
  ld.arg width
  ld.arg height
  mul
  ret
```

Correct scalar constants use the locked opcode mnemonics:

```text
const.int i32 0
const.nat n64 10
const.flt f64 1.5
const.bit true
const.nil (ref Node)
```

Scalar constants use the opcode mnemonics shown above.

## Symbols And Escaping

Directive words are SEIL syntax. Program symbols are exact logical symbols. Assemblers must not case-fold, dash-convert, Unicode-normalize, abbreviate, or rewrite symbols.

Simple symbols may be bare. Symbols that collide with directive-head positions or contain non-bare characters use backtick escaping:

```text
`module`
`name-with-dash`
`weird space`
```

Inside escaped symbols, `` \` `` denotes a literal backtick and `\\` denotes a literal backslash. Newlines are not allowed inside escaped symbols.

Strings use double quotes with backslash escapes for `"`, `\`, `n`, `r`, `t`, and `u{HEX}` Unicode scalar values. Canonical text emits shortest valid escapes and UTF-8 source text.

## Canonical Ordering

Canonical `.seil` output orders declarations by semantic dependency:

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

Within one dependency class, canonical output preserves source/tool order when available; otherwise it sorts by exact symbol byte order.

Non-canonical metadata argument order is not semantic. If a tool wants to preserve original order, it must store that order in `tool` metadata.

## Assembler Obligations

An assembler must:

- resolve symbols to namespace-relative table indices for the internal image;
- preserve exact source/program symbols;
- resolve body labels to body-local block metadata;
- resolve mnemonics to opcode ids from `seil_opcodes.def`;
- validate instruction operands against opcode schemas;
- encode declarations into typed tables for SEAM loading;
- reject unknown executable semantics;
- keep tool metadata non-semantic.
