# Fixture family `bigint_modular`: modular number theory on 65 to 500 bit inputs,
# for the `bigint` feature (src/bigint/modular.rs). Replayed by
# tests/compat_bigint_modular.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1, Math::BigInt). The inputs come from a
# fixed linear congruential generator, so the file is the same on every host.
# Composite moduli are products of small primes (below 2^20) and one large prime,
# so that both the Perl oracle and the port can factor them; group orders for
# znorder, znprimroot, is_primitive_root and znlog are smooth for the same
# reason. znlog in pure Perl is slow on large inputs (up to a minute a call
# at 500 bits), so most of its moduli stay at 65 to 128 bits and three rows
# per size cover 200 to 500 bits, at the end of the file.
use strict; use warnings;

my $mod_lcg = 20_261_018;
# One 32-bit pseudo-random chunk. The product stays below 2^53, so plain doubles are exact.
sub _mod_chunk32 {
  $mod_lcg = ($mod_lcg * 1_664_525 + 1_013_904_223) % 4_294_967_296;
  return $mod_lcg;
}

# A Math::BigInt with exactly $bits bits (top bit set).
sub _mod_big {
  my $bits = shift;
  my $n = Math::BigInt->new(0);
  for (my $have = 0; $have < $bits; $have += 32) {
    $n = ($n << 32) + _mod_chunk32();
  }
  $n = $n % (Math::BigInt->new(1) << $bits);
  return $n | (Math::BigInt->new(1) << ($bits - 1));
}

sub _mod_odd { my $n = _mod_big(shift); return $n | 1; }

# A prime of about $bits bits; with $residue, one that is $residue mod 8.
sub _mod_prime {
  my ($bits, $residue) = @_;
  my $p = Math::BigInt->new(next_prime("" . _mod_odd($bits)));
  while (defined $residue && $p % 8 != $residue) {
    $p = Math::BigInt->new(next_prime("$p"));
  }
  return $p;
}

# A small prime of about $bits (at most 32) bits.
sub _mod_small_prime {
  my $bits = shift;
  return next_prime((_mod_chunk32() % (1 << ($bits - 1))) + (1 << ($bits - 1)));
}

# A prime p of about $bits bits with p - 1 a product of primes below 2^16.
my @mod_smooth_primes = @{primes(32768, 65536)};
sub _mod_smooth_prime {
  my $bits = shift;
  while (1) {
    my $m = Math::BigInt->new(2);
    $m *= $mod_smooth_primes[_mod_chunk32() % @mod_smooth_primes] while length($m->as_bin) - 2 < $bits - 1;
    my $p = $m + 1;
    return $p if is_prime("$p");
  }
}

# A composite of about $bits bits: a large prime times three small primes.
sub _mod_composite {
  my $bits = shift;
  return _mod_prime($bits - 57) * _mod_small_prime(19) * _mod_small_prime(19) * _mod_small_prime(19);
}

sub _mod_pow { my ($b, $e) = @_; return Math::BigInt->new($b)->bpow($e); }

sub _mod_emit_list {
  my ($fh, $fn, $args, $code) = @_;
  emit($fh, $fn, $args, sub { my @r = $code->(); return @r ? join(' ', @r) : undef });
}

sub gen_bigint_modular {
  my $fh = shift;
  my @sizes = (65, 96, 128, 200, 256, 333, 500);
  my (%prime, %smooth);
  $prime{$_} = _mod_prime($_) for @sizes;
  $smooth{$_} = _mod_smooth_prime($_) for (65, 96, 128, 200, 333, 500);
  my @small = (_mod_small_prime(20), _mod_small_prime(20), _mod_small_prime(17));

  # --- sqrtmod, rootmod(a, 2, n), allsqrtmod: primes of each residue class mod 8,
  # prime powers, powers of two, composites
  my @sqrt_moduli;
  for my $bits (@sizes) {
    push @sqrt_moduli, $prime{$bits};
    push @sqrt_moduli, _mod_prime($bits, 1), _mod_prime($bits, 5) if $bits == 128 || $bits == 333;
  }
  push @sqrt_moduli, _mod_pow($small[0], 4), _mod_pow($small[0], 7), _mod_pow($small[2], 12),
                     _mod_pow(2, 65), _mod_pow(2, 100), _mod_pow(2, 200),
                     _mod_composite(96), _mod_composite(200), _mod_composite(500),
                     _mod_pow(2, 20) * $prime{65}, _mod_pow(3, 30) * _mod_pow(2, 7) * $prime{96},
                     _mod_pow(2, 65) * 3 * 5 * 7 * 11 * 13;
  for my $n (@sqrt_moduli) {
    my $bits = length($n->as_bin) - 2;
    my $x = _mod_big($bits - 1) % $n;
    my $square = ($x * $x) % $n;
    my $random = _mod_big($bits) % $n;
    my $root4 = (4 * $x * $x) % $n;
    # Zero has p^(k/2) roots modulo p^k: only prime moduli take it here.
    my @values = ($square, $random, $root4, $small[0] * $x, 1, $n - 1);
    push @values, 0 if is_prime("$n");
    for my $a (@values) {
      emit($fh, 'sqrtmod', [$a, $n], sub { sqrtmod("$a", "$n") });
      emit($fh, 'rootmod', [$a, 2, $n], sub { rootmod("$a", 2, "$n") });
      _mod_emit_list($fh, 'allsqrtmod', [$a, $n], sub { allsqrtmod("$a", "$n") });
    }
  }
  { my $n = _mod_pow(2, 20) * $prime{65};
    _mod_emit_list($fh, 'allsqrtmod', [0, $n], sub { allsqrtmod(0, "$n") });
    my $m = _mod_pow(3, 4) * $prime{65};
    my $a = 36;
    _mod_emit_list($fh, 'allsqrtmod', [$a, $m], sub { allsqrtmod("$a", "$m") });
    emit($fh, 'sqrtmod', [$a, $m], sub { sqrtmod("$a", "$m") });
  }
  for my $n (0, 1, 2) {
    my $a = _mod_big(100);
    emit($fh, 'sqrtmod', [$a, $n], sub { sqrtmod("$a", $n) });
    emit($fh, 'rootmod', [$a, 3, $n], sub { rootmod("$a", 3, $n) });
    _mod_emit_list($fh, 'allsqrtmod', [$a, $n], sub { allsqrtmod("$a", $n) });
  }

  # --- rootmod with k > 2: primes, prime powers, composites, edge exponents
  my @root_moduli = (map({ $prime{$_} } 65, 128, 256, 500), _mod_prime(200, 1),
                     _mod_pow($small[0], 4), _mod_pow(2, 70), _mod_pow(3, 50),
                     _mod_composite(96), _mod_composite(200), _mod_pow(2, 20) * $prime{65});
  for my $n (@root_moduli) {
    my $bits = length($n->as_bin) - 2;
    # A prime of n that divides k stalls both Hensel lifting and PP's allrootmod,
    # and PP's allrootmod on a prime power does not finish with a large k.
    my $square_free = is_square_free("$n");
    my @exponents = $square_free ? (3, 5, 6, 7, 1048583) : (3, 5, 6, 7);
    for my $k (@exponents) {
      next unless gcd($k, "$n") == 1;
      my $x = _mod_big($bits - 1) % $n;
      my $random = _mod_big($bits) % $n;
      # A base divisible by a prime of a prime-power modulus is slow in PP's allrootmod.
      unless ($square_free) {
        $x += 1 while gcd("$x", "$n") != 1;
        $random += 1 while gcd("$random", "$n") != 1;
      }
      my $power = Math::BigInt->new(powmod("$x", $k, "$n"));
      for my $a ($power, $random) {
        emit($fh, 'rootmod', [$a, $k, $n], sub { rootmod("$a", $k, "$n") });
      }
    }
  }
  { my $n = $prime{128};
    my $k = _mod_big(65);
    my $a = _mod_big(100);
    emit($fh, 'rootmod', [$a, $k, $n], sub { rootmod("$a", "$k", "$n") });
    emit($fh, 'rootmod', [$a, 0, $n], sub { rootmod("$a", 0, "$n") });
    emit($fh, 'rootmod', [1, 0, $n], sub { rootmod(1, 0, "$n") });
    emit($fh, 'rootmod', [$a, 1, $n], sub { rootmod("$a", 1, "$n") });
    emit($fh, 'rootmod', [0, 3, $n], sub { rootmod(0, 3, "$n") });
    emit($fh, 'rootmod', [0, 0, $n], sub { rootmod(0, 0, "$n") });
    emit($fh, 'rootmod', [_mod_pow(3, 60), 5, $n], sub { rootmod("" . _mod_pow(3, 60), 5, "$n") });
  }

  # --- znorder, znprimroot, is_primitive_root: smooth primes and their
  # multiples, prime powers of small primes, composites without roots
  my @zn_moduli = (map({ $smooth{$_} } 65, 96, 128, 200, 333, 500),
                   2 * $smooth{128}, 4 * $smooth{128}, $smooth{65} * $small[0],
                   _mod_pow($small[0], 4), 2 * _mod_pow($small[0], 7), _mod_pow($small[2], 12),
                   _mod_pow(2, 70), _mod_pow(2, 20) * $prime{65}, _mod_pow(3, 50));
  for my $n (@zn_moduli) {
    my $bits = length($n->as_bin) - 2;
    emit($fh, 'znprimroot', [$n], sub { znprimroot("$n") });
    my $g = znprimroot("$n");
    my @bases = (2, 3, $n - 1, _mod_big($bits - 1) % $n, $small[1], $small[0] * 2);
    push @bases, Math::BigInt->new($g), Math::BigInt->new($g) ** 2 if defined $g;
    for my $a (@bases) {
      emit($fh, 'znorder', [$a, $n], sub { znorder("$a", "$n") });
      emit($fh, 'is_primitive_root', [$a, $n], sub { is_primitive_root("$a", "$n") });
    }
  }
  for my $n (0, 1, 2, 3, 4) {
    emit($fh, 'znprimroot', [$n], sub { znprimroot($n) });
    my $a = _mod_big(70);
    emit($fh, 'znorder', [$a, $n], sub { znorder("$a", $n) });
    emit($fh, 'is_primitive_root', [$a, $n], sub { is_primitive_root("$a", $n) });
  }
  { my $n = $smooth{200};
    emit($fh, 'znorder', [0, $n], sub { znorder(0, "$n") });
    emit($fh, 'znorder', [1, $n], sub { znorder(1, "$n") });
    emit($fh, 'znorder', [$n, $n], sub { znorder("$n", "$n") });
    emit($fh, 'znorder', [$n + 2, $n], sub { my $a = $n + 2; znorder("$a", "$n") });
    emit($fh, 'is_primitive_root', [0, $n], sub { is_primitive_root(0, "$n") });
    emit($fh, 'is_primitive_root', [1, $n], sub { is_primitive_root(1, "$n") });
    emit($fh, 'is_primitive_root', [$n, $n], sub { is_primitive_root("$n", "$n") });
  }

  # --- znlog: smooth primes of 65 to 128 bits, a generator and a non-generator
  # base, bases that share a factor with the modulus
  for my $bits (65, 96, 128) {
    my $p = $smooth{$bits};
    my $g = Math::BigInt->new(znprimroot("$p"));
    my $h = ($g * $g) % $p;
    for my $x (_mod_big($bits - 1), _mod_big(40), 5, 1, 0) {
      my $a = Math::BigInt->new(powmod("$g", "$x", "$p"));
      emit($fh, 'znlog', [$a, $g, $p], sub { znlog("$a", "$g", "$p") });
      my $b = Math::BigInt->new(powmod("$h", "$x", "$p"));
      emit($fh, 'znlog', [$b, $h, $p], sub { znlog("$b", "$h", "$p") });
      emit($fh, 'znlog', [$b, $g, $p], sub { znlog("$b", "$g", "$p") });
    }
    my $outside = ($g ** 3) % $p;
    emit($fh, 'znlog', [$outside, $h, $p], sub { znlog("$outside", "$h", "$p") });
    emit($fh, 'znlog', [$g, $h, $p], sub { znlog("$g", "$h", "$p") });
    emit($fh, 'znlog', [$p - 1, $g, $p], sub { my $a = $p - 1; znlog("$a", "$g", "$p") });
    emit($fh, 'znlog', [0, $g, $p], sub { znlog(0, "$g", "$p") });
  }
  { my $n = $smooth{65} * $small[0];
    my $g = Math::BigInt->new(znprimroot("$smooth{65}"));
    my $x = _mod_big(60);
    my $a = Math::BigInt->new(powmod("$g", "$x", "$n"));
    emit($fh, 'znlog', [$a, $g, $n], sub { znlog("$a", "$g", "$n") });
    emit($fh, 'znlog', [$a + 1, $g, $n], sub { my $b = $a + 1; znlog("$b", "$g", "$n") });
    my $m = _mod_pow(2, 70);
    emit($fh, 'znlog', [_mod_pow(2, 10), 2, $m], sub { znlog(1024, 2, "$m") });
    emit($fh, 'znlog', [0, 2, $m], sub { znlog(0, 2, "$m") });
    my $t = Math::BigInt->new(powmod(3, "" . _mod_big(60), "$m"));
    emit($fh, 'znlog', [$t, 3, $m], sub { znlog("$t", 3, "$m") });
    emit($fh, 'znlog', [$t, 5, $m], sub { znlog("$t", 5, "$m") });
    emit($fh, 'znlog', [$t, 0, $m], sub { znlog("$t", 0, "$m") });
    emit($fh, 'znlog', [0, 0, $m], sub { znlog(0, 0, "$m") });
    emit($fh, 'znlog', [1, 0, $m], sub { znlog(1, 0, "$m") });
    emit($fh, 'znlog', [$t, 3, 0], sub { znlog("$t", 3, 0) });
    emit($fh, 'znlog', [$t, 3, 1], sub { znlog("$t", 3, 1) });
  }

  # --- chinese: coprime big moduli, non-coprime consistent and inconsistent
  # pairs, a zero modulus, one pair, no pairs
  my @chinese_sets = (
    [[_mod_big(64), $prime{65}], [_mod_big(90), $prime{96}], [_mod_big(120), $prime{128}]],
    [[_mod_big(64), $prime{200}], [_mod_big(90), $prime{333}], [_mod_big(120), $prime{500}], [7, 2]],
    [[_mod_big(100), $prime{128}]],
    [[_mod_big(100), 1], [_mod_big(100), $prime{65}]],
    [[_mod_big(100), $prime{128}], [_mod_big(100), 0]],
  );
  { my $x = _mod_big(150);
    my $m1 = _mod_pow(2, 70) * 3;
    my $m2 = _mod_pow(2, 65) * 5 * 7;
    push @chinese_sets, [[$x % $m1, $m1], [$x % $m2, $m2]],
                        [[$x % $m1, $m1], [($x + 1) % $m2, $m2]],
                        [[$x % $m1, $m1], [$x % $m2, $m2], [$x % $prime{200}, $prime{200}], [$x % $m1, $m1]];
  }
  for my $set (@chinese_sets) {
    my @args = map { @$_ } @$set;
    emit($fh, 'chinese', \@args, sub { chinese(map { [map { "$_" } @$_] } @$set) });
  }
  emit($fh, 'chinese', [], sub { chinese() });

  # --- factorialmod: prime moduli with small n and with n near m (Wilson),
  # composite moduli with and without the zero short cut
  for my $bits (65, 128, 333, 500) {
    my $m = $prime{$bits};
    for my $n (0, 1, 10, 100, 999, 1000, 1001, 5000, $m - 1, $m - 2, $m - 3, $m - 10, $m - 1001, $m) {
      emit($fh, 'factorialmod', [$n, $m], sub { factorialmod("$n", "$m") });
    }
  }
  for my $m (_mod_pow(2, 70) * 27, _mod_pow(2, 20) * $prime{65}, _mod_pow(2, 65), _mod_composite(200),
             3 * _mod_pow(5, 40)) {
    for my $n (10, 50, 101, 200, 1000, 2500) {
      emit($fh, 'factorialmod', [$n, $m], sub { factorialmod($n, "$m") });
    }
  }
  emit($fh, 'factorialmod', [20, 0], sub { factorialmod(20, 0) });
  { my $n = _mod_big(70);
    emit($fh, 'factorialmod', [$n, 1], sub { factorialmod("$n", 1) });
  }

  # --- binomialmod: large n, small k, prime and composite moduli
  my @binom_moduli = (map({ $prime{$_} } 65, 128, 333, 500), _mod_pow(2, 70), _mod_pow(3, 50),
                      _mod_pow(2, 10) * 27 * $prime{65}, _mod_composite(200), 2, 4, _mod_pow(2, 65) * 3);
  for my $m (@binom_moduli) {
    for my $bits (65, 200, 500) {
      my $n = _mod_big($bits);
      for my $k (0, 1, 2, 3, 50, 200, $n - 1, $n, $n + 1) {
        emit($fh, 'binomialmod', [$n, $k, $m], sub { binomialmod("$n", "$k", "$m") });
      }
    }
  }
  { my $m = $prime{128};
    for my $n (0, 1, 7, 100, 1000) {
      for my $k (0, 1, 2, 3, 50, 100) {
        emit($fh, 'binomialmod', [$n, $k, $m], sub { binomialmod($n, $k, "$m") });
      }
    }
    for my $m (0, 1) {
      my $n = _mod_big(70);
      emit($fh, 'binomialmod', [$n, 5, $m], sub { binomialmod("$n", 5, $m) });
    }
    my $n = _mod_big(100);
    my $k = _mod_big(80);
    emit($fh, 'binomialmod', [$n, $k, 2], sub { binomialmod("$n", "$k", 2) });
    emit($fh, 'binomialmod', [$n, $n - 3, 7], sub { my $j = $n - 3; binomialmod("$n", "$j", 7) });
    emit($fh, 'binomialmod', [$n, 3000, 125], sub { binomialmod("$n", 3000, 125) });
    emit($fh, 'binomialmod', [$n, 3000, 128], sub { binomialmod("$n", 3000, 128) });
    emit($fh, 'binomialmod', [$n, 3000, 11 * 125], sub { binomialmod("$n", 3000, 1375) });
  }

  # --- lucasumod, lucasvmod: (P, Q) pairs on big k and odd or even n
  for my $bits (65, 128, 333, 500) {
    my $n = _mod_odd($bits);
    my $e = _mod_big($bits) * 2;
    my $k = _mod_big($bits);
    for my $pq ([1, -1], [3, -1], [1, 1], [5, 1], [2, 1], [4, 4], [-3, -5], [7, 11], [-$n, 3], [3, $n - 1]) {
      my ($p, $q) = @$pq;
      for my $m ($n, $e) {
        emit($fh, 'lucasumod', [$p, $q, $k, $m], sub { lucasumod("$p", "$q", "$k", "$m") });
        emit($fh, 'lucasvmod', [$p, $q, $k, $m], sub { lucasvmod("$p", "$q", "$k", "$m") });
      }
    }
    for my $small_k (0, 1, 2) {
      emit($fh, 'lucasumod', [3, -1, $small_k, $n], sub { lucasumod(3, -1, $small_k, "$n") });
      emit($fh, 'lucasvmod', [3, -1, $small_k, $n], sub { lucasvmod(3, -1, $small_k, "$n") });
    }
    emit($fh, 'lucasumod', [3, -1, $k, 1], sub { lucasumod(3, -1, "$k", 1) });
    emit($fh, 'lucasvmod', [3, -1, $k, 1], sub { lucasvmod(3, -1, "$k", 1) });
  }

  # --- is_carmichael: Chernick numbers (6k+1)(12k+1)(18k+1) below and above
  # the 50-digit line where pure Perl stops factoring, primes, products of
  # two primes, pseudoprimes, random odd numbers
  # k is a multiple of the primes to 23 so that 6k+1, 12k+1 and 18k+1 are 1 mod each.
  my $wheel = 5 * 7 * 11 * 13 * 17 * 19 * 23;
  for my $kbits (20, 22, 53, 100) {
    my ($found, $k) = (0, _mod_big($kbits));
    $k -= $k % $wheel;
    while ($found < ($kbits > 60 ? 1 : 2)) {
      $k += $wheel;
      my ($a, $b, $c) = (6 * $k + 1, 12 * $k + 1, 18 * $k + 1);
      next unless is_prime("$a") && is_prime("$b") && is_prime("$c");
      my $n = $a * $b * $c;
      emit($fh, 'is_carmichael', [$n], sub { is_carmichael("$n") });
      emit($fh, 'is_carmichael', [$n * 5], sub { my $m = $n * 5; is_carmichael("$m") });
      emit($fh, 'is_carmichael', [$n + 2], sub { my $m = $n + 2; is_carmichael("$m") });
      $found++;
    }
  }
  for my $n (map({ $prime{$_} } @sizes), $prime{65} * $prime{65}, $prime{65} * $prime{96}, $prime{200} * $prime{333},
             _mod_pow(2, 67) - 1, _mod_pow(2, 257) - 1, _mod_pow(2, 127) - 1,
             (_mod_pow(2, 74) - 1) / 3, (_mod_pow(2, 254) - 1) / 3, _mod_pow(2, 70) + 1,
             _mod_odd(65), _mod_odd(200), _mod_odd(500), _mod_big(100) * 2, 561, 1105, 1729, 560, 2465,
             $smooth{65} * 3, 41 * 61 * 101 * 1601 * $prime{65}) {
    emit($fh, 'is_carmichael', [$n], sub { is_carmichael("$n") });
  }

  # --- is_quasi_carmichael: two primes p * q with q a safe prime, three or
  # four primes with small cofactors, primes, squares, composites with a
  # square factor
  for my $bits (65, 96, 128, 160, 200) {
    my $p = _mod_small_prime(20);
    my $q;
    while (1) {
      my $r = _mod_prime($bits - 21);
      $q = 2 * $r + 1;
      last if is_prime("$q");
    }
    my $n = $p * $q;
    emit($fh, 'is_quasi_carmichael', [$n], sub { is_quasi_carmichael("$n") });
    my $m = 3 * $q;
    emit($fh, 'is_quasi_carmichael', [$m], sub { is_quasi_carmichael("$m") });
  }
  for my $bits (65, 128, 256, 500) {
    my ($p1, $p2);
    while (1) {
      ($p1, $p2) = (_mod_small_prime(20), _mod_small_prime(19));
      my $t = $p1 * $p2 - 1;
      last if vecall { $_ < 65536 } factor($t);
    }
    my $q = _mod_prime($bits - 37);
    my $n = $p1 * $p2 * $q;
    emit($fh, 'is_quasi_carmichael', [$n], sub { is_quasi_carmichael("$n") });
    my $m = 3 * $p1 * $q;
    emit($fh, 'is_quasi_carmichael', [$m], sub { is_quasi_carmichael("$m") });
    my $four = 3 * 5 * $p1 * $q;
    emit($fh, 'is_quasi_carmichael', [$four], sub { is_quasi_carmichael("$four") });
  }
  for my $n ($prime{65}, $prime{200}, $prime{65} * $prime{65}, _mod_pow(2, 70), 9 * $prime{128},
             4 * $prime{128}, 121 * $prime{65}, 2 * $prime{65}, 35, 77, 1121, 0, 1, 34) {
    emit($fh, 'is_quasi_carmichael', [$n], sub { is_quasi_carmichael("$n") });
  }

  # --- znlog on the 200 to 500 bit smooth primes: last, so that the inputs
  # above keep their values.
  for my $bits (200, 333, 500) {
    my $p = $smooth{$bits};
    my $g = Math::BigInt->new(znprimroot("$p"));
    my $h = ($g * $g) % $p;
    my $x = _mod_big($bits - 1);
    my $a = Math::BigInt->new(powmod("$g", "$x", "$p"));
    emit($fh, 'znlog', [$a, $g, $p], sub { znlog("$a", "$g", "$p") });
    my $b = Math::BigInt->new(powmod("$h", "$x", "$p"));
    emit($fh, 'znlog', [$b, $h, $p], sub { znlog("$b", "$h", "$p") });
    emit($fh, 'znlog', [$b, $g, $p], sub { znlog("$b", "$g", "$p") });
  }
}

1;
