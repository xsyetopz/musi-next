# 09. Operators, Membership, and Casts

Status: normative for source spelling and semantic family.

## Mathematical operator precedence and conflict policy

Musi operator precedence follows mathematical convention for the mathematical operator layer.

The specification must define precedence and associativity for the mathematical operators admitted by the language, including unary-vs-binary disambiguation and the relation of mathematical operators to call/member/index postfix forms, `??`, `|>` where admitted, `when ... else`, and `:=` contexts.

Parser conflicts are language-design failures. They are not resolved by parser-generator defaults, hidden precedence repairs, semantic predicates, arbitrary lookahead, or name/type resolution.

User-defined symbolic operators are not admitted.

Operator spellings are fixed by the language. Traits provide semantics through evidence where appropriate.

## Logical words

```text
and  short-circuit Bit conjunction
or   short-circuit Bit disjunction, and also pattern alternative in pattern position
not  Bit negation
xor  Bit exclusive-or
```

## Binding and equality

```text
:=   binding / write / named slot
=    equality
/=   inequality
```

`=` is not binding.

Equality behavior is backed by `Equatable` evidence.

## Membership

`in` is mathematical set membership.

```musi
item in container
```

Type:

```text
Bit
```

Membership is backed by `Contains` evidence.

Canonical semantic shape:

```text
Contains[C, I].contains(container, item)
```

for:

```musi
item in container
```

Important rules:

```text
in is not iteration syntax.
There is no primitive for loop.
not in is not a compound keyword.
Use ordinary negation around membership.
```

Example:

```musi
not (item in banned)
```

## Maybe fallback

`??` is Maybe-only fallback.

```musi
maybeName ?? "anonymous"
```

It is not an `Expect` fallback operator.

## Cast and type-test operators

```text
value :? T   -> Bit
value :?> T  -> ?T
value :> T   -> T
```

Rules:

```text
:? is a runtime type test.
:?> is an optional/conditional cast.
:> is a required cast.
```

`:?>` returns `.Some(value-as-T)` on success and `.None` on failure.

`:>` returns `T` on success. If the cast is dynamically checked and fails, it traps.

`as` is never a cast operator.

## Expect methods

`E!T = Expect[T, E]`.

Constructors:

```text
.Success(value)
.Failure(error)
```

There is no `try` keyword and no `catch` keyword.

Canonical methods:

```text
valueOr
valueOrElse
map
mapFailure
bind
recover
```

`then` is excluded to avoid duplicating `bind`.

Method meanings:

```text
valueOr       unwrap success or use fallback value
valueOrElse   unwrap success or compute fallback from failure value
map           transform success value
mapFailure    transform failure value
bind          chain success into another Expect
recover       handle failure into another Expect
```
