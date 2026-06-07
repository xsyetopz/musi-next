# SEIL Modules And Artifacts

A SEIL module is the executable unit loaded by SEAM. Public SEIL is textual `.seil`. SEAM tooling may assemble that text into an internal binary image before execution.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, `grammar/seil.ebnf`.

## Artifact Roles

| Artifact          | Producer                          | Consumer                    | Behavior                        |
| ----------------- | --------------------------------- | --------------------------- | ------------------------------- |
| `.ms`             | Musi frontend/user                | Musi compiler               | source-language input           |
| `.seil`           | compiler, developer, disassembler | SEIL assembler/SEAM tooling | textual executable IL           |
| SEAM binary image | assembler/cache/package tooling   | SEAM loader/runtime         | dense internal executable image |

Developers may hand-author `.seil`. Assemblers must not invent behavior outside SEIL. Disassemblers preserve executable semantics, except for optional tool metadata they intentionally omit.

## Design Model

SEIL text uses a WAT/Lisp-like `module` root because SEIL is a typed executable module language. Declarations carry CIL-like assembly/reference, metadata, and executable-body roles. Procedure bodies use Forth/RPN-like stack instruction streams.

SEIL removes the CIL object-model center. There is no mandatory class declaration, no method modifier soup, and no authored `.maxstack`.

Core declarations are VM-behavior declarations:

- `asm`
- `asmref`
- `type`
- `layout`
- `sig`
- `global`
- `const`
- `proc`
- `import`
- `export`
- `ext`
- `tool`

## Asm Identity

A module has exactly one local `asm` declaration. It provides load/link identity, asm version, entry metadata, and the textual home for early runtime/capability contract members.

Asm references use `asmref`, following the CIL distinction between current asm and referenced asms. Reference name and version are semantic dependency identity. Origin strings describe where a dependency comes from, such as `musi:core`, but do not replace asm identity.

Assembly lowers textual `runtime` and `cap` members, top-level `ext` declarations, `asmref` declarations, and `import` declarations into binary `deps` rows. The binary `asm` section remains the identity/version/entry section, not a dependency catch-all.

## Internal Binary Image

The internal binary image has a fixed 40-byte probe header and section directory. The header is not module metadata. Semantic contracts live in sections and tables.

The mandatory early `asm` section is the encoded identity subset of the textual local `asm` declaration. Runtime, capability, extension, dependency, and import requirements live in `deps`.

## Section Families

Internal binary images use these section families:

- `names`
- `asm`
- `deps`
- `defs`
- `code`
- `data`
- `meta`
- `tool`

Rows inside those sections stay narrow. `deps` owns runtime/cap/ext requirements, asm refs, and imports. `defs` owns types, fields, alts, signatures, globals, constants, procedures, and exports. `code` owns body/control tables and instruction bytes. `data` owns constant payloads, layouts, ref maps, ABI records, and dynamic/cap schemas. `meta` owns required semantic metadata not owned elsewhere. `tool` is optional, non-semantic, and skippable for execution.

## Imports And Exports

Imports declare dependencies on module, native/compiler, or foreign-provided declarations. Exports declare callable or value surfaces visible outside the module. Compatibility uses signatures, types, layout metadata, target gates, capability requirements, and ABI metadata.

An import/export name alone is insufficient for compatibility. SEAM rejects a link where the name matches but semantic contracts do not.

## Procedure Ownership

Executable bodies belong to `proc` declarations. A `proc` has a signature and exactly one implementation origin:

- SEIL instruction body;
- `extern` foreign/ABI binding;
- `intrin` SEAM/runtime binding;
- core-defined native/runtime origin.

A procedure body has block metadata, branch-table metadata, cleanup/handler/yield metadata, address-target metadata, argument/local/environment storage metadata, and an instruction stream. Callee origin lives in procedure metadata, not in distinct call opcodes.

## Validation And Failure Cases

A module is rejected before execution when:

- text parsing fails;
- required declarations are missing or duplicated;
- the internal image header or section directory is malformed;
- the mandatory asm section is missing, duplicated, truncated, or not decodable;
- required sections are missing or truncated;
- table indices reference missing entries;
- declarations reference unavailable target features or unsupported core ext contracts;
- imports/exports are semantically incompatible;
- body verification fails;
- required body or VM metadata is absent;
- ext row-kind or opcode schemas required by `deps` are unsupported;
- unknown executable opcodes, semantic section kinds, or required semantic row kinds appear.
