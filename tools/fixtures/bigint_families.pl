# Short bigint intervals. Run with MPU_NO_XS=1 and MPU_NO_GMP=1.
use strict; use warnings;
sub gen_bigint_families {
  my ($fh) = @_;
  die 'bigint_families needs the pure-Perl oracle' unless $ENV{MPU_NO_XS} && $ENV{MPU_NO_GMP};
  $fh->autoflush(1);
  for my $n (Math::BigInt->new(2)->bpow(64), Math::BigInt->new(2)->bpow(70)) {
    my $hi = $n + 24;
    for my $fn (qw(primes twin_primes prime_powers)) {
      no strict 'refs';
      warn "oracle $fn $n\n";
      emit($fh, $fn, [$n,$hi], sub { @{ &{$fn}($n,$hi) } });
    }
    for my $fn (qw(next_perfect_power prev_perfect_power next_prime_power prev_prime_power next_chen_prime minimal_goldbach_pair)) {
      no strict 'refs';
      warn "oracle $fn $n\n";
      emit($fh, $fn, [$n], sub { &{$fn}($n) });
    }
    emit($fh, 'semi_primes', [$n,$hi], sub { @{semi_primes($n,$hi)} });
    emit($fh, 'almost_primes', [3,$n,$hi], sub { @{almost_primes(3,$n,$hi)} });
    emit($fh, 'omega_primes', [2,$n,$hi], sub { @{omega_primes(2,$n,$hi)} });
    emit($fh, 'sieve_range', [$n,$hi,7], sub { sieve_range($n,25,7) });
    emit($fh, 'sieve_prime_cluster', [$n,$hi,2], sub { sieve_prime_cluster($n,$hi,2) });
    emit($fh, 'print_primes', [$n,$hi], sub { @{primes($n,$hi)} });
  }
}
1;
