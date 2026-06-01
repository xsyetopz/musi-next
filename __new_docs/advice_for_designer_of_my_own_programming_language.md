## Two goals

- Make it predictable
  - Make it deterministic
  - Make it easy to reason about (for humans and computers)
  - Avoid global state at all phases
  - Use static types
- Make it efficient

To that end

There are things almost every language does for good reasons, and other things copied from outdated tutorials or papers. Below are concrete recommendations.

## Lexer / Parser

- Keep the lexer and parser separate. At most, use lexer states.
- Avoid surprises: follow familiar language conventions where possible.
- Use [] for generics, not <>.
- Treat comments as part of the grammar (with reasonable placement restrictions) to ease refactoring tools.
- Allow lexing from any random point in a file (this constrains multi-line string design).
- Consider avoiding floating-point literals in the lexer; use a compile-time function like `float("1.0")` if possible.
- Ensure linear efficiency for lexer and parser (avoid exponential behaviors).
  - In the lexer, match keywords as identifiers and perform a lookup before parsing.
  - This allows versioned keywords and makes identifier-based compiled code possible.
- Prefer DFA/NFA-based regex engines; avoid Perl-style regexes.
- Avoid parser tools/approaches mentioning: parser combinators, LL(*), memoization, packrat, PEG, GLL, GLR.
- Prefer: LALR(1), IELR(1), CLR(1), LR(1), recursive ascent. Conditionally good: LL(1) (with automation), recursive descent (if finite lookahead is verified).
- Efficient but often not useful: LR(0), SLR(1), SLL(1).
- Inspect and override tool defaults where sensible.
  - `bison --xml` can help build runtimes while bison builds the LR machine.

## Lowering

- To handle arbitrary function calls, either require forward declarations or perform two passes over the input.
- Never implement a tree-evaluator; they are slow.
- If you support constexpr evaluation, provide a bytecode interpreter even when generating machine code.
- Avoid offloading compiler responsibilities to the standard library when they belong in the compiler.
- Understand lvalues vs rvalues; treat them as separate lowering strategies on expression AST nodes.
- Make the driver able to produce multiple outputs (as toggles):
  - nothing (syntax/semantic checks)
  - pretty-printed source
  - semantic AST dump (pre-lowering)
  - high-level IR (text and binary)
  - low-level IR (text and binary)
  - machine code (text and binary)
  - executable or shared library
  - run the produced executable immediately
- Ensure inputs and compilation stages can be parallelized and are race-safe.

## Runtime

- Consider stack vs register machine trade-offs (Java's bytecode is illustrative).
- Avoid GC when possible; prefer explicit ownership (e.g. `weak`, `unique`) and consider a cycle detector at exit.
- Avoid pointer-chasing data structures where possible.
- Think through shared library paths, TLS models, and relocations even for interpreted languages.
- Dynamically-scoped variables can be implemented via TLS: use push/save semantics instead of global set.

--
Notes: this document is a concise checklist of design recommendations for language implementers.
