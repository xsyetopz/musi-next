# SEAM bytecode opcode registry

Project evidence: `seam_bytecode_opcodes.def`.

The locked core opcode registry is authoritative for core opcode ids, canonical mnemonics, operand schemas, and stack-effect schemas. `.seam` image stores numeric opcode ids; SEAM bytecode text/disassembly stores canonical mnemonics.

## Registry rules

- Opcode ids are `u16`.
- Opcode ids are assigned by sparse family range, not list order.
- Opcode ids never change meaning.
- Removed ids remain reserved forever.
- Unknown opcode ids are loader/verifier diagnostics unless ext metadata declares them.
- Ext opcodes require module feature metadata.
- `bad` (`0x0000`) is an invalid sentinel, not an executable no-op.

## Family ranges

| Range            | Family                                          |
| ---------------- | ----------------------------------------------- |
| `0x0000..0x00FF` | control / terminators                           |
| `0x0100..0x01FF` | stack                                           |
| `0x0200..0x02FF` | constants                                       |
| `0x0300..0x03FF` | frame / globals                                 |
| `0x0400..0x04FF` | calls / dispatch                                |
| `0x0500..0x05FF` | scalar arithmetic                               |
| `0x0600..0x06FF` | bitwise / shifts / rotates                      |
| `0x0700..0x07FF` | comparison / tests                              |
| `0x0800..0x08FF` | conversion / reinterpretation                   |
| `0x0900..0x09FF` | memory / refs / VM pointers/access / allocation |
| `0x0A00..0x0AFF` | product layout                                  |
| `0x0B00..0x0BFF` | sum / tag / payload                             |
| `0x0C00..0x0CFF` | indexed storage                                 |
| `0x0D00..0x0DFF` | reserved core future                            |
| `0x0E00..0x0EFF` | dynamic / capability / keyed storage            |
| `0x0F00..0x0FFF` | reserved core future                            |
| `0x1000..0x10FF` | suspension / yield                              |
| `0x1100..0x11FF` | cleanup edges                                   |
| `0x1200..0x1FFF` | reserved core future                            |
| `0x2000..0xEFFF` | standard extensions                             |
| `0xF000..0xFFFF` | private/vendor                                  |

## Locked core opcodes

| Raw value | Swift case   | Mnemonic      | Operands         | Stack effect                                  |
| --------: | ------------ | ------------- | ---------------- | --------------------------------------------- |
|  `0x0000` | `bad`        | `bad`         | `none`           | `bad`                                         |
|  `0x0001` | `nop`        | `nop`         | `none`           | `... -> ...`                                  |
|  `0x0002` | `ret`        | `ret`         | `none`           | `..., outputs(current) -> terminal`           |
|  `0x0003` | `br`         | `br`          | `block_idx`      | `... -> terminal`                             |
|  `0x0004` | `brTrue`     | `br.true`     | `block_idx`      | `..., Bit -> terminal-or-next`                |
|  `0x0005` | `brFalse`    | `br.false`    | `block_idx`      | `..., Bit -> terminal-or-next`                |
|  `0x0006` | `brTbl`      | `br.tbl`      | `table_idx`      | `..., Nat -> terminal`                        |
|  `0x0007` | `leave`      | `leave`       | `region_idx`     | `... -> terminal-or-region-exit`              |
|  `0x0008` | `trap`       | `trap`        | `none`           | `... -> terminal`                             |
|  `0x0009` | `throwOp`    | `throw`       | `none`           | `..., T -> terminal`                          |
|  `0x000A` | `rethrowOp`  | `rethrow`     | `none`           | `... -> terminal`                             |
|  `0x0100` | `drop`       | `drop`        | `none`           | `..., A -> ...`                               |
|  `0x0101` | `dup`        | `dup`         | `none`           | `..., A -> ..., A, A`                         |
|  `0x0102` | `swap`       | `swap`        | `none`           | `..., A, B -> ..., B, A`                      |
|  `0x0103` | `rot`        | `rot`         | `none`           | `..., A, B, C -> ..., B, C, A`                |
|  `0x0200` | `const`      | `const`       | `const_idx`      | `... -> ..., T`                               |
|  `0x0202` | `constBit`   | `const.bit`   | `u8`             | `... -> ..., Bit`                             |
|  `0x0203` | `constNil`   | `const.nil`   | `type_idx`       | `... -> ..., T`                               |
|  `0x0210` | `constInt`   | `const.int`   | `type_idx, vari` | `... -> ..., T`                               |
|  `0x0211` | `constNat`   | `const.nat`   | `type_idx, varu` | `... -> ..., T`                               |
|  `0x0212` | `constFlt`   | `const.flt`   | `type_idx, f64`  | `... -> ..., T`                               |
|  `0x0213` | `constTxt`   | `const.txt`   | `const_idx`      | `... -> ..., T`                               |
|  `0x0214` | `constBytes` | `const.bytes` | `const_idx`      | `... -> ..., T`                               |
|  `0x0300` | `ldArg`      | `ld.arg`      | `arg_idx`        | `... -> ..., T`                               |
|  `0x0301` | `ldLoc`      | `ld.loc`      | `loc_idx`        | `... -> ..., T`                               |
|  `0x0302` | `stLoc`      | `st.loc`      | `loc_idx`        | `..., T -> ...`                               |
|  `0x0303` | `ldEnv`      | `ld.env`      | `env_idx`        | `... -> ..., T`                               |
|  `0x0304` | `stEnv`      | `st.env`      | `env_idx`        | `..., T -> ...`                               |
|  `0x0305` | `ldGlobal`   | `ld.global`   | `global_idx`     | `... -> ..., T`                               |
|  `0x0306` | `stGlobal`   | `st.global`   | `global_idx`     | `..., T -> ...`                               |
|  `0x0400` | `call`       | `call`        | `func_idx`       | `..., inputs(S) -> ..., outputs(S)`           |
|  `0x0401` | `callDisp`   | `call.disp`   | `func_idx`       | `..., receiver, inputs(S) -> ..., outputs(S)` |
|  `0x0402` | `callInd`    | `call.ind`    | `sig_idx`        | `..., Fn[S], inputs(S) -> ..., outputs(S)`    |
|  `0x0403` | `callDyn`    | `call.dyn`    | `sig_idx`        | `..., callee, argpack -> ..., outputs(S)`     |
|  `0x0404` | `mkFn`       | `mk.fn`       | `func_idx`       | `..., env? -> ..., Fn[S]`                     |
|  `0x0500` | `add`        | `add`         | `none`           | `..., T, T -> ..., T`                         |
|  `0x0501` | `sub`        | `sub`         | `none`           | `..., T, T -> ..., T`                         |
|  `0x0502` | `mul`        | `mul`         | `none`           | `..., T, T -> ..., T`                         |
|  `0x0503` | `div`        | `div`         | `none`           | `..., T, T -> ..., T`                         |
|  `0x0504` | `divUn`      | `div.un`      | `none`           | `..., T, T -> ..., T`                         |
|  `0x0505` | `rem`        | `rem`         | `none`           | `..., T, T -> ..., T`                         |
|  `0x0506` | `remUn`      | `rem.un`      | `none`           | `..., T, T -> ..., T`                         |
|  `0x0507` | `neg`        | `neg`         | `none`           | `..., T -> ..., T`                            |
|  `0x0508` | `abs`        | `abs`         | `none`           | `..., T -> ..., T`                            |
|  `0x0509` | `addChk`     | `add.chk`     | `none`           | `..., T, T -> ..., T`                         |
|  `0x050A` | `subChk`     | `sub.chk`     | `none`           | `..., T, T -> ..., T`                         |
|  `0x050B` | `mulChk`     | `mul.chk`     | `none`           | `..., T, T -> ..., T`                         |
|  `0x0600` | `and`        | `and`         | `none`           | `..., T, T -> ..., T`                         |
|  `0x0601` | `or`         | `or`          | `none`           | `..., T, T -> ..., T`                         |
|  `0x0602` | `xor`        | `xor`         | `none`           | `..., T, T -> ..., T`                         |
|  `0x0603` | `not`        | `not`         | `none`           | `..., T -> ..., T`                            |
|  `0x0610` | `shl`        | `shl`         | `none`           | `..., T, Nat -> ..., T`                       |
|  `0x0611` | `shr`        | `shr`         | `none`           | `..., T, Nat -> ..., T`                       |
|  `0x0612` | `sar`        | `sar`         | `none`           | `..., T, Nat -> ..., T`                       |
|  `0x0613` | `rol`        | `rol`         | `none`           | `..., T, Nat -> ..., T`                       |
|  `0x0614` | `ror`        | `ror`         | `none`           | `..., T, Nat -> ..., T`                       |
|  `0x0700` | `cmpEq`      | `cmp.eq`      | `none`           | `..., A, A -> ..., Bit`                       |
|  `0x0701` | `cmpNe`      | `cmp.ne`      | `none`           | `..., A, A -> ..., Bit`                       |
|  `0x0702` | `cmpLt`      | `cmp.lt`      | `none`           | `..., T, T -> ..., Bit`                       |
|  `0x0703` | `cmpLe`      | `cmp.le`      | `none`           | `..., T, T -> ..., Bit`                       |
|  `0x0704` | `cmpGt`      | `cmp.gt`      | `none`           | `..., T, T -> ..., Bit`                       |
|  `0x0705` | `cmpGe`      | `cmp.ge`      | `none`           | `..., T, T -> ..., Bit`                       |
|  `0x0706` | `cmpLtUn`    | `cmp.lt.un`   | `none`           | `..., T, T -> ..., Bit`                       |
|  `0x0707` | `cmpLeUn`    | `cmp.le.un`   | `none`           | `..., T, T -> ..., Bit`                       |
|  `0x0708` | `cmpGtUn`    | `cmp.gt.un`   | `none`           | `..., T, T -> ..., Bit`                       |
|  `0x0709` | `cmpGeUn`    | `cmp.ge.un`   | `none`           | `..., T, T -> ..., Bit`                       |
|  `0x0710` | `testTy`     | `test.ty`     | `type_idx`       | `..., A -> ..., Bit`                          |
|  `0x0711` | `castTy`     | `cast.ty`     | `type_idx`       | `..., A -> ..., T`                            |
|  `0x0800` | `conv`       | `conv`        | `type_idx`       | `..., A -> ..., T`                            |
|  `0x0801` | `convChk`    | `conv.chk`    | `type_idx`       | `..., A -> ..., T`                            |
|  `0x0802` | `bitcast`    | `bitcast`     | `type_idx`       | `..., A -> ..., T`                            |
|  `0x0803` | `convRepr`   | `conv.repr`   | `type_idx`       | `..., A -> ..., T`                            |
|  `0x0900` | `ldRef`      | `ld.ref`      | `none`           | `..., Ref[T] -> ..., T`                       |
|  `0x0901` | `stRef`      | `st.ref`      | `none`           | `..., Ref[T], T -> ...`                       |
|  `0x0902` | `ldPtr`      | `ld.ptr`      | `type_idx`       | `..., Ptr[T] -> ..., T`                       |
|  `0x0903` | `stPtr`      | `st.ptr`      | `type_idx`       | `..., Ptr[T], T -> ...`                       |
|  `0x0904` | `ldAddr`     | `ld.addr`     | `addr_idx`       | `... -> ..., Ref[T]`                          |
|  `0x0905` | `alloc`      | `alloc`       | `type_idx`       | `... -> ..., Ref[T]`                          |
|  `0x0906` | `allocArr`   | `alloc.arr`   | `type_idx`       | `..., Nat -> ..., Ref[T]`                     |
|  `0x0910` | `memCopy`    | `mem.copy`    | `none`           | `..., Ptr[dst], Ptr[src], Nat -> ...`         |
|  `0x0911` | `memFill`    | `mem.fill`    | `none`           | `..., Ptr[dst], Byte, Nat -> ...`             |
|  `0x0912` | `memMove`    | `mem.move`    | `none`           | `..., Ptr[dst], Ptr[src], Nat -> ...`         |
|  `0x0913` | `size`       | `size`        | `type_idx`       | `... -> ..., Nat`                             |
|  `0x0A00` | `mkProd`     | `mk.prod`     | `type_idx`       | `..., fields(T) -> ..., T`                    |
|  `0x0A01` | `ldFld`      | `ld.fld`      | `field_idx`      | `..., P -> ..., F`                            |
|  `0x0A02` | `stFld`      | `st.fld`      | `field_idx`      | `..., Ref[P], F -> ...`                       |
|  `0x0A04` | `addrFld`    | `addr.fld`    | `field_idx`      | `..., Ref[P] -> ..., Ref[F]`                  |
|  `0x0A10` | `ldIdx`      | `ld.idx`      | `field_idx`      | `..., P -> ..., F`                            |
|  `0x0A11` | `stIdx`      | `st.idx`      | `field_idx`      | `..., Ref[P], F -> ...`                       |
|  `0x0A13` | `addrIdx`    | `addr.idx`    | `field_idx`      | `..., Ref[P] -> ..., Ref[F]`                  |
|  `0x0B00` | `mkSum`      | `mk.sum`      | `alt_idx`        | `..., payld? -> ..., S`                       |
|  `0x0B01` | `ldTag`      | `ld.tag`      | `type_idx`       | `..., S -> ..., Nat`                          |
|  `0x0B02` | `isTag`      | `is.tag`      | `alt_idx`        | `..., S -> ..., Bit`                          |
|  `0x0B03` | `ldPayld`    | `ld.payld`    | `alt_idx`        | `..., S -> ..., P`                            |
|  `0x0B04` | `hasPayld`   | `has.payld`   | `alt_idx`        | `..., S -> ..., Bit`                          |
|  `0x0B05` | `addrPayld`  | `addr.payld`  | `alt_idx`        | `..., Ref[S] -> ..., Ref[P]`                  |
|  `0x0C00` | `mkArr`      | `mk.arr`      | `type_idx, varu` | `..., elems[N] -> ..., T`                     |
|  `0x0C01` | `len`        | `len`         | `none`           | `..., T -> ..., Nat`                          |
|  `0x0C02` | `ldElem`     | `ld.elem`     | `none`           | `..., T, Nat -> ..., E`                       |
|  `0x0C03` | `stElem`     | `st.elem`     | `none`           | `..., Ref[T], Nat, E -> ...`                  |
|  `0x0C04` | `addrElem`   | `addr.elem`   | `none`           | `..., Ref[T], Nat -> ..., Ref[E]`             |
|  `0x0E00` | `box`        | `box`         | `type_idx`       | `..., T -> ..., B`                            |
|  `0x0E01` | `unbox`      | `unbox`       | `type_idx`       | `..., B -> ..., T`                            |
|  `0x0E02` | `capHas`     | `cap.has`     | `cap_idx`        | `..., A -> ..., Bit`                          |
|  `0x0E03` | `capNeed`    | `cap.need`    | `cap_idx`        | `..., A -> ..., A`                            |
|  `0x0E04` | `ldKey`      | `ld.key`      | `none`           | `..., Obj, Key -> ..., Value`                 |
|  `0x0E05` | `stKey`      | `st.key`      | `none`           | `..., Obj, Key, Value -> ...`                 |
|  `0x0E06` | `hasKey`     | `has.key`     | `none`           | `..., Obj, Key -> ..., Bit`                   |
|  `0x0E07` | `delKey`     | `del.key`     | `none`           | `..., Obj, Key -> ..., Bit`                   |
|  `0x1000` | `yld`        | `yld`         | `sig_idx`        | `..., yield(S) -> ..., resume(S)`             |
|  `0x1100` | `clnPush`    | `cln.push`    | `region_idx`     | `... -> ...`                                  |
|  `0x1101` | `clnPop`     | `cln.pop`     | `region_idx`     | `... -> ...`                                  |
|  `0x1102` | `clnRun`     | `cln.run`     | `region_idx`     | `... -> ...`                                  |

## Schema ownership

A verified module has exactly one active schema for each accepted opcode id. Core exts may add ext/private schemas but must not override locked core schemas. Ext opcode schemas must be available before decoding operands beyond the opcode id.
