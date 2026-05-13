--! Floating-point math helpers.

--- Absolute float value.
export let absFloat(value : Float) : Float :=
  match () (
  | _ where value >= 0.0 => value
  | _ => - value
  );

--- Maximum float value.
export let maxFloat(left : Float, right : Float) : Float :=
  match () (
  | _ where left >= right => left
  | _ => right
  );

--- Minimum float value.
export let minFloat(left : Float, right : Float) : Float :=
  match () (
  | _ where left <= right => left
  | _ => right
  );

--- Clamp a float into inclusive bounds.
export let clampFloat(value : Float, low : Float, high : Float) : Float :=
  match () (
  | _ where value < low => low
  | _ where value > high => high
  | _ => value
  );
