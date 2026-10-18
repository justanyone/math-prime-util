# Mutation testing measurements

This is a partial measurement, not a whole-crate score or a READY claim.
Inventory measured on 2026-10-18 with cargo-mutants 27.1.0:
`cargo mutants --list --json --all-features`.

| Module | Candidates |
| --- | ---: |
| `arith` | 1,662 |
| `bits` | 20 |
| `bits_pos` | 48 |
| `combinat` | 1,234 |
| `config` | 35 |
| `count` | 1,086 |
| `digits` | 317 |
| `factor` | 2,932 |
| `families` | 7,598 |
| `int` | 319 |
| `iter` | 476 |
| `lucas` | 331 |
| `misc_gap` | 76 |
| `modular` | 1,164 |
| `primality` | 839 |
| `proving` | 683 |
| `random` | 729 |
| `rational` | 588 |
| `real` | 943 |
| `sieve` | 525 |
| `vec` | 383 |
| **Total** | **21,988** |

## Execution scope

The continuation runs all candidates in `bits` and `config`, with all features,
two mutation jobs, offline Cargo, and library tests. Integration tests and
rustdoc tests are outside this measurement. No candidates are excluded as
"equivalent" before testing. The remaining modules still need execution.

```sh
nice -n 19 cargo mutants --all-features \
  -f 'src/bits/**' -f 'src/config/**' -j 2 \
  --timeout 60 --build-timeout 120 -C --offline \
  -o /tmp/mpu-mutation-small -- --lib
```

The run results and survivor review will be recorded below. A caught mutant
means a test failed; unviable mutants do not enter the score denominator.
Report the strict kill rate as caught / (caught + missed + timeout), and
report timeouts separately because they are not assertion-based kills.
Never extrapolate a module score to the whole crate.
