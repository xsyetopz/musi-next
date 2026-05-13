--! Compiler syntax builtin surface.

let Core := import "musi:core";
let Any := Core.Any;
let String := Core.String;
let Syntax := Core.Syntax;
let Type := Core.Type;
let Unit := Core.Unit;

@external(abi := .musi)
let evalIntrinsic (body : Syntax, result : Type) : Any;

export let eval (body : Syntax, result : Type) : Any := unsafe {
  evalIntrinsic(body, result);
};

@external(abi := .musi)
let registerModuleIntrinsic (spec : String, body : Syntax) : Unit;

export let registerModule (spec : String, body : Syntax) : Unit := unsafe {
  registerModuleIntrinsic(spec, body);
};
