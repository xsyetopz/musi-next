export let prepare (value : Int) :=
  (
    let doubled := value * 2;
    let shifted := doubled + 1;
    shifted
  );

export let demo () := (prepare(3), prepare(10));
demo();
