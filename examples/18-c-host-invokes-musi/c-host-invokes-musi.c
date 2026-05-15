#include <stdio.h>
#include <stdlib.h>

static int run(const char *command) {
  printf("$ %s\n", command);
  return system(command);
}

int main(void) {
  int status = run("cargo run -q -p musi -- run examples/12-number-guess-game");
  if (status != 0) {
    return status;
  }
  return run("cargo run -q -p musi -- run examples/24-software-rasterizer");
}
