export let ints := [1, 2, 3, 4];

export let nested := [[1, 2], [3, 4]];

export let withTail := [1, 2, 3, 4, 5];

export let replay (value) := value;

export let demo () := (replay(ints), replay(nested), replay(withTail));
demo();
