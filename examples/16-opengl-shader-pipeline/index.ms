@foreign(abi := .c, symbol := "glClearColor")
let glClearColorNative (r : Float, g : Float, b : Float, a : Float) : Unit;
@foreign(abi := .c, symbol := "glClear")
let glClearNative (mask : Nat32) : Unit;
@foreign(abi := .c, symbol := "glViewport")
let glViewportNative (
  x : Int32,
  y : Int32,
  width : Int32,
  height : Int32
) : Unit;

export let colorBufferBit := 16384n32;

export let beginFrame (width : Int32, height : Int32) :=
  unsafe
  (
    glViewportNative(0, 0, width, height);
    glClearColorNative(0.1, 0.1, 0.15, 1.0);
    glClearNative(colorBufferBit)
  );

@foreign(abi := .c, symbol := "glCreateShader")
let glCreateShaderNative (kind : Nat32) : Nat32;

export let vertexShaderKind := 35633n32;

export let fragmentShaderKind := 35632n32;

export let createShaderPair () :=
  unsafe (
    glCreateShaderNative(vertexShaderKind),
    glCreateShaderNative(fragmentShaderKind),
  );

export let demo () :=
  (beginFrame, createShaderPair, vertexShaderKind, fragmentShaderKind);
demo();
