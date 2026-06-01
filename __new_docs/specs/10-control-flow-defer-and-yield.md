# 10. Control Flow, Defer, and Yield

Status: normative for source surface.

## while

```musi
while condition (
  body
)
```

Type:

```text
Unit
```

Rules:

```text
condition must have type Bit.
the body is checked in Unit/effect position.
a non-Unit final expression inside the body is invalid unless explicitly discarded with `;`.
```

Valid:

```musi
while running (
  step();
);
```

Invalid if `computeValue()` is non-Unit:

```musi
while running (
  computeValue()
);
```

Valid after explicit discard:

```musi
while running (
  computeValue();
);
```

## Infinite loops

There is no separate `loop` keyword.

Infinite loops use `while true`.

`true` is a value, not a keyword.

```musi
while true (
  step();
);
```

## exit and next

```text
exit : Empty
next : Empty
```

Rules:

```text
exit leaves the nearest enclosing loop.
next abandons the remaining body of the nearest enclosing loop iteration and begins the next iteration.
exit and next are invalid outside loops.
exit and next accept no operands.
```

Valid:

```musi
while running (
  exit when done;
  next when skipped;
  step();
);
```

Invalid:

```musi
exit value;
next value;
```

Reason: `exit` and `next` carry no values.

## defer

```musi
defer cleanup();
defer cleanup() when opened;
```

Rules:

```text
defer expr; registers expr as cleanup for the current computation scope.
the deferred expression must be Unit-producing.
deferred actions run in reverse registration order.
a guarded defer evaluates its guard at registration time.
if the guard is true, the cleanup is registered.
if the guard is false, nothing is registered.
```

Marker:

```musi
defer close(file) when opened;
      -----------      ------
      cleanup          evaluated at registration time
```

Deferred actions run when their computation scope is left by normal completion, typed failure propagation, `exit`, or `next`.

Deferred actions cannot transfer control across their own defer boundary.

The source language does not define bytecode unwinding mechanics.

## yield

`yield` is a 1.0 keyword and the only core suspension primitive.

The following are not keywords:

```text
async
await
spawn
```

Core form:

```musi
yield value;
```

Meaning:

```text
suspend the current resumable computation;
emit value;
resume after the yield point when continued.
```

A body containing `yield` is a resumable body.

All yielded values join to yield type `Y`.

The final expression remains completion type `R`.

The core suspended computation type is:

```text
Resumable[Y, R]
```

The pull-style interface type is:

```text
Generator[Y, R]
```

Example:

```musi
let oneTwo() : Generator[Nat32, Text] := (
  yield 1n32;
  yield 2n32;
  "done"
);
```

The precise protocol member names for `Resumable` and `Generator` are outside this source-language grammar. The source meaning of `yield` is fixed.
