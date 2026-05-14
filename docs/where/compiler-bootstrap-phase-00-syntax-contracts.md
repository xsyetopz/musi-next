# Compiler Bootstrap Phase P00 — Syntax Contract Map

Status: `[~] in progress`
Depends on: `docs/where/bootstrap-bytecode-ledger.md` bootstrap boundary
Next phase: `docs/where/compiler-bootstrap-phase-01-module-name-graph.md`

## Objective

Establish the smallest compiler bootstrap contract slice for syntax-facing data and parser-adjacent helpers before moving to resolver or semantic phases.

## Deliverables

- [ ] Produce producer/contract/consumer map for syntax-facing crates:
  - `music_base`
  - `music_names`
  - `music_syntax`
  - `music_module`
- [ ] Identify syntax data shapes that can move to Musi compiler-side modules without importing runtime execution strategy.
- [ ] List host-only APIs that remain Rust-owned in this phase.
- [ ] Define initial contract set for:
  - token and trivia views
  - syntax node and span views
  - module key and module-source records
- [ ] Record unresolved design points as explicit decision items.

## Non-goals

- [ ] Parser rewrite.
- [ ] Resolver or semantic algorithm migration.
- [ ] Runtime or VM contract changes.
- [ ] New package graph finalization.

## Validation gates

- [ ] `cargo test -p music_syntax --lib`
- [ ] `cargo test -p music_module --lib`
- [ ] Confirm contract map references current syntax canon files and `docs/where/workspace-map.md`.

## Exit criteria

- [ ] P00 deliverables are all `[x]`.
- [ ] P00 non-goals remain unchanged.
- [ ] P00 validation gates are all `[x]`.
