let bool := import "@std/bool";
let testing := import "@std/testing";

export let test () :=
  (
    testing.describe("bool");
    testing.it("true names passing bit", bool.true = (0 = 0));
    testing.it("false names failing bit", bool.false = (0 = 1));
    testing.endDescribe()
  );
