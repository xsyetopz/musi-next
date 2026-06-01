# 20. External Layout and Representation

Status: normative addendum for layout semantics.

## Layout scope

Layout metadata is only for external representation contracts.

Ordinary Musi layout remains implicit, private, and VM-owned.

No source-level layout metadata is needed for ordinary Musi-managed data.

Ordinary Musi layout is not FFI-stable and not byte-stable.

## Balanced layout model

Layout metadata uses a balanced model:

- compact presets for common external layout contracts;
- explicit representation overrides only where needed.

A preset defines the default external representation contract.

Overrides may refine representation details only where the preset allows it.

Examples of representation details:

- padding;
- alignment;
- field order;
- tag or discriminator layout;
- passability.

Overrides cannot change source semantics.

Layout metadata may not change:

- field names;
- type identity;
- mutability;
- ownership;
- destruction/finalization behavior;
- pattern semantics;
- method lookup;
- trait/evidence satisfaction.

## Packing

There is no `packed` keyword.

Packing is a layout policy, not a standalone declaration family and not a layout domain.

Packed behavior is expressed through external layout metadata where admitted.

## Externally laid-out data fields

Externally laid-out data may contain only fields with an explicit external representation.

Ordinary Musi-managed values do not become externally laid out merely because they appear inside externally laid-out data.

Managed values must cross external layout boundaries through explicit representation forms:

- `Root[T]`;
- `Host[T]`;
- `RawPtr[T]` / `RawPtr[mut T]`;
- copied bytes or views;
- ABI scalars;
- compatible externally laid-out data.

Externally laid-out data cannot contain the following by implicit layout:

- `Text`;
- `Vec[T]`;
- ordinary `data` without external layout metadata;
- closures;
- traits/interfaces/evidence values;
- ordinary `Ref[T]`;
- generic unspecialized `T`;
- managed object references.

## C ABI aliases

C ABI aliases exist for target C ABI types.

They are distinct from fixed-width Musi numeric types.

Examples of C ABI aliases include:

```text
CChar
CSChar
CUChar
CShort
CUShort
CInt
CUInt
CLong
CULong
CLongLong
CULongLong
CSize
CPtrDiff
CFloat
CDouble
CLongDouble
CBool
```

A fixed-width Musi integer and a target C ABI integer may lower to the same machine representation on a target, but they are not the same source concept.

## `opaque` and `erased`

Existing consequence words from the 1.0 candidate pack remain distinct:

- `opaque` is representation opacity at an exported/returned boundary.
- `erased` is existential erasure.

`Host[T]` does not require a new opaque-resource declaration family.

Use existing `opaque` and `erased` rules where those concepts apply.
