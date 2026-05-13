--! System module.

@external(abi := .musi)
let Musi__targetOs () : String;

@external(abi := .musi)
let Musi__targetArch () : String;

@external(abi := .musi)
let Musi__targetArchFamily () : String;

@external(abi := .musi)
let Musi__targetFamily () : String;

@external(abi := .musi)
let Musi__targetPointerWidth () : Int;

@external(abi := .musi)
let Musi__targetEndian () : String;

@external(abi := .musi)
let Musi__matchesOs (value : String) : Bit;

@external(abi := .musi)
let Musi__matchesArch (value : String) : Bit;

@external(abi := .musi)
let Musi__matchesFamily (value : String) : Bit;

--- Raw target operating system name.
export let os : String := unsafe {
  Musi__targetOs();
};
--- Raw target architecture name.
export let arch : String := unsafe {
  Musi__targetArch();
};
--- Raw target architecture family name.
export let archFamily : String := unsafe {
  Musi__targetArchFamily();
};
--- Raw target platform family name.
export let family : String := unsafe {
  Musi__targetFamily();
};
--- Target pointer width in bits.
export let pointerWidth : Int := unsafe {
  Musi__targetPointerWidth();
};
--- Target byte order name.
export let endian : String := unsafe {
  Musi__targetEndian();
};

--- Raw target fact record.
export let target :=
  {
    os := os,
    arch := arch,
    archFamily := archFamily,
    family := family,
    pointerWidth := pointerWidth,
    endian := endian,
  };

--- Returns true when value names current OS.
export let matchesOs (value : String) : Bit := unsafe {
  Musi__matchesOs(value);
};
--- Returns true when value names current architecture.
export let matchesArch (value : String) : Bit := unsafe {
  Musi__matchesArch(value);
};
--- Returns true when value names current platform family.
export let matchesFamily (value : String) : Bit := unsafe {
  Musi__matchesFamily(value);
};

--- Returns true on Unix-family targets.
export let isUnix () : Bit := matchesFamily("unix");
--- Returns true on Darwin-family targets.
export let isDarwin () : Bit := matchesFamily("darwin");
--- Returns true on Windows targets.
export let isWindows () : Bit := matchesFamily("windows");
--- Returns true on WebAssembly-family targets.
export let isWebAssembly () : Bit := matchesFamily("webassembly");
