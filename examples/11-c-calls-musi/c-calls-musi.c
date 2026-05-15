#include <stdio.h>
#include <stdlib.h>

static int run_cmd(const char *label, const char *cmd) {
  printf("== %s ==\n$ %s\n", label, cmd);
  int code = system(cmd);
  if (code != 0) {
    fprintf(stderr, "[%s] failed with code %d\n", label, code);
  }
  return code;
}

int main(void) {
  int failed = 0;

  failed |= run_cmd(
      "check",
      "cargo run -q -p music -- check examples/10-sqlite3-interop/index.ms");
  failed |= run_cmd(
      "hil",
      "cargo run -q -p music -- disasm examples/10-sqlite3-interop/index.ms --level hil");
  failed |= run_cmd(
      "il",
      "cargo run -q -p music -- disasm examples/10-sqlite3-interop/index.ms --level seam");
  failed |= run_cmd(
      "build",
      "cargo run -q -p music -- build examples/10-sqlite3-interop/index.ms --out /tmp/calls-c.seam");
  failed |= run_cmd(
      "decomp",
      "cargo run -q -p musi -- decomp /tmp/calls-c.seam");

  if (failed != 0) {
    fprintf(stderr, "c-calls-musi: one or more Musi commands failed\n");
    return 1;
  }

  printf("c-calls-musi: all Musi commands succeeded\n");
  return 0;
}
