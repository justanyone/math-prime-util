# Fixture family `bigint_counts`: prime counts, bounds and estimates, the nth
# prime and is_polygonal beyond the 64-bit range, for the `bigint` feature
# (src/bigint/counts.rs). Replayed by tests/compat_bigint_counts.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1, Math::BigInt, Math::BigFloat at its
# default 40 digits). Inputs above 2^64 take the Math::BigFloat formulas of
# PP.pm; the native count fixture (count.tsv) holds the XS values below.
#
# nth_prime itself: PP sieves from a lower bound in pure Perl and does not
# return in reasonable time above 2^64, so its rows come from the next_prime
# chain that starts at the largest 64-bit prime (MPU_MAXPRIME, index
# MPU_MAXPRIMEIDX = 425656284035217743).
#
# Known precision limits of the oracle (docs/upstream-findings.md):
# - nth_prime_approx: a double estimate whose integer part PP keeps to 15
#   significant digits (toint on an NV).
# - inputs of 40 digits and more: Math::BigFloat carries 40 digits through
#   log, so the bounds there have trailing noise. Two rows show it.
use strict; use warnings;

sub _cnt_pow { my ($b, $e) = @_; return Math::BigInt->new($b)->bpow($e); }

sub gen_bigint_counts {
  my $fh = shift;

  # Inputs above the native range: 2^64, round powers, and the branch limits
  # of PP's prime_count_lower/upper (1e20, 1.101e26) on both sides.
  my @big = (_cnt_pow(2, 64), _cnt_pow(2, 64) + 1, _cnt_pow(2, 65), _cnt_pow(10, 20) - 1, _cnt_pow(10, 20),
             _cnt_pow(10, 20) + 1, _cnt_pow(10, 21), Math::BigInt->new('123456789012345678901234'),
             _cnt_pow(10, 25), Math::BigInt->new('110099999999999999999999999'),
             Math::BigInt->new('110100000000000000000000000'), _cnt_pow(10, 27), _cnt_pow(2, 100),
             _cnt_pow(10, 30), _cnt_pow(10, 33), _cnt_pow(2, 127), _cnt_pow(10, 40), _cnt_pow(2, 200));
  # nth-prime indices between MPU_MAXPRIMEIDX and 2^64 take the Perl path too.
  my @nth = (Math::BigInt->new('425656284035217744'), Math::BigInt->new('425656284035217745'),
             _cnt_pow(10, 19), _cnt_pow(2, 64) - 1, @big);

  for my $n (@nth) {
    emit($fh, 'nth_prime_lower',  [$n], sub { nth_prime_lower($n) });
    emit($fh, 'nth_prime_upper',  [$n], sub { nth_prime_upper($n) });
    emit($fh, 'nth_prime_approx', [$n], sub { nth_prime_approx($n) });
  }
  for my $n (@big) {
    emit($fh, 'prime_count_lower',  [$n], sub { prime_count_lower($n) });
    emit($fh, 'prime_count_upper',  [$n], sub { prime_count_upper($n) });
    emit($fh, 'prime_count_approx', [$n], sub { prime_count_approx($n) });
  }

  # nth_prime above MPU_MAXPRIMEIDX: the next_prime chain from MPU_MAXPRIME.
  my $idx = Math::BigInt->new('425656284035217743');
  my $p = Math::BigInt->new('18446744073709551557');
  for my $k (1 .. 5) {
    $p = Math::BigInt->new(next_prime($p));
    my $n = $idx + $k;
    emit($fh, 'nth_prime', [$n], sub { $p });
  }

  # is_polygonal on k-gonal numbers P(k, r) = r((k-2)(r-1)+2)/2 and their
  # neighbours, with n or k above 2^64.
  for my $k (3, 4, 5, 6, 10, _cnt_pow(2, 32) + 1, _cnt_pow(2, 64), _cnt_pow(2, 64) + 1, _cnt_pow(2, 100) + 3) {
    for my $r (2, 3, 7, _cnt_pow(2, 31), _cnt_pow(2, 64), _cnt_pow(2, 70) + 5) {
      my ($kk, $rr) = (Math::BigInt->new($k), Math::BigInt->new($r));
      my $n = $rr * (($kk - 2) * ($rr - 1) + 2) / 2;
      next if $n <= Math::BigInt->new('18446744073709551615') && $kk <= Math::BigInt->new('18446744073709551615');
      for my $m ($n, $n + 1, $n - 1) {
        emit($fh, 'is_polygonal', [$m, $kk], sub { is_polygonal($m, $kk) });
      }
    }
  }
}

1;
