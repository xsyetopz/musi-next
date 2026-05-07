--! UTF-8 encoding helpers.

let maybe := import "@std/maybe";
let encodingHost := import "musi:encoding";
let Maybe := maybe.Maybe;

--- Encode a value into this format.
export let encode (source : String) : String := encodingHost.utf8Encode(source);

--- Returns true when source is valid for this format.
export let isValid (bytes : String) : Bool :=
  encodingHost.utf8IsValid(bytes) > 0;

--- Decode a value from this format.
export let decode (bytes : String) : Maybe[String] :=
  match isValid(bytes) (
  | valid if valid => maybe.Some[String](encodingHost.utf8Decode(bytes))
  | _ => maybe.None[String]()
  );
