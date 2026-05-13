--! Iterator helpers for collections.

--- Build an empty iterator value.
export let empty [T] () : []T := [];
--- Build an iterator with one value.
export let once [T] (value : T) : []T := [value];
--- Concatenate two iterator values.
export let concat [T] (left : []T, right : []T) : []T := [...left, ...right];
--- Append one value to an iterator.
export let append [T] (values : []T, value : T) : []T := [...values, value];
--- Prepend one value to an iterator.
export let prepend [T] (values : []T, value : T) : []T := [value, ...values];
--- Collect iterator values into an array copy.
export let collect [T] (values : []T) : []T := [...values];
--- Compare integer iterator values for equality.
export let equalsInt (left : []Int, right : []Int) : Bit := left = right;
--- Returns true when iterator has no values.
export let isEmpty [T] (values : []T) : Bit :=
  if values =[] then 0 = 0 else 0 = 1;
--- Returns true when iterator has values.
export let nonEmpty [T] (values : []T) : Bit :=
  if values =[] then 0 = 1 else 0 = 0;
