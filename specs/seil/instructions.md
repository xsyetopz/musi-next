# SEIL instruction semantics

Locked core SEIL instruction families have VM-oriented behavior. `specs/seil/opcodes.md` lists opcode ids, mnemonic spelling, operand schemas, and compact stack-effect schemas from `seil_opcodes.def`.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` opcode semantics, `seil_opcodes.def`.

References: WebAsm defines stack-machine instruction behavior independently from source languages: <https://webassembly.github.io/spec/core/exec/instructions.html>.

## General rules

SEIL is a typed stack-effect instruction language. Instructions consume operands from the evaluation stack, optional immediate operands from the instruction stream, and metadata from module/body tables. SEIL instruction semantics are VM-oriented and are not Musi syntax, but they preserve enough type and source relationship structure for analysis, transformation, and high-fidelity tooling.

The rightmost stack value is top of stack. Instruction names use canonical mnemonics in text and stable `u16` ids in binary. Binary operands are decoded only through the active opcode schema.

## Control and terminators

- `nop` leaves execution state unchanged.
- `ret` returns outputs matching the current callable signature and terminates the current frame.
- `br` transfers control to a body-local block target.
- `br.true` and `br.false` consume `Bit` and branch or fall through.
- `br.tbl` consumes a natural selector and transfers through a body-local branch table.
- `leave` exits a cleanup/region edge identified by `region_idx`.
- `trap` terminates execution through the trap/failure channel.
- `throw` consumes a value and transfers through exceptional metadata.
- `rethrow` continues the active exceptional edge.

Handler/catch/finally shape is metadata, not a separate opcode family.

## Stack operations

`drop`, `dup`, `swap`, and `rot` operate on stack values without changing their types. Verification rejects use when the required stack prefix is unavailable.

## Consts and frame storage

`const` loads a typed constant-table entry. Inline constants load scalar values encoded in the instruction stream. Frame operations access argument, local, environment/capture, and global namespaces. Stores require assignment compatibility with the destination storage type.

## Calls and callable values

- `call` invokes a statically referenced callable declaration.
- `call.disp` invokes a dispatch target with receiver metadata.
- `call.ind` invokes a callable value constrained by a signature.
- `call.dyn` invokes through dynamic-call protocol metadata and an argument pack.
- `mk.fn` constructs a callable value from a callable reference and required environment/capture values.

Callee origin is declaration metadata: ordinary SEIL body, intrin/runtime binding, extern/foreign binding, or core-defined runtime callable target.

## Scalar arithmetic, bitwise operations, and comparisons

Arithmetic, bitwise, shift, rotate, unary numeric, and comparison opcodes operate on scalar types admitted by core numeric rules. Signed/unsigned variants are distinct integer modes. Division by zero, invalid shifts, overflow, NaN ordering, and checked-conversion failures use core trap or diagnostic behavior.

`rem` is CPU-style remainder. `.un` suffixes on division, remainder, and comparison mean unsigned integer mode.

## Type tests and conversions

`test.ty` returns `Bit`. `cast.ty` performs a checked type cast/coercion according to core type rules. `conv` performs ordinary conversion. `conv.chk` performs checked conversion. `bitcast` reinterprets representation-compatible values. `conv.repr` crosses declared core representations.

## Memory, references, pointers, and allocation

`ld.ref` and `st.ref` operate on managed references. `ld.ptr` and `st.ptr` operate on pointer values with explicit target types. `ld.addr` materializes an address/reference described by body metadata. `alloc` and `alloc.arr` allocate runtime storage described by type/layout metadata.

`mem.copy`, `mem.fill`, and `mem.move` operate on explicit pointer/addressable storage. Aliasing, overlap, trap behavior, and memory-region permissions are SEAM rules.

`size` returns the layout size of a type as `Nat`.

## Products, sums, and indexed storage

Products are records or positional products. `mk.prod` constructs a product value from exactly the required fields. `ld.fld`, `st.fld`, and `addr.fld` operate on named product fields. `ld.idx`, `st.idx`, and `addr.idx` operate on positional product fields.

Sums are tagged variants. `mk.sum` constructs a tagged value with the required optional payload. `ld.tag` exposes tag identity as `Nat`; `is.tag` tests a specific alt; payload operations require the active alt to admit a payload.

Indexed storage operations address runtime-indexed arrays/sequences. `mk.arr` consumes exactly the encoded element count from the stack.

## Dynamic, capability, keyed storage, suspension, and cleanup

`box` and `unbox` are representation transitions. `cap.has` and `cap.need` operate on VM capability evidence. `ld.key`, `st.key`, `has.key`, and `del.key` operate on explicit keyed-storage protocol metadata; they are not implicit dynamic lookup on `Any`.

`yld` suspends according to yield/resume shape metadata. `cln.push`, `cln.pop`, and `cln.run` manage cleanup regions and are tied to region metadata.


## GC safepoints and barriers

Instruction schemas declare whether an instruction is a safepoint, may allocate, may call runtime code, or may write managed references. Allocation, calls, dynamic calls, throws, yields, and native/foreign boundaries that can allocate, block, or call back are safepoints. At a safepoint, SEAM must be able to enumerate live managed references from verifier-derived stack maps.

Stores through `st.ref`, `st.fld`, `st.idx`, `st.elem`, `st.key`, representation transitions, and core storage operations carry a write-barrier obligation when the target layout can contain managed references. The instruction does not name cards, remembered sets, or Immix internals; those are SEAM implementation details.

## Unknowns

- Exact trap taxonomy is not fully specified.
- Exact numeric overflow and floating-point exception behavior is not fully specified.
- Exact pointer-region permission metadata is not fully specified.
