--! Host-backed process primitives.

let Core := import "musi:core";
let Int := Core.Int;
let String := Core.String;
let Unit := Core.Unit;

@external(
  abi := .musi
)
let argCountIntrinsic() : Int;

export let argCount() : Int := unsafe {
  argCountIntrinsic();
};

@external(
  abi := .musi
)
let argAtIntrinsic(index : Int) : String;

export let argAt(index : Int) : String := unsafe {
  argAtIntrinsic(index);
};

@external(
  abi := .musi
)
let cwdIntrinsic() : String;

export let cwd() : String := unsafe {
  cwdIntrinsic();
};

@external(
  abi := .musi
)
let runIntrinsic(command : String) : Int;

export let run(command : String) : Int := unsafe {
  runIntrinsic(command);
};

@external(
  abi := .musi
)
let exitIntrinsic(code : Int) : Unit;

export let exit(code : Int) : Unit := unsafe {
  exitIntrinsic(code);
};
