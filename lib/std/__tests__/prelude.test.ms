let prelude := import "@std/prelude";
let testing := import "@std/testing";

export let test () :=
  (
    testing.describe("prelude");
    let someValue := prelude.Some[Int](7);
    let noneValue := prelude.None[Int]();
    let successValue := prelude.Success[Int, String](3);
    let failureValue := prelude.Failure[Int, String]("bad");
    let trueValue := prelude.true;
    let falseValue := prelude.false;
    testing.it("'Some' builds Some maybe", someValue.isSome());
    testing.it("'None' builds None maybe", noneValue.isNone());
    testing.it("'Success' builds successful expect", successValue.isSuccess());
    testing.it("'Failure' builds failed expect", failureValue.isFailure());
    testing.it("'true' exports passing bit", trueValue = (0 = 0));
    testing.it("'false' exports failing bit", falseValue = (0 = 1));
    testing.endDescribe()
  );
