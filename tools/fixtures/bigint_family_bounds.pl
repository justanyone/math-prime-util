# MPU 0.75 pure Perl count bounds above the native range.
use strict;
use warnings;

sub gen_bigint_family_bounds {
  my ($fh) = @_;
  die "bigint_family_bounds requires MPU_NO_XS=1\n" unless $ENV{MPU_NO_XS};
  die "bigint_family_bounds requires MPU_NO_GMP=1\n" unless $ENV{MPU_NO_GMP};
  for my $bits (64, 65, 80, 100, 127) {
    my $base = Math::BigInt->new(2)->bpow($bits);
    for my $delta (-1, 0, 1) {
      my $n = $base + $delta;
      for my $family (qw(perfect_power)) {
        for my $kind (qw(approx lower upper)) {
          my $name = "${family}_count_${kind}";
          my $code = Math::Prime::Util->can($name);
          emit($fh, $name, [$n], sub { $code->($n) });
        }
      }
    }
  }
}

1;
