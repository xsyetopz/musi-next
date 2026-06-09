# Musi Full Spec Solution Options

Purpose: show three possible directions for what Musi should be. These are not maturity levels. They are different answers to: how many first-class citizens can Musi fit while staying WYSIWYG, capability-checked, small-core, scriptable, embeddable, programmable, and extendable?

Parser/runtime complexity is not the rejection gate. Rejection gates are missing ability, forced workaround, weak bidirectional FFI, weak embedding, weak extension, weak self-hosting, or hidden behavior.

Syntax note: examples marked `current grammar shape` use forms present in `grammar/musi.ebnf`. Examples marked `conceptual proposal sketch, not current grammar` are direction sketches only and must not be copied into grammar/specs as accepted syntax without a later lock.

## Shared constraints

All directions keep:

- explicit behavior over hidden magic;
- long-term maintainability over convenience;
- one obvious way for each behavior;
- verbose spelling where it makes behavior visible;
- maximal munch + one-token-lookahead source syntax;
- managed default runtime with explicit `fixed`, `unmanaged`, `Address`, `Region`, `Access[T]`, and `Access[mut T]`;
- `known` execution by verified SEIL, not source-tree evaluation;
- runtime/compiler intrinsics declared through `musi:rt` metadata;
- no Rust-derived design source.

## A — Musi as Explicit Systems Script

Direction: Musi is a direct interpreted systems scripting language. The language feels like explicit source modules plus ordinary values/calls. Host power exists, but most behavior stays ordinary Musi API surface instead of special runtime reflection.

First-class citizens:

- source modules and scripts;
- `let`, `import`, `export`;
- `data`, `shape`, receiver methods;
- `known` values/functions;
- explicit capabilities as ordinary values;
- explicit memory access through `Address`, `Region`, `Access[T]`, `Access[mut T]`;
- explicit failures through ordinary result types.

What becomes easy:

- run a `.ms` file as a module/script;
- pass resources in explicitly;
- call host-provided functions through imported modules;
- write low-level code without C pointer syntax;
- self-host compiler pieces without hidden host hooks.

Where it avoids workarounds:

- no fake global host object;
- no implicit dynamic lookup;
- no hidden exception channel for host failures;
- no magic address-of/deref syntax.

Current grammar shape:

```musi
let host := import "musi:host";

let main(args : []Text) : Expect[Unit, Error] := (
  host.log("start");
  Unit
);

export let run(args : []Text) : Expect[Unit, Error] := main(args);
```

Current grammar shape:

```musi
let Buffer := data {
  let bytes : []Nat8;
};

let (self : Buffer).len() : Nat := self.bytes.len();
```

Conceptual proposal sketch, not current grammar:

```musi
let memory : Region := host.region("scratch");
let view : Access[mut Nat8] := memory.access[Nat8](0, 1024);
```

Open gaps closed this way:

- package/module rules choose explicit script/module manifests;
- host API stays small and ordinary-call oriented;
- dynamic behavior uses declared capability APIs, not reflection;
- metadata supports WYSIWYG diagnostics/decompile only as needed.

## B — Musi as Host-Programmable VM

Direction: Musi is a serious embedded programmable VM for apps, games, tools, plugins, devices, and hosts. The language still stays small-core, but host interaction becomes a first-class design center: bidirectional FFI, handles, lifecycle, cancellation, and capability tables are not afterthoughts.

First-class citizens:

- all A citizens;
- bidirectional FFI contracts;
- host resource handles;
- capability tables;
- script lifecycle: load, verify, link, init, call, yield/resume, cancel;
- dynamic argpacks for host/plugin calls;
- native module declarations;
- structured host failure outcomes.

What becomes easy:

- host calls Musi and Musi calls host;
- plugins declare needed capabilities;
- host resources cross boundaries as explicit handles;
- scripts can be loaded, stopped, resumed, cancelled, and diagnosed;
- low-memory hosts declare limits without changing language meaning.

Where it avoids workarounds:

- no ad-hoc C shim per plugin shape;
- no hidden host-global API;
- no fake stringly dynamic call protocol;
- no lost failure/context when crossing the host boundary.

Conceptual proposal sketch, not current grammar:

```musi
@extern(.C, "host_log")
let host_log(text : CString) : CInt;

@extern(.C, "plugin_update")
export let plugin_update(dt : Float32) : CInt := (
  host_log("update");
  0
);
```

Current grammar shape:

```musi
let Plugin := data {
  let name : Text;
};

let (self : Plugin).update(dt : Float32) : Expect[Unit, Error] := (
  Unit
);
```

Conceptual proposal sketch, not current grammar:

```musi
@host.capabilities(.Filesystem, .Clock, .Graphics)
export let plugin : Plugin := #{ name := "tool" };
```

Open gaps closed this way:

- package rules include file, bundle, archive, and host-provided module sources;
- capability table, dynamic argpack, keyed storage, and host outcome schemas become normative;
- frame/control specs include script lifecycle and cancellation;
- FFI specs define both directions as first-class.

## C — Musi as Reflective Programmable System

Direction: Musi is a programmable systems environment. Code, metadata, modules, frames, failures, capabilities, and SEIL/runtime structures are explicit typed values when authority is present. This maximizes language power without using hidden magic.

First-class citizens:

- all B citizens;
- typed metadata schemas;
- module reflection;
- SEIL/decompile metadata;
- frame and failure inspection;
- capability inspection;
- schema derivation;
- compile-time code/data generation through `known` APIs;
- reflective runtime structures behind explicit capabilities.

What becomes easy:

- write tools in Musi that inspect Musi modules;
- build schema/decompile/debug metadata without host-only machinery;
- inspect running frames/failures when capability permits;
- build live systems and self-hosted tooling with fewer hidden dependencies.

Where it avoids workarounds:

- no separate host-only reflection engine;
- no untyped metadata blobs as the normal path;
- no tool metadata that cannot round-trip;
- no special debugger/runtime backdoor outside capability model.

Current grammar shape:

```musi
let BuildInfo := data {
  let module_name : Text;
  let version : Text;
};

let info : known BuildInfo := #{ module_name := "demo", version := "1" };
```

Current grammar shape:

```musi
@tool.schema(name := "Plugin")
let Plugin := data {
  let name : Text;
};
```

Conceptual proposal sketch, not current grammar:

```musi
known let exports := reflect.module(Self).exports();
let frame := runtime.current_frame(capability);
```

Open gaps closed this way:

- tool/source metadata schemas become language-visible, typed, and stable;
- verifier diagnostics, SEIL text, binary rows, and decompile metadata get exact schemas;
- frame, failure, capability, and host APIs become inspectable typed runtime structures;
- self-hosting and live tooling become explicit language goals.

## Direction comparison

| Question                  | A — Explicit Systems Script       | B — Host-Programmable VM                   | C — Reflective Programmable System                |
| ------------------------- | --------------------------------- | ------------------------------------------ | ------------------------------------------------- |
| Main feel                 | ordinary explicit scripts/modules | embedded plugin/runtime VM                 | programmable system environment                   |
| First-class host relation | imported host APIs                | bidirectional FFI + lifecycle              | typed host/runtime reflection                     |
| Extension style           | modules + declared intrinsics     | modules + native extensions + capabilities | typed metadata + reflective APIs + extensions     |
| Dynamic behavior          | explicit capability APIs          | host/plugin dynamic protocols              | typed reflective protocols                        |
| Tooling power             | enough for WYSIWYG diagnostics    | host/tool metadata for embedding           | full typed metadata/decompile/reflection          |
| Workaround avoided most   | basic scripting/systems access    | embedding/FFI lifecycle gaps               | self-hosting/live tooling/runtime inspection gaps |

## Exact current unknown coverage

Every current `specs/**` unknown maps to one row below. USER selection chooses the language direction; later docs fold chosen rules back into owning specs.

| Current gap                               | A — Explicit Systems Script       | B — Host-Programmable VM           | C — Reflective Programmable System     |
| ----------------------------------------- | --------------------------------- | ---------------------------------- | -------------------------------------- |
| control block layout patterns             | canonical source-form lowering    | script-friendly canonical lowering | exhaustive lowering + metadata         |
| generator object representation           | stable resumable object           | host lifecycle suspension object   | typed resumable runtime object         |
| nested cleanup ordering                   | lexical LIFO cleanup table        | lifecycle-aware cleanup table      | exact cleanup order matrix             |
| every source expression lowering          | one canonical lowering            | compact loaded-script lowering     | exhaustive algorithm                   |
| source-map/tool metadata payloads         | WYSIWYG essentials                | layered host/tool metadata         | full near-identical schemas            |
| import path resolution + module packaging | explicit manifest + roots         | host/file/bundle/archive sources   | exact names, paths, archives, versions |
| package format + module discovery         | simple explicit package           | plugin/script package profiles     | normative package/archive spec         |
| standard native module catalog            | self-host essentials              | host extension catalog             | full native catalog                    |
| SEAM failure to Musi diagnostics          | stable code mapping               | host boundary mapping              | full diagnostic/failure map            |
| trap taxonomy                             | stable structured traps           | host/profile declared traps        | exact reason taxonomy                  |
| numeric overflow + FP exceptions          | hardware-faithful or checked/trap | declared host support/trap         | exact opcode/schema behavior           |
| access/region permission metadata         | stable permission rows            | host/resource permission rows      | full permission schema                 |
| module-name canonicalization              | exact simple symbol rule          | host/package canonical rule        | full canonical naming spec             |
| multi-module package/archive              | manifest package                  | plugin/script archive              | normative archive format               |
| compatibility edge schema                 | explicit table only               | compact explicit table             | full schema + verifier corpus          |
| type/metadata binary encodings            | stable schema-packed rows         | compact profile rows               | all row schemas specified              |
| ABI descriptor grammar                    | stable ABI metadata grammar       | bidirectional host ABI grammar     | full ABI descriptor grammar            |
| verifier diagnostic codes/messages        | mature stable catalog             | compact host/tool text             | complete typed diagnostics             |
| capability table schema                   | fixed capability table            | host/resource capability table     | full capability schema                 |
| dynamic argument-pack representation      | fixed argpack                     | bounded host argpack               | full argpack protocol                  |
| keyed-storage constraints                 | fixed key/value rules             | host/resource key/value profile    | full keyed-storage schema              |
| reason-code enum                          | stable enum                       | compact host/profile enum          | full normative enum                    |
| host outcome representation               | stable outcome ABI                | C-compatible host outcome ABI      | versioned normative API                |
| numeric failure to trap kind map          | stable mapping table              | profile-declared mapping           | exhaustive mapping table               |
| in-memory frame layout                    | stable root/safepoint frame       | compact script frame               | exact frame layout/API                 |
| handler matching table format             | stable handler table              | compact handler table              | exact handler table format             |
| suspended cancellation API                | resume/cancel ABI                 | host cancellation API              | normative cancellation API             |
| object header layout                      | conformance header contract       | compact profile header             | exact object header                    |
| GC algorithm parameters                   | semantic policy + tunables        | host/profile limits                | exact parameters/probes                |
| write/read barrier rules                  | required barrier obligations      | compact barrier encodings          | exact barrier spec                     |
| finalization/destructor semantics         | no implicit finalizers            | explicit host handle cleanup       | explicit decision table, default none  |
| module init after load/verify/link/init   | deterministic init order          | host/package init order            | exact normative init algorithm         |
| canonical whitespace policy               | one formatter                     | compact formatter                  | exact round-trip formatting spec       |
| text parse/assemble diagnostic wording    | stable text codes                 | compact host/tool text             | exact wording/catalog                  |
| `tool` metadata schemas                   | WYSIWYG schemas                   | layered host/tool schemas          | full typed schemas                     |
| section-family ids beyond core            | stable extension ids              | host/profile extension ranges      | full id registry policy                |
| per-row binary schemas                    | schema-packed core rows           | compact profile rows               | every row schema locked                |
| package/container transport               | outside core package layer        | plugin/script bundle transport     | full package/container transport       |
