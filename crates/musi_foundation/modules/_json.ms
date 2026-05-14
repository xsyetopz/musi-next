--! Host-backed JSON primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;

@external(abi := .musi)
let Musi__isValid (source : String) : Int;

export let isValid (source : String) : Int := unsafe (Musi__isValid(source));

@external(abi := .musi)
let Musi__normalize (source : String) : String;

export let normalize (source : String) : String := unsafe (Musi__normalize(source));
