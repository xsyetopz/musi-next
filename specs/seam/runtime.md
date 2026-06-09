# SEAM runtime and execution

SEAM = Stack Effect Abstract Machine. Loads, verifies, links, executes SEIL modules. Not Musi syntax, not SEIL text syntax, not binary image format.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, known-phase notes, opcode semantics, `docs/language_checklist_for_musi.md`.

References: WebAsm separates store/module instances/frames/traps from validation: <https://webassembly.github.io/spec/core/exec/index.html>. JVM defines runtime frames/heap areas: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.5>.

## Responsibilities

SEAM owns:

- load `.seil` modules;
- decode/validate section + table data;
- verify SEIL bodies;
- resolve imports, exports, native bindings, foreign bindings;
- construct module instances + runtime frames;
- execute stack-effect instructions;
- enforce deterministic known-phase limits;
- manage memory, refs, capabilities, dynamic protocols, suspension, cleanup, failures.

## Module lifecycle

1. Load `.seil` text or assembled SEAM binary image.
2. Validate header, section directory, required sections.
3. Decode logical tables.
4. Verify opcode schemas, operands, stack effects, metadata refs, control edges.
5. Link imports/exports/native/foreign declarations under active target, capability, ABI metadata.
6. Init module globals + required runtime structures.
7. Execute entry points or callable exports.

Load/verify/link failure means no execution.

## Execution state

State includes module instances, callable frames, stacks, locals, args, env/captures, globals, heap/runtime storage, capability evidence, cleanup regions, exception state, suspension state, fuel/step counters, memory limits.

Calls create frames from verified metadata. Returns must match signature outputs.

## Known-phase execution

Musi known execution runs verified SEIL under SEAM, not source-tree evaluator. Deterministic limits. No ambient time/random/process/env/filesystem/network/IO unless explicit deterministic known import or declared `musi:rt` intrinsic supplies it.

Fuel, step, recursion, and memory limits are enforcement. Limit exhaustion = known-execution failure, not fallback to runtime.

## Imports, exports, native, foreign calls

Import/export compatibility checks signature, type, layout, target, capability, ABI metadata. Foreign/native calls mediated by declarations + ABI metadata. Dangerous/unrepresentable ABI behavior rejected by compiler/runtime validation, not warning.

SEAM has one core C-compatible FFI bridge. SEIL calls native via import metadata. Native calls SEIL via exported callable tables. Managed values cross as handles unless core ABI metadata marks represented, fixed, or copied. Native callbacks enter via SEAM trampolines so frames, roots, safepoints, and failures stay valid.

## Control edges

SEAM executes verified branch, return, exception, cleanup, leave, yield edges using body metadata. Cleanup regions run by explicit cleanup instructions + region metadata. Suspension records yield/resume state by signature metadata.

## Failure channels

Runtime failures: traps, rejected casts, checked conversions, failed capabilities, invalid dynamic protocol, memory violations, bounds, nil misuse where disallowed, unhandled exceptions, failed linkage, deterministic-limit exhaustion.

Failures stay structured runtime/diagnostic events. SEAM never turns failure into host UB.

## Unknowns

- Exact frame object layout not specified.
- Exact module initialization order after load/verify/link/init not fully specified.
- Exact host embedding API not specified.
