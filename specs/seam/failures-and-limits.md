# SEAM failures, traps, limits, and halt behavior

SEAM failures are structured execution outcomes. They are not undefined host behavior and must not be hidden by source-language fallback.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` known-phase, safety, opcode, and runtime notes; `docs/language_checklist_for_musi.md` safety and known-execution requirements.

References: WebAsm distinguishes traps from successful completion in its execution model: <https://webassembly.github.io/spec/core/exec/instructions.html>. The JVM specification separates loading, linking, verification, and runtime errors: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html>.

## Invocation outcomes

A SEAM invocation ends in exactly one of these states:

| Outcome          | Meaning                                                                       |
| ---------------- | ----------------------------------------------------------------------------- |
| `returned`       | entry frame returned normally with verified outputs                           |
| `trapped`        | `trap` or core-defined trap condition occurred                                |
| `threw`          | exception escaped all handlers                                                |
| `link-failed`    | import/export/native/foreign linkage failed before execution                  |
| `limit-exceeded` | fuel, step, recursion, memory, or known-phase limit was exceeded              |
| `suspended`      | invocation yielded and produced a resumable state instead of final completion |
| `cancelled`      | suspended computation was closed/dropped/cancelled by host/runtime protocol   |

`halt` names any non-suspended final outcome. `returned`, `trapped`, `threw`, `link-failed`, `limit-exceeded`, and `cancelled` are halt outcomes.

## Trap sources

Trap sources include explicit `trap`, failed checked casts/conversions, invalid `unbox`, failed `cap.need`, bounds violations, memory permission violations, invalid pointer/reference access, invalid dynamic protocol use, divide-by-zero when core numeric rules choose trap semantics, and other core runtime violations.

Verification rejects statically knowable violations. Traps cover accepted modules whose runtime values trigger a failure condition.

## Limits

SEAM enforces configured limits:

- fuel/step limits;
- recursion/call-depth limits;
- memory allocation limits;
- stack-depth limits derived from verifier metadata;
- known-phase deterministic execution limits;
- host-specific resource limits.

Known-phase limit exhaustion is a compile-time known-execution failure. Runtime limit exhaustion is a runtime failure. Neither silently retries through an interpreter or host fallback.

## Diagnostic payload

A structured failure retains module identity, procedure/body identity, instruction offset or block/instruction index, opcode, relevant operand/table reference, source span when tool metadata supplies one, and a reason code. Failure classification cannot require source spans.

## Validation and failure cases

Load, verification, and link failures happen before execution. Runtime traps happen during execution. A SEAM implementation must not continue executing a frame after a final halt outcome.

## Unknowns

- Exact reason-code enum is not specified.
- Exact host embedding representation of outcomes is not specified.
- Exact mapping from numeric failures to trap kinds is not specified.
