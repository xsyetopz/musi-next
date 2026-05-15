#!/usr/bin/env sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"

for example in examples/[0-9]*; do
  [ -d "$example" ] || continue
  printf '== %s\n' "$example"
  cargo run -q -p musi -- run "$example"
done
