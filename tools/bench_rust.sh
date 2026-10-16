#!/bin/sh
# Internal wrapper. The Perl driver creates the private manifest and sample dir.
set -eu
if [ "$#" -ne 3 ]; then
  echo 'Usage: sh tools/bench_rust.sh MANIFEST SAMPLE_DIR TARGET_DIR' >&2
  exit 2
fi
export CRITERION_HOME="$2"
exec cargo bench --offline --locked --manifest-path "$1" --target-dir "$3" \
  --bench int -- --output-format bencher --noplot \
  --warm-up-time 0.05 --measurement-time 0.1 --sample-size 10
