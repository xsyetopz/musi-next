# Locked SEAM bytecode/SEAM Design

Status: locked platform design for SEAM bytecode, SEAM, and Musi boundary. High-level gaps resolved; details fold into owning specs.

## 1. Platform Identity

SEAM = runtime platform. Loads, verifies, links, initializes, and executes `.seam` bytecode images.

SEAM bytecode = final portable executable bytecode for SEAM. CIL-like role: source-to-runtime artifact, not compiler-private IR. SEAM bytecode modules are compiler outputs and runtime inputs.

Musi = flagship source language for SEAM. Musi lowers directly to `.seam`. No hidden IR layer and no second bytecode artifact.

SEAM bytecode stack effects belong to verification/execution. Musi source stays infix + expression-shaped because source serves humans; SEAM bytecode exposes stack machine directly.

No executable-semantics dialect escape hatch. Feature is core, library/native, frontend-owned, or unsupported. Unknown executable semantics rejected.

## 2. Artifact And Metadata

`.seam` = canonical compiled SEAM bytecode image, analogous to Erlang `.beam`. It is the public build/cache/distribution artifact for compiled Musi modules.

SEAM bytecode text/disassembly is a tool format for reading, assembling, testing, and debugging bytecode. It is not a separate language artifact extension and is not the package/distribution target. Tools may assemble text/disassembly into `.seam` and disassemble `.seam` back to readable text.

A `.seam` image starts with fixed 40-byte probe header. Header identifies image format + section directory only. Semantic compatibility lives in sections and dependency rows.

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

Package/archive/container formats may wrap `.seam` images. Compression, checksums, signatures, resources, and multi-image bundling live outside the core `.seam` image.

Assembly/module/package names are canonical + case-sensitive. Tools must not case-fold, Unicode-normalize, dash-convert, or rewrite logical names.

## 3. Execution Semantics

SEAM bytecode = typed stack-effect executable bytecode.

SEAM bytecode text/disassembly combines:

- WAT/Lisp-like parenthesized declarations
- line-oriented RPN/Forth-like instruction bodies

Readable assembly form, exact VM behavior. `.seam` itself remains the compiled bytecode image.

Every opcode has stable numeric id, mnemonic, immediate operand schema, stack-effect schema, verification behavior, runtime behavior.

Verifier tracks stack height + value types at every instruction. Procedure body declares signature. `ret` must match outputs.

Branch targets have incoming stack shape. All incoming edges to same target must match. No hidden coercion at joins; conversion must appear in bytecode.

Core value forms: scalars, products, sums, indexed storage/arrays, callable values, boxes, managed refs, unmanaged access values, address tokens.

Products/sums are core data forms. No mandatory CLR/JVM class-object center.

Core call/control forms: direct call, indirect call, receiver dispatch, dynamic call, branch, table branch, return, trap, throw, rethrow, cleanup edge, yield/suspend.

Dynamic mechanisms must carry explicit callee, arg, key, evidence, result, failure contracts.

Verification rejects unknown opcodes, malformed operands, stack underflow, stack type mismatch, invalid branch joins, missing metadata, invalid reference/access/address use, invalid calls, unsupported semantic module data.

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
9. initialize dependency modules before dependents; manifest declaration order breaks otherwise equal ties
10. execute entry or exported callable

Frame contains callable identity, instruction position, arg slots, local slots, env/capture slots, operand stack, cleanup/handler/yield state.

Args/locals/env/captures are typed slots. Operand stack typed by verifier state, not runtime guessing.

SEAM derives GC roots from verified stack maps. Roots include managed refs on operand stack, frame slots, globals, active exception state, yield state, runtime handles.

Safepoints: allocation, calls, dynamic calls, throws, yields, native/foreign boundaries that can allocate/block/call back.

Managed-ref writes into heap/global/array/boxed/ref-bearing storage must execute write barrier. Raw byte writes cannot smuggle managed refs.

`fixed` = stable address/access path required. SEAM may pin, use nonmoving storage, copy to unmanaged storage, or reject. `fixed` not GC-off. `unmanaged` = representation outside managed tracing, movement, reclamation unless core metadata says otherwise.

SEAM outcomes are tagged: `returned`, `yielded`, `failed`, `trapped`, `cancelled`. Load/verify/link failures occur before invocation. Host APIs expose the same model and must not collapse failures into host exceptions.

Suspended computations are opaque resumable handles to hosts. Host API exposes resume, cancel, close/drop, status, and outcome. Cancellation is cooperative at safepoints/yield points first; forced close exists only for teardown. Cancellation runs pending defers before `cancelled` unless cleanup traps/fails.

Nested defer cleanup order is lexical LIFO for normal return, `leave`, `cycle`, cancellation, and close. Trap/abort cleanup remains separately specified.

## 5. Musi, Packages, FFI, Dynamic Semantics

Musi remains flagship source language. Human-first, infix, expression-shaped.

Musi lowers directly to `.seam`. Musi docs explain stack effects through SEAM bytecode text/disassembly + verifier behavior, not by forcing Forth-like source.

Source package canonical format: `musi.json` + `.ms` files. Import syntax uses ESM-like string paths: `import "path/to/file"`. `musi.json` owns package metadata, `imports`, `exports`, dependencies, and policy knobs.

Bare specifiers/package names resolve through manifest `imports`/`dependencies`. `musi:` is reserved like `node:`/`bun:`; user packages and import maps cannot shadow it. Source `export` controls module surface; manifest `exports` controls package public surface.

Extensionless imports are policy/lint controlled. If enabled, `./foo` resolves to `./foo.ms`; directory fallback to `./foo/index.ms` is a default fallback only when no file match exists.

`.seam` is build/cache/distribution artifact. Package/container transport outside core SEAM bytecode owns compression, checksums, signatures, resources, and multiple images.

Host-provided modules participate in the package graph as explicit nodes with provider and capability metadata.

Musi low-level memory names: `Address`, `Region`, `Access[T]`, `Access[mut T]`. `MutAccess[T]` and `OpaqueAccess[T]` are source DRY aliases only. No source `Ptr`/`Pointer`; SEAM bytecode may still use VM `(ptr T)` and `Ptr[T]` notation.

`@extern` is metadata/attribute, not keyword. Direction is visible from body + export:

```musi
@extern(abi := .c, symbol := "foo")
let foo(value : CInt) : CInt;

@extern(abi := .c, symbol := "foo")
export let foo(value : CInt) : CInt := value;
```

`@extern` without body imports from host/native/foreign code. `@extern export let ... := ...` exports a Musi implementation outward. `@extern let ... := ...` without `export` is diagnostic.

Callbacks from host into Musi are exported Musi functions passed by symbol/handle through host embedding API; no callback syntax. Native resources crossing FFI use opaque handles by default; typed `Access[T]`/`Address` only when ABI metadata declares representable memory access. Native calls are failure-capable unless metadata proves otherwise.

Musi dynamic features lower to bounded core SEAM bytecode dynamic mechanisms. Dynamic calls must carry callee, UALO-shaped arg pack, expected signature, result contract, structured failure. Keyed storage is limited to declared key domains. Capability checks define evidence semantics.

Capabilities are first-class non-forgeable runtime values plus metadata requirements. Capability requirements appear in SEAM bytecode/module metadata, not new Musi syntax for now. Host resource handles are values protected by capabilities; identity stays separate from authority.

`Any` does not auto-enable duck lookup. `Address` is non-authoritative by itself; load/store/permission comes from `Region`/`Access`/capability metadata.

`@noalloc` = core allocation contract. Callable performs no managed heap allocation. Not GC-off; does not prevent collection by other threads/runtime unless SEAM later defines stronger scheduling rule.

`@noalloc` callable rejects managed allocation, boxing, managed array/text/object creation, closure allocation, calls to non-`@noalloc`, dynamic calls unless target proven `@noalloc`.

Native imports may be `@noalloc` only by explicit declaration. Wrong declarations are boundary contract violations.

SEAM bytecode preserves `@noalloc` metadata so SEAM/tools can validate allocation-free paths for low-latency code, FFI callbacks, runtime internals.

Tool metadata optional + non-semantic. May preserve Musi source spans, comments, docs, grouping, import/export shape, spelling, decompilation hints. Cannot affect execution.

Known-phase execution deterministic. No ambient time/random/process/env/filesystem/network/IO unless explicit deterministic import provides it.

Standard native modules provide filesystem, process, time, randomness, text, encoding, system integration. Library/native surface, not hidden SEAM bytecode semantics.

## 6. Gap Resolution Policy

Every unknown resolves to one category:

- core SEAM bytecode/SEAM behavior
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
- host embedding API detail
- native/foreign ABI descriptors
- package/archive/container format beyond loose `musi.json` + `.ms` + `.seam`
- Musi-to-SEAM-bytecode lowering patterns
- tool metadata payloads for source maps + decompilation

Detail gaps only, not platform identity gaps. Fold into owning specs.
