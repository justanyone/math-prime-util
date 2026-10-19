# Fixture family `bigint_arith`: the integer core on 65 to 1000 bit inputs, for the
# `bigint` feature (src/bigint/arith.rs). Replayed by tests/compat_bigint_arith.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1, Math::BigInt). The inputs come from a
# fixed linear congruential generator, so the file is the same on every host.
# Every value is written in full as MPU prints it.
use strict; use warnings;

my $lcg = 20_261_018;
# One 32-bit pseudo-random chunk. The product stays below 2^53, so plain doubles are exact.
sub _chunk32 {
  $lcg = ($lcg * 1_664_525 + 1_013_904_223) % 4_294_967_296;
  return $lcg;
}

# A Math::BigInt with exactly $bits bits (top bit set).
sub _big {
  my $bits = shift;
  my $n = Math::BigInt->new(0);
  for (my $have = 0; $have < $bits; $have += 32) {
    $n = ($n << 32) + _chunk32();
  }
  $n = $n % (Math::BigInt->new(1) << $bits);
  return $n | (Math::BigInt->new(1) << ($bits - 1));
}

sub _odd { my $n = _big(shift); return $n | 1; }

sub gen_bigint_arith {
  my $fh = shift;
  my @sizes = (65, 96, 128, 129, 200, 256, 333, 512, 777, 1000);
  my $g65 = _big(65);

  # --- gcd / lcm: coprime-looking pairs and pairs with a shared 65-bit factor
  for my $bits (@sizes) {
    my ($a, $b) = (_big($bits), _big($bits));
    my ($ga, $gb) = ($g65 * _big($bits), $g65 * _big(65));
    for my $ab ([$a, $b], [$ga, $gb], [$a, 0], [0, $a]) {
      my ($x, $y) = @$ab;
      emit($fh, 'gcd', [$x, $y], sub { gcd("$x", "$y") });
      emit($fh, 'lcm', [$x, $y], sub { lcm("$x", "$y") });
    }
  }
  # --- gcdext: (g, u, v) as in the int family; a few signed rows
  for my $bits (@sizes) {
    my ($a, $b) = (_big($bits), _big(65));
    emit($fh, 'gcdext', [$a, $b], sub { my ($u, $v, $g) = gcdext("$a", "$b"); ($g, $u, $v) });
    my ($ga, $gb) = ($g65 * _big($bits), $g65 * _big(65));
    emit($fh, 'gcdext', [$ga, $gb], sub { my ($u, $v, $g) = gcdext("$ga", "$gb"); ($g, $u, $v) });
  }
  for my $ab ([-_big(65), _big(65)], [_big(200), -_big(96)], [-_big(128), -_big(128)], [_big(100), 0], [0, -_big(100)]) {
    my ($a, $b) = @$ab;
    emit($fh, 'gcdext', [$a, $b], sub { my ($u, $v, $g) = gcdext("$a", "$b"); ($g, $u, $v) });
  }
  # --- modular: invmod, divmod, addmod, mulmod, powmod
  for my $bits (@sizes) {
    my $n = _odd($bits);
    my ($a, $b) = (_big($bits), _big(65));
    my $big_a = _big(1000);
    emit($fh, 'invmod', [$a, $n], sub { invmod("$a", "$n") });
    emit($fh, 'invmod', [$big_a, $n], sub { invmod("$big_a", "$n") });
    emit($fh, 'invmod', [$a * 3, $n * 3], sub { my ($x, $y) = ($a * 3, $n * 3); invmod("$x", "$y") });
    emit($fh, 'divmod', [$a, $b, $n], sub { divmod("$a", "$b", "$n") });
    emit($fh, 'divmod', [$a, $b * 3, $n * 3], sub { my ($x, $y) = ($b * 3, $n * 3); divmod("$a", "$x", "$y") });
    emit($fh, 'addmod', [$a, $b, $n], sub { addmod("$a", "$b", "$n") });
    emit($fh, 'addmod', [$big_a, $big_a, $n], sub { addmod("$big_a", "$big_a", "$n") });
    emit($fh, 'mulmod', [$a, $b, $n], sub { mulmod("$a", "$b", "$n") });
    emit($fh, 'mulmod', [$big_a, $big_a, $n], sub { mulmod("$big_a", "$big_a", "$n") });
    emit($fh, 'powmod', [$a, $b, $n], sub { powmod("$a", "$b", "$n") });
    emit($fh, 'powmod', [$a, 0, $n], sub { powmod("$a", 0, "$n") });
    emit($fh, 'powmod', [$big_a, $n, $n], sub { powmod("$big_a", "$n", "$n") });
  }
  for my $f ('invmod', 'addmod', 'mulmod', 'powmod') {
    my $a = _big(100);
    my @args = $f eq 'invmod' ? ($a) : ($a, 7);
    emit($fh, $f, [@args, 0], sub { no strict 'refs'; &$f(map { "$_" } @args, 0) });
    emit($fh, $f, [@args, 1], sub { no strict 'refs'; &$f(map { "$_" } @args, 1) });
  }
  { my $a = _big(100); emit($fh, 'divmod', [$a, 7, 0], sub { divmod("$a", 7, 0) }); emit($fh, 'divmod', [$a, 7, 1], sub { divmod("$a", 7, 1) }); }
  # --- roots, logs and powers
  for my $bits (@sizes) {
    my $r = _big($bits);
    my $sq = $r * $r;
    for my $n ($sq - 1, $sq, $sq + 1) {
      emit($fh, 'sqrtint', [$n], sub { sqrtint("$n") });
      emit($fh, 'is_square', [$n], sub { is_square("$n") ? 1 : 0 });
    }
    my $n = _big($bits);
    for my $k (2, 3, 5, 7, 13, 64, 65) {
      emit($fh, 'rootint', [$n, $k], sub { rootint("$n", $k) });
      emit($fh, 'is_power', [$n, $k], sub { is_power("$n", $k) ? 1 : 0 });
    }
    my $root = _big(23);
    for my $k (3, 7, 29) {
      my $p = $root->copy->bpow($k);
      emit($fh, 'rootint', [$p, $k], sub { rootint("$p", $k) });
      emit($fh, 'is_power', [$p, $k], sub { is_power("$p", $k) ? 1 : 0 });
      emit($fh, 'is_power', [$p - 1, $k], sub { my $m = $p - 1; is_power("$m", $k) ? 1 : 0 });
      emit($fh, 'is_power_exp', [$p], sub { is_power("$p") });
    }
    emit($fh, 'is_power_exp', [$n], sub { is_power("$n") });
    emit($fh, 'is_power_exp', [$sq], sub { is_power("$sq") });
    for my $b (2, 3, 10, _big(65)) {
      emit($fh, 'logint', [$n, $b], sub { logint("$n", "$b") });
    }
  }
  for my $e (6, 12, 30) {
    my $p = Math::BigInt->new(6)->bpow($e * 4);   # 2^(4e) * 3^(4e): exponent gcd is 4e
    emit($fh, 'is_power_exp', [$p], sub { is_power("$p") });
    my $q = Math::BigInt->new(2)->bpow(3 * $e) * Math::BigInt->new(3)->bpow(2 * $e);
    emit($fh, 'is_power_exp', [$q], sub { is_power("$q") });
  }
  { my $two = Math::BigInt->new(2)->bpow(1000);
    emit($fh, 'is_power_exp', [$two], sub { is_power("$two") });
    emit($fh, 'logint', [$two, 2], sub { logint("$two", 2) });
    emit($fh, 'logint', [$two - 1, 2], sub { my $m = $two - 1; logint("$m", 2) });
    emit($fh, 'rootint', [$two, 1000], sub { rootint("$two", 1000) });
    emit($fh, 'rootint', [$two, 999], sub { rootint("$two", 999) });
    emit($fh, 'rootint', [$two, 0], sub { rootint("$two", 0) });
    emit($fh, 'logint', [$two, 1], sub { logint("$two", 1) });
    emit($fh, 'logint', [0, 2], sub { logint(0, 2) });
  }
  # --- jacobi (kronecker with an odd modulus) and kronecker
  for my $bits (@sizes) {
    my $n = _odd($bits);
    my $a = _big($bits);
    emit($fh, 'jacobi', [$a, $n], sub { kronecker("$a", "$n") });
    emit($fh, 'jacobi', [-$a, $n], sub { my $m = -$a; kronecker("$m", "$n") });
    emit($fh, 'jacobi', [$a * $a, $n], sub { my $s = $a * $a; kronecker("$s", "$n") });
    emit($fh, 'jacobi', [$n * 5, $n], sub { my $m = $n * 5; kronecker("$m", "$n") });
    my $e = $n * 2;
    emit($fh, 'kronecker', [$a, $e], sub { kronecker("$a", "$e") });
    emit($fh, 'kronecker', [$a | 1, $e], sub { my $o = $a | 1; kronecker("$o", "$e") });
    emit($fh, 'kronecker', [$a | 1, $e * 4], sub { my ($o, $m) = ($a | 1, $e * 4); kronecker("$o", "$m") });
    emit($fh, 'kronecker', [-($a | 1), -$n], sub { my ($o, $m) = (-($a | 1), -$n); kronecker("$o", "$m") });
    emit($fh, 'kronecker', [$a, -$e], sub { my $m = -$e; kronecker("$a", "$m") });
  }
  for my $a (_big(65), -_big(65), 1, -1) {
    emit($fh, 'kronecker', [$a, 0], sub { kronecker("$a", 0) });
  }
  # --- binomial, factorial, valuation
  my ($b65, $b100) = (_big(65), _big(100));
  for my $nk ([_big(65), 2], [_big(65), 30], [_big(96), 7], [_big(128), 20], [_big(200), 3], [_big(1000), 1], [_big(1000), 0],
              [300, 150], [1000, 500], [2000, 7], [$b65, $b65], [$b65, $b65 + 1], [$b100, $b100 - 20]) {
    my ($n, $k) = @$nk;
    emit($fh, 'binomial', [$n, $k], sub { binomial("$n", "$k") });
  }
  emit($fh, 'factorial', [$_], sub { factorial($_) }) for (21, 25, 40, 100, 170, 300);
  for my $bits (65, 128, 333, 1000) {
    my $m = _odd($bits);
    for my $k (2, 3, 10, 65537) {
      my $v = ($bits % 7) + 3;
      my $n = $m * Math::BigInt->new($k)->bpow($v);
      $n = $n * 3 if $k == 2 && $m % 3 == 0;
      emit($fh, 'valuation', [$n, $k], sub { valuation("$n", $k) });
    }
    emit($fh, 'valuation', [$m, $m], sub { valuation("$m", "$m") });
    emit($fh, 'valuation', [$m * $m, $m], sub { my $s = $m * $m; valuation("$s", "$m") });
  }
  { my $n = _big(100);
    emit($fh, 'valuation', [$n, 1], sub { valuation("$n", 1) });
    emit($fh, 'valuation', [$n, 0], sub { valuation("$n", 0) });
    emit($fh, 'valuation', [0, $n], sub { valuation(0, "$n") });
  }
}

1;
