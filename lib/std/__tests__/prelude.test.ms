let prelude := import "@std/prelude";
let testing := import "@std/testing";

export let test() :=
  (
    testing.describe("prelude");
    let someValue := prelude.Some[Int](7);
    let noneValue := prelude.None[Int]();
    let successValue := prelude.Success[Int, String](3);
    let failureValue := prelude.Failure[Int, String]("bad");
    let trueValue : prelude.Bool := prelude.true;
    let falseValue : prelude.Bool := prelude.false;
    testing.it("'Some' builds Some maybe", someValue.isSome());
    testing.it("'None' builds None maybe", noneValue.isNone());
    testing.it("'Success' builds successful expect", successValue.isSuccess());
    testing.it("'Failure' builds failed expect", failureValue.isFailure());
    testing.it("'true' exports Bool true variant", trueValue = .True);
    testing.it("'false' exports Bool false variant", falseValue = .False);
    testing.endDescribe()
  );
