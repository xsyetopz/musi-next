# SEAM failures, traps, limits, and halt behavior

SEAM failures are structured execution outcomes, not host UB, not hidden source fallback.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` known-phase, safety, opcode, runtime notes; `docs/language_checklist_for_musi.md` safety + known-execution requirements.

References: WebAsm separates traps from success: <https://webassembly.github.io/spec/core/exec/instructions.html>. JVM separates loading, linking, verification, runtime errors: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html>.

## Invocation outcomes

Host-visible invocation outcome is exactly one tagged state:

| Outcome     | Meaning                                                                 |
| ----------- | ----------------------------------------------------------------------- |
| `returned`  | entry frame returned normally with verified outputs                     |
| `yielded`   | invocation yielded and produced opaque resumable handle                 |
| `failed`    | structured non-trap failure escaped or occurred before/during execution |
| `trapped`   | `trap` or core-defined runtime invariant violation occurred             |
| `cancelled` | suspended computation was cancelled/closed by runtime/host protocol     |

Load, verify, link, init, and resource-limit failures are classified under `failed` with phase/reason payloads. Structured failures are explicit operation/host outcomes. Traps are VM/runtime invariant violations or explicit `trap`. Host exceptions do not cross boundary as host exceptions; they become `failed` or `trapped` by ABI/host metadata.

`halt` = any non-yielded final outcome. `returned`, `failed`, `trapped`, and `cancelled` halt.

## Trap sources

Trap sources: explicit `trap`, VM/runtime invariant violations, invalid reference/access/address use, memory permission violations, invalid dynamic protocol states, bounds, failed checked casts/conversions when core numeric/type rules classify them as traps, invalid `unbox`, divide-by-zero when core numeric rules choose trap semantics, other core runtime violations. Failed `cap.need` and host/operation failures use structured `failed` unless metadata classifies a violation as trap.

Verification rejects statically knowable violations. Traps cover accepted modules whose runtime values fail.

## Limits

SEAM enforces:

- fuel/step limits;
- recursion/call-depth limits;
- memory allocation limits;
- stack-depth limits from verifier metadata;
- known-phase deterministic execution limits;
- host-specific resource limits.

Known-phase exhaustion = compile-time known-execution failure. Runtime exhaustion = structured `failed` outcome. No silent interpreter/host fallback.

## Diagnostic payload

Structured failure keeps module id, procedure/body id, instruction offset or block/instruction index, opcode, relevant operand/table ref, source span when tool metadata supplies one, and reason code. Failure classification cannot require source spans.

## Validation and failure cases

Load, verification, link, and init failures happen before user entry execution. Runtime traps happen during execution. SEAM must not continue frame after final halt.

## Unknowns

- Exact reason-code enum not specified.
- Exact numeric-failure to trap-kind map not specified.
