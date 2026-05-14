--! Host-backed cryptographic primitives.

let Core := import "musi:core";
let String := Core.String;

@external(abi := .musi)
let Musi__sha256Hex (source : String) : String;

export let sha256Hex (source : String) : String := unsafe (Musi__sha256Hex(source));

@external(abi := .musi)
let Musi__sha256Base64 (source : String) : String;

export let sha256Base64 (source : String) : String := unsafe (Musi__sha256Base64(source));
