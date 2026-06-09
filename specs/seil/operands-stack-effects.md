# SEIL operands and stack effects

Opcode id, immediate operands, table metadata, and stack-effect schemas define SEIL instruction behavior. Operand decoding belongs to validation, not runtime recovery.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` section 18, `seil_opcodes.def`, `grammar/seil.ebnf`.

References: WebAsm validates stack effects before execution: <https://webassembly.github.io/spec/core/valid/instructions.html>. JVM verifies typed operand stack + locals before code executes: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1>.

## Immediate operand encodings

| Schema                    | Meaning                            | Validation                                                   |
| ------------------------- | ---------------------------------- | ------------------------------------------------------------ |
| `u8`, `u16`, `u32`, `u64` | unsigned fixed-width integer       | exact byte width, no overflow                                |
| `i8`, `i16`, `i32`, `i64` | signed fixed-width integer         | exact byte width, no overflow                                |
| `f32`, `f64`              | IEEE-sized floating scalar payload | exact byte width; semantic acceptance depends on target type |
| `varu`                    | unsigned LEB128 integer            | canonical unsigned decode, no overflow                       |
| `vari`                    | signed LEB128 integer              | canonical signed decode, no overflow                         |

Fixed-width integer and float operands use little-endian bytes in SEAM binary image. Text `.seil` uses source literals assembled to same values.

## Index operands

Index operands are `varu` into namespace. Same number may name different entries in different namespaces.

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

Verification rejects out-of-range indices and namespace mismatch.

## Stack-effect notation

Stack-effect strings use `seil_opcodes.def` notation.

| Notation                          | Meaning                                           |
| --------------------------------- | ------------------------------------------------- |
| `...`                             | preserved lower stack prefix                      |
| rightmost item                    | top of stack                                      |
| `A`, `B`, `T`, `P`, `F`, `S`, `E` | type variables constrained by opcode and metadata |
| `Bit`, `Nat`, `Byte`              | primitive scalar classes                          |
| `Ref[T]`, `Ptr[T]`, `Fn[S]`       | VM reference, VM pointer/access value, callable   |
| `inputs(S)`, `outputs(S)`         | expanded from signature metadata                  |
| `yield(S)`, `resume(S)`           | expanded from suspension metadata                 |
| `terminal`                        | no fallthrough successor                          |
| `terminal-or-next`                | one edge terminates and fallthrough remains       |
| `terminal-or-region-exit`         | region metadata determines local continuation     |

## Verification behavior

Verifier computes each instruction input/output stack shape. At block boundary, every predecessor must provide stack compatible with target entry stack. Conditional branch has two verified successors. Branch table has one successor per arm plus optional default arm.

Type variables are not wildcards. They solve from operands, table metadata, incoming stack types, opcode constraints.

## Failure cases

Verification fails when instruction consumes unavailable values, stack values have incompatible types, branch target expects different stack, operand references invalid table entry, ext opcode lacks schema, or metadata cannot solve stack-effect variable.

## Canonical LEB128

`varu` and `vari` must be shortest-form encodings. Decoders reject >64 payload bits, encodings past body end, and encodings whose shorter equivalent decodes same.

## Type compatibility

Default compatibility = exact type equality after aliases. Core metadata may declare explicit widening, representation, nil-admission, callable, box/unbox, or dynamic compatibility edges. Verifier must not invent compatibility from source names.

## Unknowns

- Exact compatibility edge schema not fully specified.
