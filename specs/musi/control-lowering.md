# Musi control-flow lowering

Musi control lowers to SEIL stack-effect bodies, branch metadata, cleanup-region metadata, and suspension metadata.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` control rules, `grammar/musi.ebnf`, `seil_opcodes.def`.

Reference: WebAssembly type-checks control edges for stack-machine validation: <https://webassembly.github.io/spec/core/valid/instructions.html>. SEIL uses separate block + metadata tables.

## Conditional expressions

Postfix `when` needs `Bit`. `a when cond else b` lowers to verified SEIL control; both result paths must match. Musi never invents hidden `Maybe`, `Unit`, bottom, or union to accept mismatched branches.

Guarded expression without `else` valid only where guarded emission is admitted.

## Loops

`while expr { ... }` lowers to explicit SEIL blocks + branches. Condition must be `Bit`. `leave` and `cycle` lower to branch/region edges preserving verified stack shape.

## Defer and cleanup

`defer expr` registers cleanup in current region. Cleanup runs on normal exit, `leave`, and `cycle` by region metadata + cleanup instructions. Guarded defer condition checked at registration.

Cleanup regions lower to `cln.push`, `cln.pop`, `cln.run`, `leave`, and region metadata.

## Yield and suspension

`yield expr?` lowers to `yld` plus yield/resume signature metadata. Yield suspends/resumes; not function call. Defers do not run at suspension. Pending defers run at final close/drop/cancel by runtime rules.

## Match

`match` lowering enforces exhaustiveness unless context admits non-emission. Pattern guards use `when` and verify as `Bit`. Tagged sums lower through tag/payload ops plus branch metadata.

## Failure cases

Compiler rejects non-`Bit` conditions, incompatible branch results, invalid `leave`/`cycle` targets, invalid guarded-emission contexts, non-exhaustive matches where not admitted, and suspension lacking compatible yield/resume metadata.

## Unknowns

- Exact SEIL block layout pattern per control form not locked.
- Exact generator object representation not specified.
- Exact cleanup order among nested regions needs runtime rule table.
