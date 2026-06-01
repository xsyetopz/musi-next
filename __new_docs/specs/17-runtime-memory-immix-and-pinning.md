# 17. Runtime Memory, Generational Immix, Unsafe, and Pinning

Status: normative addendum for runtime-facing semantics.

This chapter specifies the semantic model for managed storage, raw address exposure, `unsafe`, and `pin`. It does not specify bytecode encoding, object header format, collector implementation details, or exact lowering.

## VM memory model

Musi uses a Generational Immix VM/runtime model.

Ordinary managed storage is movable. Source code must not treat a managed allocation's physical address as stable unless an explicit language/runtime construct creates an address-stability condition.

Ordinary Musi layout is implicit, private, and VM-owned.

The VM may use private representation strategies, including but not limited to headers, forwarding state, tags, field placement, compression, indirection, movement, promotion, evacuation, and block/line management.

These VM representation choices are not source-level layout promises.

## `unsafe`

`unsafe (...)` is checked lexical authority for explicitly unsafe operations.

It does not mean unchecked code.

Inside `unsafe`, ordinary Musi checking remains in force unless a specific operation is classified as unsafe and admitted by the language/runtime rules.

`unsafe` does not:

- pin managed storage;
- root managed values;
- extend lifetimes;
- suppress GC;
- bypass write barriers;
- permit pointer escape;
- make invalid addresses valid;
- make foreign retention legal;
- disable ordinary type, mutability, or visibility rules.

Examples of operations that may require `unsafe`, where admitted by the relevant chapter:

- raw pointer dereference;
- raw pointer write;
- raw pointer arithmetic;
- raw pointer cast;
- direct raw FFI call;
- unsafe VM intrinsic;
- explicit layout reinterpretation.

## `pin`

Existing source form from the 1.0 candidate pack:

```musi
pin value as name (
  body
)
```

`pin` creates a lexical Immix pin region for the exposed managed storage.

During the dynamic extent of the region, the exposed managed storage is:

- kept alive;
- not relocated by the VM;
- address-stable for the views or raw addresses derived under the region.

`pin` does not:

- grant unsafe authority;
- recursively pin reachable children;
- allow raw pointers to escape;
- allow foreign retention;
- disable GC;
- bypass write barriers;
- imply any C# CLR object model behavior.

Pinning an interior field or element pins the owning allocation or owning backing storage required to preserve that interior address.

Pinning a view pins the backing storage, not merely the view descriptor.

Pinning a container does not automatically pin every object referenced by elements inside the container.

## Raw pointer source name

The previous source names `Ptr[T]` and `Ptr[mut T]` are replaced by:

```text
RawPtr[T]
RawPtr[mut T]
```

`RawPtr` is a raw physical address category. It is:

- non-owning;
- non-rooting;
- non-lifetime-extending;
- not a managed reference;
- unsafe to use as memory;
- invalid beyond the contract that produced it.

A raw pointer into movable managed storage requires either:

- a surrounding `pin` region; or
- explicit non-moving storage.

A `RawPtr` produced from pinned managed storage cannot outlive the pin region.

## Non-moving storage

Musi admits explicit non-moving storage as a low-level boundary feature.

Non-moving storage is distinct from ordinary Generational Immix movable storage.

Non-moving storage may expose stable `RawPtr` values without `pin`, according to the storage type's contract.

Non-moving storage is not scanned for managed references by default.

Managed references inside non-moving storage require an explicit VM-rooted storage mechanism, not ordinary external/native memory.

## Suspension boundary

No suspension point may occur inside `unsafe` or `pin`.

This includes `yield` and any other source-level suspension mechanism admitted by the language.

Reason: `unsafe` and `pin` are dynamic lexical runtime obligations. Suspending across them would require continuation, root, pin, and address-validity rules that are not part of the ordinary source model.

## GC barriers

`unsafe` does not bypass GC barriers.

Any operation that stores a managed reference into managed storage must perform the required VM barrier.

Raw pointer writes must not smuggle managed references into unmanaged, host, external, or non-scanned memory.

Managed-reference writes require checked operations or explicit VM intrinsics with barrier semantics.
