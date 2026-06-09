# SEAM bytecode verification

SEAM bytecode verification = static acceptance pass over decoded modules before SEAM link/execute. It checks module shape, table refs, opcode schemas, stack effects, control edges, metadata refs, GC roots, barriers, capabilities, and target gates.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, `seam_bytecode_opcodes.def`, `grammar/seam-bytecode-text.ebnf`.

External sources:

- WebAsm validation separates module/type checking from execution: <https://webassembly.github.io/spec/core/valid/index.html>.
- JVM/CLI verify typed bytecode before execution: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10>, <https://docs.ecma-international.org/ecma-335/Ecma-335-part-i-iv.pdf>.
- Wasmtime stack maps show precise live-ref metadata at safepoints: <https://bytecodealliance.org/articles/reference-types-in-wasmtime>.

## Inputs

Verifier consumes decoded SEAM bytecode module, active target metadata, capability metadata, opcode schemas, type/layout tables, sig tables, procedure declarations, body-local metadata tables, import/export declarations, and the generated compatibility relation. It does not consume Musi source and does not depend on optional tool metadata.

## Acceptance order

1. Validate fixed header + section directory.
2. Locate exactly one mandatory `asm` section; decode identity/version/entry rows using only core container format version.
3. Decode dependency contracts from `deps` before remaining semantic payloads.
4. Resolve runtime requirements, capabilities, asm refs, imports, core ext row-kind declarations, and core ext opcode schema declarations from `deps`.
5. Reject unsupported required dependency contracts.
6. Decode remaining section payloads into typed logical tables under accepted contracts; skip only metadata core marks non-semantic + skippable.
7. Validate table shape, index ranges, and required acyclic dependencies.
8. Verify types, layouts, signatures, globals, constants, imports, exports, procedure declarations.
9. Decode each body with active opcode schema set.
10. Verify body-local metadata tables before referenced instructions.
11. Verify each instruction operand + stack effect using the generated compatibility relation.
12. Verify control-flow joins + terminal edges.
13. Derive safepoints + live managed-reference maps for stack, args, locals, envs, globals.
14. Verify managed-reference writes carry barrier obligations.
15. Compute verifier-owned execution metadata: max stack depth, frame-storage requirements.

Authored `.maxstack` is never authority. Verifier computes stack bounds.

## Opcode schema validation

Each opcode id has one active schema. Core opcodes use `seam_bytecode_opcodes.def`. Core ext opcodes require `deps` metadata that supplies schema before operand decoding. Unknown opcode ids fail verification.

Operand validation is schema-driven:

| Operand schema                                                                    | Validation                                                               |
| --------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| fixed scalar (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`) | byte width and canonical encoding match schema                           |
| `varu` / `vari`                                                                   | LEB128 form decodes without overflow and uses schema-required signedness |
| table index (`type_idx`, `sig_idx`, `func_idx`, etc.)                             | decoded `varu` in range for referenced logical namespace                 |
| body-local index (`block_idx`, `table_idx`, `region_idx`, `addr_idx`)             | referenced metadata exists in current body and has opcode-required shape |

## Stack model

Stack-effect strings in `seam_bytecode_opcodes.def` are compact canonical schemas. Verification interprets them against concrete body types + metadata.

Rules:

- Rightmost stack-effect item = top of stack.
- `...` preserves prefix stack.
- `terminal` means no next-instruction continuation.
- `terminal-or-next` means one successor terminal, one successor fallthrough.
- `terminal-or-region-exit` means cleanup/region edge decides local fallthrough.
- Polymorphic names (`A`, `B`, `T`, `P`, `F`, `S`, `E`) constrained by operands and type/layout metadata.
- `inputs(S)` and `outputs(S)` read from signature metadata.
- `yield(S)` and `resume(S)` read from suspension metadata for `S`.

Body accepted only when every instruction consumes available compatible stack values and produces declared output stack.

## GC root and barrier verification

Verifier classifies every stack, arg, local, env, global, and heap field as managed ref, unmanaged pointer/access, unmanaged storage value, address, scalar, aggregate, boxed value, callable, or core runtime kind. At each safepoint, exact live managed refs must be enumerable.

Safepoints: allocation, ordinary/dynamic calls, throws, yields, native/foreign boundaries that can allocate/block/call back, and any opcode schema-declared safepoint. SEAM may add safepoints only if it can derive equivalent live-ref maps.

Stores into ref-bearing heap fields, arrays, boxes, globals, or other ref-bearing locations require write barrier. Barrier obligations are layout-driven: they derive from ref maps, layout metadata, storage effects declared by opcode schema, and active collector policy. Raw memory ops cannot write managed-ref fields unless core checked op preserves barriers + root visibility.

## Diagnostics

Verifier diagnostics use stable codes/kinds plus subject-first full messages, labels, and real hints. Diagnostics include module/proc/body context and exact offending opcode, operand, table ref, stack value, metadata row, or control edge when known.

## Detail gaps

- Exact generated compatibility relation entries are not fully specified.
- Exact verifier diagnostic code/message catalog is not specified here.
