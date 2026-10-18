# Mutation testing

The crate is measured with [cargo-mutants](https://mutants.rs) 27.1.0.
The nightly workflow (`.github/workflows/nightly.yml`, job `mutants`) runs the
whole crate and uploads `mutants.out`. This file records the runs made by hand
on the development hosts, the survivors and their triage. A caught mutant means
a test failed; a timeout means the mutant made the tests hang (counted as
killed); unviable mutants do not compile and are outside the score. Score =
(caught + timeout) / (caught + timeout + missed).

## Inventory

`cargo mutants --list --all-features` on dev-002 (2026-10-18, after PR 118):
22,282 candidates in 22 modules.

| Module | Candidates |
| --- | ---: |
| `arith` | 1,664 |
| `bigint` | 273 |
| `bits` | 20 |
| `bits_pos` | 48 |
| `combinat` | 1,234 |
| `config` | 35 |
| `count` | 1,104 |
| `digits` | 317 |
| `factor` | 2,933 |
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
| **Total** | **22,282** |

## factor: full run and survivor closure

PR 92, 101 and 109 added the mutation-killing tests `tests/mutants_factor*.rs`
and `tests/mutants_rest.rs`. The full `src/factor` run of the first batch
(aragog, PR 109) tested 840 mutants: 605 caught, 202 missed, 33 timeouts
(76 % killed before the group re-runs). PR 109 then reasoned or killed the
survivors by group and left fifteen sites open.

The open sites were re-run on 2026-10-18 on the named lines only
(`--re` on `file:line`, 56 mutants, `-j 3 --timeout 300`; tests: lib,
`mutants_factor*`, `compat_factor`, `prop_factor`, `compat_primality`):
41 caught, 4 timeouts, 11 missed. Nine of the eleven are equivalent with a
written reason and two (`is_carmichael.rs:64`, the retry polynomial after a
failed Brent attempt) are unreachable with any known input. The reasons are
in the commit text of `task/fx9-mpu-close-the-open-factor-mutation-survivors-20261018-x`.

## Other modules: sampled shards

A whole-crate run takes days on the available hosts, so each module that had
no run yet was sampled with one `--shard 0/N` slice (about twenty mutants
per module, chosen by cargo-mutants' deterministic hash, not by hand) with
`--timeout 300 -j 3`, running the module's own tests only (`--lib`,
`compat_<module>`, `prop_<module>` and `mutants_rest`). A sample bounds the
module score; it is not the module score.

The shards were prepared as follows (shard size chosen so each sample has
about twenty mutants): `arith` 0/83, `combinat` 0/62, `families` 0/380,
`iter` 0/24, `modular` 0/58, `count` 0/55, `random` 0/36. The run was started
on jeltz on 2026-10-18 and stopped by host memory pressure during the `arith`
baseline build, before any mutant was tested. No sampled number is recorded
yet; the table below is to be filled by the next run.

| Module | Shard | Tested | Caught | Timeout | Missed | Sample kill rate |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| `arith` | 0/83 | - | - | - | - | - |
| `combinat` | 0/62 | - | - | - | - | - |
| `families` | 0/380 | - | - | - | - | - |
| `iter` | 0/24 | - | - | - | - | - |
| `modular` | 0/58 | - | - | - | - | - |
| `count` | 0/55 | - | - | - | - | - |
| `random` | 0/36 | - | - | - | - | - |

Sampled total: none yet.

### Survivors of the samples, triaged

None recorded yet: the sampled runs have not completed.

## Running it

```sh
# whole crate, as the nightly job does
cargo mutants --all-features --in-place -j 2
# one module, its own tests only
cargo mutants --all-features -f 'src/arith/**' -j 2 --timeout 300 -- --lib --test compat_arith --test prop_arith
# one shard of a module
cargo mutants --all-features -f 'src/families/**' --shard 0/380 -j 2 --timeout 300 -- --lib --test compat_families --test prop_families
```

cargo-mutants copies the tree once per job and builds there: set `TMPDIR`
to a disk with room for a build directory per job, and run at `nice -n 10`
on shared hosts.
