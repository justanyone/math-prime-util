# MPU 0.75 pure Perl oracle for the additional bigint tests and factors.
use strict;
use warnings;
sub gen_bigint_moretests {
  my ($fh) = @_;
  die "bigint_moretests requires MPU_NO_XS=1" unless $ENV{MPU_NO_XS};
  for my $text (qw(0 1 2 3 4 5 7 23 101 5907 294001
      18446744073709551616 18446744073709551617 18446744073709551629
      18446744073709551647 170141183460469231731687303715884105727)) {
    my $n = Math::BigInt->new($text);
    for my $fn (qw(is_safe_prime is_chen_prime is_frobenius_pseudoprime
        is_frobenius_khashin_pseudoprime is_frobenius_underwood_pseudoprime)) {
      no strict 'refs';
      emit($fh, $fn, [$n], sub { &{"Math::Prime::Util::PP::$fn"}($n->copy) });
    }
    for my $base (2, 3, 10) {
      emit($fh, 'is_delicate_prime', [$n, $base], sub { Math::Prime::Util::PP::is_delicate_prime($n->copy, $base) });
    }
    for my $r (0..3) {
      emit($fh, 'is_perrin_pseudoprime', [$n, $r], sub { Math::Prime::Util::PP::is_perrin_pseudoprime($n->copy, $r) });
    }
    for my $b (0, 1, 2) {
      emit($fh, 'is_gaussian_prime', [$n, $b], sub { Math::Prime::Util::PP::is_gaussian_prime($n->copy, Math::BigInt->new($b)) });
    }
  }
  for my $text (qw(0 1 4 49 91 403 8051 18446744073709551629 18446744400127067027 340282366920938463426481119284349108225)) {
    my $n = Math::BigInt->new($text);
    emit($fh, 'fermat_factor', [$n, 100], sub { Math::Prime::Util::PP::fermat_factor($n->copy, 100) });
    emit($fh, 'cheb_factor', [$n, 100, 72], sub { Math::Prime::Util::PP::cheb_factor($n->copy, 100, 72) });
  }
  # PP substitutes Brent for Lehman. These cases agree with C's input cutoff.
  for my $text (qw(0 1 49 91 18446744073709551629 129127208515966861403)) {
    my $n = Math::BigInt->new($text);
    emit($fh, 'lehman_factor', [$n], sub { Math::Prime::Util::PP::lehman_factor($n->copy) });
  }
}
1;
