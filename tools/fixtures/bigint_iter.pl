use strict;
use warnings;
sub gen_bigint_iter {
  my ($fh) = @_;
  for my $n (('18446744073709551616', '18446744073709551617', '18446744073709551620')) {
    for my $fn (qw(forsemiprimes forsquarefreeint forsquarefree forfactored)) {
      emit($fh, $fn, [$n, $n], sub { return $fn eq 'forfactored' ? factor($n) : is_square_free($n) ? $n : () });
    }
    emit($fh, 'foralmostprimes', [2, $n, $n], sub { is_semiprime($n) ? $n : () });
    emit($fh, 'fordivisors', [$n], sub { divisors($n) });
  }
}
1;
