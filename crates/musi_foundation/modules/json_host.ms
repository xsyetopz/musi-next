--! Host-backed JSON primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;

@external(
  abi := .musi
)
let isValidIntrinsic(source : String) : Int;

export let isValid(source : String) : Int := unsafe {
  isValidIntrinsic(source);
};

@external(
  abi := .musi
)
let normalizeIntrinsic(source : String) : String;

export let normalize(source : String) : String := unsafe {
  normalizeIntrinsic(source);
};
