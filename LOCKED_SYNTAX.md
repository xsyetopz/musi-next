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

Variant cases may have explicit known tag/discriminant values:

```musi
let TokenKind := data {
  case Eof := 0;
  case Ident(text : Text) := 1;
  case Int(value : Word64) := 2;
};
```

`:= value` on the `case` itself initializes or defines the variant identity.

Rules:
- the tag/discriminant value must be `known`
- tags must be unique within the sum
- if omitted, tags are assigned by the compiler in declaration order
- payload defaults stay inside payload parameters

Payload defaults use their own local `:=`:

```musi
let TokenKind := data {
  case Int(value : Word64 := 0);
};
```

Rationale:
- `:=` on the case itself initializes/defines the variant identity
- payload defaults already have their own local `:=`
- this avoids confusing constructor implementation with tag assignment
- this fits `@packed` and FFI/ABI layout later

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

A `data` body must not mix product `let` entries and sum `case` entries. This is locked.

Product and sum data stay separate. If both are needed, pass one around as a field of the other. This follows the same useful shape as Rust's `enum TokenKind` plus `struct Token`, without adding separate `enum` or `struct` keywords.

Example:

```musi
let TokenKind := data {
  case Ident(text : Text);
  case Int(value : Word64);
  case Eof;
};

let Token := data {
  let kind : TokenKind;
  let span : SourceSpan;
};
```

Invalid:

```musi
let BadToken := data {
  let span : SourceSpan;
  case Ident(text : Text);
  case Eof;
};
```




## Datum Literal Grammar

Datum literals use `#` plus delimiter as a compound lexical category so value literals do not get confused with type syntax or computation delimiters.

Tuple datum literal:

```musi
#(a, b)
```

Record/product datum literal:

```musi
#{ name := value, other := value }
```

Array/list datum literal:

```musi
#[a, b, c]
```

Rules:
- datum literals use comma separators
- trailing comma is allowed
- record datum fields use `:=`, not `:`
- record datum keys are names, not arbitrary expressions
- product construction uses `Type#{ ... }`
- inferred product construction uses `#{ ... }`

Examples:

```musi
let point := #(x, y);
```

```musi
let person := Person#{ name := "Ada", age := 36 };
```

```musi
let person : Person := #{ name := "Ada", age := 36 };
```

```musi
let bytes := #[1, 2, 3, 4];
```

Empty forms are valid:

```musi
#()
#{}
#[]
```

Meanings:
- `#()` is unit datum / empty tuple
- `#{}` is empty record datum
- `#[]` is empty array/list datum and requires type context


## Type Delimiters And Indexing

Plain tuple types use ordinary type-position parentheses.

```musi
(A, B)
()
```

Tuple values use datum syntax.

```musi
#(a, b)
#()
```

Examples:

```musi
let pair : (Text, Nat) := #("Ada", 36);
let unit : () := #();
```

Datums exist to separate value construction from type syntax:
- record/product values use `#{ ... }`
- record/product types use `data { ... }` or named product data
- sum values use dot variant syntax
- sum types use `data { case ...; }` or named sum data
- tuple values use `#( ... )`
- tuple types use `( ... )` in type position
- array/list values use `#[ ... ]`
- array/list types use prefix bracket syntax

Array/list types are prefixed on the element type.

```musi
[A, B]T
```

The bracket prefix carries array/list type parameters such as size, bounds, shape, or other locked array/list metadata. Exact array/list parameter meanings remain open.

Generic/type application uses postfix brackets on the type constructor.

```musi
T[A, B]
```

This separates array/list type construction from generic application.

Indexing rules:
- tuple fields index by numeric field access
- array/list values index by compound `.[` access

Examples:

```musi
let first := pair.0;
let second := pair.1;
```

```musi
let item := list.[0];
```

## Product And Sum Construction

Product data construction uses named or unnamed record datum literals.

Named product construction applies a record datum literal to the product type/name:

```musi
let ada : opaque Named := Person#{ name := "Ada" };
```

Context-inferred product construction can use an unnamed record datum literal:

```musi
let ada : opaque Named := #{ name := "Ada" };
```

Product data is not constructed with function-call syntax.

Rejected:

```musi
let ada : opaque Named := Person(name := "Ada");
```

Sum data construction uses dot variant syntax.

Unqualified variant construction:

```musi
let optionalType := .Some(Type);
```

Qualified variant construction:

```musi
let optionalType := Maybe.Some(Type);
```

Rationale:
- product construction is datum construction, so it uses `#` datum syntax
- sum construction selects a variant, so it uses dot variant syntax
- dot variant syntax follows the same useful rationale as Swift and Zig while remaining part of Musi's own product/sum distinction

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


## Binding Qualifiers

Binding syntax is plain.

```musi
let Name := expr;
let Name : Type := expr;
```

`known`, `fixed`, and `mut` do not appear before `let` and do not appear between `let` and the binding name.

Invalid:

```musi
known let Name := expr;
fixed let Name := expr;
mut let Name := expr;
let known Name := expr;
let fixed Name := expr;
let mut Name := expr;
```

Qualifiers belong to type positions or expression positions.

Valid:

```musi
let Name : known Nat := 0;
let Name := known 0;
let Name : fixed mut Buffer := makeBuffer();
let Name : mut Item := item;
```

If a binding has no annotation, inference preserves the qualified type of the right-hand side. It does not invent qualifiers and does not strip qualifiers.

Examples with inlay hints:

```musi
let Natural /- : Nat -/ := 0;
let Natural /- : known Nat -/ := known 0;
```

If the right-hand side has type `fixed mut Buffer`, the inferred binding type is `fixed mut Buffer`. If the right-hand side has type `Buffer`, the inferred binding type is `Buffer`.

No modifier means no qualifier unless the expression already has that qualifier.

Canonical type qualifier order is:

```musi
known fixed mut T
```

Other orders are rejected or canonicalized by diagnostics/formatting according to the final parser and formatter design.


## Comments

Comment spellings are locked.

Line comment:

```musi
-- line comment
```

Line documentation comment:

```musi
--- line doc comment
```

Block comment:

```musi
/- block comment -/
```

Block documentation comment:

```musi
/-- block doc comment -/
```

Line module documentation comment:

```musi
--! module line doc comment
```

Block module documentation comment:

```musi
/-! module block doc comment -/
```

The longer opener wins by maximal munch, so `--!` is a module doc comment rather than a line comment followed by `!`, `---` is a doc comment rather than a line comment followed by `-`, `/--` is a block doc comment rather than a block comment followed by `-`, and `/-!` is a block module doc comment rather than a block comment followed by `!`.

Block comments nest.

```musi
/-
outer
  /- inner -/
outer continues
-/
```

Block documentation comments and block module documentation comments participate in the same nesting system.

```musi
/--
outer doc
  /- ordinary nested block comment -/
outer doc continues
-/
```

Rules:
- `/- ... -/`, `/-- ... -/`, and `/-! ... -/` may contain nested block comment openers
- all nested block comment forms close with `-/`
- line comments inside block comments are comment text
- nested block comments are implemented with a linear depth counter
- unterminated nested block comments are diagnostic errors

Rationale:
- nested comments allow temporarily commenting out code that already contains block comments
- nesting avoids accidental early close on inner `-/`
- the delimiter pair is explicit enough to keep lexing deterministic and linear
- module docs are supported separately from item docs

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




## Optional Type And Operators

`?T` is the optional type sugar for `Maybe[T]`.

```musi
?T
Maybe[T]
```

`?` in type position names optionality/maybe-ness. It does not name `Expect`.

`??` is the Maybe fallback operator.

```musi
value ?? fallback
```

Rules:
- `value` must have type `?T` / `Maybe[T]`
- `fallback` must produce `T`
- result type is `T`
- fallback is lazy and is evaluated only when `value` is absent
- `??` does not operate on `Expect`

Examples:

```musi
let name : Text := maybeName ?? "unknown";
```

```musi
let user : User := findUser(id) ?? defaultUser();
```

`?.` is optional access.

```musi
value?.member
value?.method(args)
value?.[index]
```

Rules:
- `?.` operates only on `?T` / `Maybe[T]`
- access, call, or index happens only when the value is present
- absent stays absent
- `?.` does not invent null
- `?.` composes with `??`

Examples:

```musi
let city : ?Text := user?.address?.city;
```

```musi
let city : Text := user?.address?.city ?? "unknown";
```

```musi
let first := maybeList?.[0];
```

Distinctions:
- `when ... else ...` branches on `Bit`
- `??` branches on optional presence
- `?.` propagates absence through access
- `Expect` remains explicit unless a separate error/failure sugar is locked later

## Type Annotation Marker

`:` is the universal type annotation marker.

Type annotations always use:

```musi
a : B
```

This applies in value, parameter, field, result, receiver, pattern, and shape-member positions.

Examples:

```musi
let x : Int := 1;
let name : Text;
let packet : fixed Packet := readPacket();
let sink : opaque Writer := fileWriter;
let makeSink() : erased Writer := FileWriter(path);
```

```musi
let add(a : Int, b : Int) : Int := a + b;
```

```musi
let (self : User).name() : Text := self.name;
```

```musi
let Writer := shape {
  let write(self : Self, text : Text) : Unit;
};
```

```musi
match value {
case id : UserId => id.raw;
}
```

`:` is not overloaded for casts, subtyping, runtime type tests, type equivalence, or conformance. Those use their own operators.

Reserved type-related operator roles:
- `<:` subtype relation
- `:?` runtime type test returning `Bit`
- `:>` explicit static conversion/cast
- `:?>` checked runtime cast returning an explicit failure-capable result
- `~=` type equivalence relation
- `|=` conformance/fits relation

`:=` remains binding/definition/initialization.

`=` remains equality.
`/=` remains inequality.

## Type Operator Family

Musi uses a coherent `:`-led family for type-related operators.

Meanings:
- `:` annotates
- `:?` tests runtime type and returns `Bit`
- `:>` requests explicit static conversion/cast
- `:?>` performs a checked runtime cast and returns an explicit failure-capable result
- `<:` states subtype relation
- `~=` states type equivalence relation
- `|=` states shape conformance/fits relation

Examples:

```musi
let isUser : Bit := value :? User;
```

```musi
let label := "user" when value :? User else "other";
```

```musi
let widened : Word64 := small :> Word64;
```

```musi
let checked := value :?> User;
```

```musi
match value :?> User {
case .Ok(user) => user.name;
case .Err(error) => "not user";
};
```

Rules:
- `:?` never returns the narrowed value
- `:?>` never returns `Bit`
- `:>` is not runtime checked; it is an explicit static or known-valid conversion request
- `:` is only annotation
- `:=` is only binding, definition, or initialization
- `=` is equality

`?=` is rejected. It has no strong Musi rationale and does not belong to the coherent `:` type-operator family.


## Expect And Checked Casts

`Expect` remains explicit.

```musi
Expect[T, E]
```

There is no locked error/failure sugar for `Expect`. Possible sugar such as `E!T`, a keyword, or another operator is left to future/community design unless a strong rationale appears.

`?T`, `??`, and `?.` are Maybe-only and do not apply to `Expect`.

`:?>` returns an explicit `Expect` value.

```musi
value :?> T
```

Result type:

```musi
Expect[T, CastError]
```

Examples:

```musi
let checked : Expect[User, CastError] := value :?> User;
```

```musi
match value :?> User {
  case .Ok(user) => user.name;
  case .Err(error) => "not user";
};
```

Rationale:
- failure stays distinct from absence
- failed casts carry error information instead of only `Bit` or `Maybe` absence
- no hidden exceptions are introduced
- `Expect` sugar is not locked prematurely

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

let ada : opaque Named := Person#{ name := "Ada" };
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
- [ ] Whether visibility words are hard keywords or contextual introducers
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
- [x] Optional/error type surface forms
- [ ] Whether callable types use stack-effect syntax directly
- [x] Whether type annotations use `:` in every context
- [x] Whether casts/tests use symbolic operators such as `:>` and `:?>`

### Stack Effect

- [ ] Exact source syntax for stack effects
- [ ] Whether stack effects are first-class type values
- [ ] Whether ordinary functions expose stack-effect types or parameter/result sugar
- [ ] Stack-effect compatibility for `when`, `match`, `defer`, `yield`, and receiver methods
- [ ] Whether guarded emission requires a special effect kind or row-polymorphic stack effect

### Data

- [ ] Product field grammar inside `data`
- [ ] Sum variant grammar inside `data`
- [x] Exact meaning of `case Variant(...) := value`
- [x] Whether product `let` entries and sum `case` entries can ever mix
- [ ] Associated data/value binding rules inside `data`
- [x] Constructor generation rules
- [ ] Destructuring and pattern syntax for product data
- [x] Variant tag/discriminant rules

### Representation And Metadata

- [x] Attribute syntax
- [x] Whether `@packed` is the final packed-data spelling
- [ ] Representation controls such as alignment, endian, tags, padding, and ABI layout
- [ ] Whether representation metadata appears before `data`, after `data`, or inside the structural body
- [ ] Whether metadata is preserved in SEIL for decompilation

### Comments

- [x] Line comment spelling
- [x] Line doc comment spelling
- [x] Line module doc comment spelling
- [x] Block comment spelling
- [x] Block doc comment spelling
- [x] Block module doc comment spelling
- [x] Nested block comment support

### Delimiters And Separators

- [x] Exact grammar for `#(` tuple datum literals
- [x] Exact grammar for `#{` record/product datum literals
- [x] Exact grammar for `#[` array/list datum literals
- [x] Whether plain tuple types use `(A, B)` or another form
- [x] Whether `[]` is used for generics, indexing, stack effects, type application, or a reduced subset
- [ ] Trailing separator rules for `,` and `;`
- [x] Empty tuple, empty record, and empty array syntax

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

- [x] Exact meaning of `known`
- [x] Whether `known` applies to expressions, bindings, parameters, types, or all of them
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
