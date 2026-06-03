# LOCKED_SYNTAX.md — Consolidated

## 1. Core Identity

Musi is a small systems language with a small core.

Musi is expression-first:
- statements are not a separate semantic category
- a top-level expression terminated by `;` is an accepted top-level item
- `;` may discard a value or sequence expressions
- definitions are expressions
- control flow is expression-based

Musi targets SEIL bytecode directly. SEIL is the canonical lowered form, similar in role to CIL. Musi source should lower to SEIL so future SEIL-to-Musi decompilation can recover near-identical source when metadata is preserved.

There is no IR layer between Musi source and SEIL.

## 2. Parsing, Lexing, And Notation

Syntax must preserve:
- maximal-munch lexical design
- one-token-lookahead parsing
- no speculative parsing beyond one token
- no syntax retained only because existing or conventional languages use it

If a form needs more than one token of lookahead, it is not accepted.

Grammar snippets use W3C XML 1.0 EBNF notation and document accepted surface shapes. Omitted forms are not accepted unless another locked section adds them. Snippets are documentation grammar, not the generated parser grammar.

```ebnf
A       ::= B C
A       ::= B | C
A       ::= B?
A       ::= B*
A       ::= B+
A       ::= "token"
A       ::= B /* comment */
IDENT   ::= /* lexical identifier token */
EXPR    ::= /* expression production defined by final grammar */
TYPE    ::= /* type-expression production defined by final grammar */
PATTERN ::= /* pattern production defined by final grammar */
ATTR    ::= /* attribute production defined by final grammar */
```

## 3. Keywords

A keyword is a hard-reserved source word required to introduce or disambiguate a grammar form.

A word is not a keyword merely because it is built in, compiler-owned, common, or standard-library-provided. Operators, compiler intrinsics, methods, traits/shapes, sum types, product types, and built-in types are not keywords unless hard-reserved as grammar introducers.

Hard/form keywords locked so far:

```text
case
cycle
data
defer
else
erased
export
fixed
import
known
leave
let
match
mut
opaque
shape
when
while
yield
```

Count: 19.

Contextual or non-keyword decisions:
- `in` is a contextual word operator, not a form keyword.
- `as` is a contextual pattern keyword, not cast syntax.
- `await`, `spawn`, and `task` remain ordinary identifiers, method names, shape names, or data names.
- `import` and `export` are hard keywords. `import` takes in; `export` puts out.
- `known import` is compile-time acquisition/import.
- Module boundary forms affect source shape and SEIL/decompilation metadata.

Not core keywords:

```text
unsafe
pin
recur
for
break
continue
next
trait
hidden
static
const
fn
type
struct
enum
class
impl
and
or
xor
not
is
```

## 4. Comments

Comment spellings are locked.

```ebnf
line-comment       ::= "--" line-comment-text
line-doc-comment   ::= "---" line-comment-text
line-module-doc    ::= "--!" line-comment-text
block-comment      ::= "/-" block-comment-body "-/"
block-doc-comment  ::= "/--" block-comment-body "-/"
block-module-doc   ::= "/-!" block-comment-body "-/"
block-comment-body ::= (block-comment | block-doc-comment | block-module-doc | block-comment-char)*
```

Maximal munch applies:
- `--!` is a module doc comment.
- `---` is a doc comment.
- `/--` is a block doc comment.
- `/-!` is a block module doc comment.

Block comments, block doc comments, and block module doc comments participate in the same nesting system.

Rules:
- line comments inside block comments are comment text
- nested block comments use a linear depth counter
- unterminated nested block comments are diagnostic errors
- module docs are supported separately from item docs

## 5. Binding

`let` is the universal binding form. It binds values, functions, data definitions, shape definitions, module/import results, compile-time values, runtime values, and attached receiver methods.

There is no separate `fn`, `type`, `struct`, `enum`, `class`, `impl`, `const`, or `static` keyword.

```ebnf
let-expr        ::= "let" bind-head generic-param-list? type-annot? param-list? result-type? ":=" EXPR
                  | "let" receiver-head "." IDENT generic-param-list? param-list result-type? ":=" EXPR
bind-head       ::= IDENT | "_" | operator-name | PATTERN
receiver-head   ::= "(" IDENT type-annot ")"
generic-param-list ::= "[" generic-param-list-body? "]"
generic-param-list-body ::= required-generic-param ("," required-generic-param)* ("," default-generic-param)* ","?
                          | default-generic-param ("," default-generic-param)* ","?
required-generic-param ::= IDENT type-annot?
default-generic-param  ::= IDENT type-annot? ":=" EXPR
param-list      ::= "(" param-list-body? ")"
param-list-body ::= required-param ("," required-param)* ("," default-param)* ","?
                  | default-param ("," default-param)* ","?
required-param  ::= IDENT type-annot | IDENT type-annot?
default-param   ::= IDENT type-annot? ":=" EXPR
result-type     ::= type-annot
type-annot      ::= ":" TYPE
```

Defaults must be trailing in every parameter list, including function parameters, method parameters, constructor-like parameters, and variant payload parameters.

Bracketed generic parameters belong between the binding name and ordinary call parameters.

```musi
let name[A : Type, N : known Nat](value : A) : [N]A := ...;
let (self : Point).make[T : Type]() : Point := ...;
```

Generic call arguments use the same bracket-before-call shape:

```musi
name[Int, 4](value)
point.Point.make[Int]()
```

### Binding Qualifiers

Binding syntax is plain. `known`, `fixed`, and `mut` do not appear before `let` and do not appear between `let` and the binding name.

```ebnf
let-binding        ::= "let" bind-head type-annot? ":=" EXPR
qualified-binding  ::= "let" bind-head ":" qualified-type ":=" EXPR
qualified-rhs-bind ::= "let" bind-head ":=" qualified-expr
qualified-type     ::= "known"? "fixed"? "mut"? TYPE
qualified-expr     ::= "known" EXPR | EXPR
```

Canonical type qualifier order:

```text
known fixed mut TYPE
```

Other orders are absent from accepted grammar and may be canonicalized by diagnostics/formatting.

If a binding has no annotation, inference preserves the qualified type of the right-hand side. It does not invent or strip qualifiers.

## 6. Regions And Separators

### Computation Regions

Parentheses delimit computation regions.

```ebnf
computation-region ::= "(" computation-body? ")"
computation-body   ::= EXPR (";" EXPR)* ";"?
```

Semicolon inside a computation region sequences/discards:
- `(step1(); step2())` produces the value/effect of `step2()`.
- `(step1(); step2();)` discards `step2()` and produces `Unit` or the corresponding empty stack effect.

Leading semicolon is not accepted because it implies an empty computation step.

### Structural Regions

Curly braces delimit structural regions. They define members, fields, variants, cases, or rule tables. They are not sequential computation bodies.

```ebnf
structural-region ::= "{" structural-body? "}"
structural-body   ::= structural-member (";" structural-member)* ";"?
structural-member ::= data-field | data-case | shape-member | match-case
```

Structural semicolon is a member/rule terminator, not a discard operator.

### Trailing Separators

Trailing separators are allowed where the separator follows an item. Leading separators are not allowed.

```ebnf
comma-items     ::= EXPR ("," EXPR)* ","?
semicolon-items ::= structural-member (";" structural-member)* ";"?
```

Comma-list positions use `X ("," X)* ","?`. Structural regions use `X (";" X)* ";"?`. Computation regions use `;` as sequencing/discard.

## 7. Control Flow

### Conditional Expressions

`when` is the conditional guard operator.

```ebnf
total-conditional ::= non-when-expr "when" non-when-expr "else" EXPR
guarded-emission  ::= non-when-expr "when" non-when-expr
non-when-expr     ::= /* expression production excluding unparenthesized when-expr */
```

Rules:
- condition must be `Bit`
- `when` is postfix guard syntax, not prefix syntax
- total conditional branches must have compatible type/stack effect
- `else` is the explicit fallback branch
- no `then` keyword exists
- guarded emission has zero-or-one emission shape
- guarded emission is accepted only in contexts that consume zero-or-one emission
- no hidden `Maybe`, `Unit`, bottom, or union is synthesized
- unparenthesized nested `when` is not accepted in guarded value or condition position
- parentheses are required for nested conditionals

Examples:

```musi
value when ready else fallback
value when ready
value when ready else (other when available else fallback)
(value when ready else other) when enabled else fallback
```

### Loops

`while` is the only source loop form.

```ebnf
while-expr   ::= "while" EXPR computation-region
loop-control ::= "leave" | "cycle"
```

Rules:
- `while` is a zero-or-more conditional repetition expression.
- condition must be `Bit`.
- body is a computation region.
- `while` produces `Unit`.
- `leave` exits the nearest enclosing `while`.
- `cycle` skips remaining body and proceeds to the next condition check.

There is no `for`, `break`, `continue`, `next`, or `recur` keyword in core.

Iterable loops are expressed through ordinary functions, methods, and shapes. Postcondition repetition can be expressed by sequencing an initial body with a `while` loop or by named library helpers.

`recur` does not earn a keyword slot because it would create a one-off postfix binding modifier such as `let recur N := ...`, violating the locked binding qualifier rule and duplicating ordinary recursion.

`pin` is not a core keyword. Stable-address semantics are handled by `fixed`; scoped temporary non-moving access must be justified against `fixed`.

### Defer And Yield

```ebnf
defer-expr ::= "defer" EXPR ("when" EXPR)?
yield-expr ::= "yield" EXPR?
```

`defer` registers an expression to run when the current computation region/scope exits. It produces `Unit`.

Rules:
- `defer` cleanup runs on normal exit and loop-control exits such as `leave` and `cycle`.
- Exact cleanup ordering remains part of runtime/control-flow design.
- Guarded cleanup uses existing `when` syntax.

```musi
defer file.close();
defer lock.release() when locked;
```

`yield` is a core keyword/expression for resumable/generator-compatible contexts. It is not an ordinary function call.

Rules:
- `yield expr` suspends or emits through the enclosing resumable protocol.
- outside a resumable/generator-compatible context, `yield` is a diagnostic.
- yielded value type must match the enclosing resumable/generator output type.
- bare `yield` is accepted only when yielded type is `Unit`.
- `yield` produces `Unit` locally after handing off the value.
- suspension is not scope exit.
- `defer` does not run at suspension points.
- pending defers run on final scope exit, close, drop, or cancel according to final resumable runtime rules.

Concurrency is protocol/capability driven, not hard-coded syntax. `yield` is the only core suspension keyword. `Task`, `Scheduler`, `Resumable`, `Generator`, and `Stream` are library/runtime shapes or data types.

`await`, `spawn`, and `task` remain ordinary names.

## 8. Match And Patterns

### Match

Pattern matching uses `match`. Each arm starts with `case` and ends with `;`. `=>` is the body/result arrow for both match arms and lambdas.

```ebnf
match-expr        ::= "match" EXPR "{" match-case+ "}"
match-case        ::= "case" case-pattern-list case-guard? "=>" EXPR ";"
case-pattern-list ::= PATTERN ("," PATTERN)* ","?
case-guard        ::= "when" EXPR
lambda-expr       ::= '\' param-list type-annot? "=>" EXPR
```

Rules:
- `;` after a `case` arm terminates the structural case rule.
- It does not discard the selected arm value.
- To discard inside an arm, use a computation region with a final semicolon.
- Pattern alternatives use comma separation inside one `case`.
- `|` is not used for pattern alternatives.
- Alternatives in one `case` share the same guard and body.
- Pattern bindings must be compatible across alternatives: same names with compatible types in every alternative that reaches the shared body.

### Exhaustiveness

`match` is exhaustive by default. Non-exhaustive `match` is a semantic error.

Rules:
- finite sum `data` matches must cover all variants or include wildcard catch-all
- guarded cases do not count as unconditional coverage
- catch-all is `case _`
- there is no `case else` syntax
- `else` remains only the fallback marker for `when ... else`

### Guard Evaluation

Cases are tested top-to-bottom. Within one `case`, comma-separated pattern alternatives are tested left-to-right.

Guard rules:
- pattern alternative is tested first
- guard runs only after its pattern alternative matched
- guard can reference bindings from the matched pattern
- guard expression must be `Bit`
- if pattern matches but guard is false, matching continues
- guards do not run for non-matching patterns
- first matching unguarded case or guard-true case wins
- guarded cases are conditional coverage for exhaustiveness

### Pattern Grammar

Patterns mirror datum syntax where they destructure values. Let binding heads may be patterns, so ordinary binding identifiers are identifier patterns.

```ebnf
pattern              ::= alias-pattern
alias-pattern        ::= pattern-primary type-annot? ("as" identifier-pattern)?
pattern-primary      ::= wildcard-pattern | identifier-pattern | literal-pattern | variant-pattern | tuple-pattern | record-pattern | array-pattern | rest-pattern
wildcard-pattern     ::= "_"
identifier-pattern   ::= IDENT
literal-pattern      ::= INT | FLOAT | STRING | RUNE
variant-pattern      ::= "." IDENT pattern-args? | TYPE "." IDENT pattern-args?
pattern-args         ::= "(" (pattern ("," pattern)* ","?)? ")"
tuple-pattern        ::= "#(" (pattern ("," pattern)* ","?)? ")"
record-pattern       ::= "#{" (record-pattern-field ("," record-pattern-field)* ","?)? "}"
record-pattern-field ::= IDENT (":=" pattern)?
array-pattern        ::= "#[" (pattern ("," pattern)* ","?)? "]"
rest-pattern         ::= ".." identifier-pattern?
```

Examples:

```musi
case #(x, y) => expr;
case #{ name := n, age := _ } => expr;
case #[head, ..tail] => expr;
case #[first, second, ..] => expr;
case .Some(value) => expr;
case Maybe.Some(value) => expr;
case id : UserId => id.raw;

let #(x, y) := point;
let #{ name := n, age := a } := person;
```

Record pattern shorthand:

```musi
case #{ name } => expr;
```

means:

```musi
case #{ name := name } => expr;
```

### Alias Patterns

`as` is a contextual pattern keyword for alias patterns. It is not cast syntax.

```ebnf
alias-pattern ::= pattern-primary type-annot? ("as" identifier-pattern)?
```

The alias binds the whole value matched by the pattern.

```musi
case .Some(x) as option => option;
case #{ name := n } as person => n;
case id : UserId as rawPattern => id.raw;
```

In pattern alternatives, aliases must be binding-compatible across alternatives if the shared body uses them.

### Underscore Names

`_` is the wildcard pattern. It matches and binds nothing.

Identifiers beginning with underscore are ordinary identifiers. `_name` is not special syntax for silencing unused bindings. Normal unused-binding rules apply.

### Rest Patterns

```ebnf
rest-pattern        ::= ".." identifier-pattern?
array-rest-pattern  ::= ".." identifier-pattern?
record-rest-pattern ::= ".." identifier-pattern?
```

Rules:
- at most one rest pattern may appear in a tuple, record, or array pattern
- array rest may ignore or bind remaining elements
- record rest may ignore or bind remaining fields
- tuple rest requires tuple rest/variadic tuple semantics; until locked, tuple rest is not accepted

```musi
case #[head, ..tail] => tail;
case #[head, ..] => head;
case #{ name := n, ..rest } => rest;
case #{ name := n, .. } => n;
```

## 9. Data And Datum Syntax

### Data

`data` is the single data-definition form. The body determines whether data is product-shaped or sum-shaped. A `data` body must not mix product `let` entries and sum `case` entries.

```ebnf
data-expr              ::= attr-list? "data" data-body
data-body              ::= product-data-body | sum-data-body | empty-data-body
product-data-body      ::= "{" data-field (";" data-field)* ";"? "}"
sum-data-body          ::= "{" data-case (";" data-case)* ";"? "}"
empty-data-body        ::= "{" "}"
data-field             ::= "let" IDENT type-annot field-default?
                         | "let" IDENT ":=" EXPR
field-default          ::= ":=" EXPR
data-case              ::= "case" IDENT variant-payload? case-tag?
variant-payload        ::= "(" variant-param-list? ")"
variant-param-list     ::= required-variant-param ("," required-variant-param)* ("," default-variant-param)* ","?
                         | default-variant-param ("," default-variant-param)* ","?
required-variant-param ::= IDENT type-annot | type-annot | TYPE
default-variant-param  ::= IDENT type-annot? ":=" EXPR
case-tag               ::= ":=" known-expr
known-expr             ::= EXPR /* context requires known value */
```

Rules:
- `:= value` on the `case` initializes/defines variant identity.
- tag/discriminant value must be `known`.
- tags must be unique within the sum.
- omitted tags are assigned by compiler in declaration order.
- payload defaults stay inside payload parameters.
- product and sum data stay separate.
- if both product and sum are needed, pass one as a field of the other.
- a `data` body may bind data-valued fields or associated data through `let`.
- receiver methods are defined outside `data` or `shape` body with `let (self : Parent).method() := expr`.

There is no separate `struct`, `enum`, `union`, `class`, or `impl` form.

### Datum Literals

Datum literals use `#` plus delimiter as a compound lexical category, separating value literals from type syntax and computation delimiters.

```ebnf
datum-literal      ::= tuple-datum | record-datum | array-datum
tuple-datum        ::= "#(" (EXPR ("," EXPR)* ","?)? ")"
record-datum       ::= "#{" (record-datum-field ("," record-datum-field)* ","?)? "}"
array-datum        ::= "#[" (EXPR ("," EXPR)* ","?)? "]"
record-datum-field ::= IDENT ":=" EXPR
```

Meanings:
- `#()` is empty tuple datum and canonicalizes to `Unit`.
- `#{}` is empty record datum.
- `#[]` is empty array/list datum and requires type context.
- plain `{ ... }` never means value record literal.
- plain `( ... )` never means tuple datum literal unless introduced by `#`.

### Type Delimiters And Indexing

```ebnf
tuple-type          ::= "(" (TYPE ("," TYPE)* ","?)? ")"
array-list-type     ::= "[" array-bound? "]" TYPE
array-bound         ::= EXPR | EXPR ".." EXPR | EXPR "..<" EXPR
generic-application ::= TYPE "[" (TYPE ("," TYPE)* ","?)? "]"
tuple-field-access  ::= EXPR "." INT
array-index-access  ::= EXPR ".[" EXPR "]"
```

Array/list types are prefixed on the element type:

```musi
[]T
[N]T
[A .. B]T
[A ..< B]T
```

Meanings:
- `[]T` is a dynamic/unbounded sequence of `T`.
- `[N]T` is an exact known length `N` sequence.
- `[A .. B]T` is an inclusive known length range.
- `[A ..< B]T` is a half-open known length range.
- bounds must be known `Nat` values.
- generic/type application uses postfix brackets: `T[A, B]`.
- tuple fields index by numeric field access: `pair.0`.
- array/list values index by compound `.[` access: `list.[0]`.

### Product And Sum Construction

```ebnf
product-construction ::= TYPE record-datum
inferred-product     ::= record-datum
sum-construction     ::= unqualified-variant | qualified-variant
unqualified-variant  ::= "." IDENT variant-args?
qualified-variant    ::= TYPE "." IDENT variant-args?
variant-args         ::= "(" (EXPR ("," EXPR)* ","?)? ")"
```

Rules:
- product data construction uses named or unnamed record datum literals.
- product data is not constructed with function-call syntax.
- sum construction selects a variant through dot variant syntax.

```musi
let ada : opaque Named := Person#{ name := "Ada" };
let ada : opaque Named := #{ name := "Ada" };
let optionalType := .Some(Type);
let optionalType := Maybe.Some(Type);
```

## 10. Operators

### Fixed Operator Vocabulary

Musi core has no user-defined symbolic operators. Only locked operator tokens have operator syntax. Domain-specific operations use named functions or methods.

Fixed core operator tokens:

```text
.
?.
.[
?.[
#(
#{
#[
:
:=
:?
:>
:?>
<:
~=
|=
=
/=
<
<=
>
>=
in
+
-
*
/
%
|<
>|
>+
@<
@>
&
^
|
~
??
..
..<
=>
->
```

### Word Operators

`in` is the only core word operator.

```ebnf
word-op ::= "in"
```

Rules:
- `in` is contextual in operator position.
- no `and`, `or`, `xor`, `not`, `is`, `lsh`, `rsh`, or similar word operators exist in core.
- negated membership uses `~(x in y)`.

### Equality, Ordering, Equivalence, Membership

```ebnf
equality-op    ::= "=" | "/="
ordering-op    ::= "<" | "<=" | ">" | ">="
equivalence-op ::= "~="
membership-op  ::= "in"
```

Meanings:
- `=` value equality
- `/=` value inequality
- `<`, `<=`, `>`, `>=` ordering comparisons
- `~=` type/equivalence relation, not approximate numeric equality
- `in` membership

There is no approximate-equality operator in core. Approximation depends on tolerance, units, absolute vs relative error, domain, and numeric type, so it belongs in named functions or methods.

### Binding And Update

`:=` is binding/definition/initialization/update. `=` is equality only and never assignment.

```ebnf
binding-expr ::= "let" bind-head type-annot? ":=" EXPR
update-expr  ::= place-expr ":=" EXPR
place-expr   ::= IDENT | EXPR "." IDENT | EXPR "." INT | EXPR ".[" EXPR "]"
```

Rules:
- `let name := expr` creates or binds.
- `place := expr` updates an existing place.
- record/product datum fields use `:=` because they initialize named fields.
- update requires mutable access or equivalent capability.
- `:=` has the lowest precedence.
- chained updates are not accepted unless a later rule explicitly defines them.

### Algebra Operators

Core Boolean/bit algebra operators:

```ebnf
algebra-op ::= "&" | "|" | "^" | "~"
```

Meanings:
- `&` conjunction / bitwise-and / type-phase intersection where type checking proves it
- `|` disjunction / bitwise-or / type-phase union where type checking proves it
- `^` xor / symmetric difference where type checking proves it
- `~` complement / not where type checking proves it

There is no separate logical/bitwise operator split. `Bit`, `Word`, `Word8`, `Word16`, `Word32`, `Word64`, and `Bits[N]` use the same symbolic algebra where accepted by type checking.

Guard contexts require `Bit`. There is no truthiness.

Short-circuiting is control flow, not algebra. Use `when ... else ...` or `match`.

Not core Boolean/bit algebra syntax:

```text
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

### Parser Strategy, Precedence, And Associativity

Musi does not parse all infix expressions as one flat semantic chain. Locked core operators parse with the locked precedence table.

```ebnf
postfix-expr      ::= EXPR postfix-op+
postfix-op        ::= "." IDENT | "." INT | ".[" EXPR "]" | "?." IDENT | "?.[" EXPR "]" | generic-call-args | call-args
generic-call-args ::= "[" (EXPR ("," EXPR)* ","?)? "]"
prefix-expr       ::= prefix-op EXPR
prefix-op         ::= "known" | "fixed" | "mut" | "?" | "~" | "-"
multiplicative-op ::= "*" | "/" | "%"
additive-op       ::= "+" | "-"
shift-op          ::= "|<" | ">|" | ">+"
rotate-op         ::= "@<" | "@>"
range-op          ::= ".." | "..<"
relation-op       ::= "<" | "<=" | ">" | ">=" | "=" | "/=" | "~=" | ":?" | ":>" | ":?>" | "<:" | "|=" | "in"
algebra-and-op    ::= "&"
algebra-xor-op    ::= "^"
algebra-or-op     ::= "|"
nil-coalesce-op   ::= "??"
conditional-op    ::= "when"
binding-op        ::= ":="
```

Precedence, highest to lowest:
1. postfix access/call/index
2. prefix unary and modifiers
3. callable arrow in type position: `->`
4. multiplicative: `* / %`
5. additive: `+ -`
6. shift/rotate: `|< >| >+ @< @>`
7. range: `.. ..<`
8. relational/type/equality/membership: `< <= > >= = /= ~= :? :> :?> <: |= in`
9. algebra AND: `&`
10. algebra XOR: `^`
11. algebra OR: `|`
12. nil-coalesce / Maybe fallback: `??`
13. conditional: `when ... else` / `when`
14. binding/update: `:=`

Rules:
- `%` means remainder, not mathematical modulo.
- true modulo belongs in a named operation such as `mod(a, b)` or a standard-library/compiler intrinsic.
- shift and rotate operators are symbolic single tokens under maximal munch.
- no `<<` or `>>` shift syntax.
- no separate arithmetic-left-shift operator unless semantics distinct from zero-fill left shift are later defined.
- `&` binds tighter than `^`; `^` binds tighter than `|`.
- relational/type/equality operators are non-chainable.
- `??` is right-associative and Maybe-only.

Shift/rotate meanings:
- `a |< n` zero-fill left shift
- `a >| n` zero-fill right shift
- `a >+ n` sign-fill arithmetic right shift
- `a @< n` rotate left
- `a @> n` rotate right

## 11. Type System Surface

### Bidirectional Gradual Inference

Musi uses bidirectional type checking and inference.

Rules:
- missing annotations request inference, not fallback to dynamic typing
- annotations push expected types inward
- expressions synthesize types outward
- inference chooses the most precise/principal type supported by the type system
- ambiguity that the type system cannot resolve is a diagnostic
- inference never silently inserts `Any`
- dynamic boundaries must be explicit through annotation, conversion, import/FFI boundary, or API return type
- inferred structural, row, and capability constraints are ordinary Musi type information

Core lattice roles:
- `Type[N]` is the universe of types at level `N`
- `Type` is a predefined core `let` binding alias for `Type[0]`
- `Any` is the explicit dynamic top value type
- `_` in type position is an explicit inference hole
- unresolved `_` holes are diagnostics
- `Unit` is the canonical zero-information inhabited type
- `Empty` is the uninhabited bottom type

```ebnf
universe-type    ::= "Type" "[" EXPR "]"
universe-param   ::= "known" IDENT ":" "Nat"
type-alias       ::= "Type"
type-hole        ::= "_"
any-type         ::= "Any"
unit-type        ::= "Unit" | "()"
empty-type       ::= "Empty"
structural-type  ::= record-type | row-type | capability-type
```

`Type[N]` is the primitive universe use form. Its constructor parameter is `known N : Nat`, so the call site writes `Type[N]` and the compiler checks that `N` is known. If a runtime/non-known value is used in this known-required position, it is a diagnostic.

Known requirements are explicit at definition sites. Calls to known-required positions are implicit at use sites because the callee/type/form already declares the known requirement.

`Type[N]` has type `Type[N + 1]`. `Type` is not special syntax and not a keyword; it is the predefined core binding:

```musi
export let Type := Type[0];
```

`type` remains not a keyword. `Type0`, `Type1`, and similar names are not built-in source forms unless user or library code defines them.

`()` canonicalizes to `Unit`. `Empty` is not the empty tuple; it is a type with no values. An expression with type `Empty` may fit any required result position because it never produces a value.

`Any` does not mean null, absence, failure, callable-anything, or permission to ignore effects/capabilities. Dynamic-to-static use requires `:?>` or another explicit checked operation. Static-to-dynamic use is explicit through annotation, conversion, or a dynamic boundary.

Unannotated code may infer anonymous structural record/row types.

```musi
let p := #{ name := "Ada", age := 36 };
```

The inferred type is a structural record/row type with fields for `name` and `age`, not `Any`.

FFI and exported ABI boundaries require explicit representable types or explicit representation metadata. Anonymous inferred structural/row types do not silently become ABI types.

### Type Annotation

`:` is the universal type annotation marker.

```ebnf
type-annot          ::= ":" TYPE
annotated-name      ::= IDENT type-annot
annotated-result    ::= param-list type-annot
annotated-receiver  ::= "(" IDENT type-annot ")"
annotated-pattern   ::= PATTERN type-annot
```

It applies in value, parameter, field, result, receiver, pattern, and shape-member positions.

`:` is not overloaded for casts, subtyping, runtime type tests, type equivalence, or conformance.

### Callable Types

`->` is the callable type arrow.

```ebnf
callable-type        ::= callable-input "->" TYPE
callable-input       ::= TYPE | tuple-type
multi-input-callable ::= "(" TYPE ("," TYPE)+ ","? ")" "->" TYPE
```

Examples:

```musi
Int -> Text
(Int, Text) -> Unit
() -> Unit
```

Rules:
- `->` is type-space callable arrow, not a curry operator in expression space.
- `Unit` is the only canonical zero-information result type.
- `()` is empty tuple type shape and canonicalizes to `Unit`.
- chained callable arrows require explicit design before implicit currying is accepted.
- parentheses spell intent: `A -> (B -> C)` or `(A, B) -> C`.

Musi source and SEIL metadata use the same callable type surface:

```ebnf
source-callable-type ::= callable-type
seil-callable-type   ::= callable-type
```

Musi source does not use old stack-effect bracket syntax as the callable surface. SEIL is the lowered verifier form of Musi, not a separate source language. SEIL metadata preserves callable types in Musi syntax for near-identical decompilation.

### Type Algebra

Type-position algebra uses the same symbolic algebra family as value/bit algebra when the operands are types.

```ebnf
type-union        ::= TYPE "|" TYPE
type-intersection ::= TYPE "&" TYPE
type-difference   ::= TYPE "^" TYPE
type-complement   ::= "~" TYPE
```

Meanings:
- `A | B` is union: a value is in `A` or `B`
- `A & B` is intersection: a value is in both `A` and `B`
- `A ^ B` is symmetric difference: a value is in either `A` or `B`, but not both
- `~A` is complement: a value is outside `A` within the relevant type universe

`A ^ B` is derived type algebra:

```text
A ^ B = (A | B) & ~(A & B)
```

Normalization laws:
- `A | A` normalizes to `A`
- `A & A` normalizes to `A`
- `A | Empty` normalizes to `A`
- `A & Empty` normalizes to `Empty`
- `A | Any` normalizes to `Any`
- `A & Any` normalizes to `A`
- `~~A` normalizes to `A`
- `A ^ A` normalizes to `Empty`
- `A ^ Empty` normalizes to `A`
- `A ^ Any` normalizes to `~A`

Subtyping/equivalence facts:
- `A` is a subtype of `B` exactly when `A | B` is equivalent to `B`
- `A & B` is a subtype of `A`
- `A & B` is a subtype of `B`

Complement and symmetric difference are accepted only where the type universe makes them normalizable/checkable. Ambiguous or non-normalizable type algebra is a diagnostic.

Type algebra is Musi type space. ABI/FFI representability is checked separately. Algebraic types do not silently become ABI-safe tagged unions or layout-compatible records.

No `iff`, `<=>`, `<->`, `=>`, or `==` operator is introduced for type logic. Type equivalence remains `~=`. Subtyping remains `<:`.

### Type Operator Family

Musi uses a coherent `:`-led family for type-related operators.

```ebnf
type-test            ::= EXPR ":?" TYPE
static-cast          ::= EXPR ":>" TYPE
checked-cast         ::= EXPR ":?>" TYPE
subtype-relation     ::= TYPE "<:" TYPE
type-equivalence     ::= TYPE "~=" TYPE
conformance-relation ::= TYPE "|=" TYPE
```

Meanings:
- `:` annotates
- `:?` tests runtime type and returns `Bit`
- `:>` requests explicit static conversion/cast
- `:?>` performs checked runtime cast and returns explicit failure-capable result
- `<:` states subtype relation
- `~=` states type equivalence relation
- `|=` states shape conformance/fits relation

Rules:
- `:?` never returns narrowed value.
- `:?>` never returns `Bit`.
- `:>` is not runtime checked.
- `?=` is not accepted and does not belong to the `:` type-operator family.

### Optional Type And Operators

`?T` is optional type sugar for `Maybe[T]`.

```ebnf
optional-type   ::= "?" TYPE
maybe-fallback  ::= EXPR "??" EXPR
optional-access ::= EXPR "?." IDENT
                  | EXPR "?." IDENT call-args
                  | EXPR "?.[" EXPR "]"
call-args       ::= "(" (EXPR ("," EXPR)* ","?)? ")"
```

Rules:
- `?` in type position names optionality/maybe-ness.
- `?` does not name `Expect`.
- `??` works only on `?T` / `Maybe[T]`.
- `??` fallback produces `T`.
- `??` result type is `T`.
- `??` fallback is lazy.
- `?.` operates only on `?T` / `Maybe[T]`.
- absent stays absent.
- `?.` does not invent null.
- `?.` composes with `??`.

Distinctions:
- `when ... else ...` branches on `Bit`.
- `??` branches on optional presence.
- `?.` propagates absence through access.
- `Expect` remains explicit unless separate failure sugar is locked later.

### Expect And Checked Casts

```ebnf
expect-type         ::= "Expect" "[" TYPE "," TYPE "]"
checked-cast-result ::= "Expect" "[" TYPE "," "CastError" "]"
```

`:?>` returns an explicit `Expect` value:

```musi
let checked : Expect[User, CastError] := value :?> User;
```

Rules:
- no locked error/failure sugar for `Expect`
- `?T`, `??`, and `?.` are Maybe-only
- failed casts carry error information
- no hidden exceptions are introduced

### Fixed Storage

`fixed` is a type/storage-space modifier.

```ebnf
fixed-type     ::= "fixed" TYPE
fixed-mut-type ::= "fixed" "mut" TYPE
qualified-type ::= "known"? "fixed"? "mut"? TYPE
```

`fixed T` means storage-qualified `T` whose address is stable for the value's lifetime and cannot be moved by the collector/runtime during that lifetime.

`fixed` does not mean:
- static/global
- immutable
- compile-time
- type-associated
- permanent
- thread-safe by itself

`fixed` is orthogonal to `mut`:
- `fixed T`: stable address, not necessarily mutable
- `mut T`: mutable access, not necessarily stable-address storage
- `fixed mut T`: stable address and mutable access

Address-taking requires fixed storage. Movable values cannot expose stable raw addresses.

### Opaque And Erased Types

`opaque` and `erased` are type-space modifiers, not attributes.

```ebnf
opaque-type ::= "opaque" TYPE
erased-type ::= "erased" TYPE
```

They affect type identity, representation, dispatch, checking, ABI/SEIL metadata, and decompilation.

`hidden` is removed. Use exact concepts:
- `opaque` for existential type hiding
- `erased` for opaque-result/static-hidden concrete type
- `export` or non-export for module visibility
- metadata/attributes for representation, ABI, or interop details

`opaque T` is closest to Swift's `any T`: existential/capability value whose concrete type is hidden behind `T`.

`erased T` is closest to Swift's `some T`: exposed type hides the concrete type name while the defining expression still has one compiler-known concrete underlying type. Static specialization may remain possible.

## 12. Known Phase

`known` is a phase modifier. It answers: can this be compile-time?

`known` is not `const` and not `static`.

Rules:
- `known expr` requests or requires compile-time evaluation.
- `known T` requires a compile-time-known value/type-phase value of type `T`.
- `known` appears only where compile-time availability is meaningful.
- known requirements are explicit at definition sites by using `known` in type position.
- call sites to known-required parameters do not repeat `known`; the compiler checks that the supplied argument is known.
- if context already requires knownness, spelling is omitted at the call/use site.
- if a value cannot be compile-time, `known` produces a diagnostic.
- without `known`, evaluation is runtime unless context requires knownness.
- known phase can construct datum literals when contained values are known-compatible.
- case tag/discriminant positions require known values by context.
- array/list type bounds are known-phase contexts.
- `known import` is compile-time acquisition/import.

```musi
let Vector[N : known Nat, T : Type](items : [N]T) := ...;
let v := Vector[4, Int](items);
```

The definition states that `N` must be known. The call supplies `4`, not `known 4`. If a runtime value is supplied for `N`, it is a diagnostic.

Known/runtime boundary:
- known code may depend only on known values, known imports, type information, and compiler-permitted deterministic known intrinsics.
- runtime values cannot be captured by `known`.
- known values may generate or runtime-initialize representable values.
- crossing from known to runtime is allowed by embedding/lowering known results.
- crossing from runtime to known is not allowed.

Known functions are Musi code lowered to SEIL. Known evaluation executes SEIL in the known phase. There is no separate source-tree evaluator semantics.

Known evaluation is deterministic and resource-limited:
- no ambient runtime state
- no wall-clock, time, random, environment, process, or IO access unless supplied by explicit deterministic known imports/intrinsics
- bounded fuel, step, and memory limits are compiler settings
- nontermination or limit exhaustion is a diagnostic
- cannot depend on target runtime mutable state
- may use known imports, pure computation, type information, and compiler-approved deterministic intrinsics

## 13. Safety

There is no `unsafe` keyword or unsafe expression/block form.

```ebnf
unsafe-keyword ::= /* no production */
```

Unsafe-ness is represented by operation metadata, capabilities, types, and diagnostics rather than a lexical region.

Rationale:
- avoids lexical unsafe blocks that can hide too much
- unsafe is a property of operations, boundaries, and capabilities
- keeps keyword count down
- does not lock foreign/extern attribute syntax here

## 14. Attributes And Representation Metadata

Attributes are structural metadata prefixes. They attach to the next grammar-owned node and do not compute, branch, emit a value, or participate in runtime evaluation.

```ebnf
attr-list          ::= attr+
attr               ::= "@" attr-name attr-args?
attr-name          ::= IDENT ("." IDENT)*
attr-args          ::= "(" attr-arg-list? ")"
attr-arg-list      ::= attr-arg ("," attr-arg)* ","?
attr-arg           ::= IDENT ":=" attr-value | attr-value
attr-value         ::= literal | tuple-datum | record-datum | array-datum | variant-value | known-expr
attributed-let     ::= attr-list let-expr
attributed-data    ::= attr-list data-expr
attributed-shape   ::= attr-list shape-expr
attributed-case    ::= attr-list case-rule
attributed-match   ::= attr-list match-expr
attributed-while   ::= attr-list while-expr
attributed-defer   ::= attr-list defer-expr
attributed-import  ::= attr-list import-expr
attributed-export  ::= attr-list export-expr
attributed-lambda  ::= attr-list lambda-expr
attributed-region  ::= attr-list computation-region
packed-data-expr   ::= "@packed" "data" data-body
aligned-data-expr  ::= "@align" "(" attr-value ")" "data" data-body
witness-shape-expr ::= "@witness" "shape" shape-body
```

Confirmed surface attributes:
- `@packed`
- `@align(...)`
- `@witness`

Meanings:
- `@packed` marks packed/bit-structured representation metadata.
- `@align(...)` marks representation alignment metadata.
- `@witness` marks a `shape` as requiring explicit witness conformance.

Rules:
- attribute arguments are compile-time metadata values.
- positional and named arguments are accepted.
- named arguments use `:=`.
- datum literals and sum-type values are accepted attribute values.
- conditional attributes are not a separate grammar form.
- conditionality belongs in the attribute payload, usually `when := ...`.
- `when` payload must be `known Bit` when the schema defines it as a condition.
- if condition is true, metadata is present; if false, absent.
- the condition does not create runtime branching.
- attributes may prefix grammar-owned nodes.
- attributes do not prefix arbitrary infix expressions unless wrapped in a computation region.
- an attribute applies only to the exact next node.
- propagation to child nodes exists only when the attribute schema defines it.
- unknown compiler attributes are diagnostics unless declared by an imported/tooling mechanism.
- repeatability is defined by attribute schema.
- unique attribute repeated on same target is diagnostic.
- recognized attributes are preserved in SEIL metadata when they affect representation, ABI, checking, tooling, or near-identical decompilation.
- packed/bit-structured data is still `data`; there is no `bitstruct` keyword.

Rule:
- type identity/storage/checking concept: type-space modifier
- representation/ABI/interop annotation: attribute

## 15. Shapes And Conformance

`shape` is the locked spelling for structural contracts.

```ebnf
shape-expr   ::= "shape" shape-body
shape-body   ::= "{" shape-member (";" shape-member)* ";"? "}"
shape-member ::= "let" IDENT param-list? type-annot
```

`shape` means an observable structure/capability contract: a value or type fits a shape when it provides required members and operations according to Musi conformance rules.

`trait` is not accepted as core spelling.

`data` defines what a thing is. `shape` defines what a thing must look like.

Default `shape` conformance is structural. A type or value fits a structural shape when it provides required observable members and operations with compatible types and stack effects. No conformance declaration is required.

`@witness shape` defines a witness-required shape. Witness-required shapes are for semantic, lawful, marker, or capability contracts where members alone are insufficient. Empty marker shapes must use `@witness shape` to avoid every type fitting them accidentally.

`|=` is the locked conformance/fits relation operator.

```ebnf
conformance-relation ::= TYPE "|=" TYPE
witness-binding      ::= "let" TYPE "|=" TYPE ":=" record-datum
```

Roles:
- `T |= Shape` states or constrains that `T` fits `Shape`.
- `let T |= Shape := witnessValue;` binds an explicit witness for witness-required conformance.

There is no `impl`, `implements`, `extends`, or `trait` keyword. Receiver methods and witness bindings use universal `let`.

`|=` should not become a general-purpose ordinary Boolean test by default. Runtime fit checks for dynamic or opaque values remain open.

## 16. Modules, Imports, Exports, Visibility

`import` and `export` are hard keywords with ESM-like directionality. `import` takes in. `export` puts out.

```ebnf
import-expr   ::= "import" import-source
import-source ::= STRING | record-datum | tuple-datum
export-expr   ::= "export" let-expr | "export" export-block
export-block  ::= "{" export-item (";" export-item)* ";"? "}"
export-item   ::= let-expr
```

Rules:
- `import` is an expression that takes in a module, resource, or package according to module-system rules.
- `known import` is compile-time import/acquisition.
- import can use datum literals for multiple import inputs.
- `export` marks a `let` binding for the current module surface.
- exported receiver-method bindings are still `let` bindings.
- standalone `match`, `while`, or arbitrary expressions are not export targets.
- `export { ... }` is a structural export block, sugar over separate `export let ...;` forms.
- modules are top-to-bottom strict.
- export block items are processed top-to-bottom.
- module boundary forms affect source shape and SEIL/decompilation metadata.

```musi
export let makePoint() : Point := ...;
export let (self : Point).make() : Point := ...;
```

The first exports an ordinary callable binding. The second exports a receiver-method binding attached to `Point` through receiver-prefix syntax.

Modules are records. Imports bring in records.

```ebnf
module-value      ::= record-datum | named-record-value
named-import-bind ::= "let" IDENT ":=" import-expr
anonymous-import  ::= "let" "_" ":=" import-expr
```

Rules:
- a named import binds imported module record to a name.
- an anonymous import brings imported record contents into scope without binding the record itself.
- multi-import datum forms produce record-shaped imports.

Visibility:
- `export` is the only visibility mechanism.
- exported binding is visible from the module.
- non-exported binding is module-private.
- no `public`, `private`, `protected`, `internal`, or `hidden` visibility words exist.
- `opaque` controls type abstraction, not visibility.
- modules are records; exports define the module record surface.

SEIL round-trip metadata must preserve:
- import binding mode: named or anonymous
- import source shape: string, tuple datum, or record datum
- known/runtime phase of import
- exported binding names
- optional export block grouping as source metadata for near-identical decompilation

If grouping metadata is absent, the decompiler may emit canonical separate `export let` forms while preserving semantics.

## 17. Open Question Checklist

### Keyword Set

- [x] Final hard-reserved keyword list
- [x] Whether visibility words are hard keywords or contextual introducers
- [x] Whether `import` is a keyword or a compiler-owned function/form with special lowering
- [x] Whether `export` is a keyword, metadata, or a structural member rule
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

- [x] Bidirectional gradual type-system model
- [x] Type-phase algebra for `|`, `&`, `^`, and `~`
- [x] Union/intersection representation and normalization rules
- [x] Optional/error type surface forms
- [x] Whether callable types use stack-effect syntax directly
- [x] Whether type annotations use `:` in every context
- [x] Whether casts/tests use symbolic operators such as `:>` and `:?>`

### Stack Effect

- [x] Exact source syntax for stack effects
- [x] Whether stack effects are first-class type values
- [x] Whether ordinary functions expose stack-effect types or parameter/result sugar
- [ ] Stack-effect compatibility for `when`, `match`, `defer`, `yield`, and receiver methods
- [x] Whether guarded emission requires a special effect kind or row-polymorphic stack effect

### Data

- [x] Product field grammar inside `data`
- [x] Sum variant grammar inside `data`
- [x] Exact meaning of `case Variant(...) := value`
- [x] Whether product `let` entries and sum `case` entries can ever mix
- [x] Associated data/value binding rules inside `data`
- [x] Constructor generation rules
- [x] Destructuring and pattern syntax for product data
- [x] Variant tag/discriminant rules

### Representation And Metadata

- [x] Attribute syntax
- [x] Whether `@packed` is the final packed-data spelling
- [ ] Representation controls such as alignment, endian, tags, padding, and ABI layout
- [x] Whether representation metadata appears before `data`, after `data`, or inside the structural body
- [x] Whether metadata is preserved in SEIL for decompilation

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
- [x] Trailing separator rules for `,` and `;`
- [x] Empty tuple, empty record, and empty array syntax

### Control Flow

- [x] Exact precedence and associativity of `when ... else ...`
- [x] Dangling-else prevention rule
- [x] Whether `when` condition may contain unparenthesized `when`
- [x] Whether guarded emission is allowed in specific structural contexts
- [x] Whether loops exist as syntax or are expressed through recursion/recur forms
- [x] Whether `defer`, `yield`, and `pin` earn hard keyword status

### Match And Patterns

- [x] Exact pattern grammar
- [x] Whether pattern alternatives exist
- [x] Whether pattern alternatives use `|`, repeated `case`, or another form
- [x] Whether match cases require semicolons in all positions
- [x] Exhaustiveness rules
- [x] Guard evaluation order
- [x] Pattern binding syntax

### Operators

- [x] Full symbolic operator set
- [x] Operator precedence table or precedence-avoidance strategy
- [x] Whether all infix expressions parse flat and precedence is semantic
- [x] Whether user-defined symbolic operators exist
- [x] Whether word operators exist at all
- [x] Whether assignment/binding/update operators are distinct from equality
- [x] Equality, equivalence, ordering, approximation, membership, and remainder operators

### Modules And Imports

- [x] Whether modules are ordinary record-like values
- [x] Import expression syntax
- [x] Export surface syntax
- [x] Visibility rules
- [x] Whether package/module paths are strings, symbols, datums, or dedicated syntax
- [x] How imports/exports round-trip through SEIL

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
- [x] Known-phase evaluation limits
- [x] Known/runtime boundary rules
- [x] Whether known values can construct `#` datum literals
- [x] Whether known functions compile to SEIL or evaluate through a separate interpreter

### Safety

- [x] Exact meaning of `unsafe`
- [x] Whether unsafe is an expression wrapper, attribute, capability, or all of these
- [ ] Pointer types and pointer operations
- [ ] Pinning syntax and semantics
- [ ] Foreign boundary rules
- [ ] Whether dangerous behavior can ever be a warning instead of an error
