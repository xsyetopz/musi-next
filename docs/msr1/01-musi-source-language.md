> This agent-indexable topic view is extracted from [`../MSR1.md`](../MSR1.md). `MSR1.md` remains the sole normative authority.

# Part I — The Musi source language

Status: **finalized language design**.

The governing constraint is **zero hidden cost**: any material runtime consequence must follow from source-visible semantics. See `design-rationale.md`.

## 1. Source, tokens, identifiers

Source is valid UTF-8. UTF-8 BOM is rejected. LF and CRLF are accepted and denote LF; lone CR is invalid. Source is not Unicode-normalized.

Identifiers are ASCII-only:

```text
first:     A-Z | a-z
following: A-Z | a-z | 0-9
```

`_` is a standalone discard/wildcard token and is never part of an identifier. Backticks do not escape names.

Naming is semantic:

```text
UpperCamelCase  type-valued bindings and choice case names
lowerCamelCase  ordinary values, callables, members, fields, payloads,
                receiver names, attribute items, pragma items
```

Reserved words:

```text
let var export static known record choice case match when else
while leave cycle defer import lambda yield not write volatile
```

Contextual infix/bit operators:

```text
and or shl shr rol ror
```

Musi lexing uses maximal munch over the complete explicitly defined token vocabulary. When more than one token matches the current input prefix, the token consuming the greatest number of source characters is selected.

Every multi-character punctuation token in that vocabulary is one indivisible compound token. Adjacent punctuation characters do not form a compound token unless that exact sequence is itself defined as a token. Trivia cannot split a compound token.

Examples of compound tokens include:

```text
:= => -> ~> ..< ..= .. ?. .[ .^ $[ %[ #( #[ #{ ...
```

Thus `~=` is one token, while `).` is `)` followed by `.` because `).` is not a Musi token.

The normative source grammar is LL(1): one token of lookahead is sufficient at every grammatical decision. Expression precedence may be implemented with Pratt parsing, but this does not relax the LL(1) requirement for the source grammar or permit token-level ambiguity.

## 2. Comments, trivia, separators

```text
--          ordinary line comment
---         documentation line comment
/- ... -/   ordinary block comment
/-- ... --/ documentation block comment
```

Both block forms nest, including mixed nesting. Documentation comments attach to the following binding.

Physical newline is whitespace. There is no automatic separator insertion and no continuation syntax.

Musi has exactly two separator roles:

- `;` terminates an expression used as a statement and separates sequential expressions;
- `,` separates peer/list items.

A statement is exactly an expression followed by mandatory `;`. Statement is a syntactic sequencing role, not a separate semantic category: evaluating the statement evaluates its expression, and the statement context retains no resulting value.

Sequential uses include source expressions, computation expressions, mutual-recursion members, record fields, and choice cases.

Peer/list uses include parameters, arguments, tuple/array/record datum members, match arms, attribute items, and pragma items.

A trailing separator is allowed only where the grammar explicitly permits one.

## 3. Values, expressions, evaluation order, and discarded values

Musi is value- and expression-based. Every executable language construct is an expression and every expression has a value. Named constructs are introduced through binding expressions.

A binding expression evaluates its defining semantics, establishes its binding or storage, and has value Unit. A statement is `expression ;` and does not introduce separate evaluation semantics.

Runtime evaluation order is deterministic.

Except for control-dependent forms listed below, operands and peer expressions evaluate left-to-right in source order.

- call: callee, then runtime arguments left-to-right;
- indexing: base, then index/range;
- binary operator: left operand, then right operand;
- assignment: target-place computation, then right-hand value, then store;
- tuple/array datum: members left-to-right;
- record update: explicit replacement expressions in source order, then final expansion expression;
- choice payload construction: arguments left-to-right.

Control-dependent forms evaluate only what they select:

- `a when condition`: evaluate `condition`; evaluate `a` only when true;
- `a when condition else b`: evaluate `condition`; then exactly one branch;
- `and`/`or`: short-circuit left-to-right;
- `match`: evaluate scrutinee once; arms are tested in source order; evaluate only the selected arm body;
- `while`: evaluate condition before each iteration.

A computation's final expression supplies its result. An empty computation is Unit. A final binding expression therefore supplies Unit.

Every non-final expression in a sequential computation must have type Unit or Never. A top-level expression statement must have type Unit or Never. Discarding any other value is explicit:

```musi
let _ := expression;
```

This prevents silent value loss and makes intentionally ignored results visible.

## 4. Bindings, mutability, scope, storage modifier

A binding is an expression whose result is Unit.

`let` binds a value. `var` establishes mutable storage initialized from a value. These are distinct semantics, not declaration classes separate from expressions.

`:=` is the sole association/update symbol. It is used for:

- binding definitions;
- mutation;
- defaults;
- named arguments;
- record datum field association;
- explicit choice case values.

Mutation requires a writable place, evaluates to Unit, and is non-associative.

```musi
a := value;
```

`a := b := c` is invalid.

`_ := expression` is invalid; use `let _ := expression`.

No binding shadows another visible binding. A binding name must be unique in its lexical visibility region.

`export` is valid only at module/source scope.

`static` is valid only for module-scope runtime data storage bindings. It means exactly:

> the binding has stable program-lifetime storage.

`static` has no type/class/method meaning and does not mean compile-known.

Named callable definitions are code bindings and do not use `static`.

Modifier order is:

```text
export static let/var
```

subject to the semantic restrictions above.

Ordinary runtime data without `static` has its lexical/initialization lifetime. A runtime datum exported from a module must therefore have explicit persistent storage; exported runtime data uses `static let`. Mutable storage is never exported directly.

Parameterized binding heads, callable heads, receiver-prefix heads, and mutual-recursion groups are `let` forms because they bind values. `var` accepts only datum binding patterns and cannot declare a callable/type-family/receiver head.

## 5. Local bindings and computations

Binding expressions are valid sequential expressions inside computation expressions:

```musi
(
    let x := compute();
    var y := x;
    y := y + 1;
    y
)
```

A local binding is visible strictly after its complete binding expression, except for the explicit recursion rules below.

`export`, `static`, foreign declarations, interrupt entries, and receiver-method bindings are module-scope only.

## 6. Recursion

A named callable becomes recursively visible after its complete binding head, including known/runtime parameters and explicit result type.

Any directly recursive binding must therefore provide an explicit result type/effect; recursive result inference is invalid.

Mutual recursion uses the explicit bounded group:

```musi
let {
    first(x Int) Int := second(x);
    second(x Int) Int := first(x);
};
```

All heads are established before any body. Every member whose result participates in the recursion must have an explicit result type. Bodies are checked/evaluated in source order. No cyclic inference is performed.

## 7. Compile-known phase

`known expression` evaluates `expression` at compile time.

Compile-known execution may use ordinary deterministic Musi computation whose operands are compile-known. It may not perform runtime storage access, raw/volatile memory transactions, foreign calls, interrupt entry, or yielding execution.

Compile-known values of ordinary runtime-representable types may later materialize as constants when explicitly used in runtime context. Types and `Index` themselves have no runtime representation and cannot implicitly materialize.

Compile-known evaluation does not require target runtime storage.

## 8. Parameters, calls, defaults, inference

Known parameters use `[...]`. Runtime parameters use `(...)`.

```musi
let f[T Type, N Index](x T, count Nat := 1) T := ...;
```

Parameter forms are:

```text
name Type
name Type := default
_ Type
_ Type := default
```

Within each known/runtime parameter list, once a default appears all following parameters in that same list must also have defaults.

A default may reference earlier parameters and visible outer bindings, never later parameters.

Known defaults evaluate in known context. Runtime defaults evaluate at the call when omitted, in declaration order after explicit call-site arguments have been evaluated.

Named arguments use `name := value`. Positional arguments precede named arguments. A named argument identifies exactly one declared parameter and may appear once.

In an argument list, a top-level unparenthesized `lowerCamelName := expression` is a named argument rather than mutation. To intentionally evaluate mutation as an argument value, nest it in a computation expression.

Known arguments may be omitted only when supplied by a default or uniquely inferred from:

- explicit runtime argument types;
- explicit known arguments;
- receiver known parameters;
- the expected result type.

Inference uses exact structural/type equality only. It performs no overload search, subtyping, numeric conversion, dynamic widening, or backtracking. If there is not exactly one solution, source must state the argument explicitly.

Runtime/known parameter types are always explicit in declarations and lambdas.

A non-yielding callable result type may be locally inferred from its body when omitted. A yielding callable must explicitly write `~>` and its result type; suspension is never inferred.

## 9. Type universes

`Index` is the exact nonnegative compile-known integer domain used for structural/static quantities.

```text
Type[N] : Type[N+1]
Type0    = Type[0]
Type     = Type[0]
```

Universes are exact and non-cumulative. There is no `Type : Type` and no universe subtyping.

Runtime types inhabit `Type[0]`. Type values are compile-known.

Type aliases are ordinary bindings of type values and do not create nominal identity.

There is no general subtyping, structural subtyping, nominal inheritance, variance system, or implicit numeric hierarchy. The one-way callable effect compatibility and write-to-read access weakening are explicitly specified capabilities, not general subtyping.

A nonreturning/uninhabited expression may satisfy an expected result type without creating a subtype relation.

## 10. Core semantic substrate and compile-known representation facts

The irreducible bootstrap boundary is exposed through `$[intrinsic]` bindings with normative semantics. `$[intrinsic]` is an implementation mechanism for a specification-defined binding; it is not permission to add implementation-specific language semantics.

The source-visible fundamental semantic categories are exhausted by the following language substrate and the explicitly selected normative CPC/target contracts:

```text
compile-known foundations
- Type[N]
- Index
- target representation facts
- size/alignment queries

fundamental runtime semantic types/type constructors
- Bool
- Unit
- Never
- Rune
- Bits[N]
- Bytes[N]
- Signed[N]
- Unsigned[N]
- Floating[F]
- fixed arrays, written `[N]T`
- safe access types, written `^T` / `^write T` / `^volatile T` / `^write volatile T`
- Storage[N,A]
- Atomic[T]
- Unknown
- Address

irreducible operations
- guaranteed/exact integer conversion
- checked integer conversion
- low-order truncating integer conversion
- safe Access to raw Address exposure
- raw Address formation/arithmetic
- represented raw load/store
- volatile represented raw load/store
- establishment/end of a typed object lifetime in Storage
- Unknown erasure and exact type testing
- Atomic load/store/exchange/compare-exchange
- execution/target leaves explicitly defined by the selected CPC abstract machine or target contract
```

An implementation may use arbitrary private compiler machinery internally, but shall not expose an additional fundamental source semantic type, value category, operation class, or `$[intrinsic]` meaning. An operation supplied by a CPC/target contract is conforming only when that contract normatively defines its source-observable semantics.

Ordinary core abstractions such as `Int`, `Nat`, `Real`, `Endian`, `RealFormat`, Fallible families, containers, allocators, and schedulers are ordinary Musi definitions where their semantics can be derived from the substrate. Nullable, fixed-array, safe-access, range, and callable type/value formation use their normative grammar rather than parallel named source constructors.

Representative bootstrap bindings include:

```musi
$[intrinsic]
let Index Type;

let Endian Type := choice {
    case Little;
    case Big;
};

let Target := record {
    integerWidth Index;
    pointerWidth Index;
    endian Endian;
};

$[intrinsic]
let target Target;
```

`target` is compile-known and has no runtime object.

The language abstract memory byte is 8 bits. Object sizes and alignment are measured in bytes. `target.pointerWidth` is positive and byte-multiple.

The compile-known intrinsic operations `sizeOf[T]` and `alignOf[T]` return the runtime storage size and required alignment of a runtime-representable type T as `Index`. They are invalid for compile-only entities that have no runtime representation.

These capabilities are required so allocator/storage/compiler code can be written in Musi without compiler-private layout knowledge. Their canonical source-visible binding organization is fixed by Annex C of MSR1. A conforming implementation may use private compiler machinery internally, but portable source shall require no compiler-private intrinsic name or operation.

## 11. Core scalar types

### Bool

`Bool` has exactly the values `False` and `True`. Conditions require Bool; there is no truthiness.

Ordinary standalone Bool storage occupies one byte with values represented as 0 and 1. Explicit bit representation may pack Boolean states into `Bits[...]`.

### Unit and Never

`record {}` and `#()` are definitionally Unit. Unit is zero-sized with alignment 1.

`Never` is the canonical uninhabited type. No value of Never can materialize. It may satisfy any expected expression result because execution cannot continue from it.

### Rune

`Rune` is exactly one Unicode scalar value, excluding surrogate code points. Its ordinary representation is `Nat[32]` constrained to valid scalar values.

### Bits and Bytes

`Bits[N]`, N : Index, is an N-bit fixed value. N may be zero. Standalone storage uses `ceil(N/8)` bytes and alignment 1. Bits are representation data, not integers; arithmetic is not implicitly provided.

`Bytes[N]`, N : Index, is exactly N initialized bytes. It is distinct from `[N]Bits[8]`, String, and raw Storage. Byte indexing with `.[Nat]` produces `Bits[8]` value/place semantics where addressability permits.

## 12. Numeric types and arithmetic

The irreducible integer constructors are fixed width:

```musi
$[intrinsic]
let Signed[width Index] Type;

$[intrinsic]
let Unsigned[width Index] Type;

let Int[width Index := target.integerWidth] Type := Signed[width];
let Nat[width Index := target.integerWidth] Type := Unsigned[width];
```

Width must be positive.

`Signed[N]`/`Int[N]` use N-bit two's-complement representation. `Unsigned[N]`/`Nat[N]` use N-bit unsigned representation.

Bare Int/Nat derive their ordinary width from the explicit visible default `target.integerWidth`; pointer width is independent. There are no NativeInt/NativeNat types.

Integer literals denote exact mathematical integers until contextual typing chooses a runtime integer type. Without another expected runtime integer type, a runtime integer literal defaults to Int. A signed literal with prefix `-` is range-checked as the complete signed literal so the minimum negative value is expressible.

Compile-known `Index` arithmetic is exact and unbounded by runtime integer widths; invalid negative results are compile-time errors.

Runtime integer operations:

- `+`, `-`, `*` are checked; representational failure traps;
- Nat subtraction below zero traps;
- signed `/` truncates toward zero;
- unsigned `/` is floor division;
- `%` is remainder satisfying `a = (a / b) * b + (a % b)`;
- signed remainder has the dividend's sign or zero;
- division by zero traps;
- signed minimum divided by -1 traps;
- `shl`/`shr` take a Nat count and require count < width or trap;
- signed `shr` is arithmetic;
- `shl` traps if the mathematical result is not representable;
- `rol`/`ror` take a Nat count and normalize it modulo width.

There is no implicit numeric promotion, widening, narrowing, signedness conversion, or wrapping arithmetic.

The language requires explicit semantic operations for:

- guaranteed representable conversion;
- checked conversion returning Fallible;
- explicit low-order truncation.

Their API spelling is not source grammar.

### Real

```musi
let RealFormat Type := choice {
    case Binary16;
    case Binary32;
    case Binary64;
};

$[intrinsic]
let Floating[format RealFormat] Type;

let Real[format RealFormat] Type := Floating[format];
```

There is no bare Real.

A real literal denotes an exact compile-known mathematical real until contextual typing selects a concrete `Floating[F]`/`Real[F]` representation. A real literal without a uniquely determined contextual floating representation is invalid; Musi has no default floating format. Conversion of a finite literal to the selected IEEE format uses round-to-nearest ties-to-even and the selected format's defined overflow/subnormal semantics.

The three formats implement IEEE 754 binary16/binary32/binary64 semantics including infinities, NaNs, subnormals, and round-to-nearest ties-to-even for ordinary arithmetic. No excess precision may be observable. NaN payload preservation is not portable beyond NaN-ness.

A backend may use native instructions or software lowering, but support is materialized only when source explicitly uses the format. Absence of hardware floating point is not a different language semantics.

## 13. Numeric and Boolean operators

Primitive operator applicability is fixed; operators are not overloadable.

- `+ - * /` apply to equal concrete integer types and equal Real formats; `%` applies to integers;
- unary `-` applies to signed integers and Real, not Nat;
- `~`, `&`, `^`, `|`, `shl`, `shr`, `rol`, `ror` apply to equal concrete Int/Nat widths except that shift/rotate count is Nat;
- `not`, `and`, `or` apply to Bool;
- `=`, `~=` apply to Bool, equal concrete integers, equal Real formats, Rune, String, Bits[N], Bytes[N], and Address;
- `< <= > >=` apply to equal concrete integers, equal Real formats, and Rune;
- aggregate, Access, View, Atomic, Storage, Unknown, and callable equality are not primitive.

Real comparisons use IEEE ordered-comparison semantics; comparisons involving NaN are false except `~=` is true when `=` is false.

String equality is Unicode scalar-sequence equality and is equivalent to byte equality because String is valid canonical UTF-8 without normalization semantics. String ordering is not primitive.

## 14. Ranges

Ranges have one canonical operator family. A lower endpoint, upper endpoint, or both may be absent. When an upper endpoint is present its inclusion is always explicit:

```musi
lower ..< upper
lower ..= upper
lower ..
..< upper
..= upper
..
```

The first form has a bounded lower endpoint and exclusive upper endpoint; the second has a bounded lower endpoint and inclusive upper endpoint; `lower ..` has no upper endpoint; `..< upper` and `..= upper` have no lower endpoint; bare `..` has neither endpoint.

There is no `lower ..<` or `lower ..=` spelling for an absent upper endpoint. Inclusivity is meaningful only for a present upper endpoint.

Semantically a range contains exactly the endpoints that are present plus compile-known lower/upper bound kinds. The specification uses `Range[T,lowerKind,upperKind]` as meta-notation; it is not an alternative source type spelling. `..` without endpoints requires an expected range/domain type from its consumer.

Ranges are non-associative. Indexing and slicing consume ordinary range values; `.[...]` has no separate colon-based slice grammar.

## 15. Tuples, arrays, records

### Tuple

`#(...)` is the tuple datum/pattern/type shape.

```text
#()       zero tuple = Unit
#(T)      one-tuple type
#(A,B)    two-tuple type
```

`#(x)` is a one-tuple datum, not grouping.

Tuple representation is source-order product layout using each element's normal alignment. Tuple identity is structural by arity and element types.

### Array

`[N]T`, with compile-known `N : Index`, is a fixed contiguous N-element value type. The specification uses `Array(T,N)` as semantic meta-notation; `Array[...]` is not an alternative source spelling.

- no hidden length or capacity;
- element stride is `sizeOf[T]`;
- zero length is valid;
- array values have ordinary value/copy semantics;
- indexing requires Nat and checks index < N;
- compile-known impossible indexing is a compile-time error;
- runtime out-of-bounds safe indexing traps.

`#[...]` is the array datum/pattern shape.

A non-empty unconstrained array datum infers `[N]T` only when every member has exactly the same inferred T. Empty `#[]` requires an expected fixed-array type.

### Record

`record { ... }` defines a structural labelled product type.

```musi
let Point := record {
    x Int;
    y Int;
};
```

Fields are immutable by default. `var` makes a field mutable and mutability participates in structural type identity.

```musi
let State := record {
    var count Int;
};
```

Field defaults use `:=`, evaluate in declaration order, and may reference earlier fields only.

A record datum `#{...}` requires an expected record type and never synthesizes a structural record type by itself.

Explicit record fields must appear in the same relative order as their declarations. Omitted fields use defaults. This preserves source-order evaluation without hidden reordering temporaries.

Record update uses explicit replacements followed by at most one final expansion:

```musi
let updated := #{
    field := value,
    ...base,
};
```

Explicit replacements are evaluated in source order, then `base`; the source object is not mutated.

`...` in tuple/array/record patterns denotes structural remainder. Binding patterns must be statically irrefutable.

## 16. Choices

`choice { ... }` defines a nominal sum type. Nominal identity is determined by the lexical `choice` expression site together with the exact compile-known argument values of the parameterized binding instantiation that evaluates that site. Type-valued arguments contribute their own exact type identities.

Consequences:

- repeated evaluation of the same lexical choice site with the same exact compile-known arguments denotes the same nominal type;
- evaluating that site with different compile-known arguments denotes a distinct nominal type;
- distinct lexical choice sites denote distinct nominal types even when their structure and arguments are otherwise identical;
- a case identity is the identity of its enclosing choice plus that lexical case declaration.

This identity relation is part of static semantics and does not imply a source-level Type equality operator or mandatory runtime type metadata.

Exactly three case forms exist:

```text
case None
case Some(value T)
case Red := value
```

A payloadless case is a value. A payload-bearing case is a callable constructor whose payload parameters are immutable, named runtime parameters. Choice payloads have no defaults, known parameters, variadics, or mutable parameters.

An explicit `:=` case value is compile-known and payloadless. Duplicate/overflow case values are errors.

The enclosing choice itself is not implicitly callable.

`choice {}` is a nominal uninhabited type distinct from Never.

An enum is simply a choice with only payloadless cases. A tagged union is a choice with payloads. Raw overlays are not choices.

## 17. Physical layout and representation

Ordinary record fields appear in source order; implementations do not reorder them.

Without representation constraints, natural padding/alignment is target-defined by the selected normative target contract and is always available through compile-known `sizeOf`/`alignOf`.

Core representation attributes:

- `$[aligned(N)]` — minimum N-byte alignment; N is compile-known, positive, power-of-two;
- `$[packed]` — record alignment becomes 1 unless raised by `aligned`, and target-inserted inter-field/trailing padding is removed; fields may therefore be unaligned;
- `$[represented(T)]` — fixes the complete observable representation to T only where this specification or a selected normative contract defines a total value-to-representation mapping;
- `$[tagged(T)]` — fixes only a choice discriminator representation.

`represented(T)` is not a reinterpretation/transmute facility and shall not infer a representation mapping merely from structural similarity. It is invalid unless a normative mapping exists for the attached construct and T.

For a record represented by `Bits[N]`:

- every field shall have a statically fixed represented bit width;
- field widths shall sum exactly to N;
- fields consume bits in source order from the least-significant available bit upward;
- bit numbering is independent of byte endianness;
- there are no implicit spare or padding bits; reserved bits must be source-visible fields, normally `Bits[K]`.

For a payloadless choice, `represented(T)` is legal only when T has a normatively defined discrete representation and every case maps to one unique representable value. For payload-bearing choices, `represented(T)` is invalid unless a separate normative rule defines the entire discriminator-and-payload encoding. `$[tagged(T)]` remains the mechanism for constraining only the discriminator.

Choice implicit discriminator values begin at zero and increment in source order. After an explicit value, the next implicit value is the next representable value.

Ordinary payload choices contain a discriminator plus storage sufficient/aligned for the largest payload. Exact unconstrained layout facts are target-defined by the selected target contract.

Representation optimizations such as niche/tag elision are permitted only when representation is not constrained or otherwise observable. They may reduce cost but may not introduce additional hidden cost or change semantics.

Zero-sized values consume zero data bytes. Distinct zero-sized objects and zero-sized array elements are not required to have distinct raw addresses. Safe semantics never depend on raw address identity of zero-sized objects.

A field can be a readable/writable place without being addressable. Bit fields and unaligned packed fields are not safe-addressable unless the target representation actually satisfies the field type's addressing/alignment requirements. `@` requires an addressable place.

## 18. Text and strings

String literals are valid Unicode text encoded as UTF-8.

```musi
"escaped"
"""raw
multiline"""
```

Normal strings process escapes. Raw triple-quoted strings contain their text exactly after source newline normalization and perform no escape/interpolation/indentation processing.

There is no string interpolation or implicit adjacent concatenation.

`String` is an immutable non-owning valid-UTF-8 view with explicit byte length. It has no semantic NUL terminator, ownership, allocator, or GC requirement. A runtime String therefore visibly carries dynamic-length state; a normal flat-target implementation is base plus byte length. String literals may point entirely into program storage.

There is no primitive integer indexing of String. Rune iteration/decoding is library behavior over valid UTF-8.

Literal value types are fixed:

```text
rune literal        -> Rune
string literal      -> String
byte rune literal   -> Bits[8]
byte string literal -> Bytes[N]
```

For a byte string literal, N is the exact compile-known number of bytes after lexical escape processing. These literal forms do not introduce hidden ownership or dynamic-length representation.

Rune escapes:

```text
\\  \'  \0  \n  \r  \t  \u{H...}
```

Unicode escape has 1-6 hex digits and must denote a valid scalar. `\xHH` is byte-only.

Byte literals/string literals use `b'...'`, `b"..."`, `b"""..."""`; direct byte characters are ASCII-only and `\xHH` emits one byte.

## 19. Places, safe access, raw address

A **place** is a storage-designating expression. A **writable place** permits mutation. An **addressable place** additionally denotes independently byte-addressable storage suitable for safe `Access` formation.

Places may arise from:

- local/static storage bindings;
- mutable/immutable record fields;
- fixed-array/Bytes indexing;
- View indexing;
- safe designation `.^`.

`@place` requires an addressable place and forms non-null safe typed access.

Safe access types use one canonical symbolic type grammar:

```musi
^T
^write T
^volatile T
^write volatile T
```

The specification uses the semantic meta-notation `Access(T,mode,kind)` when discussing the type family. The four source forms denote, respectively, `Access(T,Read,Ordinary)`, `Access(T,Write,Ordinary)`, `Access(T,Read,Volatile)`, and `Access(T,Write,Volatile)`.

`Access[...]`, `^read T`, `^ordinary T`, and any bracket-parameterized access spelling are not alternative source spellings. Defaults are represented only by omission, preserving one canonical spelling per semantic form.

Immutable storage forms read access. Mutable storage forms write access. Write access may satisfy a read-access requirement by representation-preserving capability weakening; the reverse is invalid. This is not general subtyping.

`p.^` designates the referred place. Store through designation requires Write.

Access values are copyable and may alias. Write does not imply uniqueness. There is no safe Access arithmetic.

Every Access has compile-time accessibility bounded by the designated object's storage lifetime. It may not be returned, stored, captured, or exported into a context that can outlive that storage. This requires no runtime lifetime token or borrow counter.

Ordinary flat targets can represent Access as one address-sized value. The language imposes no extra ownership metadata.

### Address

`Address` is the raw target data-address type. It is not Nat and not Access.

Its representation is target.pointerWidth bits for ordinary flat-address targets; exotic target profiles may define a different raw-address representation while preserving the same explicit raw contract.

Safe access may explicitly expose a raw Address. Raw address creation/arithmetic and raw load/store are explicit primitive capabilities supplied through core/target bindings, not implicit conversions.

An Address alone can never create safe `^T`: it proves neither that a live T exists nor its lifetime/provenance.

Raw memory transactions may fault or interact with hardware according to the target contract. They do not introduce optimizer-style language undefined behavior.

## 20. Storage and object lifetime

`Storage[N,A]` is raw capacity of N bytes with at least A-byte alignment. It is not initialized Bytes and is not an ordinary copyable value payload.

`Storage`, `Atomic`, and aggregates containing them are storage-only where ordinary copying would violate their semantics. Storage-only values cannot be passed, returned, assigned, or copied by ordinary value operations; they are manipulated through places/access and their defined primitive capabilities.

The core provides explicit primitive capabilities to establish and end a typed object lifetime in suitably sized/aligned Storage. Their canonical source-visible spelling is fixed by Annex C. These are ordinary bindings supplied by the MSR1 core environment; they do not introduce additional grammar.

The requirements are:

- size/alignment legality is checked statically where known and otherwise explicitly;
- establishing T creates exactly one live T in that region;
- resulting safe Access cannot outlive the Storage domain;
- ending a T lifetime does not itself reclaim the surrounding domain;
- reading uninitialized Storage as T is impossible through safe operations.

There is no global allocator, implicit `new`, implicit heap promotion, GC, hidden allocation header, or general safe `free(Access)`.

Independent reclamation abstractions must visibly carry whatever ownership/generation/tracking representation they need. Whole-region reclamation is safe only after no safe access into the region can remain usable.

## 21. Views and slicing

`View[T,mode,kind]` is an explicit non-owning contiguous runtime-length view.

It carries:

- designation of contiguous T storage;
- runtime length;
- Access mode/kind;
- accessibility no longer than the backing storage.

It has no capacity, allocator, ownership, or implicit growth semantics. A normal flat-target representation is base plus length. Arrays never silently become Views.

`view.[i]` uses Nat indexing and traps on out-of-bounds access.

`value.[range]` may form a checked View/subview where the base type supports contiguous slicing, including bounded and open-bound range forms such as `lo..<hi`, `lo..=hi`, `lo..`, `..<hi`, `..=hi`, and `..`. Bounds are validated before the view is produced. Zero-length Views are valid.

## 22. Nullable, Fallible, trap

Safe Musi has no language-level undefined behavior.

Failure categories are:

1. statically knowable invalidity -> compile-time error;
2. expected recoverable runtime failure -> Fallible;
3. absence -> Nullable;
4. violated required execution condition -> trap;
5. raw/foreign operations -> explicit external contract.

A trap terminates the current execution path and has type Never. It does not unwind and therefore does not execute pending `defer` expressions.

The canonical nullable type syntax is prefix:

```musi
?T
```

The specification uses `Nullable(T)` as semantic meta-notation for this type family; `Nullable[...]` and postfix `T?` are not source spellings.

Fallible remains an ordinary choice abstraction whose exact source declaration is defined by the core library rather than by a second nullable-style type grammar.

`receiver?.member` maps member selection over one Nullable layer:

- None -> None;
- Some(x) -> Some(x.member).

It does not implicitly flatten a member that is itself Nullable.

No force/propagation postfix operator exists.

## 23. Match and patterns

`match` is exhaustive and evaluates exactly one selected arm body.

```musi
match value {
    case None => a,
    case Some(x) => b(x),
}
```

Choice patterns resolve case names against the scrutinee's nominal choice and destructure the declared payload shape.

Binding destructuring is statically irrefutable and has no runtime match/failure machinery. The right-hand expression is evaluated exactly once, then decomposed structurally and bindings are established in source order.

For tuple and fixed-array binding patterns, explicit elements select the corresponding structural elements. `...name` binds the fixed structural remainder as a value of the same structural category; for arrays this is `[N]T` with compile-known remaining N. `..._` ignores the remainder and does not require a remainder value to materialize.

For record binding patterns, `field` is shorthand for selecting field `field` and binding it with the same name. `field := pattern` selects source field `field` and applies `pattern` to that value. `...name` binds a structural record value containing the unselected fields in original source-field order and with their exact field types/mutability; `..._` ignores them. Duplicate selected fields and nonexistent fields are invalid.

`let` destructuring follows ordinary value semantics. `var` destructuring creates independent mutable storage initialized from each selected component value; it never creates aliases into the source value. Aliasing requires explicit `Access`.

Typed binding patterns:

```musi
case value T => ...
case _ T => ...
```

perform exact runtime-type narrowing when the scrutinee is Unknown. There is no coercion.

Literal patterns and compile-known literal ranges are supported. Guards use `when`; guarded arms do not contribute to exhaustiveness.

Or-patterns are absent.

Unreachable/redundant unguarded arms are compile-time errors.

Without an expected result type, all reachable arm bodies must infer exactly the same result type; Never arms may satisfy any expected result.

## 24. Control flow

One-way conditional:

```musi
a when condition
```

requires `a : Unit` (or Never) and returns Unit.

Two-way conditional:

```musi
a when condition else b
```

requires exactly compatible branch result types or an expected type that resolves both exactly.

`while` is the only primitive loop:

```musi
while condition (
    ...
)
```

Condition is Bool. Body result must be Unit/Never. `while` returns Unit.

`leave` exits the nearest while; `cycle` continues the nearest while. Both are control-transfer expressions of type Never at their occurrence.

There is no primitive `for`, `foreach`, `loop`, repeat, or do-while construct.

### defer

`defer expression` requires expression to produce Unit/Never when eventually evaluated.

When execution reaches a `defer` site, that lexical defer site becomes active for the nearest enclosing computation/callable scope. The deferred expression itself is evaluated only when that scope exits normally or through structured `leave`/`cycle`, in LIFO order. Its referenced local bindings remain alive until the defer executes.

A deferred expression observes the values/storage present at exit; Musi does not silently snapshot arbitrary values at registration time. If a snapshot is required, bind the snapshot explicitly before `defer`.

A trap does not unwind and does not run deferred expressions.

## 25. Operators and precedence

Postfix operations bind strongest and compose left-to-right:

```text
(...)  [...]  .[...]  .name  ?.name  .^
```

Prefix operations bind next and nest rightward:

```text
@  -  ~  not  known
```

Arithmetic hierarchy:

```text
* / %
+ -
```

Bitvector hierarchy:

```text
shl shr rol ror
&
^
|
```

Arithmetic and bitvector infix families are intentionally incomparable. An unparenthesized expression may not cross between them. Parentheses/computation make the intended grouping explicit.

Range operators `..<`/`..=`/`..` are non-associative.

Comparisons `= ~= < <= > >=` are non-associative.

`and` binds above `or`; both short-circuit.

Conditional `when ... else ...` is weaker than Boolean operations and associates on the `else` side.

Mutation `:=` is weakest and non-associative.

Callable arrows are type grammar, not value precedence. `->` and `~>` associate right.

## 26. Callable types and anonymous callables

Callable type syntax distinguishes arity from tuple passing:

```text
A -> B          one runtime parameter
(A, B) -> C     two runtime parameters
#(A, B) -> C    one tuple parameter
() -> C         zero runtime parameters
```

`A -> B -> C` means `A -> (B -> C)`.

`~>` is the yielding callable arrow and has the same arity/association rules.

Callable parameters/results use ordinary value semantics. Use Access explicitly when indirection is desired.

Anonymous callables use `lambda`:

```musi
lambda(x Int, y Int) Int => x + y
lambda[T Type](x T) T => x
lambda() ~> Unit => (step(); yield)
```

Runtime/known parameter types are explicit. A non-yielding result type may be inferred. Yielding lambdas must explicitly write `~>` and the result type.

### No runtime capture

Callable values do not implicitly capture automatic runtime bindings.

A lambda or named callable may reference:

- its own parameters/local bindings;
- compile-known bindings;
- callable/type bindings;
- program-lifetime `static` storage.

It may not refer to an outer automatic runtime binding whose lifetime would require a hidden environment. Pass a value or Access explicitly instead.

Consequently a callable value has no hidden closure environment. There is no bound-method callable object.

Callable equality/identity operations are absent.

Ordinary Musi callables are not variadic.

## 27. Receiver-prefix methods

Receiver methods are module-scope ordinary callable bindings attached at compile time to a named receiver binding.

### Instance receiver

A non-parameterized receiver type:

```musi
let (self Point).length() Int := ...;
```

A generic receiver family declares the receiver constructor's known parameters in the receiver prefix:

```musi
let (self Box[T Type]).value() T := ...;
```

The receiver parameter list must mirror the target type constructor's known parameter arity/types; it has no defaults.

Runtime receivers are place aliases, not copied hidden values. Read-only receiver:

```musi
(self T)
```

Writable receiver:

```musi
(var self T)
```

A runtime method call therefore requires an addressable receiver place. Writable receiver calls additionally require a writable place. `self` aliases that place for the duration of the call; no heap object or environment is created.

### Type-constructor receiver

If the named receiver binding itself declares known parameters, omitting receiver parameters denotes the compile-known constructor binding itself:

```musi
let (self ?T).of(value T) ?T := ...;
```

which is invoked as:

```musi
(?T).of(value)
```

`self` is compile-known in a type-constructor receiver and contributes no runtime argument.

Concrete non-parameterized type bindings use `(self T)` as instance receivers; constructor-style APIs for them remain ordinary free bindings.

### Lookup

Receiver methods attach to the named receiver binding, not merely its underlying structural representation. Exported receiver methods travel with an exported receiver binding through module import.

At a dotted expression, an actual structural/module member is resolved first. Receiver-method resolution is attempted only for an immediate method invocation (`receiver.name...(...)`), never to create a bound callable value.

For one receiver binding and method name, at most one applicable exported/visible receiver method may exist. There is no overload ranking, virtual dispatch, vtable, or runtime method lookup.

## 28. Yielding callables and cooperative execution

`yield` is the sole primitive cooperative scheduling transfer.

```text
A -> B   cannot directly or transitively yield
A ~> B   may directly or transitively yield
```

A `->` callable may satisfy a corresponding `~>` requirement; `~>` may never satisfy `->`. This is effect capability compatibility, not general subtyping.

`yield` is valid only in a `~>` callable and evaluates to Unit when that execution context is resumed.

Suspension is stackful: the entire yielding call chain and its automatic storage remain alive.

The language does not create an execution context, task, future, scheduler, poll object, or heap allocation merely because a `~>` callable exists.

A cooperative execution domain executes at most one Musi context at a time. Ordinary context transfer occurs only at explicit `yield`, context completion, or explicit host/runtime resumption of a non-running context.

Task/context creation and backing storage are explicit runtime/library operations outside language syntax. Exhausting explicitly supplied execution storage traps rather than corrupting memory.

Cancellation, join, channels, sleep, scheduling policy, and task containers are library/runtime facilities, not language constructs.

## 29. Atomic and volatile semantics

`volatile` and atomicity are separate.

`$[volatile]` on storage means every source-level read/write of that storage is an observable target memory transaction. Such transactions occur exactly once when the source operation executes and preserve source execution order relative to other volatile transactions in that execution context. Volatile does not imply inter-context synchronization.

`Atomic[T]` is a storage-only atomic object type for target-supported scalar T. Ordinary value copy/load/store of Atomic[T] is invalid; access occurs only through the defined atomic primitive operations.

Atomic semantics include:

- load;
- store;
- exchange;
- compare-exchange;

with explicit memory orders:

```text
Relaxed Acquire Release AcqRel SeqCst
```

Meaning:

- Relaxed: atomicity only;
- Acquire: subsequent memory actions cannot move before the acquiring operation and a matching reads-from relation synchronizes with Release;
- Release: preceding memory actions cannot move after the releasing operation;
- AcqRel: both;
- SeqCst: Acquire/Release semantics plus one total order among SeqCst atomic operations.

Operation-specific invalid order combinations are compile-time errors.

A target must implement requested Atomic[T] semantics without hidden dynamically allocated/shared synchronization state; otherwise Atomic[T] for that T is unsupported on that target. Native atomic instructions or bounded target mechanisms such as interrupt exclusion may implement the semantics where valid.

## 30. Interrupts

Interrupt entry is construct metadata on a module-scope non-yielding callable:

```musi
$[onInterrupt(Timer0)]
let timerHandler() Unit := ...;
```

Requirements:

- zero runtime parameters;
- non-yielding `->` semantics;
- no direct/transitive call to `~>`;
- target-defined interrupt identifier and entry ABI;
- externally initiated execution that may preempt cooperative execution.

An interrupt handler is a code binding and does not require `static` data storage.

Shared state between interrupt/external agents and ordinary execution uses explicit Atomic, volatile/raw transactions, or target synchronization facilities according to the target contract.

## 31. Unknown

`Unknown` is the explicit bounded runtime type-erasure boundary.

It is **not** an owning arbitrary-size box.

Semantically an Unknown contains exactly:

- a non-owning read designation of an existing runtime object;
- an exact runtime type-identity token.

It therefore has accessibility no longer than the designated object, like safe Access. Erasure is explicit and may only be formed from existing addressable ordinary storage through a defined core intrinsic/API operation. The API spelling is not language grammar.

Unknown does not copy, allocate, own, or resize the erased payload. On ordinary flat targets its normal representation is two pointer-width components: location and type identity. Required type descriptors are static metadata and are emitted only for concrete types actually erased/tested.

Typed `match` narrowing checks exact type identity. On a successful typed pattern, reading/binding T follows ordinary T value semantics and therefore any T copy is visible from the typed binding itself.

Unknown has no primitive dynamic member lookup, indexing, call, arithmetic, comparison, hashing, or serialization.

Unknown cannot erase raw Address, safe Access, Storage, Atomic, module values, or compile-known Type/Index values.

Inference never chooses Unknown.

A program that never uses Unknown requires no Unknown descriptors or dynamic-type runtime support.

## 32. Attributes

Construct metadata is one `$[...]` group at an attachment point:

```musi
$[
    packed,
    aligned(4),
]
let Header := record { ... };
```

`$[` is a compound token. Items are comma-separated lowerCamelCase properties/relations. The generic metadata grammar does not make core attribute contracts open: every core language-semantic attribute below has the exact attachment/argument contract specified here, and any other use is invalid.

Core language-semantic attribute names are:

```text
foreign aligned packed inSection retained targeting intrinsic
represented tagged volatile at onInterrupt
```

### intrinsic

Legal target: a binding. Arguments: none.

Marks a binding whose irreducible value/type constructor/operation is supplied directly by the implementation. The binding shall correspond to the closed semantic substrate in section 10 or to an explicitly selected normative CPC/target contract. It may omit a body. The attribute cannot create implementation-specific language semantics.

### foreign

Legal target: a module-scope callable or static data binding. Arguments: exactly the compile-known ABI/linkage descriptor required by the explicitly selected normative ABI profile.

The ABI profile fixes calling/representation/linkage behavior. A foreign-defined binding may omit its Musi body. Foreign pointer values are raw Address unless that ABI profile explicitly establishes a stronger wrapper contract.

### targeting

Legal target: a binding. Argument: exactly one compile-known Bool.

The attached binding is present when true and absent when false. No runtime branch or metadata is created.

### aligned

Legal target: a runtime-representable type definition or static storage binding. Argument: exactly one compile-known positive power-of-two Index. Physical semantics are in section 17.

### packed

Legal target: a record type. Arguments: none. Physical semantics are in section 17.

### represented

Legal target: a record or choice for which section 17 or an explicitly selected normative representation contract defines a total mapping. Argument: exactly one compile-known Type. Any other attachment/type combination is invalid.

### tagged

Legal target: a choice. Argument: exactly one compile-known Type that is valid as a discriminator representation under section 17 or the selected normative representation contract.

### volatile

Legal target: addressable data storage, including an `$[at(...)]` static binding. Arguments: none. Transaction semantics are in section 29.

### at

Legal target: a module-scope `static let` or `static var` runtime-representable data binding. Argument: exactly one compile-known Address.

`at(address)` supplies externally established storage for the binding at that raw location. Such a binding may omit its Musi initializer/body. Musi performs no allocation and no initialization write for the binding. The selected target contract shall establish that the location, lifetime, alignment, accessibility, and object representation satisfy the declared type; otherwise compilation fails.

`at` does not imply `volatile`. Conflicting live safe objects at the same location are invalid. Overlapping reinterpretation belongs to raw memory operations.

### inSection

Legal target: module-scope code or static data binding. Argument: exactly one compile-known section identity defined by the selected target contract. The requested placement shall be honored or compilation fails.

### retained

Legal target: module-scope code or static data binding. Arguments: none. The binding shall be retained according to the selected target/link contract or compilation fails.

### onInterrupt

Legal target: a module-scope callable satisfying section 30. Argument: exactly one compile-known interrupt identity defined by the selected target contract.

Tool-specific metadata is outside the core registry and may not change Musi value/type/memory semantics.

## 33. Pragmas

Compilation-context metadata uses standalone `%[...]` source items:

```musi
%[
    diagnostic(...),
];
```

`%[` is a compound token. Items are comma-separated peers and act in strict source order.

Core Musi assigns no runtime semantic capability through pragmas. A pragma may control diagnostics, compiler policy, analysis, or other compilation context, but it may not silently alter value semantics, typing, arithmetic, representation, ABI, storage duration, lifetime, synchronization, suspension, or target facts of otherwise identical source.

Consequently pragma inventories may evolve in tooling without reopening language semantics.

## 34. Modules and import

`import` is valid only at module/source scope. Modules and their dependencies are statically resolved and acyclic.

A module value is a compile-known structural record of exported binding references. It has no mandatory runtime representation and cannot be materialized as arbitrary runtime data.

Therefore:

```musi
let module := import "./module.ms";
let #{TypeName, functionName} := import "./module.ms";
```

are compile-time binding operations; they do not copy a runtime module aggregate.

A reachable module initializes exactly once, dependency-first and then in strict top-to-bottom source order. Top-level observable effects are retained.

Exports:

- compile-known/type/callable bindings may export directly;
- immutable runtime data that must persist after module initialization is `export static let`;
- mutable storage is never exported directly (`export var` / `export static var` invalid); expose explicit Access/callable interfaces instead;
- exported receiver methods travel with their exported receiver binding as compile-time method metadata.

Import specifier categories:

```text
./...   or ../...    relative source-unit specifier
musi:...            reserved built-in module specifier
other non-relative  environment-resolved external module specifier
```

`musi:` is the only language-reserved prefix and cannot be claimed/remapped/shadowed by an external resolver.

The package/dependency mechanism used to resolve non-relative external specifiers is not language semantics.

## 35. Source files and specifications

`.ms` denotes Musi source.

An optional sibling `.spec.ms` source unit is an authoritative exported contract for its implementation module.

If present:

- dependents use the spec's exported bindings/method metadata;
- implementation must provide exactly matching public types, known/runtime parameters, effects, receiver signatures, and contract-relevant attributes;
- implementation may add private bindings;
- bodyless declarations are legal in `.spec.ms` by source-unit role.

Outside `.spec.ms`, a body may be omitted only when an established semantic mechanism supplies it: `intrinsic`, `foreign`, or `at`.

File discovery, project manifests, tests, build profiles, and package layouts are tooling concerns.

## 36. Foreign boundary

Foreign interaction is explicit through `$[foreign(...)]` and raw representation types.

The language guarantees:

- no implicit mapping from foreign pointers to safe Access;
- ABI-reifiable types/aggregate lowering are defined by the selected ABI profile;
- Musi default aggregate layout is not automatically a foreign ABI layout;
- foreign entry into Musi is non-yielding;
- foreign/raw behavior never creates safe lifetime/provenance guarantees by itself.

Concrete ABI identities and platform calling conventions are instances of the ABI contract schema in Part III; they are not Musi grammar. Variadics are available only when the selected ABI contract defines them. String helpers are ordinary libraries. Any trampoline required for a Musi-defined foreign entry is an implementation consequence visible from `$[foreign(...)]` and may not impose cost when no such entry exists.

## 37. Normative outcome categories and target dependence

Normative outcome terms are closed:

- **defined** — fully determined by this specification;
- **target-defined** — fixed by the explicitly selected normative Musi target contract before source execution;
- **foreign-defined** — fixed by an explicitly selected normative ABI/external contract;
- **invalid program** — compilation shall reject;
- **trap** — defined terminal execution outcome;
- **raw external behavior** — behavior of an explicitly requested hardware/foreign transaction under its selected external contract.

Musi has no residual `implementation-defined`, `unspecified`, or language-level undefined-behavior category. A compiler implementation is not a semantic authority. Any permitted variation that can affect programmer-observable behavior shall be assigned by this specification or by an explicitly selected normative target/CPC abstract machine/ABI contract.

For fixed specification version, source text, explicit compilation inputs, selected target contract, selected CPC contract, and selected ABI contracts, conforming implementations shall agree on every programmer-observable semantic consequence required by those specifications.

Target-dependent facts must be available during compilation. Target dependence never authorizes a backend to invent hidden runtime facilities.

## 38. Freestanding / zero-hidden-cost conformance

A conforming implementation must support ordinary Musi programs on a freestanding target without requiring an OS, global heap, GC, scheduler, RTTI registry, exception unwinder, filesystem, clock, thread runtime, dynamic loader, or floating runtime when the program does not use semantics requiring them.

The following ordinary subset must admit a runtime with no general allocator/runtime framework:

```text
Int/Nat and fixed integers
Bool/Unit/Never
records, choices, fixed arrays
local let/var
module-scope static let/var
ordinary -> callables and noncapturing lambdas
when, match, while, leave, cycle, defer
safe Access
raw Address/MMIO
modules and compile-known facilities
```

Incremental facilities are pay-for-use:

```text
View        -> explicit dynamic length state
String      -> explicit base + byte length semantics
Unknown     -> explicit designation + type identity metadata
~> context  -> explicitly provisioned execution storage/context state
Atomic      -> requested target atomic mechanism
Real[...]   -> requested floating implementation
Storage APIs-> explicit user/library storage metadata only as selected
```

Constrained systems with approximately 64 KiB address spaces or memory budgets are an explicit architectural feasibility floor; 400–512 KiB microcontrollers shall be comfortable implementation targets. This is not a weakened language profile. Exact executable size is not a conformance property, but MSR1 facilities shall not impose architectural requirements that make such implementations inherently infeasible. Implementations scale monotonically upward from this foundation.

## 39. Literal lexical contracts

These scanner contracts complete the lexer-produced `integer-token`, `real-token`, `rune-token`, `string-token`, `raw-string-token`, `byte-token`, `byte-string-token`, and `raw-byte-string-token` terminals in the EBNF.

### Integer

Digit runs may contain `_` only between two valid digits of that radix.

```text
decimal:   DIGIT (DIGIT | _ DIGIT)*
binary:    0b BINDIGIT (BINDIGIT | _ BINDIGIT)*
octal:     0o OCTDIGIT (OCTDIGIT | _ OCTDIGIT)*
hex:       0x HEXDIGIT (HEXDIGIT | _ HEXDIGIT)*
```

Radix prefixes are lowercase only. There are no numeric suffixes.

### Real

Real literals are decimal only and must contain either a fractional part or exponent.

```text
fractional: decimalDigits "." decimalDigits exponent?
exponent:   decimalDigits "e" ("+" | "-")? decimalDigits
```

`e` is lowercase only. There are no hex floats or real suffixes.

Requiring digits on both sides of the fractional dot keeps member-access lexing unambiguous (`1.foo` is integer/member syntax, not a partial real token).

### Rune

Rune literal delimiters are single quotes. After escape processing the literal must denote exactly one Unicode scalar value.

A literal newline is invalid inside an ordinary rune/string literal.

### String

Ordinary strings use `"..."` and process the defined escapes. Raw strings use `"""..."""` and perform no escape/interpolation/indentation transformation beyond source newline normalization.

### Byte forms

Byte rune/string forms are prefixed with `b`. Direct source characters are ASCII only. `\xHH` emits exactly one byte. After escape processing a byte rune literal contains exactly one byte.

### Escapes

Ordinary text escapes include:

```text
\\  \'  \"  \0  \n  \r  \t
```

Rune/String additionally allow `\u{H...}` with 1-6 hex digits denoting a valid Unicode scalar. Byte forms do not allow Unicode escapes.

---
