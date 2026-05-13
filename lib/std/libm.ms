--! C math library surface.

let maybe := import "@std/maybe";
let Maybe := maybe.Maybe;

@external(abi := .c)
let cAcos (value : Float) : Float;
@external(abi := .c)
let cAsin (value : Float) : Float;
@external(abi := .c)
let cAtan (value : Float) : Float;
@external(abi := .c)
let cAtan2 (y : Float, x : Float) : Float;
@external(abi := .c)
let cCeil (value : Float) : Float;
@external(abi := .c)
let cCos (value : Float) : Float;
@external(abi := .c)
let cExp (value : Float) : Float;
@external(abi := .c)
let cFabs (value : Float) : Float;
@external(abi := .c)
let cFloor (value : Float) : Float;
@external(abi := .c)
let cFmod (left : Float, right : Float) : Float;
@external(abi := .c)
let cHypot (left : Float, right : Float) : Float;
@external(abi := .c)
let cLog (value : Float) : Float;
@external(abi := .c)
let cLog10 (value : Float) : Float;
@external(abi := .c)
let cPow (base : Float, exponent : Float) : Float;
@external(abi := .c)
let cRound (value : Float) : Float;
@external(abi := .c)
let cSin (value : Float) : Float;
@external(abi := .c)
let cSqrt (value : Float) : Float;
@external(abi := .c)
let cTan (value : Float) : Float;
@external(abi := .c)
let cTrunc (value : Float) : Float;

@external(abi := .musi)
let floatIsNanIntrinsic (value : Float) : Bit;
@external(abi := .musi)
let floatIsInfiniteIntrinsic (value : Float) : Bit;
@external(abi := .musi)
let floatIsFiniteIntrinsic (value : Float) : Bit;

--- Archimedes constant.
export let pi : Float := 3.141592653589793;
--- Euler constant.
export let e : Float := 2.718281828459045;

let both (left : Bit, right : Bit) : Bit := if left then right else 0 = 1;

--- Returns true when value is NaN.
export let isNan (value : Float) : Bit := unsafe {
  floatIsNanIntrinsic(value);
};
--- Returns true when value is positive or negative infinity.
export let isInfinite (value : Float) : Bit := unsafe {
  floatIsInfiniteIntrinsic(value);
};
--- Returns true when value is neither NaN nor infinite.
export let isFinite (value : Float) : Bit := unsafe {
  floatIsFiniteIntrinsic(value);
};

let finiteBoth (value : Float, condition : Bit) : Bit :=
  both(isFinite(value), condition);
let isUnitDomain (value : Float) : Bit :=
  finiteBoth(value, both(value >= - 1.0, value <= 1.0));
let isNonNegative (value : Float) : Bit := finiteBoth(value, value >= 0.0);
let isPositive (value : Float) : Bit := finiteBoth(value, value > 0.0);
let isFiniteNonZero (value : Float) : Bit := finiteBoth(value, value /= 0.0);

--- Arc cosine.
export let acos (value : Float) : Float := unsafe {
  cAcos(value);
};
--- Arc cosine when input is inside [-1, 1].
export let checkedAcos (value : Float) : Maybe[Float] :=
  if isUnitDomain(value) then maybe.Some[Float](acos(value))
  else maybe.None[Float]();
--- Arc sine.
export let asin (value : Float) : Float := unsafe {
  cAsin(value);
};
--- Arc sine when input is inside [-1, 1].
export let checkedAsin (value : Float) : Maybe[Float] :=
  if isUnitDomain(value) then maybe.Some[Float](asin(value))
  else maybe.None[Float]();
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
  if both(isFinite(left), isFiniteNonZero(right)) then maybe.Some[Float](
    fmod(left, right)
  ) else maybe.None[Float]();
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
  if isPositive(value) then maybe.Some[Float](log(value))
  else maybe.None[Float]();
--- Base-10 logarithm.
export let log10 (value : Float) : Float := unsafe {
  cLog10(value);
};
--- Base-10 logarithm when input is positive.
export let checkedLog10 (value : Float) : Maybe[Float] :=
  if isPositive(value) then maybe.Some[Float](log10(value))
  else maybe.None[Float]();
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
  if isNonNegative(value) then maybe.Some[Float](sqrt(value))
  else maybe.None[Float]();
--- Tangent.
export let tan (value : Float) : Float := unsafe {
  cTan(value);
};
--- Integer part, rounded toward zero.
export let trunc (value : Float) : Float := unsafe {
  cTrunc(value);
};
