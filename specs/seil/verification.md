# SEIL verification

SEIL verification is the static acceptance pass over decoded SEIL modules before SEAM linking or execution. It validates module structure, table references, opcode schemas, stack effects, control edges, metadata references, GC root maps, barrier obligations, capabilities, and target gates.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, `seil_opcodes.def`, `grammar/seil.ebnf`.

External sources used:

- WebAsm validation separates module/type checking from execution: <https://webassembly.github.io/spec/core/valid/index.html>.
- JVM and CLI verification check typed bytecode before execution: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10>, <https://docs.ecma-international.org/ecma-335/Ecma-335-part-i-iv.pdf>.
- Wasmtime reference-type stack maps illustrate precise live-reference metadata at safepoints: <https://bytecodealliance.org/articles/reference-types-in-wasmtime>.

## Inputs

The verifier consumes a decoded SEIL module, active target metadata, capability metadata, opcode schemas, type/layout tables, sig tables, procedure declarations, body-local metadata tables, and import/export declarations. It does not consume Musi source syntax and does not depend on optional tool metadata.

## Acceptance order

1. Validate the fixed header and section directory.
2. Locate exactly one mandatory `asm` section and decode its identity/version/entry rows using only the core container format version.
3. Decode dependency contracts from `deps` before remaining semantic payloads.
4. Resolve runtime requirements, capability requirements, asm references, imports, core ext row-kind declarations, and core ext opcode schema declarations from `deps`.
5. Reject the module if any required dependency contract is unsupported.
6. Decode remaining section payloads into typed logical tables according to accepted dependency contracts; skip only metadata declared non-semantic and skippable by core.
7. Validate table shape, index ranges, and acyclic table dependencies where required by table kind.
8. Verify types, layouts, signatures, globals, constants, imports, exports, and procedure declarations.
9. Decode each body with the active opcode schema set.
10. Verify body-local metadata tables before instructions that reference them.
11. Verify every instruction operand and stack effect.
12. Verify control-flow joins and terminal edges.
13. Derive safepoints and live managed-reference maps for evaluation stack, arguments, locals, environments, and globals.
14. Verify that managed-reference writes carry required barrier obligations.
15. Compute verifier-owned execution metadata such as maximum stack depth and frame frame-storage requirements.

No authored `.maxstack` is accepted as authority. Stack bounds are verifier-computed.

## Opcode schema validation

Each opcode id has exactly one active schema. Core opcodes use `seil_opcodes.def`. Core ext opcodes require `deps`-declared metadata that supplies their schema before operand decoding. Unknown opcode ids fail verification.

Operand validation is schema-driven:

| Operand schema                                                                    | Validation                                                                              |
| --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| fixed scalar (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`) | byte width and canonical encoding match schema                                          |
| `varu` / `vari`                                                                   | LEB128 form decodes without overflow and uses the signedness required by schema         |
| table index (`type_idx`, `sig_idx`, `func_idx`, etc.)                             | decoded `varu` is in range for the referenced logical namespace                         |
| body-local index (`block_idx`, `table_idx`, `region_idx`, `addr_idx`)             | referenced metadata exists in the current body and has the shape required by the opcode |

## Stack model

Stack-effect strings in `seil_opcodes.def` are the canonical compact schemas. Verification interprets them against concrete body types and body metadata.

Rules:

- The rightmost item in a stack-effect schema is the top of stack.
- `...` preserves the prefix stack.
- `terminal` means control does not continue to the next instruction.
- `terminal-or-next` means one successor is terminal and one successor is the fallthrough instruction.
- `terminal-or-region-exit` means a cleanup/region edge controls whether local fallthrough exists.
- Polymorphic names such as `A`, `B`, `T`, `P`, `F`, `S`, and `E` are constrained by opcode operands and type/layout metadata.
- `inputs(S)` and `outputs(S)` are read from signature metadata.
- `yield(S)` and `resume(S)` are read from suspension metadata associated with `S`.

A body is accepted only when every instruction consumes available stack values of compatible types and produces the declared output stack.

## GC root and barrier verification

The verifier classifies every stack, argument, local, environment, global, and heap field value as managed reference, unmanaged pointer, scalar, value aggregate, boxed value, callable value, or core-defined runtime kind. At every safepoint, it must be possible to enumerate exact live managed references.

Safepoints include allocation, ordinary and dynamic calls, throws, yields, native/foreign boundaries that can allocate, block, or call back, and any opcode declared as a safepoint by its schema. A SEAM implementation may add extra safepoints only when it can derive equivalent live-reference maps.

Stores into managed-reference-bearing heap fields, array elements, boxed layouts, globals, or other reference-bearing locations have a write-barrier obligation. Raw memory operations cannot write managed-reference fields unless a core checked operation preserves barriers and root visibility.

## Unknowns

- Exact compatibility edge schemas are not fully specified.
- Exact diagnostic codes/messages for verifier failures are not specified here.
