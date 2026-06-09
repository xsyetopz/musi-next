# SEAM frames, calls, returns, branches, and suspension

SEAM executes verified SEAM bytecode bodies with frames and typed evaluation stacks. Applies after verification accepts module.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` opcode/control rules, `seam_bytecode_opcodes.def`, `grammar/musi.ebnf`.

References: WebAssembly defines frames, labels, calls, traps: <https://webassembly.github.io/spec/core/exec/runtime.html>. JVM defines frames with locals, operand stack, returns: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6>.

## Frame structure

SEAM frame contains:

- current module instance;
- procedure/body declaration;
- instruction/block cursor;
- typed evaluation stack;
- argument slots;
- local slots;
- environment/capture slots;
- active cleanup-region stack;
- active handler/exception state;
- active suspension state if frame can yield.

Verifier computes max stack depth and frame storage. Runtime allocates from verified metadata, not authored limits.

## Calls

`call` creates frame for static declaration. `call.disp` resolves receiver dispatch via declaration metadata. `call.ind` invokes callable value matching `sig_idx`. `call.dyn` invokes explicit dynamic-call protocol.

Arguments leave caller stack in signature order and become callee argument slots. Results return to caller stack in signature output order.

## Returns and halt

`ret` ends current frame. With caller: transfer results. Entry frame: successful halt with entry outputs.

`trap`, runtime violations impossible to verify statically, limit exhaustion, unhandled failures, and failed required capabilities halt unsuccessfully through structured failure channel.

## Branches

`br` jumps to block target. `br.true`/`br.false` consume `Bit`, then branch or fall through. `br.tbl` consumes natural selector and uses branch-table metadata. Verification already type-checks targets; runtime only selects target and cursor.

## Exceptions and cleanup

`throw` starts exceptional edge. `rethrow` continues active exceptional edge. Handler match lives in body metadata by protected region + exit/failure reason, not raw instruction ranges. `leave` exits region and triggers required cleanup.

Cleanup order is lexical LIFO for normal return, `leave`, `cycle`, cancellation, and close. Trap/abort cleanup remains separately specified.

Cleanup ops manage active cleanup-region stack:

- `cln.push` registers cleanup region;
- `cln.pop` removes cleanup region without running when metadata admits edge;
- `cln.run` runs cleanup region.

## Suspension

`yld` suspends invocation by yield/resume metadata. Suspension captures enough frame state to resume at correct continuation with required resume values. Hosts see an opaque resumable handle, not raw frames.

Opaque resumable handles support resume, cancel, close/drop, status, and outcome. Cancellation is cooperative at safepoints/yield points first; forced close exists only for teardown.

Defers/cleanups do not run merely because frame suspends. Pending cleanups run when suspended computation closes, drops, cancels, or exits normally by runtime metadata. Cancellation runs pending defers before `cancelled` unless cleanup traps/fails.

## Failure cases

Runtime control failure: unhandled failure/throw, invalid dynamic call resolution, failed capability, trap, limit exhaustion, invalid memory/control state. Failure halts invocation unsuccessfully and preserves diagnostic context.

## Unknowns

- Exact in-memory frame layout not specified.
- Exact binary handler matching table format not specified.
- Exact host embedding function signatures for resumable handles not specified.
