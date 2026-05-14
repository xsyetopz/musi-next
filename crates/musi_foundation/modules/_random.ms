--! Host-backed random primitives.

let Core := import "musi:core";
let Float := Core.Float;
let Int := Core.Int;
let String := Core.String;

@external(abi := .musi)
let Musi__int () : Int;

export let int () : Int := unsafe (Musi__int());

@external(abi := .musi)
let Musi__intInRange (lowerBound : Int, upperBound : Int) : Int;

export let intInRange (lowerBound : Int, upperBound : Int) : Int := unsafe (Musi__intInRange(lowerBound, upperBound));

@external(abi := .musi)
let Musi__bool () : Int;

export let bool () : Int := unsafe (Musi__bool());

@external(abi := .musi)
let Musi__float01 () : Float;

export let float01 () : Float := unsafe (Musi__float01());

@external(abi := .musi)
let Musi__entropyHex (count : Int) : String;

export let entropyHex (count : Int) : String := unsafe (Musi__entropyHex(count));
