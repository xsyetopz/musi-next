# Agent Do-Not-Invent Index

Status: normative for AI agents and implementation agents.

This file is intentionally blunt. If a construct is not listed as allowed in the specs, do not add it.

## Do not invent these source constructs

```text
if / then / else-if syntax
return
null
for loop
foreach loop
async / await / spawn
try / catch
foreign keyword
foreign block
module declaration keyword
class / impl / instance
fn / fun / def
:: namespace syntax
#[...] attributes
reader macros
token macros
implicit semicolon insertion
newline-separated expressions
```

## Do not rename locked terms

Use:

```text
known
fixed
pin
unsafe
Any
Unknown
Empty
Root[T]
Host[T]
RawPtr[T]
```

Do not replace them with:

```text
comptime
static
pinned
trust
Dynamic
Never
void pointer
native handle
C pointer
```

Unless quoting/comparing an external language, do not use external names as Musi design names.

## Do not conflate these axes

```text
known   != fixed
fixed   != pin
pin     != unsafe
unsafe  != raw pointer lifetime extension
Any     != Unknown
Any     != erased Trait
Host[T] != Root[T]
Root[T] != RawPtr[T]
Host[T] != RawPtr[T]
# datum != ~ syntax quote
$ template interpolation != ~ syntax splice
#[...] datum sequence != attribute
... spread/splat/rest != #[T] sequence value
```

## Required response to gaps

If a requested feature is absent from the spec, report:

```text
This is not specified.
```

Then identify the exact chapter where it would belong.

Do not fill gaps from Rust, Swift, Zig, C++, JavaScript, TypeScript, Python, Lisp, Scheme, Java, CLR, JVM, Lua, or WebAssembly unless the developer explicitly asks for prior-art comparison.

## SEIL/SEBC warning

SEIL and SEBC concrete syntax/encoding are not locked by the 1.2 addendum.

Allowed statement:

```text
SEIL is intended as a fixed Lisp-shaped/S-expression VM language.
SEBC is the bytecode encoding of SEIL.
```

Disallowed statement:

```text
SEIL syntax is exactly this form: ...
SEBC opcode layout is exactly this encoding: ...
```

until a later SEIL/SEBC chapter locks those details.
