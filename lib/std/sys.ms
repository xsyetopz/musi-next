--! System helper namespace.

@external(
  abi := .musi
)
let targetOsIntrinsic() : String;

@external(
  abi := .musi
)
let targetArchIntrinsic() : String;

@external(
  abi := .musi
)
let targetArchFamilyIntrinsic() : String;

@external(
  abi := .musi
)
let targetFamilyIntrinsic() : String;

@external(
  abi := .musi
)
let targetPointerWidthIntrinsic() : Int;

@external(
  abi := .musi
)
let targetEndianIntrinsic() : String;

@external(
  abi := .musi
)
let jitSupportedIntrinsic() : Int;

@external(
  abi := .musi
)
let jitBackendIntrinsic() : String;

@external(
  abi := .musi
)
let jitIsaIntrinsic() : String;

@external(
  abi := .musi
)
let matchesOsIntrinsic(value : String) : Bit;

@external(
  abi := .musi
)
let matchesArchIntrinsic(value : String) : Bit;

@external(
  abi := .musi
)
let matchesFamilyIntrinsic(value : String) : Bit;

--- Raw target operating system name.
export let os : String := unsafe {
  targetOsIntrinsic();
};
--- Raw target architecture name.
export let arch : String := unsafe {
  targetArchIntrinsic();
};
--- Raw target architecture family name.
export let archFamily : String := unsafe {
  targetArchFamilyIntrinsic();
};
--- Raw target platform family name.
export let family : String := unsafe {
  targetFamilyIntrinsic();
};
--- Target pointer width in bits.
export let pointerWidth : Int := unsafe {
  targetPointerWidthIntrinsic();
};
--- Target byte order name.
export let endian : String := unsafe {
  targetEndianIntrinsic();
};

--- Raw JIT support facts.
export let jit :=
  {
    supported := unsafe {
    jitSupportedIntrinsic();
  } > 0,
    backend := unsafe {
    jitBackendIntrinsic();
  },
    isa := unsafe {
    jitIsaIntrinsic();
  },
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
    jit := jit,
  };

--- Returns true when value names current OS.
export let matchesOs(value : String) : Bit := unsafe {
  matchesOsIntrinsic(value);
};
--- Returns true when value names current architecture.
export let matchesArch(value : String) : Bit := unsafe {
  matchesArchIntrinsic(value);
};
--- Returns true when value names current platform family.
export let matchesFamily(value : String) : Bit := unsafe {
  matchesFamilyIntrinsic(value);
};

--- Returns true on Unix-family targets.
export let isUnix() : Bit := matchesFamily("unix");
--- Returns true on Darwin-family targets.
export let isDarwin() : Bit := matchesFamily("darwin");
--- Returns true on Windows targets.
export let isWindows() : Bit := matchesFamily("windows");
--- Returns true on WebAssembly-family targets.
export let isWebAssembly() : Bit := matchesFamily("webassembly");
--- Returns true when current target supports JIT.
export let supportsJit() : Bit := jit.supported;
