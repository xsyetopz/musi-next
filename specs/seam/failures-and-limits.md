# SEAM failures, traps, limits, and halt behavior

SEAM failures are structured execution outcomes, not host UB, not hidden source fallback.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` known-phase, safety, opcode, runtime notes; `docs/language_checklist_for_musi.md` safety + known-execution requirements.

References: WebAsm separates traps from success: <https://webassembly.github.io/spec/core/exec/instructions.html>. JVM separates loading, linking, verification, runtime errors: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html>.

## Invocation outcomes

Each SEAM invocation ends in exactly one state:

| Outcome          | Meaning                                                                       |
| ---------------- | ----------------------------------------------------------------------------- |
| `returned`       | entry frame returned normally with verified outputs                           |
| `trapped`        | `trap` or core-defined trap condition occurred                                |
| `threw`          | exception escaped all handlers                                                |
| `link-failed`    | import/export/native/foreign linkage failed before execution                  |
| `limit-exceeded` | fuel, step, recursion, memory, or known-phase limit was exceeded              |
| `suspended`      | invocation yielded and produced a resumable state instead of final completion |
| `cancelled`      | suspended computation closed/dropped/cancelled by host/runtime protocol       |

`halt` = any non-suspended final outcome. `returned`, `trapped`, `threw`, `link-failed`, `limit-exceeded`, and `cancelled` halt.

## Trap sources

Trap sources: explicit `trap`, failed checked casts/conversions, invalid `unbox`, failed `cap.need`, bounds, memory permission violations, invalid reference/access/address use, invalid dynamic protocol, divide-by-zero when core numeric rules choose trap semantics, other core runtime violations.

Verification rejects statically knowable violations. Traps cover accepted modules whose runtime values fail.

## Limits

SEAM enforces:

- fuel/step limits;
- recursion/call-depth limits;
- memory allocation limits;
- stack-depth limits from verifier metadata;
- known-phase deterministic execution limits;
- host-specific resource limits.

Known-phase exhaustion = compile-time known-execution failure. Runtime exhaustion = runtime failure. No silent interpreter/host fallback.

## Diagnostic payload

Structured failure keeps module id, procedure/body id, instruction offset or block/instruction index, opcode, relevant operand/table ref, source span when tool metadata supplies one, and reason code. Failure classification cannot require source spans.

## Validation and failure cases

Load, verification, link failures happen before execution. Runtime traps happen during execution. SEAM must not continue frame after final halt.

## Unknowns

- Exact reason-code enum not specified.
- Exact host embedding outcome representation not specified.
- Exact numeric-failure to trap-kind map not specified.
