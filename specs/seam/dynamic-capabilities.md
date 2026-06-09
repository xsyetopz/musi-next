# SEAM dynamic and capability protocols

SEAM dynamic ops and capability checks are explicit VM protocols, not fallback behavior from Musi `Any`.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` dynamic/capability opcode notes; `docs/language_checklist_for_musi.md` explicit dynamic + systems-safety entries.

Reference: WebAssembly host interaction uses imports/module instances, not ambient effects: <https://webassembly.github.io/spec/core/exec/modules.html>.

## Capability evidence

Capabilities are first-class non-forgeable runtime values plus metadata requirements. Capability/resource graph uses typed nodes and typed authority edges. Graph nodes have opaque stable non-forgeable runtime identity. Nodes include provider, module, resource, capability, and handle metadata. Capability evidence = typed metadata/runtime state referenced by `cap_idx`. `cap.has` tests capability presence. `cap.need` requires presence and fails through structured failure channel when absent.

Capability checks must be explicit in SEAM bytecode. `Any` values do not auto-provide capabilities. Capability requirements appear in module/bytecode metadata, not new Musi syntax for now. Host resource handles are values protected by capabilities; identity is separate from authority. Graph inspection requires authority.

## Dynamic call protocol

`call.dyn` uses signature operand, callee value, and typed argpack record. Dynamic argpacks follow UALO semantics: positional arguments first, then named arguments, with defaults/schema validation from metadata. Typed argpack record preserves expected signature, result contract, and failure contract. Core type metadata defines callee inspection, argument pack/unpack, and result validation.

Verifier accepts only when dynamic-call metadata exists for operand signature and callee protocol.

## Box/unbox protocol

`box` and `unbox` move between unboxed values and boxed/dynamic/heap representations. Representation ops, not `Any`-only source ops. `unbox` can fail when boxed value lacks compatible representation.

## Keyed storage protocol

`ld.key`, `st.key`, `has.key`, and `del.key` use typed key schemas. Declared key domains may be symbol, string, integer, enum, or compound, with value constraints and capability requirements. Arbitrary `Any` keys do not become valid by default. No implicit JavaScript/Python field lookup.

## Unknowns

- Exact capability graph node/edge schemas and authority predicates not specified.
- Exact typed argpack record fields and binary/runtime encoding not specified.
- Exact typed key schema encoding not specified.
