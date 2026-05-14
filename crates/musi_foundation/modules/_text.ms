--! Host-backed text primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;

@foreign(abi := .musi)
let Musi__length (value : String) : Int;

export let length (value : String) : Int := unsafe (Musi__length(value));

@foreign(abi := .musi)
let Musi__concat (left : String, right : String) : String;

export let concat (left : String, right : String) : String := unsafe (Musi__concat(left, right));

@foreign(abi := .musi)
let Musi__slice (value : String, start : Int, end : Int) : String;

export let slice (value : String, start : Int, end : Int) : String := unsafe (Musi__slice(value, start, end));

@foreign(abi := .musi)
let Musi__byteAt (value : String, index : Int) : Int;

export let byteAt (value : String, index : Int) : Int := unsafe (Musi__byteAt(value, index));

@foreign(abi := .musi)
let Musi__fromByte (value : Int) : String;

export let fromByte (value : Int) : String := unsafe (Musi__fromByte(value));
