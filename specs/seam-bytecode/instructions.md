# SEAM bytecode instruction semantics

Locked core SEAM bytecode instruction families have VM behavior. `specs/seam-bytecode/opcodes.md` lists ids, mnemonics, operand schemas, stack-effect schemas from `seam_bytecode_opcodes.def`.

Project evidence: `LOCKED_LANGUAGE_DESIGN.md` opcode semantics, `seam_bytecode_opcodes.def`.

Reference: WebAsm defines stack-machine behavior independent from source languages: <https://webassembly.github.io/spec/core/exec/instructions.html>.

## General rules

SEAM bytecode is typed stack-effect instruction language. Instructions consume stack values, immediate operands, and module/body metadata. Semantics are VM-oriented, not Musi syntax, while preserving type/source relationship for analysis and tooling.

Rightmost stack value = top. Text uses canonical mnemonics; binary uses stable `u16` ids. Binary operands decode only through active opcode schema.

## Control and terminators

- `nop`: no state change.
- `ret`: return outputs matching current callable signature; terminates frame.
- `br`: jump to body-local block target.
- `br.true` / `br.false`: consume `Bit`; branch or fall through.
- `br.tbl`: consume natural selector; branch through body-local table.
- `leave`: exit cleanup/region edge by `region_idx`.
- `trap`: terminate through trap/failure channel.
- `throw`: consume value and enter exceptional metadata edge.
- `rethrow`: continue active exceptional edge.

Handler/catch/finally shape is metadata, not opcode family.

## Stack operations

`drop`, `dup`, `swap`, `rot` operate on stack values without changing types. Verification rejects unavailable stack prefix.

## Consts and frame storage

`const` loads typed constant-table entry. Inline constants load scalar values from instruction stream. Frame ops access arg, local, env/capture, global namespaces. Stores require assignment compatibility with destination storage type.

## Calls and callable values

- `call`: static callable declaration.
- `call.disp`: receiver dispatch via metadata.
- `call.ind`: callable value constrained by signature.
- `call.dyn`: dynamic-call protocol + arg pack.
- `mk.fn`: callable value from callable ref + env/capture values.

Callee origin lives in declaration metadata: SEAM bytecode body, intrin/runtime binding, extern/foreign binding, or core runtime target.

## Scalar arithmetic, bitwise operations, and comparisons

Arithmetic, bitwise, shift, rotate, unary numeric, and comparison ops use scalar types admitted by core numeric rules. Signed/unsigned variants are distinct integer modes. Division by zero, invalid shifts, overflow, NaN ordering, and checked-conversion failures use core trap/diagnostic behavior.

`rem` = CPU-style remainder. `.un` suffix on division, remainder, comparison = unsigned integer mode.

## Type tests and conversions

`test.ty` returns `Bit`. `cast.ty` checked cast/coercion by core type rules. `conv` ordinary conversion. `conv.chk` checked conversion. `bitcast` reinterprets representation-compatible values. `conv.repr` crosses declared core representations.

## Memory, references, access, pointers, and allocation

`ld.ref` / `st.ref` operate on managed refs. `ld.ptr` / `st.ptr` operate on VM pointer/access values with explicit target types. Musi `Access[T]` and `Access[mut T]` lower to these ops plus layout, region, permission, capability metadata. `ld.addr` materializes address/ref described by body metadata. `alloc` / `alloc.arr` allocate runtime storage described by type/layout metadata.

`mem.copy`, `mem.fill`, `mem.move` operate on explicit pointer/access-addressable storage. Aliasing, overlap, trap behavior, and region/access permissions are SEAM rules.

`size` returns layout size of type as `Nat`.

## Products, sums, and indexed storage

Products = records or positional products. `mk.prod` constructs value from exactly required fields. `ld.fld`, `st.fld`, `addr.fld` use named fields. `ld.idx`, `st.idx`, `addr.idx` use positional fields.

Sums = tagged variants. `mk.sum` constructs tagged value with optional payload. `ld.tag` exposes tag identity as `Nat`; `is.tag` tests alt; payload ops require active alt with payload.

Indexed storage ops address runtime-indexed arrays/sequences. `mk.arr` consumes encoded element count from stack.

## Dynamic, capability, keyed storage, suspension, and cleanup

`box` / `unbox` are representation transitions. `cap.has` / `cap.need` use VM capability evidence. `ld.key`, `st.key`, `has.key`, `del.key` use explicit keyed-storage metadata; not implicit dynamic lookup on `Any`.

`yld` suspends by yield/resume metadata. `cln.push`, `cln.pop`, `cln.run` manage cleanup regions tied to region metadata.

## GC safepoints and barriers

Instruction schemas declare safepoint, allocation, runtime-call, and managed-ref-write behavior. Allocation, calls, dynamic calls, throws, yields, and native/foreign boundaries that can allocate/block/call back are safepoints. At safepoint, SEAM must enumerate live managed refs from verifier stack maps.

Stores through `st.ref`, `st.fld`, `st.idx`, `st.elem`, `st.key`, representation transitions, and core storage ops carry write-barrier obligation when target layout can contain managed refs. Instruction does not name cards, remembered sets, or Immix internals; those are SEAM details.

## Unknowns

- Exact trap taxonomy not fully specified.
- Exact numeric overflow + floating exception behavior not fully specified.
- Exact access/region permission metadata not fully specified.
