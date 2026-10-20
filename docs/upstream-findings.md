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

Counts: confirmed 8, probable 6, API difference or documentation only 8.
Entries 15 to 19 are numbered in the order they were found, not by section.

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

### 18. `is_aks_prime` (PP, `PP.pm`) dies for every input above 2^64 that passes its trivial tests

- Function and input: `is_aks_prime(18446744073709551617)` (2^64 + 1), also
  2^64 + 3 and 2^128 + 1 (`bigint_proving.tsv` lines 268, 269, 271).
- Perl 0.75 value: dies with `toint: 'NaN' is not a valid number`.
  `is_aks_prime(18446744073709551621)` returns 0, but only because the input
  is divisible by 3 and the trivial checks answer first.
- Correct value: 0 for all three (2^64 + 1 = 274177 * 67280421310721,
  2^64 + 3 = 467443687 * 39463029637, 2^128 + 1 is the Fermat number F7 =
  59649589127497217 * 5704689200685129054721).
- Location: `PP.pm` `sub is_aks_prime`, `my $log2n = log($n)/log(2) + 0.0001`.
  For a `Math::BigInt` argument the overloaded `log` returns a `Math::BigInt`
  (the integer part, 44 for 2^64 + 1); dividing it by the float `log(2)` gives
  `NaN`, so `$r0` is `NaN` and `Mtoint($r0)` dies. Every input above
  2^64 that is not a perfect power and not divisible by 2, 3, 5, 7 or 11
  reaches this line, prime or composite.
- How verified: PP run today (`MPU_NO_XS=1`, no GMP). XS `is_aks_prime`
  handles only native integers and hands bigints to PP, so no XS value exists.
- Port: `bigint::proving::is_aks_prime` computes the bit length as a float
  from the integer and returns 0 for the three inputs.
- Test that skips the row: `aks_oracle` in `tests/compat_bigint_proving.rs`
  asserts 0 for the `ERROR` rows instead of requiring a panic.
- Status: confirmed (reachable in 0.75; the dying line is in MPU).

### 19. `allsqrtmod` and `rootmod` (PP, `PP.pm`) on moduli with a factor 2^k, 33 <= k <= 63: an overflow gives wrong roots

- Function and input: `allsqrtmod(17, 2^63)` and `rootmod(17, 2, 2^63)`;
  in the fixture `allsqrtmod(18174344979715097060, 2^65)` and
  `allsqrtmod(509519686070420241709284, 2^65 * 15015)`, where `a = 4 mod 8`
  leaves a square root modulo 2^63 to find (`bigint_modular.tsv` lines 295
  and 439), and `rootmod(a, 2, n)` on the same inputs (lines 294 and 438).
- Perl 0.75 value: `allsqrtmod(17, 2^63)` gives
  `0 405959429219100393 8817412607635675415 9223372036854775808`; 0 and
  2^63 do not square to 17 and 2^63 is not below the modulus.
  `rootmod(17, 2, 2^63)` gives `undef`. Line 295 lists 0, 2^64 twice and
  2^65 among its 8 roots; lines 294 and 438 are `undef`.
- Correct value: `405959429219100393 4205726589208287511 5017645447646488297
  8817412607635675415`, each of which squares to 17 modulo 2^63;
  `rootmod(17, 2, 2^63)` has a root, for example the first of them. The
  fixture rows have the right count of roots, 8 and 64.
- Location: `PP.pm` `sub _allsqrtmodpk`, line 6634,
  `my $q2 = ($q * ($pj - 1)) % $pk;`. With `$q` and `$pk = 2^k` native
  integers the product passes 2^64 for `k >= 33` and becomes a double, so
  `%` returns 0 or 2^(k-1) where it should return `q * (2^(k-1) - 1) mod 2^k`.
  `rootmod` takes the first root of `allrootmod` for a prime power
  (`_rootmod_composite1`), and `allrootmod` with `k = 2` goes through
  `_allsqrtmodfact` to the same line, so the first root, 0, fails the final
  check. Fix: `my $q2 = Mmulmod($q, $pj - 1, $pk);`.
- How verified: PP run today (`MPU_NO_XS=1`, no GMP);
  `allsqrtmod(17, 2^32)` is right, `allsqrtmod(17, 2^63)` is not. XS
  handles moduli below 2^64 in C; a larger modulus, or a residue reduced to
  a native integer inside `_allsqrtmodpk` as on line 295, takes the PP code
  on every build. Not checked against upstream master.
- Port: `bigint::modular::allsqrtmod` and `rootmod` multiply modulo `p^k`
  in `BigUint`.
- Test that skips the rows: `sqrtmod_oracle` (constant `PP_ALLSQRTMOD_WRONG`)
  and `rootmod_oracle` (constant `PP_ROOTMOD_UNDEF`) in
  `tests/compat_bigint_modular.rs`; they check that the port's roots square
  back, are distinct and below the modulus, and include every valid root the
  oracle lists.
- Upstream test: `is_deeply([allsqrtmod(17, "9223372036854775808")], [qw/405959429219100393 4205726589208287511 5017645447646488297 8817412607635675415/], "allsqrtmod 17 mod 2^63");`
- Status: confirmed (reachable in 0.75; the overflowing line is in MPU).

### 20. `semiprime_count` (PP, `PP.pm`) with a range that starts below 2^64 and reaches 2^64 counts nothing past 2^64 - 1

- Function and input: `semiprime_count(18446744073709551610, 18446744073709551622)`
  (`bigint_family_counts.tsv`, the first `semiprime_count` row).
- Perl 0.75 value: `0`.
- Correct value: `2`. The semiprimes in the range are 18446744073709551617 and
  18446744073709551619 (`is_semiprime` is true for both).
- Location: `PP.pm` `sub semiprime_count`, the walk branch (lines 4572 to 4578):
  `validate_integer_nonneg($lo)` leaves a start that fits in 64 bits as a
  native integer, and `$lo++` at 2^64 - 1 becomes a floating point number, so
  every later `Mis_semiprime($lo)` is false. A start of 2^64 or more stays a
  `Math::BigInt` and counts 2.
- How verified: PP run today (`MPU_NO_XS=1`, no GMP); starts at 2^64 - 6 to
  2^64 - 1 give 0, starts at 2^64 and above give 2, 2, 1 and 1 for the shrinking
  ranges (2^64, 2^64 + 1, 2^64 + 2 and 2^64 + 3 as starts). `almost_prime_count(2, ...)` summed over the same window gives 2.
- Port: `bigint::family_counts::semiprime_count` walks with `BigUint`.
- Test that skips the row: the generator `tools/fixtures/bigint_family_counts.pl`
  writes the correct count of 2 for this row (the fixture comment names this
  entry), so `replay_bigint_family_counts` asserts the correct value.
- Upstream test: `is(semiprime_count("18446744073709551610","18446744073709551622"), 2, "semiprime_count across 2^64");`
- Status: confirmed (reachable in 0.75; the walk branch is in MPU).

### 23. `powerful_numbers` (PP, `PP.pm`) dies past 2^64 when a product of two native integers passes 2^64; `nth_powerful(n, 2)` dies for `n >= 2^64`

- Function and input: `powerful_numbers(2^64, 2^64 + 1000000, 3)`,
  `powerful_numbers(2^64, 2^65, 20)`, `powerful_numbers(2^100, 2^100 + 100000, 4)`;
  `nth_powerful(2^64, 2)`.
- Perl 0.75 value: the three lists die with `Parameter '1.84467440737096e+19'
  must be an integer` (and `1.89535253532865e+19`, `1.85304617017301e+19`);
  `nth_powerful` dies with `Parameter 'NaN' must be an integer`. The comment
  lines of `bigint_family_nth.tsv` give the PP result for each fixture interval.
- Correct value: `[2^64]` for the first; the 13 values of the second are in
  `bigint_family_nth.tsv` (2^64, 18953525353286467584 = 2^26 * 3^24, ...).
- Location: `PP.pm` `sub _pcg` lines 2231 and 2236: `$m * Mpowint($_,$r)` and
  `$m * Mpowint($v,$r)` use Perl's `*`, so when both factors are native and
  the product passes 2^64 it becomes a double; the next `Mdivint` or the final
  `Mvecsorti` (line 2264) rejects it. A product that is a `Math::BigInt`
  already (one factor past 2^64) works, which is why
  `powerful_numbers(2^64 - 100, 2^64 + 100, 2)` returns `[2^64]`. Fix:
  `Mmulint($m, Mpowint(...))`. `sub nth_powerful` line 2130 raises a
  `Math::BigInt` index to the power `5/3`, which is `NaN`.
- How verified: run today with XS and no GMP (`MPU_NO_GMP=1`); the same with
  `MPU_NO_XS=1`. The fixture rows for such intervals hold the integers of the
  interval that pass `is_powerful`, or for a wide interval every product of
  prime powers `p^e` (`e >= k`) in it, computed in Perl by the generator.
- Port: `bigint::family_nth::powerful_numbers` multiplies in `u128` or
  `BigUint`; `bigint::family_nth::nth_powerful` searches with the exact count
  and needs no estimate.
- Test that skips the row: none; `bigint_family_nth_oracle` in
  `tests/compat_bigint_family_nth.rs` replays the definition rows, and the
  oracle has no `nth_powerful` row with `k = 2` and `n >= 2^64` (its value is
  near `n^2 / 4.7`, past what an exact count reaches).
- Upstream test: `is_deeply(powerful_numbers("18446744073709551616","18446744073709551616",3), ["18446744073709551616"], "powerful_numbers at 2^64, k=3");`
- Status: confirmed (reachable in 0.75 with any interval past 2^64 whose
  generation multiplies two native values past 2^64).

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
  `binomialmod` (PP) reduces `Mbinomial($n, $k)` and dies the same way for
  every bigint `n` with `k >= 100`: the `binomialmod` rows of
  `bigint_modular.tsv` with `k = 200`, `k = 3000` and `k = n - 3` are
  `ERROR`; `factorial_binomial_oracle` in `tests/compat_bigint_modular.rs`
  asserts the exact binomial reduced mod `m` for them.
- Status: probable (reachable in 0.75 with a current Math::BigInt; the
  failing call lives in Math::BigInt, the missing reflection in MPU).

### 15. `nth_prime_approx` (PP, `PP.pm`) keeps 15 digits of its estimate above 2^46

- Function and input: `nth_prime_approx(425656284035217744)` and every index
  above `10^12` whose estimate is above 2^46 (`bigint_counts.tsv` lines 4 to
  67, every `nth_prime_approx` row).
- Perl 0.75 value: PP gives `18446744070138400000`; the XS build gives
  `18446744070138417152` (`count.tsv` line 991, where XS hands the index to
  the same PP code).
- Correct value: the XS value. Above `10^12` `nth_prime_approx` is
  `toint(inverse_li_nv(n) + 0.5)`. `inverse_li_nv` is a `double`; the XS
  `toint` converts it exactly, the PP `toint` goes through the string form
  of the NV, which Perl prints with 15 significant digits (`PP.pm`
  `sub toint`, the "being very conservative" branch). The two builds also
  differ in the estimate itself: XS evaluates the inverse in `long double`
  (`real.c` `ld_inverse_li`), PP in `double`; the difference is a few ulps
  of the double, visible in the 15th digit for `2^64 - 1` (line 13: PP
  `870566678483616000000`, the long double model `870566678483615350784`).
- How verified: PP run today (`MPU_NO_XS=1`); `Math::BigFloat->new(2**64)`
  prints `18446744073709600000` on the same Perl, which shows the 15-digit
  conversion. The XS value is the fixture row.
- Port: `bigint::counts::nth_prime_approx` follows the XS build (the
  `long double` model up to `10^35`, exact arithmetic rounded to a `double`
  beyond) and returns the whole double.
- Test that skips the rows: `nth_prime_approx_oracle` in
  `tests/compat_bigint_counts.rs` requires agreement to 14 significant
  digits on all 22 rows; `nth_prime_bounds_above_max_prime_idx` in
  `tests/compat_count.rs` replays the XS rows exactly.
- Upstream test: `is(nth_prime_approx("18446744073709551616"), "870566678483615350784", "nth_prime_approx keeps its double");`
  (the pure Perl value today ends in `616000000`).
- Status: probable (the PP `toint` comment calls the conversion deliberate;
  the XS and PP builds still return different values for the same call).

### 16. The PP bounds carry 40 significant digits: trailing noise above 10^38

- Function and input: `nth_prime_lower(2^127)`, `nth_prime_lower` and
  `nth_prime_upper` of `10^40` and `2^200`, `prime_count_upper(10^40)`,
  `prime_count_lower` and `prime_count_upper` of `2^200`
  (`bigint_counts.tsv` lines 59, 62, 63, 65, 66, 117, 119, 120).
- Perl 0.75 value: for example `nth_prime_lower(2^127)` =
  `15573908835133645926997934045400650697606` (41 digits).
- Correct value: the Axler formula in exact arithmetic gives
  `...697605`; `nth_prime_upper(10^40)` is `...786073727` where PP prints
  `...786073686`. Every row agrees with the exact value on its first 40
  significant digits and differs after them.
- Location: `PP.pm` `nth_prime_lower`, `nth_prime_upper`, `prime_count_lower`,
  `prime_count_upper`. The index or count is upgraded to `Math::BigFloat`
  with no accuracy set, so `log` and the divisions run at
  `Math::BigFloat->div_scale`, 40 digits, while the results have 41 to 63.
  `prime_count_approx` sets the accuracy from the length of its input and is
  exact on the same inputs (its rows replay).
- How verified: PP run today; the port's 256-bit fixed point evaluation of
  the same formulas, which replays every other row of the family to the
  last digit.
- Port: `bigint::counts` keeps 77 digits and returns the exact floor or
  ceiling of each bound.
- Test that skips the rows: `nth_prime_bounds_oracle` and
  `prime_count_bounds_oracle` in `tests/compat_bigint_counts.rs` compare
  the three inputs on their leading 36 digits.
- Upstream test: `is(nth_prime_lower(Math::BigInt->new(2)**127), "15573908835133645926997934045400650697605", "nth_prime_lower 2^127 exact to the unit");`
- Status: probable (a precision limit rather than a wrong formula; the
  bounds still hold, as the noise is far below the gap to the true prime).

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

17. `squfof_factor`, `pplus1_factor` and `lehman_factor` (PP, `PP.pm`
    `_stub_factor`) are Pollard-Brent stubs: the method's bound or round
    count becomes the Brent round count and the named method never runs.
    Documented in the PP source ("No implementations of these here"), so
    not a defect; the XS versions run the real methods. `bigint_factor.tsv`
    records the stub's results (`pplus1_factor` and `squfof_factor` rows);
    `stub_method_oracle` in `tests/compat_bigint_factor.rs` checks the
    port's C-method results as valid splits and equal to the oracle where
    both sides split.
21. `setinsert`, `setinvert`, `setcontains` and `set_is_proper_intersection`
    (PP) assume their set arguments are sorted and unique and give
    undefined results otherwise (for example `setinsert` of the unsorted list
    `2^100+3,0,2^100,2^100+3,1` into the empty set returns 5 and an unsorted
    list with a duplicate). The port sorts and de-duplicates first. 53 rows
    of `bigint_sets.tsv` with such an argument are skipped by
    `bigint_sets_oracle` in `tests/compat_bigint_sets.rs`; the rows with
    valid sets all replay.
22. Family bounds above 2^64 (`bigint_family_bounds.tsv`, PP under
    `MPU_NO_GMP=1`). In `PP.pm`: (a) `lucky_count_{approx,lower,upper}`
    above 2^64, `nth_prime_power_*` above about 2^59 and several
    `nth_almost_prime_*` rows die or return `NaN`, because a `Math::BigFloat`
    goes into an integer function; the port runs the search the source
    states and the tests check it by definition. (b) The fitted search range
    of `lucky_count_*` misses the answer near 2^64 (the search then returns
    an end of the range, an invalid bound); the port widens the range.
    (c) `toint` of a double goes through a 15 digit string, so a result of
    16 or more digits ends in zeros; the port copies this where PP does.
    (d) The exponents of `nth_perfect_power_*` are the doubles 1.33333333333333
    and 1.2; the port uses exact rational exponents, so results agree to 15
    digits only. (e) `ramanujan_prime_count_approx` exits its search early
    only if `hi` is a native integer (`Math::BigInt` division never meets
    the 1e-15 test), so the result depends on the type of `hi`; the port
    copies it. (f) Native `inverse_li(10^15)` is off by 44 against the exact
    value; the bigint form is exact.

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
- bigint proving: `tools/gen_fixtures.pl bigint_proving` under `MPU_NO_XS=1`
  (276 rows: `is_provable_prime`, `is_provable_prime_with_cert` and
  `prime_certificate` on 40 inputs of 65 to 664 bits, 144 `verify_prime`
  rows of which 46 accept and 98 reject, 9 `is_aks_prime` rows). Two
  inputs timed out under PP and are recorded as skipped comments. The 40
  `is_provable_prime` rows replay, every PP certificate verifies in the port
  (PP writes BLS5 and Small blocks only; Lucas, BLS3, Pocklington, BLS15,
  ECPP and ECPP3 blocks come from hand-built certificates), and every
  `verify_prime` row replays, including the tampered and malformed texts.
  The 3 `ERROR` rows are entry 18. `compat_bigint_proving` and
  `prop_bigint_proving` pass.
- bigint modular: `tools/gen_fixtures.pl bigint_modular` under
  `MPU_NO_XS=1` (1605 rows on 65 to 500 bit inputs: square and `k`-th
  roots, orders, primitive roots, discrete logarithms, the Chinese remainder
  theorem, factorial and binomial residues, Lucas sequences, the Carmichael
  and quasi-Carmichael tests); every row replays except the 4 of entry 19
  and the 34 `binomialmod` rows of entry 8. Over composite moduli 15
  `sqrtmod` rows and 9 `rootmod` rows give another root than PP; the POD
  allows any root there, and the tests check that the port's root is one.
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

### MPU075_PP_SAFE_INTERNAL: seeded bigint safe prime

MPU 0.75, pure Perl, no GMP: `csrand("bigint-random-fixed-seed-2026");
Math::Prime::Util::PP::random_safe_prime(65)` raises
`random safe prime internal failure`. The check is in
`RandomPrimes.pm::_random_safe_prime_large`. The bigint_random fixture records
ERROR. Its replay skips value equality for this named row and checks the correct
65-bit safe prime form: both p and (p-1)/2 must pass primality tests.
