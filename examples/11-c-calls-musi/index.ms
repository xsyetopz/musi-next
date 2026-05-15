export let note := "See c-calls-musi.c and c-calls-musi.sh in this directory.";

export let echoNote (text : String) := text;

export let demo () := echoNote(note);
demo();
