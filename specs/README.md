# Musi/SEAM bytecode/SEAM specification map

Specs grouped by boundary:

- `musi/`: Musi source behavior that lowers to SEAM bytecode.
- `seam-bytecode/`: `.seam` compiled bytecode image, text/disassembly tooling format, opcodes, verification, instruction behavior, metadata.
- `seam/`: runtime loading, linking, execution, memory, dynamic protocols, failures.
- Full spec lock tracker: `docs/language_checklist_for_musi.md`.

## Boundary rules

Musi = source. SEAM bytecode = typed executable bytecode image: lower than ASTs, higher than disposable IR, with compact semantic sections and optional readable text/disassembly for tools. SEAM = VM/runtime executing verified `.seam` images.

Runtime-affecting behavior must appear in SEAM bytecode semantic declarations, compact binary section families, or required VM metadata. Optional tool metadata helps tooling only.

No executable-semantics dialects. Behavior is core, library/native, frontend-owned, or unsupported. Unknown executable semantics rejected.

Spec gates: simple, explicit, maintainable, one obvious way, no hidden magic, one-token-lookahead source syntax. If form needs more lookahead or hides runtime behavior, redesign before lock.

## Current spec files

| File                                      | Scope                                                                                          |
| ----------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `musi/lowering-to-seam-bytecode.md`       | Musi-to-SEAM-bytecode boundary, known execution, targets, FFI, fixed storage, shapes/witnesses |
| `musi/control-lowering.md`                | Musi control forms and SEAM bytecode/runtime obligations                                       |
| `musi/runtime-expectations.md`            | Musi expectations over SEAM execution, memory, dynamic behavior, failures, packages            |
| `seam-bytecode/modules-artifacts.md`      | SEAM bytecode module image, artifact roles, imports/exports, procedure ownership               |
| `seam-bytecode/binary-image-format.md`    | `.seam` binary image structure, 40-byte header, compact sections, instruction encoding         |
| `seam-bytecode/text-format.md`            | SEAM bytecode text/disassembly syntax and assembler obligations                                |
| `seam-bytecode/opcodes.md`                | locked core opcode registry sourced from `seam_bytecode_opcodes.def`                           |
| `seam-bytecode/operands-stack-effects.md` | operands, index namespaces, stack-effect notation, validation                                  |
| `seam-bytecode/verification.md`           | module/body verification, stack effects, metadata refs, failures                               |
| `seam-bytecode/instructions.md`           | locked core instruction-family behavior                                                        |
| `seam-bytecode/types-metadata.md`         | types, signatures, layouts, required VM metadata, optional tool metadata                       |
| `seam/runtime.md`                         | SEAM lifecycle, execution state, known phase, linking, control edges                           |
| `seam/frames-control.md`                  | frames, calls, returns, branches, exceptions, cleanup, suspension                              |
| `seam/failures-and-limits.md`             | traps, structured failures, resource limits, halt outcomes                                     |
| `seam/memory-gc.md`                       | values, refs, access/address tokens, fixed storage, allocation, GC intent                      |
| `seam/dynamic-capabilities.md`            | explicit dynamic, capability, box/unbox, keyed-storage protocols                               |
