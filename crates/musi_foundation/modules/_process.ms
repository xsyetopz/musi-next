--! Host-backed process primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;
let Unit := Core.Unit;

@external(abi := .musi)
let Musi__argCount () : Int;

export let argCount () : Int := unsafe {
  Musi__argCount();
};

@external(abi := .musi)
let Musi__argAt (index : Int) : String;

export let argAt (index : Int) : String := unsafe {
  Musi__argAt(index);
};

@external(abi := .musi)
let Musi__cwd () : String;

export let cwd () : String := unsafe {
  Musi__cwd();
};

@external(abi := .musi)
let Musi__run (command : String) : Int;

export let run (command : String) : Int := unsafe {
  Musi__run(command);
};

@external(abi := .musi)
let Musi__exit (code : Int) : Unit;

export let exit (code : Int) : Unit := unsafe {
  Musi__exit(code);
};
