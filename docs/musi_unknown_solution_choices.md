# Musi Unknown Solution Choices

Purpose: decision sheet for current Musi/SEAM spec gaps. Checked choices are locked by USER grill answers. Remaining bullets are detail work, not product-direction choices.

Rules:

- Checked item = chosen direction.
- Unchecked item = rejected alternative or still-detail subchoice.
- Fold checked choices into owning specs; keep only exact schema/API/message contents in `specs/UNKNOWNS.md`.

## Locked choices

### Lowering and source metadata

- [x] **Control lowering uses region/edge metadata first.** `when`, `while`, `match`, `defer`, `yield`, `leave`, `cycle`, cancellation, and handlers lower through protected regions, edge reasons, cleanup/yield metadata, and verifier-checked stack shapes.
- [x] **Every accepted grammar form gets normative lowering recipe + pass/fail fixtures.** Fixtures are conformance evidence, not examples.
- [x] **Tool metadata uses typed non-semantic registry.** Source maps, decompiler hints, debugger/probe data, docs/comments, import/export grouping, pattern/operator/datum spelling, and tool-owned rows use stable typed schemas. Execution cannot depend on them.

### Packages, modules, and native catalog

- [x] **Source package canonical form is loose graph:** `musi.json` + `.ms` + `.seam` files.
- [x] **Container/archive is split out behind distribution gate.** Container spec is mandatory before bundled distribution, signed packages, resource bundles, plugin archives, or streaming package loading. Future container must preserve loose-graph behavior.
- [x] **Missing optional standard module import is load/link failure.** No ambient fallback and no lazy module query by default.
- [x] **`known import` is stricter.** Optional provider must be present and deterministic-known-capable; otherwise compile-time diagnostic.
- [x] **Required native modules:** `musi:core`, `musi:rt`, `musi:ffi`, `musi:text`.
- [x] **Optional provider/capability-gated modules:** `musi:host`, `musi:fs`, `musi:process`, `musi:time`, `musi:random`, `musi:encoding`, `musi:reflect`, `musi:probe`, `musi:package`, `musi:schema`, `musi:bytecode`, `musi:test`.
- [x] **Package/module initialization cycles reject.** Dependency/init cycles are load/link diagnostics; no lazy cycle breaking and no half-initialized SCC ordering.

### Diagnostics and failures

- [x] **Failure diagnostics are boundary-aware.** Diagnostics include phase, module/proc location, source span when present, and host/resource/capability/ABI boundary context.
- [x] **SEAM bytecode text/disassembly diagnostics include module context.** Stable codes report expected/found/offending token or form plus asm/proc/body path where available.
- [x] **Verifier diagnostics are subject-first full diagnostics.** Stable codes/kinds plus subject-first headlines, labels, and real hints.
- [x] **Reason codes use phase tuple.** Shape is `phase + subsystem + reason + source relation`.
- [x] **Numeric edge behavior visible by opcode/schema.** Ordinary ops use locked CPU-like behavior; checked/trapping/failing behavior appears in explicit opcode/schema such as `.chk`.

### Binary schemas, compatibility, ABI

- [x] **`.seam` row/type/metadata schemas use one declarative generated schema source.** It drives docs, encoder/decoder, and conformance tests.
- [x] **Type/verifier compatibility uses generated declarative relation.** Same source drives verifier rules, docs, and tests for type, block, call, and control edges.
- [x] **ABI descriptor grammar is host-ABI capable from start.** Includes C ABI plus handles, callbacks through exported callable handles, resources, async/yield/resumable interaction, cancellation, failure outcomes, and representable memory access metadata.

### Dynamic protocols and capability graph

- [x] **Dynamic UALO argpacks are typed records.** Preserve positional order, named args, defaults/schema validation, expected signature, result contract, and failure contract.
- [x] **Keyed storage uses typed key schemas.** Declared key domains may be symbol/string/int/enum/compound with value constraints and capability requirements. Arbitrary `Any` keys still rejected.
- [x] **Capability/resource graph uses typed nodes and typed edges.** Nodes have opaque stable non-forgeable identity. Edges carry authority relation. Nodes include provider, module, resource, capability, and handle metadata. Inspection requires authority.

### Host API and outcomes

- [x] **Host-visible outcomes use canonical tagged result structs.** Host bindings may adapt shape but must preserve exact tags and payload fields.
- [x] **Baseline host embedding API is capability-aware.** Includes load/link/call/resume/cancel/close/outcome plus modules, resources, and capability graph. Reflective frame/failure/package inspection stays behind `musi:probe`/tooling APIs.

### Text/disassembly format

- [x] **Readable canonical formatter only.** Stable indentation, blank-line rules, one instruction per line. No compact canonical mode.

### Memory and GC

- [x] **Object layout uses compact headers plus side tables.** Object headers stay small; type/layout/GC metadata lives in side tables.
- [x] **GC parameters are manifest/host policy through `seamArguments`.** Runtime policy inputs affect performance/resource failures, not language semantics.
- [x] **`seamArguments` use JVM-style flag shape and terminology, SEAM-specific where needed:** `-Xms`, `-Xmx`, `-Xss`, `-Xmn`, `-XX:NewRatio`, `-XX:SurvivorRatio`, `-XX:MaxGCPauseMillis`, `-XX:GCTimeRatio`, `-XX:+/-UseIncrementalGC`, `-XX:+/-UseImmixDefrag`, `-XX:ImmixBlockSize`, `-XX:ImmixLineSize`, `-XX:+StressGC`, `-XX:FuelMax`.
- [x] **Barrier rules are layout-driven.** Barrier obligations derive from ref maps, layout metadata, opcode storage effects, and active collector policy. Source never calls barriers.
- [x] **No core finalizers.** Ordinary managed values have no finalization/destructor semantics. Resource cleanup is explicit through `defer`, cleanup regions, handles, and host APIs. No resurrection model.

## Remaining detail gaps

- [ ] Exact lowering recipe contents and fixture corpus.
- [ ] Exact typed tool metadata schemas.
- [ ] Exact native module API contents.
- [ ] Exact boundary-aware diagnostic code/message catalog.
- [ ] Exact generated schema source format and generated artifact ownership.
- [ ] Exact compatibility relation entries.
- [ ] Exact host ABI descriptor fields and validation rules.
- [ ] Exact typed argpack/key schema encodings.
- [ ] Exact capability graph node/edge schemas and authority predicates.
- [ ] Exact host API function signatures and result struct field layout.
- [ ] Exact readable formatter whitespace rules.
- [ ] Exact object header fields, side-table rows, GC defaults/ranges, barrier decision table.
- [ ] Exact package/container spec once distribution gate is entered.
