# MPU 0.75 PP oracle. Inputs have small factors for offline generation.
use strict;
use warnings;
sub gen_bigint_functions {
  my ($fh) = @_;
  require Math::Prime::Util::PP;
  die 'GMP oracle is not permitted' if exists $INC{'Math/Prime/Util/GMP.pm'};
  my @inputs = (0, 1, 2, 12, 27, '18446744073709551616',
                '36893488147419103232', '12157665459056928801',
                '36472996377170786403');
  for my $str (@inputs) {
    for my $name (qw(abundance aliquot_sum dedekind_psi exp_mangoldt liouville prime_signature sopf sopfr squarefree_kernel)) {
      my $n = Math::BigInt->new("$str");
      no strict 'refs';
      emit($fh, $name, [$n], sub { &{"Math::Prime::Util::PP::$name"}($n->copy) });
    }
    for my $k (1, 2, 3, 7) {
      for my $name (qw(crootint powerfree_part)) {
        my $n = Math::BigInt->new("$str");
        no strict 'refs';
        emit($fh, $name, [$n, $k], sub { &{"Math::Prime::Util::PP::$name"}($n->copy, $k) });
      }
    }
    my $n = Math::BigInt->new("$str");
    emit($fh, 'jordan_totient', [2, $n], sub { Math::Prime::Util::PP::jordan_totient(2, $n->copy) });
    emit($fh, 'ramanujan_sum', [$n, $n], sub { Math::Prime::Util::PP::ramanujan_sum($n->copy, $n->copy) });
    for my $name (qw(remove_factors remove_factors_exp)) {
      no strict 'refs';
      emit($fh, $name, [$n, 2], sub { &{"Math::Prime::Util::PP::$name"}($n->copy, 2) });
    }
  }
  # Odd inputs have no preimages. These include values above 2^64.
  for my $str (0, 1, 2, 4, 12, 24, '18446744073709551617') {
    my $n = Math::BigInt->new("$str");
    emit($fh, 'inverse_totient', [$n], sub { Math::Prime::Util::PP::inverse_totient($n->copy) });
  }
}
1;
