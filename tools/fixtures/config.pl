# Fixture family `config`: prime_set_config / prime_get_config in reference/mpu/lib/Math/Prime/Util.pm.
# The function column is the key as written; the argument is the value; the result is the
# setting it leaves behind (verbose or assume_rh), or ERROR when MPU croaks on the key.
use strict; use warnings;

sub gen_config {
  my $fh = shift;
  my @keys = (
    [ 'verbose',            [0, 1, 2, 5] ],
    [ 'VERBOSE',            [3] ],
    [ 'assume_rh',          [0, 1] ],
    [ 'rh',                 [0, 1] ],
    [ 'grh',                [1] ],
    [ 'erh',                [1] ],
    [ 'assume rh',          [1, 0] ],
    [ 'assume_grh',         [1] ],
    [ 'assume erh',         [1] ],
    [ 'assumerh',           [1] ],
    [ 'riemann hypothesis', [1] ],
    [ 'riemannh',           [1] ],
    [ 'assume',             [1] ],
    [ 'maxbits',            [1, 32, 64] ],
    [ 'no_such_key',        [1] ],
  );
  for my $k (@keys) {
    my ($key, $vals) = @$k;
    for my $v (@$vals) {
      emit($fh, $key, [$v], sub {
        prime_set_config(verbose => 0, assume_rh => 0);
        prime_set_config($key => $v);
        my $cfg = prime_get_config();
        return $key =~ /verbose/i ? $cfg->{verbose} : $cfg->{assume_rh};
      });
    }
  }
}

1;
