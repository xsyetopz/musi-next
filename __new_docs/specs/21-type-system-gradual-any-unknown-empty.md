# 21. Gradual Type System, Any, Unknown, and Empty

Status: normative semantic addendum.

## Purpose

Musi supports static typing, mixed typing, and dynamic typing through one gradual type system.

It does not have three separate type systems.

```text
static typing  = precise types are known and checked before execution
mixed typing   = static code and Any-typed code interact through explicit boundaries
dynamic typing = values are intentionally carried as Any and checked at use
```

## Core type names

```text
Any     runtime top value carrier
Unknown checker/frontend imprecision
Empty   bottom type / uninhabited type
```

`Dynamic` is not the Musi type name.

## Any

`Any` is a real runtime value type.

A value of type `Any` may carry any Musi/SEAM value together with enough runtime type identity for inspection, checking, casting, or dynamic dispatch.

`Any` must not mean "turn off type checking".

Operations on `Any` require one of:

```text
type test
checked cast
optional cast
pattern/type match
decode/validation
explicit dynamic dispatch operation
unsafe unchecked operation
```

Examples:

```musi
let value : Any := host.config.get("health");
let health : Nat32 := value :> Nat32;
```

```musi
let value : Any := host.config.get("health");
let health : ?Nat32 := value :?> Nat32;
```

Invalid unless Musi has an explicit dynamic-add operation at the call site:

```musi
let value : Any := host.config.get("health");
let next := value + 1n32;
```

## Unknown

`Unknown` means the checker does not yet know a more precise type.

`Unknown` is a checker/frontend state, not a normal runtime value representation.

Rules:

```text
omitted parameter annotation starts as Unknown;
omitted local annotation starts as Unknown;
omitted return annotation starts as Unknown;
Unknown is solved by constraints, contextual type, body type, return type, and trait/evidence resolution;
Unknown must not reach SEIL or SEBC.
```

## Empty

`Empty` is the bottom type.

```text
Empty <: T for every T
```

It is used for unreachable or non-continuing control:

```text
exit
next
trap
unreachable paths
impossible match arms
non-returning computation
```

`exit` and `next` have type `Empty`.

## Missing annotations

Missing type annotations do not mean `Any`.

```musi
let add(x, y) := x + y;
```

Checking model:

```text
x : Unknown
y : Unknown
result : Unknown
constraint: x + y must be valid
```

The checker must solve the Unknown types.

Allowed result if operator evidence exists:

```text
add : [T, U, R where Add[T, U, R]] (T, U) -> R
```

Allowed result if context proves concrete types:

```text
add : (Int32, Int32) -> Int32
```

Rejected by default:

```text
add : (Any, Any) -> Any
```

unless the author explicitly requests `Any`.

## Partial annotations

Examples:

```musi
let add(x : Int32, y) := x + y;
let add(x : Int32, y : Int32) := x + y;
let add(x : Int32, y : Int32) : Int32 := x + y;
let add(x : Int32, y) : Int32 := x + y;
let add(x, y) : Int32 := x + y;
```

Rules:

```text
explicit annotations create constraints;
return annotation creates a result constraint;
operator traits/evidence solve omitted types where unique;
ambiguous evidence is an error;
implicit Any is not a fallback.
```

## Exported/public signatures

Exported functions must have stable signatures.

An exported function may omit source annotations only if the exported SEIL signature is uniquely inferred and recorded in the artifact.

Recommended source style for exported API:

```musi
export let add(x : Int32, y : Int32) : Int32 := x + y;
```

Rejected unless a stable constrained-generic export format is explicitly specified:

```musi
export let add(x, y) := x + y;
```

## Any boundaries

`Any` is appropriate at explicit dynamic boundaries:

```musi
let parseConfig(value : Any) : Expect[Config, Text] := ...;
let inspect(value : Any) : Text := ...;
let callScript(fn : Any, args : #[Any]) : Expect[Any, Text] := ...;
```

`#[Any]` is a sequence value. It is not spread/splat syntax.

```musi
callScript(fn, args);    -- passes the sequence as one value
fn(args...);             -- spreads elements into argument positions
```

`...` is the only syntax that changes argument-list shape.

## Embedded-safety rule

A design is rejected for embedded suitability if it allows unannotated code to silently become `Any` or to silently use dynamic dispatch, dynamic allocation, reflection, or runtime type services.
