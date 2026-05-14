--! ASCII character classification and case helpers.

let maybe := import "@std/maybe";
let Maybe := maybe.Maybe;

--- Returns true when value is in inclusive ASCII byte range.
export let inRange (value : Int, lowerBound : Int, upperBound : Int) : Bit :=
  if value < lowerBound then 0 = 1 else value <= upperBound;

--- Returns true for ASCII byte values.
export let isAscii (value : Int) : Bit := inRange(value, 0, 127);

--- Returns true for ASCII control bytes.
export let isControl (value : Int) : Bit :=
  if value = 127 then 0 = 0 else inRange(value, 0, 31);

--- Returns true for printable ASCII bytes.
export let isPrintable (value : Int) : Bit := inRange(value, 32, 126);

--- Returns true for ASCII whitespace bytes.
export let isWhitespace (value : Int) : Bit :=
  match value (
  | 9 => 0 = 0
  | 10 => 0 = 0
  | 11 => 0 = 0
  | 12 => 0 = 0
  | 13 => 0 = 0
  | 32 => 0 = 0
  | _ => 0 = 1
  );

--- Returns true for ASCII decimal digits.
export let isDigit (value : Int) : Bit := inRange(value, 48, 57);

--- Returns true for lowercase ASCII letters.
export let isLower (value : Int) : Bit := inRange(value, 97, 122);

--- Returns true for uppercase ASCII letters.
export let isUpper (value : Int) : Bit := inRange(value, 65, 90);

--- Returns true for ASCII letters.
export let isAlpha (value : Int) : Bit :=
  if isLower(value) then 0 = 0 else isUpper(value);

--- Returns true for ASCII letters or decimal digits.
export let isAlphanumeric (value : Int) : Bit :=
  if isAlpha(value) then 0 = 0 else isDigit(value);

--- Convert ASCII uppercase byte to lowercase.
export let toLower (value : Int) : Int :=
  if isUpper(value) then value + 32 else value;

--- Convert ASCII lowercase byte to uppercase.
export let toUpper (value : Int) : Int :=
  if isLower(value) then value - 32 else value;

--- Returns true for ASCII hexadecimal digits.
export let isHexDigit (value : Int) : Bit :=
  if isDigit(value) then 0 = 0 else inRange(toLower(value), 97, 102);

--- Decimal digit value.
export let digitValue (value : Int) : Maybe[Int] :=
  if isDigit(value) then maybe.Some[Int](value - 48) else maybe.None[Int]();

--- Hexadecimal digit value.
export let hexValue (value : Int) : Maybe[Int] :=
  if isDigit(value) then maybe.Some[Int](value - 48)
  else if inRange(toLower(value), 97, 102) then maybe.Some[Int](
    toLower(value) - 87
  ) else maybe.None[Int]();
