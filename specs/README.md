# Musi/SEIL/SEAM specification map

Specs grouped by boundary:

- `musi/`: Musi source behavior that lowers to SEIL.
- `seil/`: executable IL artifacts, text, opcodes, verification, instruction behavior, metadata.
- `seam/`: runtime loading, linking, execution, memory, dynamic protocols, failures.
- Full spec lock tracker: `docs/language_checklist_for_musi.md`.

## Boundary rules

Musi = source. SEIL = typed executable artifact: lower than ASTs, higher than disposable IR, WAT/Lisp-like declarations, Forth/RPN-like bodies, CIL-like asm/reference role. SEAM = VM/runtime executing verified SEIL.

Runtime-affecting behavior must appear in SEIL semantic declarations, compact binary section families, or required VM metadata. Optional tool metadata helps tooling only.

No executable-semantics dialects. Behavior is core, library/native, frontend-owned, or unsupported. Unknown executable semantics rejected.

Spec gates: simple, explicit, maintainable, one obvious way, no hidden magic, one-token-lookahead source syntax. If form needs more lookahead or hides runtime behavior, redesign before lock.

## Current spec files

| File                             | Scope                                                                                 |
| -------------------------------- | ------------------------------------------------------------------------------------- |
| `musi/lowering-to-seil.md`       | Musi-to-SEIL boundary, known execution, targets, FFI, fixed storage, shapes/witnesses |
| `musi/control-lowering.md`       | Musi control forms and SEIL/runtime obligations                                       |
| `musi/runtime-expectations.md`   | Musi expectations over SEAM execution, memory, dynamic behavior, failures             |
| `seil/modules-artifacts.md`      | SEIL module text, asm identity, artifact roles, imports/exports, procedure ownership  |
| `seil/binary-image-format.md`    | SEAM binary image structure, 40-byte header, compact sections, instruction encoding   |
| `seil/text-format.md`            | `.seil` textual IL syntax and assembler obligations                                   |
| `seil/opcodes.md`                | locked core opcode registry sourced from `seil_opcodes.def`                           |
| `seil/operands-stack-effects.md` | operands, index namespaces, stack-effect notation, validation                         |
| `seil/verification.md`           | module/body verification, stack effects, metadata refs, failures                      |
| `seil/instructions.md`           | locked core instruction-family behavior                                               |
| `seil/types-metadata.md`         | types, signatures, layouts, required VM metadata, optional tool metadata              |
| `seam/runtime.md`                | SEAM lifecycle, execution state, known phase, linking, control edges                  |
| `seam/frames-control.md`         | frames, calls, returns, branches, exceptions, cleanup, suspension                     |
| `seam/failures-and-limits.md`    | traps, structured failures, resource limits, halt outcomes                            |
| `seam/memory-gc.md`              | values, refs, access/address tokens, fixed storage, allocation, GC intent             |
| `seam/dynamic-capabilities.md`   | explicit dynamic, capability, box/unbox, keyed-storage protocols                      |
