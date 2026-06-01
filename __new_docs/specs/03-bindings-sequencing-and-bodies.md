# 03. Bindings, Sequencing, and Body Results

Status: normative.

## Binding introduction

`let` is the binding introducer.

```musi
let x := 1n32;
let id[T](value : T) : T := value;
```

There is no `fun`, `fn`, `def`, or `var` keyword.

## Binding, writing, and named slots

`:=` is used for binding, writing, and named slots.

```musi
let x := 1n32;
let mut y := 2n32;
y := y + 1n32;

call(name := value);
#{field := value}
@attr(key := value)
```

`=` is equality. It is not binding.

## Semicolon sequencing

Semicolons are mandatory between sequence items.

Newlines never separate expressions.

A semicolon discards the preceding expression and produces `Unit` for that sequence position.

```musi
expr;
```

means:

```text
evaluate expr;
discard its value;
produce Unit for sequencing.
```

## Final expression body result

A final expression without a semicolon is the body result.

```musi
let f() : Text := (
  "done"
);
```

The body result is `Text`.

```musi
let f() : Unit := (
  "done";
);
```

The string value is discarded. The body result is `Unit`.

Marker:

```musi
"done";
      - discard marker; result becomes Unit
```

Invalid:

```musi
let f() : Text := (
  "done";
);
```

Reason: the body result is `Unit`, not `Text`.

## No return keyword

`return` is not a keyword.

A function, lambda, known body, unsafe body, pin body, or computation block uses the same final-expression result rule.

## Pattern binding

A `let` pattern must be irrefutable for the value being bound.

```musi
let #(x, y) := pair;
```

Refutable destructuring is performed with `match`.

## Uniform let binding model

`let` is the declaration/binding form.

RHS forms such as `data`, `trait`, and `import` are not separate declaration families unless this specification explicitly says otherwise. They are values or value-producing forms bound by `let`.

## Slot syntax

`lhs := rhs` is the named-slot value form in contexts that admit slots.

The same token is used for:

```text
binding a name;
writing through admitted write authority;
providing a tagged call argument;
defining a default for a parameter slot;
attaching a value/default to a product field;
attaching a value/default to a sum variant or variant payload where admitted;
attribute key slots.
```

The context determines which slot family is being filled. The parser must not use name/type resolution to decide whether `:=` is present or which syntactic family is being parsed.

`=` remains equality and is never binding, writing, a tagged argument marker, or a default marker.
