## Two goals

- Predictable
  - deterministic
  - easy for humans + computers to reason about
  - no global state by default
  - static types
- Efficient

Rules below support those goals.

## Lexer / Parser

- Keep lexer and parser separate. Lexer states OK.
- Avoid surprise. Use familiar conventions only when behavior matches.
- Use `[]` for generics, not `<>`.
- Treat comments as grammar/trivia with placement rules. Refactoring tools need them.
- Allow lexing from random file points where possible; this constrains multi-line strings.
- Consider moving float parsing out of lexer into compile-time API like `float("1.0")`.
- Keep lexer/parser linear. No exponential behavior.
  - Lex keywords as identifiers, then keyword-lookup before parse.
  - This permits versioned keywords and identifier-based compiled code.
- Prefer DFA/NFA regex engines. Avoid Perl-style regexes.
- Avoid parser tools/approaches that rely on parser combinators, LL(*), memoization, packrat, PEG, GLL, or GLR.
- Prefer LALR(1), IELR(1), CLR(1), LR(1), recursive ascent.
- LL(1) and recursive descent OK only when finite lookahead is verified.
- LR(0), SLR(1), and SLL(1) are efficient but often too weak.
- Inspect tool defaults. Override when needed.
  - `bison --xml` can expose LR machine data while bison builds parser.

## Lowering

- For arbitrary calls, require forward declarations or use two-pass input analysis.
- Never use source-tree evaluator for execution. Slow, divergent from runtime.
- If compile-time evaluation exists, execute same verified VM form runtime loads.
- Do not push compiler duties into stdlib when compiler must know rule.
- Model lvalues and rvalues separately in lowering.
- Driver should support output toggles:
  - no output, checks only
  - pretty source
  - semantic AST dump
  - high-level IR text/binary
  - low-level IR text/binary
  - machine code text/binary
  - executable/shared library
  - run produced executable
- Compilation stages and inputs should be parallel-safe.

## Runtime

- Choose stack vs register machine deliberately; this repo uses SEAM bytecode/SEAM as source of truth.
- Avoid GC pressure where possible; consider explicit ownership such as `weak` and `unique`, plus exit-time cycle detection.
- Avoid pointer-chasing data structures where possible.
- Think through shared library paths, TLS models, and relocations even for interpreted languages.
- Dynamic-scope vars can use TLS push/save semantics, not global set.

--
Notes: concise checklist for language implementers.
