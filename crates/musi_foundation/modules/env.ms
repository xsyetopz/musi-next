--! Host-backed environment primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;

@external(abi := .musi)
let getIntrinsic (name : String) : String;

export let get (name : String) : String := unsafe {
  getIntrinsic(name);
};

@external(abi := .musi)
let hasIntrinsic (name : String) : Int;

export let has (name : String) : Int := unsafe {
  hasIntrinsic(name);
};

@external(abi := .musi)
let setIntrinsic (name : String, value : String) : Int;

export let set (name : String, value : String) : Int := unsafe {
  setIntrinsic(name, value);
};

@external(abi := .musi)
let removeIntrinsic (name : String) : Int;

export let remove (name : String) : Int := unsafe {
  removeIntrinsic(name);
};
