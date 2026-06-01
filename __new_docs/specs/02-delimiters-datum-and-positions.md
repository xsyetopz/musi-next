# 02. Delimiters, Datum Forms, and Syntactic Positions

Status: normative.

## Delimiter roles

```text
(...)      computation / grouping / tuple type in type position
{...}      structural form
[...]      type arguments / stack effects / index contexts where admitted
#(...)     datum tuple literal / datum tuple pattern / datum requirement set
#[...]     datum sequence literal / datum sequence pattern
#{...}     datum record literal / datum record pattern
```

`#(`, `#[`, and `#{` are compound openers.

## Type position and value position

A syntactic position determines what kind of term is accepted.

```text
Type position expects a type.
Value position expects a value/literal term.
```

The two are not interchangeable.

```musi
let text : Text := "hello";
         ----    -------
         type    value position
```

Valid:

```musi
let pair : (K, V) := #(key, value);
          ------    -------------
          type      datum value
```

Invalid:

```musi
let value : #("Text") := Text;
            ---------    ----
            datum        type name in value position
            invalid      invalid
```

## Tuple type and datum tuple

Tuple type syntax and datum tuple syntax are distinct.

```musi
(K, V)
```

is tuple type syntax in type position.

```musi
#(key, value)
```

is datum tuple syntax in value/pattern/datum positions.

A datum tuple does not satisfy a tuple type position. A tuple type does not satisfy a value/datum position.

## Computation blocks

A computation block uses parentheses.

```musi
(
  let x := 1n32;
  let y := 2n32;
  x + y
)
```

A computation block is a sequence. Its result is governed by the sequencing rules in Chapter 03.

## Structural braces

Plain braces are structural.

```musi
let Point := data {
  let x : Real64;
  let y : Real64;
};
```

Plain braces are not computation blocks.

## Datum record

Datum records use `#{...}`.

```musi
let point := #{x := 1r64, y := 2r64};
```

## Datum sequence

Datum sequences use `#[...]`.

```musi
let values : Vec[Nat32] := #[1n32, 2n32, 3n32];
```
