# Builtin Registry

Musi builtins use a hidden catalog, not a public raw `Builtin` module.

- `music_builtin` owns compiler builtin names, foundation module specs, std package file paths, and intrinsic symbols.
- `musi:core`, focused `musi:*` host/VM primitive modules, and `@std/*` expose stable public APIs over that hidden layer.
- `Maybe[T]` and `Expect[T, E]` are ordinary library data types. Source sugar `?T`, `E!T`, and `??` lowers through those public types.
- Failure is handled with `match`, `let ... else`, or named helpers, not hidden exception syntax.
- `Bit` is the primitive condition type; `0` is false and `1` is true. There is no truthiness.
- `Bool` is an `@std/bool` library item, not a keyword or primitive. It binds `.True := 1` and `.False := 0`, with lowercase `true` and `false` exported aliases for users who prefer them, similar to C's `stdbool.h`.
- `@std/io` uses `write`, `writeLn`, `writeErr`, and `writeErrLn` for output, following the Ada/Pascal/C# `WriteLine` family.
- `Pin[T]` is compiler-owned scoped pin capability created only by `pin value as name in expr`.
- External boundary facts are modeled with `@external` metadata, stack effects, explicit unsafe surfaces, and ordinary import/export records.

This keeps user code on stdlib APIs while giving future VM/JIT work one central source of truth.
