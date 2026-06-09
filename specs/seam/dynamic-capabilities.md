# SEAM dynamic and capability protocols

SEAM dynamic ops and capability checks are explicit VM protocols, not fallback behavior from Musi `Any`.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` dynamic/capability opcode notes; `docs/language_checklist_for_musi.md` explicit dynamic + systems-safety entries.

Reference: WebAssembly host interaction uses imports/module instances, not ambient effects: <https://webassembly.github.io/spec/core/exec/modules.html>.

## Capability evidence

Capabilities are first-class non-forgeable runtime values plus metadata requirements. Capability evidence = typed metadata/runtime state referenced by `cap_idx`. `cap.has` tests capability presence. `cap.need` requires presence and fails through structured failure channel when absent.

Capability checks must be explicit in SEAM bytecode. `Any` values do not auto-provide capabilities. Capability requirements appear in module/bytecode metadata, not new Musi syntax for now. Host resource handles are values protected by capabilities; identity is separate from authority.

## Dynamic call protocol

`call.dyn` uses signature operand, callee value, and argument pack. Dynamic argpacks follow UALO semantics: positional arguments first, then named arguments, with defaults/schema validation from metadata. Core type metadata defines callee inspection, argument pack/unpack, and result validation.

Verifier accepts only when dynamic-call metadata exists for operand signature and callee protocol.

## Box/unbox protocol

`box` and `unbox` move between unboxed values and boxed/dynamic/heap representations. Representation ops, not `Any`-only source ops. `unbox` can fail when boxed value lacks compatible representation.

## Keyed storage protocol

`ld.key`, `st.key`, `has.key`, and `del.key` use explicit keyed-storage protocol metadata. Keyed storage is limited to declared key domains and declared value constraints; arbitrary `Any` keys do not become valid by default. No implicit JavaScript/Python field lookup.

## Unknowns

- Exact capability table schema not specified.
- Exact binary/runtime representation of UALO-shaped dynamic argument packs not specified.
- Exact keyed-storage domain/value schema encoding not specified.
