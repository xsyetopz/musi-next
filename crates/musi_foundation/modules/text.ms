--! Host-backed text primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;

@external(
  abi := .musi
)
let lengthIntrinsic(value : String) : Int;

export let length(value : String) : Int := unsafe {
  lengthIntrinsic(value);
};

@external(
  abi := .musi
)
let concatIntrinsic(left : String, right : String) : String;

export let concat(left : String, right : String) : String := unsafe {
  concatIntrinsic(left, right);
};

@external(
  abi := .musi
)
let sliceIntrinsic(value : String, start : Int, end : Int) : String;

export let slice(value : String, start : Int, end : Int) : String := unsafe {
  sliceIntrinsic(value, start, end);
};

@external(
  abi := .musi
)
let byteAtIntrinsic(value : String, index : Int) : Int;

export let byteAt(value : String, index : Int) : Int := unsafe {
  byteAtIntrinsic(value, index);
};

@external(
  abi := .musi
)
let fromByteIntrinsic(value : Int) : String;

export let fromByte(value : Int) : String := unsafe {
  fromByteIntrinsic(value);
};
