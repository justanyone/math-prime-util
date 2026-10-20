# math-prime-util: full port of Perl's Math::Prime::Util to Rust

Status: plan v1, 2026-10-16. Every phase below is done; 1.2.0 (2026-10-20) is the final release of the port.

## 1. What is being ported

Reference: Math::Prime::Util (MPU) 0.75, released 2026-08-08, by Dana Jacobsen.
Source: https://github.com/danaj/Math-Prime-Util (a shallow clone sits in the
session scratchpad; it will be vendored read-only under `reference/mpu/` as a
vendored copy so every port commit can cite the line it came from).

Measured footprint of the reference:

| Part | Size |
|---|---|
| C sources | 49 files, ~36,000 lines |
| C headers | 63 files |
| XS glue (`XS.xs`) | 270 KB |
| Pure-Perl fallback (`PP.pm`) | 14,300 lines |
| Perl front end (`Util.pm`) | 8,700 lines |
| Exported functions | 428 names |
| Test files (`t/`) | 141 |

MPU has two integer regimes: native (64-bit `UV`, with 128-bit paths in
`factor128.c` and `tinysiqs128.c`) and bigint (Math::BigInt, accelerated by the
separate Math::Prime::Util::GMP module). The Rust port covers both regimes but
ships the native path first. Math::Prime::Util::GMP's own algorithms (ECPP
proofs, large-number factoring) are phase 9, not part of the first release.

## 2. Owner decisions (interview 2026-10-16)

| Topic | Decision |
|---|---|
| License | `GPL-1.0-or-later OR Artistic-1.0-Perl` (matches MPU; faithful port of the C allowed). LICENSE file to be replaced; MIT goes. |
| Crate name | `math-prime-util`; repo renamed to match. |
| First publish | 0.1.0 after phase 2 (primality, sieve, factoring benchmarked). |
| Bigint | u64/u128 first; `num-bigint` feature in phase 9; `rug` after that. |
| MSRV | 1.85, edition 2024. |
| API | Rust-idiomatic with MPU names kept: `Result`/`Option` for croak/undef, iterators for the `for*` family. |
| CI on every push | full compat fixtures + proptests, 95% line coverage gate, OS matrix (linux x86_64 + aarch64, macOS, windows), criterion with a 10% regression alarm. Mutants and Miri nightly. |
| Perf bar | native path within 1.2x of MPU's C; slower functions become issues, not blocks. |
| Workers | SkyBuild board rows per phase/module (project onboarded to the task service, like SkyKeep). |
| Phase 0 | This session builds scaffold, CI, oracle env, fixture pipeline, worker image with rustup; then onboards the project to the task service. |
| Rows | One row per MPU C file (about 45) plus fixture rows, from phase 1 on; each row names the `t/*.t` files it must pass. |
| Worker models | Codex for sizes m and l, Haiku for s (today's SkyBuild setting). |
| Merge gate | cargo nextest (unit, doc, compat) on stable; clippy `-D warnings` and `fmt --check`; 95% line coverage on the touched module; MSRV 1.85 build. |

Done: the crates.io login (1.0.0 published 2026-10-18) and `libgmp-dev` on the build hosts
(2026-10-18, for the GMP backend).

## 3. How Rust distributes libraries (CPAN / PyPI equivalent)

- **crates.io** is the registry. A library is a "crate". `cargo publish`
  uploads it; users add one line to their `Cargo.toml`. Versions are immutable
  once published (yank only, no delete).
- **docs.rs** builds and hosts API docs for every published version
  automatically from the doc comments. No separate step.
- **Semantic versioning** is enforced by the ecosystem. Plan: `0.1.0` after
  phase 2 to claim the name and get the pipeline proven; `0.x` while the API
  moves; `1.0.0` when all 428 functions are ported and the API is frozen.
  Result: 0.1.0 was skipped; 1.0.0 shipped on 2026-10-18, 1.1.0 (add-only `BigUint`
  forms behind the `bigint` feature) on 2026-10-19 and 1.2.0 (`BigUint` forms of 242 more names) on 2026-10-20. Changes per version: `CHANGELOG.md`.

## 4. Toolchain isolation ("virtual environments")

Rust:
- `rustup` is already installed (1.29.1) with only `stable` (1.98.1). Rustup
  toolchains are the virtual environments: each is a separate compiler install
  under `~/.rustup/toolchains/`, selected per command with `cargo +1.85.0 test`
  or per project with `rust-toolchain.toml`.
- Install: `stable`, `beta`, `nightly` (for Miri and coverage), and the pinned
  MSRV (minimum supported Rust version). Proposed MSRV: **1.85** (first
  edition-2024 compiler). `cargo-msrv` verifies it.
- Dependencies are already isolated per project by `Cargo.lock` and `target/`.
- Cargo dev tools to install under `~/.cargo/bin`: `cargo-nextest`,
  `cargo-llvm-cov`, `cargo-mutants`, `cargo-msrv`, `cargo-semver-checks`,
  `cargo-deny`, `cargo-criterion`.

Perl (the oracle):
- Bootstrap `cpanm` + `local::lib` into `.perl-local/` inside the repo, no sudo.
  Install Math::Prime::Util 0.75, Math::BigInt, bignum, Test::More there. Add
  `Math::Prime::Util::GMP` once libgmp-dev exists.
- `tools/perl-env.sh` sets `PERL5LIB`/`PATH` for one shell. The Rust test
  suite never calls Perl: Perl only generates fixture files that are committed.

Multiple Perl versions are not needed: MPU's behaviour does not depend on the
Perl version, and the oracle output is frozen into fixtures.

## 5. Crate layout

```
math-prime-util/
  Cargo.toml              one crate, feature flags below
  src/
    lib.rs                re-exports; one module per MPU C file family
    int/                  u64 and u128 primitives: mulmod, montmath, bits, isqrt, iroot, ilog
    sieve/                segmented sieve, wheel, prime_precalc cache, sieve_range, clusters
    primality/            MR, BPSW, Lucas variants, Frobenius, AKS, Perrin, Catalan, Mersenne
    count/                prime_count (LMO, Lehmer, Legendre phi, tables), nth_prime, bounds
    factor/               trial, rho, brent, p-1, p+1, SQUFOF, HOLF, Lehman, Fermat, ECM, SIQS-128
    arith/                totients, moebius, mertens, divisor sums, liouville, mangoldt, omega
    families/             twin, semi, almost, omega, ramanujan, lucky, powerful, powerfree, perfect powers, prime powers
    modular/              addmod..powmod, invmod, sqrtmod/rootmod (all roots), znlog, znorder, primroot, factorialmod, binomialmod, lucas mod
    combinat/             binomial, stirling, bell, partitions, factorials, powersum, intcomplexity
    digits/               todigits, fromdigits, zeckendorf, harshad, palindromes
    rational/             contfrac, convergents, bestrational, farey, Calkin-Wilf, Stern-Brocot, cornacchia
    vec/                  vecsum..vecwindow, set ops, sidon/sumfree
    iter/                 forprimes family as iterators and closures; prime_iterator
    random/               ChaCha20 CSPRNG, irand*, urandom*, random_*_prime, Maurer, Shawe-Taylor
    proving/              is_provable_prime, certificates, verify_prime (BLS75 n-1, ECPP verify)
    real/                 Ei, li, RiemannZeta, RiemannR, LambertW, Pi, bernfrac/bernreal, harmonic
    bigint/               trait Int over u64/u128/num-bigint (feature), rug (feature)
  tests/
    fixtures/             oracle output, one file per MPU test family, generated by tools/gen_fixtures.pl
    compat_*.rs           one test file per MPU t/*.t, same name, same cases
  benches/                criterion; mirrors MPU bench/ and primal/num-prime baselines
  tools/                  perl-env.sh, gen_fixtures.pl, bench_vs_perl.pl, coverage.sh
  reference/mpu/          vendored copy, read-only, MPU 0.75 (commit 581c217)
  PLAN.md  STATUS.md      STATUS.md = the 428-name checklist, one line each
```

API shape:
- Native functions take and return `u64` (and `u128` where MPU has 128-bit
  paths). Errors are `Result<_, Error>` where MPU would `croak`; MPU's
  "returns undef" becomes `Option`.
- The `Int` trait gives one generic surface across `u64`, `u128`, and the
  bigint backends, so `is_prime(&big)` works when the `bigint` feature is on.
- The bigint API is additive. Functions live in the `bigint` module under
  their MPU names and take `&BigUint` (or `T: Int`). Native functions keep
  their `u64`/`u128` signatures. Everything sits behind feature `bigint`.
  Tests check that the bigint path agrees with the native path on the `u64` range.
- The `for*` family becomes iterators (`primes_in(lo..=hi)`) plus
  closure-taking functions named like MPU for people porting Perl code.
- Feature flags: `std` (default), `bigint` (num-bigint, pure Rust), `gmp`
  (rug), `rand` (random module), `serde` (certificates).

## 6. Phases, in dependency order

Each phase: port C to Rust side by side, write the fixture generator for the
matching MPU test files, reach the coverage gate, bench against the C, then
tick the names in STATUS.md. A phase is done when its MPU `t/` files all pass
through the Rust compat tests.

| # | Scope (MPU C files) | MPU test files | Functions (approx) |
|---|---|---|---|
| 0 | Scaffold: Cargo.toml, CI, toolchains, Perl oracle env, fixture generator, STATUS.md checklist, license file | 01..04 | 0 |
| 1 | `util.c` int core, `mulmod.h`, `montmath.h`, `util_bits.h`, `sieve.c`, `primality.c`, `lucas_seq.c`, `cache.c` | 10, 11-primes, 12, 15, 17, 18-*, 25 | ~80 |
| 2 | `factor.c`, `factor128.c`, `tinysiqs128.c`, `totients.c`, `moebius.c`, divisors, gcd/lcm/chinese, `rootmod.c`, `znlog.c`, `factmod.c` | 19-*, 20-*, 21, 26-modops, 26-rootmod, 50, 51 | ~90 |
| → | **publish 0.1.0** | | |
| 3 | `prime_counts.c`, `lmo.c`, `lehmer.c`, `legendre_phi.c`, `prime_count_cache.c`, `prime_sums.c`, nth_prime, bounds | 13, 14, 11-sumprimes | ~25 |
| 4 | Families: `twin_primes.c`, `semi_primes.c`, `almost_primes.c`, `omega_primes.c`, `ramanujan_primes.c`, `prime_powers.c`, `perfect_powers.c`, `powerful.c`, `powerfree.c`, `lucky_numbers.c`, `sieve_cluster.c`, `goldbach.c`, chen/safe primes | 11-*, 26-lucky, 26-perfectpowers, 26-powerful, 26-powerfree, 26-chenprimes, 26-goldbach | ~90 |
| 5 | `strint.c` digits, `rational.c`, `congruent_numbers.c`, `inverse_sigma0.c`, `inverse_interpolate.c`, combinatorics, partitions, `intcomplexity.c` | 24, 26-* remainder, 27 | ~80 |
| 6 | `sort.c`, `xs_set.c`, `ds_iset`, vec*/set* ops | 26-vec, 26-setops, 26-setbinop, 26-sumset, 26-mex | ~60 |
| 7 | Iterators and `for*` family, `lastfor`, prime_iterator, PrimeArray | 32-* | ~20 |
| 8 | `chacha.c`, `csprng.c`, `entropy.c`, `random_prime.c`, RandomPrimes.pm (Maurer, Shawe-Taylor, safe/strong) | 16, 34, 35-* | ~25 |
| 9 | `aks.c`, PrimalityProving.pm, certificates; `real.c` and ZetaBigFloat; bigint backends; MPU::GMP algorithms | 22, 23-*, 28, 29, 53, 70, 81 | ~20 + bigint coverage of all above |
| → | **publish 1.0.0** when STATUS.md shows 428/428 (done 2026-10-18) | | |
| 10 | Bigint coverage: `BigUint` forms of 82 names in `bigint::{arith, primality, factor, divisors, modular, counts, proving}`, audit of all 428 names (`docs/bigint-coverage.md`) | bigint fixtures (MPU pure-Perl path) | 82 |
| → | **publish 1.1.0** (done 2026-10-19) | | |
| 11 | `BigUint` forms of the names that were native only in 1.1.0 (board rows `MPU-BIGINT2-*`): 242 ported; for the other 104 a `BigUint` form does not apply, each with a reason; none stays native only | bigint fixtures | 242 |
| → | **publish 1.2.0** (done 2026-10-20) | | |

Threading: MPU's C uses pthread locks only around its shared sieve cache. The
Rust port uses `OnceLock`/`Mutex` for the same cache and is `Send + Sync`
everywhere; `t/31-threading.t` maps to a Rust test under `loom` or plain
threads.

## 7. Test strategy ("exceptionally good coverage")

Five layers, all run in CI on every push:

1. **Compat fixtures.** `tools/gen_fixtures.pl` runs MPU 0.75 on the exact
   inputs from each `t/*.t` plus extra edge inputs (0, 1, 2, 2^32±1, 2^63±1,
   2^64-1, u128 edges, Carmichael numbers, strong pseudoprimes to the MR bases
   used, known factoring hard cases). Output is committed as text under
   `tests/fixtures/`. One Rust compat test per MPU test file asserts equality.
   This is the compatibility spec; the Rust suite needs no Perl at test time.
2. **Property tests** (`proptest`): algebraic laws that must hold for any
   input: `factor(n)` multiplies back to `n` with each factor `is_prime`;
   `prime_count(nth_prime(k)) == k`; `invmod(a,m)*a ≡ 1`; `sqrtmod` squares
   back; `moebius`/`euler_phi` Dirichlet identities; `next_prime(prev_prime(p)) == p`;
   bigint path agrees with native path on the u64 range.
3. **Cross-implementation checks.** For every function that both the C (XS)
   and the pure-Perl `PP.pm` implement, fixtures are generated from both; a
   disagreement is reported upstream, not papered over. `primal` and
   `num-prime` are used as a third opinion for `is_prime`, sieves and counts
   in a dev-dependency-only test.
4. **Coverage gate.** `cargo llvm-cov` with a line threshold of 95% and
   branch threshold of 90% per module, enforced in CI; the README carries the
   badge. `cargo mutants` runs nightly on the changed modules; surviving
   mutants are tracked as issues.
5. **Unsafe audit.** Target is zero `unsafe` outside `int/` (128-bit
   intrinsics). Everything that is `unsafe` runs under Miri in CI.

Also: `cargo clippy -D warnings`, `cargo fmt --check`, `cargo deny` (license
and advisory audit of dependencies), `cargo semver-checks` on release PRs,
doc tests for every public function (the example in each doc comment is a
test), and `cargo msrv verify`.

CI matrix (GitHub Actions): `{MSRV, stable, beta, nightly}` × `{linux x86_64,
linux aarch64, macOS, windows}`; nightly job adds Miri, coverage and mutants.

## 8. Performance strategy

- Port the C algorithms as they are: the point of MPU is that its algorithms
  are already the fast ones (LMO prime count, SQUFOF/rho/ECM/SIQS factoring,
  Montgomery mulmod, wheel sieve). Faithful port first, Rust-idiom cleanup
  second, with the benchmark deciding.
- `benches/` under criterion mirrors MPU's `bench/` scripts. `tools/bench_vs_perl.pl`
  runs the same workload through the Perl XS build on the same box so every
  bench report shows Rust-vs-C ratio. Acceptance per phase: native-path
  functions within 1.2× of the C; any slower function gets an issue.
- Baselines against `primal` 0.3.3 (sieve) and `num-prime` 0.7.0 (primality,
  factoring) to show where the port stands in the Rust ecosystem.
- Candidate permissive-licensed sources for the hottest paths if MPU's C is
  ever bested: primesieve and primecount (both BSD-2-Clause, C++), the
  fastest known sieve and prime-count implementations. Their Cython bindings
  (`primesieve`, `primecountpy` on PyPI) are wrappers only and carry no extra
  algorithms. Decide per function on the bench numbers, not up front.
- Portability: `u128` arithmetic is native in Rust, so MPU's `#ifdef` 128-bit
  branches become unconditional. No inline assembly in v1; a later `asm`
  feature can add the x86-64 `mulx`/`adx` paths behind a flag.

## 9. First work items (phase 0), in order

1. Replace LICENSE with the Perl dual license text; set `license` in Cargo.toml.
2. `cargo init --lib`, Cargo.toml metadata, `rust-toolchain.toml`, CI workflow,
   README with the goal and the compat promise, STATUS.md with 428 lines.
3. `rustup toolchain install 1.85.0 beta nightly`; `cargo install` the dev tools.
4. Perl oracle env under `.perl-local/`; MPU 0.75 installed; `tools/gen_fixtures.pl`
   producing fixtures for `t/10-isprime.t` as the proof of the pipeline.
5. Vendor MPU 0.75 as `reference/mpu` (a plain copy, not a submodule: workers are offline).
6. First real code: `int/` (mulmod, montmath, isqrt, ilog) with fixtures,
   proptests and benches, as the pattern every later module copies.
