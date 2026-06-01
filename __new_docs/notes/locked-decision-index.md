# Locked Decision Index

Status: reference summary.

## Keywords removed / not keywords

```text
if
then
return
null
async
await
spawn
for
module
class
impl
instance
try
catch
foreign
```

## Conditional system

```text
a when condition else b : T
a when condition        : ?T = Maybe[T]
```

`where` is type/evidence-only.

## Sequencing

```text
Semicolons are mandatory between sequence items.
Newlines never separate expressions.
expr; discards and produces Unit.
Final expression without ; is the body result.
```

## Loop control

```text
while condition (...) : Unit
exit : Empty
next : Empty
exit/next carry no values
```

Infinite loop:

```musi
while true (
  ...
);
```

`true` is a value, not a keyword.

## Suspension

```text
yield is the only core suspension primitive.
async / await / spawn are not keywords.
Resumable[Y, R]
Generator[Y, R]
```

## Patterns

```text
PATTERN as name only
or-pattern uses `or`
rest/spread/splat is `...`
match arms use pipe prefix
match guards use when
```

## Constraints

```text
T |= #(A, B)
(K, V) |= (A, B)
(K, V) |= (#(A, B), C)
```

Invalid:

```text
(K, V) |= #(A, B)
T |= (A, B)
```

Reason: type position expects type shape; datum position expects datum/literal shape.

## Traits and operators

```text
Equatable is the equality trait.
Ordering is the ordering trait.
Contains backs membership.
in is mathematical set membership.
```

## Casts

```text
:?   type test
:?>  optional cast, returns ?T
:>   required cast, traps on failed dynamic check
```

`as` is never a cast.

## Attributes

```text
@name
@name(...)
```

No `#[...]` attributes.

`#(...)`, `#[...]`, and `#{...}` remain datum literal/pattern sigils.

`known` and `unsafe` are modifier keywords, not attributes.

## FFI

```text
@foreign(...)
```

No `foreign` keyword and no foreign block syntax.

## Expect

```text
E!T = Expect[T, E]
.Success(value)
.Failure(error)
```

No `try`, no `catch`, `??` is Maybe-only.

Canonical methods:

```text
valueOr
valueOrElse
map
mapFailure
bind
recover
```

## Syntax/language hard locks for 1.0 candidate

```text
Lexing uses maximal munch.
Parser discipline is LR(1) / LL(1)-compatible; more lookahead means the syntax is wrong.
Parser conflicts are not resolved by generator convenience.
let is the binding form.
RHS forms do not become declaration families unless explicitly specified.
lhs := rhs is the named-slot/default/value-attachment form where slots are admitted.
Defaulted parameters must not precede non-defaulted parameters.
known is compiler-known value.
fixed is fixed storage / fixed placement / fixed lifetime.
known/fixed/import do not introduce ::.
Dot remains ordinary selection.
Runtime/bytecode/VM, FFI interop, and attributes remain separate follow-up areas.
```
