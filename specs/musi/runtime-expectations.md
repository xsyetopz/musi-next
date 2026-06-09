# Musi runtime expectations over SEAM

Musi source lowers to SEIL and runs under SEAM. Any runtime behavior visible to Musi must be expressible through SEIL verification, metadata, capabilities, imports, exports, or SEAM failure outcomes.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md`; `grammar/musi.ebnf`; `specs/seil/*`; `specs/seam/*`.

## Managed memory

Ordinary Musi values lower to SEAM-managed storage. `unmanaged T` opts representation out of managed tracing, movement, and reclamation unless explicit core metadata says otherwise. SEAM may use generational Immix; Musi sees only language contract:

- reachable managed refs stay valid;
- unreachable managed objects may be reclaimed;
- object movement allowed unless storage fixed/pinned by language or ABI rule;
- access/address stability not implicit;
- address + low-level access behavior explicit and capability-constrained.

`fixed` requests stable storage. Not GC-off. Lowers to SEIL metadata/ops describing which storage needs stable address/access semantics and lifetime.

Musi low-level memory names: `Address`, `Region`, `Access[T]`, `Access[mut T]`. `Address` is address token only: no provenance, bounds, lifetime, permission, typed access, or root. `Access[T]` readable typed access. `Access[mut T]` readable/writable typed access. `MutAccess[T]` and `OpaqueAccess[T]` are DRY aliases only.

## GC-observable behavior

Musi cannot observe Immix line/block/card/nursery details. It can observe allocation failure, resource limits, explicit cleanup, and specified access/address semantics.

SEIL/SEAM provide precise GC through typed refs, layouts, safepoints, stack maps, and write-barrier obligations. Musi does not expose write barriers as source APIs.

## Dynamic behavior and capabilities

Dynamic ops, capability checks, and FFI/native calls require explicit SEIL metadata. `Any` does not imply implicit dynamic lookup or reflection. Capability failures are structured SEAM failures.

## No-allocation contract

`@noalloc` marks callable allocation-free for managed heap. Not GC-off. `@noalloc` callable cannot allocate managed objects, box values, create managed arrays/text/objects, allocate closures, call non-`@noalloc`, or perform dynamic calls unless target proven `@noalloc`.

SEAM/tooling may use `@noalloc` for low-latency paths, runtime internals, FFI callbacks.

## Known phase

Known execution runs verified SEIL under deterministic known-phase limits. No ambient time/random/process/env/IO/filesystem/network unless explicit deterministic known import or declared `musi:rt` intrinsic provides it.

## Failures

SEAM failures map to Musi diagnostics or runtime outcomes by phase:

- loader/verifier/link errors reject module before execution;
- known-phase errors become compiler diagnostics;
- runtime traps become structured runtime failures;
- resource exhaustion is structured failure, not host UB.

## Unknowns

- Exact Musi package format and module discovery rules not specified.
- Exact standard native module catalog not specified.
- Exact user-facing mapping from SEAM failure payloads to Musi diagnostics not specified.
