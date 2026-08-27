> This agent-indexable topic view is extracted from [`../MSR1.md`](../MSR1.md). `MSR1.md` remains the sole normative authority.

# Part III — Target and ABI contracts

## III.1 Contract rule

Target and ABI contracts are data satisfying schemas defined by MSR1, not separate language standards. Their concrete storage/registry mechanism is tooling. A contract shall have a stable ASCII identity and revision.

MSR1 may delegate an observable consequence to a contract only where this Part defines the corresponding semantic field. Missing required contract information makes the selected configuration invalid; compiler choice shall not fill the gap.

## III.2 Target contract schema

A target contract shall determine, where applicable:

- identity and revision;
- compatible MSR/CPC revisions;
- address-space and raw `Address` representation facts;
- addressable unit and pointer/address widths where meaningful;
- supported physical integer and floating representations;
- natural alignment and aggregate-layout rules delegated by Part I;
- section identities and fixed-placement guarantees;
- readable/writable/volatile memory-region constraints exposed to compilation;
- supported `Atomic[T]` types, operations, and memory orders;
- interrupt identities, entry constraints, and target-visible interrupt semantics;
- target execution leaves exposed through normative intrinsic bindings;
- CPC capabilities needed to deploy a module;
- entry/reset/environment events supplied by that target environment.

A field is either required, explicitly not applicable, or optional with an MSR1-defined default. Silence is not a semantic choice.

Target contracts shall permit non-flat, banked, Harvard, segmented, or otherwise constrained systems where their raw-address semantics can satisfy Part I. A flat hosted address space is not assumed.

## III.3 ABI contract schema

An ABI contract shall determine, where applicable:

- identity and revision;
- compatible target contracts and MSR/CPC revisions;
- external symbol identity and linkage visibility;
- callable calling convention;
- parameter and result classification/lowering;
- aggregate passing and required foreign representation;
- stack alignment and caller/callee preservation requirements;
- register assignments where semantically required for interoperation;
- raw pointer and foreign function-pointer representation;
- static-data import/export behavior;
- variadic behavior when supported;
- Musi-defined callback entry and any required trampoline behavior;
- foreign re-entry requirements and execution-context establishment;
- foreign unwind/error behavior and Musi trap interaction;
- initialization/finalization hooks required by that ABI environment.

### III.3.1 Bidirectional foreign bindings

`$[foreign(abiDescriptor)]` applies symmetrically to imports and exports.

- A module-scope foreign callable/static-data binding without a Musi definition denotes a foreign-defined import.
- A module-scope foreign callable/static-data binding with a Musi definition denotes a Musi-defined foreign export.
- `export` controls Musi module visibility; the ABI descriptor controls foreign linkage visibility/symbol identity. Where both are required, both shall be explicit.

A Musi-defined foreign callable entry is non-yielding. A `~>` callable cannot itself be a direct foreign ABI entry. Foreign entry may invoke ordinary `->` Musi code.

Foreign re-entry is permitted only when the selected ABI/environment contract establishes a valid Musi execution context for that entry. MSR1 does not imply threads or an operating system.

A foreign pointer is `Address` unless the ABI contract explicitly establishes a stronger wrapper whose object/lifetime facts satisfy Part I. Mere receipt of a foreign pointer never creates safe `Access`.

Foreign exception/unwind state shall not cross Musi frames unless the ABI contract defines a total MSR1-compatible mapping. A Musi `trap` is a terminal Musi outcome and does not implicitly perform foreign unwinding.

---

# Part IV — Musi to CPC semantic correspondence

A conforming Musi-to-CPC producer shall emit a CPC module whose programmer-observable behavior is equivalent to the Musi program under the same selected target and ABI contracts.

The producer is free to optimize and is not required to emit a prescribed instruction sequence. It shall preserve at least:

- strict evaluation order and discarded-value semantics;
- binding, initialization, and module dependency order;
- exact type identity and compile-known decisions;
- checked integer/floating semantics;
- places, safe `Access`, raw `Address`, and volatile distinction;
- `Storage` lifetime establishment/end and invalid accesses;
- representation constraints visible in source;
- choice/tag semantics;
- control flow and `defer` effects;
- plain versus yielding callable effects and suspension points;
- atomic operations and memory orders;
- foreign boundaries and ABI-visible representations;
- trap outcomes.

A producer may erase abstractions, scalarize aggregates, fold compile-known computation, eliminate proved checks, change calling convention internally, or otherwise transform code only when the resulting CPC preserves these semantics.

---
