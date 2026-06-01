# FFI / Layout / Retention Syntax Candidates

Status: candidate syntax notes only.

This file is not a normative syntax chapter. It records candidate surface forms that follow the existing syntax anchors where possible.

Existing syntax anchors from the 1.0 candidate pack:

```musi
@name
@name(...)
```

```musi
@foreign(abi := .cdecl, name := "foreign_name")
let f(x : T) : U;
```

```musi
let name := import "path";
```

```musi
let Point := data {
  let x : Real64;
  let y : Real64;
};
```

```musi
pin value as name (
  body
)
```

```musi
unsafe (
  body
)
```

## Candidate type names

Confirmed semantic names:

```text
RawPtr[T]
RawPtr[mut T]
Root[T]
Host[T]
```

`RawPtr[T]` / `RawPtr[mut T]` replace `Ptr[T]` / `Ptr[mut T]`.

`Root[T]` is the VM-owned stable token for a Musi-managed value.

`Host[T]` is the host-owned resource/object/service/value token.

## Candidate layout attribute

Candidate:

```musi
@layout(preset := .c)
let CPoint := data {
  let x : Real64;
  let y : Real64;
};
```

Candidate:

```musi
@layout(preset := .bytes, padding := .none, alignment := .byte)
let PacketHeader := data {
  let tag : Nat16;
  let size : Nat32;
};
```

Notes:

- `@layout` is candidate attribute spelling.
- `preset` is candidate field spelling.
- `.c`, `.bytes`, `.none`, `.byte` are candidate variant values.
- Ordinary Musi layout has no source layout attribute.
- This file does not define a final attribute grammar.

## Candidate compact retention attribute

Candidate default-retention override:

```musi
@retains(callback, state, until := unregisterEvent)
```

Candidate use:

```musi
@foreign(abi := .cdecl, name := "register_event")
@retains(callback, state, until := unregisterEvent)
let registerEvent(
  callback : Callback[(Host[Event], Root[State]) -> Unit],
  state : Root[State],
) : Host[EventRegistration];
```

Candidate release:

```musi
@foreign(abi := .cdecl, name := "unregister_event")
let unregisterEvent(registration : Host[EventRegistration]) : Unit;
```

Notes:

- `@retains` is the candidate attribute spelling for non-default retention.
- The compact intended shape is retained names plus an end condition.
- Extra policy fields are not part of the common form.
- Thread/reentry policy remains default unless a declaration needs to override it.
- `Callback[...]` is candidate callback type spelling, not existing syntax from the uploaded pack.

## Candidate host binding stub

Candidate:

```musi
@host(name := "engine.spawn")
let spawn(name : Text) : Host[Entity];
```

Notes:

- `@host` is a candidate marker for a VM-aware host binding stub.
- It is not raw FFI.
- Calling a host binding does not inherently require `unsafe`.
- Host modules may also be surfaced by ordinary `let name := import "...";` bindings.

Existing import binding shape applied to a host module path:

```musi
let engine := import "host/engine";
```

## Non-moving storage naming

No source spelling is selected in this file.

Semantic requirement:

```text
explicit non-moving storage exists as a low-level boundary feature
```

Any future name must be marked as candidate unless sourced from a later accepted spec.
