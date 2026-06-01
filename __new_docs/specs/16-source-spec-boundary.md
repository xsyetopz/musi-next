# 16. Source-Spec Boundary

Status: normative scope statement.

This pack specifies source-language syntax and source-level semantics.

The following topics are not part of this source spec:

```text
SEAM bytecode instruction set
.seam binary/artifact layout
VM verifier instruction encoding
object header layout
GC rooting ABI
foreign calling-convention lowering
host handle representation
runtime trampoline mechanics
exact Resumable / Generator protocol member names
implementation target catalog vocabulary
```

This boundary does not make those topics optional. It states that they belong to implementation/runtime/bytecode specifications rather than this source-language pack.

The source surface remains fixed where this pack defines it:

```text
@foreign(...) for FFI source declarations
yield for source suspension
pin value as name (...) for source pinning
unsafe (...) for source unsafe authority
```
