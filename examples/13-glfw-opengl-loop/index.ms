let core := import "musi:core";

let CPtr := core.CPtr;
let CInt := core.Int32;

@foreign(abi := .c, symbol := "glfwInit")
let glfwInitNative () : CInt;
@foreign(abi := .c, symbol := "glfwTerminate")
let glfwTerminateNative () : Unit;
@foreign(abi := .c, symbol := "glfwPollEvents")
let glfwPollEventsNative () : Unit;
@foreign(abi := .c, symbol := "glfwSwapBuffers")
let glfwSwapBuffersNative (window : CPtr) : Unit;
@foreign(abi := .c, symbol := "glfwWindowShouldClose")
let glfwWindowShouldCloseNative (window : CPtr) : CInt;
@foreign(abi := .musi)
let ptrNull () : CPtr;

export let createContext () := unsafe (glfwInitNative());

export let runFrame (window : CPtr) :=
  unsafe
  (
    glfwPollEventsNative();
    glfwSwapBuffersNative(window)
  );

export let shouldClose (window : CPtr) :=
  if unsafe (glfwWindowShouldCloseNative(window)) = 0 then 0 = 1 else 0 = 0;

export let demo () := (createContext, runFrame, shouldClose, ptrNull);
demo();
