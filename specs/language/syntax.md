# Source Syntax

Status: current small-core draft

## Delimiters

- `( ... )` is computation: grouping, blocks, tuple expressions, and sequencing.
- `{ ... }` is structure: records, data bodies, shapes, and attributes.
- `[ ... ]` is array/type-argument/index shape where grammar admits it.

Accepted empty forms:

```musi
()        -- unit
[,]       -- empty array
[;]       -- empty stack effect
data {;}  -- empty product
data {|}  -- empty sum
match x (|) -- empty match for uninhabited subject
```

## Expressions And Semicolons

Everything is an expression. Top-level statements require `;`. In computation
blocks, non-final expressions require `;`; a final expression may omit `;` and
becomes the block result.

```musi
(
  let x := 1;
  x + 1
)
```

## Bind, Assignment, Equality

`:=` binds a value or assigns into an existing mutable place. Assignment returns
`()`. Equality is `=` and inequality is `/=`.

## Data Bodies

Product data uses `let` members:

```musi
let Buffer := data {
  let ptr : Ptr[mut Byte];
  let len : Nat;
};
```

Sum data uses `|` alternatives:

```musi
let Maybe[T] := data {
| Some(value : T)
| None
};
```

## Control

`if` is binary `Bit` selection and always has `else`.

```musi
if count /= 0 then use(count) else 0
```

`match` is structural inspection. Alternatives start with `|`.

```musi
match value (
| .Some(x) where x > 0 => x
| .Some(_) => 0
| .None => fallback
)
```

`let pattern := expr else fallback;` is refutable forward binding.

## Functions And Calls

Lambdas start with `\`. Function types use `->`.

```musi
let id[T](value : T) : T := value;
let f := \(x : Int) : Int => x;
```

Named arguments use `name := value`. Pipelines use `|>`.

## Operators

Fixed source operators:

```text
users may not invent symbolic operators
arithmetic: + - * / %
comparison: = /= < <= > >=
words: and or xor not in
ranges: ..< ..
fallback: ??
pipeline: |>
```

`and`, `or`, `xor`, and `not` are strict value operators. Short-circuiting is
written with `if`, `match`, or explicit delayed helpers.

## Attributes

Accepted source attributes are:

```text
@deprecated
@skip
@layout
@external
```

`@external` direction comes from `export` and body presence; it does not use
`mode`, `import`, or `export` keys.
