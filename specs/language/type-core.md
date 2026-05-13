# Core Type Model

Status: current small-core draft

## Primitive And Library Types

`Bit` is the primitive condition type. Conditions require `Bit`; there is no
truthiness for integers, arrays, strings, unit, or data values.

`Bool` is a standard-library item, not a primitive keyword. It binds `.True` to
bit value `1` and `.False` to bit value `0`, and exports lowercase aliases
`true` and `false`.

`Unknown` is an inference or recovery type. `Unit` has exactly one value and is
written `()`. `Empty` is uninhabited.

## Data Shapes

`data` defines concrete data. Product fields use `let` members with semicolon
terminators:

```musi
let Buffer := data {
  let ptr : Ptr[mut Byte];
  let len : Nat;
};
```

Sum variants use `|` alternatives without semicolons:

```musi
let Maybe[T] := data {
| Some(value : T)
| None
};
```

Do not mix product fields and sum variants in one `data` body.

## Mutability

`mut` is local to the type or value on its right.

```musi
let x := mut 0;
let x : mut Int := mut 0;
let p : Ptr[mut Byte] := getMutablePtr();
let q : mut Ptr[mut Byte] := mut getMutablePtr();
```

An explicit mutable annotation does not make an immutable initializer mutable.

## Absence And Failure

Absence and failure are algebraic data:

```musi
?T   == Maybe[T]
E!T  == Expect[T, E]
```

`??` is Maybe fallback only. Failure is handled with `match`, `let ... else`,
or named helpers.

## Callable Types

`A -> B` is a callable type. Declarations use `:` for result types.

```musi
let add(a : Int, b : Int) : Int := a + b;
let f : Int -> Int := \(x : Int) => x;
```

## Ranges

Range expressions are values:

```musi
a ..< b   -- half-open [a, b)
a .. b    -- inclusive [a, b]
```

`...` remains spread/splat syntax and is lexed before range tokens.
