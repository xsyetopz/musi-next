export hidden let MaybeInt := data {
  | None()
  | Some(value : Int)
};

export let plusOneMaybe (value : MaybeInt) : MaybeInt :=
  match value (
  | .None() => .None()
  | .Some(value := x) => .Some(value := x + 1)
  );

export let fallbackInt (value : MaybeInt, fallback : Int) :=
  match value (
  | .None() => fallback
  | .Some(value := x) => x
  );

export let sampleSome : MaybeInt := .Some(value := 41);

export let sampleNone : MaybeInt := .None();

export let demo () :=
  (
    plusOneMaybe(sampleSome),
    plusOneMaybe(sampleNone),
    fallbackInt(plusOneMaybe(sampleSome), 0),
    fallbackInt(plusOneMaybe(sampleNone), 99),
  );
demo();
