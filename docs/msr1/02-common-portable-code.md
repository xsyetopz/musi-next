> This agent-indexable topic view is extracted from [`../MSR1.md`](../MSR1.md). `MSR1.md` remains the sole normative authority.

# Part II — Common Portable Code (CPC)

## II.1 Status and role

CPC is language-neutral. A conforming Musi compiler may use any internal representation, but a component claiming to be an MSR1 CPC producer or consumer shall implement the CPC semantics below. CPC is suitable for direct interpretation, ahead-of-time translation, storage as an interchange artifact, or use as a verification boundary.

## II.2 Capability requirements

Every CPC module has an explicit requirement set. A requirement names a CPC revision and any target/ABI capabilities whose semantics are needed by the module. Requirements are compile/load-time facts and are not runtime feature probes.

A consumer shall establish before execution or native lowering that every mandatory requirement is supported by the selected contracts. Unsupported mandatory requirements cause rejection. Optional implementation facilities do not alter accepted module semantics.

## II-A — CPC semantic definition

Common Portable Code (CPC) is a generalized typed, verified, stack-based portable code model independent of Musi. Musi is the first source language normatively mapped to CPC. The CPC abstract operand machine defines CPC semantics; it does not require a resident virtual machine.

The machine is designed implementation-up: a conforming interpreter/AOT implementation must be possible on small-word, memory-constrained systems without requiring a heap, GC, scheduler, operating system, exception unwinder, dynamic loader, resident compiler, or runtime type system for unused facilities.

The CPC semantic definition defines semantics. `Part II-B` and `Annex B` define the one canonical textual CPC form. Binary opcode allocation/container encoding is a separate representation contract and may compress common textual operations, but may not alter their semantics.

## 1. Design contract

For every CPC facility, code-size, interpreter state, metadata, and runtime machinery shall be attributable to that facility's presence or use. Unused facilities shall not impose mandatory general runtime support. This is the CPC form of zero hidden cost.

The machine is:

- architecture-neutral and host-word-neutral;
- byte-address-neutral except where a selected target contract defines addressable units;
- typed and verifier-checkable per function;
- zero-address/operand-stack based for computation;
- streamable in declaration order;
- suitable for direct interpretation or AOT translation;
- source-language-neutral;
- deterministic wherever the selected target contract is deterministic;
- free of language-level undefined behavior.

The abstract operand stack is semantic. AOT implementations need not materialize it physically.

## 2. Module and function model

A CPC module is an ordered sequence of declarations. A declaration is visible only after its declaration point. References therefore target already-declared type, external, global, or function IDs. This permits bounded single-pass readers and emitters.

A function has:

- a numeric function ID unique in the module;
- an exact parameter vector;
- zero or one result type;
- effect `PLAIN` or `YIELD`;
- an ordered local-type vector;
- a declared maximum operand-stack depth;
- a linear instruction stream containing labels.

Every branch target names a label in the same function. Every label has exactly one verifier-known incoming operand-stack type vector.

Execution state contains only what the executed program requires:

```text
current function and instruction position
arguments and locals
semantic operand stack
explicit yielding-context state only when yielding is used
```

## 3. Types

### 3.1 Primitive scalar types

Canonical scalar types are:

```text
UNIT
BOOL
I8 I16 I32 I64
U8 U16 U32 U64
F16 F32 F64
BITS[n]          n >= 1, compile-known
ADDR
```

Implementations may lower widths larger than the native machine width through software. A target contract may reject deployment only when it cannot implement the required semantics; it may not silently narrow them.

`ADDR` is an opaque target data address. Arithmetic on it is available only through the raw-address instructions below.

### 3.2 Declared types

Compound/runtime-capability types are declared once and referred to by numeric type ID `Tn`:

```text
ARRAY element,count
RECORD field-types...
VARIANT case-layouts...
REF target,mode,kind
STORAGE bytes,alignment
ATOMIC value-type
FUNC parameter-types,result,effect
```

`DYNREF` is a single primitive runtime-capability type rather than a declared nominal family.

Reference mode is `R` or `W`. Reference kind is `O` or `V` (ordinary or volatile).

`STORAGE` and `ATOMIC` are storage-only. They are never copied by ordinary value operations.

`DYNREF` is a non-owning erased typed reference containing only the designation and the runtime type identity required for checked refinement. It implies no allocation or ownership.

Source-language abstractions such as strings, views, nullable values, choices, modules, methods, closures, containers, and schedulers are not intrinsic CPC abstract machine categories unless represented using the types above.

## 4. Verification model

Before execution or native lowering, each function is verified independently after all referenced declarations are known.

Verification proves:

- every instruction and operand is well formed;
- every referenced ID already exists and has the required category;
- every local/argument access is in range;
- exact input and output stack types for every instruction;
- exact stack equality at every control-flow join;
- declared maximum stack depth is not exceeded and is sufficient;
- result state matches the function signature at `RET`;
- `YLD`/yielding calls occur only in `YIELD` functions;
- safe reference mode/kind restrictions;
- safe indexing bounds or an explicit runtime check performed by the instruction;
- storage lifetime transitions;
- atomic type/order legality;
- dynamic-reference refinement legality.

Verifier success never converts raw-address or foreign behavior into safe-reference behavior.

## 5. Instruction-set rule

A new primary opcode exists only for a semantically distinct operation whose frequency or decoding cost justifies direct dispatch. Type/width variation alone does not create a new semantic opcode. Rare capability families use a primary family opcode plus a compact suboperation.

The normative semantic ISA consists exactly of the instruction families in sections 6–15. Textual mnemonics are canonical and case-sensitive uppercase.

For stack effects below, the rightmost item is the top of stack.

## 6. Constants, arguments, locals, stack

| Instruction | Static operands | Stack before | Stack after | Semantics |
| --- | --- | --- | --- | --- |
| `LDC` | `T value` | `[]` | `[T]` | Push exactly represented constant. |
| `LDA` | argument index | `[]` | `[Tn]` | Push argument value. Storage-only argument types are invalid. |
| `LDF` | function/external ID | `[]` | `[FUNC(signature,effect)]` | Push callable identity without an environment. |
| `LDL` | local index | `[]` | `[Tn]` | Push initialized local value. Storage-only locals are invalid. |
| `STL` | local index | `[Tn]` | `[]` | Store value into local. Storage-only locals are invalid. |
| `DUP` | none | `[T]` | `[T,T]` | Duplicate copyable value. |
| `POP` | none | `[T]` | `[]` | Discard value. Ending an object lifetime is not `POP`. |
| `SWP` | none | `[A,B]` | `[B,A]` | Exchange two top copyable values. |

Locals have verifier state `uninitialized` until first `STL`; `LDL` before initialization is invalid.

## 7. Arithmetic and bits

Checked integer operations trap on non-representable result, division by zero, or the signed minimum divided by `-1` where applicable. Floating operations implement the exact IEEE format semantics of their `F16/F32/F64` type.

| Instruction | Legal T | Stack | Result |
| --- | --- | --- | --- |
| `ADD T` | integer, float | `[T,T] -> [T]` | checked integer / IEEE addition |
| `SUB T` | integer, float | `[T,T] -> [T]` | checked integer / IEEE subtraction |
| `MUL T` | integer, float | `[T,T] -> [T]` | checked integer / IEEE multiplication |
| `DIV T` | integer, float | `[T,T] -> [T]` | checked integer / IEEE division |
| `REM T` | integer | `[T,T] -> [T]` | checked remainder |
| `NEG T` | signed integer, float | `[T] -> [T]` | checked integer / IEEE negation |
| `WAD T` | integer or `BITS[n]` | `[T,T] -> [T]` | modulo-2^width addition |
| `WSB T` | integer or `BITS[n]` | `[T,T] -> [T]` | modulo-2^width subtraction |
| `WML T` | integer or `BITS[n]` | `[T,T] -> [T]` | modulo-2^width multiplication |
| `AND T` | integer or `BITS[n]` | `[T,T] -> [T]` | bitwise and |
| `OR T` | integer or `BITS[n]` | `[T,T] -> [T]` | bitwise or |
| `XOR T` | integer or `BITS[n]` | `[T,T] -> [T]` | bitwise xor |
| `NOT T` | integer or `BITS[n]` | `[T] -> [T]` | bitwise complement |
| `SHL T` | integer or `BITS[n]` | `[T,U32] -> [T]` | logical left shift; count outside width traps |
| `SHR T` | unsigned integer or `BITS[n]` | `[T,U32] -> [T]` | logical right shift; count outside width traps |
| `SAR T` | signed integer | `[T,U32] -> [T]` | arithmetic right shift; count outside width traps |
| `ROL T` | integer or `BITS[n]` | `[T,U32] -> [T]` | rotate left modulo width |
| `ROR T` | integer or `BITS[n]` | `[T,U32] -> [T]` | rotate right modulo width |

## 8. Comparison and conversion

`CMP predicate,T` consumes two equal T values and produces `BOOL`. Predicates are exactly `EQ NE LT LE GT GE`. Ordering predicates are legal only for ordered scalar types. Floating comparisons are IEEE ordered comparisons; `NE` is logical negation of `EQ`.

```text
CMP predicate,T       [T,T] -> [BOOL]
```

`CVT mode,from,to` consumes `from` and produces `to`. Modes are:

- `EXACT`: conversion is statically guaranteed representable; verifier rejects otherwise-unprovable use;
- `CHECK`: runtime representability failure traps;
- `TRUNC`: explicit low-order integer/bit truncation only;
- `BIT`: same-width bit reinterpretation only between types for which the selected target/CPC abstract machine representation contract defines a total bit mapping.

```text
CVT mode,from,to      [from] -> [to]
```

## 9. Aggregate values

Aggregate operations are grouped under `AGG subop,...` because they are less frequent than scalar arithmetic. `AGG` never implies allocation.

Suboperations:

```text
NEW T                 components -> [T]
GET T index           [T] -> [field]
SET T index           [T,field] -> [T]
IDX T                 [T,U32] -> [element]
PUT T                 [T,U32,element] -> [T]
TAG T                 [T] -> [U32]
CASE T case           payload-components -> [T]
PAY T case index      [T] -> [payload-field]
```

`NEW` applies to `ARRAY` and `RECORD` declarations and consumes components in declaration/index order. `GET`/`SET` perform compile-known field/element projection/update. `IDX`/`PUT` are runtime array-value indexing/update and perform bounds checks unless proven redundant. Runtime indexed safe access uses `REF IDX`.

`TAG`, `CASE`, and `PAY` apply to `VARIANT`. `PAY` is verifier-valid only on a control-flow path refined to the matching case, otherwise it traps.

## 10. Safe references and memory

`REF` is a grouped safe-reference constructor. It never creates a safe reference from an arbitrary `ADDR`.

Suboperations:

```text
REF ARG n             [] -> [REF(argument,R,O)]
REF LOC n             [] -> [REF(local,W,O)]
REF GLB n             [] -> [REF(global,declared-mode,O)]
REF FLD field         [REF(record)] -> [REF(field)]
REF IDX               [REF(array), U32] -> [REF(element)]
REF WEAK              [REF(T,W,K)] -> [REF(T,R,K)]
```

`REF IDX` performs the declared array bounds check unless the verifier proves it redundant. Reference mode/kind propagated by `FLD`/`IDX` may only be weakened by representation/field constraints, never strengthened.

Memory transactions:

```text
LD T                   [REF(T,R-or-W,O)] -> [T]
ST T                   [REF(T,W,O),T] -> []
VLD T                  [REF(T,R-or-W,V)] -> [T]
VST T                  [REF(T,W,V),T] -> []
```

Storage-only T is illegal for ordinary `LD/ST/VLD/VST`.

## 11. Raw addresses

```text
ADR                     [REF(T,M,K)] -> [ADDR]
AOF                     [ADDR,I64] -> [ADDR]
ADS                     [ADDR,ADDR] -> [I64]
RLD T                   [ADDR] -> [T]
RST T                   [ADDR,T] -> []
RVL T                   [ADDR] -> [T]
RVS T                   [ADDR,T] -> []
```

`AOF`/`ADS` follow the selected target address contract. Raw loads/stores are explicit target-memory transactions and may fault or interact with hardware as defined by that contract; they do not create optimizer undefined behavior.

## 12. Storage lifetime

Storage operations are grouped under `LIF`:

```text
LIF BEG T              [REF(STORAGE,W,O)] -> [REF(T,W,O)]
LIF END T              [REF(T,R-or-W,O)] -> []
```

A storage region is designated using ordinary `REF LOC`/`REF GLB`; storage-only values cannot be loaded or copied. `LIF BEG` requires sufficient size/alignment and no overlapping live object; it establishes exactly one live T. `LIF END` ends that lifetime. The verifier tracks the storage lifetime state along control flow. Raw stores cannot synthesize a live safe T.

## 13. Atomics

Atomic operations are grouped under `ATM`. Orders are exactly `RELAX ACQ REL ACQREL SEQCST`; verifier legality depends on the operation.

```text
ATM LD T order                         [REF(ATOMIC(T),R-or-W,O)] -> [T]
ATM ST T order                         [REF(ATOMIC(T),W,O),T] -> []
ATM XCH T order                        [REF(ATOMIC(T),W,O),T] -> [T]
ATM CMP T success failure              [REF(ATOMIC(T),W,O),T,T] -> [T,BOOL]
ATM ADD T order                        [REF(ATOMIC(T),W,O),T] -> [T]
ATM SUB T order                        [REF(ATOMIC(T),W,O),T] -> [T]
ATM AND T order                        [REF(ATOMIC(T),W,O),T] -> [T]
ATM OR T order                         [REF(ATOMIC(T),W,O),T] -> [T]
ATM XOR T order                        [REF(ATOMIC(T),W,O),T] -> [T]
```

RMW forms return the previous value. `ATM CMP` consumes expected then desired and returns previous value plus success. A target may reject deployment when it cannot implement the exact requested atomic semantics; it may not weaken them.

## 14. Dynamic typed references

Dynamic-reference operations are grouped under `DYN`:

```text
DYN ERA T              [REF(T,R,O)] -> [DYNREF]
DYN ISA T              [DYNREF] -> [DYNREF,BOOL]
DYN REF T              [DYNREF] -> [REF(T,R,O)]
```

`DYN REF` is verifier-safe on a path refined by a matching `DYN ISA T`; without such proof it performs a checked refinement and traps on mismatch. No payload is copied, boxed, owned, or allocated. Type identities are required only for types actually participating in these operations.

## 15. Control flow and calls

```text
JMP label              [] -> []
BRT label              [BOOL] -> []
BRF label              [BOOL] -> []
CAL callable-id        [args...] -> [result?]
CAI signature          [FUNC(signature,PLAIN),args...] -> [result?]
YCL callable-id        [args...] -> [result?]
YCI signature          [FUNC(signature,YIELD),args...] -> [result?]
YLD                    [] -> []
RET                    [result?] -> []
TRP code               any -> no successor
```

`CAL/CAI` target `PLAIN` functions or compatible externals. `YCL/YCI` target `YIELD` functions or compatible externals and are legal only from `YIELD` functions. `YLD` is legal only in a `YIELD` function and suspends the complete current execution context without destroying frames, locals, or operand state.

The machine defines no scheduler, spawn, join, task, channel, mutex, allocation, exception, or unwinding instruction.

## 16. Globals, externals, initialization

A module may declare typed globals and external functions. Each global declares `RO` or `RW`; `REF GLB` derives its safe-reference mode from that declaration. Globals have explicit initialization data or are initialized by ordinary functions in declaration-defined order. There is no runtime module object.

Foreign/native ABI details are not CPC semantics. External declarations name an ABI contract explicitly; internal CPC calls use only the CPC abstract machine signature/effect model.

## 17. Streamability and constrained implementations

A conforming textual reader, verifier, interpreter, or translator can process declarations top-to-bottom. It need retain only the declaration tables required by referenced IDs and the current function's verification state. Whole-program AST/CFG reconstruction is not required for basic verification or execution.

Function declarations carry maximum operand-stack depth so a small interpreter can provision bounded execution storage before entry. A producer may overstate this bound; a verifier rejects an understated bound.

Code and immutable data may execute/read directly from ROM, flash, cartridge, banked storage, or another target-defined non-RAM store. The specification does not require a decoded instruction-object representation or a second in-RAM copy of the program.

## 18. AOT freedom

AOT lowering may erase the operand stack, fold or scalarize aggregates, eliminate checks proven redundant, select target registers/zero-page/direct-page storage, overlay non-overlapping frames, use hidden destination arguments for large value returns, and dead-strip unused support. It must preserve all CPC observable semantics.

## 19. Closedness

The semantic instruction inventory in this document is closed for this specification version. An implementation may use private fused/specialized opcodes internally, but portable CPC shall use only the canonical semantic instructions defined here. Extensions require a new CPC semantic definition version and may not silently reinterpret existing code.

## II-B — Canonical textual CPC

Textual CPC is the canonical human-readable interchange spelling of CPC. Its conventions intentionally follow production UCSD/Pascal-family CPC where those conventions are general: short uppercase mnemonics, postfix operands, labels, linear function bodies, and stack-machine execution. Pascal-specific static links, lexical displays, built-in procedures, Pascal sets/strings, and historical frame layouts are not inherited.

There is exactly one textual spelling for each declaration and semantic instruction. Implementations may use private compact encodings internally, but those encodings are not portable CPC artifacts and do not create alternate CPC instructions.

`Annex B` is normative for syntax. This document defines lexical/value conventions and maps syntax to the CPC semantics.

## 1. Character set and lines

Portable text is ASCII. A logical line ends in LF. CRLF input may be normalized to LF before parsing. Tabs are invalid. Indentation has no semantics; leading spaces are permitted only on instruction lines and are canonically four spaces.

Comments begin with `;` and continue to logical line end. There is no block comment.

Keywords, declaration words, mnemonics, type names, predicates, effects, orders, and suboperations are uppercase and case-sensitive. Symbolic debug names, where present in nonsemantic metadata, are outside this core grammar.

## 2. Module header

Every file begins:

```text
CPC 1
```

The integer is the CPC textual-format major version. Version 1 is defined by MSR1.

Zero or more requirement declarations follow the header and precede semantic declarations:

```text
REQUIRE TARGET musi.target.avr8 1
REQUIRE ABI 0 c.avr-gcc 1
REQUIRE CAP atomic.u8
```

`TARGET id revision` selects the target-contract identity/revision required by the module. `ABI slot id revision` binds a dense zero-based ABI slot used by `EXTERN ... ABI n`. `CAP id` names an additional CPC/target capability required by the module. Contract/capability identifiers are case-sensitive ASCII tokens made from letters, digits, `.`, `_`, and `-`, beginning with a letter. Duplicate target requirements, duplicate ABI slots, or duplicate capability identities are invalid.

## 3. IDs

Semantic entities use unsigned decimal IDs with no leading sign:

```text
T0       declared type
G0       global
X0       external
F0       function
L0       label local to one function
```

IDs within each category are allocated densely from zero in declaration order. A reference may name only an already-declared ID except labels, which may be forward-referenced within the current function.

Dense IDs permit compact private remapping without a mandatory runtime symbol table.

## 4. Type spelling

Primitive types have exactly these spellings:

```text
UNIT BOOL
I8 I16 I32 I64
U8 U16 U32 U64
F16 F32 F64
ADDR
BITS[n]
```

Declared types are `Tn`.

Type declarations:

```text
TYPE T0 ARRAY U8 32
TYPE T1 RECORD I16 U8 T0
TYPE T2 REF T1 W O
TYPE T3 STORAGE 64 8
TYPE T4 ATOMIC U16
TYPE T5 FUNC I16 I16 -> I16 PLAIN
```

`VARIANT` uses explicit case records. A case is introduced by `CASE`, followed by its zero-based case number and zero or more payload field types:

```text
TYPE T7 VARIANT
CASE 0
CASE 1 I16
CASE 2 U8 U8
ENDTYPE
```

Cases are dense from zero in declaration order.

`ARRAY` count and `STORAGE` byte count are unsigned decimal compile-known integers and may be zero; `STORAGE` alignment and `BITS[n]` width are positive decimal compile-known integers.

## 5. Globals and externals

Globals:

```text
GLOBAL G0 RW I16 ZERO
GLOBAL G1 RO U8 CONST 42
```

`ZERO` initializes the complete value to its all-zero representation only for types whose CPC representation defines that value. `CONST` accepts exactly one scalar constant. Compound static initialization is performed by ordinary initialization functions.

External function declarations:

```text
EXTERN X0 FUNC I16 I16 -> I16 PLAIN ABI 0
```

`ABI n` selects the ABI requirement whose slot is `n`. Its concrete foreign semantics are supplied by the matching MSR1 ABI contract.

## 6. Functions

Example:

```text
FUNC F0 I16 I16 -> I16 PLAIN STACK 2
LOCAL I16
L0:
    LDA 0
    LDA 1
    ADD I16
    RET
END
```

Parameter order is source order. `VOID` denotes no result. `PLAIN` and `YIELD` are the only effects. `STACK n` declares maximum semantic operand-stack depth.

Locals are numbered from zero by declaration order and precede the first label/instruction.

A function body contains one or more labels. The first instruction executed is the first instruction following the first label.

## 7. Constants

`LDC` syntax is:

```text
LDC <type> <literal>
```

Integer literals are decimal by default, `0x` hexadecimal, or `0b` binary. `_` separators are not permitted in CPC. Signed literals use a leading `-` only for signed integer types.

`BOOL` literals are `FALSE` and `TRUE`.

Floating constants use exact hexadecimal IEEE bit patterns rather than decimal source notation:

```text
LDC F32 0x3F800000
LDC F64 0x3FF0000000000000
```

This prevents parser/host floating conversion from affecting portable code.

`ADDR` constants are not portable core constants and therefore cannot be emitted by `LDC`; addresses originate from references, globals/externals under target contracts, or raw target capabilities.

## 8. Canonical instructions

The complete textual instruction families are exactly:

```text
LDC LDA LDF LDL STL DUP POP SWP
ADD SUB MUL DIV REM NEG WAD WSB WML
AND OR XOR NOT SHL SHR SAR ROL ROR
CMP CVT
AGG
REF LD ST VLD VST
ADR AOF ADS RLD RST RVL RVS
LIF
ATM
DYN
JMP BRT BRF
CAL CAI YCL YCI YLD RET TRP
```

`AGG`, `REF`, `LIF`, `ATM`, and `DYN` require one of their normative suboperations. Their semantics and legal stack types are defined by `Part II-A`.

Examples:

```text
CMP LT I16
CVT CHECK I32 I16
AGG GET T2 1
AGG IDX T3
REF LOC 0
REF IDX
LIF BEG T3
ATM CMP U16 ACQREL ACQ
DYN ISA T4
BRT L3
CAL F2
TRP 7
```

No alias mnemonics exist. In particular, private short forms such as specialized local-zero loads or type-specialized arithmetic opcodes are never alternate portable CPC spellings.

## 9. Canonical whitespace

Canonical emission uses:

- no blank lines except between top-level declarations;
- one ASCII space between tokens;
- four leading ASCII spaces before instructions;
- labels and declarations at column zero;
- no trailing spaces;
- one final LF at end of file.

Parsers may accept additional ASCII spaces where the EBNF permits `space`, but canonical emitters shall produce the form above.

## 10. Optimization boundary

Portable CPC expresses semantic operations, not encoding tricks. An implementation may translate CPC into a private compact representation using dedicated encodings for frequent combinations, branch displacements, or capability suboperations. That representation is not an interchange form and is conforming only insofar as its execution preserves exactly the CPC semantics specified here.

The optimization objective is minimum total consumer plus CPC cost, not minimum abstract opcode count. A capability absent from a program shall not require its decoder/runtime support in an implementation that can omit it.
