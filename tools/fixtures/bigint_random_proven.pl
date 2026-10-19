# Seeded MPU 0.75 pure-Perl proven random prime oracle. No GMP is used.
use strict; use warnings;
sub gen_bigint_random_proven {
  my ($fh) = @_;
  require Math::Prime::Util::PP;
  my $seed = 'bigint-random-proven-seed-2026';
  for my $bits (65, 80, 128, 200) {
    for my $fn (qw(random_maurer_prime random_shawe_taylor_prime random_proven_prime)) {
      csrand($seed);
      no strict 'refs';
      emit($fh, $fn, [$bits], sub { &{"Math::Prime::Util::PP::$fn"}($bits) });
    }
  }
}
1;
