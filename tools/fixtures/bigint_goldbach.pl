# Fixture family `bigint_goldbach`: goldbach_pair_count and goldbach_pairs on
# Math::BigInt input, for the `bigint` feature (src/bigint/families.rs).
# Replayed by tests/compat_bigint_goldbach.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1 and MPU_NO_GMP=1). PP.pm answers an
# odd n (and n = 4) of any size at once from is_prime(n - 2). For an even n it
# tests n - p for every prime p from n/2 to n - 3, so the even rows stay small:
# past 2^64 that walk does not finish (measured 2026-10-19: 2^64 timed out
# after 25 s), and the Rust form does the same walk.
use strict;
use warnings;
sub gen_bigint_goldbach {
  my ($fh) = @_;
  die 'bigint_goldbach needs the pure-Perl oracle' unless $ENV{MPU_NO_XS} && $ENV{MPU_NO_GMP};
  my $two = Math::BigInt->new(2);
  my @n = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 22, 27, 100, 101, 128, 1000, 4096, 65536,
           1000000, 1000001,
           # odd, near u64::MAX: 2^64 - 59 is the largest 64-bit prime
           '18446744073709551557', '18446744073709551559', '18446744073709551561',
           '18446744073709551615');
  # odd, past 2^64: next_prime(2^k) + 2 (n - 2 prime), the odd number after
  # it, and 2^k + 1 (n - 2 = 2^k - 1, composite)
  for my $k (64, 65, 100, 200) {
    my $p = next_prime($two->copy->bpow($k));
    push @n, $p + 2, $p + 4, $two->copy->bpow($k) + 1;
  }
  for my $v (@n) {
    my $n = Math::BigInt->new("$v");
    emit($fh, 'goldbach_pair_count', [$n], sub { goldbach_pair_count($n->copy) });
    emit($fh, 'goldbach_pairs', [$n], sub { my @r = goldbach_pairs($n->copy); @r });
  }
}
1;
