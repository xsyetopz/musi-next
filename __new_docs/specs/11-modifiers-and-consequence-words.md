# 11. Modifiers and Consequence Words

Status: normative for source spelling and consequence semantics.

## Consequence vocabulary

```text
known   compiler-known / known during compilation
fixed   fixed storage / fixed placement / fixed lifetime
mut     write authority
opaque  representation opacity
erased  existential erasure
unsafe  unchecked/manual obligation boundary
pin     temporary pinning action for address stability
```

`static` and `pinned` are not Musi keywords.

## known

`known` means the value is known during compilation.

```musi
known let wordBits := 64n64;

let Buffer[known N : Nat64] := data {
  let storage : Array[Nat8, N];
};
```

`known` is a modifier keyword, not an attribute.

`known` is not storage duration, not immutability, and not namespace syntax.

## fixed

`fixed` means fixed storage / fixed placement / fixed lifetime.

```musi
fixed let table := makeTable();
fixed mut counter := 0n32;
```

`fixed` is not compile-time knowledge. Use `known` for compiler-known values.

A value may be `known` without being `fixed`, `fixed` without being `known`, both, or neither.

```musi
let schema := known parseSchema("Enemy");

fixed let table := known makeLookupTable();
```

## mut

`mut` grants write authority to a binding, parameter, pattern binder, or view where admitted.

```musi
let mut count := 0n32;
count := count + 1n32;
```

Without `mut`, rebinding/shadowing is allowed by a new `let`, but writing through the existing binding is not.

## opaque

`opaque` hides representation at the boundary where it is exported or returned.

It is representation opacity, not existential erasure.

## erased

`erased Trait` is an existential package: value plus trait evidence.

It is not `Any`.

## unsafe

`unsafe` is a modifier keyword, not an attribute.

It marks a lexical/manual obligation boundary for operations the verifier or type checker cannot fully prove.

```musi
unsafe (
  rawPtr.write(value);
)
```

`unsafe` does not pin, root, extend lifetimes, suppress GC, bypass write barriers, or turn `Any` into unchecked dynamic behavior by itself.

Unsafe should be as narrow as possible.

## pin

`pin` is a lexical action that temporarily provides address stability for a movable value.

Locked form:

```musi
pin value as name (
  body
)
```

Example:

```musi
pin buffer as p (
  unsafe (
    foreignWrite(p.ptr, p.len);
  );
)
```

Semantics:

```text
pin value for lexical duration of body;
as name binds the pinned regional view;
pin is released after body exits;
the alias cannot escape unless its type explicitly permits it.
```

`as` here is name aliasing, not casting.

`pin` does not grant unsafe authority. `unsafe` does not pin.

## known and fixed are distinct

`known` means a value the compiler knows before runtime reaches it.

`fixed` means storage/placement/lifetime is fixed by declaration or storage domain.

Neither keyword introduces namespace syntax. Neither keyword changes member-selection spelling. Selection from known values, fixed values, imports, and ordinary values uses ordinary selection syntax.
