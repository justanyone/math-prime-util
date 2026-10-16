# Fixture family `real`: t/53-realfunctions.t, t/28-pi.t, bernreal/harmreal of t/27-bernfrac.t.
# Floating results are emitted exactly as Perl prints them (the Rust test compares with a relative tolerance).
# Integer-only functions (inverse_li, chebyshev_*, *_approx) get integer arguments only.
use strict; use warnings;

sub gen_real {
  my $fh = shift;
  my $inf = 9**9**9;
  my $nan = $inf - $inf;
  my @extras = ('0.5', '1', '2', '10', '1000000', '10000000000', '1000000000000000');
  my @fextras = ('0.5', '1', '2', '10', '1e6', '1e10', '1e15');
  my $f = sub {   # float-argument function
    my ($fn, $code, @args) = @_;
    for my $a (@args) { emit($fh, $fn, [$a], sub { $code->($a) }); }
  };
  # ExponentialIntegral (t eivals plus extras)
  my @ei = ('-10','-0.5','-0.1','-0.001','-0.00001','-0.00000001','0.693147180559945','1','1.5','2','5','10','12',
            '20','40','41','79','710','170','2.2','0','-1','-2','-5','3','0.1','0.01','100','300', @fextras,
            'inf', '-inf', 'nan');
  $f->('ExponentialIntegral', sub { my $x = shift; ExponentialIntegral($x eq 'inf' ? $inf : $x eq '-inf' ? -$inf : $x eq 'nan' ? $nan : $x) }, @ei);
  my @li = ('0','1.01','2','10','24','1000','100000','100000000','4294967295','10000000000','100000000000','1e60',
            '-1','-0.5','0.5','1','1.5','3','100','1e15','18446744073709551615', @fextras, 'inf');
  $f->('LogarithmicIntegral', sub { my $x = shift; LogarithmicIntegral($x eq 'inf' ? $inf : $x) }, @li);
  my @r = ('1.01','2','10','1000','1000000','10000000','4294967295','10000000000','18446744073709551615','1e60',
           '0','-1','0.5','1','1.5','3','24','100','1e15', @fextras, 'inf');
  $f->('RiemannR', sub { my $x = shift; RiemannR($x eq 'inf' ? $inf : $x) }, @r);
  my @z = ('2','2.5','4.5','7','8.5','20.6','0.5','1','1.5','3','4','5','10','0','-1','-2','-3','-0.5','0.1','50','100',
           '1e6','1e10','1e15','inf','-inf','nan');
  $f->('RiemannZeta', sub { my $x = shift; RiemannZeta($x eq 'inf' ? $inf : $x eq '-inf' ? -$inf : $x eq 'nan' ? $nan : $x) }, @z);
  my @lw = ('-0.367879441171443','-0.3678794411714423215955237701614608674458','-.1','0',
            '0.3678794411714423215955237701614608674458','1','10','10000','100000000000','18446744073709551615',
            '-0.3678794411715','-0.36','-0.2','-0.01','0.01','0.1','0.5','2','3','100','1e6','1e10','1e15','inf','nan');
  $f->('LambertW', sub { my $x = shift; LambertW($x eq 'inf' ? $inf : $x eq 'nan' ? $nan : $x) }, @lw);

  # Pi (digits) -> string
  my @pi = (0..110, 150, 200, 250, 461, 500, 760..770, 1000, 2000);
  for my $d (@pi) { emit($fh, 'Pi', [$d], sub { my $p = Pi($d); "$p" }); }
  emit($fh, 'Pi', [], sub { my $p = Pi(); defined $p ? "$p" : undef });

  # integer-argument functions
  my @ints = (0..40, 50, 64, 97, 100, 101, 200, 255, 256, 1000, 1024, 5000, 10000, 100000, 1000000, 10000000,
              100000000, 1000000000, 4294967295, 4294967296, 10000000000, 100000000000, 1000000000000,
              1000000000000000, 10000000000000000, 100000000000000000, 1000000000000000000,
              '10000000000000000000', '18446744073709551615');
  my @big = ('18446744073709551616', '100000000000000000000', '1000000000000000000000');
  for my $n (@ints, @big, -1) {
    emit($fh, 'inverse_li', [$n], sub { my $v = inverse_li($n); defined $v ? "$v" : undef });
    emit($fh, 'prime_count_approx', [$n], sub { my $v = prime_count_approx($n); defined $v ? "$v" : undef });
    emit($fh, 'nth_prime_approx', [$n], sub { my $v = nth_prime_approx($n); defined $v ? "$v" : undef });
  }
  for my $n (grep { $_ <= 1e12 } @ints) {
    emit($fh, 'chebyshev_theta', [$n], sub { chebyshev_theta($n) });
    emit($fh, 'chebyshev_psi', [$n], sub { chebyshev_psi($n) });
  }
  for my $n (@fextras, '0.5', '2.5', '100.5', '-1', '1e20', '1e60', '18446744073709551615', '123456789.123') {
    emit($fh, 'inverse_li_nv', [$n], sub { inverse_li_nv($n) });
  }
  for my $n (0..40, 100, 1000, 1000000, 1e10, 1e15, '18446744073709551615') {
    emit($fh, 'inverse_li_nv', [$n], sub { inverse_li_nv($n) });
  }

  # bernreal / harmreal (t/27-bernfrac.t)
  for my $n (0..60, 70, 80, 100, 150, 200) {
    emit($fh, 'bernreal', [$n], sub { my $v = bernreal($n); "$v" });
  }
  for my $n (0..60, 100, 200, 500, 1000) {
    emit($fh, 'harmreal', [$n], sub { my $v = harmreal($n); "$v" });
  }
  emit($fh, 'bernreal', [-1], sub { my $v = bernreal(-1); "$v" });
  emit($fh, 'harmreal', [-1], sub { my $v = harmreal(-1); "$v" });
}
1;
