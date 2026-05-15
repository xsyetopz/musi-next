let math := import "@std/math";

export hidden let Pixel := data {
  | Pixel(x : Int, y : Int, shade : Int)
};

export hidden let Triangle := data {
  | Triangle(ax : Int, ay : Int, bx : Int, by : Int, cx : Int, cy : Int)
};

export let framebufferWidth := 8;

export let framebufferHeight := 6;

export let clampX (x : Int) := math.clamp(x, 0, framebufferWidth - 1);

export let clampY (y : Int) := math.clamp(y, 0, framebufferHeight - 1);

export let pixel (x : Int, y : Int, shade : Int) :=
  .Pixel(x := clampX(x), y := clampY(y), shade := math.clamp(shade, 0, 255));

export let boundingBoxSample (tri : Triangle) :=
  match tri (
  | .Triangle(ax := ax, ay := ay, bx := bx, by := by, cx := cx, cy := cy) => (
    pixel(math.min(math.min(ax, bx), cx), math.min(math.min(ay, by), cy), 32),
    pixel(math.max(math.max(ax, bx), cx), math.max(math.max(ay, by), cy), 224),
  )
  );

export let demoTriangle :=
  .Triangle(ax := 1, ay := 1, bx := 6, by := 2, cx := 3, cy := 5);

export let demo () := boundingBoxSample(demoTriangle);
demo();
