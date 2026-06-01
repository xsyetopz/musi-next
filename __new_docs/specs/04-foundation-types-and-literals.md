# 04. Foundation Types and Literal Policy

Status: normative for source names used by this pack.

## Core foundation names

```text
Unit
Empty
Bit
Type / Type[n]
Any
Word[N]
Nat[N]
Int[N]
Real[N]
```

Surface numeric aliases include:

```text
Nat8 Nat16 Nat32 Nat64
Int8 Int16 Int32 Int64
Real32 Real64
```

## Names not used

Musi does not use the following core names:

```text
Bool
Byte
String
Char
```

Use:

```text
Bit
Nat8
Text
Rune
```

## Bit values

`true` and `false` are ordinary values of type `Bit`.

They are not keywords.

## Empty

`Empty` is the bottom type used for non-continuing local control expressions.

Examples:

```text
exit : Empty
next : Empty
```

`Empty` can inhabit a position because execution does not continue at that expression.

## Maybe and Expect

```text
?T  = Maybe[T]
E!T = Expect[T, E]
```

Constructors are dotted:

```text
.Some(value)
.None
.Success(value)
.Failure(error)
```

There are no bare `Some`, `None`, `Success`, or `Failure` constructors.

## No null

`null` is not a keyword and not a value.

Absence is represented by `?T = Maybe[T]`.

## Type/value system closure points

Musi uses a type/value model. Values include ordinary runtime values, known values, type-values, callable values, imported value-like surfaces, and non-returning/bottom results where applicable.

A type-value is a value that may classify/check other values in type position where admitted by the source rules.

`Empty` is bottom/non-returning. It is not `Unit`. It can typecheck in a position that expects a value because execution does not continue from that expression.

`Unit` is the ordinary completed no-information value.

The specification must state the restrictions that prevent type/value collapse, including the role of `Type / Type[n]`, without importing another language's `Type`, `Never`, `!`, `constexpr`, `consteval`, or `comptime` semantics as Musi law.
