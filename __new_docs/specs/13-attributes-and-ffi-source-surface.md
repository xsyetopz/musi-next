# 13. Attributes and FFI Source Surface

Status: normative for source spelling.

## Attribute syntax

Attributes use Swift-style attachment syntax.

Without arguments:

```musi
@name
```

With arguments:

```musi
@name(...)
```

No attribute arguments are allowed without parentheses.

No Rust-style `#[...]` attributes exist.

`#` remains datum/literal/pattern syntax.

## Attribute argument values

Attribute arguments use ordinary Musi value syntax where admitted.

They may include:

```text
scalar literals
text/rune literals
names / paths
#(...)
#[...]
#{...}
.Variant(...)
nested ordinary attribute values
```

Example:

```musi
@target(os := .macos, arch := .aarch64, features := #[.simd, .neon])
let platformName : Text := "macOS ARM64";
```

## inline

```musi
@inline
```

means inline mode `.always`.

Explicit forms:

```musi
@inline(mode := .always)
@inline(mode := .never)
```

## known and unsafe are not attributes

`known` and `unsafe` are modifier keywords.

Invalid:

```musi
@known
@unsafe
```

## FFI source surface

FFI uses `@foreign(...)`.

There is no `foreign` keyword and no foreign block syntax.

Foreign import into Musi:

```musi
@foreign(abi := .cdecl, name := "foreign_name")
let f(x : T) : U;
```

This imports a foreign symbol into Musi because it is a bodyless `let` under `@foreign`.

Foreign export from Musi:

```musi
@foreign(abi := .cdecl, name := "foreign_name")
export let f(x : T) : U := (
  body
);
```

This exports a Musi function to the foreign boundary because it is `export let` with a body under `@foreign`.

The ABI mechanics, object layout, handle representation, rooting mechanics, and calling-convention lowering are implementation/runtime specifications.

## target

`@target(...)` is the source form for target metadata.

The source grammar admits ordinary attribute values. The target vocabulary belongs to the target catalog, not this grammar chapter.
