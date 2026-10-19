# MPU 0.75 pure-Perl oracle for the range iterators on Math::BigInt endpoints (no GMP).
use strict;
use warnings;
sub gen_bigint_iter {
  my ($fh) = @_;
  die 'bigint_iter requires MPU_NO_XS=1' unless $ENV{MPU_NO_XS};
  my $b = sub { Math::BigInt->new("$_[0]") };
  my $p64 = $b->(2)->copy->bpow(64);
  my $p80 = $b->(2)->copy->bpow(80);
  # [lo, hi] pairs: all endpoints are above 2^64 except the one that crosses it.
  my @ranges = (
    [$p64, $p64 + 12], [$p64 + 1, $p64 + 1], [$p64 + 20, $p64 + 10],
    [$p64 - 6, $p64 + 6], [$p80 + 1000003, $p80 + 1000010],
  );
  for my $r (@ranges) {
    my ($lo, $hi) = @$r;
    for my $fn (qw(forsemiprimes forsquarefree forsquarefreeint)) {
      emit($fh, $fn, [$lo, $hi], sub { my @o; no strict 'refs'; &{$fn}(sub { push @o, "$_" }, $lo, $hi); @o });
    }
    for my $k (1 .. 3) {
      emit($fh, 'foralmostprimes', [$k, $lo, $hi], sub { my @o; foralmostprimes { push @o, "$_" } $k, $lo, $hi; @o });
    }
    emit($fh, 'forfactored', [$lo, $hi], sub { my @o; forfactored { push @o, "$_:" . join(',', @_) } $lo, $hi; @o });
  }
  for my $n ($b->(0), $b->(1), $p64, $p64 + 4, $p64 + 1, $p80, $p80 + 1, $p64 * 3) {
    emit($fh, 'fordivisors', [$n], sub { my @o; fordivisors { push @o, "$_" } $n; @o });
  }
}
1;
