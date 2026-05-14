--! Host-backed process primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;
let Unit := Core.Unit;

@foreign(abi := .musi)
let Musi__argCount () : Int;

export let argCount () : Int := unsafe (Musi__argCount());

@foreign(abi := .musi)
let Musi__argAt (index : Int) : String;

export let argAt (index : Int) : String := unsafe (Musi__argAt(index));

@foreign(abi := .musi)
let Musi__cwd () : String;

export let cwd () : String := unsafe (Musi__cwd());

@foreign(abi := .musi)
let Musi__run (command : String) : Int;

export let run (command : String) : Int := unsafe (Musi__run(command));

@foreign(abi := .musi)
let Musi__exit (code : Int) : Unit;

export let exit (code : Int) : Unit := unsafe (Musi__exit(code));
