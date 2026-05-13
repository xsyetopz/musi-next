--! Host-backed input and output primitives.

let Core := import "musi:core";
let String := Core.String;
let Unit := Core.Unit;

@external(abi := .musi)
let writeIntrinsic (text : String) : Unit;

export let write (text : String) : Unit := unsafe {
  writeIntrinsic(text);
};

@external(abi := .musi)
let writeLnIntrinsic (text : String) : Unit;

export let writeLn (text : String) : Unit := unsafe {
  writeLnIntrinsic(text);
};

@external(abi := .musi)
let writeErrIntrinsic (text : String) : Unit;

export let writeErr (text : String) : Unit := unsafe {
  writeErrIntrinsic(text);
};

@external(abi := .musi)
let writeErrLnIntrinsic (text : String) : Unit;

export let writeErrLn (text : String) : Unit := unsafe {
  writeErrLnIntrinsic(text);
};

@external(abi := .musi)
let readLineIntrinsic () : String;

export let readLine () : String := unsafe {
  readLineIntrinsic();
};
