--! Host-backed random primitives.

let Core := import "musi:core";
let Float := Core.Float;
let Int := Core.Int;
let String := Core.String;

@external(abi := .musi)
let intIntrinsic () : Int;

export let int () : Int := unsafe {
  intIntrinsic();
};

@external(abi := .musi)
let intInRangeIntrinsic (lowerBound : Int, upperBound : Int) : Int;

export let intInRange (lowerBound : Int, upperBound : Int) : Int := unsafe {
  intInRangeIntrinsic(lowerBound, upperBound);
};

@external(abi := .musi)
let boolIntrinsic () : Int;

export let bool () : Int := unsafe {
  boolIntrinsic();
};

@external(abi := .musi)
let float01Intrinsic () : Float;

export let float01 () : Float := unsafe {
  float01Intrinsic();
};

@external(abi := .musi)
let entropyHexIntrinsic (count : Int) : String;

export let entropyHex (count : Int) : String := unsafe {
  entropyHexIntrinsic(count);
};
