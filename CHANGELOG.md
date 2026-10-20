# Changelog

All notable changes to this crate are listed here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate uses
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - 2026-10-20

### Added

- `BigUint` forms of 242 more names (feature `bigint`): 324 of the 428 names now have one, none stays native
  only, and for 104 a `BigUint` form does not apply. Each module has oracle fixtures recorded from MPU 0.75's pure-Perl
  path on inputs past 2^64 and a property test that checks the `BigUint` form against the native one.
  - `bigint::combinat`: Bell, Catalan and Stirling numbers, partitions, factorial forms, permutation ranks,
    Bernoulli and harmonic fractions and Ramanujan tau.
  - `bigint::digits`: digit lists and strings, Zeckendorf form, palindromes, Harshad and happy numbers, digital roots.
  - `bigint::functions` and `bigint::predicates`: functions and predicates of one integer (totients, `sopf`,
    prime signature, perfect and prime powers, powerful, smooth, rough and practical numbers, and more).
  - `bigint::modular`: `submod`, `negmod`, `muladdmod`, `mulsubmod`, `chinese2`, `lucasuvmod`, `is_qr`, `qnr`,
    `cornacchia`, `pisano_period` and `allrootmod`.
  - `bigint::primality` and `bigint::factor`: the Frobenius and Perrin pseudoprime tests, Chen, delicate,
    Gaussian and safe primes, and Fermat, Lehman and Chebyshev factoring.
  - `bigint::rational`: continued fractions, the Calkin-Wilf and Stern-Brocot sequences and `next_farey`.
  - `bigint::sequences`: Fibonacci and Lucas numbers, primorials, `consecutive_integer_lcm`, power sums, vector
    sums and products, `floor_sum` and `frobenius_number`.
  - `bigint::sets`: the set and vector operations.
  - `bigint::family_bounds`: bounds and estimates of the prime family counts and `n`th values, and `inverse_li`.
  - `bigint::family_counts`: the exact family counts and sums whose run time stays practical past 2^64.
  - `bigint::families`: next and previous family members and the family lists of short ranges, and
    `goldbach_pair_count` and `goldbach_pairs` (an odd `n` of any size at once; an even `n` past 2^64 tests
    every prime from `n/2` up, as MPU's pure-Perl path does).
  - `bigint::family_nth`: `nth_almost_prime`, `nth_omega_prime`, `nth_powerful` and `powerful_numbers` past
    2^64.
  - One-integer names: `inverse_sigma0` and `inverse_sigma0_count` (`bigint::functions`),
    `is_congruent_number` (`bigint::predicates`) and `farey_rank` (`bigint::rational`).
  - `bigint::iter`: `fordivisors`, `forfactored`, `forsemiprimes`, `foralmostprimes`, `forsquarefree` and
    `forsquarefreeint`.
  - `bigint::random`: random integers and random, safe, strong and proven primes of any size (Maurer and
    Shawe-Taylor, with certificates).
- `docs/bigint-coverage.md` gives the reason for each of the 104 names without a `BigUint` form (`n.a.`). New
  `n.a.` reasons, each measured on MPU 0.75: the lucky, Ramanujan and twin-prime sieves (9 names; their native
  forms are checked against MPU just below 2^64 by `tests/compat_native_top_lucky.rs`), `hclassno` and
  `integer_complexity` (MPU rejects `n` above 2^63), `farey`, `is_catalan_pseudoprime` and `is_pillai` (MPU's
  pure-Perl path does not finish past 2^64), and `nth_semiprime` and `nth_prime_power` (MPU finds them from
  exact counts near or past 2^64 and does not return).
- `docs/upstream-findings.md`: findings 20 to 24 from the bigint oracles; two are confirmed defects
  (`semiprime_count` on a range across 2^64, `powerful_numbers` past 2^64).

### Changed

- `forsquarefreeint` on a short range near `u64::MAX` sieves primes in bounded blocks and factors the range.
- The CI bench job runs the base commit and the head of each bench back to back on one runner and reports a
  benchmark as slower only when the lower bound of its change estimate is above 10 % in the first run and in
  two repeats, and the head adds more than 1 ns to the mean time. A bench of less than a few ns moves by one or
  two CPU cycles when its code is placed at another address.

## [1.1.0] - 2026-10-19

### Added

- Big integers past `u128`. The `Int` trait gives `is_prime`, `factor`, `powmod`, `gcd`, `next_prime`, checked
  arithmetic, shifts and decimal text one surface over `u64`, `u128` and, with the cargo feature `bigint`,
  `num_bigint::BigUint`.
- Feature `gmp`: `rug::Integer` as a third `Int` backend. It links the installed GMP and never builds GMP from source.
- Feature `serde`: `Serialize` and `Deserialize` on the certificate types of `bigint::proving`.
- `bigint::arith`: the integer core on `BigUint`: the functions of `int`, `modular`, `combinat` and `arith` with
  the same names (signed results such as `gcdext`, `jacobi` and `kronecker` use `BigInt`).
- `bigint::primality`: BPSW, the pseudoprime and Lucas tests and the prime walkers on `BigUint`. Above 2^64
  `is_prime` answers 1 (probable prime), as MPU does.
- `bigint::factor` and `bigint::divisors`: factoring and the divisor functions on `BigUint`.
- `bigint::counts`: prime counts, count bounds, nth prime, the prime stream and `is_polygonal` past `u64`.
- `bigint::modular`: modular number theory on `BigUint`: square and `k`-th roots (Tonelli-Shanks, Hensel lifting),
  discrete logarithms (Pohlig-Hellman with baby-step giant-step), the Chinese remainder theorem and binomials
  modulo `n` (Lucas and Granville).
- `bigint::proving`: primality proofs on `BigUint` (BLS75 theorem 5, then an Atkin-Morain ECPP downrun where
  `n - 1` does not factor far enough), MPU's certificate text as a `Certificate` value, and `verify_cert` for
  every proof type MPU reads. Proofs factor `n - 1` and curve orders with a fixed amount of Pollard-Brent work,
  so a proof does not depend on the clock or the load of the host.
- Oracle fixtures recorded from MPU 0.75 on 65 to 1000 bit inputs for the bigint modules, with replay tests.
- `docs/upstream-findings.md`: the places where MPU 0.75 gives a wrong answer, with the correct value this crate gives.
- `docs/mutation-testing.md`: mutation scores for the `factor` module and sampled scores for seven more modules,
  with the survivor triage.
- `docs/bigint-coverage.md` and a `bigint:` field on every line of `STATUS.md`: which of the 428 names have a
  `BigUint` form (82), which are native only in this release (258) and where a `BigUint` form does not apply (88),
  each with its reason.
- Benchmarks for every module, including the bigint path.
- README sections on installation, requirements, the three builds (default, `bigint`, `gmp`) and what works
  with and without `bigint`.

### Changed

- `nth_semiprime` past its table follows MPU's `semi_primes.c` (approximation, interpolation against the exact
  count, block sieve, final walk) instead of a linear scan; `nth_semiprime_approx` applies MPU's inverse
  interpolation.
- The prime count bounds share one long double type.
- CI: a benchmark alert posts a comment and no longer fails the run. The stored results come from other
  shared runners, which differ by more than 10 % on the same code.

### Fixed

- `nth_prime(n)` near the last `u64` prime: `n = MAX_PRIME_IDX` returns the last prime, larger `n` returns
  `None`, and the search bound can no longer overflow.
- `is_power_exp(n)` finds prime exponents from 41 to 61 (for example `2^61`); it stopped at 37 before.

## [1.0.0] - 2026-10-18

### Added

- First release. All 428 exported names of Math::Prime::Util 0.75 are ported on the native `u64` and `u128`
  path, except `is_higgs_prime`, which does not exist in 0.75. See `STATUS.md` for the per-function checklist.

[1.2.0]: https://github.com/justanyone/math-prime-util/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/justanyone/math-prime-util/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/justanyone/math-prime-util/releases/tag/v1.0.0
