--! Machine word helpers.

let sys := import "#sys";
let bits := import "@std/bits";
let core := import "musi:core";

let Word8 := core.Word8;
let Word16 := core.Word16;
let Word32 := core.Word32;
let Word64 := core.Word64;

--- Target word width in bits.
export let width : Int := sys.pointerWidth;

--- Build zero-valued 8-bit machine word.
export let zero8 () : Word8 := bits.zero8();
--- Build zero-valued 16-bit machine word.
export let zero16 () : Word16 := bits.zero16();
--- Build zero-valued 32-bit machine word.
export let zero32 () : Word32 := bits.zero32();
--- Build zero-valued 64-bit machine word.
export let zero64 () : Word64 := bits.zero64();

--- Build all-ones 8-bit machine word.
export let ones8 () : Word8 := bits.ones8();
--- Build all-ones 16-bit machine word.
export let ones16 () : Word16 := bits.ones16();
--- Build all-ones 32-bit machine word.
export let ones32 () : Word32 := bits.ones32();
--- Build all-ones 64-bit machine word.
export let ones64 () : Word64 := bits.ones64();

--- Convert natural number to 8-bit machine word.
export let fromNat8 (value : Nat) : Word8 := bits.fromNat8(value);
--- Convert natural number to 16-bit machine word.
export let fromNat16 (value : Nat) : Word16 := bits.fromNat16(value);
--- Convert natural number to 32-bit machine word.
export let fromNat32 (value : Nat) : Word32 := bits.fromNat32(value);
--- Convert natural number to 64-bit machine word.
export let fromNat64 (value : Nat) : Word64 := bits.fromNat64(value);

--- Convert integer to 8-bit machine word.
export let fromInt8 (value : Int) : Word8 := bits.fromInt8(value);
--- Convert integer to 16-bit machine word.
export let fromInt16 (value : Int) : Word16 := bits.fromInt16(value);
--- Convert integer to 32-bit machine word.
export let fromInt32 (value : Int) : Word32 := bits.fromInt32(value);
--- Convert integer to 64-bit machine word.
export let fromInt64 (value : Int) : Word64 := bits.fromInt64(value);

--- Convert 8-bit machine word to natural number.
export let toNat8 (value : Word8) : Nat := bits.toNat8(value);
--- Convert 16-bit machine word to natural number.
export let toNat16 (value : Word16) : Nat := bits.toNat16(value);
--- Convert 32-bit machine word to natural number.
export let toNat32 (value : Word32) : Nat := bits.toNat32(value);
--- Convert 64-bit machine word to natural number.
export let toNat64 (value : Word64) : Nat := bits.toNat64(value);

--- Convert 8-bit machine word to integer.
export let toInt8 (value : Word8) : Int := bits.toInt8(value);
--- Convert 16-bit machine word to integer.
export let toInt16 (value : Word16) : Int := bits.toInt16(value);
--- Convert 32-bit machine word to integer.
export let toInt32 (value : Word32) : Int := bits.toInt32(value);
--- Convert 64-bit machine word to integer.
export let toInt64 (value : Word64) : Int := bits.toInt64(value);

--- Returns true when two 32-bit machine words match.
export let equals32 (left : Word32, right : Word32) : Bit :=
  bits.equals32(left, right);

--- Combine 32-bit machine words with bitwise and.
export let logicalAnd32 (left : Word32, right : Word32) : Word32 :=
  bits.logicalAnd32(left, right);

--- Combine 32-bit machine words with bitwise or.
export let logicalOr32 (left : Word32, right : Word32) : Word32 :=
  bits.logicalOr32(left, right);

--- Combine 32-bit machine words with bitwise xor.
export let logicalXor32 (left : Word32, right : Word32) : Word32 :=
  bits.logicalXor32(left, right);

--- Invert 32-bit machine word bits.
export let logicalNot32 (value : Word32) : Word32 := bits.logicalNot32(value);
