--! Input and output helper namespace.

let ioHost := import "musi:io";

--- Read one line from stdin.
export let readLine () : String := ioHost.readLine();

--- Write text to stdout.
export let write (value : String) : Unit := ioHost.write(value);

--- Write text plus newline to stdout.
export let writeLn (value : String) : Unit := ioHost.writeLn(value);

--- Write text to stderr.
export let writeErr (value : String) : Unit := ioHost.writeErr(value);

--- Write text plus newline to stderr.
export let writeErrLn (value : String) : Unit := ioHost.writeErrLn(value);
