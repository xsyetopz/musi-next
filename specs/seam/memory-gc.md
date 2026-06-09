# SEAM memory and GC

SEAM owns runtime value representation + memory management for SEAM bytecode execution. SEAM bytecode describes required types, layouts, memory ops; SEAM implements runtime contract.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 10, 16-18; `specs/seam-bytecode/types-metadata.md`; `specs/seam-bytecode/verification.md`.

External sources:

- Immix mark-region collector with line/block allocation + opportunistic defrag, evaluated as mature space in generational collector: <https://openresearch-repository.anu.edu.au/items/32c6080b-51ee-433e-981d-e5960787a3fb>.
- Generational collectors need remembered sets + mutator write barriers for old-to-young refs: <https://www.usenix.org/conference/java-vm-02/concurrent-remembered-set-refinement-generational-garbage-collection>.
- Wasmtime reference types use stack maps for live-ref discovery at safepoints: <https://bytecodealliance.org/articles/reference-types-in-wasmtime>.
- ECMA-335 managed-pointer restrictions show need to separate managed refs from unmanaged pointers: <https://docs.ecma-international.org/ecma-335/Ecma-335-part-i-iv.pdf>.

## Language-level model

Musi is managed systems language. Ordinary managed values may be allocated, moved, traced, reclaimed by SEAM. `unmanaged T` marks storage/representation outside managed tracing, movement, reclamation unless core metadata says otherwise. Musi observes object addresses only through `fixed`, `Address`, `Region`, or `Access` capabilities. Stable address/access requirements are semantic and lower to SEAM bytecode metadata or checked ops.

SEAM bytecode exposes managed refs as `(ref T)` and VM unmanaged access/pointer values as `(ptr T)`. Musi names these through `Address`, `Region`, `Access[T]`, `Access[mut T]`, and aliases like `MutAccess[T]`. Verification distinction:

- managed refs are precise GC roots when live;
- unmanaged pointer/access values and addresses are not roots;
- managed refs cannot hide in integers, byte arrays, address storage, or opaque access storage in verifiable SEAM bytecode;
- interior/pinned refs need explicit metadata/capability support.

## Generational Immix runtime

SEAM may use generational Immix:

- young objects allocated in young generation;
- mature objects live in Immix mark-region space with line/block allocation + opportunistic movement/defrag;
- survivors promote by runtime policy;
- movement allowed unless layout/storage metadata says fixed/pinned;
- old-to-young refs tracked by remembered sets via write barriers.

Collector details are not ordinary SEAM bytecode syntax. SEAM bytecode specifies semantic data: exact managed-ref locations, layout metadata, safepoints, barrier obligations.

## Values and references

SEAM values: scalars, aggregates, managed refs, unmanaged pointer/access values, address tokens, callables, boxes, nil sentinels where core metadata admits, core runtime values.

Managed refs point to SEAM-managed objects. Layout metadata supplies fields, reference maps, size/alignment, array elem kind, tags, core representation.

Unmanaged pointer/access values are explicit system values. Not interchangeable with managed refs or addresses. Access load/store obey type/layout/region/capability rules and cannot bypass barriers. `Address` alone cannot load, store, or root managed objects.

## Roots and stack maps

SEAM must enumerate live managed refs at each safepoint. Roots include:

- eval stack entries with managed-ref type;
- frame args, locals, env/capture slots with managed-ref type;
- globals/static slots with managed-ref type;
- active exception/yield/dynamic protocol state;
- runtime handles declared as roots.

Verifier derives root maps from typed stack effects, signatures, locals, envs, body metadata. Interpreters may use verifier state directly. JIT/AOT must preserve equivalent stack maps.

## Safepoints

Required safepoints:

- allocation ops;
- calls + dynamic calls;
- throws + handler transfers;
- yields/suspensions;
- runtime ops declared as safepoints;
- explicit safepoint opcodes if future schema adds them.

SEAM may add safepoints only with exact live-ref maps and preserved semantics.

## Write barriers

Generational collection needs barriers when managed ref stored into location outliving young collection. Barrier-relevant writes:

- heap reference fields;
- array reference elements;
- boxed layouts containing refs;
- globals/static slots;
- core-defined ref-bearing storage.

SEAM bytecode store/transition ops carry barrier obligations when target layout contains managed refs. Source never calls card-mark/remembered-set APIs directly. SEAM/JIT/interpreter inserts or executes barrier.

Raw byte/memory writes cannot update managed-ref-bearing storage in verifiable SEAM bytecode. Core checked bulk op may preserve barriers + root visibility when needed.

## Fixed storage and pinning

Musi `fixed` = address stability required for value/region. Lowers to SEAM bytecode metadata/ops constraining movement for lifetime. Moving/partly moving collector may implement with:

- nonmoving allocation;
- temporary pinning with lexical/dynamic lifetime metadata;
- copy into explicit unmanaged storage for FFI;
- reject when lifetime cannot be supported safely.

Pinning visible to SEAM, not ordinary GC-unaware source logic. Excess/long pinning can hurt compaction/defrag; performance effect, not semantic change.

## Finalization and destructors

SEAM bytecode assumes no finalization for ordinary managed values. Resource management lowers through explicit cleanup/control metadata or core runtime types. If finalization added, core SEAM must define ordering, resurrection, safepoints, moving-collector interaction.

## Unknowns

- Exact object header layout not specified.
- Exact GC algorithm parameters not specified.
- Exact write/read barrier encodings not specified.
- Exact finalization/destructor semantics not specified.
