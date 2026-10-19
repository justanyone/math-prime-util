# MPU 0.75 pure Perl oracle. Use Math::BigInt for every input.
use strict;
use warnings;
sub gen_bigint_predicates {
  my ($fh) = @_;
  require Math::Prime::Util::PP;
  my @inputs = (0..40, '18446744073709551616',
    '18446744073709551617', '36893488147419103232',
    '1180591620717411303424', '618970019642690137449562111',
    '2658455991569831744654692615953842176');
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    emit($fh, 'is_semiprime', [$n], sub { Math::Prime::Util::PP::is_semiprime($n->copy) });
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    emit($fh, 'is_cyclic', [$n], sub { Math::Prime::Util::PP::is_cyclic($n->copy) });
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    emit($fh, 'is_fundamental', [$n], sub { Math::Prime::Util::PP::is_fundamental($n->copy) });
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    emit($fh, 'is_perfect_number', [$n], sub { Math::Prime::Util::PP::is_perfect_number($n->copy) });
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    emit($fh, 'is_perfect_power', [$n], sub { Math::Prime::Util::PP::is_perfect_power($n->copy) });
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    emit($fh, 'is_prime_power', [$n], sub { Math::Prime::Util::PP::is_prime_power($n->copy) });
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    emit($fh, 'is_practical', [$n], sub { Math::Prime::Util::PP::is_practical($n->copy) });
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    emit($fh, 'is_totient', [$n], sub { Math::Prime::Util::PP::is_totient($n->copy) });
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    for my $k (0, 1, 2, 3, 4, 7, 65) {
      emit($fh, 'is_powerfree', [$n, $k], sub { Math::Prime::Util::PP::is_powerfree($n->copy, $k) });
    }
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    for my $k (0, 1, 2, 3, 4, 7, 65) {
      emit($fh, 'is_powerful', [$n, $k], sub { Math::Prime::Util::PP::is_powerful($n->copy, $k) });
    }
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    for my $k (0, 1, 2, 3, 4, 7, 65) {
      emit($fh, 'is_rough', [$n, $k], sub { Math::Prime::Util::PP::is_rough($n->copy, $k) });
    }
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    for my $k (0, 1, 2, 3, 4, 7, 65) {
      emit($fh, 'is_smooth', [$n, $k], sub { Math::Prime::Util::PP::is_smooth($n->copy, $k) });
    }
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    for my $k (0, 1, 2, 3, 4, 7, 65) {
      emit($fh, 'is_sum_of_squares', [$n, $k], sub { Math::Prime::Util::PP::is_sum_of_squares($n->copy, $k) });
    }
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    for my $k (0, 1, 2, 3, 4, 7, 65) {
      emit($fh, 'is_almost_prime', [$k, $n], sub { Math::Prime::Util::PP::is_almost_prime($k, $n->copy) });
    }
  }
  for my $text (@inputs) {
    my $n = Math::BigInt->new("$text");
    for my $k (0, 1, 2, 3, 4, 7, 65) {
      emit($fh, 'is_omega_prime', [$k, $n], sub { Math::Prime::Util::PP::is_omega_prime($k, $n->copy) });
    }
  }
}
1;
