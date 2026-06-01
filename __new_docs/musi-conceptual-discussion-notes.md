# Musi Conceptual Discussion Notes

Status: **Conceptual only**
Scope: keyword/operator/attribute pressure, staged metaprogramming, low-level/system-adjacent features, and SEIL direction.
Not applied to the formal Musi specification.

## 1. Design posture

Musi is intended to remain a small, opinionated, embeddable interpreted language with a custom stack-effect VM, granular bidirectional typing, and typed staging/metaprogramming.

The core design pressure is:

```text
small surface
large capability
minimal reserved syntax
strong verifier/runtime semantics
few first-class constructs that unlock many use cases
```

The language should avoid copying C-family, B-family, or arbitrary foreign syntax patterns. Existing Musi syntax and conventions should control any proposed surface form.

## 2. SEIL direction

A possible long-term direction is to treat SEIL as the canonical executable/verifiable VM language, closer to a CIL-like model.

Conceptual model:

```text
Musi source
→ Musi core / elaborated form
→ SEIL canonical artifact
→ SEAM runtime
```

Under this model, a separate SEIL/SEBC semantic split may be unnecessary. A binary format could exist, but only as a binary encoding of SEIL, not as a separate semantic layer.

Open conceptual distinction:

```text
SEIL textual form
SEIL binary encoding
SEIL verifier metadata
SEIL runtime semantics
```

## 3. Keyword budget

Target: maximum 25 reserved keywords.

Current discussion considered reducing reserved keywords by moving non-core concepts into operators, attributes, UDTs, or contextual grammar.

Conceptual candidate keyword set:

```text
as
data
defer
else
exit
export
fixed
import
in
known
let
match
mut
next
pin
trait
when
while
yield
```

Count: 19.

This is conceptual only.

Words considered removable from the reserved keyword set:

```text
and
or
not
xor
unsafe
erased
opaque
where
with
```

Rationale:

```text
and/or/not/xor → symbolic operators
unsafe          → explicit unsafe/unchecked APIs, effects, attributes, or UDTs
erased          → attribute or type/evidence metadata
opaque          → attribute or boundary metadata
where/with      → contextual grammar words if possible
```

## 4. Fixed and known

`fixed` and `known` remain strong keyword candidates.

Conceptual meanings:

```text
fixed = storage / placement / lifetime class, STATIC-like in broad role
known = compile-time phase, comparable in role to comptime-style evaluation
```

They are not passive metadata. They affect legality, phase, storage, lifetime, and lowering.

## 5. Operators

Operators should not inherit C meanings by default.

Conceptual rule:

```text
operators express compact expression composition or algebraic relations
operators do not grant authority
operators do not imply raw memory access
operators do not imply native machine behavior
```

Accepted / existing discussion point:

```text
|> = pipeline / value-threading operator
```

Boolean word operators may be replaced by symbolic operators.

Conceptual symbolic family:

```text
&   conjunction / AND
|   disjunction / OR
~   negation / NOT
^   exclusive OR
~&  NAND
~|  NOR
~^  XNOR
```

Important constraint:

```text
These should not be C bitwise operators by default.
```

A better Musi framing is:

```text
Bit algebra operators
truth-control positions
explicit bit-collection types
```

Rather than:

```text
logical operators vs bitwise operators
```

Open issue:

```text
~ is currently staging-related in the spec.
If ~ becomes NOT, typed staging quote/splice syntax must move.
If ~ remains staging-related, NOT needs another spelling.
```

## 6. Bit / truth model

Conceptual direction:

```text
Bit = core truth value and one-bit algebra value
```

Condition positions may require `Bit`, avoiding broad truthiness.

Bit collections should not automatically become conditions.

Examples of conceptual types:

```text
Bit
BitVec
Mask
BitSet
```

Integers should not automatically inherit C-style bitwise behavior. If integer bit operations exist, they should be explicit through evidence, views, masks, or bit-vector conversion.

## 7. Attributes

Attributes should be self-documenting and Musi-shaped.

Existing documented attribute syntax uses:

```musi
@name
@name(...)
```

Named slots use Musi’s existing `:=` form where admitted.

Existing documented examples include:

```musi
@foreign(abi := .cdecl, name := "foreign_name")
@target(os := .macos, arch := .aarch64, features := #[.simd, .neon])
```

Conceptual attribute naming rule:

```text
name the consequence, not the tradition
```

Do not introduce borrowed payload syntax.

Potential attribute roles, conceptual only:

```text
@erased      runtime/type/evidence erasure
@opaque      hidden representation or API-boundary opacity
@unchecked   user-asserted proof obligation
@unmanaged   lifetime not managed by Musi runtime
```

Possible grouped attribute heads, conceptual only:

```text
@layout      representation/layout contract
@requires    environment/runtime/target requirement
@does_not    verifier-enforced absence of an effect
```

These are not applied.

## 8. Unsafe

A possible direction is to remove `unsafe` as a keyword.

Instead of a broad unsafe block or unsafe mode, danger could be visible at the exact use site through:

```text
Unsafe* UDTs
Unchecked* operations
@unchecked attributes
effect-visible obligations
capability requirements
```

Conceptual examples of categories, not final syntax:

```text
UnsafePtr
UncheckedLoad
UncheckedStore
UnmanagedHost
```

Goal:

```text
danger is local, visible, and typed
```

rather than hidden in a large lexical unsafe region.

## 9. System-adjacent features

Inline assembly should not become free-form source syntax.

Conceptual replacement:

```text
target primitive
VM primitive
host primitive
compiler intrinsic
```

These would be verifier/runtime-known operations, not arbitrary text.

Pointer arithmetic should not become C-style pointer arithmetic.

Conceptual replacement:

```text
UnsafePtr / raw address token
Region
Cursor
Offset
ByteRegion
Load / Store / UncheckedLoad / UncheckedStore
```

The source language should avoid:

```text
ptr + n
*ptr
&value
->
```

unless Musi independently defines equivalent semantics without importing C’s model.

## 10. Typed staged metaprogramming

Typed staging is a major conceptual design area and should not be treated as a toy macro layer.

Core conceptual roles:

```text
known = compile-time phase
quote = typed staged code construction, if adopted as a keyword
splice = source-position insertion / staged escape
# = datum/literal/pattern family
$ = template interpolation
\ = lambda
```

Current issue:

```text
~ is currently used by staging-related syntax.
~ is also attractive as ASCII NOT.
These conflict.
```

The discussion remains open.

Important staged metaprogramming questions:

```text
What is the type of quoted code?
Does quote produce source syntax, typed core, or SEIL-level code?
What positions can be spliced into?
Are generated names hygienic by default?
Can staged code intentionally capture names?
Can known code inspect types and evidence?
Can known code emit declarations?
Can staged code be stored and composed as ordinary values?
Does staging run through the same SEAM/SEIL machinery?
```

Conceptual possible types:

```text
Code[T]
Datum
Syntax
```

No final syntax selected.

## 11. UDTs and intrinsics

Conceptual direction:

```text
use UDTs for value-level powers
use intrinsics for SEIL/SEAM-known operations
use attributes for metadata and verifier obligations
reserve keywords for grammar, phase, binding, control, and lifetime
```

Possible UDT categories:

```text
Code
Datum
UnsafePtr
Region
Cursor
Offset
Host
Root
UnmanagedHost
AtomicCell
DeviceCell
Register
```

Possible intrinsic categories:

```text
Load
Store
UncheckedLoad
UncheckedStore
UncheckedReinterpret
SizeOf
AlignOf
OffsetOf
Trap
Fence
AtomicLoad
TargetOp
HostCall
```

Names are conceptual unless already defined elsewhere.

## 12. Non-applied status

Nothing in this document is an applied Musi specification change.

This document only records the current conceptual discussion so that a separate project/spec branch can evaluate these ideas without contaminating the current formal source specification.
