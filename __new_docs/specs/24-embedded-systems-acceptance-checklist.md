# 24. Embedded Systems Acceptance Checklist

Status: normative rejection checklist.

A Musi/SEAM design change is rejected for embedded-system suitability if it violates any item below without an explicit replacement rule.

This checklist is semantic. It is not a profile system.

## Type and inference

```text
[ ] Unknown does not reach SEIL or SEBC.
[ ] Missing annotations do not silently become Any.
[ ] Any appears only through explicit annotation, explicit boxing, explicit dynamic boundary, or imported dynamic-language frontend policy.
[ ] Operators resolve to concrete operations or trait/evidence-constrained operations.
[ ] Ambiguous evidence resolution is an error.
[ ] Public/exported signatures are stable in SEIL/SEBC.
[ ] Static calls are not silently lowered to dynamic dispatch.
[ ] Any operations are checked, narrowed, explicitly dynamic, or unsafe.
```

## Allocation and runtime cost

```text
[ ] Function metadata can express whether the function may allocate.
[ ] Boxing into Any is allocation-visible unless represented inline by the VM.
[ ] Closure creation is allocation-visible unless proven allocation-free.
[ ] Syntax value construction is allocation-visible unless represented by compiler/runtime handles.
[ ] Host calls declare whether they may allocate.
[ ] Spread/splat does not silently allocate unless specified.
[ ] Dynamic dispatch does not appear without explicit source or frontend policy.
```

## Memory movement, root, and pin

```text
[ ] Managed values may move unless fixed or pinned.
[ ] pin creates temporary address stability only.
[ ] fixed is storage/placement/lifetime, not compiler knowledge.
[ ] Root keeps managed values alive across host/native retention.
[ ] Host handles are host-owned and opaque.
[ ] RawPtr is untraced and unsafe.
[ ] RawPtr into managed storage requires a valid pin region.
[ ] Pinned views cannot escape their valid region unless a type explicitly permits a safe handle.
```

## Host and raw FFI

```text
[ ] Host interop remains distinct from raw ABI FFI.
[ ] Host[T] is not RawPtr[T].
[ ] Root[T] is not RawPtr[T].
[ ] Raw FFI calls require unsafe.
[ ] Managed addresses cannot be retained by native code.
[ ] Native retention of Musi-managed values uses Root[T].
[ ] Host-owned resources exposed to Musi use Host[T].
[ ] Callbacks declare retention, reentry, thread, trap/unwind, and lifetime behavior.
```

## Parser and source syntax

```text
[ ] The parser remains LR(1) / LL(1)-compatible by design.
[ ] New source syntax does not require parser backtracking.
[ ] New source syntax does not require semantic predicates.
[ ] Name/type resolution does not choose grammar alternatives.
[ ] New constructs do not reuse #, $, ~, or ... contrary to their locked ownership.
[ ] Examples do not become syntax unless a spec chapter says so.
```

## SEIL/SEBC

```text
[ ] SEIL remains language-neutral and not Musi-shaped.
[ ] SEIL has fixed syntax once specified; no reader macros.
[ ] SEBC is a bytecode encoding of SEIL, not a separate semantic language.
[ ] SEAM verifies stack effects and type effects before execution.
[ ] SEBC can be loaded from memory buffers without filesystem assumptions.
[ ] Unknown sections are skippable unless marked required.
[ ] Debug/source metadata supports generated-code origin chains.
```

## Failure condition

If one checklist item cannot be satisfied, the design must either:

```text
1. define a stricter semantic rule;
2. reject the feature;
3. move the behavior behind explicit unsafe;
4. move the behavior behind explicit host/compiler capability;
5. or mark the design unresolved.
```

A warning is not enough for memory, FFI, hidden Any, hidden allocation, or unverifiable pin/root behavior.
