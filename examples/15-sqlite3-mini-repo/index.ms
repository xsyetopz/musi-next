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

@foreign(abi := .c, symbol := "sqlite3_exec")
let sqlite3Exec (
    db : CPtr,
    sql : CString,
    callback : CPtr,
    arg : CPtr,
    err : CPtr
) : Int32;

export let createTableSql : CString :=
    "create table if not exists note(id integer primary key, title text);";

export let execCreateTable () :=
    unsafe (
        sqlite3Exec(ptrNull(), createTableSql, ptrNull(), ptrNull(), ptrNull())
    );

export let demo () :=
    (openMemoryDb, execCreateTable, closeNullDb, createTableSql);
demo();
