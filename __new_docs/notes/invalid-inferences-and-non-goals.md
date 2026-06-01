# Invalid Inferences and Non-Goals

Status: guardrail notes.

## Invalid inferences

The following inferences are invalid:

- `unsafe` means ordinary checking is disabled.
- `unsafe` means managed objects are pinned.
- `pin` grants permission to dereference raw pointers.
- A RawPtr or pinned view may escape its lexical pin region.
- A foreign parameter may be retained unless explicitly declared.
- `Root[T]` implies retention by type alone.
- `Host[T]` implies retention by type alone.
- `RawPtr` owns, roots, or extends lifetime.
- Host interop is the same thing as raw FFI.
- A host callback is automatically a raw ABI callback.
- Ordinary Musi `data` has C ABI layout.
- External layout metadata changes source semantics.
- `opaque` means existential erasure.
- `erased` means representation opacity.
- Import introduces standalone namespace syntax.

## Non-goals

This addendum does not define:

- bytecode instruction encoding;
- object header layout;
- exact Immix block/line representation;
- exact root table representation;
- exact host embedding API function names;
- exact callback trampoline machine layout;
- exact final syntax for `@layout(...)` fields;
- exact final syntax for callback type expressions;
- exact final syntax for non-moving storage types.

Where syntax is not final, the archive marks it as candidate syntax.
