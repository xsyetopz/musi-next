--! Host-backed UUID primitives.

let Core := import "musi:core";
let String := Core.String;

@external(abi := .musi)
let v4Intrinsic () : String;

export let v4 () : String := unsafe {
  v4Intrinsic();
};
