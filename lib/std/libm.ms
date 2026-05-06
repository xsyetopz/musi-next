--! C math library surface.

let maybe := import "@std/maybe";
let Maybe := maybe.Maybe;

@link(name := "m", symbol := "acos")
native "c" let cAcos (value : Float) : Float;
@link(name := "m", symbol := "asin")
native "c" let cAsin (value : Float) : Float;
@link(name := "m", symbol := "atan")
native "c" let cAtan (value : Float) : Float;
@link(name := "m", symbol := "atan2")
native "c" let cAtan2 (y : Float, x : Float) : Float;
@link(name := "m", symbol := "ceil")
native "c" let cCeil (value : Float) : Float;
@link(name := "m", symbol := "cos")
native "c" let cCos (value : Float) : Float;
@link(name := "m", symbol := "exp")
native "c" let cExp (value : Float) : Float;
@link(name := "m", symbol := "fabs")
native "c" let cFabs (value : Float) : Float;
@link(name := "m", symbol := "floor")
native "c" let cFloor (value : Float) : Float;
@link(name := "m", symbol := "fmod")
native "c" let cFmod (left : Float, right : Float) : Float;
@link(name := "m", symbol := "hypot")
native "c" let cHypot (left : Float, right : Float) : Float;
@link(name := "m", symbol := "log")
native "c" let cLog (value : Float) : Float;
@link(name := "m", symbol := "log10")
native "c" let cLog10 (value : Float) : Float;
@link(name := "m", symbol := "pow")
native "c" let cPow (base : Float, exponent : Float) : Float;
@link(name := "m", symbol := "round")
native "c" let cRound (value : Float) : Float;
@link(name := "m", symbol := "sin")
native "c" let cSin (value : Float) : Float;
@link(name := "m", symbol := "sqrt")
native "c" let cSqrt (value : Float) : Float;
@link(name := "m", symbol := "tan")
native "c" let cTan (value : Float) : Float;
@link(name := "m", symbol := "trunc")
native "c" let cTrunc (value : Float) : Float;

native "musi" let floatIsNanIntrinsic (value : Float) : Bool;
native "musi" let floatIsInfiniteIntrinsic (value : Float) : Bool;
native "musi" let floatIsFiniteIntrinsic (value : Float) : Bool;

--- Archimedes constant.
export let pi : Float := 3.141592653589793;
--- Euler constant.
export let e : Float := 2.718281828459045;

let both (left : Bool, right : Bool) : Bool :=
  match left (
  | success if success => right
  | _ => 0 = 1
  );

--- Returns true when value is NaN.
export let isNan (value : Float) : Bool := unsafe {
  floatIsNanIntrinsic(value);
};
--- Returns true when value is positive or negative infinity.
export let isInfinite (value : Float) : Bool := unsafe {
  floatIsInfiniteIntrinsic(value);
};
--- Returns true when value is neither NaN nor infinite.
export let isFinite (value : Float) : Bool := unsafe {
  floatIsFiniteIntrinsic(value);
};

let finiteBoth (value : Float, condition : Bool) : Bool :=
  both(isFinite(value), condition);
let isUnitDomain (value : Float) : Bool :=
  finiteBoth(value, both(value >= - 1.0, value <= 1.0));
let isNonNegative (value : Float) : Bool := finiteBoth(value, value >= 0.0);
let isPositive (value : Float) : Bool := finiteBoth(value, value > 0.0);
let isFiniteNonZero (value : Float) : Bool := finiteBoth(value, value /= 0.0);

--- Arc cosine.
export let acos (value : Float) : Float := unsafe {
  cAcos(value);
};
--- Arc cosine when input is inside [-1, 1].
export let checkedAcos (value : Float) : Maybe[Float] :=
  match isUnitDomain(value) (
  | success if success => maybe.Some[Float](acos(value))
  | _ => maybe.None[Float]()
  );
--- Arc sine.
export let asin (value : Float) : Float := unsafe {
  cAsin(value);
};
--- Arc sine when input is inside [-1, 1].
export let checkedAsin (value : Float) : Maybe[Float] :=
  match isUnitDomain(value) (
  | success if success => maybe.Some[Float](asin(value))
  | _ => maybe.None[Float]()
  );
--- Arc tangent.
export let atan (value : Float) : Float := unsafe {
  cAtan(value);
};
--- Arc tangent from `y` and `x`.
export let atan2 (y : Float, x : Float) : Float := unsafe {
  cAtan2(y, x);
};
--- Smallest integer value not less than input.
export let ceil (value : Float) : Float := unsafe {
  cCeil(value);
};
--- Cosine.
export let cos (value : Float) : Float := unsafe {
  cCos(value);
};
--- Natural exponential.
export let exp (value : Float) : Float := unsafe {
  cExp(value);
};
--- Absolute floating-point value.
export let fabs (value : Float) : Float := unsafe {
  cFabs(value);
};
--- Largest integer value not greater than input.
export let floor (value : Float) : Float := unsafe {
  cFloor(value);
};
--- Floating-point remainder.
export let fmod (left : Float, right : Float) : Float := unsafe {
  cFmod(left, right);
};
--- Floating-point remainder when divisor is non-zero.
export let checkedFmod (left : Float, right : Float) : Maybe[Float] :=
  match both(isFinite(left), isFiniteNonZero(right)) (
  | success if success => maybe.Some[Float](fmod(left, right))
  | _ => maybe.None[Float]()
  );
--- Euclidean hypotenuse.
export let hypot (left : Float, right : Float) : Float := unsafe {
  cHypot(left, right);
};
--- Natural logarithm.
export let log (value : Float) : Float := unsafe {
  cLog(value);
};
--- Natural logarithm when input is positive.
export let checkedLog (value : Float) : Maybe[Float] :=
  match isPositive(value) (
  | success if success => maybe.Some[Float](log(value))
  | _ => maybe.None[Float]()
  );
--- Base-10 logarithm.
export let log10 (value : Float) : Float := unsafe {
  cLog10(value);
};
--- Base-10 logarithm when input is positive.
export let checkedLog10 (value : Float) : Maybe[Float] :=
  match isPositive(value) (
  | success if success => maybe.Some[Float](log10(value))
  | _ => maybe.None[Float]()
  );
--- Base raised to exponent.
export let pow (base : Float, exponent : Float) : Float := unsafe {
  cPow(base, exponent);
};
--- Nearest integer, halfway away from zero.
export let round (value : Float) : Float := unsafe {
  cRound(value);
};
--- Sine.
export let sin (value : Float) : Float := unsafe {
  cSin(value);
};
--- Square root.
export let sqrt (value : Float) : Float := unsafe {
  cSqrt(value);
};
--- Square root when input is non-negative.
export let checkedSqrt (value : Float) : Maybe[Float] :=
  match isNonNegative(value) (
  | success if success => maybe.Some[Float](sqrt(value))
  | _ => maybe.None[Float]()
  );
--- Tangent.
export let tan (value : Float) : Float := unsafe {
  cTan(value);
};
--- Integer part, rounded toward zero.
export let trunc (value : Float) : Float := unsafe {
  cTrunc(value);
};
