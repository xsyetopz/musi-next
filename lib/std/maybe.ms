--! Maybe value helpers for Some or None.

--- Maybe value with either Some value or None.
export opaque let Maybe [T] := data {
  | Some(T)
  | None
};

--- Build Some maybe value.
export let Some [T] (value : T) : Maybe[T] := .Some(value);

--- Build None maybe value.
export let None [T] () : Maybe[T] := .None;

--- Core operation for `isSome`.
let maybeIsSome [T] (target : Maybe[T]) : Bool :=
  match target (
  | .Some(_) => 0 = 0
  | .None => 0 = 1
  );

--- Core operation for `isSome`.
export let isSome [T] (target : Maybe[T]) : Bool := maybeIsSome[T](target);

--- Core operation for `isNone`.
let maybeIsNone [T] (target : Maybe[T]) : Bool :=
  match target (
  | .Some(_) => 0 = 1
  | .None => 0 = 0
  );

--- Core operation for `isNone`.
export let isNone [T] (target : Maybe[T]) : Bool := maybeIsNone[T](target);

--- Core operation for `unwrapOr`.
let maybeUnwrapOr [T] (target : Maybe[T], fallback : T) : T :=
  match target (
  | .Some(item) => item
  | .None => fallback
  );

--- Core operation for `unwrapOr`.
export let unwrapOr [T] (target : Maybe[T], fallback : T) : T :=
  maybeUnwrapOr[T](target, fallback);

--- Core operation for `unwrapOrElse`.
let maybeUnwrapOrElse [T] (target : Maybe[T], fallback :() -> T) : T :=
  match target (
  | .Some(item) => item
  | .None => fallback()
  );

--- Core operation for `unwrapOrElse`.
export let unwrapOrElse [T] (target : Maybe[T], fallback :() -> T) : T :=
  maybeUnwrapOrElse[T](target, fallback);

--- Core operation for `map`.
let maybeMap [T, U] (target : Maybe[T], f : T -> U) : Maybe[U] :=
  match target (
  | .Some(item) => Some[U](f(item))
  | .None => None[U]()
  );

--- Core operation for `map`.
export let map [T, U] (target : Maybe[T], f : T -> U) : Maybe[U] :=
  maybeMap[T, U](target, f);

--- Core operation for `mapOr`.
let maybeMapOr [T, U] (target : Maybe[T], fallback : U, f : T -> U) : U :=
  match target (
  | .Some(item) => f(item)
  | .None => fallback
  );

--- Core operation for `mapOr`.
export let mapOr [T, U] (target : Maybe[T], fallback : U, f : T -> U) : U :=
  maybeMapOr[T, U](target, fallback, f);

--- Core operation for `mapOrElse`.
let maybeMapOrElse [T, U] (
  target : Maybe[T],
  fallback :() -> U,
  f : T -> U
) : U :=
  match target (
  | .Some(item) => f(item)
  | .None => fallback()
  );

--- Core operation for `mapOrElse`.
export let mapOrElse [T, U] (
  target : Maybe[T],
  fallback :() -> U,
  f : T -> U
) : U := maybeMapOrElse[T, U](target, fallback, f);

--- Core operation for `andThen`.
let maybeAndThen [T, U] (target : Maybe[T], f : T -> Maybe[U]) : Maybe[U] :=
  match target (
  | .Some(item) => f(item)
  | .None => None[U]()
  );

--- Core operation for `andThen`.
export let andThen [T, U] (target : Maybe[T], f : T -> Maybe[U]) : Maybe[U] :=
  maybeAndThen[T, U](target, f);

--- Core operation for `orElse`.
let maybeOrElse [T] (target : Maybe[T], fallback : Maybe[T]) : Maybe[T] :=
  match target (
  | .Some(_) => target
  | .None => fallback
  );

--- Core operation for `orElse`.
export let orElse [T] (target : Maybe[T], fallback : Maybe[T]) : Maybe[T] :=
  maybeOrElse[T](target, fallback);

--- Core operation for `filter`.
let maybeFilter [T] (target : Maybe[T], predicate : T -> Bool) : Maybe[T] :=
  match target (
  | .Some(item)if predicate(item) => target
  | .Some(_) => None[T]()
  | .None => None[T]()
  );

--- Core operation for `filter`.
export let filter [T] (target : Maybe[T], predicate : T -> Bool) : Maybe[T] :=
  maybeFilter[T](target, predicate);

--- Core operation for `fold`.
let maybeFold [T, U] (target : Maybe[T], onNone : U, onSome : T -> U) : U :=
  match target (
  | .Some(item) => onSome(item)
  | .None => onNone
  );

--- Core operation for `fold`.
export let fold [T, U] (target : Maybe[T], onNone : U, onSome : T -> U) : U :=
  maybeFold[T, U](target, onNone, onSome);

--- Returns true when the maybe contains a value.
export let (self : Maybe[T]).isSome [T] () : Bool := maybeIsSome[T](self);

--- Returns true when the maybe contains None value.
export let (self : Maybe[T]).isNone [T] () : Bool := maybeIsNone[T](self);

--- Return the contained value or the fallback.
export let (self : Maybe[T]).unwrapOr [T] (fallback : T) : T :=
  maybeUnwrapOr[T](self, fallback);

--- Return the contained value or compute the fallback.
export let (self : Maybe[T]).unwrapOrElse [T] (fallback :() -> T) : T :=
  maybeUnwrapOrElse[T](self, fallback);

--- Transform a Some value.
export let (self : Maybe[T]).map [T, U] (f : T -> U) : Maybe[U] :=
  maybeMap[T, U](self, f);

--- Transform a Some value or return the fallback.
export let (self : Maybe[T]).mapOr [T, U] (fallback : U, f : T -> U) : U :=
  maybeMapOr[T, U](self, fallback, f);

--- Transform a Some value or compute the fallback.
export let (self : Maybe[T]).mapOrElse [T, U] (
  fallback :() -> U,
  f : T -> U
) : U := maybeMapOrElse[T, U](self, fallback, f);

--- Chain an operation that may return None.
export let (self : Maybe[T]).andThen [T, U] (f : T -> Maybe[U]) : Maybe[U] :=
  maybeAndThen[T, U](self, f);

--- Return this maybe or a fallback maybe.
export let (self : Maybe[T]).orElse [T] (fallback : Maybe[T]) : Maybe[T] :=
  maybeOrElse[T](self, fallback);

--- Keep a Some value only when the predicate passes.
export let (self : Maybe[T]).filter [T] (predicate : T -> Bool) : Maybe[T] :=
  maybeFilter[T](self, predicate);

--- Fold both maybe cases into one value.
export let (self : Maybe[T]).fold [T, U] (onNone : U, onSome : T -> U) : U :=
  maybeFold[T, U](self, onNone, onSome);
