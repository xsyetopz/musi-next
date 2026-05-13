--! Host-backed encoding primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;

@external(abi := .musi)
let Musi__base64Encode (source : String) : String;

export let base64Encode (source : String) : String := unsafe {
  Musi__base64Encode(source);
};

@external(abi := .musi)
let Musi__base64Decode (source : String) : String;

export let base64Decode (source : String) : String := unsafe {
  Musi__base64Decode(source);
};

@external(abi := .musi)
let Musi__base64IsValid (source : String) : Int;

export let base64IsValid (source : String) : Int := unsafe {
  Musi__base64IsValid(source);
};

@external(abi := .musi)
let Musi__utf8Encode (source : String) : String;

export let utf8Encode (source : String) : String := unsafe {
  Musi__utf8Encode(source);
};

@external(abi := .musi)
let Musi__utf8Decode (source : String) : String;

export let utf8Decode (source : String) : String := unsafe {
  Musi__utf8Decode(source);
};

@external(abi := .musi)
let Musi__utf8IsValid (source : String) : Int;

export let utf8IsValid (source : String) : Int := unsafe {
  Musi__utf8IsValid(source);
};
