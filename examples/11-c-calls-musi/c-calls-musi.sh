#!/usr/bin/env sh
set -eu

cc -std=c11 -O2 -Wall -Wextra -pedantic \
  examples/11-c-calls-musi/c-calls-musi.c \
  -o /tmp/c-calls-musi

/tmp/c-calls-musi
