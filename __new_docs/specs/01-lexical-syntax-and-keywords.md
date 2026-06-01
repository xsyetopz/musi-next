# 01. Lexical Syntax and Keywords

Status: normative at keyword/policy level.

## Reserved keywords

The following words are reserved by the source language:

```text
and
as
data
defer
else
erased
exit
export
fixed
import
in
known
let
match
mut
next
not
opaque
or
pin
trait
unsafe
when
where
while
with
xor
yield
```

A reserved keyword is not an identifier.

## Words that are not keywords

The following are not Musi keywords:

```text
if
then
return
null
for
fun
fn
def
module
class
impl
instance
async
await
spawn
try
catch
foreign
static
pinned
true
false
```

`true` and `false` are ordinary values of type `Bit`, not keyword literals.

`static` and `pinned` are intentionally not Musi keywords. Use `fixed` for fixed storage/placement/lifetime. Use `pin` for temporary pinning action.

## Comments

Line comments use `--`.

```musi
-- this is a comment
let x := 1n32;
```

## Identifiers

Identifier spelling is implementation-lexical, but reserved keywords are excluded. Dotted variant forms such as `.Some` are not bare identifiers.

## Consequence words

The following keywords are consequence words and must remain visually meaningful:

```text
known
fixed
mut
opaque
erased
unsafe
pin
```

They are not decorative annotations.

## Sigil ownership

```text
#   datum literal/pattern family
$   template literal interpolation
~   syntax quote/template splice family
... spread/splat/rest
```

`#` is not used for attributes or syntax quoting. `$` is not used for staged metaprogramming. `~` is not datum syntax.

## Maximal-munch lexing and parser contract

Lexing uses maximal munch.

Tokenization is context-free: the parser does not direct the lexer, and name/type resolution does not affect token boundaries.

Musi syntax is constrained to LR(1) / LL(1)-compatible parsing discipline. If a source form requires more than one parser lookahead token, parser backtracking, semantic predicates for syntactic decisions, or name/type resolution to choose a grammar alternative, that source form is invalid for Musi syntax.

Grammar artifacts may be published for tooling, including ANTLR4 consumers, but parser-generator acceptance is not the language authority. The language authority is this source contract plus the specified grammar and conflict policy.
