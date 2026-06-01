# 06. Patterns and Match

Status: normative.

## Principle

Patterns are structural. They do not run arbitrary code, do not call functions, and do not use name-resolution tricks to decide whether an identifier is a binder or a constant.

Bare names bind.

Constructors and variants are dotted.

## Pattern grammar

```text
Pattern :=
  AliasPattern

AliasPattern :=
  OrPattern
  OrPattern as name

OrPattern :=
  PrimaryPattern
  PrimaryPattern or PrimaryPattern ...

PrimaryPattern :=
  _
  name
  mut name
  literal
  .Variant
  .Variant(Pattern, ...)
  Type.Variant
  Type.Variant(Pattern, ...)
  #(...)
  #[...]
  #{...}
  (Pattern)
```

Precedence:

```text
primary pattern > or > as
```

So:

```musi
.A(x) or .B(x) as value
```

means:

```musi
(.A(x) or .B(x)) as value
```

Plain parentheses group patterns. They do not make tuple patterns.

## Wildcard

```musi
_
```

matches anything and binds nothing. `_` is not a name.

## Binding patterns

```musi
name
mut name
```

A bare identifier in pattern position is a binder.

```musi
let #(x, mut y) := pair;
     -  -----
     x  mutable binder y
```

`mut` applies only to binders.

Invalid:

```musi
let mut #(x, y) := pair;
```

Reason: `mut` does not apply to a whole pattern.

## Literal patterns

Literal patterns match the literal value exactly.

```musi
match code (
| 0n32 => "zero"
| 1n32 => "one"
| _ => "many"
)
```

A bare identifier is not a constant pattern.

```musi
match value (
| expected => expected
)
```

Here `expected` binds the matched value. To compare with an existing value, use a guard:

```musi
match value (
| x when x = expected => x
| _ => fallback
)
```

## Sum variant patterns

Variant patterns are dotted.

```musi
.Some(value)
.None
.Success(value)
.Failure(error)
```

Qualified form is valid when required or clearer:

```musi
Maybe.Some(value)
Expect.Failure(error)
```

Bare variant names are invalid.

```musi
Some(value)
```

## Tuple datum patterns

Tuple datum patterns use `#(...)`.

```musi
let #(x, y) := pair;
```

Tuple datum patterns are exact-arity.

```musi
let #(x, _, z) := point3;
```

There is no tuple rest pattern. Use `_` to ignore fixed positions.

## Sequence datum patterns

Sequence patterns use `#[...]`.

Exact-length:

```musi
#[first, second]
```

Rest:

```musi
#[first, ...rest]
```

Ignore rest:

```musi
#[first, ...]
```

Rules:

```text
... may appear at most once.
... must be last.
...name binds the remaining sequence.
bare ... ignores the remaining sequence.
```

A sequence pattern without `...` is exact-length.

## Record datum patterns

Record patterns use `#{...}`.

```musi
#{x := px, y := py}
```

Shorthand:

```musi
#{x}
```

desugars to:

```musi
#{x := x}
```

Mutable binder shorthand:

```musi
#{mut x}
```

desugars to:

```musi
#{x := mut x}
```

Rest:

```musi
#{head, ...rest}
```

Ignore rest:

```musi
#{head, ...}
```

Record patterns are exact unless `...` appears.

Rules:

```text
duplicate fields are rejected;
unknown fields are rejected when the target shape is known;
... may appear at most once;
... must be last syntactically;
bare ... ignores remaining fields;
...name binds remaining fields.
```

## Pattern aliasing

Only this form is valid:

```musi
PATTERN as name
```

Meaning: match `PATTERN` normally and also bind the whole value matched by `PATTERN` as `name`.

Example:

```musi
.Some(x) as whole
------      -----
inner       whole matched .Some value
binding
```

Invalid:

```musi
name as PATTERN
PATTERN as mut name
```

The alias binder is immutable. Mutable pieces are bound inside the pattern.

Duplicate binding is invalid:

```musi
.Some(x) as x
```

## Or-patterns

`or` combines alternatives.

```musi
.A(x) or .B(x)
```

Every branch of an or-pattern must bind the same names with compatible types and mutability.

Valid:

```musi
.A(x) or .B(x)
```

Invalid:

```musi
.A(x) or .B(y)
```

Invalid:

```musi
.A(mut x) or .B(x)
```

## Match arms

Match arms are pipe-prefixed.

```musi
match value (
| PATTERN when condition => result
| PATTERN => result
)
```

Pattern matching happens first. Pattern bindings enter scope for the guard. The guard must have type `Bit`. If the guard is false, matching continues to later arms.

Guarded arms do not prove exhaustiveness by themselves. A total match still needs unguarded coverage.

Example:

```musi
match maybeName (
| .Some(name) when name /= "" => name
| .Some(_) => "anonymous"
| .None => "anonymous"
)
```

## Irrefutable and refutable contexts

Plain `let PATTERN := value;` requires an irrefutable pattern for the static type of `value`.

Refutable patterns are handled with `match`.

```musi
match findUser(id) (
| .Some(user) => user.name
| .None => "anonymous"
)
```
