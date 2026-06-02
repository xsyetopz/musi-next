# LOCKED_SYNTAX.md

## Core Identity

Musi is a small systems language with a small core.

Musi is expression-first:
- statements are not a separate semantic category
- a top-level expression terminated by `;` is a valid top-level item
- `;` may discard a value or sequence expressions
- definitions are expressions
- control flow is expression-based

Musi targets SEIL bytecode directly. SEIL is the canonical lowered form, similar in role to CIL. Musi source should lower to SEIL in a way that enables future SEIL-to-Musi decompilation to recover near-identical source when metadata is preserved.

Musi does not have an IR layer between source and SEIL.

## Parsing And Lexing Constraints

Musi syntax must preserve:
- maximal-munch lexical design
- one-token-lookahead parsing
- no syntax that requires speculative parsing beyond one token
- no syntax retained only because existing or conventional languages use it

If a form needs more than one token of lookahead, the design is rejected and redesigned.

## Keyword Rule

A keyword is a hard-reserved source word required to introduce or disambiguate a grammar form.

A word is not a keyword merely because it is built in, compiler-owned, common, or standard-library-provided.

Operators, compiler intrinsics, methods, traits/shapes, sum types, product types, and built-in types are not keywords unless they are hard-reserved grammar introducers.

## Universal Binding

`let` is the universal binding form.

It binds values, functions, data definitions, shape definitions, module/import results, compile-time values, runtime values, and attached receiver methods.

Examples:

```musi
let x := value;
let Name := data { };
let Contract := shape { };
let imported := import "path";
let (self : Parent).method() := expr;
```

There is no separate `fn`, `type`, `struct`, `enum`, `class`, `impl`, `const`, or `static` keyword.

## Expression Sequencing

Parentheses delimit computation regions.

```musi
(
  first();
  second();
  third()
)
```

Semicolon separates sequential computation steps.

## Structural Regions

Curly braces delimit structural regions.

Structural regions define members, fields, variants, cases, or rule tables. They are not sequential computation bodies.

Examples:

```musi
let Person := data {
  let name : Text;
  let age : Word8 := 0;
};
```

```musi
let myValue := match value {
case .Some(x) => x;
case .None => 0;
};
```

## Datum Literals

Datum literals use `#` plus a delimiter as a compound lexical category.

```musi
#(a, b)
#{ name := "Ada", age := 36 }
#[1, 2, 3]
```

Meaning:
- `#(` begins a tuple datum literal
- `#{` begins a record/product datum literal
- `#[` begins an array/list datum literal

Plain `{ ... }` never means a value record literal.
Plain `( ... )` never means a tuple datum literal unless introduced by `#`.

Value datum syntax and type syntax are separate:
- value datum uses `#`
- type syntax does not use `#`

## Separators

`,` separates sibling items in datum, argument, parameter, and type-argument lists.

`;` separates computation steps and structural members/rules.

Examples:

```musi
foo(a, b, c)
#(x, y)
#[1, 2, 3]
#{ x := 1, y := 2 }
```

```musi
let Point := data {
  let x : Int;
  let y : Int;
};
```

```musi
let Choice := match value {
case .A => 1;
case .B => 2;
};
```

## Conditional Expressions

`when` is the conditional guard operator.

Total conditional expression:

```musi
value when condition else fallback
```

Rules:
- `condition` must be `Bit`
- true and false branches must have compatible type/stack effect
- `else` provides the fallback branch explicitly
- no hidden `Maybe`, `Unit`, bottom, or union is synthesized

Guarded emission expression:

```musi
value when condition
```

Rules:
- valid only in contexts where emitting nothing is meaningful
- invalid in ordinary required-value positions
- no hidden value is synthesized on false

## Match And Case

Pattern matching uses `match`.

Each match arm starts with `case`.

```musi
match value {
case pattern when guard => expr;
case pattern => expr;
}
```

Rules:
- `match` introduces a pattern decision expression
- `case` introduces one pattern alternative
- `when` guards a case
- `=>` separates the pattern/guard from the result expression
- match arms are separated by `;`
- the match body uses `{}` because it is a structural decision table, not a computation sequence

## Data

`data` is the single data-definition form.

The body determines whether the data is product-shaped or sum-shaped.

Product-shaped data uses `let` entries:

```musi
let Person := data {
  let name : Text;
  let age : Word8 := 0;
};
```

Sum-shaped data uses `case` entries:

```musi
let Maybe := data {
  case Some(value : T);
  case None;
};
```

Variant cases may have defaults:

```musi
let Token := data {
  case Ident(text : Text);
  case Int(value : Word64 := 0);
  case Eof := eofTokenValue;
};
```

A `data` body may also bind data-valued fields or associated data through `let`:

```musi
let Packet := data {
  let header := Header;
  let Payload := data {
    case Text(message : Text);
    case Binary(bytes : Bytes);
  };
};
```

Receiver methods are defined outside the `data` body:

```musi
let (self : Parent).method() := expr;
```

There is no separate `struct`, `enum`, `union`, `class`, or `impl` form.

A `data` body must not mix product `let` entries and sum `case` entries unless a later locked rule explicitly permits it.


## Fixed Storage

`fixed` is a type/storage-space modifier.

`fixed T` means storage-qualified `T` whose address is stable for the value's lifetime and cannot be moved by the collector/runtime during that lifetime.

`fixed` is the chosen spelling. `stable` is rejected because it is too broad and can mean API stability, value immutability, deterministic behavior, ABI stability, numeric stability, or sorting stability. `fixed` names the required storage guarantee directly: the value is fixed in memory.

`fixed` does not mean:
- static/global
- immutable
- compile-time
- type-associated
- permanent
- thread-safe by itself

`fixed` is orthogonal to `mut`:

```musi
fixed T
mut T
fixed mut T
```

Meanings:
- `fixed T` has stable address and is not necessarily mutable
- `mut T` has mutable access and is not necessarily stable-address storage
- `fixed mut T` has stable address and mutable access

Examples:

```musi
let packet: fixed Packet := readPacket();
```

```musi
let update(packet: fixed mut Packet) := (
  packet.length := packet.length + 1;
  packet
);
```

```musi
let ptr := address(packet);
```

Address-taking requires fixed storage. Movable values cannot expose stable raw addresses.

Rejected:

```musi
let packet: Packet := readPacket();
let ptr := address(packet);
```

`fixed` can make a separate `pin` keyword unnecessary. If scoped temporary non-moving access is needed later, it must be justified against `fixed` instead of added by default.

## Opaque And Erased Types

`opaque` and `erased` are type-space modifiers, not attributes.

They affect type identity, representation, dispatch, checking, ABI/SEIL metadata, and decompilation. They are not declaration decoration.

`hidden` is removed. It is too broad and does not identify a precise type-system operation. Use exact concepts instead:
- `opaque` for existential type hiding
- `erased` for opaque-result/static-hidden concrete type
- `export` or non-export for module visibility
- metadata/attributes for representation, ABI, or interop details

### `opaque`

`opaque T` is closest to Swift's `any T`.

It means an existential/capability value whose concrete type is hidden behind the `T` shape/type boundary. Operations may go through existential, witness, or capability representation.

Example:

```musi
let sink: opaque Writer := fileWriter;
```

The consumer knows `sink` fits `Writer`, but does not know the concrete stored type.

### `erased`

`erased T` is closest to Swift's `some T`.

It means the exposed type hides the concrete type name, while the defining expression still has one compiler-known concrete underlying type. Static specialization may remain possible.

Example:

```musi
let makeSink(): erased Writer := FileWriter(path);
```

The caller knows the result fits `Writer`, but the concrete result type is erased from the exposed signature.

### Attributes Versus Type Modifiers

Attributes fit representation and interop metadata, not core type-space operations.

Examples of attribute-shaped concepts, if retained:

```musi
@packed data {
  let version: Bits[4];
}
```

```musi
let read := @foreign(language := "c", symbol := "read") import "libc";
```

`foreign`/`extern` interop metadata belongs in attributes with parameters and remains a later FFI design topic.

Rule:
- type identity/storage/checking concept: type-space modifier
- representation/ABI/interop annotation: attribute

## Packed Data

Packed/bit-structured data is still `data`.

It does not get a new keyword such as `bitstruct`.

Packed representation should be expressed as metadata, with `@packed` as the preferred spelling if attributes are retained.

Example direction:

```musi
let Header := @packed data {
  let version : Bits[4];
  let flags : Bits[4];
  let length : Bits[16];
};
```

Representation metadata is not fully locked here; only the decision that packed data remains `data` is locked.

## Parameters And Defaults

Parameters may have defaults.

Defaults must be trailing.

Valid:

```musi
let f(a : A, b : B, c : C := default) := expr;
```

Invalid:

```musi
let f(a : A := default, b : B) := expr;
```

This rule applies uniformly to function parameters, method parameters, constructor-like parameters, and variant payload parameters.

## Algebraic Operators

Core Boolean/bit algebra operators are:

```musi
&
|
^
~
```

Meanings:
- `&` conjunction / bitwise-and / type-phase intersection where type checking proves it
- `|` disjunction / bitwise-or / type-phase union where type checking proves it
- `^` xor / symmetric difference where type checking proves it
- `~` complement / not where type checking proves it

There is no separate logical/bitwise operator split.

There is no:

```musi
and
or
xor
not
&&
||
!
&?
|?
~?
```

`Bit`, `Word`, `Word8`, `Word16`, `Word32`, `Word64`, and `Bits[N]` use the same symbolic algebra where valid.

Guard contexts require `Bit`. There is no truthiness.

Short-circuiting is control flow, not algebra. Use `when ... else ...` or `match`.

## Shape Naming

`shape` is the locked spelling for structural contracts.

`shape` means an observable structure/capability contract: a value or type fits a shape when it provides the required members and operations according to Musi's conformance rules.

`trait` is rejected as the core spelling because it carries Rust, Scala, PHP, and C++ baggage around nominal implementations, mixins, coherence rules, code reuse, or type-level metadata conventions.

`data` defines what a thing is. `shape` defines what a thing must look like.

Examples:

```musi
let Writer := shape {
  let write(self : Self, text : Text) : Unit;
  let flush(self : Self) : Unit;
};
```

```musi
let sink : opaque Writer := fileWriter;
let makeSink() : erased Writer := FileWriter(path);
```

There is no separate `trait` keyword.


## Shape Conformance

Default `shape` conformance is structural.

A type or value fits a structural shape when it provides the required observable members and operations with compatible types and stack effects. No conformance declaration is required for structural shapes.

`@witness shape` defines a witness-required shape.

Witness-required shapes are for semantic, lawful, marker, or capability contracts where members alone are not enough to prove correct conformance. Empty marker shapes must use `@witness shape` to avoid every type fitting them accidentally.

`|=` is the locked conformance/fits relation operator.

Roles:
- `T |= Shape` states or constrains that `T` fits `Shape`
- `let T |= Shape := witnessValue;` binds an explicit witness for witness-required conformance

There is no `impl`, `implements`, `extends`, or `trait` keyword. Receiver methods and witness bindings use universal `let`.

Structural example:

```musi
let Named := shape {
  let name(self : Self) : Text;
};

let Person := data {
  let name : Text;
};

let ada : opaque Named := Person(name := "Ada");
```

Witness-required example:

```musi
let Hashable := @witness shape {
  let hash(self : Self) : Word64;
};

let UserId := data {
  let raw : Word64;
};

let UserId |= Hashable := #{
  hash := \(self : UserId) => self.raw.hash()
};
```

`|=` should not become a general-purpose ordinary Boolean test by default. Runtime fit checks for dynamic or opaque values remain an open design topic.

## Confirmed Attributes

The following attributes are confirmed surface attributes:

```musi
@packed
@align(...)
@witness
```

Meanings:
- `@packed` marks packed/bit-structured representation metadata
- `@align(...)` marks representation alignment metadata
- `@witness` marks a `shape` as requiring explicit witness conformance

These are attributes because they refine representation or contract mode without introducing a new grammar category.

Examples:

```musi
let Header := @packed data {
  let version : Bits[4];
  let flags : Bits[4];
};
```

```musi
let Page := @align(4096) data {
  let bytes : fixed Bytes;
};
```

```musi
let Send := @witness shape {
};
```

## Open Question Checklist

These questions are intentionally open and are not locked by this document.

### Keyword Set

- [ ] Final hard-reserved keyword list
- [ ] Whether modifier-like words such as `known`, `mut`, and visibility words are hard keywords or contextual introducers
- [ ] Whether `import` is a keyword or a compiler-owned function/form with special lowering
- [ ] Whether `export` is a keyword, metadata, or a structural member rule
- [x] Whether `hidden` remains a surface concept
- [x] Whether `erased` remains a surface concept
- [x] Whether `fixed`, `stable`, or another word is needed for fixed storage/lifetime

### Shape, Trait, And Conformance

- [x] Final spelling: `shape`, `trait`, or another word
- [x] Structural conformance rules
- [x] Nominal/witness conformance rules, if any
- [x] Whether shape/trait conformance uses `|=`, a word operator, or a different form
- [x] Whether erased shape values are surface syntax, metadata, or compiler-owned lowering

### Type System

- [ ] Bidirectional gradual type-system model
- [ ] Type-phase algebra for `|`, `&`, `^`, and `~`
- [ ] Union/intersection representation and normalization rules
- [ ] Optional/error type surface forms
- [ ] Whether callable types use stack-effect syntax directly
- [ ] Whether type annotations use `:` in every context
- [ ] Whether casts/tests use symbolic operators such as `:>` and `:?>`

### Stack Effect

- [ ] Exact source syntax for stack effects
- [ ] Whether stack effects are first-class type values
- [ ] Whether ordinary functions expose stack-effect types or parameter/result sugar
- [ ] Stack-effect compatibility for `when`, `match`, `defer`, `yield`, and receiver methods
- [ ] Whether guarded emission requires a special effect kind or row-polymorphic stack effect

### Data

- [ ] Product field grammar inside `data`
- [ ] Sum variant grammar inside `data`
- [ ] Exact meaning of `case Variant(...) := value`
- [ ] Whether product `let` entries and sum `case` entries can ever mix
- [ ] Associated data/value binding rules inside `data`
- [ ] Constructor generation rules
- [ ] Destructuring and pattern syntax for product data
- [ ] Variant tag/discriminant rules

### Representation And Metadata

- [x] Attribute syntax
- [x] Whether `@packed` is the final packed-data spelling
- [ ] Representation controls such as alignment, endian, tags, padding, and ABI layout
- [ ] Whether representation metadata appears before `data`, after `data`, or inside the structural body
- [ ] Whether metadata is preserved in SEIL for decompilation

### Delimiters And Separators

- [ ] Exact grammar for `#(` tuple datum literals
- [ ] Exact grammar for `#{` record/product datum literals
- [ ] Exact grammar for `#[` array/list datum literals
- [ ] Whether plain tuple types use `(A, B)` or another form
- [ ] Whether `[]` is used for generics, indexing, stack effects, type application, or a reduced subset
- [ ] Trailing separator rules for `,` and `;`
- [ ] Empty tuple, empty record, and empty array syntax

### Control Flow

- [ ] Exact precedence and associativity of `when ... else ...`
- [ ] Dangling-else prevention rule
- [ ] Whether `when` condition may contain unparenthesized `when`
- [ ] Whether guarded emission is allowed in specific structural contexts
- [ ] Whether loops exist as syntax or are expressed through recursion/recur forms
- [ ] Whether `defer`, `yield`, and `pin` earn hard keyword status

### Match And Patterns

- [ ] Exact pattern grammar
- [ ] Whether pattern alternatives exist
- [ ] Whether pattern alternatives use `|`, repeated `case`, or another form
- [ ] Whether match cases require semicolons in all positions
- [ ] Exhaustiveness rules
- [ ] Guard evaluation order
- [ ] Pattern binding syntax

### Operators

- [ ] Full symbolic operator set
- [ ] Operator precedence table or precedence-avoidance strategy
- [ ] Whether all infix expressions parse flat and precedence is semantic
- [ ] Whether user-defined symbolic operators exist
- [ ] Whether word operators exist at all
- [ ] Whether assignment/binding/update operators are distinct from equality
- [ ] Equality, equivalence, ordering, approximation, and membership operators

### Modules And Imports

- [ ] Whether modules are ordinary record-like values
- [ ] Import expression syntax
- [ ] Export surface syntax
- [ ] Visibility rules
- [ ] Whether package/module paths are strings, symbols, datums, or dedicated syntax
- [ ] How imports/exports round-trip through SEIL

### Runtime And SEIL

- [ ] SEIL instruction model
- [ ] SEIL metadata required for near-identical decompilation
- [ ] Source-to-SEIL lowering guarantees
- [ ] Whether SEIL has a stable binary and textual form
- [ ] How stack-effect verification appears in SEIL
- [ ] How known-phase evaluation appears in SEIL

### Known Phase

- [ ] Exact meaning of `known`
- [ ] Whether `known` applies to expressions, bindings, parameters, types, or all of them
- [ ] Known-phase evaluation limits
- [ ] Known/runtime boundary rules
- [ ] Whether known values can construct `#` datum literals
- [ ] Whether known functions compile to SEIL or evaluate through a separate interpreter

### Safety

- [ ] Exact meaning of `unsafe`
- [ ] Whether unsafe is an expression wrapper, attribute, capability, or all of these
- [ ] Pointer types and pointer operations
- [ ] Pinning syntax and semantics
- [ ] Foreign boundary rules
- [ ] Whether dangerous behavior can ever be a warning instead of an error
