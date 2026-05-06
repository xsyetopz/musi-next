let ascii := import "@std/ascii";
let cmp := import "@std/cmp";
let ffi := import "@std/ffi";
let Ordering := cmp.Ordering;

--- C char ABI type.
export let CChar := ffi.CChar;
--- C signed char ABI type.
export let CSChar := ffi.CSChar;
--- C unsigned char ABI type.
export let CUChar := ffi.CUChar;
--- C signed short ABI type.
export let CShort := ffi.CShort;
--- C unsigned short ABI type.
export let CUShort := ffi.CUShort;
--- C signed int ABI type.
export let CInt := ffi.CInt;
--- C unsigned int ABI type.
export let CUInt := ffi.CUInt;
--- C signed long ABI type.
export let CLong := ffi.CLong;
--- C unsigned long ABI type.
export let CULong := ffi.CULong;
--- C signed long long ABI type.
export let CLongLong := ffi.CLongLong;
--- C unsigned long long ABI type.
export let CULongLong := ffi.CULongLong;
--- C size_t ABI type.
export let CSize := ffi.CSize;
--- C ptrdiff_t ABI type.
export let CSizeDiff := ffi.CSizeDiff;
--- C float ABI type.
export let CFloat := ffi.CFloat;
--- C double ABI type.
export let CDouble := ffi.CDouble;
--- Null-terminated C string ABI value.
export let CString := ffi.CString;
--- Raw C pointer ABI value.
export let CPtr := ffi.CPtr;
--- C char ABI typedef.
export let char := ffi.char;
--- C bool ABI typedef.
export let bool := ffi.bool;
--- C int8_t ABI typedef.
export let int8_t := ffi.int8_t;
--- C int16_t ABI typedef.
export let int16_t := ffi.int16_t;
--- C int32_t ABI typedef.
export let int32_t := ffi.int32_t;
--- C int64_t ABI typedef.
export let int64_t := ffi.int64_t;
--- C uint8_t ABI typedef.
export let uint8_t := ffi.uint8_t;
--- C uint16_t ABI typedef.
export let uint16_t := ffi.uint16_t;
--- C uint32_t ABI typedef.
export let uint32_t := ffi.uint32_t;
--- C uint64_t ABI typedef.
export let uint64_t := ffi.uint64_t;
--- C intptr_t ABI typedef.
export let intptr_t := ffi.intptr_t;
--- C uintptr_t ABI typedef.
export let uintptr_t := ffi.uintptr_t;
--- C size_t ABI typedef.
export let size_t := ffi.size_t;
--- C ptrdiff_t ABI typedef.
export let ptrdiff_t := ffi.ptrdiff_t;
--- Typed C pointer view.
export let Ptr := ffi.Ptr;
--- Pointer helper namespace.
export let ptr := ffi.ptr;

@link(name := "c", symbol := "strcmp")
native "c" let cStrcmp (left : CString, right : CString) : Int32;
@link(name := "c", symbol := "strerror")
native "c" let cStrerror (code : CInt) : CString;
@link(name := "c", symbol := "strlen")
native "c" let cStrlen (value : CString) : Nat;

--- C `strcmp`.
export let strcmp (left : CString, right : CString) : Int32 := unsafe {
  cStrcmp(left, right);
};
--- C `strerror`.
export let strerror (code : CInt) : CString := unsafe {
  cStrerror(code);
};
--- C `strlen`.
export let strlen (value : CString) : Nat := unsafe {
  cStrlen(value);
};

--- C string byte length.
export let length (value : CString) : Nat := strlen(value);

--- C string comparison as ordering.
export let compare (left : CString, right : CString) : Ordering :=
  match strcmp(left, right) (
  | value if value < 0 => cmp.less()
  | value if value > 0 => cmp.greater()
  | _ => cmp.equal()
  );

--- Returns true when C strings compare equal.
export let equals (left : CString, right : CString) : Bool :=
  cmp.isEqual(compare(left, right));

--- C runtime error message text for an error code.
export let errorMessage (code : CInt) : CString := strerror(code);

--- Safe ASCII replacement for locale-sensitive C `isalnum`.
export let isAlnum (value : Int) : Bool := ascii.isAlphanumeric(value);
--- Safe ASCII replacement for locale-sensitive C `isalpha`.
export let isAlpha (value : Int) : Bool := ascii.isAlpha(value);
--- Safe ASCII replacement for locale-sensitive C `isdigit`.
export let isDigit (value : Int) : Bool := ascii.isDigit(value);
--- Safe ASCII replacement for locale-sensitive C `isxdigit`.
export let isXDigit (value : Int) : Bool := ascii.isHexDigit(value);
--- Safe ASCII replacement for locale-sensitive C `isspace`.
export let isSpace (value : Int) : Bool := ascii.isWhitespace(value);
--- Safe ASCII replacement for locale-sensitive C `isprint`.
export let isPrint (value : Int) : Bool := ascii.isPrintable(value);
--- Safe ASCII replacement for locale-sensitive C `iscntrl`.
export let isCntrl (value : Int) : Bool := ascii.isControl(value);
--- Safe ASCII replacement for locale-sensitive C `tolower`.
export let toLower (value : Int) : Int := ascii.toLower(value);
--- Safe ASCII replacement for locale-sensitive C `toupper`.
export let toUpper (value : Int) : Int := ascii.toUpper(value);
