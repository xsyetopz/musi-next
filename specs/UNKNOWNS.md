# Temporary unknowns discussion index

Collect current detail gaps from specs. Product-direction choices are now locked in `docs/musi_unknown_solution_choices.md`; this file tracks exact schemas, APIs, catalogs, and fixture work still needing spec detail.

## Locked decisions

### Philosophy and design center

- Musi prioritizes simplicity, explicivity/WYSIWYG, and long-term maintainability.
- More code is not worse code when behavior becomes visible.
- Parser/runtime complexity is not rejection gate. Actual blockers are missing ability, forced workaround, weak bidirectional FFI, weak embedding, weak extension, weak self-hosting, or hidden behavior.
- Musi targets self-hostability: compiler/VM pieces should be writable in Musi.

### SEAM bytecode and artifacts

- `.seam` is the compiled SEAM bytecode image, analogous to Erlang `.beam`.
- Public pipeline is `.ms -> .seam`. No second bytecode layer or artifact exists.
- SEAM bytecode text/disassembly is a readable tool format, not the package artifact extension.
- `.seam` image keeps exactly 40-byte probe header only.
- Header carries magic, format version, header size, reserved-zero flags, section-directory location, file size.
- Core families: `names`, `asm`, `deps`, `defs`, `code`, `data`, `meta`, `tool`.
- Section payload = row-kind directory, row offset table, packed row bytes.
- Extension rows/opcodes require explicit registry/dependency declarations.
- Compression/checksum/signature/archive transport = package/container layer, not core `.seam` image.

### Packages, imports, and native modules

- Source package canonical format is loose graph: `musi.json` + `.ms` + `.seam` files.
- Import syntax uses ESM-like string paths: `import "path/to/file"`.
- Bare specifiers/package names resolve through manifest `imports`/`dependencies`.
- `musi:` is reserved like `node:`/`bun:`; user packages/import maps cannot shadow it.
- Source `export` controls module surface; manifest `exports` controls package public surface.
- Extensionless imports are policy/lint controlled. If enabled, `./foo` resolves to `./foo.ms`; `./foo/index.ms` is fallback only if no direct file exists.
- Host-provided modules participate in package graph as explicit nodes with provider/capability metadata.
- Required native modules: `musi:core`, `musi:rt`, `musi:ffi`, `musi:text`.
- Optional provider/capability-gated modules: `musi:host`, `musi:fs`, `musi:process`, `musi:time`, `musi:random`, `musi:encoding`, `musi:reflect`, `musi:probe`, `musi:package`, `musi:schema`, `musi:bytecode`, `musi:test`.
- Importing absent optional standard module is load/link missing-provider diagnostic.
- `known import` of optional module requires deterministic known-capable provider; missing/nondeterministic provider is compile-time diagnostic.
- Container/archive spec is mandatory before bundled distribution, signed packages, resource bundles, plugin archives, or streaming package loading. Future container must preserve loose package graph behavior.
- Module initialization order: resolve graph, verify/link all, initialize dependencies before dependents, manifest declaration order tie-breaks.
- Package/module dependency/init cycles reject with load/link diagnostic.

### Lowering and metadata

- Control lowering uses region/edge metadata first: protected regions, edge reasons, cleanup/yield metadata, verifier-checked stack shapes.
- Every accepted Musi grammar form gets normative lowering recipe plus pass/fail fixtures.
- Tool metadata uses typed non-semantic registry. Execution cannot depend on it.
- `.seam` row/type/metadata schemas use one declarative generated schema source.
- Type/verifier compatibility uses generated declarative relation.

### FFI, capabilities, dynamic behavior

- `@extern` is metadata/attribute, not keyword.
- `@extern let ...;` imports external implementation.
- `@extern export let ... := ...;` exports Musi implementation outward.
- `@extern let ... := ...;` without `export` is diagnostic.
- ABI descriptor grammar is host-ABI capable from start: C ABI, handles, callbacks through exported callable handles, resources, async/yield/resumable interaction, cancellation, failure outcomes, representable memory access metadata.
- Native resources crossing FFI use opaque handles by default; typed `Access[T]`/`Address` only when ABI metadata declares representable memory access.
- Native calls are failure-capable unless metadata proves otherwise.
- Capability/resource graph uses typed non-forgeable nodes and typed authority edges. Nodes include provider, module, resource, capability, and handle metadata. Inspection requires authority.
- Dynamic UALO argpacks are typed records.
- Keyed storage uses typed key schemas. Arbitrary `Any` keys do not become valid.
- `Address` is non-authoritative by itself; load/store/permission comes from `Region`/`Access`/capability metadata.

### Runtime control and diagnostics

- Host-visible invocation outcomes are tagged: `returned`, `yielded`, `failed`, `trapped`, `cancelled`.
- Host outcomes use canonical tagged result structs. Host bindings may adapt shape but preserve tags/payload fields.
- Host exceptions do not cross the SEAM boundary as host exceptions.
- Suspended computations are opaque resumable handles with resume, cancel, close/drop, status, and outcome.
- Cancellation is cooperative at safepoints/yield points first; forced close only for teardown.
- Cancellation runs pending defers before `cancelled` unless cleanup traps/fails.
- Nested defer cleanup order is lexical LIFO for normal return, `leave`, `cycle`, cancellation, and close. Trap/abort remains separately specified.
- Handler matching primary model: protected region + exit/failure reason, not raw instruction ranges.
- Frame layout VM-private by default with optional authorized inspection API.
- Baseline host embedding API is capability-aware: load/link/call/resume/cancel/close/outcome plus modules, resources, capability graph.
- Failure diagnostics are boundary-aware.
- SEAM bytecode text/disassembly diagnostics include stable code plus asm/proc/body context.
- Verifier diagnostics use stable code/kind plus subject-first full message, labels, real hints.
- Reason codes use `phase + subsystem + reason + source relation` tuple.
- Numeric edge behavior is visible by opcode/schema; no hidden target/type metadata mode.

### Memory and GC

- SEAM may use generational Immix; lines/blocks/cards/nurseries/remembered sets are runtime internals, not SEAM bytecode syntax.
- Object layout uses compact headers plus side tables.
- GC parameters are manifest/host policy through `seamArguments`, JVM-style in flag shape and terminology.
- Locked `seamArguments`: `-Xms`, `-Xmx`, `-Xss`, `-Xmn`, `-XX:NewRatio`, `-XX:SurvivorRatio`, `-XX:MaxGCPauseMillis`, `-XX:GCTimeRatio`, `-XX:+UseIncrementalGC`, `-XX:-UseIncrementalGC`, `-XX:+UseImmixDefrag`, `-XX:-UseImmixDefrag`, `-XX:ImmixBlockSize`, `-XX:ImmixLineSize`, `-XX:+StressGC`, `-XX:FuelMax`.
- Barrier rules are layout-driven from ref maps, layout metadata, opcode storage effects, and active collector policy. Source never calls barriers.
- No core finalizers. Ordinary managed values have no finalization/destructor semantics and no resurrection model. Resource cleanup is explicit through `defer`, cleanup regions, handles, and host APIs.

## Detail gaps remaining

### Musi

- Exact lowering recipe contents for every Musi expression form are not fully specified.
- Exact lowering pass/fail fixture corpus is not specified.
- Exact typed tool metadata schemas for source maps/decompiler/probe/docs/comments are not specified.
- Exact native module API contents for `musi:*` modules are not specified.
- Exact boundary-aware diagnostic code/message catalog is not specified.
- Exact cyclic-init diagnostic wording/code is not specified.

### SEAM bytecode

- Exact generated schema source file format is not specified.
- Exact generated schema ownership for docs/decoder/encoder/tests is not specified.
- Exact row/type/metadata schema contents are not specified.
- Exact compatibility relation entries are not specified.
- Exact host ABI descriptor fields and validation rules are not specified.
- Exact text/disassembly readable whitespace formatter rules are not specified.
- Exact text parse/assemble diagnostic code/message catalog is not specified.
- Exact verifier diagnostic code/message catalog is not specified.

### SEAM runtime

- Exact typed argpack record fields and binary/runtime encoding are not specified.
- Exact typed key schema encoding is not specified.
- Exact capability graph node/edge schemas and authority predicates are not specified.
- Exact host API function signatures are not specified.
- Exact canonical result struct field layout is not specified.
- Exact object header fields and side-table row schemas are not specified.
- Exact `seamArguments` units, parsing, defaults, bounds, and conflict rules are not specified.
- Exact layout-driven barrier decision table is not specified.
- Exact package/container spec is not specified until distribution gate is entered.
