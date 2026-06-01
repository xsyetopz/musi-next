# Musi / SEAM Formal Documentation Index

Status: navigational index for humans and AI agents.

## Locked source syntax and core semantics

- `specs/01-lexical-syntax-and-keywords.md` — reserved words, non-keywords, sigil ownership, parser contract.
- `specs/02-delimiters-datum-and-positions.md` — datum delimiters and position rules.
- `specs/03-bindings-sequencing-and-bodies.md` — `let`, semicolons, final-expression bodies.
- `specs/05-conditionals-guards-and-absence.md` — `when` / `else`, Maybe absence.
- `specs/06-patterns-and-match.md` — patterns and `match`.
- `specs/07-functions-callables-and-lambdas.md` — function and callable forms.
- `specs/08-data-traits-evidence-and-constraints.md` — `data`, `trait`, evidence, constraints.
- `specs/09-operators-membership-and-casts.md` — operators, `in`, casts.
- `specs/10-control-flow-defer-and-yield.md` — `while`, `exit`, `next`, `defer`, `yield`.
- `specs/11-modifiers-and-consequence-words.md` — `known`, `fixed`, `pin`, `unsafe`, and related consequence words.

## 1.2 additions

- `notes/1.2-locked-decisions.md` — concise lock index for the latest decisions.
- `specs/21-type-system-gradual-any-unknown-empty.md` — gradual typing, `Any`, `Unknown`, `Empty`, inference defaults.
- `specs/22-staging-known-syntax-values.md` — `known`, `~`, `splice`, runtime syntax values.
- `specs/23-seam-seil-sebc-boundary.md` — SEAM/SEIL/SEBC roles and non-decisions.
- `specs/24-embedded-systems-acceptance-checklist.md` — rejection checklist for embedded-system suitability.
- `specs/25-ffi-interop-contract-expanded.md` — expanded Host/Root/RawPtr/raw FFI contract.
- `specs/26-source-syntax-examples-and-anti-examples.md` — valid/invalid source examples for agents.

## Interop/runtime addendum

- `specs/17-runtime-memory-immix-and-pinning.md` — managed movement, pinning, raw pointer limits.
- `specs/18-host-interop-and-raw-ffi.md` — host interop vs raw FFI.
- `specs/19-retention-callbacks-and-boundary-entry.md` — retention/callback rules.
- `specs/20-external-layout-and-representation.md` — external layout only.

## AI-agent guardrails

- `notes/agent-do-not-invent-index.md` — constructs and names agents must not invent.
- `notes/checklist-and-agent-guardrails.md` — broader design/agent guardrails.
- `notes/invalid-inferences-and-non-goals.md` — invalid inferences and unresolved areas.
- `notes/rejected-constructs.md` — rejected keywords/spellings.
- `notes/locked-decision-index.md` — original lock index updated with 1.2 terminology.

## Migration notes

- `migration/1.0-to-1.1-delta.md`
- `migration/1.1-to-1.2-delta.md`

## Important non-decision

SEIL/SEBC concrete syntax and binary encoding are not locked in this archive. The archive locks their roles, constraints, and boundaries only. Do not infer SEIL instruction spelling or SEBC opcode layout from discussion examples.
