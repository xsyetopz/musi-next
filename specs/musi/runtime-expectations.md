# Musi runtime expectations over SEAM

Musi source lowers to SEIL and executes under SEAM. Runtime behavior visible to Musi must be expressible through SEIL verification, metadata, capabilities, imports, exports, and SEAM failure outcomes.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md`; `grammar/musi.ebnf`; `specs/seil/*`; `specs/seam/*`.

## Managed memory

Musi assumes managed memory for ordinary values that lower to SEAM-managed storage. SEAM may use generational Immix, but Musi observes only the language-level contract:

- managed references remain valid while reachable;
- unreachable managed objects may be reclaimed;
- object movement is allowed unless storage is fixed/pinned by explicit language constructs or ABI rules;
- pointer/address stability is not implicit;
- raw pointer behavior is explicit and capability constrained.

`fixed` requests stable storage. It does not disable GC globally. It lowers to SEIL metadata or operations that tell SEAM which storage requires stable address semantics and for what lifetime.

## GC-observable behavior

Musi code cannot observe Immix line/block/card/nursery details. It can observe allocation failure, resource limits, explicit cleanup behavior, and specified pointer/address semantics.

SEIL and SEAM provide precise GC behavior through typed references, layouts, safepoints, stack maps, and write-barrier obligations. Musi does not expose write barriers as source APIs.

## Dynamic behavior and capabilities

Dynamic operations, capability checks, and FFI/native calls require explicit SEIL metadata. `Any` does not imply implicit dynamic lookup or arbitrary reflection. Capability failures are structured SEAM failures.

## No-allocation contract

`@noalloc` marks a callable as allocation-free for managed heap allocation. It is not a GC-off switch. A `@noalloc` callable cannot allocate managed objects, box values, create managed arrays/text/objects, allocate closures, call non-`@noalloc` code, or perform dynamic calls unless the resolved target is proven `@noalloc`.

SEAM and tooling may use `@noalloc` for low-latency paths, runtime internals, and FFI callbacks.

## Known phase

Known execution runs verified SEIL under deterministic known-phase limits. Known code has no ambient time/random/process/env/IO/filesystem/network access unless a deterministic known import/intrinsic explicitly provides it.

## Failures

SEAM failures map to Musi diagnostics or runtime outcomes according to phase:

- loader/verifier/link errors reject the module before execution;
- known-phase errors become compiler diagnostics;
- runtime traps become structured runtime failures;
- resource exhaustion is a structured failure, not unspecified host behavior.

## Unknowns

- Exact Musi package format and module discovery rules are not specified.
- Exact standard native module catalog is not specified.
- Exact user-facing mapping from SEAM failure payloads to Musi diagnostics is not specified.
