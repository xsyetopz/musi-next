# Rejected Constructs

Status: reference.

## Rejected keywords

```text
if / then
  replaced by guarded values: a when c else b

return
  replaced by final-expression body result

null
  replaced by ?T = Maybe[T]

async / await / spawn
  library/runtime concepts over yield and Resumable, not keywords

for
  no primitive loop; iteration is library/trait behavior over while and collections

foreign
  FFI uses @foreign(...), not a keyword or block syntax

try / catch
  Expect uses explicit methods and pattern matching

module
  files evaluate to module records

class / impl / instance
  use data / trait / evidence

fun / fn / def
  use let
```

## Rejected spellings

```text
break / cycle
  not chosen; loop control uses exit / next

continue
  not used

as for casting
  invalid; casts are :?, :?>, :>

#[...] attributes
  invalid; attributes use @name / @name(...)
```

## Not introduced by the 1.0 syntax hardening pass

```text
::
  not introduced by known, fixed, import, type-values, or dotted variants

separate namespace declarations
  not introduced where ordinary let-bound exported/imported values and dot selection already express the model

Swift/Rust/Zig/C++/Python syntax imports
  external languages may be useful comparisons, but their spellings and semantics are not Musi law
```
