# SEAM runtime and execution

SEAM is the Stack Effect Abstract Machine: the VM/runtime that loads, verifies, links, and executes SEIL modules. SEAM is not Musi source syntax, SEIL text syntax, or the SEAM binary image format itself.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, known-phase notes, opcode semantics, `docs/language_checklist_for_musi.md`.

References: WebAsm execution separates store, module instances, frames, and traps from validation: <https://webassembly.github.io/spec/core/exec/index.html>. The JVM specifies runtime data areas for frames and heap storage: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.5>.

## Responsibilities

SEAM owns:

- loading `.seil` modules;
- decoding and validating section/table data;
- verifying SEIL bodies;
- resolving imports, exports, native bindings, and foreign bindings;
- constructing module instances and runtime frames;
- executing stack-effect instructions;
- enforcing deterministic known-phase limits;
- managing memory, references, capabilities, dynamic protocols, suspension, cleanup, and failure channels.

## Module lifecycle

1. Load `.seil` text or assembled SEAM binary image data.
2. Validate header, section directory, and required sections.
3. Decode logical tables.
4. Verify opcode schemas, operands, stack effects, metadata references, and control-flow edges.
5. Link imports/exports/native/foreign declarations under active target, capability, and ABI metadata.
6. Initialize module globals and required runtime structures.
7. Execute entry points or callable exports.

A module that fails loading, verification, or linking does not execute.

## Execution state

SEAM execution state includes loaded module instances, callable frames, evaluation stacks, locals, arguments, environment/capture slots, global slots, heap/runtime storage, capability evidence, cleanup-region state, exception state, suspension state, fuel/step counters, and memory-limit state.

Function calls create frames whose argument/local/environment layout is determined by verified SEIL metadata. Returns must match signature outputs.

## Known-phase execution

Musi known execution runs verified SEIL under SEAM, not a separate source-tree evaluator. Known execution has deterministic limits and no ambient access to time, random, process, environment, filesystem, networking, or IO unless a deterministic known import/intrinsic explicitly supplies it.

Fuel, step, recursion, and memory limits are enforcement mechanisms. Limit exhaustion is a known-execution failure, not silent fallback to runtime execution.

## Imports, exports, native, and foreign calls

Import/export compatibility is checked against signature, type, layout, target, capability, and ABI metadata. Foreign/native calls are mediated by declarations and ABI metadata. Dangerous or unrepresentable ABI behavior is rejected by compiler/runtime validation rather than downgraded to warnings.

SEAM exposes one core C-compatible FFI bridge. SEIL calls native code through import metadata. Native code calls SEIL through exported callable tables. Managed values cross this boundary as handles unless core ABI metadata marks them represented, fixed, or copied. Native callbacks enter through SEAM trampolines so frames, roots, safepoints, and failures remain valid.

## Control edges

SEAM executes verified branch, return, exception, cleanup, leave, and yield edges using body metadata tables. Cleanup regions run according to explicit cleanup instructions and region metadata. Suspension records yield/resume state according to signature metadata.

## Failure channels

Runtime failure may arise from traps, rejected casts, checked conversions, failed capability requirements, invalid dynamic protocol use, memory violations, out-of-bounds access, null/nil misuse where disallowed, unhandled exceptions, failed linkage, and deterministic-limit exhaustion.

Failures remain structured runtime/diagnostic events. SEAM never converts failure into unspecified host behavior.

## Unknowns

- Exact frame object layout is not specified.
- Exact module initialization ordering beyond load/verify/link/init is not fully specified.
- Exact host embedding API is not specified.
