export let answer := 42;

export let greeting := "typed values";

export let byte := 255n8;

export let pair := (answer, greeting);

export let demo () := (answer, greeting, byte, pair);
demo();
