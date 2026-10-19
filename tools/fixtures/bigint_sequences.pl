# MPU 0.75 pure-Perl sequence and product oracle. No GMP calls.
use strict;
use warnings;
require Math::Prime::Util::PP;

sub gen_bigint_sequences {
  my ($fh) = @_;
  my $big = Math::BigInt->new('18446744073709551629');
  my $emit = sub {
    my ($name, @args) = @_;
    my $code = Math::Prime::Util::PP->can($name) or die "missing PP $name";
    emit($fh, $name, [@args], sub { $code->(map { Math::BigInt->new("$_") } @args) });
  };
  for my $k (0, 1, 2, 3, 10, 63, 93, 100, 256, 1000) {
    $emit->($_, $k) for qw(fibonacci lucas_number);
  }
  for my $pq ([1,-1], [3,2], [0,1], [-2,-3], [$big,2], [2,-$big]) {
    for my $k (0, 1, 2, 3, 4, 5, 8, 17) {
      $emit->($_, @$pq, $k) for qw(lucasu lucasv lucasuv);
    }
  }
  # These products already exceed 64 bits at small indices. Indices above
  # 2^64 require impractical output storage in both MPU and Rust.
  for my $n (0, 1, 2, 3, 10, 50, 100, 500) {
    $emit->($_, $n) for qw(primorial pn_primorial consecutive_integer_lcm);
  }
  for my $n (0, 1, 2, 10, $big, $big * $big) {
    $emit->('powersum', $n, $_) for (0, 1, 2, 3, 4, 5, 6, 9);
  }
  for my $list ([], [0], [1], [$big], [$big, $big+1, $big*3], [$big,0,7]) {
    $emit->($_, @$list) for qw(vecsum vecprod vecprefixsum);
  }
  for my $args ([0,1,2,3], [4,10,6,3], [10,7,0,15],
                [$big, $big+1, $big*2, $big*3], [$big,7,$big,19],
                [17,$big,3,0], [0,0,0,0]) {
    $emit->('floor_sum', @$args);
  }
  for my $list ([], [7], [1,$big], [6,9,20], [6,9,20,20],
                [6,10,15], [7,11,$big], [$big,$big+1], [0,3], [6,9]) {
    $emit->('frobenius_number', @$list);
  }
}
1;
