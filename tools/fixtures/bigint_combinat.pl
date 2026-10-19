# Exact products, permutations, counting numbers, Bernoulli, harmonic and tau cases. Use the pure-Perl MPU 0.75 path.
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
  gen_bigint_combinat_counts($fh);
}

# Counting numbers, fractions and tau. Results above 2^64 start near n = 26.
sub gen_bigint_combinat_counts {
  my ($fh) = @_;
  my %sets = (
    bell_number => [0, 1, 5, 25, 26, 30, 50, 100, 200],
    catalan_number => [0, 1, 10, 35, 36, 50, 100, 500],
    subfactorial => [0, 1, 2, 10, 21, 22, 50, 100],
    fubini => [0, 1, 5, 13, 14, 30, 60, 100],
    partitions => [0, 1, 100, 416, 417, 1000, 2000],
    partitionsq => [0, 1, 100, 500, 1000, 2000],
  );
  for my $name (sort keys %sets) {
    my $code = Math::Prime::Util::PP->can($name) or die "no PP::$name";
    for my $n (@{ $sets{$name} }) {
      emit($fh, $name, [$n], sub { $code->($n) });
    }
  }
  for my $case ([0,0], [5,0], [5,6], [10,10], [30,5], [50,25], [100,10], [100,50], [60,1], [60,59]) {
    my ($n, $m) = @$case;
    for my $kind (1, 2, 3) {
      emit($fh, 'stirling', [$n, $m, $kind], sub { Math::Prime::Util::PP::stirling($n, $m, $kind) });
    }
  }
  for my $n (0, 1, 2, 3, 12, 30, 60, 100, 200, 300) {
    emit($fh, 'bernfrac', [$n], sub { Math::Prime::Util::PP::bernfrac($n) });
  }
  for my $n (0, 1, 2, 10, 50, 89, 100, 500, 1000) {
    emit($fh, 'harmfrac', [$n], sub { Math::Prime::Util::PP::harmfrac($n) });
  }
  # Each prime tau costs about n steps: only smooth inputs go above 2^64
  # (the pure-Perl path needs minutes for 2^100 * 9, so it is not a row).
  my $two = Math::BigInt->new(2);
  for my $n (Math::BigInt->new(0), Math::BigInt->new(1), Math::BigInt->new(2), Math::BigInt->new(30),
             Math::BigInt->new(49), Math::BigInt->new(10007),
             $two->copy->bpow(64)->bmul(15), $two->copy->bpow(65), $two->copy->bpow(70),
             Math::BigInt->new(3)->bpow(50)->bmul(Math::BigInt->new(7)->bpow(30))) {
    emit($fh, 'ramanujan_tau', [$n], sub { Math::Prime::Util::PP::ramanujan_tau($n->copy) });
  }
}
1;
