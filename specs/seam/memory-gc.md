# SEAM memory and GC

SEAM owns runtime value representation and memory management for SEIL execution. SEIL describes required types, layouts, and memory operations; SEAM implements the core runtime contract.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 10, 16-18; `specs/seil/types-metadata.md`; `specs/seil/verification.md`.

External sources used:

- Immix is a mark-region collector with line/block allocation and opportunistic defragmentation, and has been evaluated as mature space in a generational collector: <https://openresearch-repository.anu.edu.au/items/32c6080b-51ee-433e-981d-e5960787a3fb>.
- Generational collectors require remembered sets and mutator write barriers for old-to-young references: <https://www.usenix.org/conference/java-vm-02/concurrent-remembered-set-refinement-generational-garbage-collection>.
- Wasmtime reference-type implementation describes stack maps for precise live-reference discovery at GC safepoints: <https://bytecodealliance.org/articles/reference-types-in-wasmtime>.
- ECMA-335 managed-pointer restrictions show the industry need to separate managed references from unmanaged pointers for verifiability: <https://docs.ecma-international.org/ecma-335/Ecma-335-part-i-iv.pdf>.

## Language-level model

Musi is a managed systems language. At language level, ordinary managed values may be allocated, moved, traced, and reclaimed by SEAM. Musi code does not observe object addresses unless it explicitly asks for fixed storage or pointer capabilities. Stable address requirements are semantic and must lower to SEIL metadata or checked operations.

SEIL exposes managed references through `(ref T)` and unmanaged pointers through `(ptr T)`. The distinction is part of verification:

- managed references are precise GC roots when live;
- unmanaged pointers are not roots;
- managed references cannot be hidden in integers, byte arrays, or opaque pointer storage by verifiable SEIL;
- interior references/pinned references require explicit metadata/capability support.

## Generational Immix runtime

SEAM may implement managed storage with generational Immix. Under that runtime:

- young objects are allocated in a young generation optimized for frequent collection;
- mature objects are managed by an Immix-style mark-region space with line/block allocation and opportunistic movement/defragmentation;
- promotion moves surviving young objects to mature space according to runtime policy;
- object movement is allowed unless layout/storage metadata marks the object or region fixed/pinned;
- old-to-young references are tracked by remembered sets maintained through write barriers.

These collector details are not exposed as ordinary SEIL text syntax. SEIL specifies the required semantic information: exact managed-reference locations, layout metadata, safepoints, and barrier obligations.

## Values and references

SEAM values include scalars, aggregates, managed references, unmanaged pointers, callable values, boxed values, nil sentinels where admitted by core metadata, and core-defined runtime values.

Managed references point to SEAM-managed objects. Object layouts are supplied by SEIL type/layout metadata and include field kinds, reference maps, size/alignment information, array element kind, tags, and core representation details.

Unmanaged pointers are explicit unsafe/system values. They are not interchangeable with managed references. Pointer loads/stores operate under type/layout/capability rules and cannot bypass managed-reference barriers.

## Roots and stack maps

SEAM must be able to enumerate live managed references at each safepoint. Roots include:

- evaluation stack entries with managed-reference type;
- frame arguments, locals, and environment/capture slots with managed-reference type;
- globals/static slots with managed-reference type;
- active exception/yield/dynamic protocol state;
- runtime handles declared as roots.

SEIL verification derives root maps from typed stack effects, signatures, locals, environments, and body metadata. Interpreters may use verifier state directly. JIT/AOT implementations must preserve equivalent stack maps for compiled frames.

## Safepoints

Safepoints are execution points where SEAM may run GC or observe GC-visible state. Required safepoints include:

- allocation operations;
- calls and dynamic calls;
- throws and handler transfers;
- yields/suspensions;
- runtime operations declared as safepoints;
- explicit safepoint opcodes if added by a future opcode schema.

A SEAM implementation may add extra safepoints only when it can provide exact live-reference maps and preserve SEIL semantics.

## Write barriers

Generational collection requires write barriers when a managed reference is stored into a location that can outlive the young collection containing the referenced object. Barrier-relevant writes include:

- reference fields in heap objects;
- reference elements in arrays;
- boxed layouts containing references;
- global/static slots;
- core-defined reference-bearing storage.

SEIL instructions such as field stores, element stores, reference stores, and representation transitions carry barrier obligations when their target layout contains managed references. Source code does not call card-marking or remembered-set APIs directly. SEAM/JIT/interpreter inserts or executes the required barrier.

Raw byte/memory writes cannot update managed-reference-bearing storage in verifiable SEIL. A core checked bulk operation may preserve barriers and root visibility when needed.

## Fixed storage and pinning

Musi `fixed` means address stability is required for a value or storage region. It lowers to SEIL metadata or operations that constrain object movement for a defined lifetime. Under a moving or partially moving collector such as generational Immix, fixed storage may be implemented by:

- allocating into nonmoving space;
- temporary pinning with lexical/dynamic lifetime metadata;
- copying into explicit unmanaged storage when required by FFI rules;
- rejecting the operation when the requested fixed lifetime cannot be supported safely.

Pinning is visible to SEAM but not to ordinary GC-unaware source logic. Excessive or long-lived pinning may reduce compaction/defragmentation opportunities, but that is a performance consequence, not a semantic change.

## Finalization and destructors

SEIL does not assume finalization for ordinary managed values. Resource management must lower through explicit cleanup/control metadata or core runtime types. If finalization is added, core SEAM must define ordering, resurrection behavior, safepoints, and interaction with moving collection.

## Unknowns

- Exact object header layout is not specified.
- Exact GC algorithm parameters are not specified.
- Exact write-barrier/read-barrier encodings are not specified.
- Exact finalization/destructor semantics are not specified.
