export let add (a : Int, b : Int) := a + b;

export let inc (x : Int) := add(x, 1);

export let applyTwice (x : Int) := inc(inc(x));

export let computed := applyTwice(10);

export let demo () := (add(20, 22), inc(41), applyTwice(20), computed);
demo();
