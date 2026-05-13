--! Boolean aliases over primitive Bit values.

--- Boolean data surface with Bit-valued variants.
export hidden let Bool := data {
  | True := 1
  | False := 0
};

--- Lowercase true alias.
export let true : Bool := .True;

--- Lowercase false alias.
export let false : Bool := .False;
