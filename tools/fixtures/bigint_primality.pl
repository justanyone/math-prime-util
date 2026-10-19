# Fixture family `bigint_primality`: primality tests on 65 to 1000 bit inputs, for
# the `bigint` feature (src/bigint/primality.rs). Replayed by
# tests/compat_bigint_primality.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1, Math::BigInt). The inputs come from a
# fixed linear congruential generator, so the file is the same on every host.
# The Lucas tests on a 1000-bit input take seconds each in pure Perl, so the
# family stays at a few hundred rows.
use strict; use warnings;

my $prim_lcg = 20_261_018;
# One 32-bit pseudo-random chunk. The product stays below 2^53, so plain doubles are exact.
sub _prim_chunk32 {
  $prim_lcg = ($prim_lcg * 1_664_525 + 1_013_904_223) % 4_294_967_296;
  return $prim_lcg;
}

# A Math::BigInt with exactly $bits bits (top bit set).
sub _prim_big {
  my $bits = shift;
  my $n = Math::BigInt->new(0);
  for (my $have = 0; $have < $bits; $have += 32) {
    $n = ($n << 32) + _prim_chunk32();
  }
  $n = $n % (Math::BigInt->new(1) << $bits);
  return $n | (Math::BigInt->new(1) << ($bits - 1));
}

sub _prim_odd { my $n = _prim_big(shift); return $n | 1; }

sub _prim_pow2 { my $e = shift; return Math::BigInt->new(1) << $e; }

# The whole pseudoprime family on one odd input.
sub _prim_family {
  my ($fh, $n, $lucas) = @_;
  emit($fh, 'is_prime', [$n], sub { is_prime("$n") });
  emit($fh, 'is_prob_prime', [$n], sub { is_prob_prime("$n") });
  emit($fh, 'is_bpsw_prime', [$n], sub { is_bpsw_prime("$n") });
  emit($fh, 'is_strong_pseudoprime', [$n], sub { is_strong_pseudoprime("$n") });
  emit($fh, 'is_strong_pseudoprime', [$n, 2, 3], sub { is_strong_pseudoprime("$n", 2, 3) });
  emit($fh, 'is_strong_pseudoprime', [$n, 325, 9375], sub { is_strong_pseudoprime("$n", 325, 9375) });
  emit($fh, 'is_pseudoprime', [$n, 2], sub { is_pseudoprime("$n", 2) });
  emit($fh, 'is_pseudoprime', [$n, 3, 5], sub { is_pseudoprime("$n", 3, 5) });
  emit($fh, 'is_euler_pseudoprime', [$n, 2], sub { is_euler_pseudoprime("$n", 2) });
  emit($fh, 'is_euler_pseudoprime', [$n, 3, 7], sub { is_euler_pseudoprime("$n", 3, 7) });
  emit($fh, 'is_euler_plumb_pseudoprime', [$n], sub { is_euler_plumb_pseudoprime("$n") });
  return unless $lucas;
  emit($fh, 'is_lucas_pseudoprime', [$n], sub { is_lucas_pseudoprime("$n") });
  emit($fh, 'is_strong_lucas_pseudoprime', [$n], sub { is_strong_lucas_pseudoprime("$n") });
  emit($fh, 'is_extra_strong_lucas_pseudoprime', [$n], sub { is_extra_strong_lucas_pseudoprime("$n") });
  emit($fh, 'is_almost_extra_strong_lucas_pseudoprime', [$n, 1], sub { is_almost_extra_strong_lucas_pseudoprime("$n", 1) });
  emit($fh, 'is_almost_extra_strong_lucas_pseudoprime', [$n, 2], sub { is_almost_extra_strong_lucas_pseudoprime("$n", 2) });
}

sub gen_bigint_primality {
  my $fh = shift;
  my @sizes = (65, 96, 128, 129, 200, 256, 333, 512, 777, 1000);

  # --- random odd inputs, random primes, products of two primes, a square
  my %prime;
  for my $bits (@sizes) {
    my $n = _prim_odd($bits);
    _prim_family($fh, $n, 1);
    my $p = Math::BigInt->new(next_prime("$n"));
    $prime{$bits} = $p;
    _prim_family($fh, $p, 1);
    _prim_family($fh, $p * $p, 1);
  }
  for my $bits (96, 200, 512, 1000) {
    my $q = Math::BigInt->new(next_prime("" . _prim_odd($bits - 65)));
    _prim_family($fh, $prime{65} * $q, 1);
  }
  # 100-digit and larger primes from the generator.
  for my $bits (333, 512, 777, 1000) {
    my $p = Math::BigInt->new(next_prime("" . _prim_odd($bits)));
    _prim_family($fh, $p, 1);
  }
  # --- Mersenne numbers: primes and composites
  for my $e (127, 521, 607) {
    my $m = _prim_pow2($e) - 1;
    _prim_family($fh, $m, 1);
  }
  for my $e (67, 257, 1000) {
    my $m = _prim_pow2($e) - 1;
    _prim_family($fh, $m, 1);
  }
  # --- strong pseudoprimes base 2 above 2^64: composite Mersenne numbers with a
  # prime exponent and the Fermat numbers F_6 to F_9. The BPSW-hard half: the
  # Lucas tests must reject them.
  for my $p (67, 71, 73, 79, 83, 97, 101, 103, 109, 113, 131, 137, 151, 173, 199, 227, 331, 397, 449, 523, 647, 751, 853, 997) {
    _prim_family($fh, _prim_pow2($p) - 1, 1);
  }
  for my $k (6, 7, 8, 9) {
    _prim_family($fh, _prim_pow2(_prim_pow2($k)) + 1, 1);
  }
  # (4^p - 1)/3 for an odd prime p (Cipolla): Fermat pseudoprimes base 2 that
  # are 5 mod 8, so the Euler and strong tests reject them.
  for my $p (37, 61, 127, 257, 331) {
    my $n = (_prim_pow2(2 * $p) - 1) / 3;
    _prim_family($fh, $n, 1);
  }
  # Carmichael numbers of Chernick form (6k+1)(12k+1)(18k+1): Fermat pseudoprimes
  # to every coprime base; a few are strong pseudoprimes base 2 too.
  my ($found, $k) = (0, _prim_big(60));
  while ($found < 4) {
    $k += 1;
    my ($a, $b, $c) = (6 * $k + 1, 12 * $k + 1, 18 * $k + 1);
    next unless is_prime("$a") && is_prime("$b") && is_prime("$c");
    _prim_family($fh, $a * $b * $c, 1);
    $found++;
  }
  # Small Lucas and strong Lucas pseudoprimes, extra strong Lucas pseudoprimes,
  # and strong pseudoprimes base 2 below 2^64: the bigint path must agree.
  for my $n (323, 377, 1159, 1829, 3827, 5459, 5777, 10877, 989, 3239, 27971, 29681,
             2047, 3277, 4033, 4681, 8321, 15841, 29341, 42799, 49141, 561, 1105, 1729,
             4294967297, 18446744073709551557, 18446744073709551615, 9, 25, 121) {
    _prim_family($fh, Math::BigInt->new($n), 1);
  }
  # --- edge inputs: small, even, 2^64 and 2^128 boundaries
  for my $n (0, 1, 2, 3, 4, 5, 7, 11, 13) {
    _prim_family($fh, Math::BigInt->new($n), 1);
  }
  for my $n (_prim_big(100), _prim_big(600) * 2, _prim_pow2(64), _prim_pow2(64) + 13, _prim_pow2(64) - 59,
             _prim_pow2(128), _prim_pow2(128) + 51, _prim_pow2(128) - 159) {
    _prim_family($fh, $n, 1);
  }
  # Increment values on the almost extra strong test.
  for my $n ($prime{128}, $prime{333}, _prim_odd(200), _prim_pow2(521) - 1) {
    for my $inc (3, 16, 149, 256) {
      emit($fh, 'is_almost_extra_strong_lucas_pseudoprime', [$n, $inc],
           sub { is_almost_extra_strong_lucas_pseudoprime("$n", $inc) });
    }
  }
  # Invalid bases die.
  { my $n = $prime{128};
    emit($fh, 'is_strong_pseudoprime', [$n, 1], sub { is_strong_pseudoprime("$n", 1) });
    emit($fh, 'is_strong_pseudoprime', [$n, 0], sub { is_strong_pseudoprime("$n", 0) });
    emit($fh, 'is_pseudoprime', [$n, 1], sub { is_pseudoprime("$n", 1) });
    emit($fh, 'is_euler_pseudoprime', [$n, 1], sub { is_euler_pseudoprime("$n", 1) });
    emit($fh, 'is_strong_pseudoprime', [$n, $n], sub { is_strong_pseudoprime("$n", "$n") });
    emit($fh, 'is_strong_pseudoprime', [$n, $n + 2], sub { my $a = $n + 2; is_strong_pseudoprime("$n", "$a") });
    emit($fh, 'is_strong_pseudoprime', [$n, $n - 1], sub { my $a = $n - 1; is_strong_pseudoprime("$n", "$a") });
    emit($fh, 'is_pseudoprime', [$n, $n * 3 + 1], sub { my $a = $n * 3 + 1; is_pseudoprime("$n", "$a") });
    emit($fh, 'is_euler_pseudoprime', [$n, $n * 3 + 2], sub { my $a = $n * 3 + 2; is_euler_pseudoprime("$n", "$a") });
    emit($fh, 'is_euler_pseudoprime', [$n, $n * 3], sub { my $a = $n * 3; is_euler_pseudoprime("$n", "$a") });
    emit($fh, 'is_almost_extra_strong_lucas_pseudoprime', [$n, 0], sub { is_almost_extra_strong_lucas_pseudoprime("$n", 0) });
    emit($fh, 'is_almost_extra_strong_lucas_pseudoprime', [$n, 257], sub { is_almost_extra_strong_lucas_pseudoprime("$n", 257) });
  }
  # --- lucas_sequence: (n, P, Q, k) -> U_k, V_k, Q^k mod n
  for my $bits (65, 128, 333, 1000) {
    my $n = _prim_odd($bits);
    my $k = _prim_big($bits);
    my $e = _prim_big($bits) * 2;
    for my $pq ([1, -1], [3, -1], [1, 1], [5, 1], [2, 1], [4, 4], [-3, -5], [7, 11], [1, -1], [-$n, 3], [3, $n - 1]) {
      my ($p, $q) = @$pq;
      emit($fh, 'lucas_sequence', [$n, $p, $q, $k], sub { lucas_sequence("$n", "$p", "$q", "$k") });
      emit($fh, 'lucas_sequence', [$e, $p, $q, $k], sub { lucas_sequence("$e", "$p", "$q", "$k") });
    }
    emit($fh, 'lucas_sequence', [$n, 1, -1, $n + 1], sub { my $m = $n + 1; lucas_sequence("$n", 1, -1, "$m") });
    emit($fh, 'lucas_sequence', [$n, 3, 1, 0], sub { lucas_sequence("$n", 3, 1, 0) });
    emit($fh, 'lucas_sequence', [$n, 3, 1, 1], sub { lucas_sequence("$n", 3, 1, 1) });
    emit($fh, 'lucas_sequence', [$n, $k, $e, 65537], sub { lucas_sequence("$n", "$k", "$e", 65537) });
  }
  { my $m = _prim_pow2(127) - 1;
    emit($fh, 'lucas_sequence', [$m, 3, -1, $m], sub { lucas_sequence("$m", 3, -1, "$m") });
    emit($fh, 'lucas_sequence', [$m, 4, 1, $m + 1], sub { my $k = $m + 1; lucas_sequence("$m", 4, 1, "$k") });
    emit($fh, 'lucas_sequence', [1, 3, -1, $m], sub { lucas_sequence(1, 3, -1, "$m") });
    emit($fh, 'lucas_sequence', [0, 3, -1, $m], sub { lucas_sequence(0, 3, -1, "$m") });
    emit($fh, 'lucas_sequence', [2, 3, -1, $m], sub { lucas_sequence(2, 3, -1, "$m") });
    emit($fh, 'lucas_sequence', [6, 1, -1, $m], sub { lucas_sequence(6, 1, -1, "$m") });
  }
  # --- next_prime and prev_prime
  for my $bits (65, 96, 128, 129, 200, 256, 333, 512) {
    my $n = _prim_big($bits);
    emit($fh, 'next_prime', [$n], sub { next_prime("$n") });
    emit($fh, 'prev_prime', [$n], sub { prev_prime("$n") });
    my $p = $prime{$bits};
    emit($fh, 'next_prime', [$p], sub { next_prime("$p") });
    emit($fh, 'prev_prime', [$p], sub { prev_prime("$p") });
  }
  { my $n = _prim_big(1000);
    emit($fh, 'next_prime', [$n], sub { next_prime("$n") });
    emit($fh, 'prev_prime', [$n], sub { prev_prime("$n") });
  }
  for my $e (127, 521) {
    my $m = _prim_pow2($e) - 1;
    emit($fh, 'next_prime', [$m], sub { next_prime("$m") });
    emit($fh, 'prev_prime', [$m], sub { prev_prime("$m") });
  }
  for my $n (0, 1, 2, 3, 4, 11, 12, 13, 30, 31, _prim_pow2(64) - 59, _prim_pow2(64) - 58, _prim_pow2(64) - 1, _prim_pow2(64),
             _prim_pow2(64) + 13, _prim_pow2(64) + 14, _prim_pow2(128) - 159, _prim_pow2(128), _prim_pow2(128) + 51,
             _prim_pow2(128) + 52) {
    my $b = Math::BigInt->new("$n");
    emit($fh, 'next_prime', [$b], sub { next_prime("$b") });
    emit($fh, 'prev_prime', [$b], sub { prev_prime("$b") });
  }
  # --- is_mersenne_prime: the table, small composites, and the GIMPS bound
  for my $p (2, 3, 5, 7, 11, 13, 17, 19, 23, 31, 61, 89, 107, 127, 521, 607, 1279, 2203, 2281, 3217, 4253, 4423,
             9689, 9941, 11213, 19937, 21701, 23209, 44497, 86243, 110503, 132049, 216091, 756839, 859433,
             136279841, 0, 1, 4, 6, 9, 15, 257, 81307408, 100000000, 81307763) {
    emit($fh, 'is_mersenne_prime', [$p], sub { is_mersenne_prime($p) });
  }
}

1;
