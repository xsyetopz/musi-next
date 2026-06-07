# Musi lowering to SEIL

Musi is a source language that lowers directly to SEIL. SEIL is the canonical lowered executable form, and SEAM executes verified SEIL. SEIL is lower-level than Musi ASTs by clarifying ambiguous constructs and removing redundant source forms, but higher-level than typical compiler IRs by preserving semantic types and source relationship metadata. There is no separate source-tree interpreter for known execution.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md`, `grammar/musi.ebnf`, `grammar/seil.ebnf`, `docs/language_checklist_for_musi.md`.

External sources used:

- WebAsm demonstrates typed module validation and WAT text independent of source language syntax: <https://webassembly.github.io/spec/core/>.
- ECMA-335 demonstrates a VM architecture where high-level languages lower into typed executable metadata and instruction bodies: <https://docs.ecma-international.org/ecma-335/Ecma-335-part-i-iv.pdf>.
- Wasmtime reference-type stack maps show the runtime need for precise live-reference locations at safepoints: <https://bytecodealliance.org/articles/reference-types-in-wasmtime>.
- Generational GC remembered-set literature shows the need for write barriers on reference writes: <https://www.usenix.org/conference/java-vm-02/concurrent-remembered-set-refinement-generational-garbage-collection>.

## Boundary

Musi compilation emits a SEIL module with asm metadata, imports/exports, type/layout metadata, signatures, constants, globals, procedure declarations, bodies, target/capability metadata, and required body/VM metadata. Optional tool metadata supports tooling and decompilation; SEAM execution cannot require it.

Musi constructs that affect runtime representation, verification, linking, FFI, capabilities, dynamic behavior, target availability, known execution, GC rooting, or write-barrier obligations must lower into SEIL semantic declarations or required metadata.

## Names and source relationship

Musi source names lower to exact SEIL symbols. Lowering must not case-fold, dash-convert, Unicode-normalize, abbreviate, or otherwise rewrite source symbols. If a Musi name requires escaping in source, SEIL stores the same logical symbol and may use its own escaped-symbol spelling in text.

Tool metadata may preserve source grouping, import shape, export grouping, comments/docs, source spans, operator spelling, pattern spelling, and decompilation hints. None of this metadata may affect SEAM execution.

## Known phase

`known` expressions, bindings, parameters, and types lower to SEIL and execute under deterministic SEAM known-phase limits. The compiler must not evaluate known code by walking Musi ASTs with a separate evaluator.

Known execution has no ambient time/random/process/env/IO/filesystem/network access unless a deterministic known import/intrinsic explicitly provides it.

## Target filtering

`@target` metadata attaches to a node. Nonmatching target nodes are absent before semantic checks. `@target` is not a runtime branch and cannot lower to ordinary SEIL control flow for availability decisions.

Scalar target predicates are exact, array predicates are any-of, record predicates are conjunction, and arrays of records are disjunction.

## Representation, GC, and FFI

`@repr`, packing, alignment, endian, tags, padding, ABI layout, `@extern`, intrinsic/runtime binding metadata, target ABI metadata, and managed-reference layout information must lower into SEIL metadata when they affect representation, linking, tracing, barriers, or execution.

Managed references lower to SEIL `(ref T)` types. Unmanaged/system pointers lower to `(ptr T)` types. ABI boundary types must be representable. `Any`, opaque/erased values, closures, shapes, `Maybe`, `Expect`, GC references, and text values are not silently ABI-compatible unless core ABI metadata defines an explicit representation.

## Fixed storage and pointers

`fixed` lowers to stable-address storage requirements. Under a moving or partially moving SEAM collector such as generational Immix, fixed storage is implemented through pinning, nonmoving allocation, unmanaged copies, or checked rejection. `fixed` does not disable GC globally.

Pointer types and pointer access lower to explicit SEIL pointer/reference operations and layout metadata. Musi has no `unsafe` wrapper that converts dangerous behavior into warnings.

## Shapes, witnesses, and dynamic behavior

Shapes and witness conformance lower to metadata/evidence that SEIL dynamic and capability operations can reference. `Any` does not imply implicit dynamic lookup. Dynamic operations require explicit metadata and evidence.

## No-allocation contract

`@noalloc` lowers to SEIL callable metadata. A `@noalloc` callable performs no managed heap allocation. It may not allocate managed objects, box values, create managed arrays/text/objects, allocate closures, call non-`@noalloc` procedures, or use a dynamic call unless the target is proven `@noalloc`.

`@noalloc` does not disable GC globally. It is an allocation contract for the procedure body and its transitive calls.

## Unknowns

- Exact lowering algorithms for every Musi expression form are not fully specified here.
- Exact source-map/tool-metadata payloads are not specified.
- Exact import path resolution and module packaging rules remain partially unspecified.
