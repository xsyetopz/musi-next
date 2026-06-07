# SEAM dynamic and capability protocols

SEAM dynamic operations and capability checks are explicit VM protocols. They are not implicit fallback behavior from Musi `Any` values.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` dynamic/capability opcode notes, `docs/language_checklist_for_musi.md` explicit dynamic and systems-safety entries.

References: WebAssembly mediates host interaction through imports and module instances instead of ambient effects: <https://webassembly.github.io/spec/core/exec/modules.html>.

## Capability evidence

Capability evidence is typed metadata/runtime state referenced by `cap_idx`. `cap.has` tests whether a value/evidence set provides a capability. `cap.need` requires it and fails through the structured failure channel when absent.

Capability checks must be explicit in SEIL. Source values of type `Any` do not automatically provide capabilities.

## Dynamic call protocol

`call.dyn` uses a signature operand, callee value, and argument pack. Core type metadata defines how the callee is inspected, how arguments are packed/unpacked, and how results are validated.

Verifier acceptance requires dynamic-call metadata for the operand signature and callee protocol.

## Box/unbox protocol

`box` and `unbox` transition between unboxed values and boxed/dynamic/heap representations. They are representation operations, not `Any`-only source operations. `unbox` can fail if the boxed value does not contain a compatible representation.

## Keyed storage protocol

`ld.key`, `st.key`, `has.key`, and `del.key` operate on explicit keyed-storage protocol metadata. They are not implicit JavaScript/Python-style field lookup.

## Unknowns

- Exact capability table schema is not specified.
- Exact dynamic argument-pack representation is not specified.
- Exact keyed-storage key/value type constraints are not specified.
