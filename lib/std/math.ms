--! Math helper namespace.

let floatMath := import "./math/float.ms";
let intMath := import "./math/integer.ms";
let libm := import "@std/libm";

--- Archimedes constant.
export let pi := libm.pi;
--- Euler constant.
export let e := libm.e;

--- Absolute integer value.
export let abs := intMath.abs;
--- Integer sign value.
export let signInt := intMath.signInt;
--- Maximum integer value.
export let max := intMath.max;
--- Minimum integer value.
export let min := intMath.min;
--- Clamp an integer into inclusive bounds.
export let clamp := intMath.clamp;

--- Absolute float value.
export let absFloat := floatMath.absFloat;
--- Absolute floating-point value through libc math.
export let fabs := libm.fabs;
--- Returns true when value is NaN.
export let isNan := libm.isNan;
--- Returns true when value is positive or negative infinity.
export let isInfinite := libm.isInfinite;
--- Returns true when value is neither NaN nor infinite.
export let isFinite := libm.isFinite;
--- Maximum float value.
export let maxFloat := floatMath.maxFloat;
--- Minimum float value.
export let minFloat := floatMath.minFloat;
--- Clamp a float into inclusive bounds.
export let clampFloat := floatMath.clampFloat;
--- Arc cosine.
export let acos := libm.acos;
--- Arc cosine when input is inside [-1, 1].
export let checkedAcos := libm.checkedAcos;
--- Arc sine.
export let asin := libm.asin;
--- Arc sine when input is inside [-1, 1].
export let checkedAsin := libm.checkedAsin;
--- Arc tangent.
export let atan := libm.atan;
--- Arc tangent from `y` and `x`.
export let atan2 := libm.atan2;
--- Smallest integer value not less than input.
export let ceil := libm.ceil;
--- Cosine.
export let cos := libm.cos;
--- Natural exponential.
export let exp := libm.exp;
--- Largest integer value not greater than input.
export let floor := libm.floor;
--- Floating-point remainder.
export let fmod := libm.fmod;
--- Floating-point remainder when divisor is non-zero.
export let checkedFmod := libm.checkedFmod;
--- Euclidean hypotenuse.
export let hypot := libm.hypot;
--- Natural logarithm.
export let log := libm.log;
--- Natural logarithm when input is positive.
export let checkedLog := libm.checkedLog;
--- Base-10 logarithm.
export let log10 := libm.log10;
--- Base-10 logarithm when input is positive.
export let checkedLog10 := libm.checkedLog10;
--- Base raised to exponent.
export let pow := libm.pow;
--- Nearest integer, halfway away from zero.
export let round := libm.round;
--- Sine.
export let sin := libm.sin;
--- Square root.
export let sqrt := libm.sqrt;
--- Square root when input is non-negative.
export let checkedSqrt := libm.checkedSqrt;
--- Tangent.
export let tan := libm.tan;
--- Integer part, rounded toward zero.
export let trunc := libm.trunc;
