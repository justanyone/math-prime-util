# MPU 0.75 pure Perl bigint modular helper oracle.
use strict;
use warnings;
sub gen_bigint_modhelpers {
  my ($fh) = @_;
  die 'pure Perl oracle required' unless $ENV{MPU_NO_XS};
  for my $bits (65, 96, 128) {
    my $n = Math::BigInt->new(2)->bpow($bits);
    my $a = $n + 7;
    my $b = $n + 11;
    for my $fn (qw(submod negmod muladdmod mulsubmod)) {
      my @args = $fn eq 'negmod' ? ($a,$n) : $fn eq 'submod' ? ($a,$b,$n) : ($a,$b,$a,$n);
      no strict 'refs';
      emit($fh,$fn,\@args,sub { &{"Math::Prime::Util::PP::$fn"}(@args) });
    }
    for my $x (0,1,2,3,4,9) {
      emit($fh,'is_qr',[$x,$n],sub { Math::Prime::Util::PP::is_qr($x,$n) });
    }
    emit($fh,'qnr',[$n],sub { Math::Prime::Util::PP::qnr($n) });
    emit($fh,'pisano_period',[$n],sub { Math::Prime::Util::PP::pisano_period($n) });
    emit($fh,'lucasuvmod',[1,-1,$a,$n],sub { Math::Prime::Util::PP::lucasuvmod(1,-1,$a,$n) });
    emit($fh,'cornacchia',[0,$n*$n],sub { Math::Prime::Util::PP::cornacchia(0,$n*$n) });
    emit($fh,'cornacchia',[$n,3*$n],sub { Math::Prime::Util::PP::cornacchia($n,3*$n) });
    emit($fh,'chinese2',[2,$n,2,3],sub { Math::Prime::Util::PP::chinese2([2,$n],[2,3]) });
    emit($fh,'allrootmod',[1,3,$n],sub { my @r = Math::Prime::Util::PP::allrootmod(1,3,$n); return @r ? @r : undef; });
  }
}
1;
