# SEIL Modules And Artifacts

SEIL module = executable unit loaded by SEAM. Public SEIL is textual `.seil`. SEAM tooling may assemble text into internal binary image before execution.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, `grammar/seil.ebnf`.

## Artifact Roles

| Artifact          | Producer                          | Consumer                    | Behavior               |
| ----------------- | --------------------------------- | --------------------------- | ---------------------- |
| `.ms`             | Musi frontend/user                | Musi compiler               | source input           |
| `.seil`           | compiler, developer, disassembler | SEIL assembler/SEAM tooling | textual executable IL  |
| SEAM binary image | assembler/cache/package tooling   | SEAM loader/runtime         | dense executable image |

Developers may hand-author `.seil`. Assemblers must not invent behavior outside SEIL. Disassemblers preserve executable semantics except omitted optional tool metadata.

## Design Model

SEIL text uses WAT/Lisp-like `module` root because SEIL is typed executable module language. Declarations carry CIL-like assembly/reference, metadata, and body roles. Procedure bodies use Forth/RPN-like stack instruction streams.

SEIL removes CIL object-model center. No mandatory class declaration, method modifier soup, or authored `.maxstack`.

Core declarations are VM-behavior declarations: `asm`, `asmref`, `type`, `layout`, `sig`, `global`, `const`, `proc`, `import`, `export`, `ext`, `tool`.

## Asm Identity

Module has exactly one local `asm`. It provides load/link identity, asm version, entry metadata, and textual home for early runtime/capability contract members.

Asm refs use `asmref`, keeping CIL distinction between current asm and referenced asms. Reference name + version are semantic dependency identity. Origin strings such as `musi:core` describe dependency origin but do not replace asm identity.

Assembly lowers textual `runtime`, `cap`, top-level `ext`, `asmref`, and `import` declarations into binary `deps` rows. Binary `asm` section stays identity/version/entry section, not dependency catch-all.

## Internal Binary Image

Internal binary image has fixed 40-byte probe header + section directory. Header is not module metadata. Semantic contracts live in sections/tables.

Mandatory early `asm` section encodes identity subset of textual local `asm`. Runtime, capability, extension, dependency, and import requirements live in `deps`.

## Section Families

Internal binary images use section families: `names`, `asm`, `deps`, `defs`, `code`, `data`, `meta`, `tool`.

Rows stay narrow:

- `deps`: runtime/cap/ext requirements, asm refs, imports
- `defs`: types, fields, alts, sigs, inputs, outputs, globals, consts, procs, exports
- `code`: bodies, control tables, instruction bytes
- `data`: const payloads, layouts, ref maps, ABI records, dynamic/cap schemas
- `meta`: required semantic metadata not owned elsewhere
- `tool`: optional non-semantic execution-skippable data

Each section payload starts with row-kind directory, then row offset table, then packed row bytes. Row-kind entries declare row presence and required/skippable policy. Prevents catch-all structs while preserving lookup + deterministic validation.

## Imports And Exports

Imports depend on module, native/compiler, or foreign-provided declarations. Exports expose callable/value surface. Compatibility uses signatures, types, layout metadata, target gates, capability requirements, ABI metadata.

Name match alone not enough. SEAM rejects link when semantic contracts mismatch.

## Procedure Ownership

Executable bodies belong to `proc`. Procedure has signature and exactly one implementation origin:

- SEIL instruction body;
- `extern` foreign/ABI binding;
- `intrin` SEAM/runtime binding;
- core-defined native/runtime origin.

Procedure body has block metadata, branch-table metadata, cleanup/handler/yield metadata, address-target metadata, arg/local/env storage metadata, instruction stream. Callee origin lives in procedure metadata, not distinct call opcodes.

## Validation And Failure Cases

Reject module before execution when:

- text parse fails;
- required declarations missing/duplicated;
- internal header or section directory malformed;
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

- Exact module-name canonicalization not fully specified.
- Exact package/archive format for multiple modules not specified.
