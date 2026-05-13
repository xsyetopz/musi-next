--! Host-backed logging primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;
let Unit := Core.Unit;

@external(abi := .musi)
let infoIntrinsic (message : String) : Unit;

export let info (message : String) : Unit := unsafe {
  infoIntrinsic(message);
};

@external(abi := .musi)
let writeIntrinsic (level : Int, message : String) : Unit;

export let write (level : Int, message : String) : Unit := unsafe {
  writeIntrinsic(level, message);
};
