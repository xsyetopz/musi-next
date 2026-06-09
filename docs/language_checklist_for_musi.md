# Programming Language Checklist — Musi Answered Version

Source checklist: `docs/language_checklist.md`.

Keeps checklist shape; answers for Musi from `LOCKED_LANGUAGE_DESIGN.md`. Legend: `[x]` locked/answered; `[-]` rejected/not claimed; `[~]` implementation/spec evidence pending; `[?]` evidence/decision missing.

## You appear to be advocating a new:

- [x] functional
  - Expressions, lambdas, `match`, algebraic `data`, `Maybe`, `Expect`, type/value composition.
- [x] imperative
  - `while`, `leave`, `cycle`, `defer`, `mut`, `:=`, access/FFI capabilities.
- [-] object-oriented
  - No classes/inheritance/`this`/`new`/`impl`/class bodies. Receiver methods use `let` + UDNS/UFCS.
- [x] procedural
  - Callable bindings + receiver methods cover procedure/function organization.
- [x] stack-based
  - SEIL stack-effect interpreted. Musi source hides SEIL stack syntax; callable types use `(A, B) -> C`.
- [x] "multi-paradigm"
  - Expression-first syntax, algebraic data, imperative control, structural shapes, receiver methods, FFI. Not novelty claim; small-core systems design.
- [-] lazy
  - Not globally lazy. `??` fallback lazy; other laziness explicit via control/effects.
- [x] eager
  - Runtime eager unless `??`, `when`, `match`, or `yield` defines control.
- [x] statically-typed
  - Bidirectional inference, explicit `Any`, `Type[N]`, `Empty`, `Unit`, `Error`, type algebra, casts/tests locked.
- [x] dynamically-typed
  - Only explicit `Any`. Missing annotations do not mean dynamic.
- [-] pure
  - No purity claim; FFI, access caps, mutation, `defer`, `yield` in model.
- [x] impure
  - Effects + dangerous capability ops explicit and checked.
- [-] non-hygienic
  - No macro system locked.
- [-] visual
  - Not a visual language.
- [-] beginner-friendly
  - Not claimed.
- [-] non-programmer-friendly
  - Not claimed.
- [-] completely incomprehensible
  - Maximal munch, one-token lookahead, UALO, UDNS, postfix `when`, small hard keyword set.

## Your language will not work. Here is why it will not work.

## You appear to believe that:

- [-] Syntax is what makes programming difficult
  - Syntax tied to behavior: `known` phase, `fixed` stable storage, `unmanaged` non-managed representation, `Any` dynamic, `@extern` FFI, UDNS/UALO deterministic resolution.
- [-] Garbage collection is free
  - `fixed` exists because moving GC can invalidate address/access. Managed address/access derivation requires fixed storage.
- [-] Computers have infinite memory
  - Known eval deterministic/resource-limited: fuel, steps, memory.
- [-] Nobody really needs:
    - [x] concurrency
      - `yield` core suspension keyword. `Task`, `Scheduler`, `Resumable`, `Generator`, `Stream` library/runtime names.
    - [~] a REPL
      - Not syntax-locked; source→SEIL + known-phase SEIL do not block it.
    - [~] debugger support
      - SEIL metadata preservation goal; debugger still tooling work.
    - [x] IDE support
      - One-token parsing, comments, UALO, UDNS, metadata preservation help tooling.
    - [~] I/O
      - I/O is library/runtime design; FFI/imports allow it.
    - [x] to interact with code not written in your language
      - `@extern`, `@repr`, C ABI aliases, `unmanaged`, `Address`, `Region`, `Access[T]`, `Access[mut T]`, `MutAccess[T]`, and `OpaqueAccess[T]` are locked.
- [-] The entire world speaks 7-bit ASCII
  - No ASCII-only policy.
- [-] Scaling up to large software projects will be easy
  - Not claimed. Modules records; import/export locked.
- [-] Convincing programmers to adopt a new language will be easy
  - Not claimed.
- [-] Convincing programmers to adopt a language-specific IDE will be easy
  - No language-specific IDE required.
- [-] Programmers love writing lots of boilerplate
  - `let`, inference, generic inference, UALO, UDNS/UFCS, datum syntax reduce boilerplate without hiding boundaries.
- [-] Specifying behaviors as "undefined" means that programmers won't rely on them
  - Dangerous behavior = error. Invalid access/address/FFI/layout/phase/dynamic failure = diagnostic/error.
- [-] "Spooky action at a distance" makes programming more fun
  - No implicit `Any`, duck-dot, null, exceptions, or UDNS fallback from failed higher priority.

## Unfortunately, your language (has/lacks):

- [x] comprehensible syntax
  - Syntax has parser gates, UALO, UDNS, datum sigils, explicit operator families.
- [x] semicolons
  - Semicolons terminate structural members; sequence/discard in computation regions.
- [-] significant whitespace
  - Not used.
- [-] macros
  - No macro/template/syntax keyword. Type-staged metaprogramming via `known` + type values.
- [-] implicit type conversion
  - No silent `Any`; dynamic→static needs `:?>`; static conversion uses `:>`.
- [x] explicit casting
  - `:>` and `:?>` are locked.
- [x] type inference
  - Bidirectional/generic inference and `_` holes locked. Ambiguity diagnostic.
- [-] goto
  - No `goto`. Loop control is `leave`/`cycle`.
- [-] exceptions
  - No hidden exceptions. Failure uses `Expect[T, E]`; `Error` top error.
- [x] closures
  - Lambda expression syntax exists with `=>`.
- [~] tail recursion
  - No `recur`; ordinary recursion exists. Tail-call guarantees pending implementation/SEIL.
- [x] coroutines
  - `yield` core keyword for resumable/generator contexts.
- [-] reflection
  - Not surface claim.
- [x] subtyping
  - `<:` is locked. `Empty`, `Any`, shapes, opaque/erased, and type algebra participate in the type system.
- [-] multiple inheritance
  - No classes/inheritance. Shape/intersection = capability composition model.
- [-] operator overloading
  - No user symbolic operators in core.
- [x] algebraic datatypes
  - `data` products/sums; `case` variants.
- [~] recursive types
  - Not syntax-only; positivity/layout/size checks need spec/implementation.
- [x] polymorphic types
  - Generics locked: `let name[A, B](value) := ...`, `name[A](value)`.
- [-] covariant array typing
  - Not claimed.
- [-] monads
  - `Maybe`/`Expect` exist; monads not language feature.
- [-] dependent types
  - `known`, `Type[N]`, type-stage programming exist; full dependent types not claimed.
- [x] infix operators
  - Fixed operator set + precedence locked.
- [x] nested comments
  - Nested block/doc/module comments locked with linear depth counter.
- [x] multi-line strings
  - Triple-quoted strings locked. JS templates rejected; interpolation explicit API/library behavior.
- [-] regexes
  - No regex syntax is locked.
- [x] call-by-value
  - Values/calls support value passing; exact ABI lowering pending.
- [-] call-by-name
  - Not claimed.
- [x] call-by-reference
  - No implicit default; reference-like effects need `fixed`, `mut`, access/capability forms, or receiver/capability APIs.
- [-] call-cc
  - Not present.

## The following philosophical objections apply:

- [-] Programmers should not need to understand category theory to write "Hello, World!"
  - Basic Musi code uses `let`, calls, `data`, `match`, and `while`. Advanced type algebra is not required for basic programs.
- [-] Programmers should not develop RSI from writing "Hello, World!"
  - Universal `let`, inference, UDNS, UALO, and small keyword set reduce boilerplate.
- [-] The most significant program written in your language is its own compiler
  - Not claimed.
- [-] The most significant program written in your language isn't even its own compiler
  - Not claimed.
- [-] No language spec
  - `LOCKED_LANGUAGE_DESIGN.md` is the current locked syntax/design artifact. Full SEIL/runtime spec remains to be written.
- [-] "The implementation is the spec"
    - [-] The implementation is closed-source
    - [-] covered by patents
    - [-] not owned by you
  - Syntax and semantics are being documented before implementation. No closed/patent claim.
- [?] Your type system is unsound
  - The design avoids silent dynamic fallback, null, hidden exceptions, and implicit conversions. Formal soundness still needs implementation/proof work.
- [?] Your language cannot be unambiguously parsed
    - [~] a proof of same is attached
    - [-] invoking this proof crashes the compiler
  - The design requires maximal munch and one-token-lookahead only. Parser proof/evidence remains implementation work.
- [?] The name of your language makes it impossible to find on Google
  - Searchability is a claim needing evidence; not assessed here.
- [-] Interpreted languages will never be as fast as C
  - Musi does not claim C speed. It targets SEIL interpretation.
- [-] Compiled languages will never be "extensible"
  - Not relevant as a claim.
- [-] Writing a compiler that understands English is AI-complete
  - No natural-language syntax.
- [-] Your language relies on an optimization which has never been shown possible
  - No such optimization is required by locked syntax.
- [-] There are less than 100 programmers on Earth smart enough to use your language
  - The core syntax is small; advanced features are not needed for basic code.
- [?] ____________________________ takes exponential time
  - Parser design explicitly rejects speculative/more-than-one-token-lookahead forms. Type complement normalization still needs implementation bounds.
- [?] ____________________________ is known to be undecidable
  - Full dependent typing is not claimed; ambiguous/non-normalizable type algebra is diagnostic.

## Your implementation has the following flaws:

- [-] CPUs do not work that way
  - `%` is CPU remainder, not mathematical modulo. Shifts/rotates map to CPU-like operations. Access/fixed rules acknowledge address stability.
- [-] RAM does not work that way
  - `fixed` and access rules account for movable storage and stable addresses.
- [x] VMs do not work that way
  - SEIL is intentionally the VM/lowered form. Instruction model is locked by `seil_opcodes.def`, `grammar/seil.ebnf`, and owning SEIL specs.
- [-] Compilers do not work that way
  - Known evaluation executes SEIL; type/phase/layout/access checks are compiler responsibilities.
- [-] Compilers cannot work that way
  - Syntax is constrained to maximal munch and one-token lookahead; impossible parser forms are rejected by design.
- [?] Shift-reduce conflicts in parsing seem to be resolved using rand()
  - Not allowed by design; parser evidence still required.
- [-] You require the compiler to be present at runtime
  - No. Runtime executes SEIL; known evaluation is compile phase.
- [-] You require the language runtime to be present at compile-time
  - No ambient runtime state is available to `known` code.
- [~] Your compiler errors are completely inscrutable
  - Diagnostic quality requires implementation evidence; repo diagnostic guidance exists outside this file.
- [-] Dangerous behavior is only a warning
  - Dangerous behavior is an error, not a warning.
- [~] The compiler crashes if you look at it funny
  - Requires implementation evidence.
- [~] The VM crashes if you look at it funny
  - SEIL/runtime implementation evidence is still required for performance and conformance; language-level GC direction is specified through managed refs, layouts, safepoints, barriers, and `fixed` storage.
- [?] You don't seem to understand basic optimization techniques
  - The design avoids parser exponentiality, uses CPU-aligned arithmetic semantics, and separates representation metadata. Performance evidence still needed.
- [-] You don't seem to understand basic systems programming
  - `fixed`, `unmanaged`, `Address`, `Region`, `Access[T]`, `Access[mut T]`, `@extern`, `@repr`, C ABI aliases, and representability rules are locked.
- [-] You don't seem to understand pointers
  - `Address`, `Region`, `Access[T]`, and `Access[mut T]` split address, provenance, typed read access, and typed write access. No C `&x`, no C `*p`, no source `Ptr`/`Pointer`, no core pointer arithmetic, stable managed access requires `fixed`.
- [-] You don't seem to understand functions
  - Callable types, lambdas, receiver methods, generics, UDNS/UFCS, and UALO are locked.

## Additionally, your marketing has the following problems:

- [-] Unsupported claims of increased productivity
  - No productivity claim is made.
- [-] Unsupported claims of greater "ease of use"
  - No ease-of-use claim is made.
- [-] Obviously rigged benchmarks
    - [-] Graphics, simulation, or crypto benchmarks where your code just calls handwritten assembly through your FFI
    - [-] String-processing benchmarks where you just call PCRE
    - [-] Matrix-math benchmarks where you just call BLAS
  - No benchmarks are claimed.
- [-] Noone really believes that your language is faster than:
    - [-] assembly
    - [-] C
    - [-] FORTRAN
    - [-] Java
    - [-] Ruby
    - [-] Prolog
  - Musi does not claim to be faster than these.
- [-] Rejection of orthodox programming-language theory without justification
  - Unconventional choices are justified by parser constraints, explicit phase/type/effect rules, null avoidance, and SEIL/FFI goals.
- [-] Rejection of orthodox systems programming without justification
  - Systems rules are explicit: fixed storage, unmanaged storage/representation, access capabilities, addresses, FFI representability, representation metadata.
- [-] Rejection of orthodox algorithmic theory without justification
  - No such rejection.
- [-] Rejection of basic computer science without justification
  - No such rejection.

## Taking the wider ecosystem into account, I would like to note that:

- [-] Your complex sample code would be one line in: _______________________
  - Not claimed. Musi chooses explicit boundaries over minimum character count.
- [-] We already have an unsafe imperative language
  - Musi has imperative systems features but no unsafe block; dangerous operations are typed/capability/metadata governed.
- [-] We already have a safe imperative OO language
  - Musi has no classes or inheritance; receiver methods and shapes are not OO class syntax.
- [-] We already have a safe statically-typed eager functional language
  - Musi is systems/FFI/SEIL oriented with fixed storage and explicit access/address semantics.
- [-] You have reinvented Lisp but worse
  - No macro core, no S-expression syntax, no syntax-as-code claim.
- [-] You have reinvented Javascript but worse
  - No implicit dynamic fallback, no null, no duck-dot on `Any`, no `==`.
- [-] You have reinvented Java but worse
  - No classes/inheritance/null/exceptions model.
- [-] You have reinvented C++ but worse
  - No overloading of symbolic operators by users, no pointer `&`/`*`, no class/constructor/destructor surface.
- [-] You have reinvented PHP but worse
  - No weak truthiness, no silent conversions, no implicit dynamic field access.
- [-] You have reinvented PHP better, but that's still no justification
  - Not applicable.
- [-] You have reinvented Brainfuck but non-ironically
  - Not applicable.

## In conclusion, this is what I think of you:

- [?] You have some interesting ideas, but this won't fly.
  - Musi still needs SEIL/runtime/parser implementation evidence, but the locked syntax addresses the major checklist pitfalls.
- [-] This is a bad language, and you should feel bad for inventing it.
  - The design avoids many known language mistakes: null, `==`, hidden exceptions, implicit dynamic fallback, parser ambiguity, unsafe-warning downgrades.
- [-] Programming in this language is an adequate punishment for inventing it.
  - UALO, UDNS, inference, universal `let`, and explicit capability boundaries are intended to keep ordinary code concise and predictable.

## Advice Document Cross-Check

Source advice: `docs/advice_for_designer_of_my_own_programming_language.md`.

- [x] Make it predictable and deterministic
  - Maximal munch, one-token lookahead, fixed operator set, UALO, UDNS resolution order, no implicit `Any`, no implicit duck-dot.
- [x] Make it easy to reason about for humans and computers
  - `Maybe` for absence, `Expect`/`Error` for failure, `Bit` for guards, no truthiness, no hidden exceptions, explicit casts/tests.
- [x] Avoid global state at all phases
  - Known phase cannot depend on ambient runtime state and is resource-limited.
- [x] Use static types
  - Bidirectional inference and explicit dynamic top `Any`.
- [x] Make it efficient
  - CPU remainder semantics, CPU-like shift/rotate operations, representation metadata, no parser speculation, SEIL executable IL target.
- [x] Use `[]` for generics, not `<>`
  - Locked: `T[A, B]`; array/list types are prefix `[N]T`.
- [x] Treat comments as grammar/trivia with reasonable restrictions
  - Line/doc/module and nested block/doc/module comments are locked.
- [x] Avoid exponential parser behavior
  - Syntax that needs more than one token of lookahead is rejected by design.
- [x] Avoid tree evaluator for known/constexpr evaluation
  - Known functions lower to SEIL; known evaluation executes SEIL.
- [x] Avoid offloading compiler responsibilities to the standard library
  - Type, phase, fixed-storage, unmanaged storage/representation, access/address, FFI, layout, and dangerous-operation checks are diagnostics/compiler responsibilities.
- [x] Understand lvalues vs rvalues
  - `place := expr`, mutable/fixed storage, `Access[T]`, and `Access[mut T]` separate readable/writable locations.
- [x] Consider stack vs register machine tradeoffs
  - Musi intentionally targets locked SEIL stack-effect executable IL.
- [x] Think through shared library paths, TLS, and relocations
  - `@extern` includes `link`, `symbol`, `calling`, ABI descriptor, and representability rules. TLS/relocations remain ABI/runtime implementation details.

## Full Musi Spec Lock Tracker

Purpose: one place to know when Musi spec is fully locked. `[x]` = locked in docs/specs/grammar. `[~]` = direction locked, exact rules/evidence missing. `[?]` = still needs decision or proof.

USER selection gate: `docs/musi_full_spec_solution_selection.md` is the checkbox source of truth. `docs/musi_full_spec_solution_options.md` explains A/B/C as language directions and maps current unknowns. Until one option is checked, remaining `[~]` gaps stay open.

Spec gates:

- [x] Simplicity
  - Small core, universal `let`, fixed operator set, no syntax kept only from tradition.
- [x] Explicivity/WYSIWYG
  - `known`, `fixed`, `unmanaged`, `mut`, `Address`, `Region`, `Access[T]`, `Expect`, `Maybe`, `@extern`, and `@repr` expose behavior instead of hiding it.
- [x] Maintainability
  - Long-term rules beat shorthand: no hidden exceptions, null, implicit dynamic lookup, or unsafe-warning downgrade.
- [x] One obvious way
  - No source `Ptr`/`Pointer`, no `unsafe`, no `fn`/`struct`/`enum` duplicates, no user symbolic operator overloading.
- [x] Verbose when needed
  - DRY aliases allowed only when they preserve one semantic model, e.g. `MutAccess[T]` and `OpaqueAccess[T]`.
- [x] One-token-lookahead gate
  - Any source form needing more than one token lookahead must be rejected or redesigned before lock.

Source-language locks:

- [x] lexical comments, doc comments, module docs, nested block comments
- [x] numeric literals, suffixes, base prefixes, escaped identifiers, triple-quoted strings
- [x] universal `let`, binding heads, receiver methods, generics, defaults, UALO
- [x] computation vs structural regions and separator rules
- [x] postfix `when`, `else`, guarded emission
- [x] `while`, `leave`, `cycle`, `defer`, `yield`
- [x] `data`, `case`, products, sums, empty data, tags
- [x] datum forms, tuple/record/array syntax, optional/Maybe syntax
- [x] fixed operator set, precedence families, no user symbolic operator creation
- [x] type annotations, inference holes, type algebra, casts/tests, `Type[N]`, universes
- [x] shapes, `|=`, explicit witnesses, receiver lookup/UDNS/UFCS
- [x] qualified types: `known fixed unmanaged mut TYPE`
- [x] low-level memory source model: `Address`, `Region`, `Access[T]`, `Access[mut T]`
- [x] FFI surface: `@extern`, `@repr`, C ABI aliases, representability rule
- [x] attributes, `@target`, metadata call model, tooling namespace
- [x] modules: `import`, `known import`, `export`, module-record surface
- [~] exact parser proof for one-token-lookahead grammar
- [~] exact diagnostic catalog for every rejected source form

Musi-to-SEIL locks:

- [x] direct Musi-to-SEIL lowering, no intermediate IR layer
- [x] known execution runs verified SEIL, not source-tree evaluator
- [x] semantic runtime effects must lower into required SEIL metadata/declarations
- [x] `fixed`, `unmanaged`, access/address, FFI, target, shape/witness lowering obligations
- [~] exact lowering algorithm for every source expression
- [~] exact source-map/tool-metadata payloads for high-fidelity decompilation
- [~] exact import path resolution/package discovery

SEIL locks:

- [x] WAT-like `(module ...)` text shape with CIL-like asm/reference role
- [x] compact binary image: 40-byte fixed header, section directory, section families
- [x] section payload rows: row-kind directory, row offset table, packed row bytes
- [x] core section families: `names`, `asm`, `deps`, `defs`, `code`, `data`, `meta`, `tool`
- [x] required vs skippable metadata policy
- [x] opcode registry ranges and stack-effect notation
- [x] managed `(ref T)` vs VM `(ptr T)` distinction
- [x] verifier responsibilities: types, stack effects, metadata refs, roots, safepoints
- [~] exact text assembler/disassembler canonical formatting details
- [~] exact per-type metadata binary encodings
- [~] exact trap taxonomy and numeric edge behavior

SEAM locks:

- [x] loader/verify/link/init/execute lifecycle
- [x] structured halt outcomes and failure channels
- [x] frames, calls, returns, branches, cleanup, yield/resume obligations
- [x] managed memory direction: precise roots, layouts, safepoints, barriers, GenImmix allowed
- [x] `fixed` implementation choices: pin, nonmoving, unmanaged copy, reject
- [x] `unmanaged` outside managed tracing/movement/reclamation unless core metadata says otherwise
- [x] explicit dynamic/capability/box/keyed protocols
- [x] no hidden host UB; invalid behavior is reject/trap/failure
- [~] exact frame object layout
- [~] exact host embedding API
- [~] exact GC policy/tuning and allocator details

Runtime/core library locks:

- [x] `musi:core`, `musi:ffi`, `musi:rt` are native/compiler module prefixes with optional `.ms` interface surfaces.
- [x] `musi:rt` intrinsics need declared signature, phase, allocation, failure/trap, capability, target/profile, and lowering metadata.
- [~] exact `musi:rt` intrinsic catalog
- [~] exact standard native module catalog
- [~] exact Go-like/C#-like stdlib surface

Implementation/conformance evidence:

- [~] parser generated/proven one-token lookahead
- [~] lexer/parser fixtures for every locked syntax form
- [~] lowering fixtures from Musi source to SEIL text/binary
- [~] SEIL assembler/disassembler round-trip fixtures
- [~] verifier pass/fail corpus
- [~] known-phase evaluator tests
- [~] runtime failure/GC/FFI tests
- [~] diagnostic snapshot/kind coverage
- [~] self-hosting viability: compiler/VM pieces writable in Musi
- [~] full spec conformance suite

## Remaining Evidence Needed Before Full Confidence

See `Full Musi Spec Lock Tracker` above. It is canonical evidence list now.
