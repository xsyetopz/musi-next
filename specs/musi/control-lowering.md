# Musi control-flow lowering

Musi control forms lower into SEIL stack-effect bodies, branch metadata, cleanup-region metadata, and suspension metadata.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` control-flow decisions, `grammar/musi.ebnf`, `seil_opcodes.def`.

References: WebAssembly specifies type-checked control edges for stack-machine validation: <https://webassembly.github.io/spec/core/valid/instructions.html>. SEIL uses separate block and metadata tables.

## Conditional expressions

Musi postfix `when` requires a `Bit` condition. `a when cond else b` lowers to SEIL control that verifies both result paths as compatible. Musi does not introduce hidden `Maybe`, `Unit`, bottom, or union result types to accept mismatched branches.

Guarded expressions without `else` are valid only in contexts that admit guarded emission.

## Loops

`while expr { ... }` lowers to explicit SEIL blocks and branches. Loop conditions must be `Bit`. `leave` and `cycle` lower to branch/region edges that preserve verified stack shape.

## Defer and cleanup

`defer expr` registers cleanup in the current region. Cleanup runs on normal exit, `leave`, and `cycle` according to region metadata and cleanup instructions. Guarded defer conditions are checked when the cleanup is registered.

Cleanup regions lower to `cln.push`, `cln.pop`, `cln.run`, `leave`, and region metadata.

## Yield and suspension

`yield expr?` lowers to `yld` with yield/resume signature metadata. Yield suspends and resumes; it does not call a function. Defers do not run at suspension. Pending defers run at final close/drop/cancel according to runtime rules.

## Match

`match` lowering must enforce exhaustiveness unless the context admits non-emission. Pattern guards use `when` and must verify as `Bit`. Tagged sums lower through tag and payload operations plus branch metadata.

## Failure cases

The compiler rejects non-`Bit` conditions, incompatible branch results, invalid `leave`/`cycle` targets, invalid guarded-emission contexts, non-exhaustive matches where not admitted, and suspension without compatible yield/resume metadata.

## Unknowns

- Exact SEIL block layout patterns for each control form are not locked.
- Exact generator object representation is not specified.
- Exact cleanup ordering among multiple nested regions needs a dedicated runtime rule table.
