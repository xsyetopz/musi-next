# music_* Compiler Bootstrap Roadmap

Status: `[~] active`
Scope: compiler-side bootstrap preparation for `music_*` phases.

Status legend:

- `[ ]` not started
- `[~]` in progress
- `[x]` complete

Source anchors:

- syntax canon: `grammar/MusiParser.g4`, `grammar/MusiLexer.g4`, `grammar/Musi.abnf`
- phase ownership: `docs/where/workspace-map.md`
- bootstrap boundary: `docs/where/bootstrap-bytecode-ledger.md`
- lowering contracts: `specs/seam/lowering.md`

## Phase ladder

| Phase | Contract focus                                        | Primary crates                                              | Tracker file                                                  | Status |
| ----- | ----------------------------------------------------- | ----------------------------------------------------------- | ------------------------------------------------------------- | ------ |
| P00   | Syntax and parser-facing contract map                 | `music_base`, `music_names`, `music_syntax`, `music_module` | `docs/where/compiler-bootstrap-phase-00-syntax-contracts.md`  | `[~]`  |
| P01   | Module/import graph and symbol binding contract map   | `music_module`, `music_resolve`                             | `docs/where/compiler-bootstrap-phase-01-module-name-graph.md` | `[ ]`  |
| P02   | Typed surface and diagnostic contract map             | `music_sema`                                                | _to create after P01_                                         | `[ ]`  |
| P03   | IR construction and lowering boundary contract map    | `music_ir`, `music_ir_lower`                                | _to create after P02_                                         | `[ ]`  |
| P04   | SEAM emission boundary contract map                   | `music_emit`, `music_seam`                                  | _to create after P03_                                         | `[ ]`  |
| P05   | Session orchestration and project integration handoff | `music_session`, `musi_project`, `musi_foundation`          | _to create after P04_                                         | `[ ]`  |

## Cross-phase invariants

- [ ] Keep compiler phase dependencies acyclic: `music_base -> music_names -> music_syntax -> music_module -> music_resolve -> music_sema -> music_ir`, then `music_ir -> music_emit -> music_session`.
- [ ] Keep runtime-native ownership in Rust host/runtime crates; bootstrap scope stays compiler-side.
- [ ] Record package names and package graph decisions only through explicit decision docs.
- [ ] Keep each phase tracker tied to concrete crate surfaces and concrete validation gates.
- [ ] Advance to next phase only after current tracker exit criteria are marked `[x]`.

## Immediate start point

- [~] Execute P00 tracker and capture first contract matrix for syntax-facing bootstrap surfaces.
