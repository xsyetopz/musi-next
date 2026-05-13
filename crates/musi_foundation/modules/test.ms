--! Host-backed test primitives.

let Core := import "musi:core";
let Bit := Core.Bit;
let String := Core.String;
let Unit := Core.Unit;

--- Sample test case marker.
export hidden let SampleCase [T] := data {
  | Case
};

--- Sample test list marker.
export hidden let SampleList [T] := data {
  | Nil
};

--- Sample test contract.
export hidden let Sample [T] := shape {
};

@external(abi := .musi)
let Musi__suiteStart (name : String) : Unit;

--- Start a named test suite.
export let suiteStart (name : String) : Unit := unsafe {
  Musi__suiteStart(name);
};

@external(abi := .musi)
let Musi__suiteEnd () : Unit;

--- End current test suite.
export let suiteEnd () : Unit := unsafe {
  Musi__suiteEnd();
};

@external(abi := .musi)
let Musi__testCase (name : String, passed : Bit) : Unit;

--- Record one named test case.
export let testCase (name : String, passed : Bit) : Unit := unsafe {
  Musi__testCase(name, passed);
};
