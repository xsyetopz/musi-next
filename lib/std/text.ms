--! Text module.

let ascii := import "@std/ascii";
let textHost := import "musi:text";

--- Return byte length.
let textLength (target : String) : Int := textHost.length(target);

--- Return byte length.
export let length (target : String) : Int := textLength(target);

--- Function for `isEmpty`.
let textIsEmpty (target : String) : Bit := textLength(target) = 0;

--- Returns true when text has no bytes.
export let isEmpty (target : String) : Bit := textIsEmpty(target);

--- Concatenate text.
let textConcat (left : String, right : String) : String :=
  textHost.concat(left, right);

--- Concatenate text.
export let concat (left : String, right : String) : String :=
  textConcat(left, right);

--- Return byte-range text slice on character boundaries.
let textSlice (target : String, start : Int, end : Int) : String :=
  textHost.slice(target, start, end);

--- Return byte-range text slice on character boundaries.
export let slice (target : String, start : Int, end : Int) : String :=
  textSlice(target, start, end);

--- Return byte value at index.
let textByteAt (target : String, index : Int) : Int :=
  textHost.byteAt(target, index);

--- Return byte value at index.
export let byteAt (target : String, index : Int) : Int :=
  textByteAt(target, index);

--- Build one-byte text from byte value.
let textFromByte (value : Int) : String := textHost.fromByte(value);

--- Build one-byte text from byte value.
export let fromByte (value : Int) : String := textFromByte(value);

let recur trimStartIndex (target : String, index : Int, end : Int) : Int :=
  if index >= end then end
  else if ascii.isWhitespace(textByteAt(target, index)) then trimStartIndex(
    target,
    index + 1,
    end,
  ) else index;

let recur trimEndIndex (target : String, start : Int, end : Int) : Int :=
  if end <= start then start
  else if ascii.isWhitespace(textByteAt(target, end - 1)) then trimEndIndex(
    target,
    start,
    end - 1,
  ) else end;

--- Function for `trim`.
let textTrim (target : String) : String :=
  (
    let end := textLength(target);
    let start := trimStartIndex(target, 0, end);
    textSlice(target, start, trimEndIndex(target, start, end))
  );

--- Trim leading and trailing ASCII whitespace.
export let trim (target : String) : String := textTrim(target);

let recur transformAscii (
  target : String,
  index : Int,
  end : Int,
  mapper : Int -> Int
) : String := if index >= end then ""
  else textConcat(
    textFromByte(mapper(textByteAt(target, index))),
    transformAscii(target, index + 1, end, mapper),
  );

--- Function for `toLowerCase`.
let textToLowerCase (target : String) : String :=
  transformAscii(target, 0, textLength(target), ascii.toLower);

--- Convert ASCII letters to lower case.
export let toLowerCase (target : String) : String := textToLowerCase(target);

--- Function for `toUpperCase`.
let textToUpperCase (target : String) : String :=
  transformAscii(target, 0, textLength(target), ascii.toUpper);

--- Convert ASCII letters to upper case.
export let toUpperCase (target : String) : String := textToUpperCase(target);

let bytesDifferAt (
  target : String,
  needle : String,
  targetIndex : Int,
  needleIndex : Int
) : Bit := textByteAt(target, targetIndex) /= textByteAt(needle, needleIndex);

let recur matchesAt (
  target : String,
  needle : String,
  targetIndex : Int,
  needleIndex : Int
) : Bit := if needleIndex >= textLength(needle) then 0 = 0
  else if targetIndex >= textLength(target) then 0 = 1
  else if bytesDifferAt(target, needle, targetIndex, needleIndex) then 0 = 1
  else matchesAt(target, needle, targetIndex + 1, needleIndex + 1);

let recur indexOfFrom (target : String, needle : String, index : Int) : Int :=
  if textLength(needle) = 0 then index
  else if index > textLength(target) - textLength(needle) then - 1
  else if matchesAt(target, needle, index, 0) then index
  else indexOfFrom(target, needle, index + 1);

--- Function for `indexOf`.
let textIndexOf (target : String, needle : String) : Int :=
  indexOfFrom(target, needle, 0);

--- Return first byte index of needle, or -1 when missing.
export let indexOf (target : String, needle : String) : Int :=
  textIndexOf(target, needle);

--- Function for `contains`.
let textContains (target : String, needle : String) : Bit :=
  textIndexOf(target, needle) >= 0;

--- Returns true when target contains needle.
export let contains (target : String, needle : String) : Bit :=
  textContains(target, needle);

--- Function for `startsWith`.
let textStartsWith (target : String, prefix : String) : Bit :=
  matchesAt(target, prefix, 0, 0);

--- Returns true when target starts with prefix.
export let startsWith (target : String, prefix : String) : Bit :=
  textStartsWith(target, prefix);

--- Function for `endsWith`.
let textEndsWith (target : String, suffix : String) : Bit :=
  if textLength(suffix) > textLength(target) then 0 = 1
  else matchesAt(target, suffix, textLength(target) - textLength(suffix), 0);

--- Returns true when target ends with suffix.
export let endsWith (target : String, suffix : String) : Bit :=
  textEndsWith(target, suffix);

--- Count Unicode scalar values in string.
export let (self : String).length () : Int := textLength(self);

--- Returns true when string is empty.
export let (self : String).isEmpty () : Bit := textIsEmpty(self);

--- Trim surrounding whitespace.
export let (self : String).trim () : String := textTrim(self);

--- Concatenate strings.
export let (self : String).concat (other : String) : String :=
  textConcat(self, other);

--- Convert to lower case.
export let (self : String).toLowerCase () : String := textToLowerCase(self);

--- Convert to upper case.
export let (self : String).toUpperCase () : String := textToUpperCase(self);

--- Returns true when string contains substring.
export let (self : String).contains (needle : String) : Bit :=
  textContains(self, needle);

--- Returns true when string starts with prefix.
export let (self : String).startsWith (prefix : String) : Bit :=
  textStartsWith(self, prefix);

--- Returns true when string ends with suffix.
export let (self : String).endsWith (suffix : String) : Bit :=
  textEndsWith(self, suffix);

--- Return first byte index for substring, or -1 when missing.
export let (self : String).indexOf (needle : String) : Int :=
  textIndexOf(self, needle);

--- Return byte-range string slice on character boundaries.
export let (self : String).slice (start : Int, end : Int) : String :=
  textSlice(self, start, end);
