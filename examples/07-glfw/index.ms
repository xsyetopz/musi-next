@foreign(abi := .c, symbol := "glfwInit")
let glfwInitNative () : Int32;
@foreign(abi := .c, symbol := "glfwTerminate")
let glfwTerminateNative () : Unit;
@foreign(abi := .c, symbol := "glfwPollEvents")
let glfwPollEventsNative () : Unit;

export let init () := unsafe (glfwInitNative());

export let terminate () := unsafe (glfwTerminateNative());

export let pollEvents () := unsafe (glfwPollEventsNative());

export let demo () := (init, pollEvents, terminate);
demo();
