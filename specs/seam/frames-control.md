# SEAM frames, calls, returns, branches, and suspension

SEAM executes verified SEIL bodies using frames and typed evaluation stacks. This runtime control model applies after SEIL verification accepts a module.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` opcode semantics and control-flow decisions, `seil_opcodes.def`, `grammar/musi.ebnf`.

References: WebAsm defines runtime frames, labels, function calls, and traps: <https://webassembly.github.io/spec/core/exec/runtime.html>. The JVM defines frames with local variables, an operand stack, and return behavior: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6>.

## Frame structure

A SEAM call frame contains:

- current module instance;
- current procedure/body declaration;
- instruction cursor or block cursor;
- typed evaluation stack;
- argument slots;
- local slots;
- environment/capture slots;
- active cleanup-region stack;
- active handler/exception state;
- active suspension state if the frame can yield.

The verifier determines maximum stack depth and frame-storage requirements. Runtime frames allocate storage from verified metadata, not authored limits.

## Calls

`call` creates a frame for a statically referenced declaration. `call.disp` resolves receiver-aware dispatch through declaration metadata. `call.ind` invokes a callable value whose signature matches `sig_idx`. `call.dyn` invokes through an explicit dynamic-call protocol.

Call arguments are consumed from the caller stack in signature order and become callee argument slots. Call results are pushed onto the caller stack in signature output order after the callee returns.

## Returns and halt

`ret` ends the current frame. If the frame has a caller, results are transferred to the caller. If the frame is the entry frame, `ret` halts the invocation successfully with the entry outputs.

`trap`, unhandled `throw`, verification-impossible runtime violations, limit exhaustion, and failed required capability checks halt the current invocation unsuccessfully through the structured failure channel.

## Branches

`br` transfers to a block target. `br.true` and `br.false` consume `Bit` and either branch or continue with fallthrough. `br.tbl` consumes a natural selector and transfers through branch-table metadata. Targets are already type-checked by SEIL verification; runtime dispatch chooses the target and moves the instruction cursor.

## Exceptions and cleanup

`throw` starts an exceptional control edge. `rethrow` continues the active exceptional edge. Handler matching and cleanup ordering are encoded in body metadata. `leave` exits a region and triggers the cleanup behavior required by that region.

Cleanup operations manipulate the active cleanup-region stack:

- `cln.push` registers a cleanup region;
- `cln.pop` removes a cleanup region without running it when metadata admits that edge;
- `cln.run` runs a cleanup region.

## Suspension

`yld` suspends the current invocation according to yield/resume metadata. Suspension captures enough frame state to resume at the correct continuation point with the required resume values.

Defers/cleanups do not run merely because a frame suspends. Pending cleanups run when the suspended computation is finally closed, dropped, cancelled, or exits normally according to runtime metadata.

## Failure cases

Runtime control failure occurs for unhandled exceptions, invalid dynamic call resolution, failed capability requirements, trap execution, limit exhaustion, and invalid memory/control states. Such failures halt the invocation unsuccessfully and preserve diagnostic context.

## Unknowns

- Exact in-memory frame layout is not specified.
- Exact handler matching table format is not specified.
- Exact cancellation API for suspended computations is not specified.
