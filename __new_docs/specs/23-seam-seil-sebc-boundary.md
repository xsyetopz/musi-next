# 23. SEAM, SEIL, and SEBC Boundary Contract

Status: normative boundary contract. SEIL concrete syntax and SEBC binary encoding are intentionally not locked by this chapter.

## Names

```text
SEAM = Stack Effect Abstract Machine
SEIL = Stack Effect Intermediate Language
SEBC = Stack Effect Bytecode
```

## Position in the stack

```text
Musi source -> Musi Core -> SEIL -> SEBC -> SEAM
```

Other languages may target SEAM:

```text
Other source -> frontend-specific Core/IR -> SEIL -> SEBC -> SEAM
```

Musi is a flagship frontend, not the whole VM platform.

## Musi Core vs SEIL

Musi Core is frontend-specific. It may represent Musi-specific lowered semantics.

SEIL is language-neutral. It must not require Musi source syntax.

SEIL must not contain:

```text
Musi guarded-expression syntax
Musi pattern syntax
known as source syntax
splice as source syntax
~ quote syntax as source syntax
Musi let/data/trait syntax
```

SEIL may contain VM-level concepts that Musi lowers into:

```text
modules
types
functions
blocks
values
locals
stack effects
managed references
host handles
roots
pinning
raw pointers
Any
Empty
syntax-value runtime support
traps
calls
host calls
metadata
```

## SEIL role

SEIL is a small fixed language for virtual machines.

It is allowed to be Lisp-shaped / S-expression-shaped, but it is not a Lisp runtime and does not have arbitrary Lisp evaluation semantics.

Locked SEIL requirements:

```text
fixed grammar
no reader macros
no ambient macro expansion
no source-language grammar extension
static/verifiable stack effects
module-level metadata
language-neutral instruction vocabulary
host capability imports
managed/host/raw boundary operations
```

Concrete SEIL syntax is not specified in this chapter.

## SEBC role

SEBC is the compact bytecode encoding of SEIL.

SEBC must not have a different semantic model from SEIL.

Locked SEBC requirements:

```text
sectioned artifact
versioned format
loadable from memory buffers
verifier-ready metadata
symbol/type/function/import tables or equivalent
skippable custom sections unless marked required
source/debug metadata support
live-link compatibility metadata support
```

Concrete opcode numbers, section tags, encoding widths, and binary layout are not specified in this chapter.

## SEAM role

SEAM is the verifier, loader, linker, runtime, and execution machine for SEBC.

SEAM responsibilities:

```text
load SEBC
verify stack effects and types
resolve module dependencies
resolve host capabilities and imports
initialize fixed storage
execute code
maintain managed runtime invariants
maintain Root/Host/RawPtr distinctions
support pinning rules
surface traps/failures according to contract
provide debug/introspection hooks
support live linking if enabled by the embedding
```

## Stack effect requirement

SEIL/SEBC instructions must have statically checkable stack effects.

The name SEAM requires this to be central, not incidental.

The exact notation for stack effects belongs to the later SEIL syntax chapter.

## Type requirement

Verified SEIL/SEBC admits concrete VM verification types, `Any`, and `Empty`.

`Unknown` must be resolved before SEIL/SEBC.

## SEIL syntax non-decision

This chapter deliberately does not lock:

```text
parenthesis shape
instruction spelling
module form spelling
attribute spelling
comment syntax
string escape rules
opcode names
binary opcode values
section numbers
```

AI agents must not infer these details from examples in discussion unless a later SEIL syntax chapter explicitly locks them.
