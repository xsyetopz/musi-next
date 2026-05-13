--! Slice collection helpers.

--- Build an empty slice.
export let empty [T] () : []T := [];
--- Return a shallow copy of slice values.
export let copy [T] (values : []T) : []T := [...values];
--- Concatenate two slices.
export let concat [T] (left : []T, right : []T) : []T := [...left, ...right];
--- Compare integer slices for equality.
export let equalsInt (left : []Int, right : []Int) : Bit := left = right;
--- Returns true when slice has no values.
export let isEmpty [T] (values : []T) : Bit :=
  if values =[] then 0 = 0 else 0 = 1;
--- Returns true when slice has values.
export let nonEmpty [T] (values : []T) : Bit :=
  if values =[] then 0 = 1 else 0 = 0;
