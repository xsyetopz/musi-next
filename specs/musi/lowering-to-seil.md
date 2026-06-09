# Musi lowering to SEIL

Musi lowers directly to SEIL. SEIL is canonical executable form; SEAM executes verified SEIL. SEIL is lower than Musi ASTs because it removes source ambiguity, higher than disposable IR because semantic types + source relation persist. No source-tree interpreter for known execution.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md`, `grammar/musi.ebnf`, `grammar/seil.ebnf`, `docs/language_checklist_for_musi.md`.

External sources:

- WebAsm typed module validation + WAT text independent from source syntax: <https://webassembly.github.io/spec/core/>.
- ECMA-335 VM architecture uses typed executable metadata + instruction bodies: <https://docs.ecma-international.org/ecma-335/Ecma-335-part-i-iv.pdf>.
- Wasmtime reference stack maps show need for precise live-ref locations at safepoints: <https://bytecodealliance.org/articles/reference-types-in-wasmtime>.
- Generational GC remembered sets require write barriers on reference writes: <https://www.usenix.org/conference/java-vm-02/concurrent-remembered-set-refinement-generational-garbage-collection>.

## Boundary

Musi compilation emits SEIL module with asm metadata, imports/exports, type/layout metadata, signatures, constants, globals, procs, bodies, target/capability metadata, required body/VM metadata. Optional tool metadata supports tooling/decompilation; SEAM execution cannot require it.

Any Musi construct affecting runtime representation, verification, linking, FFI, capabilities, dynamic behavior, target availability, known execution, GC roots, or write barriers must lower into SEIL semantic declarations or required metadata.

## Names and source relationship

Musi names lower to exact SEIL symbols. No case-fold, dash-convert, Unicode-normalize, abbreviate, or rewrite. Escaped source name stores same logical symbol; SEIL may use its own escaped spelling in text.

Tool metadata may preserve grouping, import/export shape, comments/docs, spans, operator spelling, pattern spelling, decompilation hints. Tool metadata cannot affect SEAM execution.

## Known phase

`known` expressions, bindings, params, and types lower to SEIL and execute under deterministic SEAM known-phase limits. Compiler must not evaluate known code by walking Musi ASTs.

Known execution has no ambient time/random/process/env/IO/filesystem/network unless explicit deterministic known import or declared `musi:rt` intrinsic provides it.

## Target filtering

`@target` metadata attaches to node. Nonmatching target nodes absent before semantic checks. `@target` is not runtime branch and cannot lower to ordinary SEIL control flow.

Scalar predicates exact. Array predicates any-of. Record predicates conjunction. Arrays of records disjunction.

## Representation, GC, and FFI

`@repr`, packing, alignment, endian, tags, padding, ABI layout, `@extern`, declared `musi:rt` intrinsic/runtime metadata, target ABI metadata, managed-ref layout info must lower into SEIL metadata when affecting representation, linking, tracing, barriers, or execution.

Managed refs lower to `(ref T)`. Source `Access[T]` / `Access[mut T]` lower to explicit SEIL pointer/ref ops plus layout, region, permission, capability metadata. SEIL still uses `(ptr T)` for VM unmanaged pointer/access values. `Address` lowers only as address token; cannot load/store/root managed values without explicit region/access conversion.

ABI boundary types must be representable. `Any`, opaque/erased values, closures, shapes, `Maybe`, `Expect`, GC refs, and text values are not silently ABI-compatible unless core ABI metadata defines representation.

## Fixed storage and access

`fixed` lowers to stable-address storage requirements. `unmanaged` lowers to storage/representation outside managed tracing, movement, reclamation unless core metadata says otherwise. Under moving/partly moving SEAM collector like generational Immix, fixed storage uses pinning, nonmoving allocation, unmanaged copies, or rejection. `fixed` not GC-off.

Access types lower to explicit SEIL pointer/ref ops + layout metadata. Core runtime ops lower through explicit `musi:rt` declarations with signature, phase, allocation, failure/trap, capability, target/profile, lowering metadata. Ordinary names get no hidden compiler privilege.

`Access[T]` readable typed access. `Access[mut T]` readable/writable typed access. `MutAccess[T]` and `OpaqueAccess[T]` source aliases only. No source `Ptr`/`Pointer`; no `unsafe` wrapper that turns dangerous behavior into warning.

## Shapes, witnesses, and dynamic behavior

Shapes and witness conformance lower to metadata/evidence referenced by SEIL dynamic/capability ops. `Any` does not imply implicit dynamic lookup. Dynamic ops require explicit metadata + evidence.

## No-allocation contract

`@noalloc` lowers to SEIL callable metadata. It forbids managed heap allocation: no managed objects, boxes, arrays/text/objects, closure allocation, calls to non-`@noalloc`, or dynamic call unless target proven `@noalloc`.

`@noalloc` is allocation contract, not GC-off.

## Unknowns

- Exact lowering algorithms for every Musi expression form not fully specified here.
- Exact source-map/tool-metadata payloads not specified.
- Exact import path resolution and module packaging partly unspecified.
