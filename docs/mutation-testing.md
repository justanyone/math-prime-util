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
(PR 109) tested 840 mutants: 605 caught, 202 missed, 33 timeouts
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
`--timeout 300 -j 6` (`iter`: `--timeout 1200`, because its baseline
`compat_iter` run alone takes over five minutes on a loaded host), running
the module's own tests only (`--lib`, `compat_<module>`, `prop_<module>` and
`mutants_rest`). A sample bounds the module score; it is not the module
score. The shard sizes were chosen so each sample has about twenty mutants.
The run was made on 2026-10-18 on the tree of this branch before the test
added below.

| Module | Shard | Tested | Caught | Timeout | Missed | Unviable | Sample score |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `arith` | 0/83 | 21 | 13 | 5 | 3 | 0 | 86 % |
| `combinat` | 0/62 | 20 | 20 | 0 | 0 | 0 | 100 % |
| `families` | 0/380 | 20 | 19 | 0 | 0 | 1 | 100 % |
| `iter` | 0/24 | 20 | 6 | 0 | 6 | 8 | 50 % |
| `modular` | 0/58 | 21 | 19 | 2 | 0 | 0 | 100 % |
| `count` | 0/55 | 21 | 17 | 0 | 4 | 0 | 81 % |
| `random` | 0/36 | 21 | 20 | 0 | 0 | 1 | 100 % |
| **Total** | | **144** | **114** | **7** | **13** | **10** | **90 %** |

Sample score = (caught + timeout) / (caught + timeout + missed); unviable
mutants are outside it. Sampled total: 144 mutants tested, 121
killed (114 caught, 7 timeouts), 13 missed, 10 unviable:
**90 %** of the 134 scored mutants.

### Survivors of the samples, triaged

`arith` (three missed, all in the Pollard rho walk of `factors::split`,
`src/arith/mod.rs`; the batch product of 64 differences is only an
accelerator, and the per-step fallback recomputes the gcd):

- `src/arith/mod.rs:38:46`, `*` to `+` in the step closure: the walk becomes
  `x -> 2x + c mod n`. A linear walk still meets itself modulo each prime
  factor, so the split is the same and only slower. Equivalent in output.
- `src/arith/mod.rs:47:83`, `%` to `/` in the batch product: the product
  collapses to 0, `gcd(0, n) == n` and the fallback loop at line 57 finds the
  same divisor one step at a time. Equivalent in output.
- `src/arith/mod.rs:61:26`, `!=` to `==` in the fallback loop: the loop stops
  at the first step with `d == 1`, the batch is not split and the next batch
  of 64 steps starts one step later and splits it. Equivalent in output.

The five `arith` timeouts (lines 27, 30, 47, 51 and 54 of the same walk) are
mutants that make the walk never terminate and count as killed.

`count` (four missed, all on `src/count/mod.rs:48`, the closed form
`x - x / 2` for `Counter::phi(x, 1)`):

- `src/count/mod.rs:48:22` `-` to `/`, `-` to `+`; `src/count/mod.rs:48:26`
  `/` to `%`, `/` to `*`: the `a == 1` branch is not reachable from the public
  functions. `legendre_phi` answers `a == 1` itself before it builds a
  `Counter`, the recursion in `phi` stops at the `a == 4` table, and
  `prime_count` sieves to at least 100, so `count` calls `phi(n, a)` with
  `a = pi(n^(1/4)) >= 2`. Real gap in the unit tests: the test
  `count::tests::phi_small_a_matches_brute_force` added with this change
  checks `Counter::phi(x, a)` for `x <= 300` and `a <= 4` against a brute
  force count and kills all four (the `*` mutant by an overflow panic, the
  others by value).

`modular` (two timeouts, counted as killed): `src/modular/mod.rs:21:21`
(`%` to `/` in `addmod`) and `src/modular/mod.rs:27:14` (`>` to `>=` in
`addmod_u`, which returns `n` instead of 0 when `a + b == n`) both make a
property test loop without end.

`iter` (six missed, all on `src/iter/forsquarefreeint.rs:39-40`, the segment
bound of `SquarefreeIntegersIn::next`; the sieve runs in segments of
`65_536 * 256` numbers so memory stays bounded, and a segment boundary is
not visible in the output):

- `src/iter/forsquarefreeint.rs:39:29` `-` to `/`, `39:34` `>` to `>=`,
  `39:34` `>` to `==`, `39:43` `*` to `+`: the test that decides whether a
  range is split into segments changes, so a range is split at another
  length or not at all. Every segment end is still at most `end`, so the
  same numbers come out in the same order. Equivalent in output; only the
  memory of one segment changes.
- `src/iter/forsquarefreeint.rs:40:35` `-` to `+`, `-` to `/`: the first
  segment ends one or two numbers later; the next segment starts after it.
  Equivalent in output.

The two caught mutants on the same lines (`>` to `<` and `*` to `/`) make a
segment reach past `end` and were caught by the range tests.

Unviable (outside the score): `src/families/is_gaussian_prime.rs:50:5`
(`norm -> Wide`) and `src/random/mod.rs:36:9` (`Stream::seeded -> Self`)
have no `Default` for the return type; the eight `iter` mutants replace the
`ControlFlow<B>` result of `run` (`src/iter/mod.rs:34:5`) and
`forsquarefreeint` (`src/iter/forsquarefreeint.rs:18:5`) with constructors
`ControlFlow` does not have.

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
