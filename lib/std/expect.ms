--! Expect success or failure result helpers.

--- Recoverable success or failure value.
export opaque let Expect [T, E] := data {
  | Success(T)
  | Failure(E)
};

--- Build successful expected value.
export let Success [T, E] (value : T) : Expect[T, E] := .Success(value);

--- Build failed expected value.
export let Failure [T, E] (value : E) : Expect[T, E] := .Failure(value);

--- Core operation for `isSuccess`.
let expectIsSuccess [T, E] (target : Expect[T, E]) : Bool :=
  match target (
  | .Success(_) => 0 = 0
  | .Failure(_) => 0 = 1
  );

--- Core operation for `isFailure`.
let expectIsFailure [T, E] (target : Expect[T, E]) : Bool :=
  match target (
  | .Success(_) => 0 = 1
  | .Failure(_) => 0 = 0
  );

--- Core operation for `unwrapOr`.
let expectUnwrapOr [T, E] (target : Expect[T, E], fallback : T) : T :=
  match target (
  | .Success(item) => item
  | .Failure(_) => fallback
  );

--- Core operation for `unwrapOrElse`.
let expectUnwrapOrElse [T, E] (target : Expect[T, E], fallback : E -> T) : T :=
  match target (
  | .Success(item) => item
  | .Failure(error) => fallback(error)
  );

--- Core operation for `map`.
let expectMap [T, E, U] (target : Expect[T, E], f : T -> U) : Expect[U, E] :=
  match target (
  | .Success(item) => Success[U, E](f(item))
  | .Failure(error) => Failure[U, E](error)
  );

--- Core operation for `mapFail`.
let expectMapFail [T, E, F] (
  target : Expect[T, E],
  f : E -> F
) : Expect[T, F,] :=
  match target (
  | .Success(item) => Success[T, F](item)
  | .Failure(error) => Failure[T, F](f(error))
  );

--- Core operation for `mapOr`.
let expectMapOr [T, E, U] (
  target : Expect[T, E],
  fallback : U,
  f : T -> U
) : U := match target (
| .Success(item) => f(item)
| .Failure(_) => fallback
);

--- Core operation for `mapOrElse`.
let expectMapOrElse [T, E, U] (
  target : Expect[T, E],
  onFailure : E -> U,
  onSuccess : T -> U
) : U :=
  match target (
  | .Success(item) => onSuccess(item)
  | .Failure(error) => onFailure(error)
  );

--- Core operation for `andThen`.
let expectAndThen [T, E, U] (
  target : Expect[T, E],
  f : T -> Expect[U, E]
) : Expect[U, E,] :=
  match target (
  | .Success(item) => f(item)
  | .Failure(error) => Failure[U, E](error)
  );

--- Core operation for `orElse`.
let expectOrElse [T, E, F] (
  target : Expect[T, E],
  f : E -> Expect[T, F]
) : Expect[T, F,] :=
  match target (
  | .Success(item) => Success[T, F](item)
  | .Failure(error) => f(error)
  );

--- Core operation for `fold`.
let expectFold [T, E, U] (
  target : Expect[T, E],
  onSuccess : T -> U,
  onFailure : E -> U
) : U :=
  match target (
  | .Success(item) => onSuccess(item)
  | .Failure(error) => onFailure(error)
  );

--- Core operation for `isSuccess`.
export let isSuccess [T, E] (target : Expect[T, E]) : Bool :=
  expectIsSuccess[T, E](target);

--- Core operation for `isFailure`.
export let isFailure [T, E] (target : Expect[T, E]) : Bool :=
  expectIsFailure[T, E](target);

--- Core operation for `unwrapOr`.
export let unwrapOr [T, E] (target : Expect[T, E], fallback : T) : T :=
  expectUnwrapOr[T, E](target, fallback);

--- Core operation for `unwrapOrElse`.
export let unwrapOrElse [T, E] (target : Expect[T, E], fallback : E -> T) : T :=
  expectUnwrapOrElse[T, E](target, fallback);

--- Core operation for `map`.
export let map [T, E, U] (target : Expect[T, E], f : T -> U) : Expect[U, E] :=
  expectMap[T, E, U](target, f);

--- Core operation for `mapFail`.
export let mapFail [T, E, F] (
  target : Expect[T, E],
  f : E -> F
) : Expect[T, F,] := expectMapFail[T, E, F](target, f);

--- Core operation for `mapOr`.
export let mapOr [T, E, U] (
  target : Expect[T, E],
  fallback : U,
  f : T -> U
) : U := expectMapOr[T, E, U](target, fallback, f);

--- Core operation for `mapOrElse`.
export let mapOrElse [T, E, U] (
  target : Expect[T, E],
  onFailure : E -> U,
  onSuccess : T -> U
) : U := expectMapOrElse[T, E, U](target, onFailure, onSuccess);

--- Core operation for `andThen`.
export let andThen [T, E, U] (
  target : Expect[T, E],
  f : T -> Expect[U, E]
) : Expect[U, E,] := expectAndThen[T, E, U](target, f);

--- Core operation for `orElse`.
export let orElse [T, E, F] (
  target : Expect[T, E],
  f : E -> Expect[T, F]
) : Expect[T, F,] := expectOrElse[T, E, F](target, f);

--- Core operation for `fold`.
export let fold [T, E, U] (
  target : Expect[T, E],
  onSuccess : T -> U,
  onFailure : E -> U
) : U := expectFold[T, E, U](target, onSuccess, onFailure);

--- Returns true for successful results.
export let (self : Expect[T, E]).isSuccess [T, E] () : Bool :=
  expectIsSuccess[T, E](self);

--- Returns true for failed results.
export let (self : Expect[T, E]).isFailure [T, E] () : Bool :=
  expectIsFailure[T, E](self);

--- Return the success value or the fallback.
export let (self : Expect[T, E]).unwrapOr [T, E] (fallback : T) : T :=
  expectUnwrapOr[T, E](self, fallback);

--- Return the success value or compute from the error.
export let (self : Expect[T, E]).unwrapOrElse [T, E] (fallback : E -> T) : T :=
  expectUnwrapOrElse[T, E](self, fallback);

--- Transform a success value.
export let (self : Expect[T, E]).map [T, E, U] (f : T -> U) : Expect[U, E] :=
  expectMap[T, E, U](self, f);

--- Transform an error value.
export let (self : Expect[T, E]).mapFail [T, E, F] (
  f : E -> F
) : Expect[T, F,] := expectMapFail[T, E, F](self, f);

--- Transform a success value or return the fallback.
export let (self : Expect[T, E]).mapOr [T, E, U] (
  fallback : U,
  f : T -> U
) : U := expectMapOr[T, E, U](self, fallback, f);

--- Transform either expect case into one value.
export let (
  self : Expect[T, E]
).mapOrElse [T, E, U] (onFailure : E -> U, onSuccess : T -> U) : U
  :=
  expectMapOrElse[T, E, U](self, onFailure, onSuccess);

--- Chain an operation that may fail.
export let (
  self : Expect[T, E]
).andThen [T, E, U] (f : T -> Expect[U, E]) : Expect[U, E,] :=
  expectAndThen[T, E, U](self, f);

--- Recover from an error with another expect.
export let (self : Expect[T, E]).orElse [T, E, F] (
  f : E -> Expect[T, F]
) : Expect[T, F,] := expectOrElse[T, E, F](self, f);

--- Fold both expect cases into one value.
export let (
  self : Expect[T, E]
).fold [T, E, U] (onSuccess : T -> U, onFailure : E -> U) : U :=
  expectFold[T, E, U](self, onSuccess, onFailure);
