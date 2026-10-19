# MPU 0.75 pure-Perl oracle on Math::BigInt elements, without GMP.
use strict;
use warnings;
sub gen_bigint_sets {
  my ($fh) = @_;
  die 'bigint_sets requires MPU_NO_XS=1' unless $ENV{MPU_NO_XS};
  my $h = Math::BigInt->new(2)->bpow(100);
  my @vectors = ([], [0], [1], [$h], [$h, $h+1, $h+3],
                 [$h+3, 0, $h, $h+3, 1], [$h, $h+1, $h+2],
                 [-$h, -$h+1, $h, 0], [$h*2, $h, $h]);
  my @unary = qw(toset vecuniq vecsingleton vecsort vecrsort vecsorti vecrsorti
                 vecmin vecmax vecmex vecpmex vecfreq is_sidon_set is_sumfree_set);
  my @binary = qw(setunion setintersect setminus setdelta setcontains setcontainsany
                 set_is_disjoint set_is_equal set_is_subset set_is_superset
                 set_is_proper_subset set_is_proper_superset set_is_proper_intersection
                 sumset setbinop setinsert setremove setinvert);
  for my $raw (@vectors) {
    my @a = map { Math::BigInt->new("$_") } @$raw;
    my $aa = @a ? join(',', @a) : '-';
    for my $fn (@unary) {
      next if ($fn eq 'vecmex' || $fn eq 'vecpmex') && grep { $_ < 0 } @a;
      no strict 'refs';
      emit($fh, $fn, [$aa], sub {
        if ($fn eq 'is_sidon_set' || $fn eq 'is_sumfree_set') { return &{$fn}(\@a); }
        if ($fn eq 'vecfreq') {
          my %f = &{$fn}(@a);
          return join(' ', map { "$_:$f{$_}" } sort { Math::BigInt->new($a) <=> Math::BigInt->new($b) } keys %f);
        }
        if ($fn eq 'toset') { return @{toset(@a)}; }
        if ($fn eq 'vecsorti' || $fn eq 'vecrsorti') {
          my @copy = @a;
          return @{&{$fn}(\@copy)};
        }
        return &{$fn}(@a);
      });
    }
    for my $rawb (@vectors) {
      my @b = map { Math::BigInt->new("$_") } @$rawb;
      my $bb = @b ? join(',', @b) : '-';
      for my $fn (@binary) {
        no strict 'refs';
        emit($fh, $fn, [$aa, $bb], sub {
          if ($fn eq 'setbinop') { return @{&setbinop(sub { $a + $b }, \@a, \@b)}; }
          if ($fn eq 'setinsert' || $fn eq 'setremove' || $fn eq 'setinvert') {
            my @copy = @{toset(@a)};
            my $delta = &{$fn}(\@copy, \@b);
            return "$delta;" . join(',', @copy);
          }
          my $r = &{$fn}(\@a, \@b);
          return ref($r) eq 'ARRAY' ? @$r : $r;
        });
      }
    }
  }
}
1;
