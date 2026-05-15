export let makeAdder (delta : Int) := \(x : Int) => x + delta;

export let add3 := makeAdder(3);

export let closureResult := add3(7);

export let demo () := (closureResult, add3(40), makeAdder(10)(32));
demo();
