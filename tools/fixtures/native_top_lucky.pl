# Fixture family `native_top_lucky`: what MPU 0.75 returns at the top of the
# u64 range for the lucky, Ramanujan and twin-prime names that have no BigUint
# form (`n.a.` in docs/bigint-coverage.md: past 2^64 MPU's pure-Perl path does
# not finish, runs out of memory or croaks). Replayed against the native forms
# by tests/compat_native_top_lucky.rs: a value must match, and where MPU died
# (ERROR) the native form must return None.
#
# Oracle: MPU 0.75 with XS and without GMP (MPU_NO_GMP=1), the C code the
# native forms port. XS answers is_lucky and is_ramanujan_prime here from their
# pretests; nth_twin_prime and nth_ramanujan_prime pass an index past the C
# limits to PP.pm, which croaks on the float n + 200 or 48/19 * n.
#
# Left out, measured 2026-10-19 (20 s, 4 GB): MPU does not finish or runs out
# of memory for lucky_count(2^64 - 1), lucky_numbers and ramanujan_primes on
# [2^64 - 101, 2^64 - 1], ramanujan_prime_count on that window, nth_lucky(2^64 - 1),
# is_lucky(18446744073709551517) and is_ramanujan_prime(2^64 - 83).
use strict;
use warnings;
sub gen_native_top_lucky {
  my ($fh) = @_;
  die 'native_top_lucky needs XS' unless prime_get_config()->{xs};
  die 'native_top_lucky needs MPU_NO_GMP=1' if prime_get_config()->{gmp};
  # 2^64 - 1, 2^64 - 59 (the largest 64-bit prime) and 2^64 - 2.
  my @top = ('18446744073709551615', '18446744073709551557', '18446744073709551614');
  for my $v (@top) {
    emit($fh, 'is_lucky', [$v], sub { is_lucky($v) });
    emit($fh, 'is_ramanujan_prime', [$v], sub { is_ramanujan_prime($v) });
  }
  for my $v ('18446744073709551615', '18446744073709551614') {
    emit($fh, 'nth_twin_prime', [$v], sub { nth_twin_prime($v) });
    emit($fh, 'nth_ramanujan_prime', [$v], sub { nth_ramanujan_prime($v) });
  }
}
1;
