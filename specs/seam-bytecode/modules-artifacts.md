# SEAM bytecode Modules And Artifacts

SEAM bytecode module = executable unit loaded by SEAM. Public compiled artifact is `.seam`: a dense SEAM bytecode image with fixed header, section directory, semantic rows, and instruction streams.

SEAM bytecode text/disassembly is a readable tool format for assembly, disassembly, fixtures, diagnostics, and debugging. It is not a separate package/distribution artifact extension.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, `grammar/seam-bytecode-text.ebnf`.

## Artifact Roles

| Artifact                       | Producer                                 | Consumer                             | Behavior                |
| ------------------------------ | ---------------------------------------- | ------------------------------------ | ----------------------- |
| `.ms`                          | Musi frontend/user                       | Musi compiler                        | source input            |
| `.seam`                        | compiler, assembler, cache/build tooling | SEAM loader/runtime, package tooling | compiled bytecode image |
| SEAM bytecode text/disassembly | disassembler, developer, tests           | assembler/tooling                    | readable bytecode view  |

Developers may hand-author the text/disassembly form and assemble it to `.seam`. Assemblers must not invent behavior outside SEAM bytecode. Disassemblers preserve executable semantics except omitted optional tool metadata.

## Design Model

SEAM bytecode text/disassembly uses WAT/Lisp-like `module` root because SEAM bytecode is typed executable module language. Declarations carry CIL-like assembly/reference, metadata, and body roles. Procedure bodies use Forth/RPN-like stack instruction streams.

SEAM bytecode removes CIL object-model center. No mandatory class declaration, method modifier soup, or authored `.maxstack`.

Core declarations are VM-behavior declarations: `asm`, `asmref`, `type`, `layout`, `sig`, `global`, `const`, `proc`, `import`, `export`, `ext`, `tool`.

## Asm Identity

Module has exactly one local `asm`. It provides load/link identity, asm version, entry metadata, and textual home for early runtime/capability contract members.

Asm refs use `asmref`, keeping CIL distinction between current asm and referenced asms. Reference name + version are semantic dependency identity. Origin strings such as `musi:core` describe dependency origin but do not replace asm identity.

Assembly lowers textual `runtime`, `cap`, top-level `ext`, `asmref`, and `import` declarations into binary `deps` rows. Binary `asm` section stays identity/version/entry section, not dependency catch-all.

## `.seam` Image

A `.seam` image has fixed 40-byte probe header + section directory. Header is not module metadata. Semantic contracts live in sections/tables.

Mandatory early `asm` section encodes identity subset of textual local `asm`. Runtime, capability, extension, dependency, and import requirements live in `deps`.

## Section Families

`.seam` images use section families: `names`, `asm`, `deps`, `defs`, `code`, `data`, `meta`, `tool`.

Rows stay narrow:

- `deps`: runtime/cap/ext requirements, asm refs, imports
- `defs`: types, fields, alts, sigs, inputs, outputs, globals, consts, procs, exports
- `code`: bodies, control tables, instruction bytes
- `data`: const payloads, layouts, ref maps, ABI records, dynamic/cap schemas
- `meta`: required semantic metadata not owned elsewhere
- `tool`: optional non-semantic execution-skippable data

Each section payload starts with row-kind directory, then row offset table, then packed row bytes. Row-kind entries declare row presence and required/skippable policy. Prevents catch-all structs while preserving lookup + deterministic validation.

## Imports And Exports

Imports depend on module, native/compiler, host-provided, or foreign-provided declarations. Exports expose callable/value surface. Compatibility uses signatures, types, layout metadata, target gates, capability requirements, ABI metadata.

Name match alone not enough. SEAM rejects link when semantic contracts mismatch.

Host-provided modules are explicit graph nodes with provider and capability metadata. They do not appear through ambient globals.

## Procedure Ownership

Executable bodies belong to `proc`. Procedure has signature and exactly one implementation origin:

- SEAM bytecode instruction body;
- `extern` foreign/ABI binding;
- `intrin` SEAM/runtime binding;
- core-defined native/runtime origin.

Procedure body has block metadata, branch-table metadata, cleanup/handler/yield metadata, address-target metadata, arg/local/env storage metadata, instruction stream. Callee origin lives in procedure metadata, not distinct call opcodes.

## Validation And Failure Cases

Reject module before execution when:

- text parse fails;
- required declarations missing/duplicated;
- `.seam` header or section directory malformed;
- mandatory asm section missing, duplicated, truncated, undecodable;
- required sections missing/truncated;
- table indices reference missing entries;
- declarations reference unavailable target features or unsupported core ext contracts;
- imports/exports semantically incompatible;
- body verification fails;
- required body/VM metadata absent;
- ext row-kind or opcode schemas required by `deps` unsupported;
- unknown executable opcodes, semantic section kinds, or required semantic row kinds appear.

## Unknowns

- Exact package/archive format for multiple modules not specified.
