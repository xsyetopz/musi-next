--! Host-backed time primitives.

let Core := import "musi:core";
let Int := Core.Int;
let Unit := Core.Unit;

@external(
  abi := .musi
)
let nowUnixMsIntrinsic() : Int;

export let nowUnixMs() : Int := unsafe {
  nowUnixMsIntrinsic();
};

@external(
  abi := .musi
)
let monotonicMsIntrinsic() : Int;

export let monotonicMs() : Int := unsafe {
  monotonicMsIntrinsic();
};

@external(
  abi := .musi
)
let sleepMsIntrinsic(ms : Int) : Unit;

export let sleepMs(ms : Int) : Unit := unsafe {
  sleepMsIntrinsic(ms);
};
