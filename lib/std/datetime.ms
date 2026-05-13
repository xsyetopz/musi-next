--! Date and time module.

let timeHost := import "musi:time";

--- Milliseconds per second.
export let millisPerSecond := known 1000;
--- Milliseconds per minute.
export let millisPerMinute := known 60 * millisPerSecond;
--- Milliseconds per hour.
export let millisPerHour := known 60 * millisPerMinute;

--- Span of time in milliseconds.
export hidden let Duration := data {
  | DurationMillis(millis : Int)
};

--- Wall-clock Unix time in milliseconds.
export hidden let UnixTime := data {
  | UnixMillis(millis : Int)
};

--- Monotonic instant in milliseconds.
export hidden let Instant := data {
  | InstantMillis(millis : Int)
};

--- Duration from milliseconds.
export let millis (value : Int) : Duration := .DurationMillis(millis := value);

--- Duration from seconds.
export let seconds (value : Int) : Duration := millis(value * millisPerSecond);

--- Duration from minutes.
export let minutes (value : Int) : Duration := millis(value * millisPerMinute);

--- Duration from hours.
export let hours (value : Int) : Duration := millis(value * millisPerHour);

--- Return duration as milliseconds.
export let durationMillis (target : Duration) : Int :=
  match target (
  | .DurationMillis(millis := value) => value
  );

--- Return duration as whole seconds.
export let durationSeconds (target : Duration) : Int :=
  durationMillis(target) / millisPerSecond;

--- Return duration as whole minutes.
export let durationMinutes (target : Duration) : Int :=
  durationMillis(target) / millisPerMinute;

--- Unix wall time now.
export let nowUnix () : UnixTime := .UnixMillis(millis := timeHost.nowUnixMs());

--- Return Unix wall time as milliseconds.
export let unixTimeMillis (target : UnixTime) : Int :=
  match target (
  | .UnixMillis(millis := value) => value
  );

--- Return Unix wall time as whole seconds.
export let unixTimeSeconds (target : UnixTime) : Int :=
  unixTimeMillis(target) / millisPerSecond;

--- Monotonic instant now.
export let monotonic () : Instant :=
  .InstantMillis(millis := timeHost.monotonicMs());

--- Return monotonic instant as milliseconds.
export let instantMillis (target : Instant) : Int :=
  match target (
  | .InstantMillis(millis := value) => value
  );

--- Sleep for a duration.
export let sleep (target : Duration) : Unit :=
  timeHost.sleepMs(durationMillis(target));

--- Duration as milliseconds.
export let (self : Duration).toMillis () : Int := durationMillis(self);

--- Duration as whole seconds.
export let (self : Duration).toSeconds () : Int := durationSeconds(self);

--- Duration as whole minutes.
export let (self : Duration).toMinutes () : Int := durationMinutes(self);

--- Unix wall time as milliseconds.
export let (self : UnixTime).toMillis () : Int := unixTimeMillis(self);

--- Unix wall time as whole seconds.
export let (self : UnixTime).toSeconds () : Int := unixTimeSeconds(self);

--- Monotonic instant as milliseconds.
export let (self : Instant).toMillis () : Int := instantMillis(self);
