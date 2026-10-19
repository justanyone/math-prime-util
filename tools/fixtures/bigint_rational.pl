# MPU 0.75 pure-Perl rational algorithms on Math::BigInt inputs.
use strict;
use warnings;
use Math::Prime::Util::PP;

sub gen_bigint_rational {
  my ($fh) = @_;
  my @indices = map { Math::BigInt->new($_) }
    ('18446744073709551616', '18446744073709551617',
     '123456789012345678901234567890', '340282366920938463463374607431768211455');
  for my $n (@indices) {
    for my $name (qw(nth_calkin_wilf nth_stern_brocot nth_stern_diatomic)) {
      no strict 'refs';
      emit($fh, $name, [$n], sub { &{"Math::Prime::Util::PP::$name"}($n->copy) });
    }
    for my $tree (qw(calkin_wilf stern_brocot)) {
      no strict 'refs';
      my @pair = &{"Math::Prime::Util::PP::nth_$tree"}($n->copy);
      my $name = "${tree}_n";
      emit($fh, $name, [@pair], sub { &{"Math::Prime::Util::PP::$name"}(@pair) });
    }
  }
  for my $n (@indices) {
    my ($p, $q) = ($n + 1, $n + 3);
    emit($fh, 'contfrac', [$p,$q], sub { Math::Prime::Util::PP::contfrac($p,$q) });
    my @cf = Math::Prime::Util::PP::contfrac($p,$q);
    emit($fh, 'from_contfrac', [@cf], sub { Math::Prime::Util::PP::from_contfrac(@cf) });
    emit($fh, 'convergents', [@cf], sub { map { @$_ } Math::Prime::Util::PP::convergents(@cf) });
    for my $name (qw(next_calkin_wilf next_stern_brocot)) {
      # Adjacent Fibonacci numbers give large fractions with short tree paths.
      my ($a,$b) = (Math::BigInt->new(1), Math::BigInt->new(1));
      ($a,$b) = ($b,$a+$b) for 1..100;
      no strict 'refs';
      emit($fh, $name, [$a,$b], sub { &{"Math::Prime::Util::PP::$name"}($a,$b) });
    }
    emit($fh, 'next_farey', [$n,1,3], sub { @{Math::Prime::Util::PP::next_farey($n,[1,3])} });
  }
}
1;
