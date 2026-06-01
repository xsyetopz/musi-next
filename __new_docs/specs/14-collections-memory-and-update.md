# 14. Collections, Memory Views, and Owned Update

Status: normative for source names and surface policy.

## Collection names

```text
Vec[T]        dynamic/growable owned sequence
Array[T, N]   fixed-size owned sequence
Slice[T]      borrowed read-only contiguous view
Slice[mut T]  borrowed writable contiguous view
```

No `[]T` or `[N]T` collection type sugar exists.

Sequence literals use `#[...]` and resolve by expected type.

```musi
let values : Vec[Nat32] := #[1n32, 2n32, 3n32];
```

## Mutability

Immutable binding:

```musi
let x := 0n32;
```

Mutable binding:

```musi
let mut y := 0n32;
y := y + 1n32;
```

## Owned update with with

`with` performs record update / structural extension where admitted.

```musi
let next := state with #{count := state.count + 1n32};
```

`with` is not a control-flow payload marker.

## Raw pointers and references

Source-level names:

```text
Ref[T]
Ptr[T]
Ptr[mut T]
```

Raw pointer operations require `unsafe` where admitted.

Stable address exposure uses `pin`.

Implementation/runtime mechanics for pointer representation, object layout, handles, and GC rooting are not source syntax.
