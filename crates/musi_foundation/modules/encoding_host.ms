--! Host-backed encoding primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;

@external(abi := .musi)
let base64EncodeIntrinsic (source : String) : String;

export let base64Encode (source : String) : String := unsafe {
  base64EncodeIntrinsic(source);
};

@external(abi := .musi)
let base64DecodeIntrinsic (source : String) : String;

export let base64Decode (source : String) : String := unsafe {
  base64DecodeIntrinsic(source);
};

@external(abi := .musi)
let base64IsValidIntrinsic (source : String) : Int;

export let base64IsValid (source : String) : Int := unsafe {
  base64IsValidIntrinsic(source);
};

@external(abi := .musi)
let utf8EncodeIntrinsic (source : String) : String;

export let utf8Encode (source : String) : String := unsafe {
  utf8EncodeIntrinsic(source);
};

@external(abi := .musi)
let utf8DecodeIntrinsic (source : String) : String;

export let utf8Decode (source : String) : String := unsafe {
  utf8DecodeIntrinsic(source);
};

@external(abi := .musi)
let utf8IsValidIntrinsic (source : String) : Int;

export let utf8IsValid (source : String) : Int := unsafe {
  utf8IsValidIntrinsic(source);
};
