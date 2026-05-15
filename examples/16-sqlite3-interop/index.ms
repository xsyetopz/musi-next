let core := import "musi:core";
let CString := core.CString;
let CPtr := core.CPtr;

@foreign(abi := .c, symbol := "sqlite3_open")
let sqlite3Open (filename : CString, db : CPtr) : Int32;
@foreign(abi := .c, symbol := "sqlite3_close")
let sqlite3Close (db : CPtr) : Int32;
@foreign(abi := .c, symbol := "sqlite3_exec")
let sqlite3Exec (
    db : CPtr,
    sql : CString,
    callback : CPtr,
    arg : CPtr,
    err : CPtr
) : Int32;
@foreign(abi := .musi)
let ptrNull () : CPtr;

export let createTableSql : CString :=
    "create table if not exists note(id integer primary key, title text);";

export let demo () :=
    (sqlite3Open, sqlite3Exec, sqlite3Close, createTableSql, ptrNull);
demo();
