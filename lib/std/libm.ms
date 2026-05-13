--! C math library module.

let maybe := import "@std/maybe";
let Maybe := maybe.Maybe;

@external(abi := .c, symbol := "acos")
let C__acos (value : Float) : Float;
@external(abi := .c, symbol := "asin")
let C__asin (value : Float) : Float;
@external(abi := .c, symbol := "atan")
let C__atan (value : Float) : Float;
@external(abi := .c, symbol := "atan2")
let C__atan2 (y : Float, x : Float) : Float;
@external(abi := .c, symbol := "ceil")
let C__ceil (value : Float) : Float;
@external(abi := .c, symbol := "cos")
let C__cos (value : Float) : Float;
@external(abi := .c, symbol := "exp")
let C__exp (value : Float) : Float;
@external(abi := .c, symbol := "fabs")
let C__fabs (value : Float) : Float;
@external(abi := .c, symbol := "floor")
let C__floor (value : Float) : Float;
@external(abi := .c, symbol := "fmod")
let C__fmod (left : Float, right : Float) : Float;
@external(abi := .c, symbol := "hypot")
let C__hypot (left : Float, right : Float) : Float;
@external(abi := .c, symbol := "log")
let C__log (value : Float) : Float;
@external(abi := .c, symbol := "log10")
let C__log10 (value : Float) : Float;
@external(abi := .c, symbol := "pow")
let C__pow (base : Float, exponent : Float) : Float;
@external(abi := .c, symbol := "round")
let C__round (value : Float) : Float;
@external(abi := .c, symbol := "sin")
let C__sin (value : Float) : Float;
@external(abi := .c, symbol := "sqrt")
let C__sqrt (value : Float) : Float;
@external(abi := .c, symbol := "tan")
let C__tan (value : Float) : Float;
@external(abi := .c, symbol := "trunc")
let C__trunc (value : Float) : Float;

@external(abi := .musi)
let Musi__floatIsNan (value : Float) : Bit;
@external(abi := .musi)
let Musi__floatIsInfinite (value : Float) : Bit;
@external(abi := .musi)
let Musi__floatIsFinite (value : Float) : Bit;

--- Archimedes constant.
export let pi : Float := 3.141592653589793;
--- Euler constant.
export let e : Float := 2.718281828459045;

let both (left : Bit, right : Bit) : Bit := if left then right else 0 = 1;

--- Returns true when value is NaN.
export let isNan (value : Float) : Bit := unsafe {
  Musi__floatIsNan(value);
};
--- Returns true when value is positive or negative infinity.
export let isInfinite (value : Float) : Bit := unsafe {
  Musi__floatIsInfinite(value);
};
--- Returns true when value is neither NaN nor infinite.
export let isFinite (value : Float) : Bit := unsafe {
  Musi__floatIsFinite(value);
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
  C__acos(value);
};
--- Arc cosine when input is inside [-1, 1].
export let checkedAcos (value : Float) : Maybe[Float] :=
  if isUnitDomain(value) then maybe.Some[Float](acos(value))
  else maybe.None[Float]();
--- Arc sine.
export let asin (value : Float) : Float := unsafe {
  C__asin(value);
};
--- Arc sine when input is inside [-1, 1].
export let checkedAsin (value : Float) : Maybe[Float] :=
  if isUnitDomain(value) then maybe.Some[Float](asin(value))
  else maybe.None[Float]();
--- Arc tangent.
export let atan (value : Float) : Float := unsafe {
  C__atan(value);
};
--- Arc tangent from `y` and `x`.
export let atan2 (y : Float, x : Float) : Float := unsafe {
  C__atan2(y, x);
};
--- Smallest integer value not less than input.
export let ceil (value : Float) : Float := unsafe {
  C__ceil(value);
};
--- Cosine.
export let cos (value : Float) : Float := unsafe {
  C__cos(value);
};
--- Natural exponential.
export let exp (value : Float) : Float := unsafe {
  C__exp(value);
};
--- Absolute floating-point value.
export let fabs (value : Float) : Float := unsafe {
  C__fabs(value);
};
--- Largest integer value not greater than input.
export let floor (value : Float) : Float := unsafe {
  C__floor(value);
};
--- Floating-point remainder.
export let fmod (left : Float, right : Float) : Float := unsafe {
  C__fmod(left, right);
};
--- Floating-point remainder when divisor is non-zero.
export let checkedFmod (left : Float, right : Float) : Maybe[Float] :=
  if both(isFinite(left), isFiniteNonZero(right)) then maybe.Some[Float](
    fmod(left, right)
  ) else maybe.None[Float]();
--- Euclidean hypotenuse.
export let hypot (left : Float, right : Float) : Float := unsafe {
  C__hypot(left, right);
};
--- Natural logarithm.
export let log (value : Float) : Float := unsafe {
  C__log(value);
};
--- Natural logarithm when input is positive.
export let checkedLog (value : Float) : Maybe[Float] :=
  if isPositive(value) then maybe.Some[Float](log(value))
  else maybe.None[Float]();
--- Base-10 logarithm.
export let log10 (value : Float) : Float := unsafe {
  C__log10(value);
};
--- Base-10 logarithm when input is positive.
export let checkedLog10 (value : Float) : Maybe[Float] :=
  if isPositive(value) then maybe.Some[Float](log10(value))
  else maybe.None[Float]();
--- Base raised to exponent.
export let pow (base : Float, exponent : Float) : Float := unsafe {
  C__pow(base, exponent);
};
--- Nearest integer, halfway away from zero.
export let round (value : Float) : Float := unsafe {
  C__round(value);
};
--- Sine.
export let sin (value : Float) : Float := unsafe {
  C__sin(value);
};
--- Square root.
export let sqrt (value : Float) : Float := unsafe {
  C__sqrt(value);
};
--- Square root when input is non-negative.
export let checkedSqrt (value : Float) : Maybe[Float] :=
  if isNonNegative(value) then maybe.Some[Float](sqrt(value))
  else maybe.None[Float]();
--- Tangent.
export let tan (value : Float) : Float := unsafe {
  C__tan(value);
};
--- Integer part, rounded toward zero.
export let trunc (value : Float) : Float := unsafe {
  C__trunc(value);
};
