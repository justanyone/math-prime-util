# Fixture family `bigint_family_nth`: the nth almost prime, omega prime and
# powerful number past the 64-bit range, and the powerful numbers of an
# interval past 2^64, for the `bigint` feature (src/bigint/family_nth.rs).
# Replayed by tests/compat_bigint_family_nth.rs.
#
# Oracle: MPU 0.75 with XS and without GMP (run with MPU_NO_GMP=1, and without
# MPU_NO_XS). A value past 2^64 takes the PP.pm code, which hands native-size
# inner counts (prime counts, powerful counts) back to XS. With MPU_NO_XS=1
# the same PP.pm code runs those counts in Perl too, and several rows take
# minutes. Each row returns in under a minute: a large k with a small index
# (PP.pm `_kap_reduce_nth` writes it as a power of two times a native value),
# and indices just past the native range.
#
# powerful_numbers: PP.pm `_pcg` multiplies two native integers with Perl's
# `*`, so a product past 2^64 turns into a float and the call dies
# (docs/upstream-findings.md entry 23). Where it dies, the row holds what the
# function is defined to return: the integers of the interval that pass
# is_powerful, or, for a wide interval, every product of prime powers p^e
# with e >= k in it (`powerful_by_definition` below); a comment line before
# such a row gives the direct result where it dies or differs.
#
# Not here: nth_semiprime and nth_prime_power past the native range (also
# nth_almost_prime(2, n) and nth_omega_prime(1, n)) need prime counts near or
# past 2^64 and do not return; nth_powerful(n, 2) for n >= 2^64 dies (a NaN
# from PP.pm's estimate, entry 23) and its value is past what a count reaches.
use strict;
use warnings;

# Every product of prime powers p^e (e >= k) in lo..hi, sorted: the definition
# of a k-powerful number, independent of MPU's generator.
sub powerful_by_definition {
  my ($lo, $hi, $k) = @_;
  my @out;
  my $walk;
  $walk = sub {
    my ($m, $p) = @_;
    push @out, $m->copy if $m >= $lo;
    for (my $q = next_prime($p); ; $q = next_prime($q)) {
      my $pk = Math::BigInt->new("$q")->bpow($k);
      last if $m * $pk > $hi;
      for (my $f = $m * $pk; $f <= $hi; $f *= $q) {
        $walk->($f, $q);
      }
    }
  };
  $walk->(Math::BigInt->new(1), 1);
  return sort { $a <=> $b } @out;
}

sub gen_bigint_family_nth {
  my ($fh) = @_;
  die 'bigint_family_nth needs MPU_NO_GMP=1' unless $ENV{MPU_NO_GMP};
  die 'bigint_family_nth needs XS (unset MPU_NO_XS)' if $ENV{MPU_NO_XS};
  die 'GMP backend is on' if prime_get_config()->{gmp};
  local $SIG{__WARN__} = sub { warn @_ unless $_[0] =~ /^Deep recursion/ };
  $fh->autoflush(1);
  my $two64 = Math::BigInt->new('18446744073709551616');

  # nth_almost_prime(k, n): edges, then values past 2^64. 50 87993 and the
  # rows after it straddle the native range (max_almost_prime_count).
  for my $kn ([0, 1], [0, 2], [5, 0], [3, 10], [64, 1], [64, 2], [64, 7], [63, 3], [63, 8],
              [62, 9], [70, 100], [100, 1000], [200, 5000], [64, 1000000], [50, 87993],
              [50, 87994], [45, 3567979], [41, 66567489]) {
    my ($k, $n) = @$kn;
    emit($fh, 'nth_almost_prime', [$k, $n], sub { nth_almost_prime($k, $n) });
  }

  # nth_omega_prime(k, n): pn_primorial(16) is past 2^64; 15 299178 is the
  # last native value.
  for my $kn ([0, 1], [0, 2], [3, 0], [2, 1000], [16, 1], [16, 2], [16, 100], [17, 1],
              [20, 1], [15, 299178]) {
    my ($k, $n) = @$kn;
    emit($fh, 'nth_omega_prime', [$k, $n], sub { nth_omega_prime($k, $n) });
  }

  # nth_powerful(n, k): k <= 1 returns n; 1796 10, 9330124695 2 and 11938035 3
  # are the last native values.
  for my $nk (['0', 2], ['5', 0], ['5', 1], [$two64, 0], [$two64, 1], ['2', 64], ['3', 64],
              ['4', 64], ['10', 64], ['10', 100], ['20', 70], ['100', 60], ['1796', 10],
              ['1797', 10], ['9330124695', 2], ['9330124696', 2], ['11938035', 3],
              ['11938036', 3]) {
    my ($n, $k) = @$nk;
    $n = Math::BigInt->new("$n");
    emit($fh, 'nth_powerful', [$n, $k], sub { nth_powerful($n, $k) });
  }

  # powerful_numbers(lo, hi, k): the direct call where PP.pm returns.
  for my $w ([$two64 - 100, $two64 + 100, 2], [$two64, $two64 + 10, 0], [$two64, $two64 + 10, 1],
             [$two64 + 1, $two64 + 1000, 70], [Math::BigInt->new(1), $two64 + 5, 65],
             [$two64 - 5, $two64 + 5, 64]) {
    my ($lo, $hi, $k) = @$w;
    emit($fh, 'powerful_numbers', [$lo, $hi, $k], sub { @{powerful_numbers($lo, $hi, $k)} });
  }
  # Short windows by is_powerful, wide ones by the definition. Around 2^64 and
  # 2^70 (powers of two), p^2 for the first prime p above 2^32, and 3^41.
  my $p = Math::BigInt->new(next_prime(Math::BigInt->new('4294967296')));
  my $three41 = Math::BigInt->new(3)->bpow(41);
  my $two70 = Math::BigInt->new(2)->bpow(70);
  my @windows = (
    [$two64 - 20, $two64 + 20, [3, 4, 63, 64, 65]],
    [$p * $p - 10, $p * $p + 10, [2, 3]],
    [$three41 - 10, $three41 + 10, [2, 3, 41, 42]],
    [$two70 - 10, $two70 + 10, [2, 3, 7, 70]],
  );
  for my $w (@windows) {
    my ($lo, $hi, $ks) = @$w;
    my @ints;
    for (my $i = $lo->copy; $i <= $hi; $i++) { push @ints, $i->copy; }
    for my $k (@$ks) {
      my @want = grep { is_powerful($_, $k) } @ints;
      direct_comment($fh, $lo, $hi, $k, @want);
      emit($fh, 'powerful_numbers', [$lo, $hi, $k], sub { @want });
    }
  }
  for my $w ([$two64, $two64 * 2, 20], [$two64, $two64 * 4, 10], [$two64, $two70, 12]) {
    my ($lo, $hi, $k) = @$w;
    my @want = powerful_by_definition($lo, $hi, $k);
    direct_comment($fh, $lo, $hi, $k, @want);
    emit($fh, 'powerful_numbers', [$lo, $hi, $k], sub { @want });
  }
}

# A comment line when the direct PP.pm call dies or returns another list.
# For k = 2 the direct call walks every v up to rootint(hi, 3) with big
# integer arithmetic; near 2^70 that ran for 18 minutes (and returned the
# right list), so a k = 2 window past 2^66 gets no comment.
sub direct_comment {
  my ($fh, $lo, $hi, $k, @want) = @_;
  return if $k == 2 && $hi > Math::BigInt->new(2)->bpow(66);
  my $r = eval { join(' ', map { "$_" } @{powerful_numbers($lo->copy, $hi->copy, $k)}) };
  return if defined $r && $r eq join(' ', map { "$_" } @want);
  my $why = defined $r ? "returns [$r]" : 'dies: ' . ($@ =~ s/ at \S+ line \d+.*//sr);
  chomp $why;
  print $fh "# upstream finding 23: PP powerful_numbers($lo, $hi, $k) $why\n";
}
1;
