--! Host-backed cryptographic primitives.

let Core := import "musi:core";
let String := Core.String;

@external(
  abi := .musi
)
let sha256HexIntrinsic(source : String) : String;

export let sha256Hex(source : String) : String := unsafe {
  sha256HexIntrinsic(source);
};

@external(
  abi := .musi
)
let sha256Base64Intrinsic(source : String) : String;

export let sha256Base64(source : String) : String := unsafe {
  sha256Base64Intrinsic(source);
};
