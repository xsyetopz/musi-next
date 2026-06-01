# 18. Host Interop and Raw FFI

Status: normative addendum for external interaction model.

Musi distinguishes host interop from raw FFI.

## Host interop

Host interop is the normal embedding/scripting boundary.

It is:

- VM-aware;
- safe by default;
- the ordinary extension path for embeddable Musi;
- not raw ABI interop;
- not raw managed-address exposure.

Musi may call host-provided functions through the VM-aware binding layer.

The host may call Musi through the VM embedding API.

Host functions receive and return Musi values through checked VM conversion and binding rules.

Host interop does not expose raw managed addresses.

Calling a host function from Musi does not inherently require `unsafe`.

## Raw FFI

Raw FFI is the low-level external ABI boundary.

It is:

- ABI-oriented;
- unsafe by default;
- explicit;
- used for native/C ABI calls and equivalent raw ABI interactions;
- not the normal embedding path.

Existing source surface from the 1.0 candidate pack:

```musi
@foreign(abi := .cdecl, name := "foreign_name")
let f(x : T) : U;
```

Direct calls to declarations imported through `@foreign(...)` require `unsafe`.

No `foreign` keyword exists.

No foreign block syntax exists.

## Bidirectional external interaction

Bidirectional external interaction is split by boundary type and call direction.

Host interop:

- Musi may call host through VM-aware bindings.
- Host may call Musi through the VM embedding API.
- This path is safe by default.
- This path does not expose raw managed addresses.

Raw FFI:

- Musi may call raw foreign ABI through `@foreign(...)` declarations.
- Direct raw foreign calls require `unsafe`.
- Raw foreign ABI may call Musi only through explicit callback entries/trampolines.
- Callback entry requires declared lifetime, threading, reentry, rooting, and trap/unwind behavior.

## Root and Host

`Root[T]` is a VM-owned stable token to a Musi-managed value.

It is used when host or raw foreign code needs to hold a Musi-managed value across VM movement.

`Root[T]` is not a raw address.

`Root[T]` survives Generational Immix movement.

`Host[T]` is a host-owned resource, object, service, or value exposed to Musi.

`Host[T]` is not a raw pointer.

`Host[T]` is not inspected by Musi by layout.

`Host[T]` lifetime is governed by host binding and release rules.

Directional summary:

```text
Musi value held by host/native side: Root[T]
Host-owned value held by Musi:       Host[T]
Raw physical address:                RawPtr[T] / RawPtr[mut T]
```

## Host imports and modules

Existing import syntax from the 1.0 candidate pack:

```musi
let math := import "std/math";
```

Imports are bound through ordinary `let` bindings.

There is no standalone import declaration syntax in this addendum.

Host-provided modules follow the same import-expression discipline when surfaced to source code:

```musi
let engine := import "host/engine";
```

The above is an existing import shape applied to a host module path. The host path vocabulary is host/environment-defined.
