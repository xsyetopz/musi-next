# Musi Formal Source Spec 1.2 — SEAM / Type / Staging / Embedded Contract Addendum

Status: formal addendum archive built from the 1.1 interop/runtime/layout pack plus later locked design decisions.

This archive is written for both humans and AI agents. It is intentionally explicit about what is locked, what is rejected, and what remains unspecified.

## Main lock set

```text
known   = compiler-known / known during compilation
fixed   = fixed storage / fixed placement / fixed lifetime
pin     = temporary pinning action for address stability
unsafe  = unchecked/manual obligation boundary
```

```text
#       = datums
$       = template literal interpolation
~       = syntax quote / in-template splice
splice  = explicit source-position insertion
```

```text
Any     = runtime top value carrier
Unknown = checker/frontend imprecision, not verified bytecode
Empty   = bottom / uninhabited / unreachable
```

## VM stack

```text
Musi source -> Musi Core -> SEIL -> SEBC -> SEAM
```

```text
SEAM = Stack Effect Abstract Machine
SEIL = Stack Effect Intermediate Language
SEBC = Stack Effect Bytecode
```

SEIL is intended as a small fixed Lisp-shaped/S-expression VM language, but concrete SEIL syntax is not locked in this archive. SEBC binary encoding is also not locked.

## Embedded-safety rule

A design is rejected if it allows hidden `Any`, hidden dynamic dispatch, hidden allocation, hidden host authority, hidden raw pointer retention, unverifiable pin/root lifetimes, or unresolved `Unknown` in SEIL/SEBC.

See `specs/24-embedded-systems-acceptance-checklist.md`.

## New 1.2 files

- `INDEX.md`
- `notes/1.2-locked-decisions.md`
- `notes/agent-do-not-invent-index.md`
- `specs/21-type-system-gradual-any-unknown-empty.md`
- `specs/22-staging-known-syntax-values.md`
- `specs/23-seam-seil-sebc-boundary.md`
- `specs/24-embedded-systems-acceptance-checklist.md`
- `specs/25-ffi-interop-contract-expanded.md`
- `specs/26-source-syntax-examples-and-anti-examples.md`
- `migration/1.1-to-1.2-delta.md`

## Status discipline

- Existing syntax is syntax already present in the spec files or explicitly locked in 1.2.
- Confirmed semantic rules are locked where marked normative.
- Candidate syntax must be marked candidate.
- No source-level syntax is implied by a semantic rule unless a syntax/spec chapter explicitly states it.
- No SEIL/SEBC concrete syntax or encoding is implied by the SEAM boundary chapter.
