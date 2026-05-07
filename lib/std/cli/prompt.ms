--! Interactive command-line prompt helpers.

let io := import "@std/io";
let maybe := import "@std/maybe";
let text := import "@std/text";

let Maybe := maybe.Maybe;

--- Prompt and read one line.
export let prompt (message : String) : String :=
  (
    io.write(message);
    io.readLine()
  );

--- Read and trim one line.
export let readTrimmedLine () : String := io.readLine().trim();

--- Prompt, read, and trim one line.
export let promptTrimmed (message : String) : String := prompt(message).trim();

--- Read a non-empty trimmed line.
export let readNonEmptyLine () : Maybe[String] :=
  match readTrimmedLine() (
  | value if value.isEmpty() => maybe.None[String]()
  | value => maybe.Some[String](value)
  );

--- Prompt for a non-empty trimmed line.
export let promptNonEmpty (message : String) : Maybe[String] :=
  match promptTrimmed(message) (
  | value if value.isEmpty() => maybe.None[String]()
  | value => maybe.Some[String](value)
  );
