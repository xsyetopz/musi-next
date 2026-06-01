# 05. Conditionals, Guards, and Absence

Status: normative.

## No if / then

`if` and `then` are not keywords.

Musi conditionals use guarded values.

## Full guarded conditional

```musi
a when condition else b
```

Type rule:

```text
condition : Bit
a : T
b : T or joins with a to T

result : T
```

Example:

```musi
let label : Text :=
  "adult" when age >= 18n32 else "minor";
```

Chained form:

```musi
let sign : Text :=
  "positive" when x > 0n32 else
  "negative" when x < 0n32 else
  "zero";
```

`when ... else` is right-associative.

```musi
a when c1 else b when c2 else z
```

means:

```text
a when c1 else (b when c2 else z)
```

## Bare guarded value

```musi
a when condition
```

Type rule:

```text
condition : Bit
a : T

result : ?T
```

Meaning:

```text
.Some(a) if condition is true
.None    if condition is false
```

Marker:

```musi
index when found
-----      -----
T          Bit

result type: ?T
```

## Bare guarded effect/control in discarded position

When immediately discarded by `;`, a bare guarded expression is a guarded effect/control sequence item.

```musi
cleanup() when opened;
```

Meaning:

```text
if opened is true, evaluate cleanup() and discard its value;
if opened is false, do nothing;
the sequence item produces Unit.
```

Control examples:

```musi
exit when done;
next when skipped;
```

These do not produce `?Empty` because the enclosing sequence item discards the guarded expression.

## Precedence

`when ... else ...` has lower precedence than mathematical operators, comparison operators, logical operators, `??`, and pipeline.

`else` binds to the nearest unmatched `when`.

## Match guards

Match guards use `when`.

```musi
match value (
| pattern when condition => result
| pattern => fallback
)
```

`where` is not a runtime/value guard keyword.
