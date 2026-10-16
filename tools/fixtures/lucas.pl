# Fixture family `lucas`: t/25-lucas_sequences.t.
# Functions: lucas_sequence(n,P,Q,k) (3 values), lucasu(P,Q,k), lucasv(P,Q,k), lucasuv(P,Q,k) (2 values),
# lucasumod(P,Q,k,n), lucasvmod(P,Q,k,n), lucasuvmod(P,Q,k,n) (2 values), fibonacci(n), lucas_number(n).
# P and Q are signed (the args column keeps the minus sign); n and k of the *mod functions are u64.
# A value outside +-2^64 (a bigint in MPU) is written as `undef`. Left out: the bigint rows of the t file
# (n > u64, 77-bit and 24-digit P, Q) and the EXTENDED_TESTING row.
use strict; use warnings;

sub gen_lucas {
  my $fh = shift;
  my $U = Math::BigInt->new('18446744073709551615');
  # a result that does not fit +-u64 is `undef`
  my $fit = sub { my $v = shift; return 'undef' unless defined $v; my $b = Math::BigInt->new("$v"); $b->babs; $b > $U ? 'undef' : "$v" };
  my %dup;
  my $emit = sub {
    my ($fn, $args, $code) = @_;
    return if $dup{join(' ', $fn, @$args)}++;
    emit($fh, $fn, $args, $code);
  };
  my $u = sub { my ($P,$Q,$k) = @_; $emit->('lucasu', [$P,$Q,$k], sub { $fit->(lucasu($P,$Q,$k)) }) };
  my $v = sub { my ($P,$Q,$k) = @_; $emit->('lucasv', [$P,$Q,$k], sub { $fit->(lucasv($P,$Q,$k)) }) };
  my $uv = sub { my ($P,$Q,$k) = @_; $emit->('lucasuv', [$P,$Q,$k], sub { map { $fit->($_) } lucasuv($P,$Q,$k) }) };
  my $um = sub { my ($P,$Q,$k,$n) = @_; $emit->('lucasumod', [$P,$Q,$k,$n], sub { lucasumod($P,$Q,$k,$n) }) };
  my $vm = sub { my ($P,$Q,$k,$n) = @_; $emit->('lucasvmod', [$P,$Q,$k,$n], sub { lucasvmod($P,$Q,$k,$n) }) };
  my $uvm = sub { my ($P,$Q,$k,$n) = @_; $emit->('lucasuvmod', [$P,$Q,$k,$n], sub { lucasuvmod($P,$Q,$k,$n) }) };
  my $ls = sub { my ($n,$P,$Q,$k) = @_; $emit->('lucas_sequence', [$n,$P,$Q,$k], sub { map { "$_" } lucas_sequence($n,$P,$Q,$k) }) };
  my $all = sub { $u->(@_[0..2]); $v->(@_[0..2]); $uv->(@_[0..2]); };
  my $allm = sub { $um->(@_); $vm->(@_); $uvm->(@_); };

  # ---- OEIS sequences of the t file: lucasu / lucasv / lucasuv, and the mod versions
  my @seqs = ([1,-1],[2,-1],[1,-2],[2,2],[2,5],[3,-5],[3,-4],[3,-1],[3,1],[3,2],[4,-1],[4,1],[5,4],[4,4]);
  for my $pq (@seqs) {
    my ($P,$Q) = @$pq;
    for my $k (0 .. 60) {
      $all->($P,$Q,$k);
      $allm->($P,$Q,$k,'4294967295');
    }
    for my $k (0 .. 20) { $allm->($P,$Q,$k,'18446744073709551615'); $allm->($P,$Q,$k,1000003); }
    ls_k($ls, $P, $Q, 17, [0..12]);
  }

  # ---- negative / zero / special P and Q
  for my $P (-5 .. 5) {
    for my $Q (-5 .. 5) {
      for my $k (0 .. 12) { $all->($P,$Q,$k); }
      for my $k (0, 1, 2, 3, 10, 25, 64, 100) { $allm->($P,$Q,$k,1000003); $allm->($P,$Q,$k,'18446744073709551557'); }
      ls_k($ls, $P, $Q, 101, [0, 1, 2, 5, 10, 50]);
    }
  }
  for my $pq ([-1,1],[-2,-1],[-3,5],[7,-3],[10,-10],[100,1],[-100,-1],[46341,1],[1,46341],[3037000500,1],[1,3037000500],
              ['9223372036854775807',1],['-9223372036854775807',1],[1,'9223372036854775807'],[1,'-9223372036854775807']) {
    my ($P,$Q) = @$pq;
    for my $k (0 .. 6) { $all->($P,$Q,$k); }
    for my $k (0, 1, 2, 3, 77, 1000, '18446744073709551615') { $allm->($P,$Q,$k,1000003); $allm->($P,$Q,$k,'18446744073709551615'); }
  }

  # ---- big k and big n for the mod functions
  for my $pq ([1,-1],[3,5],[-3,7],[2,-1],[-1,-1],[5,2]) {
    for my $k (100, 1000, 1000000, '1000000000000', '4294967296', '9223372036854775808', '18446744073709551615') {
      for my $n (2, 3, 7, 1000003, '4294967295', '4294967296', '4294967311', '9223372036854775807', '18446744073709551557', '18446744073709551615') {
        $allm->($pq->[0], $pq->[1], $k, $n);
      }
    }
  }
  for my $n (0, 1, 2) { $allm->(1, -1, $_, $n) for 0, 1, 5; }

  # ---- %lucas_sequences, %lucas_dcheck, issue 47 of the t file
  my %lucas_sequences = (
    "323 1 1 324" => [0,2], "323 4 1 324" => [170,308], "323 4 5 324" => [194,156], "323 3 1 324" => [0,2],
    "323 3 1  81" => [0,287], "323 5 -1 81" => [153,195], "49001 25 117 24501" => [20933,18744],
    "18971 10001 -1 4743" => [5866,14421], "3613982123 1 -1 3613982124" => [0,3613982121],
    "3613982121 1 -1 3613982122" => [2586640546,2746447323], "3613982121 1 -1 1806991061" => [3535079342,1187662808],
    "547968611 1 -1 547968612" => [1,3], "547968611 1 -1 136992153" => [27044236,448467899],
  );
  my %lucas_dcheck = (
    "7777 -6 9 77" => 1, "7777 -6 7 77" => 1, "7777 4 3 77" => 1, "7777 4 4 77" => 1, "7777 3 5834 77" => 1,
    "7777 3 5835 77" => 1, "7777 1 5833 77" => 1, "7777 2 1 77" => 1, "7777 -8882 1 77" => 1,
    "7778 7776 1 32" => 1, "7778 7776 1 33" => 1, "7778 1976 5 32" => 1, "7778 1976 5 33" => 1,
  );
  for my $key (sort keys %lucas_sequences) {
    my ($n,$P,$Q,$k) = split ' ', $key;
    $allm->($P,$Q,$k,$n); $ls->($n,$P,$Q,$k);
  }
  for my $key (sort keys %lucas_dcheck) {
    my ($n,$P,$Q,$k) = split ' ', $key;
    $allm->($P,$Q,$k,$n); $all->($P,$Q,$k); $ls->($n,$P,$Q,$k);
  }
  my @issue47 = ([4,1,-1,951],[4,2,-1,951],[8,1,-1,47],[8,2,-1,47],[5,1,-1,0],[5,2,-1,0],[5,1,-1,66],[5,2,-1,66],
    [1001,-4,4,50],[1001,-4,7,50],[1001,1,-1,50],[5,1,-1,4],[3,6,9,36],[5,10,25,101],[6,10,25,101],[3,-6,9,0],
    [1,30,1,15],[3,3,3,1],[3,-30,-30,1],[1,9,5,0],[104,-14,49,0],[104,-14,49,1],[8,2,1,1],[16,0,0,1],[2,11,-27,0],[3,30,-2,1]);
  for my $i (@issue47) {
    my ($n,$P,$Q,$k) = @$i;
    $allm->($P,$Q,$k,$n); $ls->($n,$P,$Q,$k);
  }
  $ls->(17, 3, 1, 5);
  # lucas_sequence over odd n and small P, Q (n must be > 1 and odd in practice; others die or return)
  for my $n (2, 3, 4, 5, 7, 9, 15, 17, 101, 323, 1001, 65537, 4294967291, '18446744073709551557', '18446744073709551615') {
    for my $pq ([1,-1],[3,1],[5,-1],[10001,-1],[4,5],[-3,2]) { $ls->($n, @$pq, $_) for 0, 1, 2, 3, 10, 81, 324, 4743, '1000000007'; }
  }

  # ---- OEIS A081264 odd Fibonacci pseudoprimes via lucasumod, and A141137
  my @oeis = (323, 377, 1891, 3827, 4181, 5777, 6601, 6721, 8149, 10877, 11663, 13201, 13981, 15251, 17119, 17711, 18407, 19043, 23407, 25877, 27323, 30889, 34561, 34943, 35207, 39203, 40501, 50183, 51841, 51983, 52701, 53663, 60377, 64079, 64681);
  for my $n (@oeis, grep { $_ & 1 } 3 .. 1001) {
    my $t = (($n % 5) == 2 || ($n % 5) == 3) ? $n + 1 : $n - 1;
    $um->(1, -1, $t, $n);
  }
  { my $n = 8539786; my $e = (0,-1,1,1,-1)[$n % 5];
    $uvm->(1, -1, $n+$e, $n); $um->(1, -1, $n+$e, $n); $vm->(1, -1, $n+$e, $n); }
  # n = 257 comparison rows
  for my $k (0 .. 100) { $allm->(1, -1, $k, 257); $all->(1, -1, $k); }

  # ---- fibonacci / lucas_number (signed index; bigint results are undef)
  for my $n (-120 .. 120, 369, -369, 500, 1000) {
    $emit->('fibonacci', [$n], sub { $fit->(fibonacci($n)) });
    $emit->('lucas_number', [$n], sub { $fit->(lucas_number($n)) });
  }
  for my $n (qw/88 89 90 91 92 93 94 95 -88 -91 -93 -94 -45 55 4294967295/) {
    next if $n > 100000;
    $emit->('fibonacci', [$n], sub { $fit->(fibonacci($n)) });
    $emit->('lucas_number', [$n], sub { $fit->(lucas_number($n)) });
  }
}

sub ls_k { my ($ls, $P, $Q, $n, $ks) = @_; $ls->($n, $P, $Q, $_) for @$ks; }
1;
