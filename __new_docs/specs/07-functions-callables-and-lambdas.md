# 07. Functions, Callables, and Lambdas

Status: normative for source syntax.

## Named callable binding

Functions are introduced with `let`.

```musi
let id[T](value : T) : T := value;
```

The result is the final expression of the body.

```musi
let classify(score : Nat32) : Text := (
  "high" when score >= 80n32 else
  "mid" when score >= 50n32 else
  "low"
);
```

There is no `return` keyword.

## Callable type syntax

Callable type syntax is:

```text
(Arg) -> Result
(Arg1, Arg2) -> Result
() -> Result
```

Examples:

```musi
let f : (Int32) -> Int32 := \(x : Int32) : Int32 => x;
let combine : (Text, Text) -> Text := \(a : Text, b : Text) : Text => a + b;
```

## Lambda syntax

Lambda syntax:

```musi
\(x : Int32) : Int32 => x
```

A lambda body follows the same final-expression rule as named functions.

```musi
let abs : (Int32) -> Int32 :=
  \(x : Int32) : Int32 => (
    x when x >= 0n32 else -x
  );
```

## Body result and semicolon

Valid:

```musi
let f() : Text := (
  "done"
);
```

Invalid:

```musi
let f() : Text := (
  "done";
);
```

Reason: the final semicolon discards the string and makes the body `Unit`.

## Higher-order values

Callable values are ordinary values and can be passed as arguments.

```musi
let apply[T, U](f : (T) -> U, value : T) : U := f(value);
```

## Parameters, defaults, and tagged arguments

A parameter slot has a name and type.

```musi
let callHere(fieldNamed : Int) := ...;
```

A positional argument fills the next unfilled parameter slot from left to right.

A tagged argument fills the named parameter slot directly:

```musi
callHere(value);
callHere(fieldNamed := value);
```

For the callable above, those two calls fill the same parameter slot.

Tagged arguments are closest in role to Python keyword arguments, but Musi uses its own `lhs := rhs` slot syntax rather than `=` or `:`.

A parameter may provide a default value with `:=`:

```musi
let draw(width : Int, height : Int, scale : Int := 1) := ...;
```

Defaulted parameters must not appear before non-defaulted parameters in the same positional parameter sequence.

Valid:

```musi
let draw(width : Int, height : Int, scale : Int := 1) := ...;
```

Invalid:

```musi
let draw(width : Int := 640, height : Int) := ...;
```

Reason: a defaulted parameter precedes a required parameter. This would make positional calls ambiguous as to which parameter is being supplied.

A parameter slot may be filled at most once. A required parameter slot must be filled by either a positional argument or a tagged argument. A defaulted slot may be omitted.
