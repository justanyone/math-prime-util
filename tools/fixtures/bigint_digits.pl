# BigUint digit operations. Force the vendored MPU 0.75 pure-Perl path.
use strict;
use warnings;
sub gen_bigint_digits {
  my ($fh) = @_;
  Math::Prime::Util::prime_set_config(xs => 0, gmp => 0);
  for my $text ('0', '1', '19', '77', '18446744073709551616',
                '18446744073709551617', '123456789098765432101234567890987654321',
                '340282366920938463463374607431768211455',
                '100000000000000000000000000000000000000000000000000000000000000000000') {
    my $n = Math::BigInt->new($text);
    for my $base (2, 3, 10, 16, 36) {
      for my $name (qw(digital_root is_harshad is_palindrome mult_digital_root reverse_digits)) {
        no strict 'refs';
        emit($fh, $name, [$n, $base], sub { &{$name}($n->copy, $base) });
      }
      for my $len (undef, 0, 4, 140) {
        my @args = defined($len) ? ($n, $base, $len) : ($n, $base);
        emit($fh, 'todigits', \@args, sub { join(' ', todigits($n->copy, $base, defined($len) ? ($len) : ())) });
        emit($fh, 'todigitstring', \@args, sub { todigitstring($n->copy, $base, defined($len) ? ($len) : ()) });
      }
      my @digits = todigits($n->copy, $base);
      emit($fh, 'fromdigits', [$base, @digits], sub { fromdigits(\@digits, $base) });
      for my $k (0, 1, 2, 3) {
        emit($fh, 'is_happy', [$n, $base, $k], sub { is_happy($n->copy, $base, $k) });
      }
    }
    emit($fh, 'tozeckendorf', [$n], sub { tozeckendorf($n->copy) });
    my $z = tozeckendorf($n->copy);
    emit($fh, 'fromzeckendorf', [$z], sub { fromzeckendorf($z) });
  }
  for my $text ('0', '-0.3', '+12_345', '1.5e2', '1e-100', '0xabcdef12345678901234567890',
                '0b101010', '0o12345', '18446744073709551616.999',
                '1.2345678901234567890123456789e60', '0e100', '42E+2') {
    emit($fh, 'toint', [$text], sub { toint($text) });
  }
  for my $text ('11', '01', '10x', '00') {
    emit($fh, 'fromzeckendorf', [$text], sub { fromzeckendorf($text) });
  }
  return 1;
}
1;
