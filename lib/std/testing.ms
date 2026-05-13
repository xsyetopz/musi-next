--! Testing module.

let intrinsics := import "musi:test";

--- Start a named test suite.
export let describe (name : String) : Unit := intrinsics.suiteStart(name);
--- End current test suite.
export let endDescribe () : Unit := intrinsics.suiteEnd();
--- Record one named test case.
export let it (name : String, passed : Bit) : Unit :=
  intrinsics.testCase(name, passed);
