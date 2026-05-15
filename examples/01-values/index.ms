export let fortyTwo := 42;

export let greeting := "hello";

export let nat8Value := 255n8;

export let pair := (1, 2);

export let packValues (left : Int, text : String, byte : Nat8, tuple : (
    Int,
    Int,
  )) := (left, text, byte, tuple);

export let demo () := packValues(fortyTwo, greeting, nat8Value, pair);
demo();
