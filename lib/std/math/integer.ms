--! Integer math helpers.

--- Absolute integer value.
export let abs(value : Int) : Int :=
  match () (
  | _ where value >= 0 => value
  | _ => - value
  );

--- Integer sign value.
export let signInt(value : Int) : Int :=
  match () (
  | _ where value < 0 => - 1
  | _ where value > 0 => 1
  | _ => 0
  );

--- Maximum integer value.
export let max(left : Int, right : Int) : Int :=
  match () (
  | _ where left >= right => left
  | _ => right
  );

--- Minimum integer value.
export let min(left : Int, right : Int) : Int :=
  match () (
  | _ where left <= right => left
  | _ => right
  );

--- Clamp an integer into inclusive bounds.
export let clamp(value : Int, low : Int, high : Int) : Int :=
  match () (
  | _ where value < low => low
  | _ where value > high => high
  | _ => value
  );
