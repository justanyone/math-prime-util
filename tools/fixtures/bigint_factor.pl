# Fixture family `bigint_factor`: factoring and divisor functions on 65 to 200
# bit inputs, for the `bigint` feature (src/bigint/factor.rs and
# src/bigint/divisors.rs). Replayed by tests/compat_bigint_factor.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1, Math::BigInt). The inputs come from a
# fixed linear congruential generator, so the file is the same on every host.
# Pure Perl factoring is slow (no quadratic sieve: a 200-bit semiprime with two
# 100-bit factors never ends), so every composite input has a structure the PP
# recipe splits in seconds: small factors times one big prime, prime powers,
# a prime with a smooth p-1 (the p-1 stage), or a factor below 40 bits (the
# first ECM stages). Each method function gets inputs its method splits.
# The method functions are not exported by MPU and are called by their full
# name. List results are space-joined; factor_exp prints each pair as p^e.
use strict; use warnings;

my $fac_lcg = 20_261_019;
# One 32-bit pseudo-random chunk. The product stays below 2^53, so plain doubles are exact.
sub _fac_chunk32 {
  $fac_lcg = ($fac_lcg * 1_664_525 + 1_013_904_223) % 4_294_967_296;
  return $fac_lcg;
}

# A Math::BigInt with exactly $bits bits (top bit set).
sub _fac_big {
  my $bits = shift;
  my $n = Math::BigInt->new(0);
  for (my $have = 0; $have < $bits; $have += 32) {
    $n = ($n << 32) + _fac_chunk32();
  }
  $n = $n % (Math::BigInt->new(1) << $bits);
  return $n | (Math::BigInt->new(1) << ($bits - 1));
}

# A random prime of about $bits bits.
sub _fac_prime { my $n = _fac_big(shift) | 1; return Math::BigInt->new(next_prime("$n")); }

# A random prime below $limit (native).
sub _fac_small_prime {
  my $limit = shift;
  my $r = 2 + (_fac_chunk32() % ($limit - 2));
  my $p = next_prime($r);
  $p = next_prime(2) if $p > $limit;
  return $p;
}

# A prime p of about $bits bits with p - 1 smooth to $smooth: two times
# distinct random primes below $smooth, plus one. Returns the prime and its
# two largest p - 1 factors, so the p-1 tests can pick bounds around them.
sub _fac_smooth_prime {
  my ($bits, $smooth) = @_;
  while (1) {
    my $m = Math::BigInt->new(2);
    my %seen = (2 => 1);
    while (length($m->as_bin) - 2 < $bits - 1) {
      my $q = _fac_small_prime($smooth);
      next if $seen{$q}++;
      $m *= $q;
    }
    my $p = $m + 1;
    my @largest = (sort { $b <=> $a } keys %seen)[0, 1];
    return ($p, @largest) if is_prime("$p");
  }
}

sub _fac_pow2 { my $e = shift; return Math::BigInt->new(1) << $e; }

# The divisor family on one input: everything that needs only the
# factorization. $full = 0: factor and factor_exp; 1: everything; 2: factor.
sub _fac_divisor_family {
  my ($fh, $n, $full) = @_;
  emit($fh, 'factor', [$n], sub { factor("$n") });
  return if $full == 2;
  emit($fh, 'factor_exp', [$n], sub { map { "$_->[0]^$_->[1]" } factor_exp("$n") });
  return unless $full;
  emit($fh, 'euler_phi', [$n], sub { euler_phi("$n") });
  emit($fh, 'carmichael_lambda', [$n], sub { carmichael_lambda("$n") });
  emit($fh, 'moebius', [$n], sub { moebius("$n") });
  emit($fh, 'is_square_free', [$n], sub { is_square_free("$n") });
  emit($fh, 'prime_bigomega', [$n], sub { prime_bigomega("$n") });
  emit($fh, 'prime_omega', [$n], sub { prime_omega("$n") });
  for my $k (0, 1, 2) {
    emit($fh, 'divisor_sum', [$n, $k], sub { divisor_sum("$n", $k) });
  }
  my $count = divisor_sum("$n", 0);
  if ($count <= 600) {
    emit($fh, 'divisors', [$n], sub { divisors("$n") });
  }
  my $k = _fac_big(40);
  emit($fh, 'divisors', [$n, $k], sub { divisors("$n", "$k") });
  emit($fh, 'divisors', [$n, 1000], sub { divisors("$n", 1000) });
}

sub _fac_note { print STDERR 'bigint_factor: ', shift, ' (', scalar(localtime), ")\n"; }

sub gen_bigint_factor {
  my $fh = shift;
  select((select($fh), $| = 1)[0]);
  my @sizes = (65, 80, 100, 128, 150, 200);

  # --- group A: small factors times one big prime, prime powers, big primes.
  # These factor by trial division and a primality test alone.
  my %prime;
  for my $bits (@sizes) {
    my $p = _fac_prime($bits);
    $prime{$bits} = $p;
    _fac_divisor_family($fh, $p, 1);
    _fac_divisor_family($fh, $p * 6, 1);
    _fac_divisor_family($fh, $p * 4999 * 4999, 1);
    _fac_divisor_family($fh, $p * _fac_pow2(10) * 3 * 3 * 3 * 7, 1);
    _fac_divisor_family($fh, $p * 1009 * 2003 * 3001, 1);
  }
  for my $bits (65, 100) {
    my $p = $prime{$bits};
    _fac_divisor_family($fh, $p * $p, 1);
    _fac_divisor_family($fh, $p * $p * $p, 1);
    _fac_divisor_family($fh, $p * $p * 2 * 5, 1);
  }
  my $p33 = Math::BigInt->new(next_prime(2**33 + 17));
  _fac_divisor_family($fh, $p33 ** 4, 1);
  _fac_divisor_family($fh, $p33 ** 5 * 49, 1);
  _fac_divisor_family($fh, _fac_pow2(100), 1);
  _fac_divisor_family($fh, _fac_pow2(200), 1);
  _fac_divisor_family($fh, _fac_pow2(66) * 3 ** 20 * 5 ** 8, 1);
  _fac_divisor_family($fh, Math::BigInt->new(3) ** 100, 1);
  _fac_divisor_family($fh, Math::BigInt->new(6) ** 50, 1);
  # Mersenne numbers with known factors: 2^67-1 = 193707721 * 761838257287 and
  # 2^89-1, 2^107-1 prime; 2^101-1 = 7432339208719 * 341117531003194129.
  _fac_divisor_family($fh, _fac_pow2(89) - 1, 1);
  _fac_divisor_family($fh, _fac_pow2(107) - 1, 1);
  _fac_divisor_family($fh, (_fac_pow2(89) - 1) * 3, 1);
  # --- edge inputs
  for my $n (0, 1, 2, 3, 4, 12, 4999, 4999 * 4999, 4999 * 5003, 2 ** 32 + 1, 2 ** 61 - 1) {
    _fac_divisor_family($fh, Math::BigInt->new($n), 1);
  }
  for my $n (_fac_pow2(64), _fac_pow2(64) + 13, _fac_pow2(64) - 59, _fac_pow2(128), _fac_pow2(128) + 51,
             _fac_pow2(128) - 159) {
    _fac_divisor_family($fh, $n, 1);
  }
  # Not here: 2^128 + 1 = 59649589127497217 * 5704689200685129054721 needs the
  # ECM 40k stage in pure Perl (hours); the native path covers it in Rust.

  _fac_note('group B');
  # --- group B: a prime with a smooth p-1 times a big prime: the p-1 stage.
  my @smooth;
  for my $bits (40, 60, 80, 100) {
    my ($p, $largest) = _fac_smooth_prime($bits, 60_000);
    push @smooth, $p;
    my $q = $prime{200 - $bits > 100 ? 100 : 65};
    my $n = $p * $q;
    _fac_divisor_family($fh, $n, $bits <= 60);
    emit($fh, 'pminus1_factor', [$n, 65_536], sub { Math::Prime::Util::pminus1_factor("$n", 65_536) });
    emit($fh, 'pminus1_factor', [$n, 65_536, 65_536], sub { Math::Prime::Util::pminus1_factor("$n", 65_536, 65_536) });
  }
  # A stage 2 case: B1 covers every factor of p - 1 but the largest, which
  # lies below B2.
  for my $bits (50, 70) {
    my ($p, $largest, $second) = _fac_smooth_prime($bits, 60_000);
    my $q = $prime{100};
    my $n = $p * $q;
    my $b1 = $second < 7 ? 7 : $second;
    emit($fh, 'pminus1_factor', [$n, $b1, $largest + 10], sub { Math::Prime::Util::pminus1_factor("$n", $b1, $largest + 10) });
    emit($fh, 'pminus1_factor', [$n, $b1, $b1 + 1], sub { Math::Prime::Util::pminus1_factor("$n", $b1, $b1 + 1) });
    emit($fh, 'pminus1_factor', [$n, 3], sub { Math::Prime::Util::pminus1_factor("$n", 3) });
  }
  { my $p = $smooth[0];
    my $n = $p * $prime{65};
    emit($fh, 'pminus1_factor', [$n], sub { Math::Prime::Util::pminus1_factor("$n") });
  }

  _fac_note('group C');
  # --- group C: a factor below 40 bits times a big prime: the first ECM stages.
  my @ecm_inputs;
  for my $bits (24, 28, 32, 36, 40) {
    my $p = _fac_prime($bits);
    my $q = $prime{$bits >= 36 ? 100 : 150};
    my $n = $p * $q;
    push @ecm_inputs, $n;
    _fac_divisor_family($fh, $n, $bits <= 28 ? 1 : $bits <= 36 ? 0 : 2);
    emit($fh, 'ecm_factor', [$n, 2000, 40_000, 40], sub { Math::Prime::Util::ecm_factor("$n", 2000, 40_000, 40) });
  }
  { my $n = $ecm_inputs[0];
    emit($fh, 'ecm_factor', [$n, 500], sub { Math::Prime::Util::ecm_factor("$n", 500) });
    emit($fh, 'ecm_factor', [$n, 500, 10_000], sub { Math::Prime::Util::ecm_factor("$n", 500, 10_000) });
    emit($fh, 'ecm_factor', [$n, 500, 10_000, 5], sub { Math::Prime::Util::ecm_factor("$n", 500, 10_000, 5) });
    emit($fh, 'ecm_factor', [$n], sub { Math::Prime::Util::ecm_factor("$n") });
    emit($fh, 'ecm_factor', [$n, 2, 3, 1], sub { Math::Prime::Util::ecm_factor("$n", 2, 3, 1) });
  }
  # Three and four factors below 36 bits.
  { my $n = _fac_prime(24) * _fac_prime(30) * _fac_prime(36);
    _fac_divisor_family($fh, $n, 0);
    my $m = _fac_prime(20) * _fac_prime(24) * _fac_prime(28) * _fac_prime(32) * 7;
    _fac_divisor_family($fh, $m, 1);
  }
  _fac_note('group D');
  # --- group D: two harder inputs, a 48-bit factor (ECM 8k) times a 100-bit prime.
  for my $bits (44, 48) {
    my $n = _fac_prime($bits) * $prime{100};
    _fac_divisor_family($fh, $n, 2);
  }

  _fac_note('trial division');
  # --- trial division (a bound below 5 means the square root: never on a
  # big prime cofactor, PP divides up to the root)
  for my $bits (65, 128, 200) {
    my $p = $prime{$bits};
    for my $n ($p * 2 * 3 * 1009, $p * 4999 * 4999, $p) {
      for my $limit (5, 1000, 4999, 5000, 100_000) {
        emit($fh, 'trial_factor', [$n, $limit], sub { Math::Prime::Util::trial_factor("$n", $limit) });
      }
    }
  }
  { my $n = Math::BigInt->new(1_000_003) * 1_000_033;
    emit($fh, 'trial_factor', [$n], sub { Math::Prime::Util::trial_factor("$n") });
    emit($fh, 'trial_factor', [$n, 1_000_004], sub { Math::Prime::Util::trial_factor("$n", 1_000_004) });
    for my $m (0, 1, 2, 4, 30, 49) {
      emit($fh, 'trial_factor', [$m], sub { Math::Prime::Util::trial_factor($m) });
      emit($fh, 'trial_factor', [$m, 5], sub { Math::Prime::Util::trial_factor($m, 5) });
    }
  }

  _fac_note('Pollard rho');
  # --- Pollard rho and Brent: a factor below 28 bits times a big prime.
  for my $bits (16, 20, 24, 28) {
    my $p = _fac_prime($bits);
    my $n = $p * $prime{100};
    emit($fh, 'prho_factor', [$n, 200_000], sub { Math::Prime::Util::prho_factor("$n", 200_000) });
    emit($fh, 'pbrent_factor', [$n, 200_000], sub { Math::Prime::Util::pbrent_factor("$n", 200_000) });
    emit($fh, 'pbrent_factor', [$n, 200_000, 3], sub { Math::Prime::Util::pbrent_factor("$n", 200_000, 3) });
    emit($fh, 'prho_factor', [$n, 10], sub { Math::Prime::Util::prho_factor("$n", 10) });
    emit($fh, 'pbrent_factor', [$n, 10], sub { Math::Prime::Util::pbrent_factor("$n", 10) });
  }
  { my $n = _fac_prime(20) * $prime{100};
    emit($fh, 'prho_factor', [$n], sub { Math::Prime::Util::prho_factor("$n") });
    emit($fh, 'pbrent_factor', [$n], sub { Math::Prime::Util::pbrent_factor("$n") });
    my $sq = $prime{65} * $prime{65};
    emit($fh, 'pbrent_factor', [$sq, 1000], sub { Math::Prime::Util::pbrent_factor("$sq", 1000) });
    emit($fh, 'prho_factor', [$sq, 1000], sub { Math::Prime::Util::prho_factor("$sq", 1000) });
  }

  _fac_note('HOLF');
  # --- HOLF: two primes of the same size close together, or in a small ratio.
  for my $bits (40, 60, 80, 100) {
    my $p = _fac_prime($bits);
    my $q = Math::BigInt->new(next_prime("" . ($p + 2 + 2 * $bits)));
    my $n = $p * $q;
    emit($fh, 'holf_factor', [$n, 10_000], sub { Math::Prime::Util::holf_factor("$n", 10_000) });
    emit($fh, 'holf_factor', [$n, 10_000, 1], sub { Math::Prime::Util::holf_factor("$n", 10_000, 1) });
    my $r = Math::BigInt->new(next_prime("" . ($p * 3 + 50)));
    my $m = $p * $r;
    emit($fh, 'holf_factor', [$m, 10_000], sub { Math::Prime::Util::holf_factor("$m", 10_000) });
    emit($fh, 'holf_factor', [$m, 2], sub { Math::Prime::Util::holf_factor("$m", 2) });
    emit($fh, 'holf_factor', [$m, 2, 3], sub { Math::Prime::Util::holf_factor("$m", 2, 3) });
  }
  { my $n = $prime{65} * $prime{100};
    emit($fh, 'holf_factor', [$n, 100], sub { Math::Prime::Util::holf_factor("$n", 100) });
    emit($fh, 'holf_factor', [$n, 0], sub { Math::Prime::Util::holf_factor("$n", 0) });
  }

  _fac_note('p+1 and SQUFOF');
  # --- p+1 and SQUFOF: pure Perl stubs (Pollard-Brent with the bound as the
  # round count); see docs/upstream-findings.md. Small inputs let the stub
  # split; the big ones record the stub's unsplit result.
  for my $bits (16, 20) {
    my $p = _fac_prime($bits);
    my $n = $p * $prime{100};
    emit($fh, 'pplus1_factor', [$n, 200_000], sub { Math::Prime::Util::pplus1_factor("$n", 200_000) });
    emit($fh, 'squfof_factor', [$n, 200_000], sub { Math::Prime::Util::squfof_factor("$n", 200_000) });
    emit($fh, 'pplus1_factor', [$n, 1000], sub { Math::Prime::Util::pplus1_factor("$n", 1000) });
    emit($fh, 'squfof_factor', [$n, 10], sub { Math::Prime::Util::squfof_factor("$n", 10) });
  }
  { my $n = Math::BigInt->new(4_294_967_311) * 4_294_967_357;
    emit($fh, 'pplus1_factor', [$n, 1000], sub { Math::Prime::Util::pplus1_factor("$n", 1000) });
    emit($fh, 'squfof_factor', [$n], sub { Math::Prime::Util::squfof_factor("$n") });
    emit($fh, 'squfof_factor', [$n, 100_000], sub { Math::Prime::Util::squfof_factor("$n", 100_000) });
    my $m = $prime{65} * 1_000_003;
    emit($fh, 'pplus1_factor', [$m], sub { Math::Prime::Util::pplus1_factor("$m") });
    emit($fh, 'squfof_factor', [$m, 100], sub { Math::Prime::Util::squfof_factor("$m", 100) });
  }
}

1;
