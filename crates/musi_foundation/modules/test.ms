--! Host-backed test primitives.

let Core := import "musi:core";
let Bit := Core.Bit;
let String := Core.String;
let Unit := Core.Unit;

export hidden let SampleCase [T] := data {
  | Case
};

export hidden let SampleList [T] := data {
  | Nil
};

export hidden let Sample [T] := shape {
};

@external(abi := .musi)
let suiteStartIntrinsic (name : String) : Unit;

export let suiteStart (name : String) : Unit := unsafe {
  suiteStartIntrinsic(name);
};

@external(abi := .musi)
let suiteEndIntrinsic () : Unit;

export let suiteEnd () : Unit := unsafe {
  suiteEndIntrinsic();
};

@external(abi := .musi)
let testCaseIntrinsic (name : String, passed : Bit) : Unit;

export let testCase (name : String, passed : Bit) : Unit := unsafe {
  testCaseIntrinsic(name, passed);
};
