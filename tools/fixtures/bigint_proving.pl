# Fixture family `bigint_proving`: primality proofs and certificates on 65 to
# 664 bit inputs, for the `bigint` feature (src/bigint/proving.rs). Replayed
# by tests/compat_bigint_proving.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1, Math::BigInt; Math::Prime::Util::GMP
# is not installed, so the Perl prover is the BLS75 n-1 proof of
# PrimalityProving.pm and never ECPP). The inputs come from a fixed linear
# congruential generator, so the file is the same on every host.
#
# Certificates are multi-line text with spaces. In the args and result columns
# a space is written as `~` and a newline as `|`; the replay test decodes them.
# A call that takes longer than the alarm is skipped with a comment line.
use strict; use warnings;

my $prv_lcg = 20_261_019;
# One 32-bit pseudo-random chunk. The product stays below 2^53, so plain doubles are exact.
sub _prv_chunk32 {
  $prv_lcg = ($prv_lcg * 1_664_525 + 1_013_904_223) % 4_294_967_296;
  return $prv_lcg;
}

# A Math::BigInt with exactly $bits bits (top bit set).
sub _prv_big {
  my $bits = shift;
  my $n = Math::BigInt->new(0);
  for (my $have = 0; $have < $bits; $have += 32) {
    $n = ($n << 32) + _prv_chunk32();
  }
  $n = $n % (Math::BigInt->new(1) << $bits);
  return $n | (Math::BigInt->new(1) << ($bits - 1));
}

sub _prv_odd { my $n = _prv_big(shift); return $n | 1; }

sub _prv_prime { my $n = _prv_odd(shift); return Math::BigInt->new(next_prime("$n")); }

sub _prv_enc { my $s = shift; $s =~ s/ /~/g; $s =~ s/\n/\|/g; return $s; }

# Like emit(), with the certificate encoding and an alarm: a call that times
# out leaves a comment line instead of a row.
sub _prv_emit {
  my ($fh, $fn, $args, $code, $seconds) = @_;
  $seconds ||= 300;
  my @r;
  my $ok = eval {
    local $SIG{ALRM} = sub { die "alarm\n" };
    alarm $seconds;
    @r = $code->();
    alarm 0;
    1;
  };
  my $err = $@;
  alarm 0;
  if (!$ok && $err eq "alarm\n") {
    print $fh "# skipped (timeout after ${seconds}s): $fn ", join(' ', map { _prv_enc("$_") } @$args), "\n";
    return;
  }
  my $res = !$ok ? 'ERROR' : (!@r || !defined $r[0]) ? 'undef' : join(' ', map { _prv_enc("$_") } @r);
  print $fh join("\t", $fn, join(' ', map { _prv_enc("$_") } @$args), $res), "\n";
}

# The whole proving family on one input: status, certificate, and MPU's own
# verification of that certificate.
# MPU's is_provable_prime and prime_certificate are the two halves of
# is_provable_prime_with_cert, so one proof run feeds all three rows.
sub _prv_prove {
  my ($fh, $n, $seconds) = @_;
  my ($status, $cert);
  _prv_emit($fh, 'is_provable_prime_with_cert', [$n], sub {
    ($status, $cert) = is_provable_prime_with_cert("$n"); ($status, $cert) }, $seconds);
  return '' unless defined $status;
  _prv_emit($fh, 'is_provable_prime', [$n], sub { $status });
  _prv_emit($fh, 'prime_certificate', [$n], sub { $cert });
  if ($cert ne '') {
    _prv_emit($fh, 'verify_prime', [$cert], sub { verify_prime($cert) }, $seconds);
    # Tampered copies: a wrong target, a wrong factor, a missing terminator.
    my $bad = $cert; $bad =~ s/^(Proof for:\nN )(\d+)/$1 . (Math::BigInt->new($2) + 2)/me;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) }, $seconds) if $bad ne $cert;
    $bad = $cert; $bad =~ s/^(Q\[1\] )(\d+)/$1 . (Math::BigInt->new($2) + 2)/me;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) }, $seconds) if $bad ne $cert;
    $bad = $cert; $bad =~ s/^----\n//m;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) }, $seconds) if $bad ne $cert;
  }
  return $cert;
}

# Odd k with at most $kbits bits such that k * 2^m + 1 is prime: BLS75 proves
# these from the power of two alone.
sub _prv_proth {
  my ($kbits, $m) = @_;
  my $k = _prv_odd($kbits);
  my $base = Math::BigInt->new(1) << $m;
  while (1) {
    my $n = $k * $base + 1;
    return $n if is_prime("$n");
    $k += 2;
  }
}

# 2 * s * q + 1 prime with s a small random odd number and q a prime of $qbits
# bits: n - 1 = 2 * s * q is found by trial division and one BPSW test.
sub _prv_two_sq {
  my ($sbits, $qbits) = @_;
  my $s = _prv_odd($sbits);
  while (1) {
    my $q = _prv_prime($qbits);
    my $n = 2 * $s * $q + 1;
    return $n if is_prime("$n");
  }
}

# Hand-built certificates of the other MPU types for primes above 2^64,
# checked with MPU verify_prime: the port must accept and reject the same.
sub _prv_other_types {
  my $fh = shift;
  my $header = sub { my $n = shift; "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN $n\n\n" };
  # Lucas: n - 1 completely factored, a a primitive root. Build n = 2 * s * q + 1 with
  # small s so that factor() is quick in pure Perl.
  {
    my $n = _prv_two_sq(8, 60);
    my $nm1 = $n - 1;
    my %seen; my @f = grep { !$seen{$_}++ } factor($nm1);
    my $a;
    for my $try (2 .. 10000) {
      my $ap = Math::BigInt->new($try);
      next unless $ap->copy->bmodpow($nm1, $n) == 1;
      next if grep { $ap->copy->bmodpow($nm1 / $_, $n) == 1 } @f;
      $a = $try; last;
    }
    my $body = "Type Lucas\nN $n\n" . join('', map { "Q[" . ($_ + 1) . "] $f[$_]\n" } 0 .. $#f) . "A $a\n";
    my $cert = $header->($n) . $body;
    _prv_emit($fh, 'verify_prime', [$cert], sub { verify_prime($cert) });
    (my $bad = $cert) =~ s/^A \d+$/A 1/m;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    ($bad = $cert) =~ s/^Q\[1\] \d+\n//m;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    # Lowercase type and a comment line are accepted; a bad type name is not.
    ($bad = $cert) =~ s/^Type Lucas/# a comment\nType lucas/m;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    ($bad = $cert) =~ s/^Type Lucas/Type Lucky/m;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
  }
  # BLS3 and Pocklington: n = 4 * q + 1 with q a 63-bit prime, so that q is in
  # the native range (a q above 2^64 would need its own proof block) and n is
  # above it.
  {
    my $q = _prv_prime(63);
    $q = Math::BigInt->new(next_prime("$q")) until is_prime("" . (4 * $q + 1));
    my $n = 4 * $q + 1;
    my $a3;
    for my $try (2 .. 1000) {
      my $ap = Math::BigInt->new($try);
      if ($ap->copy->bmodpow(($n - 1) / 2, $n) == $n - 1) { $a3 = $try; last; }
    }
    my $cert = $header->($n) . "Type BLS3\nN $n\nQ $q\nA $a3\n";
    _prv_emit($fh, 'verify_prime', [$cert], sub { verify_prime($cert) });
    (my $bad = $cert) =~ s/^Type BLS3/Type Pocklington/m;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    ($bad = $cert) =~ s/^Q (\d+)/"Q " . (Math::BigInt->new($1) - 2)/me;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    # Pocklington: n = m * q + 1 with m even and m < q.
    my $m = 2 * _prv_odd(20);
    my $p = _prv_prime(60);
    $p = Math::BigInt->new(next_prime("$p")) until is_prime("" . ($m * $p + 1));
    $n = $m * $p + 1;
    my $ap;
    for my $try (2 .. 1000) {
      my $t = Math::BigInt->new($try);
      next unless $t->copy->bmodpow($n - 1, $n) == 1;
      next unless Math::BigInt::bgcd($t->copy->bmodpow($m, $n) - 1, $n) == 1;
      $ap = $try; last;
    }
    $cert = $header->($n) . "Type Pocklington\nN $n\nQ $p\nA $ap\n";
    _prv_emit($fh, 'verify_prime', [$cert], sub { verify_prime($cert) });
    ($bad = $cert) =~ s/^A \d+/A 1/m;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    ($bad = $cert) =~ s/^Type Pocklington/Type BLS3/m;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
  }
  # BLS15: n + 1 = 4 * q with q a 63-bit prime (native range, as above), Lucas
  # parameters with jacobi(D, n) = -1.
  {
    my $q = _prv_prime(63);
    $q = Math::BigInt->new(next_prime("$q")) until is_prime("" . (4 * $q - 1));
    my $n = 4 * $q - 1;
    my ($lp, $lq, $cert);
    for my $p (1 .. 200) {
      for my $qq (-50 .. 50) {
        next if $qq == 0;
        my $d = $p * $p - 4 * $qq;
        next unless kronecker($d, $n) == -1;
        next unless lucasvmod($p, $qq, ($n + 1) / 2 / $q, $n) != 0;
        next unless lucasvmod($p, $qq, ($n + 1) / 2, $n) == 0;
        ($lp, $lq) = ($p, $qq);
        last;
      }
      last if defined $lp;
    }
    if (defined $lp) {
      $cert = $header->($n) . "Type BLS15\nN $n\nQ $q\nLP $lp\nLQ $lq\n";
      _prv_emit($fh, 'verify_prime', [$cert], sub { verify_prime($cert) });
      (my $bad = $cert) =~ s/^LP (-?\d+)/"LP " . (Math::BigInt->new($1) + 1)/me;
      _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
      ($bad = $cert) =~ s/^Q (\d+)/"Q " . (Math::BigInt->new($1) + 2)/me;
      _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    }
  }
}

# One ECPP block for a prime n with discriminant -4 (curve y^2 = x^3 + a x) or
# -3 (y^2 = x^3 + b): Cornacchia gives 4n = t^2 + |D| v^2, the candidate orders
# are tried on a point, and the order must be k * q with q prime, q != m and
# q > (n^(1/4) + 1)^2. Returns (block text, q) or () when n gives none.
sub _prv_ecpp_block {
  my ($n, $d) = @_;
  my $four_n = 4 * $n;
  my $root = sqrtmod("" . ($n + $d), "$n");
  return () unless defined $root;
  my $x0 = Math::BigInt->new("$root");
  $x0 = $n - $x0 if (($x0 + abs($d)) % 2) != 0;
  my ($a, $b) = (2 * $n, $x0);
  my $limit = $four_n->copy->bsqrt;
  while ($b > $limit) { ($a, $b) = ($b, $a % $b); }
  my $t = $b;
  my $rem = $four_n - $t * $t;
  return () unless ($rem % abs($d)) == 0;
  my $v2 = $rem / abs($d);
  my $v = $v2->copy->bsqrt;
  return () unless $v * $v == $v2;
  my @orders = ($n + 1 + $t, $n + 1 - $t);
  if ($d == -4) { push @orders, $n + 1 + 2 * $v, $n + 1 - 2 * $v; }
  else { push @orders, $n + 1 + ($t + 3 * $v) / 2, $n + 1 - ($t + 3 * $v) / 2,
                       $n + 1 + ($t - 3 * $v) / 2, $n + 1 - ($t - 3 * $v) / 2; }
  my $qmin = $n->copy->broot(4)->badd(1)->bpow(2);
  require Math::Prime::Util::ECAffinePoint;
  for my $m (@orders) {
    my @f = factor($m);
    my $q = Math::BigInt->new($f[-1]);
    next unless $q > $qmin && $q < $n && $q != $m;
    for my $coef (1 .. 40) {
      my ($ca, $cb) = $d == -4 ? ($coef, 0) : (0, $coef);
      for my $x (0 .. 200) {
        my $rhs = ($x * $x * $x + $ca * $x + $cb) % $n;
        my $y = sqrtmod("$rhs", "$n");
        next unless defined $y;
        my $pt = Math::Prime::Util::ECAffinePoint->new($ca, $cb, $n, $x, $y);
        $pt->mul($m / $q);
        next if $pt->is_infinity;
        $pt->mul($q);
        next unless $pt->is_infinity;
        return ("Type ECPP\nN $n\nA $ca\nB $cb\nM $m\nQ $q\nX $x\nY $y\n", $q);
      }
    }
  }
  return ();
}

sub _prv_ecpp {
  my $fh = shift;
  my $header = sub { my $n = shift; "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN $n\n\n" };
  for my $d (-4, -3) {
    # q below 2^64 needs no further proof.
    my ($n, $block, $q) = (_prv_prime(66));
    while (1) {
      $n = Math::BigInt->new(next_prime("$n")) until ($n % abs($d)) == 1;
      ($block, $q) = _prv_ecpp_block($n, $d);
      last if defined $block;
      $n = Math::BigInt->new(next_prime("$n"));
    }
    my $cert = $header->($n) . $block;
    _prv_emit($fh, 'verify_prime', [$cert], sub { verify_prime($cert) });
    (my $bad = $cert) =~ s/^X (\d+)/"X " . (Math::BigInt->new($1) + 1)/me;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    ($bad = $cert) =~ s/^M (\d+)/"M " . (Math::BigInt->new($1) + 1)/me;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    ($bad = $cert) =~ s/^Q (\d+)/"Q 3"/me;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    # Negative A or B is reduced mod N by MPU.
    if ($d == -4) { ($bad = $cert) =~ s/^A (\d+)/"A " . (Math::BigInt->new($1) - $n)/me; }
    else          { ($bad = $cert) =~ s/^B (\d+)/"B " . (Math::BigInt->new($1) - $n)/me; }
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
  }
  # q above 2^64: the block needs a BLS5 proof of q after it.
  {
    my ($n, $block, $q) = (_prv_prime(80));
    while (1) {
      $n = Math::BigInt->new(next_prime("$n")) until ($n % 4) == 1;
      ($block, $q) = _prv_ecpp_block($n, -4);
      last if defined $block && $q > Math::BigInt->new("18446744073709551615");
      $n = Math::BigInt->new(next_prime("$n"));
    }
    my ($status, $qcert) = is_provable_prime_with_cert("$q");
    $qcert =~ s/^\[MPU - Primality Certificate\]\nVersion \S+\n+Proof for:\nN (\d+)\n+//ms;
    my $cert = $header->($n) . $block . "\n" . $qcert;
    _prv_emit($fh, 'verify_prime', [$cert], sub { verify_prime($cert) });
    # Without the proof of q the chain is incomplete.
    my $bad = $header->($n) . $block;
    _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    # ECPP3 (Primo type 3) form of the same block: A, B, T with the curve
    # scaled by L = T^3 + A T + B. The Primo types take S, R with M = R * S.
    if ($block =~ /^A (\d+)\nB (\d+)\nM (\d+)\nQ (\d+)\nX (\d+)\nY (\d+)/m) {
      my ($a, $b, $m, $qq, $x, $y) = map { Math::BigInt->new($_) } ($1, $2, $3, $4, $5, $6);
      my $s = $m / $qq;
      my $cert3 = $header->($n) . "Type ECPP3\nN $n\nS $s\nR $qq\nA $a\nB $b\nT $x\n\n" . $qcert;
      _prv_emit($fh, 'verify_prime', [$cert3], sub { verify_prime($cert3) });
      ($bad = $cert3) =~ s/^T (\d+)/"T " . (Math::BigInt->new($1) + 1)/me;
      _prv_emit($fh, 'verify_prime', [$bad], sub { verify_prime($bad) });
    }
  }
}

sub gen_bigint_proving {
  my $fh = shift;
  # --- primes with a smooth n - 1 (k * 2^m + 1): up to 200 digits
  for my $bits (65, 80, 96, 128, 160, 200, 256, 333, 400, 512, 664) {
    my $n = _prv_proth(24, $bits - 24);
    _prv_prove($fh, $n, 600);
  }
  # --- n - 1 = 2 * s * q with a big prime q
  for my $bits (65, 72, 96, 128, 200) {
    _prv_prove($fh, _prv_two_sq(16, $bits - 17), 600);
  }
  # --- random primes: the Perl prover factors n - 1 with its own methods
  for my $bits (65, 66, 68, 70, 72, 80, 88, 96, 112, 128) {
    _prv_prove($fh, _prv_prime($bits), 200);
  }
  # --- composites are rejected: products, squares, Carmichael, Mersenne
  my $p65 = _prv_prime(65);
  my $p63 = _prv_prime(63);
  _prv_prove($fh, $p65 * $p63, 60);
  _prv_prove($fh, $p65 * $p65, 60);
  _prv_prove($fh, _prv_prime(100) * _prv_prime(100), 60);
  _prv_prove($fh, _prv_odd(128) * 3, 60);
  my ($found, $k) = (0, _prv_big(30));
  while ($found < 2) {
    $k += 1;
    my ($a, $b, $c) = (6 * $k + 1, 12 * $k + 1, 18 * $k + 1);
    next unless is_prime("$a") && is_prime("$b") && is_prime("$c");
    _prv_prove($fh, $a * $b * $c, 60);
    $found++;
  }
  for my $e (67, 101, 127) {
    _prv_prove($fh, (Math::BigInt->new(1) << $e) - 1, 300);
  }
  # --- native range and its edge
  for my $n ("18446744073709551557", "18446744073709551615", "18446744073709551629", "2", "1", "0", "97") {
    _prv_prove($fh, Math::BigInt->new($n), 300);
  }
  # --- the other certificate types and ECPP blocks
  _prv_other_types($fh);
  _prv_ecpp($fh);
  # --- malformed certificates
  for my $text ("", "[MPU - Primality Certificate]\n", "[MPU - Primality Certificate]\nProof for:\nN 101\nType Small\nN 101\n",
                "[PRIMO - Primality Certificate]\nProof for:\nN 101\n", "[MPU - Primality Certificate]\nType Small\nN 101\n",
                "[MPU - Primality Certificate]\nBase 16\nProof for:\nN 101\n",
                "[MPU - Primality Certificate]\nProof for:\nN 18446744073709551629\nType Small\nN 18446744073709551629\n",
                "[MPU - Primality Certificate]\nProof for:\nN 18446744073709551629\n") {
    _prv_emit($fh, 'verify_prime', [$text], sub { verify_prime($text) });
  }
  # --- AKS: composites above 2^64 leave early; primes near the edge are slow in
  # pure Perl, so only the native range is recorded for them.
  for my $n ("18446744073709551617", "18446744073709551619", "18446744073709551621", "340282366920938463463374607431768211457",
             "1000003", "1000004", "2147483647", "4294967297", "9999999967") {
    _prv_emit($fh, 'is_aks_prime', [$n], sub { is_aks_prime($n) }, 300);
  }
}

1;
