# Fixture family `bigint_family_counts`: counts, sums and nth values beyond the
# 64-bit range, for the `bigint` feature (src/bigint/family_counts.rs).
# Replayed by tests/compat_bigint_family_counts.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1 and MPU_NO_GMP=1, Math::BigInt;
# Math::Prime::Util::GMP is not installed). The generator refuses to run when
# the GMP backend is on.
#
# Range rows (low, high inclusive) use the MPU range call where the pure-Perl
# path returns in practical time (prime_count, twin_prime_count and
# semiprime_count). The PP range forms of the other names time out or exhaust
# memory above 2^64, so each such row sums the MPU per-integer function
# (euler_phi, moebius, liouville, prime_omega, prime_bigomega, is_powerful,
# is_prime_power, is_prime) over the same window, which is what the range form
# is defined to compute.
#
# Names kept native (no BigUint form), measured above 2^64 with the PP path:
# goldbach_pair_count, lucky_count, nth_lucky, nth_almost_prime,
# nth_omega_prime, nth_semiprime, nth_twin_prime, nth_ramanujan_prime and
# nth_prime_power need a sieve or a prime count over a range proportional to
# the input (time out after 20 s or reject the input), and ramanujan_prime_count
# exhausts memory.
use strict;
use warnings;
sub gen_bigint_family_counts {
  my ($fh) = @_;
  die 'This family requires MPU_NO_XS=1' unless $ENV{MPU_NO_XS};
  die 'This family requires MPU_NO_GMP=1' unless $ENV{MPU_NO_GMP};
  die 'GMP backend is on' if prime_get_config()->{gmp};
  my $two64 = Math::BigInt->new('18446744073709551616');
  for my $v ('18446744073709551616', '36893488147419103232') {
    my $n = Math::BigInt->new($v);
    emit($fh, 'perfect_power_count', [$n], sub { perfect_power_count($n) });
    for my $k (0, 1, 2, 3, 65) {
      for my $fn (qw(powerfree_count powerfree_sum powerfree_part_sum)) {
        next if $k >= 2 && $k < 65;
        no strict 'refs';
        emit($fh, $fn, [$n, $k], sub { &{$fn}($n, $k) });
      }
    }
    for my $a (0, 1, 3, 6) {
      emit($fh, 'legendre_phi', [$n, $a], sub { legendre_phi($n, $a) });
    }
    for my $k (0, 2, 3, 5) {
      emit($fh, 'smooth_count', [$n, $k], sub { smooth_count($n, $k) });
      emit($fh, 'rough_count', [$n, $k], sub { rough_count($n, $k) });
    }
  }
  # nth values: MPU bisects on the exact counts.
  for my $v ('1', '2', '7', '1000', '18446744073709551616', '18446744073709551617', '100000000000000000000') {
    my $n = Math::BigInt->new($v);
    emit($fh, 'nth_perfect_power', [$n], sub { nth_perfect_power($n) });
  }
  for my $v ('1', '7', '18446744073709551616', '100000000000000000000') {
    my $n = Math::BigInt->new($v);
    for my $k (10, 20, 30, 65) {
      emit($fh, 'nth_powerfree', [$n, $k], sub { nth_powerfree($n, $k) });
    }
  }
  # Windows of consecutive integers: 2^64 (a power of two, powerful for every
  # k up to 64), 10^20, and a window around p^2 for a prime p, so that a
  # square of a prime and its neighbours are powerful or not as k grows.
  my $p = Math::BigInt->new(next_prime(Math::BigInt->new('8589934592')));
  my @windows = (
    [$two64 - 6, $two64 + 6],
    [Math::BigInt->new('100000000000000000000') - 5, Math::BigInt->new('100000000000000000000') + 6],
    [$p * $p - 4, $p * $p + 4],
  );
  for my $w (@windows) {
    my ($lo, $hi) = @$w;
    my @ints;
    for (my $i = $lo->copy; $i <= $hi; $i++) { push @ints, $i->copy; }
    emit($fh, 'prime_count', [$lo, $hi], sub { prime_count($lo, $hi) });
    emit($fh, 'twin_prime_count', [$lo, $hi], sub { twin_prime_count($lo, $hi) });
    # PP semiprime_count walks lo..hi with a native `$lo++`; a start below 2^64
    # that reaches 2^64 turns into a float and every later number fails
    # is_semiprime (docs/upstream-findings.md entry 20). The row then holds
    # the sum of is_semiprime over the window, the correct value.
    {
      my $direct = eval { semiprime_count($lo->copy, $hi->copy) };
      my $count = scalar grep { is_semiprime($_) } @ints;
      print $fh "# upstream finding 20: PP semiprime_count($lo, $hi) = $direct, correct $count\n"
        if !defined $direct || "$direct" ne "$count";
      emit($fh, 'semiprime_count', [$lo, $hi], sub { $count });
    }
    emit($fh, 'sum_primes', [$lo, $hi], sub { my $s = Math::BigInt->new(0); $s += $_ for grep { is_prime($_) } @ints; $s });
    emit($fh, 'prime_power_count', [$lo, $hi], sub { scalar grep { is_prime_power($_) } @ints });
    emit($fh, 'sumtotient', [$lo, $hi], sub { my $s = Math::BigInt->new(0); $s += euler_phi($_) for @ints; $s });
    emit($fh, 'mertens', [$lo, $hi], sub { my $s = Math::BigInt->new(0); $s += moebius($_) for @ints; $s });
    emit($fh, 'sumliouville', [$lo, $hi], sub { my $s = Math::BigInt->new(0); $s += liouville($_) for @ints; $s });
    for my $k (1, 2, 3, 4) {
      emit($fh, 'almost_prime_count', [$k, $lo, $hi], sub { scalar grep { prime_bigomega($_) == $k } @ints });
      emit($fh, 'omega_prime_count', [$k, $lo, $hi], sub { scalar grep { prime_omega($_) == $k } @ints });
    }
    for my $k (1, 2, 3, 64) {
      emit($fh, 'powerful_count', [$k, $lo, $hi], sub { scalar grep { is_powerful($_, $k) } @ints });
      emit($fh, 'sumpowerful', [$k, $lo, $hi], sub { my $s = Math::BigInt->new(0); $s += $_ for grep { is_powerful($_, $k) } @ints; $s });
    }
  }
}
1;
