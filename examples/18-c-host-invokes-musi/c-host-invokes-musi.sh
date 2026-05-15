#!/usr/bin/env sh
set -eu

cc -std=c11 -O2 -Wall -Wextra -pedantic \
  examples/18-c-host-invokes-musi/c-host-invokes-musi.c \
  -o /tmp/c-host-invokes-musi

/tmp/c-host-invokes-musi
