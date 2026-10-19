# Exact product and permutation cases. Use the pure-Perl MPU 0.75 path.
use strict;
use warnings;
use Math::Prime::Util::PP;

sub gen_bigint_combinat {
  my ($fh) = @_;
  for my $text (0, 1, 5, 21, '18446744073709551616',
                '18446744073709551617', '340282366920938463463374607431768211456') {
    my $n = Math::BigInt->new($text);
    for my $m (0, 1, 2, 5) {
      emit($fh, 'falling_factorial', [$n, $m], sub { Math::Prime::Util::PP::falling_factorial($n->copy, Math::BigInt->new($m)) });
      emit($fh, 'rising_factorial', [$n, $m], sub { Math::Prime::Util::PP::rising_factorial($n->copy, Math::BigInt->new($m)) });
    }
    my @steps = $n > 100 ? ($n->copy, $n-1, $n+1) : (1, 2, 7);
    for my $k (@steps) {
      emit($fh, 'multifactorial', [$n, $k], sub { Math::Prime::Util::PP::multifactorial($n->copy, Math::BigInt->new($k)) });
    }
  }
  for my $n (0, 1, 3, 10, 21, 25) {
    for my $text (0, 1, 15, '18446744073709551616', '340282366920938463463374607431768211456') {
      my $rank = Math::BigInt->new($text);
      emit($fh, 'numtoperm', [$n, $rank], sub { Math::Prime::Util::PP::numtoperm($n, $rank->copy) });
    }
    my @p = reverse 0..$n-1;
    emit($fh, 'permtonum', [@p], sub { Math::Prime::Util::PP::permtonum(\@p) });
  }
  for my $p ([0,0], [2,0], ['18446744073709551616']) {
    emit($fh, 'permtonum', $p, sub { Math::Prime::Util::PP::permtonum($p) });
  }
}
1;
