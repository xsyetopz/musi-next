# Musi runtime expectations over SEAM

Musi source lowers to SEAM bytecode and runs under SEAM. Any runtime behavior visible to Musi must be expressible through SEAM bytecode verification, metadata, capabilities, imports, exports, or SEAM failure outcomes.

## Packages and imports

Source package canonical format is loose graph: `musi.json` plus `.ms` and `.seam` files. `musi.json` owns package metadata, `imports`, `exports`, dependencies, workspaces, tasks, and compiler/tool policy.

Import syntax uses ESM-like string paths:

```musi
let local := import "./local.ms";
let defaulted := import "./tool";
let core := import "musi:core";
```

Resolution rules visible to Musi:

- relative/absolute string specifiers resolve through package/workspace path policy;
- bare specifiers/package names resolve through manifest `imports`/`dependencies`;
- `musi:` is reserved like `node:`/`bun:` and cannot be shadowed by user import maps/packages;
- manifest `exports` controls package public surface;
- source `export` controls module public surface;
- extensionless imports are policy/linter controlled;
- when extensionless import resolution is enabled, `./foo` resolves to `./foo.ms`, then `./foo/index.ms` only as fallback when no direct file exists.

`.seam` is build/cache/distribution artifact. Package/container transport outside core `.seam` owns compression, checksums, signatures, resources, and multiple images. Container/archive spec is required before bundled distribution, signed packages, resource bundles, plugin archives, or streaming package loading; until then loose `musi.json` + `.ms` + `.seam` graph is canonical.

Host-provided modules participate in the package graph as explicit nodes with provider and capability metadata.

Required native modules: `musi:core`, `musi:rt`, `musi:ffi`, `musi:text`.

Optional provider/capability-gated modules: `musi:host`, `musi:fs`, `musi:process`, `musi:time`, `musi:random`, `musi:encoding`, `musi:reflect`, `musi:probe`, `musi:package`, `musi:schema`, `musi:bytecode`, `musi:test`.

Importing an absent optional standard module is load/link missing-provider diagnostic. `known import` of optional standard module requires deterministic known-capable provider metadata; absent or nondeterministic provider is compile-time diagnostic.

Module initialization order: resolve package graph, verify/link all modules, initialize dependencies before dependents, use manifest declaration order as tie-breaker for otherwise equal nodes. Package/module initialization cycles reject with load/link diagnostic.

## Managed memory and low-level access

Ordinary Musi values lower to SEAM-managed storage. `unmanaged T` opts representation out of managed tracing, movement, and reclamation unless explicit core metadata says otherwise. SEAM may use generational Immix; Musi sees only language contract:

- ordinary managed refs may move;
- object movement allowed unless storage fixed/pinned by language or ABI rule;
- managed-ref writes observe barriers;
- address + low-level access behavior explicit and capability-constrained.

`fixed` requests stable storage. Not GC-off. Lowers to SEAM bytecode metadata/ops describing which storage needs stable address/access semantics and lifetime.

Musi low-level memory names: `Address`, `Region`, `Access[T]`, `Access[mut T]`. `Address` is address token only: no provenance, bounds, lifetime, permission, typed access, or root. `Access[T]` readable typed access. `Access[mut T]` readable/writable typed access. `MutAccess[T]` and `OpaqueAccess[T]` are DRY aliases only.

Musi cannot observe Immix line/block/card/nursery details. It can observe allocation failure, resource limits, explicit cleanup, and specified access/address semantics.

SEAM bytecode/SEAM provide precise GC through typed refs, layouts, safepoints, stack maps, and write-barrier obligations. Musi does not expose write barriers as source APIs.

Object layout uses compact headers plus side tables. GC parameters are manifest/host policy through JVM-style `seamArguments`: `-Xms`, `-Xmx`, `-Xss`, `-Xmn`, `-XX:NewRatio`, `-XX:SurvivorRatio`, `-XX:MaxGCPauseMillis`, `-XX:GCTimeRatio`, `-XX:+/-UseIncrementalGC`, `-XX:+/-UseImmixDefrag`, `-XX:ImmixBlockSize`, `-XX:ImmixLineSize`, `-XX:+StressGC`, `-XX:FuelMax`.

Barrier rules are layout-driven from ref maps, layout metadata, opcode storage effects, and active collector policy. Source never calls barriers. Ordinary managed values have no core finalizer/destructor semantics and no resurrection model; cleanup is explicit through `defer`, cleanup regions, handles, or host APIs.

## Dynamic behavior and capabilities

Dynamic ops, capability checks, and FFI/native calls require explicit SEAM bytecode metadata. `Any` does not imply implicit dynamic lookup or reflection. Capability failures are structured SEAM failures.

Capabilities are first-class non-forgeable runtime values plus metadata requirements. Capability/resource graph uses typed nodes and typed authority edges with opaque stable identity. Host resource handles are values protected by capabilities; identity stays separate from authority.

Dynamic calls use explicit callee, typed UALO argpack record, expected signature, result contract, and structured failure. Keyed storage is limited to typed key schemas.

`Address` is non-authoritative by itself; load/store/permission comes from `Region`/`Access`/capability metadata.

## FFI and host outcomes

`@extern` is metadata/attribute, not keyword. Direction is body + export:

```musi
@extern(abi := .c, symbol := "foo")
let foo(value : CInt) : CInt;

@extern(abi := .c, symbol := "foo")
export let foo(value : CInt) : CInt := value;
```

`@extern` without body imports external implementation. `@extern export let ... := ...` exports a Musi implementation outward. `@extern let ... := ...` without `export` is diagnostic.

Callbacks from host into Musi are exported Musi functions passed by symbol/handle through host embedding API. Native resources crossing FFI use opaque handles by default; typed `Access[T]`/`Address` only when ABI metadata declares representable memory access. Native calls are failure-capable unless metadata proves otherwise.

Host-visible outcomes are tagged: `returned`, `yielded`, `failed`, `trapped`, `cancelled`. Host outcomes use canonical tagged result structs; host bindings may adapt shape but must preserve tags and payload fields. Host exceptions do not cross boundary as host exceptions.

## Allocation contracts

`@noalloc` means no managed heap allocation in body or transitive calls. Not GC-off. Rejects managed allocation, boxing, managed array/text/object creation, closure allocation, calls to non-`@noalloc`, and dynamic calls unless target proven `@noalloc`.

SEAM/tooling may use `@noalloc` for low-latency paths, runtime internals, FFI callbacks.

## Known phase

Known execution runs verified SEAM bytecode under deterministic known-phase limits. No ambient time/random/process/env/IO/filesystem/network unless explicit deterministic known import or declared `musi:rt` intrinsic provides it. Optional `known import` requires deterministic known-capable provider metadata.

## Failure mapping

SEAM failures map to boundary-aware Musi diagnostics or runtime outcomes by phase. Diagnostics include phase, module/proc location, source span when present, and host/resource/capability/ABI boundary context:

- source/type/lowering errors are diagnostics;
- loader/verifier/link errors reject module before execution;
- known-phase failures are compile-time diagnostics;
- runtime traps become structured runtime failures;
- resource exhaustion is structured failure, not host UB.

## Detail gaps

- Exact native module API contents not specified.
- Exact boundary-aware diagnostic code/message catalog not specified.
- Exact cyclic-init diagnostic wording/code not specified.
