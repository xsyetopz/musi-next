# 25. Expanded Host Interop and Raw FFI Contract

Status: normative semantic expansion of specs/18-20.

## Boundary split

Musi has two external interaction paths:

```text
Host interop = VM-aware, checked, embeddable-first boundary
Raw FFI      = ABI-level, unsafe, layout/calling-convention boundary
```

They are not the same system.

## Host interop

Host interop is the normal embedding path.

Host values exposed to Musi use `Host[T]`.

```text
Host[T] is host-owned.
Host[T] is opaque to Musi layout inspection.
Host[T] is not RawPtr[T].
Host[T] is not Ref[T].
Host[T] is governed by host binding lifetime rules.
```

A host module may be surfaced through ordinary import-expression discipline:

```musi
let engine := import "host/engine";
```

This does not introduce standalone module declaration syntax.

## Root

`Root[T]` is a VM-owned stable token to a Musi-managed value.

Use `Root[T]` when host/native code retains a Musi-managed value across possible VM movement or collection.

```text
Root[T] is not a raw address.
Root[T] is not Host[T].
Root[T] survives managed heap movement.
Root[T] must be released/dropped according to VM rules.
```

## RawPtr

`RawPtr[T]` is a raw physical/ABI address.

```text
RawPtr[T] is untraced.
RawPtr[T] does not keep a managed value alive.
RawPtr[T] is unsafe to dereference or call through.
RawPtr[T] must not be confused with Host[T] or Root[T].
```

Raw access to managed storage requires `pin`.

```musi
pin buffer as p (
  unsafe (
    foreignWrite(p.ptr, p.len);
  );
)
```

## Raw FFI source surface

Existing raw FFI source surface:

```musi
@foreign(abi := .cdecl, name := "foreign_name")
let f(x : T) : U;
```

No `foreign` keyword exists.

No foreign block syntax exists.

Direct calls to declarations imported through `@foreign(...)` require `unsafe`.

## Layout

External layout is opt-in and boundary-specific.

Ordinary Musi data layout remains VM-owned and private.

Externally laid-out data may contain only fields with explicit external representation.

A managed Musi value does not become externally laid out merely by appearing in an external boundary.

## Callbacks

Raw foreign code may call Musi only through declared callback entries/trampolines.

A callback declaration must specify or be associated with:

```text
lifetime
retention
rooting
thread
reentry
trap/unwind behavior
host error mapping
calling convention
```

A callback retained by native or host code must retain Musi-managed callable state through `Root[T]` or an equivalent VM-managed token.

## Error and trap mapping

Host interop and raw FFI must state how failures cross the boundary.

Allowed categories:

```text
value-level result, such as Expect[T, E]
Maybe/absence result
host error object
trap
load/link failure
```

Unspecified exception/unwind crossing is not allowed.

## Embedded requirement

An embedding host must be able to disable raw FFI entirely while retaining host interop.

Raw FFI is not required for ordinary embeddability.
