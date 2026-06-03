# LOCKED_LANGUAGE_DESIGN.md — Heavy Compressed Reference

Status: compact, information-preserving reference of locked Musi language design before SEIL instruction definition. Grammar snippets are documentation grammar in W3C XML 1.0 EBNF style; omitted forms are not accepted unless added by another locked section.

## 1. Core invariants

- Musi is a small systems language with a small core.
- Expression-first: no separate statement semantic category; top-level `EXPR;` is an accepted top-level item; `;` sequences/discards; definitions are expressions; control flow is expression-based.
- Musi lowers directly to SEIL bytecode. SEIL is the canonical lowered form, like CIL in role.
- No IR layer exists between Musi source and SEIL.
- Source should lower to SEIL so SEIL-to-Musi decompilation can recover near-identical source when metadata is preserved.
- Syntax must preserve maximal-munch lexing, one-token-lookahead parsing, no speculative parsing beyond one token, and no syntax kept only by convention from existing languages.
- Any form needing more than one token of lookahead is rejected/redesigned.

Notation atoms:

```ebnf
IDENT   ::= /* lexical identifier token */
EXPR    ::= /* expression production defined by final grammar */
TYPE    ::= /* type-expression production defined by final grammar */
PATTERN ::= /* pattern production defined by final grammar */
ATTR    ::= /* attribute production defined by final grammar */
```

## 2. Keywords and non-keywords

Keyword rule: a keyword is a hard-reserved source word required to introduce/disambiguate a grammar form. Built-ins, compiler-owned names, intrinsics, methods, shapes, data names, product/sum names, and built-in types are not keywords unless hard-reserved grammar introducers.

Hard/form keywords, count 19:

```text
case cycle data defer else erased export fixed import known leave let match mut opaque shape when while yield
```

Contextual/non-keyword decisions:

```text
in     = contextual word operator, not form keyword
as     = contextual pattern keyword, not cast syntax
await  = ordinary name
spawn  = ordinary name
task   = ordinary name
```

`import` and `export` are hard keywords: `import` takes in; `export` puts out. `known import` is compile-time acquisition/import. Module boundary forms affect source shape and SEIL/decompilation metadata.

Not core keywords:

```text
unsafe pin recur for break continue next trait hidden static const fn type struct enum class impl and or xor not is
```

## 3. Comments

Locked comment grammar:

```ebnf
line-comment       ::= "--" line-comment-text
line-doc-comment   ::= "---" line-comment-text
line-module-doc    ::= "--!" line-comment-text
block-comment      ::= "/-" block-comment-body "-/"
block-doc-comment  ::= "/--" block-comment-body "-/"
block-module-doc   ::= "/-!" block-comment-body "-/"
block-comment-body ::= (block-comment | block-doc-comment | block-module-doc | block-comment-char)*
```

Maximal munch: `--!` module doc; `---` doc; `/--` block doc; `/-!` block module doc. Block, block-doc, and block-module-doc comments share one nesting system. Line comments inside block comments are text. Nested block comments use a linear depth counter. Unterminated nested block comments are diagnostics. Module docs are distinct from item docs.

## 3.1 Lexical literals and escaped identifiers

Numeric literal grammar:

```ebnf
digit-sep        ::= "_"
bin-prefix       ::= "0b" | "0B"
oct-prefix       ::= "0o" | "0O"
hex-prefix       ::= "0x" | "0X"
nat-suffix       ::= ("n" | "N") DIGITS
int-suffix       ::= ("i" | "I") DIGITS
float-suffix     ::= ("f" | "F") DIGITS
numeric-literal  ::= /* digits with optional `_` separators, base prefix, and numeric suffix */
multiline-string ::= "\"\"\"" multiline-string-body "\"\"\""
escaped-ident    ::= "`" escaped-ident-body "`"
```

Numeric rules:
- `_` separates digits inside numeric literals.
- `_` separators do not affect numeric value.
- `0x`/`0X`, `0o`/`0O`, and `0b`/`0B` are accepted base prefixes.
- numeric suffixes are case-insensitive: `n64` = `N64`, `i32` = `I32`, `f64` = `F64`.
- canonical formatting uses lowercase suffixes.
- `nX` suffix means natural/unsigned width.
- `iX` suffix means signed integer width.
- `fX` suffix means floating width.
- unsuffixed non-negative integer literals are `Nat`.
- negative integer expressions are `Int`.
- a decimal point or `fX`/`FX` suffix makes the literal `Float`.
- unsupported literal width for the target/profile is a diagnostic.

```musi
1_000n32
0xff_n8
0XFFN8
0b1010_0110n8
0o755n16
1i64
-1
1.0f64
1F32
```

Multi-line strings use triple quotes.

```musi
"""
plain text
multiple lines
"""
```

Rules:
- triple-quoted strings are strings, not template literals.
- no interpolation exists inside string literals.
- JS-like template literals are not core Musi.
- formatting/interpolation is explicit API/library behavior, not syntax.
- backticks are not string delimiters.
- `$` and `${` are reserved in string-literal design space for possible future interpolation discussion.
- C#-style `$"..."` and `$"""..."""` string modes are not core Musi.
- Swift-style `\(...)` interpolation is not core Musi.
- `{name}` is not core interpolation grammar.
- if interpolation is ever added, `$name` / `${EXPR}` is the reserved direction.
- no automatic indentation trimming/dedent occurs; source contents between delimiters are the string contents.
- indentation trimming belongs in explicit library/API calls, not string literal syntax.

Backticks are Swift-like escaped identifiers.

```musi
let `when` := 1;
let `Type` := Type;
let `weird-name` := 2;
```

Rules:
- escaped identifiers are identifiers, not strings.
- escaped identifiers may spell reserved keywords.
- escaped identifiers may contain characters not accepted in ordinary identifiers when the lexer permits them.
- escaped identifiers do not create new operators.
- no interpolation exists inside escaped identifiers.
- escaped identifiers are single-line.
- canonical/decompiler output prefers ordinary identifiers when possible and backticks only when required.

## 4. Universal binding and generics

`let` is the universal binding form for values, functions, data definitions, shape definitions, modules/imports, compile-time values, runtime values, and attached receiver methods. No `fn`, `type`, `struct`, `enum`, `class`, `impl`, `const`, or `static` keyword exists.

```ebnf
let-expr              ::= "let" bind-head generic-param-list? type-annot? param-list? result-type? ":=" EXPR
                        | "let" receiver-head "." IDENT generic-param-list? param-list result-type? ":=" EXPR
bind-head             ::= IDENT | "_" | operator-name | PATTERN
receiver-head         ::= "(" IDENT type-annot ")"
generic-param-list    ::= "[" generic-param-list-body? "]"
generic-param-list-body ::= required-generic-param ("," required-generic-param)* ("," default-generic-param)* ","?
                          | default-generic-param ("," default-generic-param)* ","?
required-generic-param ::= IDENT type-annot?
default-generic-param  ::= IDENT type-annot? ":=" EXPR
param-list            ::= "(" param-list-body? ")"
param-list-body       ::= required-param ("," required-param)* ("," default-param)* ","?
                        | default-param ("," default-param)* ","?
required-param        ::= IDENT type-annot | IDENT type-annot?
default-param         ::= IDENT type-annot? ":=" EXPR
result-type           ::= type-annot
type-annot            ::= ":" TYPE
```

Defaults must trail in every parameter list: function, method, constructor-like, variant payload, generic. Bracketed generic parameters occur between binding name and ordinary call parameters. Generic call arguments use the same bracket-before-call shape: `name[Int, 4](value)`, `point.Point.make[Int]()`, `name[N := 4](value)`, `name[_, 4](value)`.

Generic rules:

- Omitting bracket list asks compiler to infer all generic parameters.
- Explicit generic call arguments may be positional or named with `:=`.
- `_` in generic call argument is an explicit inference hole.
- Required generic parameters without explicit/defaulted/inferred value are diagnostics.
- Unresolved inference holes are diagnostics.
- Generic call arguments follow UALO.

Binding qualifiers:

```ebnf
let-binding        ::= "let" bind-head type-annot? ":=" EXPR
qualified-binding  ::= "let" bind-head ":" qualified-type ":=" EXPR
qualified-rhs-bind ::= "let" bind-head ":=" qualified-expr
qualified-type     ::= "known"? "fixed"? "mut"? TYPE
qualified-expr     ::= "known" EXPR | EXPR
```

Canonical qualifier order: `known fixed mut TYPE`. Other orders are absent from accepted grammar and may be canonicalized by diagnostics/formatting. Without annotation, inference preserves RHS qualified type; it does not invent or strip qualifiers.

## 5. Regions and separators

Computation regions:

```ebnf
computation-region ::= "(" computation-body? ")"
computation-body   ::= EXPR (";" EXPR)* ";"?
```

Inside computation regions, `;` sequences/discards. `(step1(); step2())` returns/effects `step2`. `(step1(); step2();) ` discards `step2` and produces `Unit` or empty stack effect. Leading `;` is rejected.

Structural regions:

```ebnf
structural-region ::= "{" structural-body? "}"
structural-body   ::= structural-member (";" structural-member)* ";"?
structural-member ::= data-field | data-case | shape-member | match-case
```

Curly structural regions define members/fields/variants/cases/rule tables, not sequential computation. Structural `;` terminates a member/rule and does not discard.

Trailing separator invariant:

```ebnf
comma-items     ::= EXPR ("," EXPR)* ","?
semicolon-items ::= structural-member (";" structural-member)* ";"?
```

Trailing separators allowed only after an item. No leading separators. Comma lists use `X ("," X)* ","?`; structural regions use `X (";" X)* ";"?`; computation regions use `;` as sequence/discard.

## 6. Conditional control, loops, defer, yield

Conditional syntax:

```ebnf
total-conditional ::= non-when-expr "when" non-when-expr "else" EXPR
guarded-emission  ::= non-when-expr "when" non-when-expr
non-when-expr     ::= /* expression production excluding unparenthesized when-expr */
```

Rules:

- `when` is postfix guard syntax, not prefix; no `then` keyword.
- Condition must be `Bit`.
- Total conditional branches must have compatible type/stack effect.
- `else` is explicit fallback.
- Bare `VALUE when CONDITION` is guarded zero-or-one emission only in contexts that consume zero-or-one emission.
- No hidden `Maybe`, `Unit`, bottom, or union is synthesized.
- Unparenthesized nested `when` is rejected in guarded value or condition position; use parentheses.
- `where` is not a guard keyword and has no core guard syntax.
- Universal postfix guard: `X when C` makes `X` conditional on `C`; `C : Bit`; `C` evaluates at the point where `X` would be admitted/registered/emitted/selected; each guarded context defines that meaning.

Loop syntax:

```ebnf
while-expr   ::= "while" EXPR computation-region
loop-control ::= "leave" | "cycle"
```

Rules: `while` is the only source loop form; zero-or-more conditional repetition; condition `Bit`; body is computation region; result `Unit`; `leave` exits nearest `while`; `cycle` skips rest of body and checks condition again. No `for`, `break`, `continue`, `next`, or `recur` keyword. Iterable loops use ordinary functions/methods/shapes. Postcondition repetition uses sequencing + `while` or named library helpers. `recur` is rejected as a keyword because it would duplicate ordinary recursion and violate binding qualifier rules. `pin` is not core; stable-address semantics use `fixed`.

Defer/yield:

```ebnf
defer-expr ::= "defer" EXPR ("when" EXPR)?
yield-expr ::= "yield" EXPR?
```

`defer` registers cleanup for current computation region/scope exit and produces `Unit`. Cleanup runs on normal exit and loop-control exits (`leave`, `cycle`). Cleanup ordering remains runtime/control-flow design. `defer cleanup() when cond`: guard checked at registration; `Bit.True` registers cleanup; `Bit.False` registers none; guard is not re-evaluated at cleanup; cleanup expression must produce `Unit`; captured values must remain valid by scope/lifetime rules.

`yield` is a core expression for resumable/generator-compatible contexts, not an ordinary call. Outside such context it is diagnostic. `yield expr` suspends/emits through enclosing protocol; yielded type must match protocol output; bare `yield` only for `Unit`; local result is `Unit`; suspension is not scope exit; `defer` does not run at suspension; pending defers run on final scope exit/close/drop/cancel by final resumable runtime rules. Concurrency is protocol/capability driven. `Task`, `Scheduler`, `Resumable`, `Generator`, `Stream` are library/runtime names, not keywords. `await`, `spawn`, `task` remain ordinary names.

## 7. Match and patterns

Match grammar:

```ebnf
match-expr        ::= "match" EXPR "{" match-case+ "}"
match-case        ::= "case" case-pattern-list case-guard? "=>" EXPR ";"
case-pattern-list ::= PATTERN ("," PATTERN)* ","?
case-guard        ::= "when" EXPR
lambda-expr       ::= '\\' param-list type-annot? "=>" EXPR
```

Rules: every arm starts `case` and ends `;`; `=>` is body/result arrow for match arms and lambdas; arm `;` terminates structural case and does not discard selected value; use computation-region final `;` to discard inside arm. Pattern alternatives use commas in one `case`, not `|`; alternatives share guard/body; bindings across alternatives must be compatible: same names with compatible types in every alternative reaching body.

Exhaustiveness: `match` is exhaustive by default; non-exhaustive match is semantic error. Finite sum `data` matches must cover all variants or include wildcard `case _`. Guarded cases do not count as unconditional coverage. No `case else`; `else` belongs only to `when ... else`.

Guard evaluation: cases top-to-bottom; alternatives left-to-right. Pattern tested before guard. Guard runs only after matching alternative, may reference pattern bindings, must be `Bit`. If matched guard is false, matching continues. Guards do not run for non-matching patterns. First unguarded match or guard-true match wins. Guarded cases are conditional coverage only.

Pattern grammar:

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

Pattern facts:

- Patterns mirror datum syntax for destructuring.
- Let binding heads may be patterns.
- Record shorthand `#{ name }` means `#{ name := name }`.
- `as` is contextual pattern alias, not cast syntax; alias binds whole matched value.
- In alternatives, aliases must be binding-compatible if shared body uses them.
- `_` matches and binds nothing.
- `_name` is an ordinary identifier if lexical grammar accepts it; it does not silence unused-binding checks.

Rest patterns:

```ebnf
rest-pattern         ::= ".." identifier-pattern?
array-rest-pattern   ::= ".." identifier-pattern?
record-rest-pattern  ::= ".." identifier-pattern?
```

At most one rest pattern per tuple/record/array. Array/record rest may ignore or bind remaining elements/fields. Tuple rest requires tuple-rest/variadic tuple semantics; until locked, tuple rest is not accepted.

## 8. Data, datum literals, construction

Data grammar:

```ebnf
data-expr              ::= attr-list? "data" data-body
data-body              ::= product-data-body | sum-data-body | empty-data-body
product-data-body      ::= "{" data-field (";" data-field)* ";"? "}"
sum-data-body          ::= "{" data-case (";" data-case)* ";"? "}"
empty-data-body        ::= "{" "}"
data-field             ::= "let" IDENT type-annot field-default? | "let" IDENT ":=" EXPR
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

Rules: `data` is the only data-definition form. Body shape decides product/sum/empty. Product `let` entries and sum `case` entries must not mix. `case ... := value` initializes/defines variant identity; tag/discriminant must be known and unique; omitted tags assigned by compiler in declaration order; payload defaults stay in payload params. Product and sum data stay separate; if both are needed, use one as a field of the other. `data` body may bind data-valued fields/associated data through `let`. Receiver methods are outside `data`/`shape`: `let (self : Parent).method() := expr`. No `struct`, `enum`, `union`, `class`, `impl`.

Datum literals:

```ebnf
datum-literal      ::= tuple-datum | record-datum | array-datum
tuple-datum        ::= "#(" (EXPR ("," EXPR)* ","?)? ")"
record-datum       ::= "#{" (record-datum-field ("," record-datum-field)* ","?)? "}"
array-datum        ::= "#[" (EXPR ("," EXPR)* ","?)? "]"
record-datum-field ::= IDENT ":=" EXPR
```

Meanings: `#()` empty tuple datum canonicalizes to `Unit`; `#{}` empty record datum; `#[]` empty array/list datum requiring type context. Plain `{...}` is never value record literal. Plain `(...)` is never tuple datum unless `#(`.

Type delimiters/indexing:

```ebnf
tuple-type          ::= "(" (TYPE ("," TYPE)* ","?)? ")"
array-list-type     ::= "[" array-bound? "]" TYPE
array-bound         ::= EXPR | EXPR ".." EXPR | EXPR "..<" EXPR
generic-application ::= TYPE "[" (TYPE ("," TYPE)* ","?)? "]"
tuple-field-access  ::= EXPR "." INT
array-index-access  ::= EXPR ".[" EXPR "]"
```

Array/list types: `[]T` dynamic/unbounded sequence; `[N]T` exact known length; `[A .. B]T` inclusive known length range; `[A ..< B]T` half-open known length range. Bounds must be known `Nat`. Generic/type application: `T[A, B]`. Tuple fields use `pair.0`; array/list indexing uses `list.[0]`.

Construction:

```ebnf
product-construction ::= TYPE record-datum
inferred-product     ::= record-datum
sum-construction     ::= unqualified-variant | qualified-variant
unqualified-variant  ::= "." IDENT variant-args?
qualified-variant    ::= TYPE "." IDENT variant-args?
variant-args         ::= "(" (EXPR ("," EXPR)* ","?)? ")"
```

Rules: product data construction uses named or unnamed record datum literals and not function-call syntax. Sum construction selects a variant by dot variant syntax, e.g. `.Some(Type)` or `Maybe.Some(Type)`.

## 9. Operators and expression parsing

Core has no user-defined symbolic operators. Only locked operator tokens have operator syntax. Domain-specific operations use named functions/methods. Fixed tokens:

```text
. ?. .[ ?.[ #( #{ #[ : := :? :> :?> <: ~= |= = /= < <= > >= in + - * / % |< >| >+ @< @> & ^ | ~ ?? .. ..< => ->
```

`in` is the only core word operator; contextual in operator position. No `and`, `or`, `xor`, `not`, `is`, `lsh`, `rsh`, or similar word operators. Negated membership: `~(x in y)`.

Relations:

```ebnf
equality-op    ::= "=" | "/="
ordering-op    ::= "<" | "<=" | ">" | ">="
equivalence-op ::= "~="
membership-op  ::= "in"
```

`=` equality only, never assignment. `/=` inequality. `< <= > >=` ordering. `~=` type/equivalence, not approximate numeric equality. Approximate equality is named function/method due tolerance/units/error/domain/type dependence.

Binding/update:

```ebnf
binding-expr ::= "let" bind-head type-annot? ":=" EXPR
update-expr  ::= place-expr ":=" EXPR
place-expr   ::= IDENT | EXPR "." IDENT | EXPR "." INT | EXPR ".[" EXPR "]"
```

`:=` binds/defines/initializes/updates. Record/product datum fields use `:=` because they initialize. Updates require mutable access or equivalent capability. `:=` lowest precedence. Chained updates are never accepted in core Musi; `a := b := c` is diagnostic.

Algebra:

```ebnf
algebra-op ::= "&" | "|" | "^" | "~"
```

Meanings: `&` conjunction/bitwise-and/type intersection where proven; `|` disjunction/bitwise-or/type union where proven; `^` xor/symmetric difference where proven; `~` complement/not where proven. No separate logical/bitwise split. Applies to `Bit`, `Word`, `Word8`, `Word16`, `Word32`, `Word64`, `Bits[N]`, and type algebra where accepted. Guard contexts require `Bit`; no truthiness. Short-circuiting is control flow via `when ... else` or `match`. Not core: `and or xor not && || ! &? |? ~? |>`.

`Bit` is a sum type with known discriminants; `true`/`false` are ordinary predefined/core bindings, not keywords. Canonical variants: `Bit.True`, `Bit.False`; shorthand `.True`/`.False` allowed when expected type is `Bit`.

```musi
export let Bit := data { case False := 0; case True := 1; };
export let true : Bit := Bit.True;
export let false : Bit := Bit.False;
```

Parser tiers:

```ebnf
postfix-expr      ::= EXPR postfix-op+
postfix-op        ::= "." IDENT | "." INT | ".[" EXPR "]" | "?." IDENT | "?.[" EXPR "]" | generic-call-args | call-args
generic-call-args ::= "[" arg-list? "]"
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

Precedence high→low:

1. postfix access/call/index
2. prefix unary/modifiers
3. callable arrow in type position `->`
4. `* / %`
5. `+ -`
6. shifts/rotates `|< >| >+ @< @>`
7. ranges `.. ..<`
8. relation/type/equality/membership `< <= > >= = /= ~= :? :> :?> <: |= in`
9. `&`
10. `^`
11. `|`
12. `??`
13. `when ... else` / `when`
14. `:=`

Rules: expressions are not parsed as one flat semantic chain; precedence is syntax. `%` is CPU-style remainder, not mathematical modulo; true modulo belongs in named op such as `mod(a,b)`. Shift/rotate are maximal-munch single tokens. No `<<`/`>>`. No arithmetic-left-shift operator exists; left shift is `|<` and fills low bits with zero. Algebra precedence: `&` > `^` > `|`. Relational/type/equality operators are non-chainable. `??` is right-associative and Maybe-only. Shift meanings: `|<` zero-fill left; `>|` zero-fill right; `>+` sign-fill arithmetic right; `@<` rotate left; `@>` rotate right.

### UDNS and UFCS

UDNS = Universal Dot Notation Syntax. Dot notation covers member access, receiver-method access, tuple field access, namespace/module access, variant qualification, optional access, and indexed access compounds.

```ebnf
dot-postfix ::= "." IDENT | "." INT | ".[" EXPR "]" | "?." IDENT | "?." IDENT call-args | "?.[" EXPR "]"
```

Owned shapes: `value.member`, `value.method(args)`, `tuple.0`, `module.item`, `Type.Variant(args)`, `.Some(args)`, `value?.member`, `value?.method(args)`, `value.[index]`, `value?.[index]`.

UFCS is semantic resolution over UDNS: receiver methods from `let (self : T).method(...) := ...` use same dot/call surface as ordinary members. `|>` is absent; UDNS/UFCS are fluent composition mechanism.

UDNS resolution order for `x.foo` / `x.foo(args)`:

1. direct member/field/variant/module-record member
2. shape member required by known static type/constraint
3. attached receiver method
4. explicit dynamic/capability member operation only for `Any`, `opaque`, or capability-gated type

Ambiguity at same priority is diagnostic. If a higher-priority candidate exists but is unusable, diagnostic; no fallback. Direct structure owns dot names. Receiver methods do not shadow fields/shape members. If `x.foo` is non-callable direct field and receiver method `foo` also exists, `x.foo()` diagnoses non-callable member. `Any` has no implicit duck-dot lookup; dynamic lookup is explicit operation/capability. No receiver-method escape syntax; receiver qualification uses ordinary UDNS/module/type paths.

## 10. Type-system surface

Musi uses bidirectional type checking/inference.

Rules:

- Missing annotations request inference, not dynamic fallback.
- Annotations push expected types inward; expressions synthesize types outward.
- Inference chooses the most precise/principal supported type.
- Irresolvable ambiguity is diagnostic.
- Inference never silently inserts `Any`.
- Dynamic boundaries must be explicit through annotation, conversion, import/FFI boundary, or API return type.
- Inferred structural, row, and capability constraints are ordinary Musi type info.

Core lattice/types:

```ebnf
universe-type    ::= "Type" "[" EXPR "]"
universe-param   ::= "known" IDENT ":" "Nat"
type-alias       ::= "Type"
type-hole        ::= "_"
any-type         ::= "Any"
unit-type        ::= "Unit" | "()"
empty-type       ::= "Empty"
error-type       ::= "Error"
structural-type  ::= record-type | row-type | capability-type
```

Facts:

- `Type[N]` is primitive universe use form; `N` is `known Nat`; runtime/non-known `N` is diagnostic.
- Known requirements are explicit at definition sites; call sites omit `known` because callee/form declares requirement.
- `Type[N] : Type[N + 1]`.
- `Type` is predefined core binding alias: `export let Type := Type[0];`.
- `type` remains not keyword. `Type0`, `Type1`, etc. are not built-in source forms unless defined.
- `()` canonicalizes to `Unit`.
- `Empty` is uninhabited bottom, not empty tuple; expression of type `Empty` may fit any required result because it never returns.
- `Any` is explicit dynamic top value type; not null, absence, failure, callable-anything, or permission to ignore effects/capabilities.
- Dynamic-to-static use requires `:?>` or explicit checked operation.
- Static-to-dynamic use is explicit via annotation/conversion/dynamic boundary.
- `Error` is built-in non-keyword top error type. Specific errors such as `AnyError`, `CastError` subtype `Error`.
- Unannotated record datum may infer anonymous structural record/row type, not `Any`.
- FFI/exported ABI boundaries require explicit representable types or representation metadata; anonymous structural/row types do not silently become ABI types.

Type annotation:

```ebnf
type-annot         ::= ":" TYPE
annotated-name     ::= IDENT type-annot
annotated-result   ::= param-list type-annot
annotated-receiver ::= "(" IDENT type-annot ")"
annotated-pattern  ::= PATTERN type-annot
```

`:` applies to value, parameter, field, result, receiver, pattern, and shape-member positions. It is not cast/subtype/type-test/equivalence/conformance syntax.

Callable types:

```ebnf
callable-type        ::= callable-input "->" TYPE
callable-input       ::= TYPE | tuple-type
multi-input-callable ::= "(" TYPE ("," TYPE)+ ","? ")" "->" TYPE
source-callable-type ::= callable-type
seil-callable-type   ::= callable-type
```

`->` is type-space callable arrow, not expression currying. `Unit` is canonical zero-information result. `()` is empty tuple type shape → `Unit`. Chained arrows require explicit design; use parentheses (`A -> (B -> C)`, `(A, B) -> C`). Musi source and SEIL metadata share callable type surface. No old source stack-effect bracket syntax. SEIL verifies lowered stack behavior while metadata preserves callable types for near-identical decompilation.

Stack-effect compatibility:

- Total conditionals require `cond : Bit`; branches unify to one observable result/effect; `Empty` branch lets other determine result; expected context pushes into both branches.
- Bare guarded emission produces zero-or-one emission accepted only by consuming context.
- `match` arms unify to one observable result/effect; `Empty` arms do not force result; guarded arms only conditional coverage; match exhaustive.
- `defer : Unit`; cleanup expression `Unit`; cleanup effect attaches to scope/region exit, not local result; cleanup cannot consume non-live values.
- `yield` only in compatible resumable/generator callable contexts; yielded value matches output protocol; local result `Unit`; suspension not scope exit; `defer` not run on suspension; pending defers run on final exit/close/drop/cancel.
- Receiver methods treat receiver as semantic first input/capability; receiver syntax preserved by source metadata/decompilation; receiver mutability/stability comes from receiver type: `T`, `mut T`, `fixed T`, `fixed mut T`.

Type algebra:

```ebnf
type-union        ::= TYPE "|" TYPE
type-intersection ::= TYPE "&" TYPE
type-difference   ::= TYPE "^" TYPE
type-complement   ::= "~" TYPE
```

Meanings: `A | B` union; `A & B` intersection; `A ^ B` symmetric difference; `~A` complement within relevant type universe. Derived law: `A ^ B = (A | B) & ~(A & B)`.

Normalization laws:

```text
A|A=A; A&A=A; A|Empty=A; A&Empty=Empty; A|Any=Any; A&Any=A; ~~A=A; A^A=Empty; A^Empty=A; A^Any=~A
```

Subtyping/equivalence facts: `A <: B` iff `A | B ~= B`; `A & B <: A`; `A & B <: B`. Complement/symmetric difference accepted only where universe makes them normalizable/checkable; ambiguous/non-normalizable type algebra is diagnostic. Type algebra is type space; ABI/FFI representability checked separately. Algebraic types do not silently become ABI-safe tagged unions/layout records. No `iff`, `<=>`, `<->`, `=>`, `==` for type logic. Type equivalence `~=`; subtyping `<:`.

UALO = Universal Argument-List Ordering:

```ebnf
arg-list       ::= positional-arg ("," positional-arg)* ("," named-arg)* ","? | named-arg ("," named-arg)* ","?
positional-arg ::= "_" | EXPR
named-arg      ::= IDENT ":=" EXPR
call-args      ::= "(" arg-list? ")"
```

Rules: positional arguments first; named arguments after; once named starts, positional cannot resume; definition defaults trail; duplicate/unknown named args diagnostic; same parameter cannot be bound twice. Applies to ordinary calls, generic calls, attributes, parameter/default definitions, named variant payload calls, and future argument-list-shaped syntax. Variant payload calls accept named arguments under UALO, e.g. `.Point(x := 1, y := 2)`. UALO is a surface invariant; receiver-style callable access uses ordinary binding/call semantics.

Type-operator family:

```ebnf
type-test            ::= EXPR ":?" TYPE
static-cast          ::= EXPR ":>" TYPE
checked-cast         ::= EXPR ":?>" TYPE
subtype-relation     ::= TYPE "<:" TYPE
type-equivalence     ::= TYPE "~=" TYPE
conformance-relation ::= TYPE "|=" TYPE
```

Meanings: `:` annotation; `:?` runtime type test → `Bit`; `:>` explicit static conversion/cast, not runtime checked; `:?>` checked runtime cast → explicit failure-capable result; `<:` subtype; `~=` type equivalence; `|=` shape conformance/fits. `:?` never returns narrowed value. `:?>` never returns `Bit`. `?=` is not accepted.

Optional/Maybe:

```ebnf
optional-type   ::= "?" TYPE
maybe-fallback  ::= EXPR "??" EXPR
optional-access ::= EXPR "?." IDENT | EXPR "?." IDENT call-args | EXPR "?.[" EXPR "]"
```

`?T` is `Maybe[T]`. `?` does not name `Expect`. `??` only for `?T`/`Maybe[T]`; fallback lazy; result `T`. `?.` only for `?T`/`Maybe[T]`; access/call/index only when present; absent stays absent; no null invented; composes with `??`. `when ... else` branches on `Bit`; `??` branches on optional presence; `?.` propagates absence. `Expect` remains explicit; no failure-propagation sugar exists in core Musi.

Expect/checked casts:

```ebnf
expect-type         ::= "Expect" "[" TYPE "," TYPE "]"
checked-cast-result ::= "Expect" "[" TYPE "," "CastError" "]"
```

`:?>` returns `Expect[Target, CastError]`. No locked `Expect` sugar. `?T`, `??`, `?.` are Maybe-only. Failed casts carry error info. `CastError <: Error`. No hidden exceptions.

Dynamic `Any` capabilities:

- `Any` does not imply implicit dot/call/index lookup.
- Dynamic ops are explicit witness-required shapes: e.g. `AnyMember.member(name) : Expect[Any, AnyError]`, `AnyIndex.index(key) : Expect[Any, AnyError]`, `AnyCall.call(name,args) : Expect[Any, AnyError]`.
- `AnyMember`, `AnyIndex`, `AnyCall`, `AnyError`, `Error` are ordinary built-in/library/runtime names, not keywords.
- `AnyError <: Error`; APIs may widen to `Expect[Any, Error]`.
- `Any` values do not automatically provide dynamic capabilities; APIs decide which values carry/provide them.

Fixed storage:

```ebnf
fixed-type     ::= "fixed" TYPE
fixed-mut-type ::= "fixed" "mut" TYPE
qualified-type ::= "known"? "fixed"? "mut"? TYPE
```

`fixed T` = storage-qualified `T` with stable address for value lifetime, not movable by collector/runtime during that lifetime. It does not mean static/global, immutable, compile-time, type-associated, permanent, or thread-safe. Orthogonal to `mut`: `fixed T` stable address; `mut T` mutable access; `fixed mut T` both. Address-taking requires `fixed`; movable values cannot expose stable raw addresses. No separate `pin` keyword/expression/block. Pinning semantics represented by `fixed`. Temporary non-moving access uses APIs/capabilities over `fixed`. GC/runtime pinning hidden behind `fixed` lowering/runtime behavior. Stable address of non-`fixed` storage is error. FFI APIs needing stable pointers require `fixed` storage or explicit copy/borrow APIs.

Opaque/erased:

```ebnf
opaque-type ::= "opaque" TYPE
erased-type ::= "erased" TYPE
```

Type-space modifiers, not attributes. Affect type identity, representation, dispatch, checking, ABI/SEIL metadata, decompilation. `hidden` removed; use `opaque` for existential hiding, `erased` for opaque-result/static-hidden concrete type, `export`/absence for visibility, attributes for representation/ABI/interop. `opaque T` ≈ Swift `any T`: existential/capability with concrete type hidden behind `T`. `erased T` ≈ Swift `some T`: exposed type hides concrete name while definition has one compiler-known concrete type; static specialization may remain possible.

## 11. Known phase

`known` is a phase modifier asking whether something can be compile-time. It is not `const` or `static`.

Rules:

- `known expr` requests/requires compile-time evaluation.
- `known T` requires compile-time-known value/type-phase value of type `T`.
- `known` appears only where compile-time availability is meaningful.
- Known requirements explicit at definition sites using `known` in type position.
- Call sites to known-required params do not repeat `known`; compiler checks argument is known.
- If context already requires knownness, spelling omitted at use site.
- If value cannot be compile-time, diagnostic.
- Without `known`, evaluation is runtime unless context requires knownness.
- Known phase can construct datum literals when contained values are known-compatible.
- Case tags/discriminants and array/list bounds require known values by context.
- `known import` is compile-time acquisition/import.

Boundary: known code may depend only on known values, known imports, type info, and compiler-permitted deterministic known intrinsics. Runtime values cannot be captured by `known`. Known values may generate or runtime-initialize representable values. Known→runtime allowed by embedding/lowering known result. Runtime→known forbidden.

Known functions are Musi lowered to SEIL; known evaluation executes SEIL in known phase. No separate source-tree evaluator. Known evaluation is deterministic/resource-limited: no ambient runtime state; no wall-clock/time/random/environment/process/IO unless explicit deterministic known import/intrinsic; bounded fuel/step/memory limits are compiler settings; nontermination/limit exhaustion diagnostic; no dependency on target runtime mutable state; may use known imports, pure computation, type information, compiler-approved deterministic intrinsics.

## 12. Safety, pointers, FFI

No `unsafe` keyword or unsafe expression/block form.

```ebnf
unsafe-keyword ::= /* no production */
```

Unsafety is represented by operation metadata, capabilities, types, and diagnostics. Dangerous behavior is error, not warning: memory unsafety, capability violation, invalid pointer use, invalid FFI boundary use, invalid representation layout, runtime-to-known phase violation, unchecked dynamic failure. Warnings are for portability, deprecation, performance, unused bindings/imports, suspicious-but-defined code, or style/tooling. Dangerous-but-allowed operations require explicit type/capability/API/metadata representation.

Pointer types are built-in/library types, not keywords:

```ebnf
unsafe-ptr        ::= "UnsafePtr" "[" TYPE "]"
unsafe-mut-ptr    ::= "UnsafeMutPtr" "[" TYPE "]"
unsafe-opaque-ptr ::= "UnsafeOpaquePtr"
```

Rules: `UnsafePtr[T]` readable pointer; `UnsafeMutPtr[T]` readable/writable pointer; `UnsafeOpaquePtr` opaque FFI/handle pointer without typed pointee access. Pointer creation explicit/capability-checked. Stable address creation requires `fixed`; mutable pointer creation requires `fixed mut`. No `&x` address-of, no `*p` dereference, no core pointer arithmetic. Pointer ops are explicit methods/fields/intrinsics/capabilities. Typed pointee access uses UDNS `.pointee`. `UnsafePtr[T].pointee` reads `T`; `UnsafeMutPtr[T].pointee` reads/writes `T`. Invalid pointer use is error. `UnsafeOpaquePtr` must be explicitly cast/converted through API/capability before typed access.

FFI uses attributes + ordinary `let`, not keywords:

```ebnf
extern-attr   ::= "@extern" attr-args
extern-import ::= extern-attr let-decl
extern-export ::= extern-attr "export" let-expr
repr-attr     ::= "@repr" attr-args
let-decl      ::= "let" bind-head generic-param-list? type-annot? param-list? result-type? ";"
```

Rust analogy: Rust `use` → Musi `import`; Rust `pub` → Musi `export`; Rust `extern` → Musi `@extern`. `@extern` is the only FFI boundary attribute. Direction determined by body presence and `export`.

Rules: `@extern let ...;` imports external implementation. `@extern export let ... := ...;` exposes Musi implementation outward. `@extern` with body but without `export` is diagnostic. `export` remains module visibility only. No `foreign` keyword, `extern` keyword, `@export`, `@abi`, or `@expose` attribute. `@repr(...)` controls representation/layout. FFI boundary types must be representable. Anonymous structural/row types are not FFI boundary types. `Any`, `opaque`, `erased`, closures, shapes, `Maybe`, `Expect`, and GC references are not FFI-safe unless a profile explicitly defines representation. Strings are not silently C strings. Pointers use `UnsafePtr`, `UnsafeMutPtr`, `UnsafeOpaquePtr`. FFI failure explicit through return values/wrappers; no hidden exceptions. Unsupported profile/calling convention/layout/type combo is diagnostic.

`@extern` args follow UALO: first positional external profile, second symbol. Meta-level call canonicalizes to known metadata record. Fields: `profile`, `symbol`, `link`, `calling` (default `.cdecl` for outward `.c`), `variadic` (ABI-specific, e.g. `.c`). C ABI names such as `CVoid`, `CChar`, `CInt`, `CLongLong`, `CSize` are ordinary predefined/core/library bindings; exact representation is implementation/profile-defined.

## 13. Attributes and representation metadata

Attributes are structural metadata prefixes attached to the next grammar-owned node. They do not compute, branch, emit runtime values, or participate in runtime evaluation. Payloads are known meta-level function calls. UALO applies. Schema maps positional slots, named args, defaults, allowed target node kinds, repeatability to canonical known metadata record.

```ebnf
attr-list          ::= attr+
attr               ::= "@" attr-name attr-args?
attr-name          ::= IDENT ("." IDENT)*
attr-args          ::= "(" attr-arg-list? ")"
attr-arg-list      ::= arg-list
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

Confirmed attributes: `@packed`, `@align(...)`, `@witness`. Meanings: `@packed` packed/bit-structured representation metadata; `@align(...)` alignment metadata; `@witness` makes a `shape` require explicit witness conformance.

Attribute rules:

- Arguments are compile-time metadata values; positional and named accepted; named use `:=`; datum literals and sum values accepted.
- Schemas define positional parameter names, named params, defaults, allowed targets, repeatability, canonical metadata record shape.
- Attribute calls canonicalize to metadata records; e.g. `@align(4)` → `#{ value := 4 }`, `@repr(.c, tag := .n8)` → `#{ profile := .c, tag := .n8 }`.
- Conditional attributes are not separate grammar; conditionality belongs in payload, e.g. non-keyword field `enabled := ...`; if schema defines it as condition, it must be `known Bit`; `True` means metadata present, `False` absent; no runtime branch.
- Attributes may prefix grammar-owned nodes only; arbitrary infix expressions need wrapped computation region.
- Attribute applies only to exact next node; child propagation only by schema.
- Unknown compiler-affecting attributes are diagnostics. Tooling-only attributes must be namespaced, such as `@tool.name(...)`, and are ignored by compiler semantics unless a tool handles them. Native/compiler modules use `musi:` import prefixes, such as `musi:core` and `musi:ffi`; these are native modules with optional `.ms` interface surfaces, not ordinary Musi implementations.
- Repeatability is schema-defined; repeated unique attribute on same target is diagnostic.
- Recognized attributes preserved in SEIL metadata when affecting representation, ABI, checking, tooling, or near-identical decompilation.
- Packed/bit-structured data remains `data`; no `bitstruct` keyword.

`@target` is the structural availability attribute. It follows the same attribute meta-level function-call/UALO rules as every other attribute and canonicalizes to known metadata.

Target rules:

- `@target(...)` attaches to the next grammar-owned node.
- Target arguments are known metadata.
- Scalar field value means exact match.
- Array datum field value means any-of for that field.
- Record fields are ANDed together.
- Array datum of record predicates is OR across record predicates.
- Nonmatching targets make the node absent before semantic checks for that compilation target.
- `@target` is unique on a node; compose with datums instead of repeating it.
- No runtime branching is introduced.
- No `cfg` keyword, string DSL, `any`/`all` keyword, or special repeated-attribute OR rule exists.

```musi
@target(os := #[.linux, .macos], arch := .x64)
@target(#[#{ os := .linux, arch := .x64 }, #{ os := .macos, arch := .aarch64 }])
```

Representation controls are schema-validated attributes only: `@repr(.c)`, `@packed`, `@align(4)`. Representation metadata args must be known. `@repr(profile, ...)` names layout/profile family. Profile schemas validate allowed targets/fields/values/combinations. Unsupported profile/field/value/combination diagnostic. Representation attributes apply only where schema allows: data definitions, fields, variants/cases, extern bindings. FFI boundary types must be representable under chosen profile. SEIL preserves layout and near-identical decompilation metadata.

Profile fields may include `tag`, `endian`, `padding`, `bits`, `layout`. Tag/profile values use Musi-native size spelling: `.nX` natural/unsigned-sized, `.iX` signed integer-sized, `.fX` floating-sized. Rule: type identity/storage/checking concept → type-space modifier; representation/ABI/interop → attribute.

## 14. Shapes and conformance

```ebnf
shape-expr   ::= "shape" shape-body
shape-body   ::= "{" shape-member (";" shape-member)* ";"? "}"
shape-member ::= "let" IDENT param-list? type-annot
```

`shape` is locked spelling for observable structure/capability contracts. A value/type fits a shape when it provides required members/operations under Musi conformance rules. `trait` is not core. `data` defines what a thing is; `shape` defines what it must look like.

Default shape conformance is structural: compatible required observable members/operations and stack effects; no declaration required. `@witness shape` requires explicit witness for semantic/lawful/marker/capability contracts where members alone are insufficient. Empty marker shapes must use `@witness shape` to avoid every type fitting accidentally.

```ebnf
conformance-relation ::= TYPE "|=" TYPE
witness-binding      ::= "let" TYPE "|=" TYPE ":=" record-datum
```

`T |= Shape` states/constrains fit. `let T |= Shape := witnessValue;` binds explicit witness for witness-required conformance. No `impl`, `implements`, `extends`, or `trait`. Receiver methods and witness bindings use `let`. `|=` is not a runtime Boolean predicate. Runtime fit checks use `:?` / `:?>` against concrete type or shape boundaries when runtime evidence exists. `Any` requires explicit capability/API for dynamic checks. `opaque` does not grant arbitrary runtime introspection.

## 15. Modules, imports, exports, visibility

```ebnf
import-expr   ::= "import" import-source
import-source ::= STRING | record-datum | tuple-datum
export-expr   ::= "export" let-expr | "export" export-block
export-block  ::= "{" export-item (";" export-item)* ";"? "}"
export-item   ::= let-expr
```

Rules: `import` expression takes in module/resource/package. `known import` compile-time import/acquisition. Import may use datums for multiple inputs. `export` marks a `let` binding for module surface; exported receiver methods are still `let` bindings. Standalone `match`, `while`, arbitrary expressions are not export targets. `export { ... }` is structural block sugar over separate `export let ...;` forms. Modules top-to-bottom strict; export block items processed top-to-bottom. Module boundary forms affect source shape and SEIL/decompilation metadata.

Modules are records; imports bring in records:

```ebnf
module-value      ::= record-datum | named-record-value
named-import-bind ::= "let" IDENT ":=" import-expr
anonymous-import  ::= "let" "_" ":=" import-expr
```

Named import binds imported record to a name. Anonymous import brings imported record contents into scope without binding record itself. Multi-import datums produce record-shaped imports. Native/compiler modules use `musi:` import prefixes, similar in role to `node:`/`bun:` prefixes: `musi:core`, `musi:ffi`, etc. These modules are native/compiler-provided and may expose `.ms` interface surfaces; their internals are not required to be written in Musi.

Visibility: `export` only. Exported binding visible from module; non-export private by absence. No `public`, `private`, `protected`, `internal`, `hidden`. `opaque` controls type abstraction, not visibility. Modules are records; exports define module record surface.

SEIL round-trip metadata preserves import binding mode (named/anonymous), import source shape (string/tuple datum/record datum), known/runtime phase, exported binding names, and optionally export-block grouping. If grouping absent, decompiler may emit canonical separate `export let` forms preserving semantics.

## 16. Open-question checklist

Checked/locked:

- [x] Keyword set: hard keyword list; visibility words not separate; `import`; `export`; `hidden` removed; `erased`; `fixed`.
- [x] Shape/conformance: `shape`; structural conformance; witness conformance; `|=`; erased shape value status.
- [x] Type system: bidirectional gradual model; type algebra `| & ^ ~`; union/intersection representation/normalization; optional/error surface; callable source syntax; universal `:` annotations; cast/test operators.
- [x] Stack effect: source syntax; first-class stack-effect decision; ordinary function callable exposure; compatibility for `when`, `match`, `defer`, `yield`, receiver methods; guarded emission effect model.
- [x] Data: product field grammar; sum variant grammar; `case Variant(...) := value`; no product/sum mixing; associated data/value binding; constructors; destructuring/pattern syntax; tags/discriminants.
- [x] Representation/metadata: attributes; `@packed`; alignment/endian/tags/padding/ABI layout controls; metadata placement; SEIL preservation.
- [x] Comments: line/doc/module doc/block/block doc/block module doc/nesting.
- [x] Delimiters/separators/literals: `#(`, `#{`, `#[`; tuple types; bracket roles; trailing separators; empty tuple/record/array; numeric suffixes/separators/base prefixes; triple-quoted multiline strings; escaped identifiers.
- [x] Control flow: `when ... else` precedence/associativity; dangling-else prevention; nested `when` parenthesization; guarded emission contexts; loop syntax vs recursion; `defer`/`yield`/`pin` status.
- [x] Match/patterns: exact pattern grammar; alternatives; comma alternatives not `|`; semicolon cases; exhaustiveness; guard order; binding syntax.
- [x] Operators: full symbolic set; precedence; not-flat parsing; no user-defined symbolic ops; word ops; assignment/binding/update vs equality; equality/equivalence/ordering/approximation/membership/remainder.
- [x] Modules/imports: modules as record-like values; import syntax; export syntax; visibility; path/source shape; SEIL round-trip.
- [x] Known phase: meaning; applies to expressions/bindings/parameters/types where meaningful; limits; boundary; datum literals; functions compile to SEIL/no separate interpreter.
- [x] Safety: no unsafe wrapper; capabilities/metadata/types; pointer types/ops; pinning via `fixed`; FFI rules; dangerous behavior errors not warnings.
- [x] Lexical literals: numeric separators; base prefixes; literal suffixes; triple-quoted strings; escaped identifiers; reserved interpolation direction; no automatic multiline-string indentation trimming.
- [x] Attributes: universal attribute call model; UALO payloads; `@target`; tooling namespace rule; compiler-affecting unknown attribute diagnostics.
- [x] Native modules: `musi:` import prefix; native/compiler-provided modules; optional `.ms` interface surfaces.

Still open:

- [ ] SEIL instruction model
- [ ] SEIL metadata required for near-identical decompilation
- [ ] Source-to-SEIL lowering guarantees
- [ ] Whether SEIL has stable binary and textual form
- [ ] How stack-effect verification appears in SEIL
- [ ] How known-phase evaluation appears in SEIL
