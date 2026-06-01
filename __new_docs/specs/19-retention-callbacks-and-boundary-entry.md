# 19. Retention, Callbacks, and Boundary Entry

Status: normative addendum for FFI retention and callback semantics.

## Non-retention default

All foreign parameters are non-retaining by default.

Retention is never inferred from type alone.

By default, a foreign callee may use passed values only during the dynamic call.

By default, a foreign callee must not:

- store passed values for later;
- use passed values after return;
- move passed values to another thread;
- schedule callbacks using passed values;
- retain `RawPtr`, `Root`, `Host`, callback, or view values.

`Root[T]` makes safe retention of Musi-managed values possible. It does not itself imply retention.

`Host[T]` represents host-owned values/resources. Passing `Host[T]` does not itself imply callee retention.

`RawPtr` is non-retaining by default.

A `RawPtr` produced from a pin region cannot outlive that pin region.

A `RawPtr` into movable managed storage cannot be retained.

## Retention groups

FFI retention is modeled through named retention groups.

Retention is allowed only when a foreign declaration explicitly assigns values to a named retention group.

A retention group defines:

- what is retained;
- who retains it;
- when retention ends;
- what releases it;
- whether cross-thread use is allowed;
- whether callback reentry is allowed;
- what happens on registration or creation failure.

Every retention group must have an end condition, such as:

- released by a specific foreign declaration;
- released through a `Host[T]` ownership protocol;
- valid until a completion callback;
- process-lifetime, if explicitly declared.

Cross-thread use is forbidden unless the retention group explicitly allows it.

Callback reentry into Musi is forbidden unless the callback or retention group explicitly allows it.

Unwinding, traps, and effects crossing the FFI boundary are forbidden unless explicitly declared by the ABI/runtime contract.

## `@retains(...)`

`@retains(...)` is the source metadata attribute for non-default retention.

It must stay compact.

It appears only when the default non-retaining rule is not enough.

Minimal intended shape:

```text
@retains(retained-name..., until := release-or-end-condition)
```

This is a syntax-candidate shape, not a claim about final attribute grammar.

The important semantic rule is:

```text
retained values + explicit end condition
```

Extra policy fields are only needed when the API requires them, such as thread policy or reentry policy.

Defaults:

- same-thread use only;
- reentry forbidden;
- no cross-boundary unwinding.

## Raw ABI callbacks

A callback crossing raw ABI is not an ordinary Musi closure by raw layout.

Raw ABI cannot call arbitrary Musi functions directly.

Raw ABI may call Musi only through explicitly created callback entries/trampolines.

A raw ABI callback entry must define:

- lifetime or retention group;
- rooting of captured Musi state;
- thread-entry policy;
- reentry policy;
- trap/unwind policy;
- calling convention.

Captured Musi state that must survive must be represented through `Root[T]` or equivalent VM-managed rooting.

## Host callbacks

Host callbacks through the VM embedding API are host interop, not raw ABI callback pointers.

The host enters Musi through an attached VM context.

The host does not receive raw managed addresses.
