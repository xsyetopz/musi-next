# 15. Text, Rune, and Raw Bytes

Status: normative for source names and surface policy.

## Core names

```text
Text   immutable Unicode text
Rune   Unicode scalar value
```

No `String` and no `Char` core names exist.

Raw bytes are numeric sequences:

```text
Vec[Nat8]
Slice[Nat8]
Slice[mut Nat8]
```

## Text policy

```text
Text is immutable.
Text storage representation is not exposed.
Text is not random-indexed by numeric “character offset”.
Rune iteration is explicit.
Grapheme segmentation is library-level.
Encoding and decoding are explicit.
```

Example:

```musi
let text : Text := "hello";
let r : Rune := 'λ';

let bytes : Vec[Nat8] := text.encodeUtf8();
let decoded : DecodeError!Text := Text.decodeUtf8(bytes);
```

## Length naming

Text APIs should avoid ambiguous plain `len()`.

Prefer explicit names:

```text
byteLen
runeCount
graphemeCount
```
