# Musi Full Spec Solution Selection

Purpose: USER selects one long-term closure path for every current language/spec gap.

Selection rule:

- [ ] A — Minimal Self-Hosted SEAM Core
- [ ] B — Embedded-First Profile Kernel
- [ ] C — Tool-Exact Spec Kernel

Only one box may be checked. Until USER selects one, this file is options only. It does not change locked semantics.

## Shared gates for A/B/C

Every option must keep:

- simplicity: small core, easy read/write/maintain;
- explicivity: WYSIWYG; no hidden runtime/compiler magic;
- maintainability over convenience;
- one obvious way for each behavior;
- no code-reduction goal;
- maximal munch + one-token lookahead for Musi source;
- known execution by verified SEIL, not source-tree eval;
- managed by default, explicit `fixed`, `unmanaged`, `Address`, `Region`, `Access[T]`, `Access[mut T]`;
- explicit intrinsic declarations through `musi:rt`;
- no Rust-derived design source.

## Option A — Minimal Self-Hosted SEAM Core

Design center: make the smallest complete VM/language kernel that can host its own compiler and VM.

Resolution rule:

- If a gap can be solved in SEIL/SEAM metadata, solve it there.
- If a gap needs host policy, make it explicit host metadata or structured failure.
- If a feature needs runtime magic, reject it or expose it as `musi:rt`.

Long-term choices:

- Parser proof: grammar remains LL/LR(1)-auditable; every accepted form gets fixture + parse table proof.
- Diagnostics: small typed catalog first; every rejected source form maps to stable code.
- Lowering: one canonical lowering per source form; no lowering variants for style.
- Source maps/tool metadata: minimum near-identical decompile metadata only.
- Imports/packages: simple module graph with explicit package manifest; no implicit discovery beyond configured roots.
- SEIL text formatting: one canonical formatter; no alternate pretty styles.
- Type/metadata binary encodings: schema-packed rows with stable ids; no field names in binary.
- Trap/numeric behavior: CPU-like exactness where hardware has one obvious behavior; otherwise checked op or structured trap.
- Compatibility edges: explicit metadata table only; verifier never infers from names.
- Capability/dynamic argpack/keyed storage: fixed row schemas; dynamic means capability-proven, not duck typing.
- Frames/handlers/cancel: minimal frame objects with required root/safepoint/cleanup tables.
- GC: GenImmix allowed; default policy fixed enough for conformance, tunables non-semantic.
- Finalization: no implicit finalizers; explicit cleanup only.
- Host API: tiny embedding ABI: load, verify, link, init, call, resume/cancel, inspect failure.
- Core modules: smallest `musi:rt` intrinsic catalog needed for self-hosting.
- Stdlib: Go-like/C#-like, but outside language core.

Best when:

- self-hosting and floppy-small implementation matter most;
- fewer moving parts beat rich tooling;
- some ergonomics wait for library layer.

Risk:

- tooling/decompilation may be less rich at first;
- embedded hosts may need adapters around the tiny host API.

## Option B — Embedded-First Profile Kernel

Design center: make Musi/SEAM fit low-memory and host-constrained devices first, without becoming embedded-only.

Resolution rule:

- If a gap affects memory, startup, or host embedding, lock the low-memory behavior first.
- Bigger behavior can exist only as explicit library/runtime extension over the same core semantics.
- Profiles may select limits, not language meaning.

Long-term choices:

- Parser proof: generated tables checked against one-token source grammar; fixtures include low-memory parser mode.
- Diagnostics: compact stable codes required; rich text optional tool metadata.
- Lowering: canonical lowering emits compact SEIL first; decompile metadata optional.
- Source maps/tool metadata: tiered metadata; execution never requires tool rows.
- Imports/packages: manifest supports single-file, bundle, and package archive; all explicit, no ambient scan.
- SEIL text formatting: canonical compact form; debug comments optional.
- Type/metadata binary encodings: favor small varu indices, interning, row compaction.
- Trap/numeric behavior: all overflow/FP edge behavior explicit per opcode/schema; low-memory hosts may trap instead of emulate unsupported modes when declared.
- Compatibility edges: compact compatibility tables; no implicit widening unless row declares it.
- Capability/dynamic argpack/keyed storage: bounded argpack layouts; host-declared maximums.
- Frames/handlers/cancel: compact frame layout; suspension/cancel tables optional unless feature used.
- GC: generational Immix with profile limits; fixed/unmanaged escape hatches explicit.
- Finalization: absent by default; explicit cleanup only; host resources use handle APIs.
- Host API: small C-compatible embedding surface with explicit allocator/profile limits.
- Core modules: `musi:rt` catalog split into required core + optional declared extensions.
- Stdlib: profile-aware subsets, but one source semantics.

Best when:

- low memory, deterministic limits, and host embedding matter most;
- consoles/embedded/old hardware are first-class;
- package/runtime size must be visible.

Risk:

- profile management can drift toward dialects unless docs enforce "limits only, not meaning";
- richer desktop tooling may require optional metadata/extensions.

## Option C — Tool-Exact Spec Kernel

Design center: lock complete source-to-SEIL-to-source fidelity and conformance first, then implement the runtime under that exact contract.

Resolution rule:

- If a gap affects diagnostics, decompilation, fixtures, or conformance, specify it fully now.
- Runtime choices stay explicit, but tool evidence is part of the spec closure path.
- The implementation is never the spec; tests prove the spec.

Long-term choices:

- Parser proof: checked grammar artifact + parse fixtures for every locked source form.
- Diagnostics: complete typed diagnostic catalog with source spans, labels, and stable codes.
- Lowering: exact algorithm for every source expression, including block layout patterns.
- Source maps/tool metadata: full near-identical decompile payloads for names, docs, comments, grouping, spans, datum/operator/pattern spelling.
- Imports/packages: exact canonical module names, path resolution, package archives, version refs.
- SEIL text formatting: fully specified canonical text, assembler/disassembler round-trip required.
- Type/metadata binary encodings: every row schema specified before implementation claim.
- Trap/numeric behavior: exact reason codes and mapping to Musi diagnostics/failures.
- Compatibility edges: full schema + verifier pass/fail corpus.
- Capability/dynamic argpack/keyed storage: full protocol schemas and failure modes.
- Frames/handlers/cancel: exact frame/handler/suspension/cancel table formats.
- GC: exact root maps, barrier rules, object header, allocator limits, and conformance probes.
- Finalization: explicit decision table; default remains no implicit finalizers unless USER selects otherwise later.
- Host API: versioned embedding API with normative outcome representation.
- Core modules: full `musi:rt` intrinsic catalog before runtime expansion.
- Stdlib: exact standard/native module catalog tracked separately from language features.

Best when:

- docs/spec conformance and tooling correctness matter most;
- compiler/runtime teams need zero ambiguity before implementation;
- decompilation and diagnostics are first-class.

Risk:

- larger spec upfront;
- slower path to tiny self-hosted VM because evidence burden comes first.

## Gap coverage matrix

| Gap family                   | A                             | B                                     | C                                |
| ---------------------------- | ----------------------------- | ------------------------------------- | -------------------------------- |
| parser proof                 | minimal proof + fixtures      | proof + low-memory parser fixtures    | full checked grammar artifact    |
| diagnostics                  | small stable catalog          | compact codes + optional rich text    | complete typed catalog           |
| lowering                     | one canonical lowering        | compact canonical lowering            | exhaustive algorithm             |
| source maps/tool metadata    | minimum decompile metadata    | tiered optional metadata              | full near-identical metadata     |
| imports/packages             | explicit simple manifest      | explicit low-memory manifests/bundles | exact names, archives, versions  |
| text formatting              | one canonical formatter       | compact canonical formatter           | full round-trip spec             |
| metadata binary encodings    | stable schema-packed rows     | compact schema-packed rows            | every row schema specified       |
| trap/numeric behavior        | CPU-like or checked/trap      | profile-declared support/trap         | exact reason-code mapping        |
| compatibility edges          | explicit table only           | compact explicit table                | full schema + corpus             |
| dynamic/capability protocols | fixed minimal schemas         | bounded low-memory schemas            | full protocol schemas            |
| frame/control layout         | minimal root/safepoint tables | compact optional tables               | exact table formats              |
| memory/GC                    | conformance policy + tunables | GenImmix profile limits               | exact object/barrier/root spec   |
| finalization                 | none; explicit cleanup        | none; explicit handle cleanup         | decision table; default none     |
| host API                     | tiny ABI                      | C-compatible limited ABI              | versioned normative API          |
| `musi:rt`                    | minimum self-host catalog     | required core + optional extensions   | full intrinsic catalog           |
| stdlib                       | outside core                  | profile-aware subsets                 | exact catalog tracked separately |

## Exact current unknown coverage

Every current `specs/**` unknown maps to one row below. USER selection chooses the closure style; later docs fold chosen rules back into owning specs.

| Current gap                               | Covered by                      | A closure                       | B closure                              | C closure                              |
| ----------------------------------------- | ------------------------------- | ------------------------------- | -------------------------------------- | -------------------------------------- |
| control block layout patterns             | lowering                        | minimal canonical blocks        | compact canonical blocks               | exhaustive per-form layouts            |
| generator object representation           | frame/control layout            | minimal resumable object        | optional compact suspension object     | exact object/table format              |
| nested cleanup ordering                   | frame/control layout            | lexical LIFO cleanup table      | compact cleanup table                  | exact cleanup order matrix             |
| every source expression lowering          | lowering                        | one canonical lowering          | compact canonical lowering             | exhaustive lowering algorithm          |
| source-map/tool metadata payloads         | source maps/tool metadata       | minimum decompile payload       | tiered optional payloads               | full near-identical payload schemas    |
| import path resolution + module packaging | imports/packages                | explicit manifest + roots       | low-memory manifest/bundle/archive     | exact names, paths, archives, versions |
| package format + module discovery         | imports/packages                | simple package manifest         | single-file/bundle/archive profiles    | normative package/archive spec         |
| standard native module catalog            | stdlib / `musi:rt`              | minimum self-host set           | required core + extensions             | full native catalog                    |
| SEAM failure to Musi diagnostics          | diagnostics / trap behavior     | stable code mapping             | compact code mapping                   | full diagnostic/failure map            |
| trap taxonomy                             | trap/numeric behavior           | small structured trap set       | profile-declared traps                 | exact reason-code taxonomy             |
| numeric overflow + FP exceptions          | trap/numeric behavior           | CPU-like or checked/trap        | declared support/trap per profile      | exact opcode/schema behavior           |
| access/region permission metadata         | dynamic/capability protocols    | minimal permission rows         | bounded compact rows                   | full permission schema                 |
| module-name canonicalization              | imports/packages                | exact simple symbol rule        | explicit compact package name rule     | full canonical naming spec             |
| multi-module package/archive              | imports/packages                | manifest package                | low-memory bundle/archive              | normative archive format               |
| compatibility edge schema                 | compatibility edges             | explicit table only             | compact explicit table                 | full schema + verifier corpus          |
| type/metadata binary encodings            | metadata binary encodings       | stable schema-packed rows       | compact varu/interned rows             | all row schemas specified              |
| ABI descriptor grammar                    | metadata binary encodings / FFI | minimal ABI metadata grammar    | compact ABI/profile grammar            | full ABI descriptor grammar            |
| verifier diagnostic codes/messages        | diagnostics                     | small stable verifier catalog   | compact codes + optional text          | complete typed verifier diagnostics    |
| capability table schema                   | dynamic/capability protocols    | fixed minimal cap table         | bounded cap table                      | full capability schema                 |
| dynamic argument-pack representation      | dynamic/capability protocols    | fixed minimal argpack           | bounded compact argpack                | full argpack protocol                  |
| keyed-storage constraints                 | dynamic/capability protocols    | fixed key/value rules           | bounded key/value profile              | full keyed-storage schema              |
| reason-code enum                          | trap/numeric behavior           | small enum                      | compact profile-aware enum             | full normative enum                    |
| host outcome representation               | host API                        | tiny outcome ABI                | C-compatible outcome ABI               | versioned normative outcome API        |
| numeric failure to trap kind map          | trap/numeric behavior           | small mapping table             | profile-declared mapping               | exhaustive mapping table               |
| in-memory frame layout                    | frame/control layout            | minimal root/safepoint frame    | compact profile frame                  | exact frame layout                     |
| handler matching table format             | frame/control layout            | minimal handler table           | compact handler table                  | exact handler table format             |
| suspended cancellation API                | frame/control layout / host API | resume/cancel ABI               | compact optional cancel API            | normative cancellation API             |
| object header layout                      | memory/GC                       | conformance header contract     | compact profile header                 | exact object header                    |
| GC algorithm parameters                   | memory/GC                       | semantic policy + tunables      | profile limits                         | exact parameters/probes                |
| write/read barrier rules                  | memory/GC                       | required barrier obligations    | compact barrier encodings              | exact barrier spec                     |
| finalization/destructor semantics         | finalization                    | no implicit finalizers          | no implicit finalizers, handle cleanup | explicit decision table, default none  |
| module init after load/verify/link/init   | host API / imports/packages     | simple deterministic init order | explicit profile init order            | exact normative init algorithm         |
| canonical whitespace policy               | text formatting                 | one formatter                   | compact formatter                      | exact round-trip formatting spec       |
| text parse/assemble diagnostic wording    | diagnostics / text formatting   | stable small text codes         | compact codes + optional text          | exact wording/catalog                  |
| `tool` metadata schemas                   | source maps/tool metadata       | minimum schemas                 | tiered schemas                         | full schemas                           |
| section-family ids beyond core            | metadata binary encodings       | stable extension id allocation  | compact extension/profile id ranges    | full id registry policy                |
| per-row binary schemas                    | metadata binary encodings       | schema-packed core rows         | compact profile rows                   | every row schema locked                |
| package/container transport               | imports/packages                | outside core package layer      | low-memory package/archive             | full package/container transport       |

## USER mark area

Chosen option:

- [ ] A
- [ ] B
- [ ] C

Notes from USER:

```text

```

