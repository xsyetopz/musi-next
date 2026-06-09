# SEAM runtime and execution

SEAM = Stack Effect Abstract Machine. Loads, verifies, links, initializes, and executes `.seam` bytecode images. Not Musi syntax and not SEAM bytecode text/disassembly syntax.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, known-phase notes, opcode semantics, `docs/language_checklist_for_musi.md`.

References: WebAsm separates store/module instances/frames/traps from validation: <https://webassembly.github.io/spec/core/exec/index.html>. JVM defines runtime frames/heap areas: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.5>.

## Responsibilities

SEAM owns:

- load `.seam` bytecode images;
- decode/validate section + table data;
- verify SEAM bytecode bodies;
- resolve imports, exports, native bindings, foreign bindings;
- construct module instances + runtime frames;
- execute stack-effect instructions;
- enforce deterministic known-phase limits;
- manage memory, refs, typed capability/resource graph, dynamic protocols, suspension, cleanup, failures.

## Module lifecycle

1. Resolve package graph and host-provided module nodes.
2. Load `.seam` bytecode images.
3. Validate header, section directory, required sections.
4. Decode `asm` and `deps`, then logical tables.
5. Verify opcode schemas, operands, stack effects, metadata refs, control edges.
6. Link imports/exports/native/foreign declarations under active target, capability, ABI metadata.
7. Reject dependency/init cycles with load/link diagnostic.
8. Initialize dependencies before dependents; manifest declaration order breaks otherwise equal ties.
9. Init module globals + required runtime structures.
10. Execute entry points or callable exports.

Load/verify/link failure means no execution.

## Execution state

State includes module instances, callable frames, stacks, locals, args, env/captures, globals, heap/runtime storage, capability evidence, cleanup regions, exception state, suspension state, fuel/step counters, memory limits.

Calls create frames from verified metadata. Returns must match signature outputs.

## Known-phase execution

Musi known execution runs verified SEAM bytecode under SEAM, not source-tree evaluator. Deterministic limits. No ambient time/random/process/env/filesystem/network/IO unless explicit deterministic known import or declared `musi:rt` intrinsic supplies it. Optional known imports require deterministic known-capable provider metadata.

Fuel, step, recursion, and memory limits are enforcement. Limit exhaustion = known-execution failure, not fallback to runtime.

## Imports, exports, native, foreign calls

Import/export compatibility checks signature, type, layout, target, capability, ABI metadata. Foreign/native calls mediated by declarations + ABI metadata. Dangerous/unrepresentable ABI behavior rejected by compiler/runtime validation, not warning.

SEAM has one host-ABI-capable FFI bridge. It includes C ABI plus handles, callbacks through exported callable handles, resources, async/yield/resumable interaction, cancellation, failure outcomes, and representable memory access metadata. SEAM bytecode calls native via import metadata. Native calls SEAM bytecode via exported callable tables or host embedding handles. Managed values cross as opaque handles unless core ABI metadata marks represented, fixed, or copied. Native callbacks enter via SEAM trampolines so frames, roots, safepoints, and failures stay valid. Native calls are failure-capable unless metadata proves otherwise; host exceptions do not cross the SEAM boundary.

## Control edges

SEAM executes verified branch, return, exception, cleanup, leave, yield edges using body metadata. Cleanup regions run by explicit cleanup instructions + region metadata. Suspension records yield/resume state by signature metadata and exposes only opaque resumable handles to hosts.

## Failure channels

Runtime failures: traps, rejected casts, checked conversions, failed capabilities, invalid dynamic protocol, memory violations, bounds, nil misuse where disallowed, unhandled failures, failed linkage, deterministic-limit exhaustion, cancellation.

Failures stay structured runtime/diagnostic events. SEAM never turns failure into host UB. Host-visible invocation outcomes are tagged: `returned`, `yielded`, `failed`, `trapped`, `cancelled` and represented by canonical tagged result structs. Host bindings may adapt shape but preserve exact tags and payload fields.

## Host embedding API

Baseline host embedding API is capability-aware. It includes load/link/call/resume/cancel/close/outcome plus module, resource, and capability-graph operations. Reflective frame/failure/package inspection belongs behind authorized `musi:probe` and tooling APIs, not baseline host API.

## Unknowns

- Exact host API function signatures not specified.
- Exact canonical result struct field layout not specified.
- Exact cyclic-init diagnostic wording/code not specified.
