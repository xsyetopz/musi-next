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

export let demo () := (openMemoryDb, closeNullDb);
demo();
