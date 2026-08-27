> This agent-indexable topic view is extracted from [`../MSR1.md`](../MSR1.md). `MSR1.md` remains the sole normative authority.

# Part V — Program execution and conformance

## V.1 Initialization and entry

Musi does not require a source binding named `main`.

Reachable modules initialize once according to Part I dependency-first, top-to-bottom rules. A target/environment contract identifies the event or binding by which execution first enters a complete program, library, reset handler, interrupt handler, or other deployment unit.

Ordinary return from an environment-designated entry has the environment-defined consequence stated by the selected contract. A `trap` remains a defined terminal Musi/CPC outcome; the final machine/environment manifestation of that trap is target/environment-defined.

## V.2 Conformance classes

MSR1 defines these conformance classes:

1. **conforming Musi program** — source accepted by MSR1 under its selected contracts;
2. **conforming Musi implementation** — accepts valid MSR1 source, rejects invalid source, and preserves all required observable semantics;
3. **conforming CPC producer** — emits only valid CPC preserving the source/producer semantics claimed for it;
4. **conforming CPC consumer** — validates CPC and preserves CPC semantics when interpreting or translating it;
5. **fully self-hosting Musi implementation** — satisfies V.3 in addition to conforming Musi implementation requirements.

## V.3 Full self-hosting

A fully self-hosting implementation shall be expressible in conforming Musi source and, using only MSR1-defined facilities plus explicitly selected target/ABI capabilities, shall be capable of implementing the complete language/CPC toolchain needed for its claimed deployment path, including:

- source reading;
- lexing;
- parsing;
- semantic and type checking;
- compile-known evaluation;
- representation/layout processing through target-contract facts;
- CPC emission;
- CPC parsing and verification;
- CPC interpretation and/or native translation.

No undocumented source-visible intrinsic or compiler-private semantic operation may be required.

Bootstrap closure is tested as:

```text
B  = bootstrap implementation
C1 = B(compilerSource)
C2 = C1(compilerSource)
```

For a canonical CPC-emitting self-host path, normalized semantic CPC emitted by `C1` and `C2` from the same inputs shall be identical. Differences are permitted only in fields MSR1 explicitly classifies as nonsemantic metadata.

If implementation of the complete compiler/toolchain demonstrates that an irreducible operation cannot be expressed using MSR1 facilities, MSR1 is incomplete and shall be revised before final publication rather than extending one compiler privately.

## V.4 Bounded implementation requirements

A CPC reader/verifier shall be implementable top-to-bottom without mandatory whole-program AST or CFG reconstruction. A small consumer may retain declaration tables required by referenced IDs and current-function verification state. CPC input shall be incrementally readable from ROM, flash, banked storage, or another target-defined non-RAM store; consumers may translate it to any more compact private representation without changing CPC semantics.

The language and CPC do not require the self-hosting compiler itself to execute on the smallest deployable target. The constrained-first requirement concerns semantic/runtime feasibility and toolchain architecture, not a requirement that a full compiler fit into 64 KiB.

## V.5 Closure criterion

For every programmer-observable consequence, MSR1 shall identify exactly one authority: MSR1 semantics, the selected target contract, the selected ABI contract, or explicit raw external behavior. No implementation choice constitutes semantic authority.

MSR1 is publication-complete only when its reference/conformance work demonstrates:

- bidirectional foreign call/data interoperability under at least one concrete ABI contract;
- foreign callback/re-entry behavior;
- a compiler and CPC toolchain written in Musi without private semantic intrinsics;
- CPC exchange with an independent producer or consumer;
- a freestanding deployment path;
- a constrained implementation path consistent with Part 3;
- rejection tests for every normative invalid-program and invalid-CPC category exercised by the suite.

---
