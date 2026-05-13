--! Ordering and comparison helpers.

--- Ordering expect for comparisons.
export hidden let Ordering := data {
  | Less
  | Equal
  | Greater
};

--- Equality contract.
export let Eq[T] := shape {
  let eq(left : T, right : T) : Bit;
};

--- Less-than ordering value.
export let less() : Ordering := .Less;

--- Equal ordering value.
export let equal() : Ordering := .Equal;

--- Greater-than ordering value.
export let greater() : Ordering := .Greater;

--- Core operation for `isLess`.
let orderingIsLess(target : Ordering) : Bit :=
  match target (
  | .Less => 0 = 0
  | _ => 0 = 1
  );

--- Core operation for `isEqual`.
let orderingIsEqual(target : Ordering) : Bit :=
  match target (
  | .Equal => 0 = 0
  | _ => 0 = 1
  );

--- Core operation for `isGreater`.
let orderingIsGreater(target : Ordering) : Bit :=
  match target (
  | .Greater => 0 = 0
  | _ => 0 = 1
  );

--- Ordering contract.
export let Ord[T] := shape {
  let compare(left : T, right : T) : Ordering;
};

--- Generic equality with built-in equality semantics.
export let eq[T] (left : T, right : T) : Bit := left = right;

--- Generic inequality with built-in equality semantics.
export let notEq[T] (left : T, right : T) : Bit := left /= right;

--- Core operation for `compareInt`.
export let compareInt(left : Int, right : Int) : Ordering :=
  match () (
  | _ where left < right => less()
  | _ where left > right => greater()
  | _ => equal()
  );

@external(
  abi := .musi
)
let compareFloatTotalIntrinsic(left : Float, right : Float) : Int;

--- Core operation for `compareFloat`.
export let compareFloat(left : Float, right : Float) : Ordering :=
  match unsafe {
  compareFloatTotalIntrinsic(left, right);
} (
| value where value < 0 => less()
| value where value > 0 => greater()
| _ => equal()
);

--- Core operation for `compareString`.
export let compareString(left : String, right : String) : Ordering :=
  match () (
  | _ where left < right => less()
  | _ where left > right => greater()
  | _ => equal()
  );

--- Core operation for `minInt`.
export let minInt(left : Int, right : Int) : Int :=
  match compareInt(left, right) (
  | .Greater => right
  | _ => left
  );

--- Core operation for `maxInt`.
export let maxInt(left : Int, right : Int) : Int :=
  match compareInt(left, right) (
  | .Less => right
  | _ => left
  );

--- Core operation for `clampInt`.
export let clampInt(value : Int, lowerBound : Int, upperBound : Int,) : Int :=
  maxInt(lowerBound, minInt(value, upperBound));

--- Integer equality instance.
export let intEq :=
  shape {
  let eq(left : Int, right : Int) : Bit := left = right;
};

--- Float equality instance.
export let floatEq :=
  shape {
  let eq(left : Float, right : Float) : Bit := left = right;
};

--- String equality instance.
export let stringEq :=
  shape {
  let eq(left : String, right : String) : Bit := left = right;
};

--- Bit equality instance.
export let boolEq :=
  shape {
  let eq(left : Bit, right : Bit) : Bit := left = right;
};

--- Integer ordering instance.
export let intOrd :=
  shape {
  let compare(left : Int, right : Int) : Ordering := compareInt(left, right);
};

--- Float ordering instance.
export let floatOrd :=
  shape {
  let compare(left : Float, right : Float) : Ordering :=
    compareFloat(left, right);
};

--- String ordering instance.
export let stringOrd :=
  shape {
  let compare(left : String, right : String) : Ordering :=
    compareString(left, right);
};

--- Returns true when ordering is less.
export let isLess(target : Ordering) : Bit := orderingIsLess(target);

--- Returns true when ordering is less.
export let (self : Ordering).isLess() : Bit := orderingIsLess(self);

--- Returns true when ordering is equal.
export let isEqual(target : Ordering) : Bit := orderingIsEqual(target);

--- Returns true when ordering is equal.
export let (self : Ordering).isEqual() : Bit := orderingIsEqual(self);

--- Returns true when ordering is greater.
export let isGreater(target : Ordering) : Bit := orderingIsGreater(target);

--- Returns true when ordering is greater.
export let (self : Ordering).isGreater() : Bit := orderingIsGreater(self);
