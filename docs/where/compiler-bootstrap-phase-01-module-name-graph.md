# Compiler Bootstrap Phase P01 — Module and Name Graph Contracts

Status: `[ ]` not started
Depends on: `docs/where/compiler-bootstrap-phase-00-syntax-contracts.md`
Next phase: P02 typed-surface tracker (to create after P01 completion)

## Objective

Define a stable bootstrap contract for module/import graph and symbol binding surfaces without crossing into full semantic checking.

## Deliverables

- [ ] Produce producer/contract/consumer map for:
  - `music_module`
  - `music_resolve`
- [ ] Define module key, import edge, and export surface contract shapes required by bootstrap compiler modules.
- [ ] Define symbol-table and name-binding contract shapes required before semantic checks.
- [ ] Map current diagnostic enum ownership for resolver-stage failures.
- [ ] Record unresolved package naming or layering decisions as explicit decision items.

## Non-goals

- [ ] `music_sema` type/effect checking migration.
- [ ] `music_ir` or `music_emit` lowering/emission migration.
- [ ] Runtime/native ABI changes.

## Validation gates

- [ ] `cargo test -p music_module --lib`
- [ ] `cargo test -p music_resolve --lib`
- [ ] Confirm phase dependencies stay acyclic with `docs/where/workspace-map.md`.

## Exit criteria

- [ ] P01 deliverables are all `[x]`.
- [ ] P01 non-goals remain unchanged.
- [ ] P01 validation gates are all `[x]`.
