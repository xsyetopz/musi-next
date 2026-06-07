# SEIL types, layouts, and metadata

SEIL module semantics depend on typed tables and required VM metadata. Types are preserved because SEIL is a typed executable language, not a disposable untyped compiler IR. Optional tool metadata supports tooling and cannot affect execution.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` sections 16-18, `grammar/seil.ebnf`, `grammar/musi.ebnf`, `docs/language_checklist_for_musi.md`.

External sources used:

- WebAsm GC/reference types show a VM text/binary format can expose typed managed references while leaving host GC implementation details to the runtime: <https://webassembly.github.io/gc/core/>.
- Wasmtime reference-type implementation uses safepoint stack maps to find live references: <https://bytecodealliance.org/articles/reference-types-in-wasmtime>.
- ECMA-335 distinguishes managed references/pointers from unmanaged pointers for verifiability: <https://docs.ecma-international.org/ecma-335/Ecma-335-part-i-iv.pdf>.

## Asm metadata

The local `asm` declaration is required metadata. Textual `asm` records asm identity, asm version, entry metadata, and early runtime/capability contract members for hand-authored SEIL.

Assembly lowers runtime/capability contract members, extension declarations, asm references, and imports into binary `deps` rows. Binary `asm` carries identity/version/entry rows only and is decoded before dependent payloads. A loader or verifier must reject a module when a required `deps` contract is unsupported. Asm `meta` entries are accepted only when defined by the core metadata schema.

## Type universe

SEIL text admits primitive names `Bit`, `Byte`, `i8`, `i16`, `i32`, `i64`, `n8`, `n16`, `n32`, `n64`, `f32`, and `f64`. Managed and unmanaged references are type constructors: `(ref T)` and `(ptr T)`.

Types are semantic metadata. They drive verification, layout, calling convention selection, representation transitions, memory access, dynamic protocol checks, export/import compatibility, root tracing, and write-barrier obligations.

## Sigs

Signature metadata defines callable inputs and outputs. Invocation instructions refer to signatures directly or through procedure declarations. Stack-effect verification expands `inputs(S)`, `outputs(S)`, `yield(S)`, and `resume(S)` from signature and suspension metadata.

## Layout metadata

Layout metadata records representation, packing, alignment, endian, tags, padding, ABI layout, fixed-storage requirements, and core representation choices. Product fields, positional product fields, tagged alts, array elements, and boxed representations are not inferred from instruction spelling alone.

Layout metadata must identify which fields/elements contain managed references. This is required for precise root tracing, heap scanning, write barriers, and safe movement/pinning under moving or partially moving collectors.

## Managed references, pointers, and roots

`(ref T)` is a managed SEAM reference and is a GC root when live in stack, local, argument, environment, global, or heap metadata that admits references. `(ptr T)` is an unmanaged pointer and is not a GC root.

Verifiable SEIL must not hide managed references in integer storage, byte arrays, opaque pointers, or unknown layouts. Conversions between managed references and unmanaged pointers require explicit core metadata and are rejected by default.

## Required VM metadata

Required metadata includes information needed for:

- verification of body stack effects and control edges;
- loading section/table dependencies;
- linking imports, exports, native declarations, and foreign declarations;
- representation/layout of values;
- root maps and stack maps at GC safepoints;
- write-barrier obligations for managed-reference writes;
- capability evidence and dynamic protocols;
- target gating;
- known-phase deterministic execution;
- exception, cleanup, branch-table, address-target, yield/resume, and dynamic argpack body metadata.

A loader/verifier must reject modules missing required VM metadata for accepted instructions or declarations.

## Optional tool metadata

Tool metadata includes source maps, exact source symbol spelling, source-shape hints, comments/docs, datum/operator/pattern spelling, and decompilation hints. Execution must not depend on it. SEAM may skip tool metadata during execution loading.

## Targets and semantic ownership

SEIL/SEAM do not use executable-semantics dialects. Numeric behavior, ABI details, FFI representability, ext row kinds, ext opcodes, capability protocols, dynamic operations, nil admission, pointer/memory policies, and native binding rules are core SEIL/SEAM behavior, library/native behavior, frontend-owned behavior, or unsupported.

Target metadata determines availability of declarations and code before semantic checks or verification. Target metadata is not a runtime branch mechanism.

## ABI boundary rules

Extern/native ABI boundary types must be representable under core ABI metadata. `Any`, opaque/erased values, closures, shapes, `Maybe`, `Expect`, GC references, and other high-level source constructs are not ABI-safe unless core ABI metadata defines an explicit representation. Text values are not silently C strings.

## Unknowns

- Exact binary encodings for all type and metadata table payloads are not defined.
- Exact ABI descriptor grammar is not fully specified.
