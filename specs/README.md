# Musi/SEIL/SEAM specification map

Specs are grouped by system boundary.

- `musi/`: Musi source-language behavior that must lower to SEIL.
- `seil/`: Stack Effect Intermediate Language artifacts, text format, opcodes, verification, instruction behavior, and metadata contracts.
- `seam/`: Stack Effect Abstract Machine runtime behavior for loading, linking, executing, memory management, dynamic protocols, and failures.

## Boundary rules

Musi is source. SEIL is the typed executable artifact: lower-level than ASTs, higher-level than typical compiler IRs, WAT/Lisp-like in declarations, Forth/RPN-like in instruction bodies, CIL-like in assembly/reference metadata role, and designed around a small core of clean executable constructs. SEAM is the VM/runtime that executes verified SEIL.

Behavior that affects runtime execution must appear in SEIL asm contracts, semantic sections, or required VM metadata. Optional tool metadata can support tooling but cannot be required for execution.

SEIL/SEAM do not use executable-semantics dialects. A behavior is core, library/native, frontend-owned, or unsupported. Unknown executable semantics are rejected.

## Current spec files

| File                             | Scope                                                                                    |
| -------------------------------- | ---------------------------------------------------------------------------------------- |
| `musi/lowering-to-seil.md`       | Musi-to-SEIL boundary, known execution, targets, FFI, fixed storage, shapes/witnesses    |
| `musi/control-lowering.md`       | Musi control forms and their SEIL/runtime obligations                                    |
| `musi/runtime-expectations.md`   | Musi expectations over SEAM execution, memory, dynamic behavior, failures                |
| `seil/modules-artifacts.md`      | SEIL module text, asm identity, artifact roles, imports/exports, procedure ownership     |
| `seil/binary-image-format.md`    | SEAM binary image structure, 40-byte header, asm section, sections, instruction encoding |
| `seil/text-format.md`            | `.seil` textual IL syntax and assembler obligations                                      |
| `seil/opcodes.md`                | locked core opcode registry sourced from `seil_opcodes.def`                              |
| `seil/operands-stack-effects.md` | immediate operands, index namespaces, stack-effect notation, stack validation            |
| `seil/verification.md`           | module/procedure-body verification, stack effects, metadata references, failure cases    |
| `seil/instructions.md`           | behavioral intent for locked core instruction families                                   |
| `seil/types-metadata.md`         | type, signature, layout, required VM metadata, optional tool metadata                    |
| `seam/runtime.md`                | SEAM lifecycle, execution state, known phase, linking, control edges                     |
| `seam/frames-control.md`         | frames, calls, returns, branches, exceptions, cleanup, suspension                        |
| `seam/failures-and-limits.md`    | traps, structured failures, resource limits, halt outcomes                               |
| `seam/memory-gc.md`              | values, references, pointers, fixed storage, allocation, GC intent                       |
| `seam/dynamic-capabilities.md`   | explicit dynamic, capability, box/unbox, and keyed-storage protocols                     |
