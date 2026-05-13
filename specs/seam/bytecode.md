# SEAM Bytecode

Status: proposed

This spec defines SEAM BC/IL: the canonical stack-based bytecode transported by `.seam` and mirrored by `.seamil` text.

SEAM BC/IL is a portable VM substrate. It is not hard-wired to Musi source syntax. Musi, or any other frontend, targets SEAM by lowering source concepts to stack operations, metadata tables, descriptors, and runtime calls.

## Design Rules

Core rule: an opcode exists only when it is a primitive stack transition.

Each opcode must define:

- what it pops from the operand stack
- what it pushes to the operand stack
- which descriptor or immediate operand constrains it
- which verifier rule makes the transition valid

Source concepts do not name opcodes. Bytecode mnemonics describe VM operations, not source declarations or type constructors.

## Rust Comparison

Rust 2024 is the host implementation language. SEAM bytecode is not Rust MIR, LLVM IR, or a Rust trait/object model.

Rust can inform implementation strategies such as enum layouts, call dispatch tables, and unsafe boundaries. Public SEAM remains stack bytecode plus stable descriptors.

## Reference Influences

SEAM naming uses existing VM instruction sets as spelling evidence, not as a rule to clone them:

- CIL: load/store roots, field roots, direct calls, indirect calls, and foreign call edges
- WebAssembly: typed stack-machine arithmetic families and branch-table structure

SEAM normalizes these into dotted text mnemonics.

## Text Mnemonics

`.seamil` uses dotted mnemonics. Binary `.seam` stores numeric opcodes; dots are text only.

Mnemonic grammar:

```text
segment      = lowercase-ascii-letter *(lowercase-ascii-letter / digit)
mnemonic     = segment *("." segment)
```

Rules:

- no aliases
- no leading, trailing, or repeated dots
- lowercase ASCII only
- dots separate stack action, target root, and qualifier
- `mod` is not a root because it conflicts with mathematical modulus
- `nat` is not a root because it conflicts with natural numbers

Canonical roots:

```text
ld      load/push value
st      store/pop into storage
new     allocate or construct value
br      branch
call    call
tail    tail-call qualifier
is      test/predicate
cast    checked cast
loc     local
glob    global
fld     field
elem    indexed element
len     length
obj     object/layout aggregate
fn      function/callable
ind     indirect target/address from stack
mdl     module
ffi     foreign function interface / ABI edge
meta    metadata
attr    attribute
syn     syntax object, only in syntax-domain metadata
```

Meaning distinctions:

- `.ind` means the callable target comes from the operand stack.
- `.ffi` means a foreign ABI boundary.

## Stack Types

Method signatures and block signatures are stack type lists.

```seamil
.method $pair (%x : Int, %y : Bool) -> [Int, Bool] locals [] {
entry stack []:
  ld.loc 0
  ld.loc 1
  ret
}
```

Rules:

- method result is a stack type list, not only one type
- block incoming stack is an exact stack type list
- each instruction transforms the current stack type list
- `ret` consumes exactly the method result stack list
- no implicit tuple allocation occurs for multi-result signatures
- a tuple/product value still uses `new.obj` when a storable value is needed

## Branch Stack Rule

Branches transfer the whole current stack.

Verifier rule:

1. `br.false` pops one `Bool`; `br.tbl` pops one integer index.
2. The remaining current stack must exactly match the target block `stack [...]` signature.
3. The target receives that whole stack.

There is no partial branch payload convention.

Example:

```seamil
left stack []:
  ld.loc 0
  br join

right stack []:
  ld.loc 1
  br join

join stack [Int]:
  ret
```

## Numeric Semantics

SEAM BC/IL is language-neutral. Source languages choose opcodes to express their own safety policy.

Locked arithmetic semantics:

- `add`, `sub`, and `mul` define the core arithmetic transition.
- `div.s`, `rem.s`, and ordered comparisons use signed integer interpretation.

Musi source may still define checked arithmetic as its source-level default by emitting helper calls before or instead of a primitive arithmetic transition.

## Operand Kinds

Canonical operand kinds:

```text
none       no inline operand
u8/u16/u32 unsigned immediate
i32/i64    signed integer immediate
f32/f64    floating-point immediate
str        string table id
const      constant table id
type       type table id
sig        signature table id
method     method table id
member     member/slot descriptor id
field      field/layout descriptor id
global     global id
foreign    foreign ABI descriptor id
block      block id
btbl       branch table id
token      typed metadata token id
```

## Opcode Table

Numeric opcode positions are canonical for this design. Gaps are reserved.

### `0x00..0x0F` Stack And Constants

### `0x00..0x0F` Core Constants

|       Hex | Mnemonic  | Operand | Stack effect | Meaning                   |
| --------: | --------- | ------- | ------------ | ------------------------- |
|      `09` | `ld.c`    | const   | `-> T`       | push constant table value |
|      `0A` | `ld.c.i4` | i32     | `-> Int32`   | push 32-bit integer       |
|      `0E` | `ld.str`  | str     | `-> String`  | push string table value   |
| `00`-`08` | reserved  |         |              | reserved                  |
| `0B`-`0D` | reserved  |         |              | reserved                  |
|      `0F` | reserved  |         |              | reserved                  |

### `0x10..0x1F` Storage And Fields

|       Hex | Mnemonic  | Operand | Stack effect | Meaning                         |
| --------: | --------- | ------- | ------------ | ------------------------------- |
|      `12` | `ld.loc`  | u16     | `-> T`       | load local                      |
|      `13` | `st.loc`  | u16     | `T ->`       | store local                     |
|      `14` | `ld.glob` | global  | `-> T`       | load global                     |
|      `15` | `st.glob` | global  | `T ->`       | store global                    |
|      `16` | `ld.fld`  | field   | `Obj -> T`   | load field by layout descriptor |
|      `17` | `st.fld`  | field   | `Obj, T ->`  | store field by layout descriptor |
| `10`-`11` | reserved  |         |              | reserved                       |
| `18`-`1F` | reserved  |         |              | reserved                       |

### `0x20..0x4F` Arithmetic, Comparison, Control Flow

|       Hex | Mnemonic   | Operand | Stack effect          | Meaning               |
| --------: | ---------- | ------- | --------------------- | --------------------- |
|      `21` | `add`      | none    | `N, N -> N`           | arithmetic add        |
|      `22` | `sub`      | none    | `N, N -> N`           | arithmetic subtract   |
|      `23` | `mul`      | none    | `N, N -> N`           | arithmetic multiply   |
|      `27` | `div.s`    | none    | `Int, Int -> Int`     | signed division       |
|      `29` | `rem.s`    | none    | `Int, Int -> Int`     | signed remainder      |
|      `2B` | `and`      | none    | `A, A -> A`           | bitwise/boolean and   |
|      `2C` | `or`       | none    | `A, A -> A`           | bitwise/boolean or    |
|      `2D` | `xor`      | none    | `A, A -> A`           | bitwise/boolean xor   |
|      `2E` | `not`      | none    | `A -> A`              | bitwise/boolean not   |
|      `37` | `ceq`      | none    | `A, A -> Bool`        | equality compare      |
|      `38` | `cne`      | none    | `A, A -> Bool`        | inequality compare    |
|      `39` | `clt.s`    | none    | `A, A -> Bool`        | signed less-than      |
|      `3B` | `cgt.s`    | none    | `A, A -> Bool`        | signed greater-than   |
|      `3D` | `cle.s`    | none    | `A, A -> Bool`        | signed less/equal     |
|      `40` | `cge.s`    | none    | `A, A -> Bool`        | signed greater/equal  |
|      `42` | `br`       | block   | `S -> target.S`       | unconditional branch  |
|      `44` | `br.false` | block   | `S, Bool -> target.S` | branch if false       |
|      `45` | `br.tbl`   | btbl    | `S, Int -> target.S`  | table branch          |
|      `47` | `ret`      | none    | `results ->`          | return result stack   |
| `20`-`20` | reserved   |         |                       | reserved              |
| `24`-`26` | reserved   |         |                       | reserved              |
| `28`-`28` | reserved   |         |                       | reserved              |
| `2A`-`2A` | reserved   |         |                       | reserved              |
| `2F`-`36` | reserved   |         |                       | reserved              |
| `3A`-`3A` | reserved   |         |                       | reserved              |
| `3C`-`3C` | reserved   |         |                       | reserved              |
| `3E`-`3F` | reserved   |         |                       | reserved              |
| `41`-`41` | reserved   |         |                       | reserved              |
| `43`-`43` | reserved   |         |                       | reserved              |
| `46`-`46` | reserved   |         |                       | reserved              |
| `48`-`4F` | reserved   |         |                       | reserved              |

### `0x50..0x6F` Calls And Function Values

|       Hex | Mnemonic    | Operand   | Stack effect          | Meaning                            |
| --------: | ----------- | --------- | --------------------- | ---------------------------------- |
|      `50` | `call`      | method    | `args -> results`     | direct managed call                |
|      `51` | `call.ind`  | sig       | `Fn, args -> results` | indirect first-class callable call |
|      `55` | `call.ffi`  | foreign   | `args -> results`     | foreign ABI call                   |
|      `56` | `tail.call` | method    | `args -> results`     | direct tail call                   |
|      `5D` | `new.fn`    | method,u8 | `captures -> Fn`      | create closure/function value      |
|      `61` | `ld.ffi`    | foreign   | `-> FfiFn`            | load foreign symbol handle         |
| `52`-`54` | reserved    |           |                       | reserved                           |
| `57`-`5C` | reserved    |           |                       | reserved                           |
| `5E`-`60` | reserved    |           |                       | reserved                           |
| `62`-`6F` | reserved    |           |                       | reserved                           |

### `0x70..0x7F` Objects, Arrays, Elements

|       Hex | Mnemonic  | Operand  | Stack effect          | Meaning                          |
| --------: | --------- | -------- | --------------------- | -------------------------------- |
|      `70` | `new.obj` | type,u16 | `fields -> Obj`       | construct layout object/value    |
|      `71` | `new.arr` | type,u16 | `items -> Array[T]`   | construct array from stack items |
|      `73` | `ld.elem` | type     | `Array[T], Int -> T`  | load element                     |
|      `74` | `st.elem` | type     | `Array[T], Int, T ->` | store element                    |
|      `76` | `ld.len`  | none     | `Array/String -> Int` | load length                      |
| `72`-`72` | reserved  |          |                       | reserved                         |
| `75`-`75` | reserved  |          |                       | reserved                         |
| `77`-`7F` | reserved  |          |                       | reserved                         |

### `0x80..0x8F` Types

|       Hex | Mnemonic  | Operand | Stack effect | Meaning                          |
| --------: | --------- | ------- | ------------ | -------------------------------- |
|      `80` | `ld.type` | type    | `-> Type`    | load type value/token            |
|      `82` | `is.inst` | type    | `A -> Bool`  | runtime instance/refinement test |
|      `83` | `cast`    | type    | `A -> B`     | checked cast                     |
| `81`-`81` | reserved  |         |              | reserved                         |
| `84`-`8F` | reserved  |         |              | reserved                         |

### `0x90..0xAF` Reserved

|       Hex | Mnemonic | Operand | Stack effect | Meaning  |
| --------: | -------- | ------- | ------------ | -------- |
| `90`-`AF` | reserved |         |              | reserved |

### `0xB0..0xBF` Link And Module

|       Hex | Mnemonic   | Operand | Stack effect  | Meaning                       |
| --------: | ---------- | ------- | ------------- | ----------------------------- |
|      `B2` | `mdl.load` | str     | `-> Module`   | dynamic module load           |
|      `B3` | `mdl.get`  | str     | `Module -> T` | dynamic module export lookup  |
| `B0`-`B1` | reserved   |         |               | reserved                      |
| `B4`-`BF` | reserved   |         |               | reserved                      |

### `0xC0..0xFE` Reserved Domain Space

`0xC0..0xFE` are reserved for future standardized domain packs. They must not be used for frontend-private extensions in portable modules.

### `0xFF` Extended Opcode Escape

Extended opcode encoding:

```text
0xFF opcode:u16
```

Extended opcode mnemonics still use the same dotted mnemonic rules.

## Lowering Examples

### Tuple/Product

```seamil
ld.loc 0
ld.loc 1
new.obj $Tuple2 2
```

Access:

```seamil
ld.loc 2
ld.fld $Tuple2.0
```

### Sum Variant / Option / Result

```seamil
ld.loc payload
new.obj $Option.Some 1
```

Tag test:

```seamil
ld.loc opt
ld.fld $Option.tag
ld.c.i4 1
ceq
br.false none
```

### Shape / Interface Dispatch

Dictionary lowering:

```seamil
ld.loc dict
ld.fld $EqDict.equal
ld.loc a
ld.loc b
call.ind $EqEqualSig
```

### Native Runtime Call

```seamil
ld.loc message
call.ffi $musi_console_writeLine
```

## See Also

- `specs/seam/format.md`
- `specs/seam/lowering.md`
- `specs/seam/domains.md`
