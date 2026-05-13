--! Host-backed file system primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;
let Unit := Core.Unit;

@external(abi := .musi)
let readTextIntrinsic (path : String) : String;

export let readText (path : String) : String := unsafe {
  readTextIntrinsic(path);
};

@external(abi := .musi)
let writeTextIntrinsic (path : String, text : String) : Unit;

export let writeText (path : String, text : String) : Unit := unsafe {
  writeTextIntrinsic(path, text);
};

@external(abi := .musi)
let existsIntrinsic (path : String) : Int;

export let exists (path : String) : Int := unsafe {
  existsIntrinsic(path);
};

@external(abi := .musi)
let appendTextIntrinsic (path : String, text : String) : Int;

export let appendText (path : String, text : String) : Int := unsafe {
  appendTextIntrinsic(path, text);
};

@external(abi := .musi)
let removeIntrinsic (path : String) : Int;

export let remove (path : String) : Int := unsafe {
  removeIntrinsic(path);
};

@external(abi := .musi)
let createDirAllIntrinsic (path : String) : Int;

export let createDirAll (path : String) : Int := unsafe {
  createDirAllIntrinsic(path);
};
