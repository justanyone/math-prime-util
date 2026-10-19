# MPU 0.75 PP oracle. Use MPU_NO_XS=1 and the vendored Perl library offline.
use strict;
use warnings;
sub gen_bigint_family_counts {
  my ($fh) = @_;
  die 'This family requires MPU_NO_XS=1' unless $ENV{MPU_NO_XS};
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
    # PP lacks range forms for several sums. Use exact singleton differences.
    # Choose 2^65 and 2^64, whose factorization is cheap in pure Perl.
    for my $fn (qw(semiprime_count prime_count sum_primes sumtotient mertens sumliouville)) {
      my %single = (
        semiprime_count => sub { is_semiprime($n) },
        prime_count => sub { is_prime($n) ? 1 : 0 },
        sum_primes => sub { is_prime($n) ? $n : 0 },
        sumtotient => sub { euler_phi($n) },
        mertens => sub { moebius($n) },
        sumliouville => sub { liouville($n) },
      );
      emit($fh, $fn, [$n, $n], $single{$fn});
    }
    for my $k (0, 1, 2, 65) {
      emit($fh, 'almost_prime_count', [$k, $n, $n], sub { is_almost_prime($k, $n) });
      emit($fh, 'omega_prime_count', [$k, $n, $n], sub { is_omega_prime($k, $n) });
      emit($fh, 'powerful_count', [$k, $n, $n], sub { is_powerful($n, $k) });
      emit($fh, 'sumpowerful', [$k, $n, $n], sub { is_powerful($n, $k) ? $n : 0 });
    }
  }
}
1;
