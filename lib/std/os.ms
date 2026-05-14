--! Operating system module.

let sys := import "#sys";

--- Operating system name.
export hidden let Os := data {
  | OsName(value : String)
};

--- CPU architecture name.
export hidden let Arch := data {
  | ArchName(value : String)
};

--- Platform family name.
export hidden let Family := data {
  | FamilyName(value : String)
};

--- Byte order.
export hidden let Endian := data {
  | Little
  | Big
};

--- Current compilation/runtime target facts.
export hidden let Target := data {
  | TargetInfo(
    os : String,
    arch : String,
    family : String,
    pointerWidth : Int,
    endian : String,
  )
};

--- Function for `os`.
let makeOs (value : String) : Os := .OsName(value := value);

--- Function for `arch`.
let makeArch (value : String) : Arch := .ArchName(value := value);

--- Build CPU architecture value.
export let arch (value : String) : Arch := makeArch(value);
--- Function for `family`.
let makeFamily (value : String) : Family := .FamilyName(value := value);

--- Build platform family value.
export let family (value : String) : Family := makeFamily(value);

--- Return operating system name text.
export let osText (target : Os) : String :=
  match target (
  | .OsName(value := value) => value
  );

--- Return CPU architecture name text.
export let archText (target : Arch) : String :=
  match target (
  | .ArchName(value := value) => value
  );

--- Return platform family name text.
export let familyText (target : Family) : String :=
  match target (
  | .FamilyName(value := value) => value
  );

--- Convert byte-order text into Endian.
export let endianFromString (value : String) : Endian :=
  if value = "big" then .Big else .Little;

--- Return current target facts from system bindings.
export let current : Target :=
  .TargetInfo(
    os := sys.os,
    arch := sys.arch,
    family := sys.family,
    pointerWidth := sys.pointerWidth,
    endian := sys.endian,
  );

--- Return target operating system.
export let targetOs (target : Target) : Os :=
  match target (
  | .TargetInfo(
    os := value,
    arch := _,
    family := _,
    pointerWidth := _,
    endian := _,
  ) => makeOs(value)
  );

--- Return target CPU architecture.
export let targetArch (target : Target) : Arch :=
  match target (
  | .TargetInfo(
    os := _,
    arch := value,
    family := _,
    pointerWidth := _,
    endian := _,
  ) => makeArch(value)
  );

--- Return target platform family.
export let targetFamily (target : Target) : Family :=
  match target (
  | .TargetInfo(
    os := _,
    arch := _,
    family := value,
    pointerWidth := _,
    endian := _,
  ) => makeFamily(value)
  );

--- Return target pointer width in bits.
export let targetPointerWidth (target : Target) : Int :=
  match target (
  | .TargetInfo(
    os := _,
    arch := _,
    family := _,
    pointerWidth := value,
    endian := _,
  ) => value
  );

--- Return target byte order.
export let targetEndian (target : Target) : Endian :=
  match target (
  | .TargetInfo(
    os := _,
    arch := _,
    family := _,
    pointerWidth := _,
    endian := value,
  ) => endianFromString(value)
  );

--- Returns true when current OS matches value.
export let matchesOs (value : Os) : Bit := sys.matchesOs(osText(value));
--- Returns true when current architecture matches value.
export let matchesArch (value : Arch) : Bit := sys.matchesArch(archText(value));
--- Returns true when current family matches value.
export let matchesFamily (value : Family) : Bit :=
  sys.matchesFamily(familyText(value));
--- Returns true on Unix-family targets.
export let isUnix () : Bit := matchesFamily(makeFamily("unix"));
--- Returns true on Darwin-family targets.
export let isDarwin () : Bit := matchesFamily(makeFamily("darwin"));
--- Returns true on Windows-family targets.
export let isWindows () : Bit := matchesFamily(makeFamily("windows"));
--- Returns true on WebAssembly-family targets.
export let isWebAssembly () : Bit := matchesFamily(makeFamily("webassembly"));

--- Convert value to string.
export let (self : Os).toString () : String := osText(self);

--- Convert value to string.
export let (self : Arch).toString () : String := archText(self);

--- Convert value to string.
export let (self : Family).toString () : String := familyText(self);

--- Target architecture.
export let (self : Target).arch () : Arch := targetArch(self);

--- Target family.
export let (self : Target).family () : Family := targetFamily(self);

--- Target pointer width.
export let (self : Target).pointerWidth () : Int := targetPointerWidth(self);

--- Target endian value.
export let (self : Target).endian () : Endian := targetEndian(self);
