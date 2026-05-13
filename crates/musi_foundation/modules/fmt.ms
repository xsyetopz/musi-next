--! Host-backed formatting primitives.

let Core := import "musi:core";
let Float := Core.Float;
let Int := Core.Int;
let String := Core.String;

@external(
  abi := .musi
)
let intIntrinsic(value : Int) : String;

export let int(value : Int) : String := unsafe {
  intIntrinsic(value);
};

@external(
  abi := .musi
)
let floatIntrinsic(value : Float) : String;

export let float(value : Float) : String := unsafe {
  floatIntrinsic(value);
};
