# 22. Staging, known, Runtime Syntax Values, and Splice

Status: normative semantic addendum for staged metaprogramming.

## Core rule

```text
~ quotes syntax.
known changes evaluation phase.
splice inserts known syntax into the program being compiled.
```

`~` by itself is not compile-time.

## Quote forms

The quote family is:

```musi
~(expr)
~type(type)
~pat(pattern)
~decl(declaration)
~decls(declarations)
```

Meaning:

```text
~(...)       expression syntax
~type(...)   type syntax
~pat(...)    pattern syntax
~decl(...)   declaration syntax
~decls(...)  declaration-set syntax
```

Typed syntax categories:

```text
Syntax[Expr]
Syntax[Type]
Syntax[Pattern]
Syntax[Decl]
Syntax[Decls]
```

The exact internal representation of syntax values belongs to the compiler/runtime service, not to source syntax.

## Runtime syntax values

Syntax values may exist at runtime.

```musi
let e := ~(x + y);
```

This creates a runtime syntax value if the current runtime provides syntax/compiler support.

```musi
let e := known ~(x + y);
```

This creates a compiler-known syntax value.

## Splicing inside a quote

Inside a quoted syntax template, `~x` splices syntax value `x` into the quoted syntax.

```musi
let rhs := known ~(y * 2n32);
let expr := known ~(x + ~rhs);
```

`rhs` must have a syntax category compatible with the splice site.

## Source-position insertion

Outside a quote, use `splice` for explicit insertion into the surrounding source position.

```musi
splice known deriveShow[EnemySpec]();
```

`splice` requires known syntax because it changes the program being compiled.

Invalid unless `makeDecl()` is already known by context:

```musi
splice makeDecl();
```

Preferred explicit form:

```musi
splice known makeDecl();
```

## Syntax vs datum

Datums use `#`.

```musi
let data := #{name := "Imp", health := 10n32};
```

Syntax uses `~`.

```musi
let code := ~(enemy.health <= 0n32);
```

`#` values are data-as-data. `~` values are code-as-code.

## Template literals

`$` belongs to template literal interpolation and is unavailable for staged metaprogramming.

## Parser contract

Quote/splice does not extend the parser.

Disallowed:

```text
reader macros
token-stream macros
arbitrary grammar extension
source generation by raw string concatenation as core metaprogramming
using type/name resolution to parse quote syntax
```

Allowed:

```text
typed syntax values
known-time generation
runtime syntax values when compiler/runtime support exists
explicit source-position splice of known syntax
```

## Embedded support boundary

A runtime that does not provide syntax/compiler support may reject runtime syntax construction.

A normal compiler must still support known syntax values required for compilation.

The absence of runtime syntax support does not change the meaning of `known` or `splice`.
