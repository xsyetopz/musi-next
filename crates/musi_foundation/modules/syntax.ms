--! Compiler syntax builtin module.

let Core := import "musi:core";
let Any := Core.Any;
let String := Core.String;
let Syntax := Core.Syntax;
let Type := Core.Type;
let Unit := Core.Unit;

@external(abi := .musi)
let Musi__eval (body : Syntax, result : Type) : Any;

--- Evaluate syntax as a value of expected type.
export let eval (body : Syntax, result : Type) : Any := unsafe {
  Musi__eval(body, result);
};

@external(abi := .musi)
let Musi__registerModule (spec : String, body : Syntax) : Unit;

--- Register module syntax under a specifier.
export let registerModule (spec : String, body : Syntax) : Unit := unsafe {
  Musi__registerModule(spec, body);
};
