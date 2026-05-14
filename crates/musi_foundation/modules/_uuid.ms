--! Host-backed UUID primitives.

let Core := import "musi:core";
let String := Core.String;

@external(abi := .musi)
let Musi__v4 () : String;

export let v4 () : String := unsafe (Musi__v4());
