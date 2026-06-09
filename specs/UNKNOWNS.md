# Temporary unknowns discussion index

Collect current `## Unknowns` from specs. Resolve here, then fold into owning specs.

USER choice file: `docs/musi_full_spec_solution_selection.md` is the checkbox source of truth. `docs/musi_full_spec_solution_options.md` defines A/B/C language directions and exact current-unknown coverage. Until USER checks one option, listed gaps remain unresolved.

## Locked decisions

### SEIL definition and design center

- SEIL lower than ASTs: clarifies source ambiguity, removes redundant forms.
- SEIL higher than disposable IR: preserves semantic types + source relation when tool metadata exists.
- Valid Musi lowers to small SEIL core with clean executable semantics.
- SEIL has syntax-directed typing for analysis, verification, transform, assembly, disassembly, execution.
- SEIL+SEAM broadly CIL+CLR-like, without CLR/CIL costs not justified by Musi/SEAM.

### Executable semantics and unknown data

- SEIL core closed by default. Unknown executable opcode rejected unless supported core ext declares schema before operand decoding/verification.
- Unknown required semantic sections rejected.
- Unknown metadata sections skippable only when core marks non-semantic/skippable.
- Unsupported required sections/opcodes/flags/metadata schemas rejected.
- Binary `asm` carries only current module id/version/entry. Runtime/cap/ext/dependency/import contracts live in `deps` and decode before dependent payload decisions.
- No executable-semantics dialects. Behavior is core, library/native, frontend-owned, or unsupported.

### SEIL text shape

- SEIL text = WAT-like typed module text: exactly one `(module ...)` root.
- Borrows CIL/ILAsm roles for `asm`, `asmref`, versioned refs, metadata, bodies; removes CLR object-model center.
- Text uses symbols for humans; assembler resolves to binary table indices. Descriptor-heavy refs not normal handwritten surface.
- Directives use clarity; opcode mnemonic parts keep 2..7 char law.

### Container header, asm, and deps sections

- Binary images keep exactly 40-byte probe header only.
- Header carries magic, format version, header size, reserved-zero flags, section-directory location, file size.
- Header must not carry asm id, deps, caps, runtime contract, or ext declarations.
- SEIL uses WAT-like module text + CIL-inspired asm/ref + typed metadata tables, not raw instruction stream.
- Section kind `2` = `asm`; mandatory early asm carries only module id/version/entry needed before dependent payload decode.
- Core families: `names`, `asm`, `deps`, `defs`, `code`, `data`, `meta`, `tool`.
- Section payload = row-kind directory, row offset table, packed row bytes.
- Row-kind entry: kind id, count, offset-table range, payload range, schema id/core tag, required/skippable policy.
- Rows schema-packed; no field names encoded.
- Loader validates header + directory, decodes `asm` + `deps`, then decides decode/skip/reject for rest.
- Bodies stay compact streams; operands reference metadata indices/tokens. Required execution metadata is mandatory; tool/debug/source metadata skippable non-semantic.
- Avoid CIL costs: no PE/COFF coupling, runtime-specific loopholes, attributes secretly changing execution, complex binding policy unless package spec requires it.
- Compression/checksum/signature/archive transport = package/container layer, not core image.

### GC and GenImmix consequence

- SEIL exposes managed refs, layouts, typed stack effects, safepoints, barrier obligations.
- SEAM may use generational Immix; lines/blocks/cards/nurseries/remembered sets are runtime internals, not SEIL syntax.
- Musi `fixed` lowers to metadata/ops constraining movement/pinning for lifetime; not GC-off.
- Musi low-level memory: `Address`, `Region`, `Access[T]`, `Access[mut T]`; no source `Ptr`/`Pointer`.
- `Address` not GC root and cannot load/store. `Access` lowers to pointer/ref ops plus region/permission/layout/cap metadata.
- `MutAccess[T]` and `OpaqueAccess[T]` DRY aliases only. `unmanaged` keyword/type qualifier marks storage/representation outside managed tracing/movement/reclamation unless core metadata says otherwise.

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

### `seil/instructions.md`

- Exact trap taxonomy is not fully specified.
- Exact numeric overflow and floating-point exception behavior is not fully specified.
- Exact access/region permission metadata is not fully specified.

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
