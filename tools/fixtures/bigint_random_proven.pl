# Seeded MPU 0.75 pure-Perl proven random prime oracle. No GMP is used.
# The certificate itself is not emitted: the Rust tests verify their own certificate.
use strict; use warnings;
sub gen_bigint_random_proven {
  my ($fh) = @_;
  require Math::Prime::Util::PP;
  my $seed = 'bigint-random-proven-seed-2026';
  my @fns = qw(random_maurer_prime random_maurer_prime_with_cert random_shawe_taylor_prime
               random_shawe_taylor_prime_with_cert random_proven_prime random_proven_prime_with_cert);
  for my $bits (65, 80, 128, 200, 300) {
    for my $fn (@fns) {
      csrand($seed);
      no strict 'refs';
      emit($fh, $fn, [$bits], sub { (&{"Math::Prime::Util::PP::$fn"}($bits))[0] });
    }
  }
  for my $bits (0, 1) {
    for my $fn (@fns) {
      csrand($seed);
      no strict 'refs';
      emit($fh, $fn, [$bits], sub { (&{"Math::Prime::Util::PP::$fn"}($bits))[0] });
    }
  }
}
1;
