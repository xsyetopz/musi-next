--! Assertion helpers for tests and runtime checks.

let pass := 0 = 0;
let failBit (message : String) : Bit := 0 = 1;

--- Successful assertion value.
export let passValue : Bit := pass;

--- Failed assertion value with message output.
export let fail (message : String) : Bit := failBit(message);

--- Assert two values are equal.
export let equal [T] (actual : T, expected : T) : Bit :=
  if actual = expected then pass else fail("expected values to be equal");

--- Assert two values are different.
export let notEqual [T] (actual : T, expected : T) : Bit :=
  if actual /= expected then pass else fail("expected values to be different");

--- Assert a boolean is true.
export let isTrue (actual : Bit) : Bit :=
  if actual then pass else fail("expected condition to be true");

--- Assert a boolean is false.
export let isFalse (actual : Bit) : Bit :=
  if actual then fail("expected condition to be false")else pass;

--- Assert an integer is greater than a lower bound.
export let greaterThan (actual : Int, lowerBound : Int) : Bit :=
  if actual > lowerBound then pass else fail(
    "expected value to be greater than lower bound"
  );

--- Assert an integer is less than an upper bound.
export let lessThan (actual : Int, upperBound : Int) : Bit :=
  if actual < upperBound then pass else fail(
    "expected value to be less than upper bound"
  );

--- Assert an integer is inside inclusive bounds.
export let betweenInclusive (
  actual : Int,
  lowerBound : Int,
  upperBound : Int
) : Bit := if actual < lowerBound then fail(
    "expected value to be inside inclusive bounds"
  )else if actual > upperBound then fail(
    "expected value to be inside inclusive bounds"
  )else pass;

--- Build integer matcher helpers for one actual value.
export let expectInt (actual : Int) :=
  (
    let shouldEqual (expected : Int) := equal[Int](actual, expected);
    let shouldNotEqual (expected : Int) := notEqual[Int](actual, expected);
    let shouldBeGreaterThan (bound : Int) := greaterThan(actual, bound);
    let shouldBeLessThan (bound : Int) := lessThan(actual, bound);
    {
      equal := shouldEqual, notEqual := shouldNotEqual, greaterThan :=
        shouldBeGreaterThan, lessThan := shouldBeLessThan
    }
  );

--- Matcher alias for integer equality.
export let toBe (actual : Int, expected : Int) : Bit :=
  equal[Int](actual, expected);

--- Matcher alias for integer equality.
export let toEqual (actual : Int, expected : Int) : Bit :=
  equal[Int](actual, expected);

--- Matcher alias for true boolean checks.
export let toBeTrue (actual : Bit) : Bit := isTrue(actual);

--- Matcher alias for false boolean checks.
export let toBeFalse (actual : Bit) : Bit := isFalse(actual);

--- Matcher alias for integer greater-than checks.
export let toBeGreaterThan (actual : Int, bound : Int) : Bit :=
  greaterThan(actual, bound);

--- Matcher alias for integer less-than checks.
export let toBeLessThan (actual : Int, bound : Int) : Bit :=
  lessThan(actual, bound);

--- Build default integer matcher helpers.
export let expect (actual : Int) := expectInt(actual);
