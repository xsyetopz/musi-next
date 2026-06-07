# Locked SEIL/SEAM Design

Status: locked platform design for SEIL, SEAM, and Musi's boundary with them. This document resolves the high-level design gaps before details are folded into the owning language, SEIL, and SEAM specifications.

## 1. Platform Identity

SEAM is the runtime platform. It loads, verifies, links, initializes, and executes SEIL modules.

SEIL is the final portable executable artifact for SEAM. It is CIL-like in architectural role: a source-to-runtime artifact, not compiler-private IR. SEIL modules are final compiler outputs and loadable runtime inputs.

Musi is the flagship source language for SEAM. Musi lowers directly to SEIL. There is no hidden IR layer between Musi and SEIL.

SEIL stack effects belong to verification and execution. Musi source may remain infix and expression-shaped because source syntax serves humans; SEIL exposes the stack machine directly.

SEIL/SEAM have no dialect escape hatch for executable semantics. A feature is core, library/native, frontend-owned, or unsupported. Unknown executable semantics are rejected.

## 2. Artifact And Metadata

`.seil` is the canonical textual executable module artifact. It is hand-writable, but it is not Musi source.

SEAM tooling may assemble `.seil` into an internal binary image for loading, caching, package transport, or execution. That binary image starts with a fixed probe header. The header identifies the container and section directory only. Semantic compatibility does not live in the header.

Semantic module data lives in mandatory metadata tables and executable bodies. Core module metadata includes:

- module identity and version
- imports and exports
- type table
- signature table
- layout table
- constants
- procedure declarations
- body metadata
- executable bodies

Unknown semantic sections, tables, metadata, or opcodes are rejected.

Tool metadata is optional. It can preserve source spans, original names, comments, docs, import/export shape, source grouping, and decompilation hints. It cannot affect execution, verification, linking, or runtime behavior.

Package and archive formats are containers over SEIL modules. They are separate from core SEIL module semantics.

Assembly, module, and package naming is canonical and case-sensitive. Tools must not case-fold, Unicode-normalize, dash-convert, or otherwise rewrite logical names.

## 3. Execution Semantics

SEIL is a typed stack-effect executable language.

SEIL text combines structured declarations with stack instruction bodies:

- declarations use WAT/Lisp-like parenthesized forms
- executable bodies use line-oriented RPN/Forth-like instructions

This gives SEIL a readable assembly form while preserving exact VM behavior.

Every opcode has:

- stable numeric id
- canonical mnemonic
- immediate operand schema
- stack-effect schema
- verification behavior
- runtime behavior

The verifier tracks stack height and value types at every instruction. A procedure body has a declared signature with inputs and outputs. `ret` must match the declared outputs.

Branch targets have an incoming stack shape. All incoming edges to the same target must be compatible with that shape. No hidden coercion is inserted at joins; required conversion must appear explicitly in code.

Core value forms include:

- scalars
- products
- sums
- indexed storage and arrays
- callable values
- boxes
- managed references
- unsafe pointers

Products and sums are core data forms. SEIL does not have a mandatory CLR/JVM-style class-object center.

Core call/control forms include direct calls, indirect calls, receiver dispatch, dynamic calls, branches, table branches, returns, traps, throws, rethrows, cleanup edges, and yield/suspend behavior.

Dynamic mechanisms are core if present. They must have explicit callee, argument, key, evidence, result, and failure contracts.

Verification rejects unknown opcodes, malformed operands, stack underflow, stack type mismatch, invalid branch joins, missing metadata, invalid reference or pointer use, invalid calls, and unsupported semantic module data.

## 4. Runtime Semantics

SEAM executes modules through this order:

1. read fixed header
2. read section directory
3. read mandatory module metadata
4. reject unknown semantic parts
5. decode tables and bodies
6. verify opcode, type, stack, control, and metadata contracts
7. link imports, exports, native procedures, and foreign procedures
8. initialize module
9. execute entry or exported callable

A frame contains:

- callable identity
- instruction position
- argument slots
- local slots
- environment and capture slots
- operand stack
- active cleanup, handler, and yield state

Arguments, locals, environments, and captures are typed slots. The operand stack is typed by verifier state, not by runtime guessing.

SEAM derives GC roots from verified stack maps. Roots include managed references on the operand stack, managed references in frame slots, globals, active exception state, active yield state, and runtime handles.

Safepoints include allocation, calls, dynamic calls, throws, yields, and native/foreign boundaries when the boundary can allocate, block, or call back into SEAM.

Any write of a managed reference into heap, global, array, boxed, or other reference-bearing storage must execute the required write barrier. Raw byte writes cannot smuggle managed references.

`fixed` means a stable address is required for a value or storage region. SEAM may implement this with pinning, nonmoving storage, copying to unmanaged storage, or checked rejection. `fixed` does not disable GC globally.

SEAM outcomes are:

- success with outputs
- trap or structured failure with reason
- load, verify, or link failure
- suspended/yielded state

Host embedding APIs expose the same outcome model. They must not collapse all failures into host exceptions.

SEAM has one core C-compatible FFI bridge:

- SEIL calls native code through import metadata
- native code calls SEIL through exported callable tables
- managed values cross the boundary as handles unless represented, fixed, or copied
- callbacks enter through SEAM trampolines so frames, roots, safepoints, and failures remain valid

## 5. Musi, Dynamic Semantics, And Metadata

Musi remains the flagship source language. Musi syntax remains human-first, infix, and expression-shaped.

Musi lowers directly to SEIL. Musi documentation explains stack effects through SEIL text and verifier behavior, not by forcing Musi source to look like Forth.

Musi dynamic features lower to bounded core SEIL dynamic mechanisms. Dynamic calls must carry explicit callee, argument pack, expected signature, result contract, and structured failure behavior. Keyed access must define key and value constraints. Capability checks must define evidence semantics.

`@noalloc` is a core allocation contract.

`@noalloc` means a callable performs no managed heap allocation. It does not disable GC globally and does not prevent collection caused by other threads or runtime activity unless SEAM later defines a stronger scheduling rule.

A `@noalloc` callable rejects:

- managed allocation
- boxing
- managed array, text, or object creation
- closure allocation
- calls to non-`@noalloc` procedures
- dynamic calls unless the target is proven `@noalloc`

Native imports may be marked `@noalloc` only by explicit declaration. Incorrect declarations are boundary contract violations.

SEIL preserves `@noalloc` callable metadata so SEAM and tools can enforce or validate allocation-free paths for low-latency code, FFI callbacks, and runtime internals.

Tool metadata remains optional and non-semantic. It may preserve Musi source spans, comments, docs, source grouping, import/export shape, original spelling, and decompilation hints. It cannot affect execution.

Known-phase execution is deterministic. It has no ambient access to time, randomness, process state, environment variables, filesystem, networking, or IO unless an explicit deterministic import provides that behavior.

Standard native modules provide runtime services such as filesystem, process, time, randomness, text, encoding, and system integration. They are library/native surface, not hidden SEIL semantics.

## 6. Gap Resolution Policy

Every remaining unknown must be resolved into one of four categories:

- core SEIL/SEAM behavior
- library or native module behavior
- frontend-owned behavior
- unsupported behavior

No gap may remain assigned to an executable-semantics dialect.

Core gaps that still need detailed specification:

- physical binary table layouts
- full verifier rule tables
- branch join compatibility rules
- dynamic call, keyed storage, and capability schemas
- trap and structured failure reason enum
- frame and object layout contracts
- GC barrier and safepoint rule tables
- host embedding API
- native/foreign ABI descriptors
- module/package discovery and archive format
- Musi-to-SEIL lowering patterns
- tool metadata payloads for source maps and decompilation

These are detail gaps, not platform identity gaps. They must be folded into the owning specs after this locked design.
