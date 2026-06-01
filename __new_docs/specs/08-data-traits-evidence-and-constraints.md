# 08. Data, Traits, Evidence, and Constraints

Status: normative for source syntax and constraint shape.

## Data

`data` defines representation.

Product data:

```musi
let Point := data {
  let x : Real64;
  let y : Real64;
};
```

Sum data:

```musi
let Maybe[T] := data {
| Some(value : T)
| None
};
```

A `data` body is product-shaped or sum-shaped. It does not mix top-level product fields and variants.

## Dotted construction

Sum variants are constructed and matched with dotted syntax.

```musi
.Some(value)
.None
.Success(value)
.Failure(error)
```

## Traits and evidence

`trait` defines a behavioral/evidence contract.

```musi
let Equatable[T] := trait {
  let equals(left : T, right : T) : Bit;
};
```

The equality trait is named `Equatable`.

Ordering is named `Ordering`.

Membership uses `Contains`.

```musi
let Contains[C, I] := trait {
  let contains(container : C, item : I) : Bit;
};
```

## Evidence constraints

`where` introduces type/evidence constraints.

`where` is not a runtime/value guard.

Single constraint:

```musi
where T |= Ordering
```

Single subject with multiple requirements:

```musi
where T |= #(Ordering, Show)
```

Tuple subject with tuple requirement:

```musi
where (K, V) |= (Ordering, Show)
```

Tuple subject with a datum requirement set for one tuple element:

```musi
where (K, V) |= (#(Ordering, Show), Show)
```

## Type position and value position in constraints

The same type/value position rule applies to constraints.

```text
Type position expects a type.
Value/datum position expects a value/datum term.
```

Valid:

```musi
where T |= #(Ordering, Show)
      -    ----------------
      type datum requirement set for one subject
```

Invalid:

```musi
where (K, V) |= #(Ordering, Show)
      ------    ----------------
      tuple     datum requirement set
      subject   invalid here
```

Reason: a tuple type subject expects a tuple-shaped satisfier. `#(...)` is a datum requirement set, not a tuple type.

Valid:

```musi
where (K, V) |= (#(Ordering, Show), Show)
      ------    ------------------------
      tuple     tuple-shaped satisfier
      subject
```

## Shape-preserving satisfaction

Constraint satisfaction is shape-preserving.

```text
Tuple subjects require tuple satisfiers of matching arity.
Record subjects require record satisfiers with matching required fields.
Datum requirement sets group multiple requirements for one subject.
No implicit broadcasting, flattening, or conversion occurs inside constraint satisfaction.
```

## Constraint forms

Core constraint relations:

```text
T |= Requirement     evidence/trait satisfaction
T <: U               subtype/refinement relation
A ~= B               type equivalence
```

These are constraint judgments, not runtime expressions.

## Data slots and `:=`

`data` is an RHS form bound by `let`.

Product fields and sum variants use the same named-slot/value discipline as parameters and calls. Where a field, variant, or payload admits an attached value/default, that attachment uses `lhs := rhs`.

The body shape determines whether a `data` body is product-shaped or sum-shaped. A product-shaped body contains field-style slots. A sum-shaped body contains variant arms. They are not mixed at the same top level.

The same named-slot rule applies across construction and matching: a named field or variant payload slot may be supplied positionally where the corresponding form admits positional supply, or directly by name with `lhs := rhs` where the corresponding form admits tagged supply.

A slot may not be filled twice. A required slot may not be omitted. Defaulted slots may be omitted.
