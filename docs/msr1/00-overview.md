# MSR1 — Musi Standard Report, Revision 1

> This agent-indexable topic view is extracted from [`../MSR1.md`](../MSR1.md). `MSR1.md` remains the sole normative authority.

**The Musi Programming Language and Common Portable Code**

Status: **normative candidate for implementation closure**
Identifier: **MSR1**
Language revision: **1**
CPC revision: **1**

## Foreword

MSR1 is the single normative report for Musi Revision 1 and Common Portable Code Revision 1. It defines the source language, its implementation-independent semantics, the CPC portable semantic form and canonical interchange representation, the Musi-to-CPC correspondence, target and ABI contract requirements, and conformance including full self-hosting.

No compiler, backend, runtime, operating system, object format, foreign language, or implementation is a semantic authority. Programmer-observable variation is valid only where MSR1 assigns it to an explicitly selected target contract, ABI contract, or raw external transaction.

## 1. Scope

MSR1 specifies:

- Musi source syntax and semantics;
- compile-known and runtime semantic boundaries;
- memory, lifetime, representation, atomic, interrupt, module, and foreign-boundary semantics;
- Common Portable Code (CPC), including its abstract machine, verifier, and canonical portable text representation;
- the semantic correspondence required of a Musi-to-CPC producer;
- the contract schemas that determine target-defined and ABI-defined behavior;
- freestanding, constrained-first, monotonic-capability, and full-self-hosting conformance.

Package managers, project manifests, editor integrations, test-runner interfaces, and ordinary library APIs are outside MSR1 unless a facility is explicitly named as normative by this report.

## 2. Normative language and authority

The terms **shall**, **shall not**, **may**, **defined**, **target-defined**, **foreign-defined**, **invalid program**, **trap**, and **raw external behavior** are normative. **Should** is informative guidance and shall not alter semantics.

Authority order is:

1. MSR1 normative clauses;
2. MSR1 normative annexes;
3. a target contract conforming to the MSR1 target-contract schema, for facts explicitly delegated to it;
4. an ABI contract conforming to the MSR1 ABI-contract schema, for facts explicitly delegated to it;
5. raw external behavior explicitly requested by the program.

If two normative MSR1 clauses conflict, the report is defective. An implementation shall not choose one interpretation and call that choice implementation-defined behavior.

## 3. Constrained-first and monotonic capability scaling

Musi and CPC are specified implementation-up: from freestanding resource-constrained systems toward larger systems.

A conforming implementation shall not require an operating system, virtual memory, MMU, garbage collector, global allocator, scheduler, threads, dynamic linker, exception unwinder, JIT, resident compiler, filesystem, clock, or large runtime merely to implement semantics that do not request those facilities.

Approximately 64 KiB-class devices are an explicit feasibility floor for suitably selected programs and implementations; 400–512 KiB microcontrollers are intended to be comfortable targets. Exact image size is target- and implementation-dependent and is not itself a conformance requirement.

**Monotonic capability rule.** An implementation may add target capabilities, resources, optimizations, libraries, and hosted services. It shall not weaken, remove, reinterpret, or subset semantics required by MSR1. A CPC consumer whose capabilities are insufficient for a module shall reject that module before execution rather than silently reduce its semantics.

Unused semantic facilities shall impose no mandatory general runtime machinery. This is the zero-hidden-cost rule applied to both Musi and CPC.

---
