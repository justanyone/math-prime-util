# Seeded MPU 0.75 pure-Perl random bigint oracle. No GMP is used.
use strict; use warnings;
sub gen_bigint_random {
  my ($fh) = @_;
  require Math::Prime::Util::PP;
  my $seed = 'bigint-random-fixed-seed-2026';
  for my $bits (65, 79, 80, 128, 512) {
    csrand($seed);
    emit($fh, 'urandomb', [$bits], sub { Math::Prime::Util::PP::urandomb($bits) });
    csrand($seed);
    emit($fh, 'random_nbit_prime', [$bits], sub { Math::Prime::Util::PP::random_nbit_prime($bits) });
  }
  for my $n ('18446744073709551616', '340282366920938463463374607431768211507') {
    my $b = Math::BigInt->new($n);
    csrand($seed);
    emit($fh, 'urandomm', [$n], sub { Math::Prime::Util::PP::urandomm($b) });
    csrand($seed);
    emit($fh, 'urandomr', [$n, $b+100], sub { Math::Prime::Util::PP::urandomr($b, $b+100) });
    csrand($seed);
    emit($fh, 'random_prime', [$n, $b+1000], sub { Math::Prime::Util::PP::random_prime($b, $b+1000) });
    csrand($seed);
    emit($fh, 'random_factored_integer', [$n], sub { Math::Prime::Util::PP::random_factored_integer($b) });
    csrand($seed);
    emit($fh, 'miller_rabin_random', [$n, 5], sub { Math::Prime::Util::PP::miller_rabin_random($b,5) });
  }
  for my $entry (['random_ndigit_prime',21], ['random_safe_prime',65], ['random_strong_prime',128], ['random_semiprime',65], ['random_unrestricted_semiprime',65]) {
    my ($fn,$bits) = @$entry;
    csrand($seed);
    no strict 'refs';
    emit($fh, $fn, [$bits], sub { &{"Math::Prime::Util::PP::$fn"}($bits) });
  }
}
1;
