let maybe := import "@std/maybe";
let Maybe := maybe.Maybe;

let both (left : Bool, right : Bool) : Bool :=
  match left (
  | success if success => right
  | _ => 0 = 1
  );

--- Returns true when value is in inclusive ASCII byte range.
export let inRange (value : Int, start : Int, end : Int) : Bool :=
  both(value >= start, value <= end);

--- Returns true for ASCII byte values.
export let isAscii (value : Int) : Bool := inRange(value, 0, 127);

--- Returns true for ASCII control bytes.
export let isControl (value : Int) : Bool :=
  match value (
  | 127 => 0 = 0
  | _ => inRange(value, 0, 31)
  );

--- Returns true for printable ASCII bytes.
export let isPrintable (value : Int) : Bool := inRange(value, 32, 126);

--- Returns true for ASCII whitespace bytes.
export let isWhitespace (value : Int) : Bool :=
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
export let isDigit (value : Int) : Bool := inRange(value, 48, 57);

--- Returns true for lowercase ASCII letters.
export let isLower (value : Int) : Bool := inRange(value, 97, 122);

--- Returns true for uppercase ASCII letters.
export let isUpper (value : Int) : Bool := inRange(value, 65, 90);

--- Returns true for ASCII letters.
export let isAlpha (value : Int) : Bool :=
  match isLower(value) (
  | success if success => 0 = 0
  | _ => isUpper(value)
  );

--- Returns true for ASCII letters or decimal digits.
export let isAlphanumeric (value : Int) : Bool :=
  match isAlpha(value) (
  | success if success => 0 = 0
  | _ => isDigit(value)
  );

--- Convert ASCII uppercase byte to lowercase.
export let toLower (value : Int) : Int :=
  match isUpper(value) (
  | success if success => value + 32
  | _ => value
  );

--- Convert ASCII lowercase byte to uppercase.
export let toUpper (value : Int) : Int :=
  match isLower(value) (
  | success if success => value - 32
  | _ => value
  );

--- Returns true for ASCII hexadecimal digits.
export let isHexDigit (value : Int) : Bool :=
  match isDigit(value) (
  | success if success => 0 = 0
  | _ => inRange(toLower(value), 97, 102)
  );

--- Decimal digit value.
export let digitValue (value : Int) : Maybe[Int] :=
  match isDigit(value) (
  | success if success => maybe.Some[Int](value - 48)
  | _ => maybe.None[Int]()
  );

--- Hexadecimal digit value.
export let hexValue (value : Int) : Maybe[Int] :=
  match isDigit(value) (
  | success if success => maybe.Some[Int](value - 48)
  | _ => match toLower(value) (
  | lower if inRange(lower, 97, 102) => maybe.Some[Int](lower - 87)
  | _ => maybe.None[Int]()
  )
  );
