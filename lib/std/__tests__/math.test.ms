let math := import "@std/math";
let testing := import "@std/testing";

let close (actual : Float, expected : Float) : Bool :=
  math.fabs(actual - expected) < 0.000001;

export let test () :=
  (
    testing.describe("math");
    testing.it("'abs' keeps positive ints", testing.toBe(math.abs(3), 3));
    testing.it("'abs' flips negative ints", testing.toBe(math.abs(- 3), 3));
    testing.it(
      "'signInt' returns -1 for negatives",
      testing.toBe(math.signInt(- 7), - 1),
    );
    testing.it("'signInt' returns 0 for zero", testing.toBe(math.signInt(0), 0));
    testing.it(
      "'signInt' returns 1 for positives",
      testing.toBe(math.signInt(7), 1),
    );
    testing.it("'max' picks larger value", testing.toBe(math.max(2, 5), 5));
    testing.it("'min' picks smaller value", testing.toBe(math.min(2, 5), 2));
    testing.it(
      "'clamp' keeps in-range value",
      testing.toBe(math.clamp(5, 0, 10), 5),
    );
    testing.it(
      "'clamp' clamps below low",
      testing.toBe(math.clamp(- 1, 0, 10), 0),
    );
    testing.it(
      "'clamp' clamps above high",
      testing.toBe(math.clamp(11, 0, 10), 10),
    );
    testing.it("'fabs' flips negative floats", close(math.fabs(- 3.0), 3.0));
    testing.it("'pow' raises powers", close(math.pow(2.0, 5.0), 32.0));
    testing.it("'isNan' detects NaN", math.isNan(math.sqrt(- 1.0)));
    testing.it("'isFinite' accepts finite values", math.isFinite(2.0));
    testing.endDescribe()
  );
