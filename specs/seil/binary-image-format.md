# SEAM Binary Image Format

`.seil` is textual executable IL. SEAM tooling may assemble `.seil` into dense binary image for loading, caching, package distribution, or execution. Image is not public SEIL source format.

Binary image favors loader-friendly density over binary golf:

- fixed probe header;
- section directory;
- compact section families with typed row kinds;
- interned names;
- `u16` opcode ids;
- schema-ordered operands using fixed scalars and `varu`/`vari`;
- skippable tool metadata.

Compression, checksums, signatures, archive transport belong to package/container layer, not core image.

## Fixed Header

Binary image begins with exactly 40-byte header. Header = loader-probe data; semantic module data lives in sections.

| Offset | Size | Field       | Meaning                                                                     |
| -----: | ---: | ----------- | --------------------------------------------------------------------------- |
|      0 |    4 | `magic`     | ASCII `SEAM`, modeled as `u32`                                              |
|      4 |    4 | `format`    | `(major: u8, minor: u8, header_size: u8, flags: u8)`; `header_size` is `40` |
|      8 |   24 | `sections`  | `(count: u32, reserved: u32, offset: u64, size: u64)`                       |
|     32 |    8 | `file_size` | total image size as `u64`                                                   |

`format.flags` and `sections.reserved` are `0` now. Fixed-width ints and floating payloads little-endian. Single-byte fields byte-order independent. Magic bytes: `53 45 41 4D` (`SEAM`).

## Section Directory

Directory starts at `header.sections.offset`, has `header.sections.count` entries, spans `header.sections.size` bytes. Each entry = 32 bytes:

| Offset | Size | Field           | Meaning                                           |
| -----: | ---: | --------------- | ------------------------------------------------- |
|      0 |    2 | `kind`          | numeric section-family id                         |
|      2 |    2 | `flags`         | payload encoding flags; `0` means plain payload   |
|      4 |    4 | `reserved`      | `0`                                               |
|      8 |    8 | `offset`        | absolute image offset of payload                  |
|     16 |    8 | `size`          | payload byte length                               |
|     24 |    4 | `count`         | logical entry count, or `0` when not table-shaped |
|     28 |    1 | `align_log2`    | required payload alignment as log2 bytes          |
|     29 |    3 | `reserved_tail` | `0`                                               |

Entries sorted by `(offset, kind)`. Payload ranges must not overlap and must fit inside `file_size`. Non-zero reserved fields or unsupported flags reject image.

## Section Kinds

|   Id | Family  |
| ---: | ------- |
|    1 | `names` |
|    2 | `asm`   |
|    3 | `deps`  |
|    4 | `defs`  |
|    5 | `code`  |
|    6 | `data`  |
|    7 | `meta`  |
|    8 | `tool`  |

`asm` mandatory and decodable using only core image format. `deps` decoded before dependent semantic payloads. Unknown core semantic sections reject. Extension payloads are not new core section families; they use declared row kinds inside `data`, `meta`, or `tool`.

## Section Payload Shape

Each section payload:

```text
row_kind_directory
row_offset_table
packed_row_bytes
```

Row-kind directory first. It lists row kinds in section. Each row-kind entry records row kind id, row count, row offset-table range, payload range, row schema id or core schema tag, required/skippable policy. Row offset table follows and gives per-row byte offsets into `packed_row_bytes`. Row bytes are schema-packed; field names not encoded.

This lets loader skip unsupported skippable row kinds, reject unsupported required row kinds before deep decode, and jump directly by namespace-relative index. Section must not encode rows outside declared row-kind directory.

## Rows And Indices

Semantic sections encode compact typed rows. Text symbols intern into `names`. Table refs are namespace-relative `varu`.

Rows use schemas owned by section + row kind. Field names not encoded. Optional fields use presence bits or row-specific tags.

Core row families:

| Section | Row kinds                                                                    |
| ------- | ---------------------------------------------------------------------------- |
| `names` | names, strings                                                               |
| `asm`   | current assembly identity, version, entry                                    |
| `deps`  | runtime/cap/ext requirements, asm refs, imports                              |
| `defs`  | types, fields, alts, sigs, inputs, outputs, globals, consts, procs, exports  |
| `code`  | bodies, blocks, regions, branch tables, address targets, instruction bytes   |
| `data`  | constant payloads, layouts, reference maps, ABI records, dynamic/cap schemas |
| `meta`  | required semantic metadata not owned by `defs`, `code`, or `data`            |
| `tool`  | optional non-semantic source/tool metadata                                   |

`tool` rows skippable only when core row schema marks non-semantic. Required executable semantics must not depend on `tool`.

## Instruction Encoding

Instruction bodies encode:

```text
opcode: u16
operands: schema-ordered bytes
```

No generic operand-count byte. Operands follow accepted opcode schema from `seil_opcodes.def`.

Primitive operand encodings:

| Encoding         | Meaning                                     |
| ---------------- | ------------------------------------------- |
| `u8/u16/u32/u64` | fixed-width unsigned integer                |
| `i8/i16/i32/i64` | fixed-width two's-complement signed integer |
| `f32/f64`        | IEEE-754 binary32/binary64                  |
| `varu`           | unsigned LEB128                             |
| `vari`           | signed LEB128                               |

`varu` and `vari` use shortest form.

## Loader Validation

Before verification, SEAM checks:

- header magic `SEAM`;
- header size `40`;
- reserved fields and unsupported flags zero;
- section directory inside `file_size`;
- section payloads inside `file_size`;
- exactly one mandatory `asm` section;
- required semantic section families and rows present;
- `deps` decode before dependent semantic rows;
- each section payload has valid row-kind directory, row offset table, packed row byte range;
- logical tables decode before operand resolution;
- `tool` rows can skip without changing execution.

## Unknowns

- Exact section-family ids beyond core not specified.
- Exact per-row binary schemas not fully specified.
- Exact package/container transport not specified.
