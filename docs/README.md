# MSR1 package

This package contains the consolidated normative candidate for **MSR1 — Musi Standard Report, Revision 1: The Musi Programming Language and Common Portable Code**.

`MSR1.md` is the sole normative authority in this package. The files under `grammar/` are convenience extracts of normative Annexes A and B; if an extraction error ever causes a difference, the annex text in `MSR1.md` controls.

## Agent-indexable topic views

The bundled report is split into focused, read-only topic views under [`msr1/`](msr1/). These views preserve the source material and are not independent normative authorities.

- [`msr1/00-overview.md`](msr1/00-overview.md) — status, scope, authority, and capability scaling
- [`msr1/01-musi-source-language.md`](msr1/01-musi-source-language.md) — Musi source language
- [`msr1/02-common-portable-code.md`](msr1/02-common-portable-code.md) — CPC semantics and canonical text
- [`msr1/03-target-abi-and-correspondence.md`](msr1/03-target-abi-and-correspondence.md) — target/ABI contracts and Musi-to-CPC correspondence
- [`msr1/04-execution-and-conformance.md`](msr1/04-execution-and-conformance.md) — execution and conformance
- [`msr1/05-annex-c-core-bindings.md`](msr1/05-annex-c-core-bindings.md) — core semantic binding closure

## Grammar extracts

- [`grammar/musi.ebnf`](grammar/musi.ebnf) — Musi grammar (Annex A)
- [`grammar/cpc.ebnf`](grammar/cpc.ebnf) — CPC textual grammar (Annex B)

Design state:

- established/current: previously closed Musi source syntax and semantics retained by MSR1;
- established/current: CPC semantic core derived from the previously closed generalized portable machine/code design;
- current: MSR1 consolidation and naming;
- current: bidirectional FFI/re-entry closure, target/ABI schemas, canonical core binding closure, source→CPC correspondence, monotonic capability scaling, and full-self-hosting criteria;
- open before final publication: implementation evidence only. A demonstrated contradiction, infeasible mandatory constrained-target cost, or missing irreducible semantic capability reopens the relevant MSR1 clause.
