# SEAM dynamic and capability protocols

SEAM dynamic ops and capability checks are explicit VM protocols, not fallback behavior from Musi `Any`.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` dynamic/capability opcode notes; `docs/language_checklist_for_musi.md` explicit dynamic + systems-safety entries.

Reference: WebAssembly host interaction uses imports/module instances, not ambient effects: <https://webassembly.github.io/spec/core/exec/modules.html>.

## Capability evidence

Capability evidence = typed metadata/runtime state referenced by `cap_idx`. `cap.has` tests capability presence. `cap.need` requires presence and fails through structured failure channel when absent.

Capability checks must be explicit in SEIL. `Any` values do not auto-provide capabilities.

## Dynamic call protocol

`call.dyn` uses signature operand, callee value, and argument pack. Core type metadata defines callee inspection, argument pack/unpack, and result validation.

Verifier accepts only when dynamic-call metadata exists for operand signature and callee protocol.

## Box/unbox protocol

`box` and `unbox` move between unboxed values and boxed/dynamic/heap representations. Representation ops, not `Any`-only source ops. `unbox` can fail when boxed value lacks compatible representation.

## Keyed storage protocol

`ld.key`, `st.key`, `has.key`, and `del.key` use explicit keyed-storage protocol metadata. No implicit JavaScript/Python field lookup.

## Unknowns

- Exact capability table schema not specified.
- Exact dynamic argument-pack representation not specified.
- Exact keyed-storage key/value constraints not specified.
