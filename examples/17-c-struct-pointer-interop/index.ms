let core := import "musi:core";
let CString := core.CString;
let CPtr := core.CPtr;

@foreign(abi := .c, symbol := "sqlite3_open")
let sqlite3Open (filename : CString, db : CPtr) : Int32;
@foreign(abi := .c, symbol := "sqlite3_close")
let sqlite3Close (db : CPtr) : Int32;
@foreign(abi := .musi)
let ptrNull () : CPtr;

export let openMemoryDb () := unsafe (sqlite3Open(":memory:", ptrNull()));

export let closeNullDb () := unsafe (sqlite3Close(ptrNull()));

export hidden let Vec3 := data {
  | Vec3(x : Float, y : Float, z : Float)
};

@foreign(abi := .c, symbol := "musi_vec3_normalize")
let cNormalizeVec3 (input : CPtr, output : CPtr) : Unit;

export let normalizeVec3Raw (input : CPtr, output : CPtr) :=
  unsafe (cNormalizeVec3(input, output));

export let demo () := (normalizeVec3Raw, openMemoryDb, closeNullDb);
demo();
