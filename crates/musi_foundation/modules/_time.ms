--! Host-backed time primitives.

let Core := import "musi:core";
let Int := Core.Int;
let Unit := Core.Unit;

@foreign(abi := .musi)
let Musi__nowUnixMs () : Int;

export let nowUnixMs () : Int := unsafe (Musi__nowUnixMs());

@foreign(abi := .musi)
let Musi__monotonicMs () : Int;

export let monotonicMs () : Int := unsafe (Musi__monotonicMs());

@foreign(abi := .musi)
let Musi__sleepMs (ms : Int) : Unit;

export let sleepMs (ms : Int) : Unit := unsafe (Musi__sleepMs(ms));
