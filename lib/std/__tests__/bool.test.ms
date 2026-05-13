let bool := import "@std/bool";
let testing := import "@std/testing";

export let test () :=
  (
    testing.describe("bool");
    testing.it("true aliases True variant", bool.true = .True);
    testing.it("false aliases False variant", bool.false = .False);
    testing.endDescribe()
  );
