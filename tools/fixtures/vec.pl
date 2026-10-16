# Fixture family `vec`: t/26-vec.t, t/26-setops.t, t/26-setbinop.t (only through sumset), t/26-sumset.t, t/26-mex.t.
# Args column: list elements space-separated; two lists are separated by a literal `|` token.
#   vecextract        args "list | mask"      (mask = bit-mask integer)
#   vecextract_idx    args "list | i j k"     (index-list form; not an MPU name)
#   setinsert/setremove/setinvert  args "set | v1 v2 .."   result = the resulting set; `<name>_n` = the returned count
#   setinsert_set/setremove_set/setinvert_set  args "set | set2" (second-argument array reference form), same result
#   setcontains / setcontainsany   args "set | v1 v2 .."
#   vecsorti / vecrsorti  result = the list after the in-place sort
# A list-valued result that is empty is written as an empty result field (not `undef`).
# setbinop takes a code reference and is skipped; sumset is its (A+B) special case.
use strict; use warnings;

sub gen_vec {
  my $fh = shift;
  my $B = sub { $_[0] ? 1 : 0 };
  my $L = sub { @_ ? @_ : ('') };       # empty list -> empty field
  my $A = sub { my ($x,$y) = @_; defined $y ? [@$x, '|', @$y] : [@$x] };
  my $big = '18446744073709551616';
  srand(20261016);

  # ---- single-list functions (t/26-vec.t, t/26-mex.t)
  my @lists = ([],[1],[0],[-1],[1,1],[1,2],[2,1],[1,2,1],[-6,0,4,-5,6,-6,0],[-6,0,4,-5,7,-6,0],
    [6,0,4,-5,6,-6,0],[7,0,4,-5,7,-8,0],['27944220269257565027','81033966278481626507','27944220269257565027'],
    [0,'-2147483648','2147483648'],[0,'-4294967296','4294967296'],[0,'-9223372036854775808','9223372036854775808'],
    ['18446744073709551615','-18446744073709551615','18446744073709551615'],['18446744073709551616','18446744073709551616','18446744073709551616'],
    ['-9223372036854775807','-9223372036854775807',-1],['-9223372036854775807','-9223372036854775807',-3],
    ['-9223372036854775807',0],['-9223372036854775807',-2],[0,'-9223372036854775808',0],['-9223372036854775295',-514],
    ['18446744073709540400',(1000) x 80],[2,-1,-2],[32767,-65535],[32768,-65535],[32768,-65536],
    [1..10,1..10],[5,3,'9223372036854775812','9223372036854775809','9223372036854775810','9223372036854775811'],
    [1..50],[3000000000,3000000000],[2147483647,1],[-2147483648,-1],['9000000000000000000','9000000000000000000'],
    ['9223372036854775807',1],['-9223372036854775808',-1],[3,-1,3,0,1,-5],[15,1..10,4,1..10,-2],
    [-1,'18446744073709551615'],[0,'18446744073709551615',0,4294967295,'18446744073709551615',4294967295],
    ['-9223372036854775808','9223372036854775807',4294967295,'9223372036854775807',4294967295,'-9223372036854775808'],
    [-5..5,reverse(-5..5),-5..5],[2,'02',2,'02'],['5',2,1,3,4],
    [0,1,2,4],[0,1,24,4],[4,2,1,0],[3,'100000000000000000000',0,2],[1,2,4],[0,1,2,3,4,5],
    [1,2,3,5],[1,2,24,5],[5,3,2,1],[4,'100000000000000000000',1,3],[2,3,5],
    ['-3937',4322,'-3619',-390,2039,2123,'-1614',-879,-4372,1793,4404,4229,286,-3613,2707,-4166]);
  for (1..80) { push @lists, [ map { int(rand(41)) - 20 } 1..int(rand(14)) ]; }
  for (1..20) { push @lists, [ map { int(rand(8)) } 1..int(rand(10)) ]; }
  for (1..12) { push @lists, [ map { (int(rand(5)) - 2) . '0' x int(rand(25)) . int(rand(10)) } 1..(1+int(rand(7))) ]; }
  for my $l (@lists) {
    my @l = @$l;
    my $a = $A->($l);
    emit($fh, 'vecsum', $a, sub { vecsum(@l) });
    emit($fh, 'vecprod', $a, sub { vecprod(@l) });
    emit($fh, 'vecmin', $a, sub { vecmin(@l) });
    emit($fh, 'vecmax', $a, sub { vecmax(@l) });
    emit($fh, 'vecprefixsum', $a, sub { $L->(vecprefixsum(@l)) });
    emit($fh, 'vecuniq', $a, sub { $L->(vecuniq(@l)) });
    emit($fh, 'vecsingleton', $a, sub { $L->(vecsingleton(@l)) });
    emit($fh, 'vecfreq', $a, sub { $L->(vecfreq(@l)) });
    emit($fh, 'vecsort', $a, sub { $L->(vecsort(@l)) });
    emit($fh, 'vecrsort', $a, sub { $L->(vecrsort(@l)) });
    emit($fh, 'vecsorti', $a, sub { my $r = [@l]; vecsorti($r); $L->(@$r) });
    emit($fh, 'vecrsorti', $a, sub { my $r = [@l]; vecrsorti($r); $L->(@$r) });
    emit($fh, 'vecmex', $a, sub { vecmex(@l) });
    emit($fh, 'vecpmex', $a, sub { vecpmex(@l) });
    emit($fh, 'toset', $a, sub { $L->(@{toset(@l)}) });
    emit($fh, 'is_sidon_set', $a, sub { $B->(is_sidon_set([@l])) });
    emit($fh, 'is_sumfree_set', $a, sub { $B->(is_sumfree_set([@l])) });
    emit($fh, 'sumset', $a, sub { $L->(@{sumset([@l])}) });
  }
  for my $n (0..50) { emit($fh, 'vecprod', [1..$n], sub { vecprod(1..$n) }); }
  # vecsum over the 'a a' undef-like strings is invalid input: not covered.

  # ---- vecextract (t/26-vec.t)
  my @ve = ([['a'..'z'],12345758],[[10,20,30],0],[[10,20,30],7],[[10,20,30],5],[[10,20,30],-1],[[1..10],1023],[[1..10],'1024'],
            [[1..64],'18446744073709551615'],[[1..70],'18446744073709551616']);
  for my $t (@ve) { my ($l,$m) = @$t;
    emit($fh, 'vecextract', [@$l,'|',$m], sub { $L->(vecextract([@$l],$m)) });
  }
  emit($fh, 'vecextract', [qw/a b c d e f g h i j k l m n o p q r s t u v w x y z | 22 14 17 10 18/], sub { 1 }) if 0;
  for my $t ([['a'..'z'],[22,14,17,10,18]],[['a'..'z'],[15,17,8,12,4]],[[1..5],[0,0,4]],[[1..5],[]],[[1..5],[5]],[[1..5],[-1]],[[10,20,30],[2,1,0]]) {
    my ($l,$ix) = @$t;
    emit($fh, 'vecextract_idx', [@$l,'|',@$ix], sub { $L->(vecextract([@$l],[@$ix])) });
  }
  for my $mask (0..31) { emit($fh, 'vecextract', [1..5,'|',$mask], sub { $L->(vecextract([1..5],$mask)) }); }

  # ---- two-list functions
  my @pairs = (
    [[7,1,3,5,1],[3,7,8,3,9]],
    [['9223372036854775808','9223372036854775807'],['9223372036854775807','9223372036854775810']],
    [['10223372036854775808','-9223372036854775807'],['-9223372036854775807','10223372036854775810']],
    [['59724578844314338843734830435499460367','98198086365677506205371483123156488634'],
     ['606571739116749108206251582180042583662','98198086365677506205371483123156488634']],
    [[map {"1844674407370955161$_"} 6,5,7,4,8],[map {"1844674407370955161$_"} 6,5,3]],
    [[0,1,2],[2,3,4]],[[0,1,2],[0,2,3,4]],[[],[1,2,3]],[[1,2,3],[]],[[],[]],[[-5..1],[-1..5]],
    [['-9223372036854775807','10223372036854775808'],['-9223372036854775807','10223372036854775810']],
    [[-20,-16,-14,-12,-10,0,12,14],[-30,-18,-14,-11,-10,-8,1,13,14]],
    [[1,2,3],[-11,-5,10]],
    [[-10..10],[-5..5]],[[-10..10],[-10..10]],[[5..8],[4..7]],[[5..8],[1..3]],
    [['-9223372036854775807'],[0]],[['18446744073709551615'],['18446744073709551616']],
    [['18446744073709551615','18446744073709551616'],['18446744073709551616']],
    [[1,2],[3,4]],[[1,2],[3,4,'18446744073709551615']],[[1,2],[3,4,'18446744073709551605']],
    [[1,2,'18446744073709551615'],[3,4]],[[1,2,'9223372036854775808'],[3,4,'9223372036854775808']],
    [[-1,'4611686018427387904'],[3,'9911686018427387905']],[[-100,-99],['9223372036854775808','9223372036854775809']],
    [[-1,1],[3,'9911686018427387905']],[[-1,'4611686018427387904'],[3,'4611686018427387905']],
    [[-1,'4611686018427387904'],[-3,'4611686018427387905']],[[1,2,'-4611686018427387904'],[3,4,'-4611686018427387904']],
    [[1,2,'-4611686018427387905'],[3,4,'-4611686018427387905']],[['-9223372036854775808'],[-1]],
    [[-1,2],[3,4]],[[-6,2],[3,4]],[[-6,2],[-3,4]],[[-6,-2],[-4,-3]],[[1,2],[3,4,'73786976294838206464']],
    [[2,4,6,8],[3,5,7]],[[1,2,3],[2,3,4]],[[1],[2]],[[1],[]],[[],[2]],[[0,3,7,12],[2,4,11,14]],
    [[1,2,3],[1,2,3,4]],[[1,2,3,4],[1,2,3]],[[1,2],[2,1]],[[1,2],[2,3]],[[1],[1]],[[1],[2]],[[3,1,2],[2,4,6]],
  );
  for (1..150) {
    my @x = map { int(rand(25)) - 8 } 1..int(rand(9));
    my @y = map { int(rand(25)) - 8 } 1..int(rand(9));
    push @pairs, [[@x],[@y]];
  }
  for (1..20) { push @pairs, [[map { Math::BigInt->new($big)->badd(int(rand(6)) - 3)->bstr } 1..4], [map { Math::BigInt->new($big)->badd(int(rand(6)) - 3)->bstr } 1..4]]; }
  for my $p (@pairs) {
    my ($x,$y) = @$p;
    my $a = $A->($x,$y);
    my @x = @$x; my @y = @$y;
    emit($fh, 'vecequal', $a, sub { $B->(vecequal([@x],[@y])) });
    emit($fh, 'sumset', $a, sub { $L->(@{sumset([@x],[@y])}) });
    emit($fh, 'setunion', $a, sub { $L->(@{setunion([@x],[@y])}) });
    emit($fh, 'setintersect', $a, sub { $L->(@{setintersect([@x],[@y])}) });
    emit($fh, 'setminus', $a, sub { $L->(@{setminus([@x],[@y])}) });
    emit($fh, 'setdelta', $a, sub { $L->(@{setdelta([@x],[@y])}) });
    # relations take distinct elements (any order): dedupe first
    my %sx; my @dx = grep { !$sx{$_}++ } @x; my %sy; my @dy = grep { !$sy{$_}++ } @y;
    my $da = $A->(\@dx,\@dy);
    for my $f (qw/set_is_disjoint set_is_equal set_is_subset set_is_proper_subset set_is_superset set_is_proper_superset set_is_proper_intersection/) {
      my $code = \&{"Math::Prime::Util::$f"};
      emit($fh, $f, $da, sub { $B->($code->([@dx],[@dy])) });
    }
    # set-form operations: normalise both sides with toset
    my @sx = @{toset(@x)}; my @sy = @{toset(@y)};
    my $sa = $A->(\@sx,\@sy);
    emit($fh, 'setinsert', $sa, sub { my $s = [@sx]; setinsert($s,@sy); $L->(@$s) });
    emit($fh, 'setinsert_n', $sa, sub { my $s = [@sx]; setinsert($s,@sy) });
    emit($fh, 'setremove', $sa, sub { my $s = [@sx]; setremove($s,@sy); $L->(@$s) });
    emit($fh, 'setremove_n', $sa, sub { my $s = [@sx]; setremove($s,@sy) });
    emit($fh, 'setinvert', $sa, sub { my $s = [@sx]; setinvert($s,@sy); $L->(@$s) });
    emit($fh, 'setinvert_n', $sa, sub { my $s = [@sx]; setinvert($s,@sy) });
    emit($fh, 'setinsert_set', $sa, sub { my $s = [@sx]; setinsert($s,[@sy]); $L->(@$s) });
    emit($fh, 'setremove_set', $sa, sub { my $s = [@sx]; setremove($s,[@sy]); $L->(@$s) });
    emit($fh, 'setinvert_set', $sa, sub { my $s = [@sx]; setinvert($s,[@sy]); $L->(@$s) });
    emit($fh, 'setcontains', $sa, sub { $B->(setcontains([@sx],@sy)) });
    emit($fh, 'setcontainsany', $sa, sub { $B->(setcontainsany([@sx],@sy)) });
    # single-value membership for the first few elements of the second list
    for my $v (@sy[0..($#sy < 1 ? $#sy : 1)]) {
      emit($fh, 'setcontains', [@sx,'|',$v], sub { $B->(setcontains([@sx],$v)) });
    }
  }
  # t/26-setops.t doc examples
  emit($fh, 'setcontains', [-12,1..20,'|',15], sub { $B->(setcontains([-12,1..20],15)) });
  emit($fh, 'setcontainsany', [-12,1..20,'|',-14,0,1,100], sub { $B->(setcontainsany([-12,1..20],-14,0,1,100)) });
  emit($fh, 'setinsert', [-10..-1,1..10,'|',0], sub { my $s = [-10..-1,1..10]; setinsert($s,0); $L->(@$s) });
  emit($fh, 'setinvert', [-10..10,'|',0], sub { my $s = [-10..10]; setinvert($s,0); $L->(@$s) });
  emit($fh, 'setremove', [-10..10,'|',0], sub { my $s = [-10..10]; setremove($s,0); $L->(@$s) });
  emit($fh, 'setinvert_set', [-10..10,'|',5,10,15,20], sub { my $s = [-10..10]; setinvert($s,[5,10,15,20]); $L->(@$s) });
  emit($fh, 'toset', [qw/1 -2147483647 3 2 2147483648/], sub { $L->(@{toset(qw/1 -2147483647 3 2 2147483648/)}) });
  my $b129 = Math::BigInt->new(2)->bpow(129);
  my @b129 = map { ($b129 + $_)->bstr } (-2,3,0,0,-2,0,3);
  emit($fh, 'toset', [@b129], sub { $L->(@{toset(@b129)}) });
  # sumset of the primes below 200
  my @pr = @{primes(200)};
  emit($fh, 'sumset', [@pr], sub { $L->(@{sumset([@pr])}) });
  emit($fh, 'sumset', [map { 1 << $_ } 1..10], sub { $L->(@{sumset([map { 1 << $_ } 1..10])}) });
  emit($fh, 'is_sidon_set', [map { 1 << $_ } 0..12], sub { $B->(is_sidon_set([map { 1 << $_ } 0..12])) });
  emit($fh, 'is_sidon_set', [0,1,4,10,12,17], sub { $B->(is_sidon_set([0,1,4,10,12,17])) });
  emit($fh, 'is_sumfree_set', [1,3,5,7,9], sub { $B->(is_sumfree_set([1,3,5,7,9])) });
  emit($fh, 'is_sumfree_set', [5..9], sub { $B->(is_sumfree_set([5..9])) });
  emit($fh, 'vecpmex', [0], sub { vecpmex(0) });
  # sigma-mex check of t/26-mex.t is covered by the vecpmex lists above.
}

1;
