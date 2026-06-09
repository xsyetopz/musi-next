# Musi control-flow lowering

Musi control lowers to SEAM bytecode through protected region/edge metadata, stack-effect bodies, branch metadata, cleanup-region metadata, and suspension metadata. Type and edge compatibility use the generated declarative compatibility relation.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` control rules, `grammar/musi.ebnf`, `seam_bytecode_opcodes.def`.

Reference: WebAssembly type-checks control edges for stack-machine validation: <https://webassembly.github.io/spec/core/valid/instructions.html>. SEAM bytecode uses separate block + region/edge metadata tables.

## Conditional expressions

Postfix `when` needs `Bit`. `a when cond else b` lowers to verified SEAM bytecode region/edge control; both result paths must match. Musi never invents hidden `Maybe`, `Unit`, bottom, or union to accept mismatched branches.

Guarded expression without `else` valid only where guarded emission is admitted.

## Loops

`while expr { ... }` lowers to explicit SEAM bytecode blocks, protected regions, edge reasons, and branches. Condition must be `Bit`. `leave` and `cycle` lower to branch/region edges preserving verified stack shape.

## Defer and cleanup

`defer expr` registers cleanup in current region. Cleanup runs on normal exit, `leave`, `cycle`, cancellation, and close by region metadata + cleanup instructions. Guarded defer condition checked at registration. Nested cleanup order is lexical LIFO for normal return, `leave`, `cycle`, cancellation, and close; trap/abort cleanup remains separately specified.

Cleanup regions lower to `cln.push`, `cln.pop`, `cln.run`, `leave`, protected-region ids, edge reasons, and region metadata.

## Yield and suspension

`yield expr?` lowers to `yld` plus yield/resume signature metadata and suspension region edge. Yield suspends/resumes; not function call. Hosts receive opaque resumable handles with resume, cancel, close/drop, status, and outcome. Defers do not run at suspension. Pending defers run at final close/drop/cancel by runtime rules.

## Match

`match` lowering enforces exhaustiveness unless context admits non-emission. Pattern guards use `when` and verify as `Bit`. Tagged sums lower through tag/payload ops plus branch metadata.

## Failure cases

Compiler rejects non-`Bit` conditions, incompatible branch results, invalid `leave`/`cycle` targets, invalid guarded-emission contexts, non-exhaustive matches where not admitted, and suspension lacking compatible yield/resume metadata.

## Detail gaps

- Exact lowering recipe contents and pass/fail fixtures per control form not specified.
- Exact generator/resumable internal object representation not specified.
