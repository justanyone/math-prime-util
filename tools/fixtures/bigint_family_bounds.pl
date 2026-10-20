# Fixture family `bigint_family_bounds`: the count and nth bounds and
# estimates of the prime families beyond the 64-bit range, for the `bigint`
# feature (src/bigint/family_bounds.rs). Replayed by
# tests/compat_bigint_family_bounds.rs.
#
# Oracle: MPU 0.75 pure Perl (MPU_NO_XS=1 and MPU_NO_GMP=1: Math::BigInt and
# Math::BigFloat at their default 40 digits; Math::Prime::Util::GMP is not
# installed). Inputs above 2^64 take the PP.pm formulas; the native fixtures
# hold the XS values below. The generator refuses to run with GMP or XS on.
#
# Result column:
# - A function that returns a Perl double (the lucky numbers) is written as
#   the integer its 15 digit string stands for ("6.5e+22" -> 65000000000000000000000).
# - ERROR is a die of MPU, NaN is its not-a-number: PP.pm hands a float to an
#   integer function or divides by zero there (docs/upstream-findings.md).
#   The Rust test skips those rows by name and checks the port by definition.
use strict; use warnings;

sub _fb_pow { my ($b, $e) = @_; return Math::BigInt->new($b)->bpow($e); }

# emit with doubles written out as integers.
sub _fb_emit {
  my ($fh, $fn, $args, $code) = @_;
  emit($fh, $fn, $args, sub {
    my @r = $code->();
    return map {
      my $v = $_;
      (defined $v && !ref($v) && "$v" =~ /^[0-9.]+e\+\d+$/) ? Math::BigFloat->new("$v")->bstr : $v;
    } @r;
  });
}

sub gen_bigint_family_bounds {
  my $fh = shift;
  my $cfg = Math::Prime::Util::prime_get_config();
  die "bigint_family_bounds requires MPU_NO_XS=1 and MPU_NO_GMP=1\n" if $cfg->{xs} || $cfg->{gmp};
  require Math::BigFloat;

  my @big = (_fb_pow(2, 64), _fb_pow(2, 64) + 1, _fb_pow(2, 65), _fb_pow(10, 20), _fb_pow(2, 70),
             _fb_pow(10, 25), _fb_pow(2, 100), _fb_pow(10, 30), _fb_pow(2, 127));
  my $two64 = _fb_pow(2, 64);

  # Counts at n > 2^64 (the count of the perfect powers and prime powers is
  # exact or a sum of prime count bounds).
  for my $bits (64, 65, 80, 100, 127) {
    my $base = _fb_pow(2, $bits);
    for my $delta (-1, 0, 1) {
      my $n = $base + $delta;
      for my $family (qw(perfect_power prime_power)) {
        for my $kind (qw(approx lower upper)) {
          my $name = "${family}_count_${kind}";
          my $code = Math::Prime::Util->can($name);
          _fb_emit($fh, $name, [$n], sub { $code->($n) });
        }
      }
    }
  }
  for my $n (@big) {
    for my $name (qw(semiprime_count_approx twin_prime_count_approx
                     lucky_count_approx lucky_count_lower lucky_count_upper
                     ramanujan_prime_count_approx ramanujan_prime_count_lower ramanujan_prime_count_upper)) {
      my $code = Math::Prime::Util->can($name);
      _fb_emit($fh, $name, [$n], sub { $code->($n) });
    }
  }
  for my $k (3, 4, 5, 6, 8, 13, 20) {
    for my $n (@big) {
      next if $n->bcmp(_fb_pow(2, $k)) < 0;
      for my $name (qw(almost_prime_count_approx almost_prime_count_lower almost_prime_count_upper)) {
        my $code = Math::Prime::Util->can($name);
        _fb_emit($fh, $name, [$k, $n], sub { $code->($k, $n) });
      }
    }
  }

  # The inverse of li: native range edge, then above it.
  for my $n (_fb_pow(2, 63), $two64 - 1, @big) {
    _fb_emit($fh, 'inverse_li', [$n], sub { inverse_li($n) });
  }

  # nth functions: indices above the native range of each function.
  my @pp = (_fb_pow(2, 32) + 1, _fb_pow(2, 40), _fb_pow(2, 63), @big);
  my @mid = (_fb_pow(2, 59), _fb_pow(2, 63), @big);
  my %idx = (
    nth_perfect_power_approx => [@pp],
    nth_perfect_power_lower  => [@pp],
    nth_perfect_power_upper  => [@pp],
    nth_prime_power_approx   => [@mid],
    nth_prime_power_lower    => [@mid],
    nth_prime_power_upper    => [@mid],
    nth_lucky_approx         => [@mid],
    nth_lucky_lower          => [@mid],
    nth_lucky_upper          => [@mid],
    nth_ramanujan_prime_approx => [@mid],
    nth_ramanujan_prime_lower  => [@mid],
    nth_ramanujan_prime_upper  => [@mid],
    nth_semiprime_approx     => [_fb_pow(2, 61), _fb_pow(2, 63), @big],
    nth_twin_prime_approx    => [_fb_pow(2, 54), _fb_pow(2, 60), _fb_pow(2, 63), @big],
  );
  for my $name (sort keys %idx) {
    my $code = Math::Prime::Util->can($name);
    for my $n (@{$idx{$name}}) {
      _fb_emit($fh, $name, [$n], sub { $code->($n) });
    }
  }
  for my $k (3, 4, 5, 8, 13, 20) {
    for my $n (_fb_pow(2, 62), _fb_pow(2, 63), @big) {
      for my $name (qw(nth_almost_prime_approx nth_almost_prime_lower nth_almost_prime_upper)) {
        my $code = Math::Prime::Util->can($name);
        _fb_emit($fh, $name, [$k, $n], sub { $code->($k, $n) });
      }
    }
  }
}
1;
