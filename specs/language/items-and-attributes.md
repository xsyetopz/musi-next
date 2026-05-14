# Builtin Items, Intrinsics, And Attributes

Status: current small-core draft

`known` is source syntax for compiler-known phase information. Reserved
`@musi.*` attributes mark compiler-owned foundation bindings and intrinsics.

## Builtin Items

Compiler-owned builtin bindings use reserved namespaced metadata:

```musi
@musi.builtin(name := "Type")
export let Type := Type;
```

Rules:

- `@musi.builtin` is valid only on exported foundation bindings.
- The source name is tied to the internal builtin registry by `name`.
- Library items such as `Maybe`, `Expect`, and helpers remain ordinary
  library values unless the compiler registry owns them.

## Intrinsics

Irreducible runtime leaves use reserved intrinsic metadata:

```musi
@musi.intrinsic(name := "ptr.load")
let ptrLoad(ptr : CPtr) : Int;
```

Source-facing APIs should wrap intrinsic-backed leaves in ordinary Musi code.

## Public Attributes

Accepted public source attributes:

```text
@deprecated
@skip
@layout
@external
```

Attribute names and keys use camelCase where needed.

## `@deprecated`

Lifecycle metadata for APIs and tooling.

```musi
@deprecated(
  use := "readFile",
  note := "renamed"
)
let readAll := ...;
```

## `@skip`

Explicit skipped compiler or verifier checks.

```musi
@skip(
  checks := [.bounds],
  why := "caller already checks index"
)
```

## `@layout`

Memory or storage representation metadata.

```musi
@layout(
  form := .packed,
  align := 1
)
let Header := data {
  let tag : Byte;
  let size : Word;
};
```

## `@external`

External linkage uses `@external` on ordinary declarations. Direction is visible
from `export` and body presence.

Rules:

- `@external` plus a declaration without a body imports an external implementation.
- `@external` plus `export` plus a body exports an external entry point.
- `export` without `@external` is a public Musi API only.
- `@external` without `export` but with a body is invalid.
- `@external` with `export` but without a body is invalid in the core.
- Do not duplicate direction with `mode`, `import`, or `export` keys.

The body may include external facts not already visible in source, such as ABI,
external name, and stack effect.
