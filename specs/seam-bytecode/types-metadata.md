# SEAM bytecode types, layouts, and metadata

SEAM bytecode semantics depend on typed tables + required VM metadata. Types persist because SEAM bytecode is typed executable language, not disposable untyped compiler IR. Optional tool metadata supports tooling only.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, `grammar/seam-bytecode-text.ebnf`, `grammar/musi.ebnf`, `docs/language_checklist_for_musi.md`.

External sources:

- WebAsm GC/reference types show VM text/binary format can expose typed managed refs while runtime owns GC details: <https://webassembly.github.io/gc/core/>.
- Wasmtime reference-type implementation uses safepoint stack maps for live refs: <https://bytecodealliance.org/articles/reference-types-in-wasmtime>.
- ECMA-335 separates managed refs/pointers from unmanaged pointers for verification: <https://docs.ecma-international.org/ecma-335/Ecma-335-part-i-iv.pdf>.

## Asm metadata

Local `asm` declaration is required metadata. Text/disassembly `asm` records identity, version, entry metadata, and early runtime/capability contract members for hand-authored bytecode text.

Assembly lowers runtime/capability contracts, ext declarations, asm refs, and imports into binary `deps` rows. Binary `asm` carries identity/version/entry only and decodes before dependent payloads. Loader/verifier rejects unsupported required `deps` contract. Asm `meta` entries accepted only when core schema defines them.

## Type universe

SEAM bytecode text/disassembly admits primitive names `Bit`, `Byte`, `i8`, `i16`, `i32`, `i64`, `n8`, `n16`, `n32`, `n64`, `f32`, `f64`. Managed/unmanaged refs use constructors `(ref T)` and `(ptr T)`.

Types are semantic metadata. They drive verification, layout, calling convention, representation transitions, memory access, dynamic protocol checks, import/export compatibility, root tracing, barriers. Type, layout, ABI, dynamic, capability, and tool metadata table schemas come from the same declarative generated schema source used by `.seam` encoding.

## Sigs

Signature metadata defines callable inputs/outputs. Invocation instructions reference signatures directly or through procedure declarations. Stack verification expands `inputs(S)`, `outputs(S)`, `yield(S)`, `resume(S)` from signature/suspension metadata.

## Layout metadata

Layout metadata records representation, packing, alignment, endian, tags, padding, ABI layout, fixed-storage requirements, core representation choices. Product fields, positional fields, tagged alts, array elems, boxed reps are not inferred from instruction spelling alone.

Layout metadata must identify managed-ref fields/elements for precise root tracing, heap scanning, write barriers, movement/pinning.

## Managed references, access values, pointers, and roots

`(ref T)` = managed SEAM ref and GC root when live in stack, local, arg, env, global, or heap metadata admitting refs. `(ptr T)` = VM unmanaged pointer/access value, not GC root. Musi source has no `Ptr`/`Pointer`; source `Access[T]` / `Access[mut T]` lower to `(ptr T)` plus region, permission, layout, capability metadata.

Verifiable SEAM bytecode must not hide managed refs in integer storage, byte arrays, address storage, opaque access storage, unmanaged storage, unknown layouts. Conversions among managed refs, addresses, and unmanaged pointer/access values require explicit core metadata; rejected by default.

## Required VM metadata

Required metadata covers:

- body stack-effect + control-edge verification;
- section/table dependency loading;
- import/export/native/foreign linking;
- value representation/layout;
- root maps + stack maps at GC safepoints;
- write barriers for managed-ref writes;
- capability evidence + dynamic protocols;
- target gating;
- known-phase deterministic execution;
- exception, cleanup, branch-table, address-target, yield/resume, dynamic argpack body metadata.

Loader/verifier rejects modules missing required VM metadata for accepted instructions/declarations.

## Optional tool metadata

Tool metadata uses typed non-semantic registry rows. It includes source maps, exact source symbol spelling, source-shape hints, comments/docs, import/export grouping, datum/operator/pattern spelling, decompilation hints, and probe/debug data. Execution must not depend on it. SEAM may skip tool metadata during execution loading.

## Targets and semantic ownership

SEAM bytecode/SEAM have no executable-semantics dialects. Numeric behavior, ABI details, FFI representability, ext rows/opcodes, capability protocols, dynamic ops, nil admission, access/memory policies, native binding rules are core, library/native, frontend-owned, or unsupported.

Target metadata determines availability before semantic checks/verification. Target metadata is not runtime branch.

## ABI boundary rules

Extern/native ABI boundary types must be representable under core ABI metadata. ABI descriptors are host-ABI capable from start: C ABI, handles, callbacks through exported callable handles, resources, async/yield/resumable interaction, cancellation, failure outcomes, and representable memory access metadata. `Any`, opaque/erased values, closures, shapes, `Maybe`, `Expect`, GC refs, and other high-level source constructs are not ABI-safe unless core ABI metadata defines explicit representation. Text values are not silently C strings.

## Detail gaps

- Exact generated type/layout/metadata schema entries are not defined.
- Exact ABI descriptor fields and validation rules are not fully specified.
