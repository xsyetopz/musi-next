# Locked SEIL/SEAM Design

Status: locked platform design for SEIL, SEAM, and Musi boundary. High-level gaps resolved; details fold into owning specs.

## 1. Platform Identity

SEAM = runtime platform. Loads, verifies, links, initializes, executes SEIL modules.

SEIL = final portable executable artifact for SEAM. CIL-like role: source-to-runtime artifact, not compiler-private IR. SEIL modules are compiler outputs and runtime inputs.

Musi = flagship source language for SEAM. Musi lowers directly to SEIL. No hidden IR layer.

SEIL stack effects belong to verification/execution. Musi source stays infix + expression-shaped because source serves humans; SEIL exposes stack machine directly.

No executable-semantics dialect escape hatch. Feature is core, library/native, frontend-owned, or unsupported. Unknown executable semantics rejected.

## 2. Artifact And Metadata

`.seil` = canonical textual executable module artifact. Hand-writable, not Musi source.

SEAM tooling may assemble `.seil` into internal binary image for loading, caching, package transport, or execution. Binary image starts with fixed probe header. Header identifies container + section directory only. Semantic compatibility lives elsewhere.

Semantic module data lives in compact section families, typed rows, executable instruction streams. Core binary section families:

- `names`: interned names + strings
- `asm`: current module identity, version, entry
- `deps`: runtime/cap/ext requirements, asm refs, imports
- `defs`: types, fields, alts, signatures, globals, constants, procedures, exports
- `code`: bodies, blocks, regions, branch tables, address targets, instruction bytes
- `data`: constant payloads, layouts, reference maps, ABI records, dynamic/capability schemas
- `meta`: required semantic metadata not owned by `defs`, `code`, or `data`
- `tool`: optional non-semantic source/tool metadata

Unknown semantic section families, required row kinds, metadata schemas, or opcodes are rejected.

Each section payload starts with row-kind directory, then row offset table, then packed row bytes. Row-kind entry states row kind id, row count, offset-table range, payload range, schema id/core schema tag, required/skippable policy. Keeps sections compact; avoids giant universal record.

Tool metadata optional. It may preserve source spans, names, comments, docs, import/export shape, grouping, decompilation hints. It cannot affect execution, verification, linking, runtime behavior.

Package/archive formats wrap SEIL modules. Separate from core module semantics.

Assembly/module/package names are canonical + case-sensitive. Tools must not case-fold, Unicode-normalize, dash-convert, or rewrite logical names.

## 3. Execution Semantics

SEIL = typed stack-effect executable language.

SEIL text combines:

- WAT/Lisp-like parenthesized declarations
- line-oriented RPN/Forth-like instruction bodies

Readable assembly form, exact VM behavior.

Every opcode has stable numeric id, mnemonic, immediate operand schema, stack-effect schema, verification behavior, runtime behavior.

Verifier tracks stack height + value types at every instruction. Procedure body declares signature. `ret` must match outputs.

Branch targets have incoming stack shape. All incoming edges to same target must match. No hidden coercion at joins; conversion must appear in code.

Core value forms: scalars, products, sums, indexed storage/arrays, callable values, boxes, managed refs, unmanaged pointer/access values, address tokens.

Products/sums are core data forms. No mandatory CLR/JVM class-object center.

Core call/control forms: direct call, indirect call, receiver dispatch, dynamic call, branch, table branch, return, trap, throw, rethrow, cleanup edge, yield/suspend.

Dynamic mechanisms must carry explicit callee, arg, key, evidence, result, failure contracts.

Verification rejects unknown opcodes, malformed operands, stack underflow, stack type mismatch, invalid branch joins, missing metadata, invalid reference/access/address/pointer use, invalid calls, unsupported semantic module data.

## 4. Runtime Semantics

SEAM executes modules in order:

1. read fixed header
2. read section directory
3. read mandatory `asm` identity/version/entry rows
4. read dependency contracts from `deps`
5. reject unsupported deps or unknown semantic parts
6. decode `defs`, `code`, `data`, `meta`, supported `tool`
7. verify opcode/type/stack/control/metadata contracts
8. link imports, exports, native procedures, foreign procedures
9. initialize module
10. execute entry or exported callable

Frame contains callable identity, instruction position, arg slots, local slots, env/capture slots, operand stack, cleanup/handler/yield state.

Args/locals/env/captures are typed slots. Operand stack typed by verifier state, not runtime guessing.

SEAM derives GC roots from verified stack maps. Roots include managed refs on operand stack, frame slots, globals, active exception state, yield state, runtime handles.

Safepoints: allocation, calls, dynamic calls, throws, yields, native/foreign boundaries that can allocate/block/call back.

Managed-ref writes into heap/global/array/boxed/ref-bearing storage must execute write barrier. Raw byte writes cannot smuggle managed refs.

`fixed` = stable address/access path required. SEAM may pin, use nonmoving storage, copy to unmanaged storage, or reject. `fixed` not GC-off. `unmanaged` = representation outside managed tracing, movement, reclamation unless core metadata says otherwise.

SEAM outcomes: success with outputs; trap/structured failure; load/verify/link failure; suspended/yielded state. Host APIs expose same model and must not collapse all failures into host exceptions.

SEAM has one core C-compatible FFI bridge:

- SEIL calls native through import metadata
- native calls SEIL through exported callable tables
- managed values cross as handles unless represented, fixed/accessed, or copied
- callbacks enter through SEAM trampolines so frames, roots, safepoints, failures stay valid

## 5. Musi, Dynamic Semantics, And Metadata

Musi remains flagship source language. Human-first, infix, expression-shaped.

Musi lowers directly to SEIL. Musi docs explain stack effects through SEIL text + verifier behavior, not by forcing Forth-like source.

Musi low-level memory names: `Address`, `Region`, `Access[T]`, `Access[mut T]`. `MutAccess[T]` and `OpaqueAccess[T]` are source DRY aliases only. No source `Ptr`/`Pointer`; SEIL may still use VM `(ptr T)` and `Ptr[T]`.

Musi dynamic features lower to bounded core SEIL dynamic mechanisms. Dynamic calls must carry callee, arg pack, expected signature, result contract, structured failure. Keyed access defines key/value constraints. Capability checks define evidence semantics.

`@noalloc` = core allocation contract. Callable performs no managed heap allocation. Not GC-off; does not prevent collection by other threads/runtime unless SEAM later defines stronger scheduling rule.

`@noalloc` callable rejects managed allocation, boxing, managed array/text/object creation, closure allocation, calls to non-`@noalloc`, dynamic calls unless target proven `@noalloc`.

Native imports may be `@noalloc` only by explicit declaration. Wrong declarations are boundary contract violations.

SEIL preserves `@noalloc` metadata so SEAM/tools can validate allocation-free paths for low-latency code, FFI callbacks, runtime internals.

Tool metadata optional + non-semantic. May preserve Musi source spans, comments, docs, grouping, import/export shape, spelling, decompilation hints. Cannot affect execution.

Known-phase execution deterministic. No ambient time/random/process/env/filesystem/network/IO unless explicit deterministic import provides it.

Standard native modules provide filesystem, process, time, randomness, text, encoding, system integration. Library/native surface, not hidden SEIL semantics.

## 6. Gap Resolution Policy

Every unknown resolves to one category:

- core SEIL/SEAM behavior
- library/native module behavior
- frontend-owned behavior
- unsupported behavior

No executable-semantics dialect bucket.

Core gaps needing detail:

- full verifier rule tables
- branch join compatibility rules
- dynamic call, keyed storage, capability schemas
- trap/failure reason enum
- frame/object layout contracts
- GC barrier + safepoint rule tables
- host embedding API
- native/foreign ABI descriptors
- module/package discovery + archive format
- Musi-to-SEIL lowering patterns
- tool metadata payloads for source maps + decompilation

Detail gaps only, not platform identity gaps. Fold into owning specs.
