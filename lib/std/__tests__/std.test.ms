let std := import "@std";
let testing := import "@std/testing";

export let test () :=
  (
    testing.describe("std");
    let sameBytes := std.bytes.fromArray([1, 2]);
    let oneList := std.collections.list.of[Int](1);
    let currentOs := std.os.targetOs(std.os.current);
    let noneInt := std.maybe.None[Int]();
    let successExpect := std.expect.Success[Int, String](1);
    let nullPointer := std.ffi.ptr.null[std.ffi.CInt]();
    let rootPath := std.path.fromString("/");
    testing.it(
      "root re-exports 'bytes' family",
      sameBytes.equals(std.bytes.fromArray([1, 2])),
    );
    testing.it(
      "root re-exports 'prelude' family",
      std.prelude.Some[Int](1).isSome(),
    );
    testing.it("root re-exports 'ascii' family", std.ascii.isHexDigit(70));
    testing.it(
      "root re-exports 'expect' family",
      std.expect.isSuccess[Int, String](successExpect),
    );
    testing.it(
      "root re-exports 'cmp' family",
      std.cmp.compareInt(1, 2).isLess(),
    );
    testing.it(
      "root re-exports 'list' family",
      std.collections.list.equalsInt(
        oneList,
        std.collections.list.fromArray[Int]([1]),
      ),
    );
    testing.it("root re-exports 'text' family", std.text.length("musi") > 0);
    testing.it("root re-exports 'json' family", std.json.isValid("{}"));
    testing.it(
      "root re-exports 'math' family",
      testing.toBe(std.math.clamp(9, 0, 4), 4),
    );
    testing.it(
      "root re-exports 'libm' family",
      std.maybe.isSome[Float](std.libm.checkedSqrt(4.0)),
    );
    testing.it("root re-exports 'libc' family", std.libc.equals("musi", "musi"));
    testing.it(
      "root re-exports 'ffi' family",
      std.ffi.ptr.isNull[std.ffi.CInt](nullPointer),
    );
    testing.it(
      "root re-exports 'bits' family",
      testing.toBe(std.bits.toInt8(std.bits.fromInt8(3)), 3),
    );
    testing.it("root re-exports 'fmt' family", std.fmt.int(42) = "42");
    testing.it(
      "root re-exports 'errors' family",
      std.errors.error("failed").toString() = "failed",
    );
    testing.it(
      "root re-exports 'encoding' family",
      std.encoding.hex.encode("a") /= "",
    );
    testing.it(
      "root re-exports 'datetime' family",
      std.datetime.seconds(2).toMillis() = 2000,
    );
    testing.it(
      "root re-exports 'random' family",
      std.random.nextIntInRange(1, 10) >= 1,
    );
    testing.it(
      "root re-exports 'crypto' family",
      std.crypto.sha256Hex("") /= "",
    );
    testing.it("root re-exports 'fs' family", std.fs.exists(rootPath));
    testing.it(
      "root re-exports 'path' family",
      std.path.fromString("dir/file.txt").baseName() = "file.txt",
    );
    testing.it("root re-exports 'io' family",
        (
          std.io.write("");
          0 = 0
        ));
    testing.it(
      "root re-exports 'env' family",
      std.env.key("PATH").toString() = "PATH",
    );
    testing.it(
      "root re-exports 'process' family",
      std.process.argsCount() >= 0,
    );
    testing.it("root re-exports 'os' family", currentOs.toString() /= "",);
    testing.it(
      "root re-exports 'maybe' family",
      testing.toBe(std.maybe.unwrapOr[Int](noneInt, 5), 5),
    );
    testing.it(
      "root re-exports 'log' family",
      std.log.warnLevel.name() = "warn",
    );
    testing.it("root re-exports 'cli' family", std.cli.process.argsCount() >= 0);
    testing.it(
      "root re-exports 'uuid' family",
      std.uuid.isValid("550e8400-e29b-41d4-a716-446655440000"),
    );
    testing.it("root re-exports 'semver' family", std.semver.isValid("1.2.3"));
    testing.it("root re-exports 'assert' family", std.assert.isTrue(1 = 1));
    testing.it("root re-exports 'testing' family", std.testing.toBeTrue(1 = 1));
    testing.it(
      "local alias preserves import record target",
      std.bytes.fromArray([3, 4]).equals(std.bytes.fromArray([3, 4])),
    );
    testing.endDescribe()
  );
