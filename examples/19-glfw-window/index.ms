@foreign(abi := .c, symbol := "glfwInit")
let glfwInitNative () : Int32;
@foreign(abi := .c, symbol := "glfwTerminate")
let glfwTerminateNative () : Unit;
@foreign(abi := .c, symbol := "glfwPollEvents")
let glfwPollEventsNative () : Unit;

export let windowApi :=
    (glfwInitNative, glfwPollEventsNative, glfwTerminateNative);

export let demo () := windowApi;
demo();
