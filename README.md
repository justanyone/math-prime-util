# math-prime-util

A Rust port of Perl's [Math::Prime::Util](https://metacpan.org/pod/Math::Prime::Util)
(MPU) 0.75: primality tests, prime sieves and counts, integer factoring, and the
number-theory toolbox around them (totients, Möbius, modular roots and logs,
Lucas sequences, continued fractions, combinatorics, random primes, and more).

Status: **version 1.0.0.** All 428 exported names of MPU 0.75 are ported, except
`is_higgs_prime`, which does not exist in 0.75. With the cargo feature `bigint`, 82 of them also
take `BigUint` (see [What works with and without `bigint`](#what-works-with-and-without-bigint)).
See [STATUS.md](STATUS.md) for the per-function checklist and [PLAN.md](PLAN.md) for the phases.
Ongoing work lands on `dev-002`.

## Goals

- **Full compatibility.** Same function names and the same answers as MPU, on
  the native `u64`/`u128` path, and on `BigUint` (cargo feature `bigint`) for the
  names listed in [docs/bigint-coverage.md](docs/bigint-coverage.md). Every MPU test
  file has a Rust counterpart that replays the oracle's output.
- **Performance.** MPU's C algorithms ported as they are, benchmarked against
  the C on the same machine; target within 1.2× of the C.
- **Exceptional test coverage.** Oracle fixtures, property tests, cross-checks
  against MPU's pure-Perl implementation and other Rust crates, a 95 % line
  coverage gate, mutation testing, and Miri on any `unsafe`.

## Testing

`tools/test.sh` runs the suite; `tools/gate.sh` adds fmt, clippy, MSRV and
doc checks. The tests need no Perl: the oracle is committed as
**279,306 rows in 23 fixture files** (`tests/fixtures/*.tsv`, recorded from
MPU 0.75; `grep -vh '^#' tests/fixtures/*.tsv | grep -c .`). The six `bigint_*` files hold MPU's
pure-Perl results on big inputs and run with `cargo test --features bigint`. Where MPU is
wrong the port asserts the correct value and skips the row; see
[docs/upstream-findings.md](docs/upstream-findings.md).

CI gates every pull request on **95 % line coverage**
(`cargo llvm-cov --all-features --fail-under-lines 95`).

Mutation testing uses cargo-mutants 27.1.0: **22,282 candidates** in
22 modules. The `factor` module has a full run (840 mutants, 76 %
killed before survivor closure; the 56 remaining sites were re-run on
2026-10-18 with 45 killed and 11 documented as equivalent or unreachable).
The other seven modules with a run (`arith`, `combinat`, `families`,
`iter`, `modular`, `count`, `random`) are sampled, one shard of about
twenty mutants each: **144 mutants tested, 121 killed, 13 missed, 10
unviable** (90 % of the scored sample); every survivor is triaged and the
one real gap has a test. The nightly workflow runs the whole crate.
Details and the survivor triage:
[docs/mutation-testing.md](docs/mutation-testing.md).

## Branches

Work lands on a numbered integration branch (`dev-001`, `dev-002`, and so on). Small task branches are cut from it
and merged back by pull request. After every 100 merges the integration branch is merged to `main`, deleted, and the
next number starts from `main`. `main` carries releases only. The rotation is `tools/rotate_dev.py` in the SkyBuild kit.

## Milestones

Each line is ticked, with its date, by the milestone row of the build board when every requirement of that milestone is done.
The phases are described in `PLAN.md` section 6.

- [x] Milestone 1: core ported (sieve, primality, Lucas sequences, configuration) (done 2026-10-17)
- [x] Milestone 2: factoring and number theory ported and benchmarked against Perl (done 2026-10-17)
- [ ] Release 0.1.0 published on crates.io
- [ ] Milestone 3: prime counting, nth prime and prime sums ported
- [x] Milestone 4: prime families ported (done 2026-10-19)
- [x] Milestone 5: digits, rationals, combinatorics and inverse sigma ported (done 2026-10-19)
- [x] Milestone 6: vector and set operations ported (done 2026-10-19)
- [x] Milestone 7: iterators and the for-family ported (done 2026-10-19)
- [x] Milestone 8: random primes and the random number generator ported (done 2026-10-19)
- [x] Milestone 9: primality proving, real functions and the bigint backend ported (done 2026-10-19)
- [x] Milestone 10: big integers: `BigUint` forms behind the `bigint` feature, every name audited (done 2026-10-19)
- [x] Release 1.0.0 published on crates.io (2026-10-18)

## Installation and requirements

Add the crate to `Cargo.toml`:

```toml
[dependencies]
math-prime-util = "1"
```

- Rust 1.85 or newer (edition 2024). Nothing else: the default build is pure Rust and links no system library.
  Its only dependency is `getrandom`.
- Feature `bigint` adds the pure-Rust `num-bigint` crate for integers beyond 128 bits. No system library.
- Feature `gmp` adds the `rug` crate, which links the installed GMP and makes `rug::Integer` a third
  `Int` backend. That build needs the GMP development package at build time (`libgmp-dev` on Debian and
  Ubuntu, `gmp-devel` on Fedora, `gmp` from Homebrew); it never compiles GMP from source, so no C compiler,
  `m4` or `make` is needed. Users who do not enable `gmp` never need GMP.

Three builds, then: the default (`cargo build`, pure Rust), `--features bigint` (pure Rust, big integers)
and `--features gmp` (links GMP). To build from a checkout: `cargo build --release`; tests:
`cargo test --all-features` (needs GMP) or `cargo test --features bigint` (pure Rust).

## Minimum Rust version

1.85 (edition 2024).

## Relationship to Math::Prime::Util

This crate is a derivative work: a Rust port of Math::Prime::Util (MPU) 0.75 by Dana Jacobsen. The algorithms, the
function behaviour and the test oracle come from MPU. The Rust code is a new implementation of them. What differs
from MPU:

- The code is Rust (edition 2024), not Perl and C. Functions that MPU answers with `undef` return `Option`.
- MPU handles big integers through Math::BigInt. This crate's native path covers `u64` and `u128`; with the
  cargo feature `bigint`, 82 names also have a `BigUint` form, and the other names stay native only, each
  for a reason given in [docs/bigint-coverage.md](docs/bigint-coverage.md).
- Where the Perl answer is wrong, this crate gives the correct value, skips that fixture row with a named comment,
  and records the difference for upstream review. See
  [upstream findings](docs/upstream-findings.md) and [NOTICE](NOTICE).

MPU is not affiliated with this port and has not endorsed it.

## What works with and without `bigint`

| Build | Integer types | What you get |
|---|---|---|
| default | `u64` and `u128` (`i64`/`i128` where MPU takes signed values) | All 428 names except `is_higgs_prime` (not in MPU 0.75), and the [`Int`] trait on `u64` and `u128`. Pure Rust. |
| `features = ["bigint"]` | adds `num_bigint::BigUint` | All of the above, plus `BigUint` forms of 82 names in `bigint::arith`, `bigint::primality`, `bigint::factor`, `bigint::divisors`, `bigint::modular`, `bigint::counts` and `bigint::proving`, and the [`Int`] trait on `BigUint`. Pure Rust. |
| `features = ["gmp"]` | adds `rug::Integer` | The [`Int`] trait on `rug::Integer` with GMP's arithmetic. Links the installed GMP. Add `bigint` as well for the `BigUint` modules. |

The `u64`/`u128` signatures are the same in every build; the feature only adds functions. The `BigUint` forms
keep the MPU names and take `&BigUint` where the native form takes a fixed-width integer; add `num-bigint = "0.4"` to your own
dependencies to name the type.

```rust
use math_prime_util::{factor, is_prime};

// Native path, every build.
assert_eq!(is_prime(1_000_000_007), 2); // 2: proven prime
assert_eq!(factor(600_851_475_143), vec![71, 839, 1471, 6857]);

// Feature `bigint`: the same names on BigUint, past 2^64.
use math_prime_util::bigint::{factor::factor as big_factor, primality::is_prime as big_is_prime};
use num_bigint::BigUint;

let m89 = (BigUint::from(1_u8) << 89_u32) - 1_u8; // 2^89 - 1, a Mersenne prime
assert_eq!(big_is_prime(&m89), 1); // 1: BPSW probable prime (proofs: bigint::proving)
let f = big_factor(&((BigUint::from(1_u8) << 64_u32) + 1_u8)).unwrap();
assert_eq!(f, [274_177_u64, 67_280_421_310_721].map(BigUint::from));
```

Of the other 346 names, 258 stay native only in this release (`no`: for example the growing results such as
`fibonacci` and `primorial`, the prime family counts, and random big primes) and for 88 a `BigUint` form does
not apply (`n.a.`: num-bigint's own operators already do it, real functions, generator state, enumeration).
[STATUS.md](STATUS.md) gives the value per name; [docs/bigint-coverage.md](docs/bigint-coverage.md) lists the
`BigUint` forms with their fixture rows and every native-only name with its reason.

## Generic integers

The [`Int`] trait gives `is_prime`, `factor`, `powmod`, `gcd` and `next_prime` one surface over `u64`, `u128`
and, with the cargo feature `bigint`, `num_bigint::BigUint`. The native free functions keep their signatures;
the trait methods call them on the native types. Import `math_prime_util::Int`, then write `97_u128.is_prime()`
or `3_u64.powmod(&5, &7)`. Factoring returns `Result` (the error is the unsplit cofactor); modular powers and
next primes return `Option`; primality returns 0, 1 or 2 as the native API does. Above the native range,
`BigUint` primality is the BPSW probable-prime test of `PP.pm::_is_prime7` (so it answers 1, never 2), and
factoring runs the `PP.pm::factor` recipe (trial division, perfect powers, p-1, ECM, HOLF, Pollard-Brent)
with no time limit; `bigint::factor::factor_within` stops at a deadline and returns what it found. The trait also carries the
arithmetic the generic algorithms need: `add_int`, `sub_int`, `mul_int`, `divrem`, `mulmod`, `shl_bits`,
`shr_bits`, `bit_length`, `is_even`, `from_u64`, `to_u64`, `from_decimal` and `to_decimal`. The checked
operations return `None` where a fixed-width type would overflow; `BigUint` never overflows.

Enable the backend with `features = ["bigint"]` on the dependency line; the default build has no dependency on
`num-bigint`. Feature `gmp` adds `rug::Integer` as a third backend with the same algorithms over GMP's arithmetic
(see "Installation and requirements"); GMP's own `gcd` and `next_prime` take `self` by value and win method
resolution on a concrete `Integer`, so write `Int::gcd(&a, &b)` and `Int::next_prime(&n)` for the trait's forms.
The compat tests replay every unsigned fixture row of the families primality, factor, modular, int and sieve
through the trait on each backend and compare with the native path; `benches/bigint.rs` compares the two
arbitrary precision backends on a 200-digit `is_prime` and a 60-digit `factor`.

The free functions of `bigint::arith` (gcd, modular arithmetic, roots, symbols, binomials) and
`bigint::primality` (the pseudoprime tests, BPSW, `lucas_sequence`, `next_prime`, `prev_prime`,
`is_mersenne_prime`), `bigint::factor` (`factor`, `factor_exp`, `trial_factor`, `prho_factor`,
`pbrent_factor`, `pminus1_factor`, `pplus1_factor`, `holf_factor`, `squfof_factor`, `ecm_factor`) and
`bigint::divisors` (`divisors`, `divisor_sum`, `euler_phi`, `carmichael_lambda`, `moebius`,
`is_square_free`, `prime_bigomega`, `prime_omega`) take `&BigUint` and keep the MPU names; the families
`bigint_arith`, `bigint_primality` and `bigint_factor` replay MPU's pure Perl path on 65 to 1000 bit
inputs against them.
`bigint::modular` carries the modular number theory (`sqrtmod`, `allsqrtmod`, `rootmod`, `znorder`,
`znprimroot`, `znlog`, `is_primitive_root`, `chinese`, `factorialmod`, `binomialmod`, `lucasumod`,
`lucasvmod`, `is_carmichael`, `is_quasi_carmichael`; family `bigint_modular`).
`bigint::counts` continues the counts, bounds and estimates, the `n`th prime, the prime stream, `forprimes` and
`is_polygonal` past `u64::MAX` (family `bigint_counts`); the native functions keep returning `None` there.

`bigint::proving` proves primality above 2^64: `is_provable_prime`, `is_provable_prime_with_cert`,
`prime_certificate` and `verify_prime` on `&BigUint`, the BLS75 n-1 proof of `PrimalityProving.pm`, and an
ECPP downrun (Atkin-Morain with the Hilbert class polynomials of the 250 discriminants of class number at most six,
`tools/class_polynomials.py`) where pure Perl MPU gives up, so proofs to 200 digits complete. `Certificate` and `Proof` hold a certificate as a value, print and parse
the MPU text format (every type MPU's `verify_prime` reads, ECPP and the Primo forms included), and derive
`Serialize` and `Deserialize` with the cargo feature `serde` (integers as decimal strings). `is_aks_prime` is
the pure Perl AKS. The family `bigint_proving` replays MPU's proofs and verifications, and the replay test runs
`Math::Prime::Util::verify_prime` on this crate's certificates when a Perl with the vendored MPU tree loads.

[`Int`]: src/bigint/mod.rs

## License

Same terms as MPU and Perl itself: GPL-1.0-or-later OR Artistic-1.0-Perl. You may choose either one. See
[LICENSE](LICENSE) and [NOTICE](NOTICE). Math::Prime::Util is copyright Dana Jacobsen; the Rust port is
copyright Kevin J. Rice and contributors.
