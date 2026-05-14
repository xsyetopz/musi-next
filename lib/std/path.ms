--! Path manipulation helpers.

let text := import "@std/text";

--- Platform path value.
export hidden let Path := data {
  | PathText(value : String)
};

--- Build a path from raw text.
export let fromString (value : String) : Path := .PathText(value := value);

--- Function for `toString`.
let pathToString (target : Path) : String :=
  match target (
  | .PathText(value := value) => value
  );

--- Return raw path text.
export let toString (target : Path) : String := pathToString(target);

let slash : String := "/";

let recur lastByteIndexFrom (
  source : String,
  needle : Int,
  index : Int,
  best : Int
) : Int := if index >= source.length() then best
  else if text.byteAt(source, index) = needle then lastByteIndexFrom(
    source,
    needle,
    index + 1,
    index,
  ) else lastByteIndexFrom(source, needle, index + 1, best);

let lastByteIndex (source : String, needle : Int) : Int :=
  lastByteIndexFrom(source, needle, 0, - 1);

--- Function for `join`.
let joinPath (target : Path, other : Path) : Path :=
  (
    let left := pathToString(target);
    let right := pathToString(other);
    if left = "" then fromString(right)
      else if right = "" then fromString(left)
      else if text.byteAt(right, 0) = 47 then fromString(right)
      else if text.byteAt(left, left.length() - 1) = 47 then fromString(
        text.concat(left, right)
      ) else fromString(text.concat(text.concat(left, slash), right))
    );

--- Function for `normalize`.
let normalizePath (target : Path) : Path := target;

--- Function for `dirName`.
let pathDirName (target : Path) : Path :=
  (
    let source := pathToString(target);
    let index := lastByteIndex(source, 47);
    if index < 0 then fromString("")
      else if index = 0 then fromString(slash)
      else fromString(text.slice(source, 0, index))
    );

--- Function for `baseName`.
let pathBaseName (target : Path) : String :=
  (
    let source := pathToString(target);
    let index := lastByteIndex(source, 47);
    if index < 0 then source
      else text.slice(source, index + 1, source.length())
    );

--- Function for `extName`.
let pathExtName (target : Path) : String :=
  (
    let base := pathBaseName(target);
    let index := lastByteIndex(base, 46);
    if index <= 0 then "" else text.slice(base, index, base.length())
  );

--- Function for `isAbsolute`.
let pathIsAbsolute (target : Path) : Bit :=
  text.byteAt(pathToString(target), 0) = 47;

--- Join two paths, using other unchanged when it is absolute.
export let join (target : Path, other : Path) : Path := joinPath(target, other);

--- Return normalized path text.
export let normalize (target : Path) : Path := normalizePath(target);

--- Return parent directory path.
export let dirName (target : Path) : Path := pathDirName(target);

--- Return final path component.
export let baseName (target : Path) : String := pathBaseName(target);

--- Return extension from final path component.
export let extName (target : Path) : String := pathExtName(target);

--- Returns true when path starts at root.
export let isAbsolute (target : Path) : Bit := pathIsAbsolute(target);

--- Return raw path text.
export let (self : Path).toString () : String := pathToString(self);

--- Join two paths.
export let (self : Path).join (other : Path) : Path := joinPath(self, other);

--- Normalize a path.
export let (self : Path).normalize () : Path := normalizePath(self);

--- Parent directory name.
export let (self : Path).dirName () : Path := pathDirName(self);

--- Base file name.
export let (self : Path).baseName () : String := pathBaseName(self);

--- Extension name, including leading `.` when Some.
export let (self : Path).extName () : String := pathExtName(self);

--- Returns true for absolute paths.
export let (self : Path).isAbsolute () : Bit := pathIsAbsolute(self);
