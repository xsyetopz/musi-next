# 12. Modules, Imports, and Exports

Status: normative at source-surface level.

## No module keyword

`module` is not a keyword.

A file evaluates to a module record.

## Import

`import expr` evaluates an import expression and returns a module record.

```musi
let math := import "std/math";
let value := math.sqrt(9r64);
```

`as` is not import aliasing. Use ordinary `let` binding for import naming.

## Export

`export` marks module-record members.

```musi
export let pi : Real64 := 3.141592653589793r64;
```

`export` is a source-level boundary marker.

## Import/export records

A module exposes an export record. Imports receive module records and access members normally.

No special `module` declaration is introduced.

## Foreign export/import interaction

FFI import/export uses `@foreign(...)`, described in Chapter 13.

## Selection and namespace discipline

Imports produce value-like module records or imported surfaces that are named with ordinary `let` binding and selected normally.

`known import` remains a known value/import result where admitted. It does not imply separate namespace syntax.

`fixed` does not imply import syntax or namespace syntax.

No `::` operator is introduced by imports, `known`, or `fixed`.
