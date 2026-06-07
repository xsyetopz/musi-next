# SEIL operands and stack effects

Opcode id, immediate operands, table metadata, and stack-effect schemas determine SEIL instruction behavior. Operand decoding belongs to instruction validation, not best-effort runtime recovery.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` section 18, `seil_opcodes.def`, `grammar/seil.ebnf`.

References: WebAsm validates stack effects and typed instruction sequences before execution: <https://webassembly.github.io/spec/core/valid/instructions.html>. JVM verification uses typed operand-stack and local-variable constraints before code can execute: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1>.

## Immediate operand encodings

| Schema                    | Meaning                            | Validation                                                   |
| ------------------------- | ---------------------------------- | ------------------------------------------------------------ |
| `u8`, `u16`, `u32`, `u64` | unsigned fixed-width integer       | exact byte width, no overflow                                |
| `i8`, `i16`, `i32`, `i64` | signed fixed-width integer         | exact byte width, no overflow                                |
| `f32`, `f64`              | IEEE-sized floating scalar payload | exact byte width; semantic acceptance depends on target type |
| `varu`                    | unsigned LEB128 integer            | canonical unsigned decode, no overflow                       |
| `vari`                    | signed LEB128 integer              | canonical signed decode, no overflow                         |

Fixed-width integer and floating operands use little-endian byte order in the SEAM binary image. Textual `.seil` uses source literals and assembles them into the same operand values.

## Index operands

Index operands are `varu` values into a namespace. The same numeric value can name different entries in different namespaces.

| Operand      | Namespace                                                      |
| ------------ | -------------------------------------------------------------- |
| `type_idx`   | module type table                                              |
| `sig_idx`    | module signature table                                         |
| `func_idx`   | procedure declaration table                                    |
| `field_idx`  | named or positional product-field namespace selected by opcode |
| `alt_idx`    | sum alternative table                                          |
| `global_idx` | global storage table                                           |
| `const_idx`  | constant table                                                 |
| `block_idx`  | body-local basic-block table                                   |
| `table_idx`  | body-local branch-table metadata                               |
| `region_idx` | body-local cleanup/handler/yield region metadata               |
| `cap_idx`    | capability metadata/evidence table                             |
| `arg_idx`    | body/frame argument storage metadata                           |
| `loc_idx`    | body/frame local storage metadata                              |
| `env_idx`    | body/frame environment/capture metadata                        |
| `addr_idx`   | body-local address-target metadata                             |

Verification rejects out-of-range index operands and namespace mismatches.

## Stack-effect notation

Stack-effect strings use the notation locked in `seil_opcodes.def`.

| Notation                          | Meaning                                              |
| --------------------------------- | ---------------------------------------------------- |
| `...`                             | preserved lower stack prefix                         |
| rightmost item                    | top of stack                                         |
| `A`, `B`, `T`, `P`, `F`, `S`, `E` | type variables constrained by opcode and metadata    |
| `Bit`, `Nat`, `Byte`              | primitive scalar classes                             |
| `Ref[T]`, `Ptr[T]`, `Fn[S]`       | typed reference, pointer, callable value             |
| `inputs(S)`, `outputs(S)`         | expanded from signature metadata                     |
| `yield(S)`, `resume(S)`           | expanded from suspension metadata                    |
| `terminal`                        | no fallthrough successor                             |
| `terminal-or-next`                | one edge terminates and the fallthrough edge remains |
| `terminal-or-region-exit`         | region metadata determines local continuation        |

## Verification behavior

The verifier computes each instruction's input and output stack shape. At a block boundary, every predecessor must provide a stack compatible with the target block's entry stack. A conditional branch has two verified successors. A branch table has one verified successor per table arm plus any default arm defined by table metadata.

Type variables are not untyped wildcards. They are solved from operands, table metadata, incoming stack types, and opcode-specific constraints.

## Failure cases

Verification fails when an instruction consumes unavailable stack values, stack values have incompatible types, a branch target expects a different stack, an operand references an invalid table entry, an ext opcode lacks a schema, or metadata cannot solve a stack-effect variable.

## Canonical LEB128

`varu` and `vari` encodings must be shortest-form encodings for their decoded value. Decoders reject encodings that exceed 64 payload bits, encodings that continue past the instruction body, and encodings whose shorter equivalent would decode to the same value.

## Type compatibility

The default compatibility rule is exact type equality after resolving aliases. Core type metadata may declare explicit widening, representation, nil-admission, callable, box/unbox, or dynamic compatibility edges. The verifier must not invent compatibility from source-language names alone.

## Unknowns

- Exact compatibility edge schema is not fully specified.
