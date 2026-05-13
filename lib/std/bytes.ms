--! Byte sequence helpers.

--- Byte buffer backed by integer byte values.
export hidden let Bytes := data {
  | ByteValues(values : []Int)
};

--- Empty byte buffer.
export let empty () : Bytes := .ByteValues(values := []);

--- Build bytes from raw byte values.
export let fromArray (values : []Int) : Bytes :=
  .ByteValues(values := [...values]);

--- Function for `toArray`.
let bytesToArray (target : Bytes) : []Int :=
  match target (
  | .ByteValues(values := values) =>[...values]
  );

--- Return a copy of raw byte values.
export let toArray (target : Bytes) : []Int := bytesToArray(target);

--- Function for `copy`.
let copyBytes (target : Bytes) : Bytes := fromArray(bytesToArray(target));

--- Copy bytes.
export let copy (target : Bytes) : Bytes := copyBytes(target);

--- Function for `concat`.
let concatBytes (target : Bytes, other : Bytes) : Bytes :=
  fromArray([...bytesToArray(target), ...bytesToArray(other)]);

--- Concatenate two byte buffers.
export let concat (target : Bytes, other : Bytes) : Bytes :=
  concatBytes(target, other);

--- Function for `append`.
let appendByte (target : Bytes, value : Int) : Bytes :=
  fromArray([...bytesToArray(target), value]);

--- Append one byte.
export let append (target : Bytes, value : Int) : Bytes :=
  appendByte(target, value);

--- Function for `prepend`.
let prependByte (target : Bytes, value : Int) : Bytes :=
  fromArray([value, ...bytesToArray(target)]);

--- Prepend one byte.
export let prepend (target : Bytes, value : Int) : Bytes :=
  prependByte(target, value);

--- Function for `equals`.
let bytesEqual (target : Bytes, other : Bytes) : Bit :=
  bytesToArray(target) = bytesToArray(other);

--- Compare byte buffers.
export let equals (target : Bytes, other : Bytes) : Bit :=
  bytesEqual(target, other);

--- Function for `notEquals`.
let bytesNotEqual (target : Bytes, other : Bytes) : Bit :=
  bytesToArray(target) /= bytesToArray(other);

--- Compare byte buffers for inequality.
export let notEquals (target : Bytes, other : Bytes) : Bit :=
  bytesNotEqual(target, other);

--- Function for `isEmpty`.
let bytesIsEmpty (target : Bytes) : Bit :=
  match bytesToArray(target) (
  | [] => 0 = 0
  | _ => 0 = 1
  );

--- Function for `nonEmpty`.
let bytesNonEmpty (target : Bytes) : Bit :=
  match bytesToArray(target) (
  | [] => 0 = 1
  | _ => 0 = 0
  );

--- Return a copy of raw byte values.
export let (self : Bytes).toArray () : []Int := bytesToArray(self);

--- Copy bytes.
export let (self : Bytes).copy () : Bytes := copyBytes(self);

--- Concatenate two byte buffers.
export let (self : Bytes).concat (other : Bytes) : Bytes :=
  concatBytes(self, other);

--- Append one byte.
export let (self : Bytes).append (value : Int) : Bytes :=
  appendByte(self, value);

--- Prepend one byte.
export let (self : Bytes).prepend (value : Int) : Bytes :=
  prependByte(self, value);

--- Compare byte buffers.
export let (self : Bytes).equals (other : Bytes) : Bit :=
  bytesEqual(self, other);

--- Compare byte buffers for inequality.
export let (self : Bytes).notEquals (other : Bytes) : Bit :=
  bytesNotEqual(self, other);

--- Returns true when byte buffer is empty.
export let (self : Bytes).isEmpty () : Bit := bytesIsEmpty(self);

--- Returns true when byte buffer has values.
export let (self : Bytes).nonEmpty () : Bit := bytesNonEmpty(self);
