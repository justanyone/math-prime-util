# Fixture family `bigint_oneint`: inverse_sigma0, inverse_sigma0_count,
# farey_rank and is_congruent_number past 2^64 (src/bigint/functions.rs,
# rational.rs, predicates.rs), and the 64-bit edge of the five names that stay
# native (hclassno, integer_complexity, is_pillai, is_catalan_pseudoprime;
# farey has no such row). Replayed by tests/compat_bigint_oneint.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1 and MPU_NO_GMP=1, Math::BigInt), the
# path MPU takes for arguments past 2^64. Each input is one the PP code finishes
# in practical time:
# - inverse_sigma0: narrow windows (factored one by one), prime k and k = 9 on
#   wide ranges (prime signature generator), and the k = 0, 1, 2 exits.
# - farey_rank: big fractions with orders on both sides of the 10^6 switch from
#   the Moebius table to Mertens values.
# - is_congruent_number: inputs that a filter decides (5, 6, 7 mod 8, square
#   part removal, prime, 2p, pq and 2pq). Other inputs go to Tunnell's count,
#   which is linear in n and does not finish past 2^64.
# The native-only names, measured above 2^64 with the PP path: hclassno and
# integer_complexity croak past 2^63-1; is_pillai (prime n), farey and
# is_catalan_pseudoprime run past a 30 s limit.
use strict;
use warnings;
sub gen_bigint_oneint {
  my ($fh) = @_;
  die 'This family requires MPU_NO_XS=1' unless $ENV{MPU_NO_XS};
  die 'This family requires MPU_NO_GMP=1' unless $ENV{MPU_NO_GMP};
  die 'GMP backend is on' if prime_get_config()->{gmp};
  require Math::Prime::Util::PP;
  my $B = sub { Math::BigInt->new("$_[0]") };
  my $two64 = $B->('18446744073709551616');

  # inverse_sigma0 and inverse_sigma0_count: (k, lo, hi).
  my @sig = (
    [0, 1, $two64 * 4], [1, 0, $two64], [1, 2, $two64], [2, $two64, $two64 + 200],
    [4, $two64, $two64 + 60], [6, $two64, $two64 + 60], [8, $two64, $two64 + 60],
    [12, $two64, $two64 + 60], [16, $two64, $two64 + 60], [3, $two64, $two64 + 60],
    [11, $two64, $two64 * 65536], [13, 1, $two64 ** 2], [7, $two64, $two64 * 64],
    [9, $two64, $two64 * 4], [$two64, $two64, $two64 + 10], [$two64 * 2, 1, $two64],
    [4, $B->('1000000000000000000000000000000'), $B->('1000000000000000000000000000040')],
  );
  for my $c (@sig) {
    my ($k, $lo, $hi) = map { $B->($_) } @$c;
    emit($fh, 'inverse_sigma0', [$k, $lo, $hi],
         sub { @{Math::Prime::Util::PP::inverse_sigma0($k->copy, $lo->copy, $hi->copy)} })
      unless $k == 7 || $k == 9 || $k == 13;   # long lists: count only
    emit($fh, 'inverse_sigma0_count', [$k, $lo, $hi],
         sub { Math::Prime::Util::PP::inverse_sigma0_count($k->copy, $lo->copy, $hi->copy) });
  }

  # farey_rank: (n, p, q).
  my ($fa, $fb) = ($B->(1), $B->(1));
  ($fa, $fb) = ($fb, $fa + $fb) for 1 .. 100;
  my @frac = ([1, $two64 + 1], [$two64, $two64 + 1], [$fa, $fb], [$fb, $fa],
              [$B->('1000000000000000000000000000000'), $B->('3000000000000000000000000000000')],
              [$B->('123456789012345678901234567890'), $B->('987654321098765432109876543210')],
              [$two64 + 1, $two64], [$two64 * 4, $two64 * 4], [0, $two64]);
  for my $n (1, 2, 9, 100, 1000, 65536, 1000000, 1000003) {
    for my $f (@frac) {
      my ($p, $q) = map { $B->($_) } @$f;
      emit($fh, 'farey_rank', [$n, $p, $q], sub { Math::Prime::Util::PP::farey_rank($n, [$p->copy, $q->copy]) });
    }
  }

  # is_congruent_number on inputs a filter decides.
  my $np = sub {   # next prime above $_[0] with the residue $_[2] mod $_[1] and the test $_[3]
    my ($from, $mod, $res, $ok) = @_;
    my $p = $B->($from);
    while (1) {
      $p = Math::Prime::Util::next_prime($p);
      return $p if $p % $mod == $res && (!$ok || $ok->($p));
    }
  };
  my $kr = sub { Math::Prime::Util::kronecker($_[0], $_[1]) };
  my $p3 = $np->($two64, 8, 3);
  my $p5 = $np->($two64, 8, 5);
  my $p9 = $np->($two64, 16, 9);
  my $p1 = $np->($two64, 8, 1, sub {   # prime 1 mod 8 that the sqrtmod test rejects
    my $r = Math::Prime::Util::sqrtmod(2, $_[0]);
    $kr->($r + 1, $_[0]) == -1 });
  my $q3 = $np->($two64, 8, 3, sub { $kr->(17, $_[0]) == -1 });
  my $q5 = $np->($two64, 8, 5, sub { $kr->(17, $_[0]) == -1 });
  my $q1 = $np->($two64, 8, 1, sub { $kr->($_[0], 3) == -1 });
  my $q1b = $np->($two64, 8, 1, sub { $kr->($_[0], 5) == -1 });
  my $s = $B->('18446744073709551629');
  for my $n ($two64 + 5, $two64 + 6, $two64 + 7, $two64 * 2 + 15,
             $two64 * $two64 * 5, $two64 * $two64 * 4, $s * $s * 3, $s * $s * 34, $s * $s * 41, $s * $s,
             $p3, $p5, $p1, $p9 * 2, $p5 * 2, $p3 * 3, $q3 * 17, $q1 * 3, $p5 * 10, $q5 * 34, $q1b * 10) {
    emit($fh, 'is_congruent_number', [$n], sub { 0 + Math::Prime::Util::PP::is_congruent_number($n->copy) });
  }

  # Native-only names at the 64-bit edge: the native forms give the same answer.
  for my $v ('9223372036854775808', '18446744073709551615') {
    my $n = $B->($v);
    emit($fh, 'hclassno', [$n], sub { Math::Prime::Util::PP::hclassno($n->copy) });
    emit($fh, 'integer_complexity', [$n], sub { Math::Prime::Util::PP::integer_complexity($n->copy) });
  }
  for my $v ('18446744073709551614', '18446744073709551615', '9223372036854775807') {
    my $n = $B->($v);
    emit($fh, 'is_pillai', [$n], sub { Math::Prime::Util::PP::is_pillai($n->copy) });
  }
  for my $v ('18446744073709551614', '18446744073709551612') {
    my $n = $B->($v);
    emit($fh, 'is_catalan_pseudoprime', [$n], sub { Math::Prime::Util::PP::is_catalan_pseudoprime($n->copy) });
  }
}
1;
