# Musi Unknown Solution Choices

Purpose: choose exact solutions for current spec gaps. Checked boxes are already locked by USER grill answers. Unchecked boxes still need USER selection before folding into owning specs.

Rules:

- One checked option per gap.
- Examples marked `current` use locked/current grammar or current SEAM bytecode text/disassembly shape.
- Examples marked `proposal` are not grammar until separately locked.
- After selection, fold rule into owning spec and remove matching unknown.

## Musi lowering and source behavior

### M-CL-1 — Control-form block layout

Current state: control forms lower to SEAM bytecode blocks, but exact block recipes are not locked.

- [ ] **A. Source-form recipe table** — one lowering recipe per `when`, `while`, `match`, `defer`, `yield`. Pro: easiest to test. Con: less room for alternate equivalent layouts.
  - Shape: `when` lowers to condition block, then block, else block, join block.
- [ ] **B. Region/edge metadata first** — recipes defined by protected regions, edge reasons, cleanup/yield metadata. Pro: better for cleanup/suspension. Con: heavier verifier tables.
  - Shape: `leave` targets region exit metadata, not raw instruction labels.
- [ ] **C. Decompile-preserving layout** — recipes also preserve source-control identity in tool metadata. Pro: best roundtrip/debug. Con: more tool metadata.
  - Shape: text/disassembly keeps canonical blocks plus source-shape `tool` rows.

### M-CL-2 — Generator/resumable representation

Current state: `yield` is locked; host-visible suspended computations are opaque handles.

- [x] **A. Opaque resumable handle** — host sees resume/cancel/close/drop/status/outcome. Pro: WYSIWYG host lifecycle. Con: internal object layout still unspecified.
  - Shape: handle outcome tags are `returned`, `yielded`, `failed`, `trapped`, `cancelled`.
- [ ] **B. Public generator object layout** — expose generator frame/object schema. Pro: inspectable. Con: leaks VM layout too early.
- [ ] **C. Library-only generator wrapper** — runtime handle hidden behind standard library type. Pro: small surface. Con: weaker embedding/debug story.

### M-LS-1 — Lowering algorithm for every expression

Current state: direct Musi→SEAM bytecode is locked; exact recipes incomplete.

- [ ] **A. Grammar-keyed lowering table** — one table row per grammar form. Pro: direct and testable. Con: large spec table.
- [ ] **B. Semantic-family lowering** — group forms by binding/control/data/call/access. Pro: less repetition. Con: individual forms can hide edge cases.
- [ ] **C. Recipe + fixture lock** — normative recipes plus one pass/fail fixture per form. Pro: best conformance. Con: most maintenance.

### M-LS-2 — Source-map/tool metadata payloads

Current state: tool metadata optional and non-semantic; exact payload schemas unknown.

- [ ] **A. Core spans** — source file, byte range, symbol spelling. Pro: small. Con: weaker decompile/debug.
- [ ] **B. Source-shape metadata** — spans plus import/export grouping, pattern/operator/datum spelling. Pro: near-identical decompile. Con: larger tool rows.
- [ ] **C. Typed tool schema registry** — stable schemas for debugger/decompiler/docs. Pro: best tools. Con: largest conformance burden.

### M-LS-3 — Import path/package resolution

Current state: locked.

- [x] **A. ESM-like manifest resolution** — `import "path/to/file"`, manifest `imports`/`dependencies`, reserved `musi:`. Pro: explicit and familiar. Con: manifest policy must be precise.
  - Current:
    ```musi
    let local := import "./local.ms";
    let defaulted := import "./tool";
    let core := import "musi:core";
    ```
- [x] **B. Extensionless policy** — linter/compiler toggle; if enabled, `./foo` → `./foo.ms`, then `./foo/index.ms` as fallback. Pro: one policy gate. Con: resolver must report exact attempted paths.
- [x] **C. Package graph nodes** — host-provided modules are explicit provider/capability nodes. Pro: no ambient host globals. Con: host embedding metadata required.

### M-RT-1 — Package format/module discovery

Current state: source package shape locked; archive/container remains separate.

- [x] **A. Source package = `musi.json` + `.ms`** — manifest owns entry/imports/exports/deps. Pro: simple source layout. Con: archive still separate.
- [x] **B. `.seam` build/cache/distribution artifact** — compiled bytecode image analogous to `.beam`. Pro: direct `.ms -> .seam` story. Con: package bundling still external.
- [ ] **C. Standard archive/container** — bundle manifest, `.seam` images, resources, checksums/signatures. Pro: distribution-ready. Con: new container spec.

### M-RT-2 — Standard native module catalog

Current state: native/compiler modules exist through `musi:`; exact catalog unknown.

- [ ] **A. Self-host essentials** — `musi:core`, `musi:rt`, `musi:ffi`, diagnostics. Pro: small. Con: fewer batteries.
- [ ] **B. Host scripting set** — filesystem/process/time/random/text/encoding behind capabilities. Pro: useful embedding. Con: broader security/cap policy.
- [ ] **C. Tooling set** — reflect/debug/package/schema/SEAM bytecode modules behind capabilities. Pro: self-hosted tooling. Con: larger standard surface.

### M-RT-3 — SEAM failure to Musi diagnostic mapping

Current state: phase split exists; exact mapping unknown.

- [ ] **A. Stable reason→diagnostic table** — one code map for load/verify/link/init/execute. Pro: deterministic. Con: less context.
- [ ] **B. Boundary-aware diagnostics** — include host symbol/resource/capability/ABI context. Pro: better FFI errors. Con: bigger payloads.
- [ ] **C. Typed failure object** — authorized tools can inspect full failure structure. Pro: best debuggability. Con: capability/API design needed.

## SEAM bytecode image, text, and verifier

### S-BI-1 — Section-family ids beyond core

Current state: extension identity policy locked.

- [x] **A. Declared registry/deps** — extension rows/opcodes require explicit dependency/registry declaration. Pro: no magic vendor ranges. Con: registry process needed.
- [ ] **B. Hard vendor ranges** — numeric ranges imply ownership. Pro: simple allocation. Con: hidden policy, collision risk.

### S-BI-2 — Per-row binary schemas

Current state: row-kind directory + offset table + packed bytes locked; exact row payload schemas unknown.

- [ ] **A. Core schema tables** — each core row kind gets exact field order/types. Pro: compact and clear. Con: many tables.
- [ ] **B. Schema-id registry** — row kind references versioned schema ids. Pro: extension-friendly. Con: registry burden.
- [ ] **C. Generated schema source** — one schema file generates docs/decoder/tests. Pro: avoids drift. Con: toolchain required.

### S-BI-3 — Package/container transport

Current state: transport is outside core `.seam` image.

- [x] **A. Outside core image** — compression/checksum/signature/archive/resources live in package/container. Pro: keeps `.seam` focused. Con: second spec needed for distribution.
- [ ] **B. Standard bundle** — manifest + images + resources + signatures. Pro: ready for apps/plugins. Con: more format surface.
- [ ] **C. Streaming/indexed container** — chunked reads, checksums, indexes. Pro: low-memory loading. Con: most format complexity.

### S-TX-1 — Text/disassembly canonical whitespace

Current state: one module root and one instruction per line locked; exact whitespace unknown.

- [ ] **A. Readable canonical formatter** — fixed indentation and blank-line rules. Pro: stable diffs. Con: less compact.
- [ ] **B. Compact canonical mode** — no optional spaces beyond parse needs. Pro: smaller fixtures. Con: worse review.
- [ ] **C. Two modes** — readable canonical + compact transport. Pro: flexible. Con: two formatter targets.

### S-TX-2 — Text parse/assemble diagnostics

Current state: subject-first diagnostics required; exact catalog unknown.

- [ ] **A. Stable parse/assemble codes** — expected/found/offending token/form. Pro: enough for tests. Con: limited context.
- [ ] **B. Add module/package context** — include asm/proc/body path. Pro: better real errors. Con: more renderer data.
- [ ] **C. Structured diagnostic payloads** — typed fields for IDE/tools. Pro: best tooling. Con: bigger API.

### S-IN-1 — Trap taxonomy

Current state: traps separated from structured failures; exact taxonomy unknown.

- [x] **A. Separate traps from failures** — traps = VM/runtime invariant violations; failures = explicit `Expect`/host/operation outcomes. Pro: clear host boundary. Con: exact enum still needed.
- [ ] **B. Small closed trap enum** — type/bounds/memory/numeric/dynamic/cap/runtime. Pro: simple. Con: may underfit host/FFI.
- [ ] **C. Typed reason catalog** — trap/failure reasons with phase/severity/retryability. Pro: best diagnostics. Con: larger catalog.

### S-IN-2 — Numeric overflow and float edge behavior

Current state: CPU-like remainder locked; exact overflow/float traps unknown.

- [ ] **A. Wrapping default + checked ops** — normal integer ops wrap, `.chk` traps/fails. Pro: CPU-like. Con: easy accidental wrap.
- [ ] **B. Trap-on-overflow default** — normal overflow traps, explicit wrapping ops later. Pro: safer. Con: more checks.
- [ ] **C. Type/metadata-selected mode** — numeric metadata selects behavior. Pro: flexible. Con: risks hidden behavior.

### S-IN-3 — Access/region permission metadata

Current state: `Address`/`Region`/`Access[T]` split locked; exact metadata unknown.

- [x] **A. Address non-authoritative** — `Address` cannot load/store/root. Permission comes from `Region`/`Access`/capability metadata. Pro: avoids C pointer mistake. Con: more metadata.
- [ ] **B. Basic permissions** — read/write/lifetime/bounds. Pro: understandable. Con: may underfit ABI/pinning.
- [ ] **C. Full provenance schema** — provenance, aliasing, alignment, volatility, atomicity, lifetime, caps. Pro: systems-complete. Con: large verifier surface.

### S-MA-1 — Module-name canonicalization

Current state: locked.

- [x] **A. Exact logical names** — canonical + case-sensitive; no case-fold, Unicode-normalize, dash-convert, or rewrite. Pro: WYSIWYG. Con: users must manage spelling exactly.
- [ ] **B. Normalized package names** — normalize for package ecosystem. Pro: easier registry policy. Con: violates exact spelling.

### S-MA-2 — Multi-module package/archive

Current state: core `.seam` image is single compiled image; package/container outside core.

- [x] **A. Loose package graph first** — `musi.json` plus `.ms`/`.seam` files; graph metadata links modules. Pro: simple. Con: not one-file distribution.
- [ ] **B. Standard archive** — one file contains graph, images, resources. Pro: deployable. Con: new container.
- [ ] **C. Indexed archive with signatures** — archive includes checksums, signatures, random access. Pro: robust. Con: largest spec.

### S-OS-1 — Type compatibility edge schema

Current state: default exact compatibility locked; explicit edges unknown.

- [ ] **A. Small edge table** — widening, nil admission, callable, box/unbox, repr conversion. Pro: clear. Con: limited.
- [ ] **B. Metadata-owned edges** — types/layouts declare allowed compatibility edges. Pro: extensible. Con: harder verifier.
- [ ] **C. Generated compatibility relation** — one schema generates verifier/docs/tests. Pro: no drift. Con: tooling dependency.

### S-TM-1 — Type/metadata binary encodings

Current state: semantic rows locked; exact type/metadata payloads unknown.

- [ ] **A. Handwritten binary tables** — explicit field order for each core table. Pro: compact spec. Con: drift risk.
- [ ] **B. Schema registry** — type/metadata rows reference schema ids. Pro: extension-ready. Con: registry needed.
- [ ] **C. Single declarative schema file** — generated encoder/decoder/spec. Pro: precise. Con: build tooling.

### S-TM-2 — ABI descriptor grammar

Current state: `@extern`/`@repr`/representability locked; exact ABI descriptor grammar unknown.

- [x] **A. Attribute direction rule** — no body imports; `export` + body exports; body without export diagnostic. Pro: visible direction. Con: ABI details remain.
  - Current:
    ```musi
    @extern(abi := .c, symbol := "foo")
    let foo(value : CInt) : CInt;

    @extern(abi := .c, symbol := "foo")
    export let foo(value : CInt) : CInt := value;
    ```
- [ ] **B. C ABI descriptor first** — calling convention, symbol/link, variadic, layout, errno/failure. Pro: enough for C. Con: not all hosts.
- [ ] **C. Host ABI descriptor** — handles, callbacks, resources, async/yield, cancellation. Pro: embedding-ready. Con: larger.

### S-VF-1 — Verifier compatibility edge schemas

Current state: same problem as S-OS-1 at verifier boundary.

- [ ] **A. Reuse type compatibility schema** — one source for type/verifier. Pro: no duplication. Con: schema must cover all verifier cases.
- [ ] **B. Separate verifier edge table** — verifier owns block/call/control compatibility. Pro: precise. Con: duplicate concepts.

### S-VF-2 — Verifier diagnostic codes/messages

Current state: verifier responsibilities locked; exact diagnostics unknown.

- [ ] **A. Kind/code only** — stable diagnostic codes and enum kinds. Pro: easy tests. Con: less user help.
- [ ] **B. Subject-first full messages** — stable headline/label/hint rules. Pro: better UX. Con: more snapshots.
- [ ] **C. Structured verifier diagnostics** — typed payload for IDE/decompiler. Pro: best tools. Con: bigger API.

## SEAM runtime, host, memory

### SM-DC-1 — Capability table schema

Current state: capabilities are explicit non-forgeable values plus metadata requirements; exact table unknown.

- [x] **A. First-class capability values** — capability evidence explicit, no `Any` auto-authority. Pro: WYSIWYG authority. Con: schema still needed.
- [ ] **B. Module-level capability table** — required/provided caps in `deps`/metadata. Pro: loader can reject early. Con: less dynamic.
- [ ] **C. Runtime capability graph** — handles/resources/caps as inspectable graph behind authority. Pro: best host tools. Con: larger runtime API.

### SM-DC-2 — Dynamic argument-pack representation

Current state: UALO semantics locked; binary/runtime representation unknown.

- [x] **A. UALO-shaped argpack** — positional first, then named, schema/default validation. Pro: matches Musi calls. Con: encoding still needed.
- [ ] **B. Dense positional + name map** — compact positional list plus optional named table. Pro: efficient. Con: more decoder cases.
- [ ] **C. Typed record argpack** — argpack is typed metadata record. Pro: introspectable. Con: heavier.

### SM-DC-3 — Keyed-storage key/value constraints

Current state: arbitrary `Any` keys rejected; declared key domains locked.

- [x] **A. Declared key domains** — `key` ops require declared key/value constraints. Pro: no JS/Python fallback. Con: metadata required.
- [ ] **B. Symbol-only dynamic keys** — keys are interned names/symbols. Pro: simple. Con: less expressive.
- [ ] **C. Typed key schemas** — key domains can be enum/string/int/compound with caps. Pro: powerful. Con: larger verifier rules.

### SM-FL-1 — Reason-code enum

Current state: host-visible outcome tags locked; exact reasons unknown.

- [ ] **A. Flat stable reason enum** — load/verify/link/init/execute/type/bounds/numeric/cap/FFI/resource. Pro: simple. Con: coarse.
- [ ] **B. Phase + reason tuple** — phase, subsystem, reason, source relation. Pro: better diagnostics. Con: more fields.
- [ ] **C. Typed reason catalog** — severity/retryability/capability/source/host fields. Pro: best tools. Con: largest catalog.

### SM-FL-2 — Host embedding outcome representation

Current state: outcome tags locked.

- [x] **A. Tagged outcomes** — `returned`, `yielded`, `failed`, `trapped`, `cancelled`. Pro: no host exception collapse. Con: host ABI structs still need shape.
- [ ] **B. C ABI result structs** — stable structs for each outcome. Pro: C-ready. Con: less ideal for other hosts.
- [ ] **C. Host-native adapters** — canonical model plus per-host bindings. Pro: ergonomic. Con: multiple adapters.

### SM-FL-3 — Numeric failure to trap mapping

Current state: numeric edge behavior unknown.

- [ ] **A. Numeric traps only for invalid operations** — divide by zero, invalid conversion. Pro: CPU-like. Con: overflow policy separate.
- [ ] **B. Checked numeric ops fail/trap by opcode** — `.chk` defines trap/failure. Pro: visible. Con: more opcode docs.
- [ ] **C. Metadata-selected numeric mode** — target/profile controls edge behavior. Pro: flexible. Con: hidden behavior risk.

### SM-FC-1 — In-memory frame layout

Current state: frame contents locked; exact memory layout unknown.

- [x] **A. VM-private layout** — layout not semantic; optional authorized inspection API. Pro: implementation freedom. Con: debugger API needed.
- [ ] **B. Stable frame ABI** — public frame object layout. Pro: easy tooling. Con: locks VM internals.
- [ ] **C. Reflective frame view** — private layout plus typed inspected projection. Pro: tools without ABI lock. Con: capability/API work.

### SM-FC-2 — Handler matching table format

Current state: primary model locked.

- [x] **A. Protected region + exit/failure reason** — no raw instruction-range semantics. Pro: source/control aligned. Con: binary table still needed.
- [ ] **B. Raw PC ranges** — handler table uses instruction ranges. Pro: common VM model. Con: worse WYSIWYG.
- [ ] **C. Region graph table** — full region graph with cleanup/handler/yield edges. Pro: precise. Con: heavier verifier.

### SM-FC-3 — Cancellation API for suspended computations

Current state: behavior locked; signatures unknown.

- [x] **A. Opaque handle operations** — resume, cancel, close/drop, status, outcome. Pro: host-friendly. Con: ABI signatures still needed.
- [x] **B. Cooperative cancellation first** — safepoints/yield points; forced close only for teardown. Pro: safe cleanup. Con: cannot always stop immediately.
- [x] **C. Defers before cancelled** — pending defers run unless cleanup traps/fails. Pro: predictable cleanup. Con: cancellation can run user code.

### SM-MG-1 — Object header layout

Current state: managed refs/layout metadata locked; exact object header unknown.

- [ ] **A. Compact header** — type/layout id + GC bits. Pro: compact. Con: less runtime info.
- [ ] **B. Header + side tables** — keep object small, metadata external. Pro: flexible. Con: extra indirection.
- [ ] **C. Profiled headers** — runtime profile selects header schema. Pro: embedded/server tuning. Con: more conformance cases.

### SM-MG-2 — GC algorithm parameters

Current state: generational Immix allowed; internals not syntax.

- [ ] **A. Implementation-defined parameters** — lines/blocks/cards nursery sizes runtime-owned. Pro: flexible. Con: less reproducibility.
- [ ] **B. Profile-defined parameters** — embedded/default/server profiles. Pro: predictable. Con: profile matrix.
- [ ] **C. Manifest/runtime policy** — host/package declares limits/tuning. Pro: low-memory devices. Con: more knobs.

### SM-MG-3 — Write/read barrier rules

Current state: barrier obligation locked; exact rule tables unknown.

- [ ] **A. Write-barrier table by storage kind** — heap/global/array/box/ref-bearing storage. Pro: clear. Con: many rows.
- [ ] **B. Layout-driven barrier generation** — barrier from ref maps/layout metadata. Pro: less duplication. Con: validator complexity.
- [ ] **C. Runtime profile barrier schema** — barrier strategy selected by collector profile. Pro: GC flexibility. Con: harder conformance.

### SM-MG-4 — Finalization/destructor semantics

Current state: ordinary managed values have no finalization assumption.

- [ ] **A. No finalization in core** — resources use explicit cleanup/defer/handles. Pro: simple. Con: less convenience.
- [ ] **B. Handle finalizers only** — host resources may define final close. Pro: practical. Con: ordering rules needed.
- [ ] **C. Full finalization model** — ordering/resurrection/moving interaction specified. Pro: powerful. Con: high complexity.

### SM-RT-1 — Host embedding API

Current state: host-visible concepts locked; exact API unknown.

- [ ] **A. Core C API** — load/link/call/resume/cancel/close/outcome. Pro: portable. Con: thin ergonomics.
- [ ] **B. Capability-aware host API** — modules/resources/caps first-class. Pro: secure embedding. Con: larger API.
- [ ] **C. Reflective embedding API** — inspect packages/modules/frames/failures with caps. Pro: best tooling. Con: broad surface.

### SM-RT-2 — Module initialization ordering

Current state: locked.

- [x] **A. Graph order** — resolve package graph, verify/link all modules, initialize dependencies before dependents, manifest declaration order tie-breaks. Pro: deterministic. Con: cyclic-init rules still need exact diagnostics.

### SM-RT-3 — Frame object layout

Current state: duplicate of SM-FC-1 at runtime spec boundary.

- [x] **A. VM-private frame layout** — semantic frame contents specified; physical layout runtime-owned. Pro: implementation freedom. Con: inspection API must project fields.
- [ ] **B. Public frame ABI** — same layout for all runtimes. Pro: debugger simplicity. Con: freezes VM internals.
