# Upstream findings: Math::Prime::Util 0.75

Differences from Math::Prime::Util (MPU) 0.75 found while porting it to Rust.
One entry per finding. Fixture references are line numbers in
`tests/fixtures/<family>.tsv`, which hold the recorded output of MPU 0.75 XS
(see [NOTICE](../NOTICE)). Where MPU is wrong, the port returns the correct
value and the named test skips the fixture row with a comment.

Evidence limits: no Perl XS build of MPU exists on the port's build hosts. XS
values are the frozen fixtures plus a reading of the C source in
`reference/mpu/`. Pure-Perl (PP) values come from running the vendored 0.75
tree with `MPU_NO_XS=1`. Entries 1 to 3 were checked against upstream master
(`205c790`, 2026-10-17): the code at issue is unchanged there.

Counts: confirmed 4, probable 4, API difference or documentation only 7.

## Confirmed

### 1. `is_smooth` (XS, `util.c`) returns 0 when the cofactor is a prime at most `k`

- Function and input: `is_smooth(4294967297, 4294967296)`; 4294967297 = 641 x 6700417.
- Perl 0.75 value: `0` (`factor.tsv` line 24083).
- Correct value: `1`. Every prime factor is at most `k`; here `n <= k` holds as well.
- Location: `util.c` 0.75 line 3045 (master line 2817): `if (is_prime(n)) return 0;`
  after the small-prime loop. The `n < pn*pn` guard before it only covers
  `n <= k`. A prime remainder above 1000 but at most `k` must give 1. Fix:
  `if (is_prime(n)) return n <= k;`.
- How verified: PP returns 1 for this input and for `is_smooth(7*1299709, 1299709)`.
  A Python port of the C function reproduces the 0. A full factorization of
  all 941 native `is_smooth` fixture rows (Miller-Rabin plus Pollard rho,
  independent of this crate) finds line 24083 the only wrong row.
- Port: corrected in `src/factor/is_smooth.rs`.
- Test that skips the row: `is_smooth_oracle` in `tests/compat_factor.rs`
  (line 24083); the test asserts the correct value directly.
- Upstream test: `is(is_smooth(4294967297, 4294967296), 1, "is_smooth: n<=k with large prime cofactor");`

### 2. `abundance` (XS, `XS.xs`) wraps results below -2^63

- Function and input: `abundance(18446744073709551557)` (a prime).
- Perl 0.75 value: `60` (`arith.tsv` line 8419).
- Correct value: `-18446744073709551556` (the aliquot sum of a prime is 1).
  60 = 2^64 - 18446744073709551556, the wrapped value. The port returns `None`
  because the value does not fit `i64`.
- Location: `XS.xs` 0.75 line 4458 (master 4520): `XSRETURN_IV((IV)(sum-n));`.
- How verified: PP returns -18446744073709551556. The XS and PP backends disagree.
  The other large fixture rows (`arith.tsv` lines for 2^64-2, 2^64-1, 2^63-25)
  have differences inside `i64` and are unchanged.
- Port: corrected in `src/arith/abundance.rs`.
- Test that skips the row: `abundance` in `tests/compat_arith.rs` (line 8419);
  the test asserts `None` directly.
- Upstream test: `is(abundance("18446744073709551557"), "-18446744073709551556", "abundance large prime");`

### 3. `is_power(n, k)` (PP, `PP.pm`): the `@rootmask` table rejects true k-th powers

- Function and input: `is_power(33**13, 13)` with a bigint `n`; also `is_power(1, 15)`.
- Perl 0.75 value: PP gives `0` for both. XS gives `1` for `is_power(1, 15)`
  (`int.tsv` line 960). The XS path falls back to PP for bigint `n`, so bigints
  are affected on every build.
- Correct value: `1` (exact integer arithmetic).
- Location: `lib/Math/Prime/Util/PP.pm` 0.75 line 7557 (master 7799),
  `return 0 if $n32 & $rootmask[$a]`. Bit `i` set means residue `i` mod 32 is
  impossible for a k-th power. Computing `r^a mod 32` for all `r` shows the
  table marks achievable residues impossible for `a` = 13, 15, 17, 18, 19,
  21-23, 25-31 and 33-39 (for `a` = 15 the residues 1, 7, 9, 19, 21, 29 and
  31 are wrongly excluded; residue 1 is wrong for most listed `a`).
- How verified: `MPU_NO_XS=1 perl -Ireference/mpu/lib -MMath::Prime::Util=:all -MMath::BigInt -e 'print is_power(Math::BigInt->new(33)->bpow(13), 13)'`
  prints 0; without `k` it prints 13; `is_power(1, 15)`, `is_power(1, 31)` and
  `is_power(1, 33)` print 0 in PP. Same for 65^13 and 97^15.
- Port: not affected. The native fixtures come from XS and are correct;
  bigint rows are outside the native API.
- Test that skips the row: none needed. `is_power_oracle` in
  `tests/compat_int.rs` checks line 960 against the correct XS value.
- Upstream test: `is(is_power(Math::BigInt->new(33)**13, 13), 1, "is_power 33^13 k=13");`

### 4. `ramanujan_primes` and `ramanujan_prime_count` (PP) are wrong for bounds 2 and 3

- Function and input: `ramanujan_primes(2)`, `ramanujan_primes(3)`,
  `ramanujan_prime_count(2)`, `ramanujan_prime_count(3)`.
- Perl 0.75 value: XS gives `2`, `2`, `1`, `1` (`families.tsv` lines 1242,
  2661, 2662). PP gives an empty list, `2 -1`, `0` and `2`.
- Correct value: the XS values. The first Ramanujan prime is 2 and the second
  is 11, so both lists are `2` and both counts are 1.
- How verified: PP run today (`MPU_NO_XS=1`); `nth_ramanujan_prime(1..5)` in
  PP gives 2 11 17 29 41, which contradicts its own count. Found by the
  per-row PP check of `families.tsv` described below.
- Port: not affected; `ramanujan_primes_oracle` and
  `ramanujan_prime_count_oracle` in `tests/compat_families.rs` pass on the XS
  rows with no skip.
- Upstream test: `is(ramanujan_prime_count(2), 1); is(ramanujan_prime_count(3), 1); is_deeply(ramanujan_primes(3), [2]);`

## Probable

### 5. `vecuniq`, `vecmax` (XS) canonicalise numeric strings; the POD and PP keep them

- Function and input: `vecuniq("2", "02", "2", "02")`, `vecmax("2", "02", "2", "02")`.
- Perl 0.75 value: XS `vecuniq` gives `2` (`vec.tsv` line 799) and `vecmax`
  gives `02` (line 797). PP gives `2 02` and `2`. `vecmin` (line 796) gives `2`
  on both.
- Correct value: the author's call. The POD for `vecuniq` says `2` and `02` are
  distinct values, which supports PP.
- How verified: PP run today. The XS values are the fixture rows.
- Port: numeric only; string spellings are not representable.
- Test that skips the rows: `vecuniq`, `vecmax` and `vecmin` in
  `tests/compat_vec.rs` skip rows whose scalars have a leading zero (comment at
  the `vecmin, vecmax, vecuniq, and vecsingleton` guard).
- Upstream test: `is_deeply([vecuniq("2","02","2","02")], ["2","02"], "vecuniq string semantics");`

### 6. `is_pillai` POD wording

- Function: `is_pillai`. The POD describes the least `v` with `v! % n == n-1`
  and `n % v != 1`; the wording was flagged against the behaviour in an
  earlier pass and could not be reproduced here. Documentation only, low
  priority.
- Perl 0.75 value: XS and PP agree on the fixture rows checked today
  (`combinat.tsv`, e.g. `is_pillai(23)` = 14, `is_pillai(29)` = 18,
  `is_pillai(59)` = 15).
- Correct value: no numeric correction proposed.
- Test that skips the row: none; `is_pillai_oracle` in
  `tests/compat_combinat.rs` passes all 1002 rows.

### 7. PP and XS disagree on the zero bounds of `semi_primes`, `semiprime_count` and `almost_prime_count`

- Function and input: `semi_primes(0)`, `semiprime_count(0)`, `almost_prime_count(0, 0)`.
- Perl 0.75 value: XS gives an empty list, `0` and `0` (`families.tsv` lines
  177, 250, 372). PP dies (`Illegal division by zero` in `PP.pm` line 4572 for
  the count, `must be a positive integer` for the list) and returns an empty
  value for the almost-prime count.
- Correct value: the XS values (nothing is at most 0).
- How verified: PP run today; found by the per-row PP check below.
- Port: follows XS; `semi_primes_oracle`, `semiprime_count_oracle` and
  `almost_prime_count_oracle` in `tests/compat_families.rs` pass with no skip.
- Status: probable (an undocumented PP-only edge). `numtoperm(0, 0)` is a
  similar edge: XS returns an empty list, PP returns undef (`combinat.tsv`
  line 18311).

### 8. `binomial` (PP, `PP.pm`) dies for a bigint `n` with `k >= 100`

- Function and input: `binomial(n, n - 20)` with the 100-bit
  `n = 1190998462705547155062215048452`; also `binomial(2^70, 100)`.
- Perl 0.75 value: dies with `Can't locate object method "_nok" via package
  "Math::BigInt"` (`bigint_arith.tsv` line 737). `binomial(2^70, 99)` works.
- Correct value: `binomial(n, 20)`, since `C(n, k) = C(n, n - k)`.
- Location: `PP.pm` `sub binomial`, step 4. For `n > INTMAX` and `k < 100` it
  divides the falling factorial by `k!`; every other bigint case calls
  `Math::BigInt::bnok("$n", "$k")`, which fails with Math::BigInt 2.003002
  (Perl 5.40). The source carries `TODO: consider reflection for large k`, so
  the `k -> n - k` reflection that would avoid the call is known missing.
- How verified: PP run today (`MPU_NO_XS=1`, no GMP). No XS value exists:
  XS `binomial` returns the bigint result through the PP path as well.
- Port: `bigint::arith::binomial` reflects `k` first and loops over the
  smaller count.
- Test that skips the row: `combinatorics_oracle` in
  `tests/compat_bigint_arith.rs` asserts the falling-factorial value instead.
- Status: probable (reachable in 0.75 with a current Math::BigInt; the
  failing call lives in Math::BigInt, the missing reflection in MPU).

## API difference or documentation only (not a bug)

8. Results above u64 are `undef` in the fixtures (the generator's
   `u64_or_undef`) and `None` in the port. `Case::check` in
   `tests/common/mod.rs` maps `None` to `undef`; `binomial` rows in
   `combinat.tsv` are an example.
9. MPU dies on bad input; the port panics or returns `Result`/`None`. The
   fixtures record `ERROR`; `each_may_panic` in `tests/common/mod.rs`
   requires a panic where the oracle died.
10. Signed and bigint rows are skipped by the native u64/u128 API, with a
    comment per test (for example `tests/compat_factor.rs`,
    `tests/compat_digits.rs`, `tests/compat_combinat.rs`). The fixture values
    are kept for the future bigint API.
11. Real-valued functions use `f64` where MPU uses C `long double`: compared
    with a relative tolerance (`tests/compat_real.rs`, tolerance 1e-12;
    `chebyshev_oracle` in `tests/compat_count.rs`).
12. `vecfreq` output order follows Perl hash order and differs between runs;
    not a defect.
13. PP-only oddities, not claimed as defects: `digital_root("-0")` dies in PP
    (`Parameter '-0' must be a non-negative integer`) while XS gives 0
    (`digits.tsv` line 36802). An earlier PP regeneration showed
    `prime_signature_scalar(24739954287740860)` as 46 against XS 4620
    (`factor.tsv` line 1223); a direct PP call today returns 4620, so that
    difference is withdrawn.
14. `is_euler_plumb_pseudoprime` (PP) returns the empty string (Perl false)
    instead of 0 when `2^((n-1)/2^e)` is 1 or `n - 1` but `n mod 8` does not
    match: the comparison result is returned as is. XS returns 0. Five
    composites in `bigint_primality.tsv` (lines 1164, 1180, 1196, 1212, 1228);
    `base_tests_oracle` in `tests/compat_bigint_primality.rs` asserts false
    for an empty oracle value.

## Checked, no defect found

Method: the compatibility tests (`tests/compat_*.rs`) pass on every fixture
row, so the port and XS 0.75 agree on the native domain. On top of that, PP
0.75 was compared with the XS fixtures: an earlier regeneration covered int,
digits, lucas, sieve, arith and vec (factor and count were incomplete), and the
areas left open there were checked today as follows.

- rational: `tools/gen_fixtures.pl rational` under `MPU_NO_XS=1` regenerated
  all 3156 rows; they equal the committed XS fixture byte for byte.
  `compat_rational` and `prop_rational` pass.
- combinat: a per-row PP check of all 18361 `combinat.tsv` rows (3 s per call)
  gave 82 mismatches and 4 timeouts. The mismatches are the `binomial` rows
  capped at u64 by the generator (13), `is_congruent_number` rows where the
  generator maps a false value to 0 (58), the `permtonum` rows called with a
  flat list instead of an array reference by the checker (10), and
  `numtoperm(0, 0)` (entry 7). Timeouts, not checked under PP:
  `integer_complexity(1000000)`, `integer_complexity(9999999)`,
  `is_congruent_number(58654083)`, `hclassno(4294967295)`.
  `compat_combinat` and `prop_combinat` pass.
- families: a per-row PP check of all 15793 `families.tsv` rows (3 s per
  call): 14426 equal, 1135 mismatches, 87 timeouts, 5 rows skipped
  (`is_prime_power_root`, the two-argument `is_prime_power(n, \$root)` form).
  1128 of the mismatches are in estimate functions (`*_approx`, `*_lower`,
  `*_upper` of the lucky, Ramanujan, perfect-power, prime-power, almost-prime,
  semiprime and twin-prime families), where PP and XS use different estimate
  formulas by design; the fixtures and the port follow XS and no bound is
  claimed wrong. The other 7 rows are entries 4 and 7 above. The 87 timeouts
  (58 `nth_powerful`, then `powerful_count`, `is_ramanujan_prime`,
  `semi_primes`, `ramanujan_prime_count`, `twin_prime_count`, `omega_primes`,
  `nth_ramanujan_prime` and single rows of 9 other functions) were not checked
  under PP. `is_gaussian_prime` (373 rows) and `minimal_goldbach_pair` are
  included and agree. `compat_families` and `prop_families` pass.
- bigint primality: `tools/gen_fixtures.pl bigint_primality` under
  `MPU_NO_XS=1` (2310 rows on 65 to 1000 bit inputs, 128 inputs through the
  whole pseudoprime family, 110 `lucas_sequence` rows, 78 prime walks, 46
  Mersenne exponents); every row replays except the 5 of entry 14. The 7
  `ERROR` rows (bases 0 and 1, increments 0 and 257, modulus 0) panic on the
  Rust side as the native functions do.
- Earlier pass (unchanged): int (`sqrtint`, `rootint`, `powint`, `is_square`,
  `logint`, `gcd`, `lcm`, `binomial`, `valuation`, `divint` family); arith
  (totients, `moebius`, `mertens` to 200000, `sigma`, `divisor_sum`,
  `aliquot_sum`); factor (`factor`, `factor_exp`, `divisors`, `omega`,
  `prime_signature`); primality (`is_prime`, `is_prime_power`,
  `is_semiprime`, `next_prime`, `prev_prime`); sieve and count (`primes`,
  `prime_count`, `sum_primes`, twin and cluster counts); digits; lucas and
  fibonacci sequences; vec functions other than entry 5.

Not checked against PP: `config.tsv`, `ramanujan_tau` above 300, and the
rows listed as timeouts above.
