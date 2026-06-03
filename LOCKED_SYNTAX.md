# LOCKED_SYNTAX.md

## Core Identity

Musi is a small systems language with a small core.

Musi is expression-first:
- statements are not a separate semantic category
- a top-level expression terminated by `;` is an accepted top-level item
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

If a form needs more than one token of lookahead, the design is not accepted and must be redesigned.

## EBNF Notation

The grammar snippets in this document use the W3C XML 1.0 EBNF notation for locked surface shapes.

```ebnf
[1]  A       ::= B C
[2]  A       ::= B | C
[3]  A       ::= B?
[4]  A       ::= B*
[5]  A       ::= B+
[6]  A       ::= "token"
[7]  A       ::= B /* comment */
[8]  IDENT   ::= /* lexical identifier token */
[9]  EXPR    ::= /* expression production defined by final grammar */
[10] TYPE    ::= /* type-expression production defined by final grammar */
[11] PATTERN ::= /* pattern production defined by final grammar */
[12] ATTR    ::= /* attribute production defined by final grammar */
```

These productions define accepted shapes. Omitted forms are not accepted by this locked syntax unless another locked section adds them. The snippets are documentation grammar, not the generated parser grammar.

## Keyword Rule

A keyword is a hard-reserved source word required to introduce or disambiguate a grammar form.

A word is not a keyword merely because it is built in, compiler-owned, common, or standard-library-provided.

Operators, compiler intrinsics, methods, traits/shapes, sum types, product types, and built-in types are not keywords unless they are hard-reserved grammar introducers.


## Hard Keyword Set

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

`in` is a contextual word operator, not a form keyword. `as` is a contextual pattern keyword, not a cast keyword.

`await`, `spawn`, and `task` are not hard keywords. They remain available as ordinary identifiers, method names, shape names, or data names.

`import` and `export` are hard keywords. `import` takes in. `export` puts out. Module boundary forms affect source shape and SEIL/decompilation metadata. `known import` is compile-time acquisition/import.

Not core keywords:

```text
unsafe
pin
yield
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

## Comments

Comment spellings are locked.

```ebnf
[13] line-comment       ::= "--" line-comment-text
[14] line-doc-comment   ::= "---" line-comment-text
[15] line-module-doc    ::= "--!" line-comment-text
[16] block-comment      ::= "/-" block-comment-body "-/"
[17] block-doc-comment  ::= "/--" block-comment-body "-/"
[18] block-module-doc   ::= "/-!" block-comment-body "-/"
[19] block-comment-body ::= (block-comment | block-doc-comment | block-module-doc | block-comment-char)*
```

The longer opener wins by maximal munch:
- `--!` is a module doc comment, not a line comment followed by `!`
- `---` is a doc comment, not a line comment followed by `-`
- `/--` is a block doc comment, not a block comment followed by `-`
- `/-!` is a block module doc comment, not a block comment followed by `!`

Block comments, block documentation comments, and block module documentation comments participate in the same nesting system.

```musi
/-
outer
  /- inner -/
outer continues
-/
```

Rules:
- line comments inside block comments are comment text
- nested block comments are implemented with a linear depth counter
- unterminated nested block comments are diagnostic errors
- module docs are supported separately from item docs

## Universal Binding

`let` is the universal binding form.

It binds values, functions, data definitions, shape definitions, module/import results, compile-time values, runtime values, and attached receiver methods.

```ebnf
[20] let-expr        ::= "let" bind-head type-annot? param-list? result-type? ":=" EXPR
                       | "let" receiver-head "." IDENT param-list result-type? ":=" EXPR
[21] bind-head       ::= IDENT | "_" | operator-name | PATTERN
[22] receiver-head   ::= "(" IDENT type-annot ")"
[23] param-list      ::= "(" param-list-body? ")"
[24] param-list-body ::= required-param ("," required-param)* ("," default-param)* ","?
                       | default-param ("," default-param)* ","?
[25] required-param  ::= IDENT type-annot
                       | IDENT type-annot?
[26] default-param   ::= IDENT type-annot? ":=" EXPR
[27] result-type     ::= type-annot
[28] type-annot      ::= ":" TYPE
```

There is no separate `fn`, `type`, `struct`, `enum`, `class`, `impl`, `const`, or `static` keyword.

Defaults must be trailing in every parameter list, including function parameters, method parameters, constructor-like parameters, and variant payload parameters.

## Binding Qualifiers

Binding syntax is plain. `known`, `fixed`, and `mut` do not appear before `let` and do not appear between `let` and the binding name.

```ebnf
[29] let-binding       ::= "let" bind-head type-annot? ":=" EXPR
[30] qualified-binding ::= "let" bind-head ":" qualified-type ":=" EXPR
[31] qualified-rhs-bind ::= "let" bind-head ":=" qualified-expr
[32] qualified-type    ::= known-mod? fixed-mod? mut-mod? TYPE
[33] known-mod         ::= "known"
[34] fixed-mod         ::= "fixed"
[35] mut-mod           ::= "mut"
[36] qualified-expr    ::= "known" EXPR | EXPR
```

Canonical type qualifier order is:

```ebnf
[37] qualified-type ::= "known"? "fixed"? "mut"? TYPE
```

Other orders are absent from the accepted grammar and may be canonicalized by diagnostics/formatting according to the final parser and formatter design.

If a binding has no annotation, inference preserves the qualified type of the right-hand side. It does not invent qualifiers and does not strip qualifiers.

```musi
let Natural /- : Nat -/ := 0;
let Natural /- : known Nat -/ := known 0;
```

No modifier means no qualifier unless the expression already has that qualifier.

## Expression Sequencing

Parentheses delimit computation regions.

```ebnf
[38] computation-region ::= "(" computation-body? ")"
[39] computation-body   ::= EXPR (";" EXPR)* ";"?
```

Semicolon inside a computation region is sequencing/discard.

```musi
(
  step1();
  step2()
)
```

produces the value/effect of `step2()`.

```musi
(
  step1();
  step2();
)
```

discards `step2()` and produces `Unit` or the corresponding empty stack effect according to the final stack-effect rules.

Leading semicolon in a computation region is not part of the grammar because it would imply an empty computation step.

## Structural Regions

Curly braces delimit structural regions.

Structural regions define members, fields, variants, cases, or rule tables. They are not sequential computation bodies.

```ebnf
[40] structural-region ::= "{" structural-body? "}"
[41] structural-body   ::= structural-member (";" structural-member)* ";"?
[42] structural-member ::= data-field | data-case | shape-member | match-case
```

Structural semicolon is a member/rule terminator, not a discard operator.

## Trailing Separators

Trailing separators are allowed where the separator follows an item. Leading separators are not allowed.

```ebnf
[43] comma-items     ::= EXPR ("," EXPR)* ","?
[44] semicolon-items ::= structural-member (";" structural-member)* ";"?
```

No production begins with `,` or `;` for these list shapes.

Comma-list positions use the same `X ("," X)* ","?` shape in datum, argument, parameter, generic, and array/list type parameter positions, with `X` replaced by the production used by that position.

Structural regions use the same `X (";" X)* ";"?` shape for members and rules, with `X` replaced by the member or rule production used by that region.

Computation regions use `;` as sequencing/discard, not as a generic list separator.


## Repetition And Loop Keywords

`while` is the only source loop form.

```ebnf
[143] while-expr ::= "while" EXPR computation-region
[144] loop-control ::= "leave" | "cycle"
[145] recur-keyword ::= /* no production */
[146] for-keyword ::= /* no production */
```

`while` is a zero-or-more conditional repetition expression. The condition must be `Bit`. The body is a computation region. `while` produces `Unit`.

```musi
while keepGoing (
  cycle when shouldSkip;
  leave when done;
  step();
  update();
)
```

Loop control uses `leave` and `cycle`. `leave` exits the nearest enclosing `while`. `cycle` skips the remaining body and proceeds to the next condition check.

There is no `for`, `break`, `continue`, `next`, or `recur` keyword in core.

Iterable loops are expressed through ordinary functions, methods, and shapes over iterable/container abstractions. Postcondition repetition can be expressed by sequencing an initial body with a `while` loop or by named library helpers.

`recur` does not earn a keyword slot because it would create a one-off postfix binding modifier such as `let recur N := ...`, violating the locked binding qualifier rule and duplicating ordinary recursion.

`pin` is not a core keyword. Stable-address semantics are handled by `fixed`; scoped temporary non-moving access must be justified against `fixed` rather than added by default.




## Known Phase Rules

`known` is a phase modifier. It answers the question: can this be compile-time?

`known` is not `const` and is not `static`.


Rules:
- `known expr` requests or requires compile-time evaluation of `expr`
- `known T` requires a compile-time-known value/type-phase value of type `T`
- `known` appears only where compile-time availability is meaningful
- if a context already requires knownness, the spelling may be omitted
- if a value cannot be compile-time, `known` produces a diagnostic
- without `known`, evaluation is runtime unless context requires knownness

Known phase can construct datum literals when contained values are known-compatible.

```musi
let point := known #(1, 2);
let config := known #{ retries := 3, timeout := 30 };
let table := known #[1, 2, 3];
```

Case tag/discriminant positions require known values by context.

```musi
let TokenKind := data {
  case Eof := 0;
  case Ident(text : Text) := 1;
};
```

`known import` is compile-time acquisition/import.


Known/runtime boundary is strict.

Rules:
- known code may depend only on known values, known imports, type information, and compiler-permitted known intrinsics
- runtime values cannot be captured by `known`
- known values may be used to generate or runtime-initialize values if representable
- crossing from known to runtime is allowed by embedding/lowering the known result
- crossing from runtime to known is not allowed

Array/list type bounds are known-phase contexts.

```musi
let n := known 4;
let xs : [n]Word8 := #[0, 0, 0, 0];
```


Known functions are Musi code lowered to SEIL. Known evaluation executes SEIL in the known phase.

There is no separate source-tree evaluator semantics for known functions.

Rationale:
- one semantics
- no source AST evaluator drift
- direct fit with Musi targeting SEIL
- known and runtime phases share verifier/lowering rules
- compiler/tooling can execute SEIL for known evaluation without requiring the compiler at runtime


Known evaluation is deterministic and resource-limited.

Rules:
- known evaluation has no ambient runtime state
- known evaluation has no wall-clock, time, random, environment, process, or IO access unless provided through explicit known imports/intrinsics that are deterministic by definition
- bounded fuel, step, and memory limits are implementation/compiler settings
- nontermination or limit exhaustion is a diagnostic
- known evaluation cannot depend on target runtime mutable state
- known evaluation may use known imports, pure computation, type information, and compiler-approved deterministic intrinsics

Rationale:
- avoids compiler hangs
- avoids spooky action at a distance
- keeps builds reproducible
- keeps known phase separate from runtime ambient state

## Unsafe

There is no `unsafe` keyword or unsafe expression/block form.

```ebnf
[149] unsafe-keyword ::= /* no production */
```

Unsafe-ness is represented by operation metadata, capabilities, types, and diagnostics rather than a magical lexical region.

Rationale:
- avoids lexical unsafe blocks that can hide too much
- unsafe is a property of operations, boundaries, and capabilities
- keeps keyword count down
- does not lock foreign/extern attribute syntax here

## Defer And Yield

`defer` is a core keyword/expression for deterministic cleanup.

```ebnf
[147] defer-expr ::= "defer" EXPR ("when" EXPR)?
[148] yield-expr ::= "yield" EXPR?
```

`defer` registers an expression to run when the current computation region/scope exits. It produces `Unit`.

`defer` cleanup runs on normal exit and on loop-control exits such as `leave` and `cycle` that leave or restart the region. Exact cleanup ordering remains part of the runtime/control-flow design.

Guarded cleanup uses existing `when` syntax.

```musi
defer file.close();
defer lock.release() when locked;
```

`yield` is a core keyword/expression for resumable/generator-compatible contexts. It is not an ordinary function call.

`yield expr` suspends or emits through the enclosing resumable protocol. Outside a resumable/generator-compatible context, `yield` is a diagnostic.

Rules:
- `yield` participates in stack/effect compatibility through the enclosing callable's result protocol
- the yielded value type must match the enclosing resumable/generator output type
- bare `yield` without an expression is accepted only when the yielded type is `Unit`
- `yield` produces `Unit` locally after handing the value to the resumable protocol
- suspension is not scope exit
- `defer` does not run at suspension points
- pending defers run on final scope exit, close, drop, or cancel according to the final resumable runtime rules

Concurrency is protocol/capability driven, not hard-coded syntax. `yield` is the only core suspension keyword. `Task`, `Scheduler`, `Resumable`, `Generator`, and `Stream` are library/runtime shapes or data types rather than keywords.

`await`, `spawn`, and `task` are ordinary names when used by libraries or runtimes.

## Conditional Expressions

`when` is the conditional guard operator.

```ebnf
[45] total-conditional ::= non-when-expr "when" non-when-expr "else" EXPR
[46] guarded-emission  ::= non-when-expr "when" non-when-expr
[198] non-when-expr    ::= /* expression production excluding unparenthesized when-expr */
```

Rules:
- the condition must be `Bit`
- `when` is postfix guard syntax, not prefix syntax
- total conditional branches must have compatible type/stack effect
- `else` provides the fallback branch explicitly
- no `then` keyword exists
- guarded emission has zero-or-one emission shape
- guarded emission is accepted only in contexts that can consume zero-or-one emission
- no hidden `Maybe`, `Unit`, bottom, or union is synthesized
- unparenthesized nested `when` is not accepted in the guarded value or condition position
- parentheses are required for nested conditional expressions

`VALUE when CONDITION else FALLBACK` is total value selection. `VALUE when CONDITION` is guarded zero-or-one emission.

```musi
value when ready else fallback
value when ready
```

The bare form does not produce `Maybe[T]`, `Unit`, bottom, or an implicit union. It produces a verifier-visible guarded emission shape: zero-or-one `T`. Ordinary total value positions require `else`; contexts that accept optional emission may consume the bare form.

Nested conditionals are grouped explicitly.

```musi
value when ready else (other when available else fallback)
(value when ready else other) when enabled else fallback
```

## Match And Case

Pattern matching uses `match`. Each match arm starts with `case` and ends with semicolon. `=>` is the body/result arrow for both match arms and lambdas.

```ebnf
[47] match-expr  ::= "match" EXPR "{" match-case+ "}"
[48] match-case  ::= "case" case-pattern-list case-guard? "=>" EXPR ";"
[49] case-pattern-list ::= PATTERN ("," PATTERN)* ","?
[50] case-guard  ::= "when" EXPR
[51] lambda-expr ::= '\' param-list type-annot? "=>" EXPR
```

The semicolon after a `case` arm terminates the structural case rule. It does not discard the selected arm value and does not make the match produce `Unit`.

```musi
match value {
  case .A => 1;
  case .B => 2;
}
```

produces `Int`.

To discard inside an arm, use a computation region whose own final semicolon performs the discard.

```musi
match value {
  case .A => (log("A"); 1);
  case .B => (log("B"); 2;);
}
```

Pattern alternatives use comma separation inside a `case`. `|` is not used for pattern alternatives. All alternatives in one `case` share the same guard and body. Pattern bindings must be compatible across alternatives: the same names must be bound with compatible types in every alternative that reaches the shared body.


`as` is a contextual pattern keyword for alias patterns.

```ebnf
[171] alias-pattern ::= pattern-primary type-annot? ("as" identifier-pattern)?
```

The alias binds the whole value matched by the pattern. `as` is not cast syntax; casts/conversions use `:>` and `:?>`.

```musi
case .Some(x) as option => option;
case #{ name := n } as person => n;
case id : UserId as rawPattern => id.raw;
```

In pattern alternatives, aliases must be binding-compatible across alternatives if the shared body uses them.


```musi
match token {
  case .LParen, .RParen, .LBrace, .RBrace => "delimiter";
  case .Ident(text) => text;
}
```



## Match Exhaustiveness

`match` is exhaustive by default. Non-exhaustive `match` is a semantic error.

Exhaustiveness is checked by semantic analysis.

Rules:
- finite sum `data` matches must cover all variants or include wildcard catch-all
- guarded cases do not count as unconditional coverage
- catch-all is `case _`
- there is no `case else` syntax

```musi
match maybe {
  case .Some(x) when x > 0 => x;
  case .Some(_) => 0;
  case .None => fallback;
}
```

```musi
match maybe {
  case .Some(x) when x > 0 => x;
  case .Some(_) => 0;
  case _ => fallback;
}
```

`else` remains the fallback marker for `when ... else`; it is not a match pseudo-pattern.


## Match Guard Evaluation

Match cases are tested top-to-bottom. Within one `case`, comma-separated pattern alternatives are tested left-to-right.

Guard evaluation order:
- pattern alternative is tested first
- guard runs only after its pattern alternative matched
- guard can reference bindings from the matched pattern
- guard expression must be `Bit`
- if the pattern matches but guard is false, matching continues to the next alternative/case
- guards do not run for non-matching patterns
- first matching unguarded case or guard-true case wins
- guarded cases are conditional coverage for exhaustiveness

```musi
match maybe {
  case .Some(x) when x > 0 => x;
  case .Some(_) => 0;
  case _ => fallback;
}
```

## Pattern Grammar

Patterns mirror datum syntax where they destructure values. Let binding heads may be patterns, so ordinary binding identifiers are identifier patterns.

```ebnf
[159] pattern              ::= alias-pattern
[171] alias-pattern        ::= pattern-primary type-annot? ("as" identifier-pattern)?
[160] pattern-primary      ::= wildcard-pattern | identifier-pattern | literal-pattern | variant-pattern | tuple-pattern | record-pattern | array-pattern | rest-pattern
[161] wildcard-pattern     ::= "_"
[162] identifier-pattern   ::= IDENT
[163] literal-pattern      ::= INT | FLOAT | STRING | RUNE
[164] variant-pattern      ::= "." IDENT pattern-args? | TYPE "." IDENT pattern-args?
[165] pattern-args         ::= "(" (pattern ("," pattern)* ","?)? ")"
[166] tuple-pattern        ::= "#(" (pattern ("," pattern)* ","?)? ")"
[167] record-pattern       ::= "#{" (record-pattern-field ("," record-pattern-field)* ","?)? "}"
[168] record-pattern-field ::= IDENT (":=" pattern)?
[169] array-pattern        ::= "#[" (pattern ("," pattern)* ","?)? "]"
[170] rest-pattern         ::= ".." identifier-pattern?
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
```

Record pattern shorthand:

```musi
case #{ name } => expr;
```

means the same binding shape as:

```musi
case #{ name := name } => expr;
```

Let bindings can destructure through pattern heads.

```musi
let #(x, y) := point;
let #{ name := n, age := a } := person;
```

Rationale:
- tuple, record, and array patterns use `#(`, `#{`, and `#[` to mirror datum value syntax
- identifier patterns cover normal let-bound names
- `_` is wildcard
- `..` is rest/spread pattern
- `#` keeps destructuring patterns in value/datum space, not type/structure space


## Underscore Names

`_` is the wildcard pattern. It matches and binds nothing.

Identifiers beginning with underscore are ordinary identifiers.

```ebnf
[172] wildcard-pattern   ::= "_"
[173] identifier-pattern ::= IDENT
```

`_name` is not special syntax for silencing an unused binding. If `_name` is accepted by the identifier grammar, it is a normal name and normal unused-binding rules apply.

Unused bindings are unused regardless of spelling. No naming convention suppresses unused-binding checks.

```musi
case .Some(_) => 0;
case .Some(_value) => 0;
```

The first binds nothing. The second binds `_value`; if `_value` is not used, it is an unused binding.


## Rest Patterns

Rest patterns use `..`.

```ebnf
[174] rest-pattern       ::= ".." identifier-pattern?
[175] array-rest-pattern ::= ".." identifier-pattern?
[176] record-rest-pattern ::= ".." identifier-pattern?
```

Rules:
- at most one rest pattern may appear in a tuple, record, or array pattern
- array rest may ignore or bind the remaining elements
- record rest may ignore or bind the remaining fields
- tuple rest requires tuple rest/variadic tuple semantics; until those are locked, tuple rest is not accepted

```musi
case #[head, ..tail] => tail;
case #[head, ..] => head;
case #{ name := n, ..rest } => rest;
case #{ name := n, .. } => n;
```

## Data

`data` is the single data-definition form.

The body determines whether the data is product-shaped or sum-shaped. A `data` body must not mix product `let` entries and sum `case` entries.

```ebnf
[51] data-expr              ::= attr-list? "data" data-body
[52] data-body              ::= product-data-body | sum-data-body | empty-data-body
[53] product-data-body      ::= "{" data-field (";" data-field)* ";"? "}"
[54] sum-data-body          ::= "{" data-case (";" data-case)* ";"? "}"
[55] empty-data-body        ::= "{" "}"
[56] data-field             ::= "let" IDENT type-annot field-default?
                             | "let" IDENT ":=" EXPR
[57] field-default          ::= ":=" EXPR
[58] data-case              ::= "case" IDENT variant-payload? case-tag?
[59] variant-payload        ::= "(" variant-param-list? ")"
[60] variant-param-list     ::= required-variant-param ("," required-variant-param)* ("," default-variant-param)* ","?
                             | default-variant-param ("," default-variant-param)* ","?
[61] required-variant-param ::= IDENT type-annot | type-annot | TYPE
[62] default-variant-param  ::= IDENT type-annot? ":=" EXPR
[63] case-tag               ::= ":=" known-expr
[64] known-expr             ::= EXPR /* context requires known value */
```

`:= value` on the `case` itself initializes or defines the variant identity.

Rules:
- the tag/discriminant value must be `known`
- tags must be unique within the sum
- if omitted, tags are assigned by the compiler in declaration order
- payload defaults stay inside payload parameters

Product and sum data stay separate. If both are needed, pass one around as a field of the other. This follows the same useful shape as Rust's `enum TokenKind` plus `struct Token`, without adding separate `enum` or `struct` keywords.

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

A `data` body may bind data-valued fields or associated data through `let`.

```musi
let Packet := data {
  let header := Header;
  let Payload := data {
    case Text(message : Text);
    case Binary(bytes : Bytes);
  };
};
```

Receiver methods are defined outside the `data` or `shape` body.

```musi
let (self : Parent).method() := expr;
```

There is no separate `struct`, `enum`, `union`, `class`, or `impl` form.

## Datum Literal Grammar

Datum literals use `#` plus delimiter as a compound lexical category so value literals do not get confused with type syntax or computation delimiters.

```ebnf
[64] datum-literal      ::= tuple-datum | record-datum | array-datum
[65] tuple-datum        ::= "#(" (EXPR ("," EXPR)* ","?)? ")"
[66] record-datum       ::= "#{" (record-datum-field ("," record-datum-field)* ","?)? "}"
[67] array-datum        ::= "#[" (EXPR ("," EXPR)* ","?)? "]"
[68] record-datum-field ::= IDENT ":=" EXPR
```

Meanings:
- `#()` is the empty tuple datum and canonicalizes to `Unit`
- `#{}` is empty record datum
- `#[]` is empty array/list datum and requires type context

Plain `{ ... }` never means a value record literal. Plain `( ... )` never means a tuple datum literal unless introduced by `#`.

## Type Delimiters And Indexing

Datums exist to separate value construction from type syntax:
- record/product values use `#{ ... }`
- record/product types use `data { ... }` or named product data
- sum values use dot variant syntax
- sum types use `data { case ...; }` or named sum data
- tuple values use `#( ... )`
- tuple types use `( ... )` in type position
- array/list values use `#[ ... ]`
- array/list types use prefix bracket syntax
- `()` is the empty tuple type shape and canonicalizes to `Unit`

```ebnf
[69] tuple-type          ::= "(" (TYPE ("," TYPE)* ","?)? ")"
[70] array-list-type     ::= "[" array-bound? "]" TYPE
[71] array-bound         ::= EXPR | EXPR ".." EXPR | EXPR "..<" EXPR
[72] generic-application ::= TYPE "[" (TYPE ("," TYPE)* ","?)? "]"
[73] tuple-field-access  ::= EXPR "." INT
[74] array-index-access  ::= EXPR ".[" EXPR "]"
```

Array/list types are prefixed on the element type.

```musi
[]T
[N]T
[A .. B]T
[A ..< B]T
```

Meanings:
- `[]T` is a dynamic/unbounded sequence of `T`
- `[N]T` is an exact known length `N` sequence of `T`
- `[A .. B]T` is an inclusive known length range
- `[A ..< B]T` is a half-open known length range

Bounds must be known `Nat` values. Range bounds use normal range syntax; there is no separate `[A; B]T` bound syntax.

Generic/type application uses postfix brackets on the type constructor.

```musi
T[A, B]
```

Tuple fields index by numeric field access. Array/list values index by compound `.[` access.

```musi
let first := pair.0;
let item := list.[0];
```

## Product And Sum Construction

Product data construction uses named or unnamed record datum literals.

```ebnf
[74] product-construction ::= TYPE record-datum
[75] inferred-product     ::= record-datum
[76] sum-construction     ::= unqualified-variant | qualified-variant
[77] unqualified-variant  ::= "." IDENT variant-args?
[78] qualified-variant    ::= TYPE "." IDENT variant-args?
[79] variant-args         ::= "(" (EXPR ("," EXPR)* ","?)? ")"
```

Product data is not constructed with function-call syntax.

```musi
let ada : opaque Named := Person#{ name := "Ada" };
let ada : opaque Named := #{ name := "Ada" };
let optionalType := .Some(Type);
let optionalType := Maybe.Some(Type);
```

Rationale:
- product construction is datum construction, so it uses `#` datum syntax
- sum construction selects a variant, so it uses dot variant syntax
- dot variant syntax follows the same useful rationale as Swift and Zig while remaining part of Musi's own product/sum distinction





## Fixed Operator Vocabulary

Musi core has no user-defined symbolic operators.

Only locked operator tokens have operator syntax. Domain-specific operations use named functions or methods rather than new symbolic operators.

Rationale:
- user-defined symbolic operators create precedence, associativity, formatting, decompilation, and readability problems
- SEIL-to-Musi decompilation needs a stable operator vocabulary
- the core already has a broad fixed operator set
- named functions and methods preserve clarity without syntax expansion

Fixed core operator tokens include:

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

## Word Operators

`in` is the only core word operator.

```ebnf
[142] word-op ::= "in"
```

Rules:
- `in` is contextual in operator position
- no `and`, `or`, `xor`, `not`, `is`, `lsh`, `rsh`, or similar word operators exist in core
- negated membership uses `~(x in y)`

## Equality, Ordering, Equivalence, And Membership

Core relation operators are locked as follows.

```ebnf
[138] equality-op    ::= "=" | "/="
[139] ordering-op    ::= "<" | "<=" | ">" | ">="
[140] equivalence-op ::= "~="
[141] membership-op  ::= "in"
```

Meanings:
- `=` is value equality
- `/=` is value inequality
- `<`, `<=`, `>`, and `>=` are ordering comparisons
- `~=` is type/equivalence relation, not approximate numeric equality
- `in` is contextual word operator for membership

`in` is an operator word, not a form-introducing keyword. It is recognized in operator position.

```musi
let ok := item in collection;
let notOk := ~(item in collection);
```

There is no approximate-equality operator in core. Approximation depends on tolerance, units, absolute vs relative error, domain, and numeric type, so it belongs in named functions or methods.

## Binding And Update Operator

`:=` is the binding/definition/initialization/update operator. `=` is equality only and never assignment.

```ebnf
[135] binding-expr ::= "let" bind-head type-annot? ":=" EXPR
[136] update-expr  ::= place-expr ":=" EXPR
[137] place-expr   ::= IDENT | EXPR "." IDENT | EXPR "." INT | EXPR ".[" EXPR "]"
```

Meanings:
- `let name := expr` creates or binds
- `place := expr` updates an existing place
- record/product datum fields use `:=` because they initialize named fields

Update requires mutable access or an equivalent capability.

`:=` has the lowest precedence. Chained updates are not accepted unless a later rule explicitly defines them.

## Fixed Storage

`fixed` is a type/storage-space modifier.

```ebnf
[80] fixed-type     ::= "fixed" TYPE
[81] fixed-mut-type ::= "fixed" "mut" TYPE
[82] qualified-type ::= "known"? "fixed"? "mut"? TYPE
```

`fixed T` means storage-qualified `T` whose address is stable for the value's lifetime and cannot be moved by the collector/runtime during that lifetime.

`fixed` is the chosen spelling. `stable` is not accepted because it is too broad and can mean API stability, value immutability, deterministic behavior, ABI stability, numeric stability, or sorting stability. `fixed` names the required storage guarantee directly: the value is fixed in memory.

`fixed` does not mean:
- static/global
- immutable
- compile-time
- type-associated
- permanent
- thread-safe by itself

`fixed` is orthogonal to `mut`:
- `fixed T` has stable address and is not necessarily mutable
- `mut T` has mutable access and is not necessarily stable-address storage
- `fixed mut T` has stable address and mutable access

Address-taking requires fixed storage. Movable values cannot expose stable raw addresses.

`fixed` can make a separate `pin` keyword unnecessary. If scoped temporary non-moving access is needed later, it must be justified against `fixed` instead of added by default.

## Opaque And Erased Types

`opaque` and `erased` are type-space modifiers, not attributes.

```ebnf
[83] opaque-type ::= "opaque" TYPE
[84] erased-type ::= "erased" TYPE
```

They affect type identity, representation, dispatch, checking, ABI/SEIL metadata, and decompilation. They are not declaration decoration.

`hidden` is removed. It is too broad and does not identify a precise type-system operation. Use exact concepts instead:
- `opaque` for existential type hiding
- `erased` for opaque-result/static-hidden concrete type
- `export` or non-export for module visibility
- metadata/attributes for representation, ABI, or interop details

`opaque T` is closest to Swift's `any T`. It means an existential/capability value whose concrete type is hidden behind the `T` shape/type boundary. Operations may go through existential, witness, or capability representation.

`erased T` is closest to Swift's `some T`. It means the exposed type hides the concrete type name, while the defining expression still has one compiler-known concrete underlying type. Static specialization may remain possible.

## Attributes And Packed Data

Attributes are structural metadata prefixes. They attach to the next grammar-owned node and do not compute, branch, emit a value, or participate in runtime evaluation.

```ebnf
[177] attr-list             ::= attr+
[178] attr                  ::= "@" attr-name attr-args?
[179] attr-name             ::= IDENT ("." IDENT)*
[180] attr-args             ::= "(" attr-arg-list? ")"
[181] attr-arg-list         ::= attr-arg ("," attr-arg)* ","?
[182] attr-arg              ::= IDENT ":=" attr-value | attr-value
[183] attr-value            ::= literal
                             | tuple-datum
                             | record-datum
                             | array-datum
                             | variant-value
                             | known-expr
[184] attributed-let        ::= attr-list let-expr
[185] attributed-data       ::= attr-list data-expr
[186] attributed-shape      ::= attr-list shape-expr
[187] attributed-case       ::= attr-list case-rule
[188] attributed-match      ::= attr-list match-expr
[189] attributed-while      ::= attr-list while-expr
[190] attributed-defer      ::= attr-list defer-expr
[191] attributed-import     ::= attr-list import-expr
[192] attributed-export    ::= attr-list export-expr
[193] attributed-lambda    ::= attr-list lambda-expr
[194] attributed-region    ::= attr-list computation-region
[195] packed-data-expr     ::= "@packed" "data" data-body
[196] aligned-data-expr    ::= "@align" "(" attr-value ")" "data" data-body
[197] witness-shape-expr   ::= "@witness" "shape" shape-body
```

Confirmed surface attributes:
- `@packed`
- `@align(...)`
- `@witness`

Meanings:
- `@packed` marks packed/bit-structured representation metadata
- `@align(...)` marks representation alignment metadata
- `@witness` marks a `shape` as requiring explicit witness conformance

Attribute arguments are compile-time metadata values. Positional arguments and named arguments are both accepted. Named arguments use `:=`. Datum literals and sum-type values are accepted attribute values.

```musi
@align(4)
data { let value : Int; };

@packed(bits := 32, layout := .dense)
data { let flags : Word; };

@witness
shape { let (self : Self).show() : Text; };
```

Conditional attributes are not a separate grammar form. Conditionality belongs in the attribute payload as known-time metadata, usually through a named argument such as `when := ...`.

```musi
@packed(when := Target.hasPackedAbi)
data { let value : Word; };
```

The `when` payload value must be `known Bit` when the attribute schema defines it as a condition. If the condition is true, the metadata is present. If the condition is false, the metadata is absent. The condition does not create runtime branching.

Attributes may prefix grammar-owned nodes such as `let`, `data`, `shape`, `case`, `match`, `while`, `defer`, `import`, `export`, lambda expressions, and computation regions. Attributes do not prefix arbitrary infix expressions unless the expression is wrapped in a computation region.

```musi
@trace (
  a + b
)
```

An attribute applies only to the exact next node. Attribute propagation to child nodes exists only when that attribute's schema explicitly defines propagation.

Unknown compiler attributes are diagnostics unless they are declared by an imported/tooling mechanism defined later. Repeatability is defined by the attribute schema. A unique attribute repeated on the same target is a diagnostic.

Recognized attributes are preserved in SEIL metadata when they affect representation, ABI, checking, tooling, or near-identical decompilation.

Packed/bit-structured data is still `data`. It does not get a new keyword such as `bitstruct`.

Rule:
- type identity/storage/checking concept: type-space modifier
- representation/ABI/interop annotation: attribute

## Algebraic Operators

Core Boolean/bit algebra operators are:

```ebnf
[90] algebra-op ::= "&" | "|" | "^" | "~"
```

Meanings:
- `&` conjunction / bitwise-and / type-phase intersection where type checking proves it
- `|` disjunction / bitwise-or / type-phase union where type checking proves it
- `^` xor / symmetric difference where type checking proves it
- `~` complement / not where type checking proves it

There is no separate logical/bitwise operator split.

The following are not core Boolean/bit algebra syntax:

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

`Bit`, `Word`, `Word8`, `Word16`, `Word32`, `Word64`, and `Bits[N]` use the same symbolic algebra where accepted by type checking.

Guard contexts require `Bit`. There is no truthiness.

Short-circuiting is control flow, not algebra. Use `when ... else ...` or `match`.



## Expression Parser Strategy

Musi does not parse all infix expressions as one flat semantic chain.

Locked core operators parse with the locked precedence table. The precedence table is syntax, not a later semantic guess.

Rationale:
- precedence is part of long-term syntax
- fixed operator vocabulary makes parser tiers stable
- relation chains can be rejected early
- formatter and SEIL-to-Musi decompiler get stable expression structure
- one-token lookahead remains compatible with Pratt, precedence-climbing, or recursive-descent parser strategies

## Operator Precedence And Associativity

Musi uses mathematical/common algebra precedence where it does not create silent semantic traps. Parentheses are required where chaining or precedence would otherwise create misleading expressions.

```ebnf
[114] postfix-expr      ::= EXPR postfix-op+
[115] postfix-op        ::= "." IDENT | "." INT | ".[" EXPR "]" | "?." IDENT | "?.[" EXPR "]" | call-args
[116] prefix-expr       ::= prefix-op EXPR
[117] prefix-op         ::= "known" | "fixed" | "mut" | "?" | "~" | "-"
[118] multiplicative-op ::= "*" | "/" | "%"
[119] additive-op       ::= "+" | "-"
[120] shift-op          ::= "|<" | ">|" | ">+"
[121] rotate-op         ::= "@<" | "@>"
[122] range-op          ::= ".." | "..<"
[123] relation-op       ::= "<" | "<=" | ">" | ">=" | "=" | "/=" | "~=" | ":?" | ":>" | ":?>" | "<:" | "|=" | "in"
[124] algebra-and-op    ::= "&"
[125] algebra-xor-op    ::= "^"
[126] algebra-or-op     ::= "|"
[127] nil-coalesce-op   ::= "??"
[128] conditional-op    ::= "when"
[129] binding-op        ::= ":="
```

Precedence, highest to lowest:

```text
1. postfix access/call/index
2. prefix unary and modifiers
3. callable arrow in type position: ->
4. multiplicative: * / %
5. additive: + -
6. shift/rotate: |< >| >+ @< @>
7. range: .. ..<
8. relational/type/equality/membership: < <= > >= = /= ~= :? :> :?> <: |= in
9. algebra AND: &
10. algebra XOR: ^
11. algebra OR: |
12. nil-coalesce / maybe fallback: ??
13. conditional: when ... else / when
14. binding/update: :=
```

`%` means remainder, not mathematical modulo.

Rationale:
- CPUs define and compute remainder directly
- true modulo has different negative-number behavior and usually needs adjustment
- Musi is a small systems language, so `%` maps to the primitive machine operation
- this is explicit CPU semantics, not C-baggage inheritance

True modulo belongs in a named operation such as `mod(a, b)` or a standard-library/compiler intrinsic with specified semantics.


Shift and rotate operators are symbolic single tokens under maximal munch.

```ebnf
[130] shift-op  ::= "|<" | ">|" | ">+"
[131] rotate-op ::= "@<" | "@>"
```

Meanings:
- `a |< n` shifts left and fills with zero bits
- `a >| n` shifts right and fills with zero bits
- `a >+ n` shifts right and fills with sign bit / arithmetic right shift
- `a @< n` rotates left
- `a @> n` rotates right

There is no `<<` or `>>` shift syntax. Those forms carry mathematical “much less/greater” meaning and C/C++ baggage.

There is no separate arithmetic-left-shift operator unless Musi later defines semantics distinct from zero-fill left shift. Signed overflow policy belongs to type/overflow rules, not a separate left-shift operator.

Algebra precedence follows mathematical/common logic-gate convention:
- `&` binds tighter than `^`
- `^` binds tighter than `|`

The same algebra table applies to `Bit`, `Word`, `Bits[N]`, and type algebra where type checking accepts the operators.

Relational/type/equality operators are non-chainable.

```musi
(a < b) & (b < c)
```

spells a range-like comparison explicitly.

`??` is right-associative nil-coalesce / Maybe fallback.

```musi
a ?? b ?? c
```

means:

```musi
a ?? (b ?? c)
```

`??` remains Maybe-only and does not apply to `Expect`.

## Optional Type And Operators

`?T` is the optional type sugar for `Maybe[T]`.

```ebnf
[91] optional-type   ::= "?" TYPE
[92] maybe-fallback  ::= EXPR "??" EXPR
[93] optional-access ::= EXPR "?." IDENT
                       | EXPR "?." IDENT call-args
                       | EXPR "?.[" EXPR "]"
[94] call-args       ::= "(" (EXPR ("," EXPR)* ","?)? ")"
```

Rules:
- `?` in type position names optionality/maybe-ness
- `?` does not name `Expect`
- `??` works only on `?T` / `Maybe[T]`
- `??` fallback produces `T`
- `??` result type is `T`
- `??` fallback is lazy and is evaluated only when `value` is absent
- `?.` operates only on `?T` / `Maybe[T]`
- `?.` access, call, or index happens only when the value is present
- absent stays absent
- `?.` does not invent null
- `?.` composes with `??`

Distinctions:
- `when ... else ...` branches on `Bit`
- `??` branches on optional presence
- `?.` propagates absence through access
- `Expect` remains explicit unless a separate error/failure sugar is locked later

## Type Annotation Marker

`:` is the universal type annotation marker.

```ebnf
[95] type-annot        ::= ":" TYPE
[96] annotated-name    ::= IDENT type-annot
[97] annotated-result  ::= param-list type-annot
[98] annotated-receiver ::= "(" IDENT type-annot ")"
[99] annotated-pattern ::= PATTERN type-annot
```

This applies in value, parameter, field, result, receiver, pattern, and shape-member positions.

`:` is not overloaded for casts, subtyping, runtime type tests, type equivalence, or conformance. Those use their own operators.

`:=` remains binding/definition/initialization.

`=` remains equality. `/=` remains inequality.


## Callable Types

`->` is the callable type arrow.

```ebnf
[132] callable-type       ::= callable-input "->" TYPE
[133] callable-input      ::= TYPE | tuple-type
[134] multi-input-callable ::= "(" TYPE ("," TYPE)+ ","? ")" "->" TYPE
```

```musi
Int -> Text
(Int, Text) -> Unit
() -> Unit
```

`->` is a type-space callable arrow. It is not a curry operator in expression space.

`Unit` is the only canonical zero-information result type. `()` is the empty tuple type shape and canonicalizes to `Unit`.

Chained callable arrows require explicit design before they are accepted as implicit currying. Until that design is locked, parentheses spell intent.

```musi
A -> (B -> C)
(A, B) -> C
```

## Musi Source And SEIL Callable Surface

Musi source and SEIL metadata use the same callable type surface.

```ebnf
[199] source-callable-type ::= callable-type
[200] seil-callable-type   ::= callable-type
```

Musi source does not use old stack-effect bracket syntax as the callable surface. Callable types use `->`.

SEIL is not a separate source language with a different stack-effect signature syntax. It is the lowered verifier form of Musi, similar in role to CIL. SEIL metadata preserves callable types in Musi syntax for near-identical decompilation.

Stack-effect facts are verified by SEIL/runtime rules, but SEIL instruction forms are not locked here.

## Type Operator Family

Musi uses a coherent `:`-led family for type-related operators.

```ebnf
[100] type-test            ::= EXPR ":?" TYPE
[101] static-cast          ::= EXPR ":>" TYPE
[102] checked-cast         ::= EXPR ":?>" TYPE
[103] subtype-relation     ::= TYPE "<:" TYPE
[104] type-equivalence     ::= TYPE "~=" TYPE
[105] conformance-relation ::= TYPE "|=" TYPE
```

Meanings:
- `:` annotates
- `:?` tests runtime type and returns `Bit`
- `:>` requests explicit static conversion/cast
- `:?>` performs a checked runtime cast and returns an explicit failure-capable result
- `<:` states subtype relation
- `~=` states type equivalence relation
- `|=` states shape conformance/fits relation

Rules:
- `:?` never returns the narrowed value
- `:?>` never returns `Bit`
- `:>` is not runtime checked; it is an explicit static or known-correct conversion request
- `?=` is not accepted; it has no strong Musi rationale and does not belong to the coherent `:` type-operator family

## Expect And Checked Casts

`Expect` remains explicit.

```ebnf
[106] expect-type         ::= "Expect" "[" TYPE "," TYPE "]"
[107] checked-cast-result ::= "Expect" "[" TYPE "," "CastError" "]"
```

There is no locked error/failure sugar for `Expect`. Possible sugar such as `E!T`, a keyword, or another operator is left to future/community design unless a strong rationale appears.

`?T`, `??`, and `?.` are Maybe-only and do not apply to `Expect`.

`:?>` returns an explicit `Expect` value:

```musi
let checked : Expect[User, CastError] := value :?> User;
```

Rationale:
- failure stays distinct from absence
- failed casts carry error information instead of only `Bit` or `Maybe` absence
- no hidden exceptions are introduced
- `Expect` sugar is not locked prematurely

## Shape Naming

`shape` is the locked spelling for structural contracts.

```ebnf
[108] shape-expr   ::= "shape" shape-body
[109] shape-body   ::= "{" shape-member (";" shape-member)* ";"? "}"
[110] shape-member ::= "let" IDENT param-list? type-annot
```

`shape` means an observable structure/capability contract: a value or type fits a shape when it provides the required members and operations according to Musi's conformance rules.

`trait` is not accepted as the core spelling because it carries Rust, Scala, PHP, and C++ baggage around nominal implementations, mixins, coherence rules, code reuse, or type-level metadata conventions.

`data` defines what a thing is. `shape` defines what a thing must look like.

There is no separate `trait` keyword.

## Shape Conformance

Default `shape` conformance is structural.

A type or value fits a structural shape when it provides the required observable members and operations with compatible types and stack effects. No conformance declaration is required for structural shapes.

`@witness shape` defines a witness-required shape.

Witness-required shapes are for semantic, lawful, marker, or capability contracts where members alone are not enough to prove correct conformance. Empty marker shapes must use `@witness shape` to avoid every type fitting them accidentally.

`|=` is the locked conformance/fits relation operator.

```ebnf
[111] conformance-relation ::= TYPE "|=" TYPE
[112] witness-binding      ::= "let" TYPE "|=" TYPE ":=" record-datum
```

Roles:
- `T |= Shape` states or constrains that `T` fits `Shape`
- `let T |= Shape := witnessValue;` binds an explicit witness for witness-required conformance

There is no `impl`, `implements`, `extends`, or `trait` keyword. Receiver methods and witness bindings use universal `let`.

`|=` should not become a general-purpose ordinary Boolean test by default. Runtime fit checks for dynamic or opaque values remain an open design topic.

## Modules, Import, And Export

`import` and `export` are hard keywords with ESM-like directionality. `import` takes in. `export` puts out.

```ebnf
[150] import-expr   ::= "import" import-source
[151] import-source ::= STRING | record-datum | tuple-datum
[152] export-expr   ::= "export" let-expr | "export" export-block
[153] export-block  ::= "{" export-item (";" export-item)* ";"? "}"
[154] export-item   ::= let-expr
```

`import` is an expression that takes in a module, resource, or package according to module-system rules.

`known import` is compile-time import/acquisition.

Import can use datum literals for multiple import inputs.

```musi
let text := import "std/text";
let grammar := known import "grammar/musi";
let std := import #{
  text := "std/text",
  io := "std/io",
};
```

`export` marks a `let` binding for the current module surface. There is no other standalone form that `export` applies to. Standalone `match`, `while`, or arbitrary expressions are not export targets.

```musi
export let parse(input : Text) : Ast := parseText(input);
```

`export { ... }` is a structural export block. It is sugar over separate `export let ...;` forms.

```musi
export {
  let parse := parseText;
  let format := formatAst;
}
```

Musi modules are top-to-bottom strict. Export block items are processed in top-to-bottom order.

Module boundary forms affect source shape and SEIL/decompilation metadata.


## Module Values

Modules are records. Imports bring in records.

```ebnf
[155] module-value       ::= record-datum | named-record-value
[156] named-import-bind  ::= "let" IDENT ":=" import-expr
[157] anonymous-import   ::= "let" "_" ":=" import-expr
```

A named import binds the imported module record to a name.

```musi
let text := import "std/text";
```

An anonymous import brings the imported record into scope without binding the record itself to a name. This is equivalent to wildcard-style import in other languages, but expressed through universal `let`.

```musi
let _ := import "std/prelude";
```

Multi-import datum forms produce record-shaped imports.

```musi
let std := import #{
  text := "std/text",
  io := "std/io",
};
```



## Visibility

`export` is the only visibility mechanism.

```ebnf
[158] visibility-form ::= "export"
```

Rules:
- exported binding is visible from the module
- non-exported binding is module-private
- no `public`, `private`, `protected`, `internal`, or `hidden` visibility words exist
- `opaque` controls type abstraction, not visibility

Modules are records. Exports define the module record surface. Non-export is private by absence.

## Module SEIL Round-Trip

SEIL module metadata must preserve import/export information needed for semantic decompilation.

Import metadata must preserve:
- import binding mode: named or anonymous
- import source shape: string, tuple datum, or record datum
- known/runtime phase of the import

Named import and anonymous import are different source/module-scope operations.

```musi
let text := import "std/text";
let _ := import "std/text";
```

The first binds the imported record as `text`. The second brings the imported record contents into scope without binding the record itself to a name.

Export metadata must preserve exported binding names. Export block grouping may be preserved as source metadata for near-identical decompilation. If grouping metadata is absent, the decompiler may emit canonical separate `export let` forms while preserving semantics.

## Open Question Checklist

These questions are intentionally open and are not locked by this document.

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

- [ ] Bidirectional gradual type-system model
- [ ] Type-phase algebra for `|`, `&`, `^`, and `~`
- [ ] Union/intersection representation and normalization rules
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

#
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

## Runtime And SEIL

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
