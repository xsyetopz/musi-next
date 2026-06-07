# Temporary unknowns discussion index

This file collects current `## Unknowns` entries from specs so they can be resolved in one place before decisions are folded back into the owning spec files.


## Locked decisions

### SEIL definition and design center

- SEIL is lower-level than ASTs: it clarifies ambiguous source constructs and removes redundant source forms.
- SEIL is higher-level than typical compiler IRs: it preserves semantic types and a close source-program relationship when tool metadata is present.
- SEIL compiles every valid Musi program into a small set of core constructs with clean executable semantics.
- SEIL has a syntax-directed type system for analysis, verification, transformation, assembly, disassembly, and execution.
- SEIL+SEAM follows the broad CIL+CLR relationship: SEAM executes verified SEIL, while SEIL avoids CLR/CIL constraints not justified by Musi/SEAM semantics.

### Executable semantics and unknown data

- SEIL core is closed by default. Unknown executable opcodes are rejected unless declared by a supported core ext before operand decoding and verification.
- Unknown required semantic sections are rejected.
- Unknown metadata sections are skippable only when explicitly marked non-semantic/skippable by core.
- Unknown required sections, opcodes, flags, or metadata schemas are rejected when the consuming VM/tool does not support the declaring core ext.
- Binary `asm` carries only current module identity, version, and entry metadata. Runtime, capability, extension, dependency, and import contracts live in `deps` and must be decoded before loaders decide whether later payloads can be decoded, skipped, or rejected.
- SEIL/SEAM do not use executable-semantics dialects. Each behavior is core, library/native, frontend-owned, or unsupported.

### SEIL text shape

- SEIL text is WAT-like typed module text: exactly one `(module ...)` root.
- SEIL text borrows CIL/ILAsm roles for `asm`, `asmref`, versioned references, metadata declarations, and executable bodies, but removes the CLR object-model center.
- SEIL text uses symbols as human-facing references; assemblers resolve symbols to binary table indices. Descriptor-heavy references are not the normal hand-written surface.
- Directive names are chosen for clarity and are not constrained by opcode mnemonic length. Opcode mnemonic parts keep the 2..7 character law.

### Container header, asm, and deps sections

- SEAM binary images keep an exactly 40-byte fixed header as container probe data only.
- The 40-byte header carries magic, container format version, header size, reserved-zero flags, section-directory location, and file size.
- The header must not carry asm identity, dependency contracts, capability set, runtime contract, or ext declarations.
- SEIL uses a WAT-like textual module model and a CIL-inspired assembly/reference plus typed metadata-table model rather than a raw instruction-stream model.
- Section kind `2` is `asm`. A mandatory early asm section carries only current module identity, version, and entry metadata needed before dependent payload decoding.
- SEAM binary image core section families are `names`, `asm`, `deps`, `defs`, `code`, `data`, `meta`, and `tool`.
- Loaders validate the 40-byte header and section directory first, then decode the mandatory core `asm` section and dependency contracts in `deps` before deciding whether remaining sections can be decoded, skipped, or rejected.
- Executable bodies remain compact streams whose operands reference metadata table indices/tokens. Required execution metadata is not optional; tool/debug/source metadata is skippable and non-semantic.
- SEIL should avoid CIL costs that do not fit SEIL/SEAM: no PE/COFF coupling, no implicit runtime-specific verification loopholes, no attributes that secretly alter execution, and no complex binding policy unless a future package spec explicitly requires it.
- Compression, checksum, signature, and archive transport are package/container-layer concerns, not core SEAM binary image concerns.

### GC and GenImmix consequence

- SEIL exposes managed references, layouts, typed stack effects, safepoints, and barrier obligations.
- SEAM may implement managed storage with generational Immix, but Immix lines, blocks, cards, nurseries, and remembered sets are runtime internals, not normal SEIL syntax.
- Musi `fixed` lowers to SEIL metadata/operations that constrain movement or pin storage for a defined lifetime; it does not disable GC globally.

## Musi

### `musi/control-lowering.md`

- Exact SEIL block layout patterns for each control form are not locked.
- Exact generator object representation is not specified.
- Exact cleanup ordering among multiple nested regions needs a dedicated runtime rule table.

### `musi/lowering-to-seil.md`

- Exact lowering algorithms for every Musi expression form are not fully specified here.
- Exact source-map/tool-metadata payloads are not specified.
- Exact import path resolution and module packaging rules remain partially unspecified.

### `musi/runtime-expectations.md`

- Exact Musi package format and module discovery rules are not specified.
- Exact standard native module catalog is not specified.
- Exact user-facing mapping from SEAM failure payloads to Musi diagnostics is not specified.

## SEIL

### `seil/binary-image-format.md`

- Physical row layouts and packing are not specified for every section payload.

### `seil/instructions.md`

- Exact trap taxonomy is not fully specified.
- Exact numeric overflow and floating-point exception behavior is not fully specified.
- Exact pointer-region permission metadata is not fully specified.

### `seil/modules-artifacts.md`

- Exact module-name canonicalization is not fully specified.
- Exact package/archive format for multiple modules is not specified.

### `seil/operands-stack-effects.md`

- Exact compatibility edge schema is not fully specified.

### `seil/types-metadata.md`

- Exact binary encodings for all type and metadata table payloads are not defined.
- Exact ABI descriptor grammar is not fully specified.

### `seil/verification.md`

- Exact compatibility edge schemas are not fully specified.
- Exact diagnostic codes/messages for verifier failures are not specified here.

## SEAM

### `seam/dynamic-capabilities.md`

- Exact capability table schema is not specified.
- Exact dynamic argument-pack representation is not specified.
- Exact keyed-storage key/value type constraints are not specified.

### `seam/failures-and-limits.md`

- Exact reason-code enum is not specified.
- Exact host embedding representation of outcomes is not specified.
- Exact mapping from numeric failures to trap kinds is not specified.

### `seam/frames-control.md`

- Exact in-memory frame layout is not specified.
- Exact handler matching table format is not specified.
- Exact cancellation API for suspended computations is not specified.

### `seam/memory-gc.md`

- Exact object header layout is not specified.
- Exact GC algorithm parameters are not specified.
- Exact write-barrier/read-barrier rules are not specified.
- Exact finalization/destructor semantics are not specified.

### `seam/runtime.md`

- Exact frame object layout is not specified.
- Exact module initialization ordering beyond load/verify/link/init is not fully specified.
- Exact host embedding API is not specified.
