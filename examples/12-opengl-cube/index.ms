let core := import "musi:core";

let CPtr := core.CPtr;
let CUInt := core.Nat32;
let CInt := core.Int32;

@foreign(abi := .c, symbol := "glGenBuffers")
let glGenBuffersNative (count : CInt, out : CPtr) : Unit;
@foreign(abi := .c, symbol := "glBindBuffer")
let glBindBufferNative (target : CUInt, buffer : CUInt) : Unit;
@foreign(abi := .c, symbol := "glBufferData")
let glBufferDataNative (
  target : CUInt,
  size : Nat,
  bytesPtr : CPtr,
  usage : CUInt
) : Unit;
@foreign(abi := .musi)
let ptrNull () : CPtr;

export let arrayBuffer : CUInt := 34962n32;

export let staticDraw : CUInt := 35044n32;

export let cubeVertices :=
  [
    - 1.0,
    - 1.0,
    - 1.0,
    1.0,
    0.0,
    0.0,
    1.0,
    - 1.0,
    - 1.0,
    0.0,
    1.0,
    0.0,
    1.0,
    1.0,
    - 1.0,
    0.0,
    0.0,
    1.0,
    - 1.0,
    1.0,
    - 1.0,
    1.0,
    1.0,
    0.0,
  ];

export let uploadCubeVbo () :=
  unsafe
  (
    glGenBuffersNative(1, ptrNull());
    glBindBufferNative(arrayBuffer, 0n32);
    glBufferDataNative(arrayBuffer, 96n, ptrNull(), staticDraw)
  );

export let demo () := (cubeVertices, uploadCubeVbo, arrayBuffer, staticDraw);
demo();
