# Temporary unknowns discussion index

Collect current `## Unknowns` from specs. Resolve here, then fold into owning specs.

USER choice files: `docs/musi_full_spec_solution_options.md` defines broad language directions. `docs/musi_unknown_solution_choices.md` is the per-gap checkbox source of truth. Until USER checks unresolved options, listed gaps remain unresolved.

## Locked decisions

### Philosophy and design center

- Musi prioritizes simplicity, explicivity/WYSIWYG, and long-term maintainability.
- More code is not worse code when behavior becomes visible.
- Parser/runtime complexity is not the rejection gate. Actual blockers are missing ability, forced workaround, weak bidirectional FFI, weak embedding, weak extension, weak self-hosting, or hidden behavior.
- Musi targets self-hostability: compiler/VM pieces should be writable in Musi.

### SEAM bytecode definition and artifact

- SEAM bytecode lower than ASTs: clarifies source ambiguity, removes redundant forms.
- SEAM bytecode higher than disposable IR: preserves semantic types + source relation when tool metadata exists.
- Valid Musi lowers to small SEAM bytecode core with clean executable semantics.
- SEAM bytecode has syntax-directed typing for analysis, verification, transform, assembly, disassembly, execution.
- SEAM bytecode + SEAM broadly CIL+CLR-like in role, without CLR/CIL costs not justified by Musi/SEAM.
- `.seam` is the compiled SEAM bytecode image, analogous to Erlang `.beam`.
- Public pipeline is `.ms -> .seam`. No second bytecode layer or artifact exists.
- SEAM bytecode text/disassembly is a readable tool format, not the package artifact extension.

### Executable semantics and unknown data

- SEAM bytecode core closed by default. Unknown executable opcode rejected unless supported core ext declares schema before operand decoding/verification.
- Unknown required semantic sections rejected.
- Unknown metadata sections skippable only when core marks non-semantic/skippable.
- Unsupported required sections/opcodes/flags/metadata schemas rejected.
- Binary `asm` carries only current module id/version/entry. Runtime/cap/ext/dependency/import contracts live in `deps` and decode before dependent payload decisions.
- No executable-semantics dialects. Behavior is core, library/native, frontend-owned, or unsupported.

### SEAM bytecode image shape

- `.seam` image keeps exactly 40-byte probe header only.
- Header carries magic, format version, header size, reserved-zero flags, section-directory location, file size.
- Header must not carry asm id, deps, caps, runtime contract, or ext declarations.
- Core families: `names`, `asm`, `deps`, `defs`, `code`, `data`, `meta`, `tool`.
- Section payload = row-kind directory, row offset table, packed row bytes.
- Row-kind entry: kind id, count, offset-table range, payload range, schema id/core tag, required/skippable policy.
- Rows schema-packed; no field names encoded.
- Loader validates header + directory, decodes `asm` + `deps`, then decides decode/skip/reject for rest.
- Bodies stay compact streams; operands reference metadata indices/tokens. Required execution metadata is mandatory; tool/debug/source metadata skippable non-semantic.
- Compression/checksum/signature/archive transport = package/container layer, not core `.seam` image.
- Section-family ids beyond core use explicit registry/dependency declarations rather than hardcoded vendor ranges.

### SEAM bytecode text/disassembly shape

- SEAM bytecode text/disassembly = WAT-like typed module text: exactly one `(module ...)` root.
- Borrows CIL/ILAsm roles for `asm`, `asmref`, versioned refs, metadata, bodies; removes CLR object-model center.
- Text uses symbols for humans; assembler resolves to binary table indices. Descriptor-heavy refs not normal handwritten surface.
- Directives use clarity; opcode mnemonic parts keep 2..7 char law.

### Packages, imports, exports

- Source package canonical format: `musi.json` + `.ms` files.
- Import syntax uses ESM-like string paths: `import "path/to/file"`.
- Bare specifiers/package names resolve through manifest `imports`/`dependencies`.
- `musi:` is reserved like `node:`/`bun:`; user packages/import maps cannot shadow it.
- Source `export` controls module surface; manifest `exports` controls package public surface.
- Extensionless imports are policy/lint controlled. If enabled, `./foo` resolves to `./foo.ms`; `./foo/index.ms` is fallback only if no direct file exists.
- Host-provided modules participate in package graph as explicit nodes with provider/capability metadata.
- Module init order: resolve graph, verify/link all, initialize dependencies before dependents, manifest declaration order tie-breaks.

### FFI, capabilities, dynamic behavior

- `@extern` is metadata/attribute, not keyword.
- `@extern let ...;` imports external implementation.
- `@extern export let ... := ...;` exports Musi implementation outward.
- `@extern let ... := ...;` without `export` is diagnostic.
- Callbacks from host into Musi are exported Musi functions passed by symbol/handle through host embedding API; no callback syntax.
- Native resources crossing FFI use opaque handles by default; typed `Access[T]`/`Address` only when ABI metadata declares representable memory access.
- Native calls are failure-capable unless metadata proves otherwise.
- Capabilities are first-class non-forgeable runtime values plus metadata requirements.
- Capability requirements appear in SEAM bytecode/module metadata, not new Musi syntax for now.
- Dynamic calls use explicit callee, UALO-shaped arg pack, expected signature, result contract, and structured failure.
- Keyed storage limited to declared key domains; arbitrary `Any` keys do not become valid.
- Host resource handles are values protected by capabilities; identity is separate from authority.
- `Address` is non-authoritative by itself; load/store/permission comes from `Region`/`Access`/capability metadata.

### GC and GenImmix consequence

- SEAM bytecode exposes managed refs, layouts, typed stack effects, safepoints, barrier obligations.
- SEAM may use generational Immix; lines/blocks/cards/nurseries/remembered sets are runtime internals, not SEAM bytecode syntax.
- Musi `fixed` lowers to metadata/ops constraining movement/pinning for lifetime; not GC-off.
- Musi low-level memory: `Address`, `Region`, `Access[T]`, `Access[mut T]`; no source `Ptr`/`Pointer`.
- `Address` not GC root and cannot load/store. `Access` lowers to pointer/ref ops plus region/permission/layout/cap metadata.
- `MutAccess[T]` and `OpaqueAccess[T]` DRY aliases only. `unmanaged` keyword/type qualifier marks storage/representation outside managed tracing/movement/reclamation unless core metadata says otherwise.

### Runtime control and outcomes

- Host-visible invocation outcomes are tagged: `returned`, `yielded`, `failed`, `trapped`, `cancelled`.
- Host exceptions do not cross the SEAM boundary as host exceptions.
- Suspended computations are opaque resumable handles with resume, cancel, close/drop, status, and outcome.
- Cancellation is cooperative at safepoints/yield points first; forced close only for teardown.
- Cancellation runs pending defers before `cancelled` unless cleanup traps/fails.
- Nested defer cleanup order is lexical LIFO for normal return, `leave`, `cycle`, cancellation, and close. Trap/abort remains separately specified.
- Handler matching primary model: protected region + exit/failure reason, not raw instruction ranges.
- Frame layout VM-private by default with optional authorized inspection API.

## Musi

### `musi/control-lowering.md`

- Exact SEAM bytecode block layout patterns for each control form are not locked.
- Exact generator/resumable internal object representation is not specified.

### `musi/lowering-to-seam-bytecode.md`

- Exact lowering algorithms for every Musi expression form are not fully specified here.
- Exact source-map/tool-metadata payloads are not specified.
- Exact package/archive/container format beyond `.seam` images is not specified.

### `musi/runtime-expectations.md`

- Exact standard native module catalog is not specified.
- Exact user-facing mapping from SEAM failure payloads to Musi diagnostics is not specified.

## SEAM bytecode

### `seam-bytecode/instructions.md`

- Exact trap taxonomy is not fully specified.
- Exact numeric overflow and floating-point exception behavior is not fully specified.
- Exact access/region permission metadata is not fully specified.

### `seam-bytecode/modules-artifacts.md`

- Exact package/archive format for multiple modules is not specified.

### `seam-bytecode/operands-stack-effects.md`

- Exact compatibility edge schema is not fully specified.

### `seam-bytecode/types-metadata.md`

- Exact binary encodings for all type and metadata table payloads are not defined.
- Exact ABI descriptor grammar is not fully specified.

### `seam-bytecode/verification.md`

- Exact compatibility edge schemas are not fully specified.
- Exact diagnostic codes/messages for verifier failures are not specified here.

### `seam-bytecode/binary-image-format.md`

- Section-family ids beyond core use explicit registry/dependency declarations; exact registry contents beyond core are not specified.
- Exact per-row binary schemas are not fully specified.
- Exact package/container transport beyond core `.seam` image is not specified.

### `seam-bytecode/text-format.md`

- Exact canonical whitespace policy is not specified.
- Exact diagnostic wording for text parse/assemble failures is not specified.
- Exact `tool` metadata schemas are not specified.

## SEAM

### `seam/dynamic-capabilities.md`

- Exact capability table schema is not specified.
- Exact binary/runtime representation of UALO-shaped dynamic argument packs is not specified.
- Exact keyed-storage domain/value schema encoding is not specified.

### `seam/failures-and-limits.md`

- Exact reason-code enum is not specified.
- Exact numeric-failure to trap-kind map is not specified.

### `seam/frames-control.md`

- Exact in-memory frame layout is not specified.
- Exact binary handler matching table format is not specified.
- Exact host embedding function signatures for resumable handles are not specified.

### `seam/memory-gc.md`

- Exact object header layout is not specified.
- Exact GC algorithm parameters are not specified.
- Exact write-barrier/read-barrier rules are not specified.
- Exact finalization/destructor semantics are not specified.

### `seam/runtime.md`

- Exact frame object layout is not specified.
- Exact host embedding API shape beyond tagged outcomes and opaque resumable handles is not specified.
