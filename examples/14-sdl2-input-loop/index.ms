let core := import "musi:core";
let CPtr := core.CPtr;

@foreign(abi := .c, symbol := "SDL_Init")
let sdlInitNative (flags : Nat32) : Int32;
@foreign(abi := .c, symbol := "SDL_Quit")
let sdlQuitNative () : Unit;
@foreign(abi := .c, symbol := "SDL_PollEvent")
let sdlPollEventNative (event : CPtr) : Int32;

export let initVideoFlag := 32n32;

export let initVideo () := unsafe (sdlInitNative(initVideoFlag));

export let quit () := unsafe (sdlQuitNative());

export let pollEvent (event : CPtr) := unsafe (sdlPollEventNative(event));

export let quitRequested (eventType : Int32) := eventType = 256;

export let demo () :=
    (initVideoFlag, initVideo, pollEvent, quitRequested(256), quit);
demo();
