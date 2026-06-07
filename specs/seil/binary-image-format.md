# SEAM Binary Image Format

`.seil` is textual executable IL. SEAM tooling may assemble `.seil` into a dense binary image for loading, caching, distribution inside a package, or execution. That image is not the public SEIL source format.

The binary image keeps loader-friendly density rather than absolute binary golf:

- fixed probe header;
- section directory;
- typed table sections;
- interned names;
- `u16` opcode ids;
- schema-ordered operands using fixed-width scalars and `varu`/`vari`;
- skippable tool metadata.

Compression, checksums, signatures, and archive transport belong to a package/container layer, not the core image.

## Fixed Header

A binary image begins with an exactly 40-byte header. The header is loader-probe data; semantic module data lives in sections.

| Offset | Size | Field       | Meaning                                                                     |
| -----: | ---: | ----------- | --------------------------------------------------------------------------- |
|      0 |    4 | `magic`     | ASCII `SEAM`, modeled as `u32`                                              |
|      4 |    4 | `format`    | `(major: u8, minor: u8, header_size: u8, flags: u8)`; `header_size` is `40` |
|      8 |   24 | `sections`  | `(count: u32, reserved: u32, offset: u64, size: u64)`                       |
|     32 |    8 | `file_size` | total image size as `u64`                                                   |

`format.flags` and `sections.reserved` are `0` in the current format. Fixed-width integers and floating payloads are little-endian. Single-byte fields are byte-order independent. The magic byte sequence is `53 45 41 4D` (`SEAM`).

## Section Directory

The section directory starts at `header.sections.offset`, contains `header.sections.count` entries, and occupies `header.sections.size` bytes. Each directory entry is 32 bytes:

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

Section entries are sorted by `(offset, kind)`. Payload ranges must not overlap and must fit inside `file_size`. Non-zero reserved fields and unsupported flags reject the image.

## Section Kinds

|            Id | Family                  |
| ------------: | ----------------------- |
|             1 | `names`                 |
|             2 | `asm`                   |
|             3 | `asmrefs`               |
|             4 | `types`                 |
|             5 | `sigs`                  |
|             6 | `consts`                |
|             7 | `imports`               |
|             8 | `exports`               |
|             9 | `procs`                 |
|            10 | `layouts`               |
|            11 | `body-meta`             |
|            12 | `bodies`                |
|            13 | `tool-meta`             |
|  2000..=32767 | standard ext sections   |
| 32768..=65535 | private/vendor sections |

`asm` is mandatory and must be decodable using only the core image format. `tool-meta` is optional and non-semantic. Unknown semantic sections reject the image. Unknown tool metadata is skippable only when core metadata marks it non-semantic.

## Tables And Indices

Semantic sections encode compact typed tables. Text symbols are interned into the `names` section. Table refs are namespace-relative `varu` values.

Rows use compact schemas owned by their section kind. Field names are not encoded in rows. Optional row fields use presence bits or row-specific tags.

## Instruction Encoding

Instruction bodies encode:

```text
opcode: u16
operands: schema-ordered bytes
```

There is no generic operand-count byte. Operands follow the accepted opcode schema from `seil_opcodes.def`.

Primitive operand encodings:

| Encoding         | Meaning                                     |
| ---------------- | ------------------------------------------- |
| `u8/u16/u32/u64` | fixed-width unsigned integer                |
| `i8/i16/i32/i64` | fixed-width two's-complement signed integer |
| `f32/f64`        | IEEE-754 binary32/binary64                  |
| `varu`           | unsigned LEB128                             |
| `vari`           | signed LEB128                               |

`varu` and `vari` use shortest-form encodings.

## Loader Validation

Before verification, SEAM checks:

- header magic is `SEAM`;
- header size is `40`;
- reserved fields and unsupported flags are zero;
- section directory lies inside `file_size`;
- section payloads lie inside `file_size`;
- exactly one mandatory `asm` section exists;
- required semantic sections are present;
- logical tables decode before operand resolution;
- tool metadata can be skipped without changing execution.
