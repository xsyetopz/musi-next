--! Host-backed environment primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;

@foreign(abi := .musi)
let Musi__get (name : String) : String;

export let get (name : String) : String := unsafe (Musi__get(name));

@foreign(abi := .musi)
let Musi__has (name : String) : Int;

export let has (name : String) : Int := unsafe (Musi__has(name));

@foreign(abi := .musi)
let Musi__set (name : String, value : String) : Int;

export let set (name : String, value : String) : Int := unsafe (Musi__set(name, value));

@foreign(abi := .musi)
let Musi__remove (name : String) : Int;

export let remove (name : String) : Int := unsafe (Musi__remove(name));
