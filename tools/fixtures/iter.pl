# Fixture family `iter`: t/32-1-iterators.t, t/32-2-for-range.t, t/32-3-for-factoring.t,
# t/32-4-for-combinatorial.t, t/32-5-lastfor.t.
# Each case records what an iteration API visits, space-joined, capped at 2000 entries.
# Token forms: forprimes etc. -> the value; forfactored/forsquarefree -> "n:p1*p2*..." ("n:" when no factors);
# forsquarefreeint -> n; forpart -> "a+b+c"; forcomp -> "a+b+c"; forcomb/forperm/forderange/formultiperm ->
# "a,b,c"; forsetproduct -> "a,b,c". An empty tuple is "()".
# Args: hash arguments of forpart/forcomp are key=value tokens; forsetproduct lists are separated by "|".
use strict; use warnings;

sub gen_iter {
  my $fh = shift;
  my $CAP = 2000;
  # run: $it->(\@out) fills @out with tokens, using $push->(tok) which stops the loop at the cap
  my $go = sub {
    my ($fn, $args, $it) = @_;
    emit($fh, $fn, $args, sub {
      my @o;
      my $push = sub { push @o, $_[0]; lastfor() if @o >= $CAP; };
      $it->($push);
      join(" ", @o) });
  };
  my $tup = sub { @_ ? join(",", @_) : "()" };
  my $sum = sub { @_ ? join("+", @_) : "()" };
  my $fac = sub { my $n = shift; "$n:" . join("*", @_) };

  # ---- forprimes ----
  for my $a (undef, '2:undef', 'undef:2', '-2:3', '2:-3', 'abc', '2:abc', '5.6') {
    my @a = defined $a ? split(/:/, $a) : ('undef');
    @a = ('undef') if !defined $a;
    my @v = map { $_ eq 'undef' ? undef : $_ } @a;
    emit($fh, 'forprimes', [@a], sub { my @o; if (@v == 1) { forprimes { push @o, $_ } $v[0]; } else { forprimes { push @o, $_ } $v[0], $v[1]; } join " ", @o });
  }
  for my $n (0, 1, 2, 3, 4, 5, 50, 100, 1000, 5000, 17389, 18000) {
    $go->('forprimes', [$n], sub { my $p = shift; forprimes { $p->($_) } $n; });
  }
  my @r2 = ([0,0],[0,1],[3,5],[3,6],[3,7],[5,7],[6,7],[5,11],[7,11],[2,20],[20,30],[199,223],[31398,31468],
            [2147483647,2147483659],[3842610774,3842611326],[1,100],[100,1],[2,2],[10,10],[11,11],
            [1000000,1002000],[4294967290,4294967400],['18446744073709551000','18446744073709551615'],
            ['18446744073709551557','18446744073709551616'],['18446744073709551600','18446744073709551700']);
  my $E66 = Math::BigInt->new(2)->bpow(66);
  my $p66 = sub { my $add = shift; $E66 + $add };
  push @r2, map { [ "".($E66+$_->[0]), "".($E66+$_->[1]) ] } ([166,199],[0,300]);
  push @r2, ["737869762948382".sprintf("%05d",166+6464), "737869762948382".sprintf("%05d",199+6464)];
  for my $r (@r2) {
    my ($lo, $hi) = @$r;
    $go->('forprimes', [$lo,$hi], sub { my $p = shift; forprimes { $p->("$_") } $lo, $hi; });
  }
  { my $sum = 0; forprimes { $sum += int(12345/$_) } 1000;
    emit($fh, 'forprimes_sum_12345_div_p', [1000], sub { $sum }); }
  $go->('forprimes_nested3', [10], sub { my $p = shift;
    forprimes { forprimes { forprimes { $p->($_) } $_, $_+10 } 10*$_, 10*$_+10 } 10; });
  emit($fh, 'forprimes_nested_divisors_count', [10], sub { my $s = 0; forprimes { my @d = divisors(6486480 * ($_-1)); $s += 1+$#d; } 2,10; $s });

  # ---- forcomposites / foroddcomposites ----
  for my $n (0, 1, 3, 4, 50, 100, 500, 2000) {
    $go->('forcomposites', [$n], sub { my $p = shift; forcomposites { $p->($_) } $n; });
    $go->('foroddcomposites', [$n], sub { my $p = shift; foroddcomposites { $p->($_) } $n; });
  }
  for my $r ([2147483647,2147483659],[200,410],[0,0],[4,4],[9,9],[10,10],[24,25],[100,1],[1000,1100],[1,1],
             ['4294967290','4294967400'],['18446744073709551550','18446744073709551615'],
             ["".($E66+1506), "".($E66+1508)], ["".($E66+1506), "".($E66+1511)]) {
    my ($lo, $hi) = @$r;
    $go->('forcomposites', [$lo,$hi], sub { my $p = shift; forcomposites { $p->("$_") } $lo, $hi; });
    $go->('foroddcomposites', [$lo,$hi], sub { my $p = shift; foroddcomposites { $p->("$_") } $lo, $hi; });
  }
  # ---- forsemiprimes / foralmostprimes ----
  for my $n (0, 1, 4, 100, 1000) {
    $go->('forsemiprimes', [$n], sub { my $p = shift; forsemiprimes { $p->($_) } $n; });
  }
  for my $r ([0,100],[1,100],[4,4],[100,200],[1000,1100],[1000000,1000300],["".($E66+98),"".($E66+99)],["".($E66+0),"".($E66+300)]) {
    my ($lo, $hi) = @$r;
    $go->('forsemiprimes', [$lo,$hi], sub { my $p = shift; forsemiprimes { $p->("$_") } $lo, $hi; });
  }
  $go->('foralmostprimes', [0,1,1000], sub { my $p = shift; foralmostprimes { $p->($_) } 0,1,1000; });
  $go->('foralmostprimes', [0,1000], sub { my $p = shift; foralmostprimes { $p->($_) } 0,1000; });
  for my $k (1 .. 10) {
    $go->('foralmostprimes', [$k,1000], sub { my $p = shift; foralmostprimes { $p->($_) } $k,1000; });
    $go->('foralmostprimes', [$k,100,400], sub { my $p = shift; foralmostprimes { $p->($_) } $k,100,400; });
  }
  $go->('foralmostprimes', [3,"".($E66+30),"".($E66+32)], sub { my $p = shift; foralmostprimes { $p->("$_") } 3,$E66+30,$E66+32; });

  # ---- forfactored / forsquarefree / forsquarefreeint ----
  for my $r ([0,0],[0,1],[1,1],[1,20],[1,100],[100,200],[100000000,100000010],[7193953,7194100],[2,2],[20,10],
             ["".($E66+29),"".($E66+30)],["".($E66+26),"".($E66+29)],["4294967290","4294967310"],
             ["18446744073709551600","18446744073709551615"]) {
    my ($lo, $hi) = @$r;
    $go->('forfactored', [$lo,$hi], sub { my $p = shift; forfactored { $p->($fac->("$_", map {"$_"} @_)) } $lo, $hi; });
    $go->('forsquarefree', [$lo,$hi], sub { my $p = shift; forsquarefree { $p->($fac->("$_", map {"$_"} @_)) } $lo, $hi; });
    $go->('forsquarefreeint', [$lo,$hi], sub { my $p = shift; forsquarefreeint { $p->("$_") } $lo, $hi; });
  }
  for my $n (0, 1, 2, 30, 100, 500) {
    $go->('forfactored', [$n], sub { my $p = shift; forfactored { $p->($fac->("$_", @_)) } $n; });
    $go->('forsquarefree', [$n], sub { my $p = shift; forsquarefree { $p->($fac->("$_", @_)) } $n; });
    $go->('forsquarefreeint', [$n], sub { my $p = shift; forsquarefreeint { $p->("$_") } $n; });
  }
  emit($fh, 'forfactored_sum', ['0 0'], sub { my $s = 0; forfactored { $s += 1+$_ } 0,0; $s });
  emit($fh, 'forfactored_sum_vecsum', [100], sub { my $s = 0; forfactored { $s += vecsum($_,@_) } 100; $s });
  emit($fh, 'forsquarefree_sum_vecsum', [100], sub { my $s = 0; forsquarefree { $s += vecsum($_,@_) } 100; $s });
  emit($fh, 'forfactored_sum_vecsum', ['100000000 100000010'], sub { my $s = 0; forfactored { $s += vecsum($_,@_) } 1e8,1e8+10; $s });
  emit($fh, 'forsquarefree_sum', ['7193953 7195732'], sub { my $s = 0; forsquarefree { $s += $_ } 7193953,7195732; $s });
  emit($fh, 'forsquarefreeint_sum', ['7193953 7195732'], sub { my $s = 0; forsquarefreeint { $s += $_ } 7193953,7195732; $s });
  emit($fh, 'a053462', [6], sub { my ($s,$n)=(0,10**6-1); forsquarefree { $s += int($n / ($_*$_)) * ((scalar(@_) & 1)?-1:1); } sqrtint($n); $s });

  # ---- fordivisors ----
  for my $n (0, 1, 2, 12, 30, 36, 54321, 360, 720720, 1000000007, '4294967296', '18446744073709551615',
             '73786976294838225404', '18446744073709551616', '-5', 'abc') {
    $go->('fordivisors', [$n], sub { my $p = shift; fordivisors { $p->("$_") } $n; });
  }
  $go->('fordivisors_all_1_50', ['1..50'], sub { my $p = shift; for my $n (1..50) { fordivisors { $p->($_) } $n; } });
  emit($fh, 'fordivisors_sum_d_d2', [54321], sub { my $a = 0; fordivisors { $a += $_ + $_*$_ } 54321; $a });

  # ---- forpart / forcomp ----
  for my $n (0 .. 12) {
    $go->('forpart', [$n], sub { my $p = shift; forpart { $p->($sum->(@_)) } $n; });
    $go->('forcomp', [$n], sub { my $p = shift; forcomp { $p->($sum->(@_)) } $n; });
  }
  $go->('forcomp', [13], sub { my $p = shift; forcomp { $p->($sum->(@_)) } 13; });
  my @hashes = ({n=>3},{n=>1},{n=>7},{amin=>2},{amax=>3},{amin=>2,amax=>4},{nmin=>2},{nmax=>3},{nmin=>2,nmax=>4},
                {prime=>1},{n=>2,prime=>1},{n=>3,amin=>2},{amin=>3,amax=>3},{amax=>1},{nmax=>1},{nmin=>20},
                {amin=>0},{n=>0},{prime=>0},{nmax=>2,amax=>4},{n=>4,amin=>1,amax=>5});
  for my $n (7, 10, 12) {
    for my $h (@hashes) {
      my @kv = map { "$_=$h->{$_}" } sort keys %$h;
      $go->('forpart', [$n, @kv], sub { my $p = shift; forpart { $p->($sum->(@_)) } $n, {%$h}; });
      $go->('forcomp', [$n, @kv], sub { my $p = shift; forcomp { $p->($sum->(@_)) } $n, {%$h}; });
    }
  }
  for my $bad ({amin=>-1},{n=>'3.5'},{prime=>'abc'},{nmax=>'3.5'}) {
    my @kv = map { "$_=$bad->{$_}" } sort keys %$bad;
    emit($fh, 'forpart', [7, @kv], sub { my @o; forpart { push @o, $sum->(@_) } 7, {%$bad}; join " ", @o });
    emit($fh, 'forcomp', [7, @kv], sub { my @o; forcomp { push @o, $sum->(@_) } 7, {%$bad}; join " ", @o });
  }
  emit($fh, 'forpart_count', [4], sub { my $t = 0; forpart { $t++ } 4; $t });
  emit($fh, 'forcomp_count', [4], sub { my $t = 0; forcomp { $t++ } 4; $t });

  # ---- forcomb / forperm / forderange / formultiperm ----
  for my $n (0 .. 8) {
    $go->('forcomb', [$n], sub { my $p = shift; forcomb { $p->($tup->(@_)) } $n; });
    for my $k (0 .. $n + 1) {
      $go->('forcomb', [$n, $k], sub { my $p = shift; forcomb { $p->($tup->(@_)) } $n, $k; });
    }
  }
  for my $nk ([12,3],[12,5],[15,2],[5,'-1']) {
    $go->('forcomb', [@$nk], sub { my $p = shift; forcomb { $p->($tup->(@_)) } $nk->[0], $nk->[1]; });
  }
  for my $n (0 .. 6) {
    $go->('forperm', [$n], sub { my $p = shift; forperm { $p->($tup->(@_)) } $n; });
  }
  $go->('forperm', [7], sub { my $p = shift; forperm { $p->($tup->(@_)) } 7; });
  for my $n (0 .. 7) {
    $go->('forderange', [$n], sub { my $p = shift; forderange { $p->($tup->(@_)) } $n; });
  }
  for my $l ([], [1], [1,1,2], [1,2,3], [qw/s m i l e/], [1,1,2,2], [1,1,1], [qw/a a b b c/], [3,3,2,1], [0,0,1,1,2], [1,1,1,1,2,2]) {
    $go->('formultiperm', [@$l], sub { my $p = shift; formultiperm { $p->($tup->(@_)) } [@$l]; });
  }
  emit($fh, 'formultiperm', ['not-an-arrayref'], sub { my @o; formultiperm { push @o, @_ } 5; join " ", @o });
  emit($fh, 'forperm_count', [3], sub { my $t = 0; forperm { $t++ } 3; $t });
  emit($fh, 'forderange_count', [4], sub { my $t = 0; forderange { $t++ } 4; $t });
  emit($fh, 'formultiperm_count', ['1 1 2'], sub { my $t = 0; formultiperm { $t++ } [1,1,2]; $t });

  # ---- forsetproduct (lists separated by |) ----
  my @sets = ([[]],[[1..3]],[[1],[2],[3],[4],[5]],[[1,2],[3,4,5],[]],[[],[1,2],[3,4,5]],[[1,2],[qw/a b c/]],
              [[1,2,3],[4,5],[6,7,8]],[[0,1],[0,1],[0,1],[0,1]],[[1..5],[1..5],[1..5]],[[qw/x/],[qw/a b/]],
              [[1..50],[1..50]]);
  for my $s (@sets) {
    my $args = [ join(" | ", map { join(" ", @$_) } @$s) ];
    $args = ['(none)'] if !@$s;
    $go->('forsetproduct', $args, sub { my $p = shift; forsetproduct { $p->($tup->(@_)) } map { [@$_] } @$s; });
  }
  emit($fh, 'forsetproduct', ['1 2 (not array refs)'], sub { my @o; forsetproduct { push @o, "@_" } 1,2; join " ", @o });
  emit($fh, 'forsetproduct_mutate_arg', ['1 | a b'], sub { my @set=([1],[qw/a b/]); my @out; forsetproduct { push @out, join(",",@_); $_[0] = 9; } @set; join(" ", @out) . " / set0=$set[0][0]" });
  emit($fh, 'forsetproduct_clear_sources', ['1 2 | a b'], sub { my @a=(1,2); my @b=('a','b'); my @out; forsetproduct { push @out, join(",",@_); @a=(); @b=(); } \@a, \@b; join(" ", @out) });

  # ---- prime_iterator / prime_iterator_object ----
  for my $s (undef, 0, 1, 2, 3, 47, 199, 200, 31397, 31396, 31398, '68719476736', 4294967291, 4294967296,
             '18446744073709551557', '18446744073709551615', '100000000000000000000', -2, 'abc', 4.5) {
    my $a = defined $s ? [$s] : [];
    emit($fh, 'prime_iterator', $a, sub {
      my $it = defined $s ? prime_iterator($s) : prime_iterator();
      join " ", map { "".$it->() } 1..50; });
    emit($fh, 'prime_iterator_object', $a, sub {
      my $it = defined $s ? prime_iterator_object($s) : prime_iterator_object();
      join " ", map { "".$it->next } 1..50; });
    emit($fh, 'prime_iterator_object_iterate', $a, sub {
      my $it = defined $s ? prime_iterator_object($s) : prime_iterator_object();
      join " ", map { "".$it->iterate } 1..50; });
  }
  emit($fh, 'prime_iterator_nested3', [], sub {
    my @b; my $ita = prime_iterator();
    while ((my $a = $ita->()) <= 10) { my $itb = prime_iterator(10*$a);
      while ((my $b = $itb->()) <= 10*$a+10) { my $itc = prime_iterator($b);
        while ((my $c = $itc->()) <= $b+10) { push @b, $c; } } }
    join " ", @b; });
  # object methods (t/32-1-iterators.t)
  my $top = "18446744073709551557";
  my $pio = sub { my ($name, $args, $code) = @_; emit($fh, "pio_$name", $args, sub { my $it = prime_iterator_object(); my @r = $code->($it); map { defined $_ ? "$_" : "undef" } @r }); };
  $pio->('next10_value', [], sub { my $it = shift; $it->next for 1..10; $it->value });
  $pio->('prev_value', [], sub { my $it = shift; $it->next for 1..10; $it->prev; $it->value });
  $pio->('peek', [], sub { my $it = shift; $it->next for 1..10; $it->prev; $it->peek });
  $pio->('iterate_twice', [], sub { my $it = shift; $it->next for 1..10; $it->prev; ($it->iterate, $it->iterate) });
  $pio->('rewind_chain', [], sub { my $it = shift; $it->next for 1..10; $it->rewind->next->next->next->prev; $it->value });
  $pio->('rewind1', [], sub { my $it = shift; $it->next for 1..10; $it->rewind(1); $it->value });
  $pio->('rewind0', [], sub { my $it = shift; $it->next for 1..10; $it->rewind(0); $it->value });
  $pio->('rewind_top', [$top], sub { my $it = shift; $it->rewind($top); my $a = $it->value; $it->next; ($a, $it->value) });
  $pio->('rewind_uvmax', ['18446744073709551615'], sub { my $it = shift; $it->rewind(~0); my $a = $it->value; $it->prev; ($a, $it->value) });
  $pio->('tell_i', [], sub { my $it = shift; $it->rewind; $it->next for 1..100; $it->tell_i });
  $pio->('i_start', [], sub { my $it = shift; $it->i_start });
  $pio->('values_min', [], sub { my $it = shift; $it->values_min });
  $pio->('values_max', [], sub { my $it = shift; $it->values_max });
  $pio->('oeis_anum', [], sub { my $it = shift; $it->oeis_anum });
  $pio->('seek_to_i', [156], sub { my $it = shift; $it->seek_to_i(156)->value });
  $pio->('seek_to_value', [156], sub { my $it = shift; $it->seek_to_value(156)->value });
  $pio->('ith', [589], sub { my $it = shift; $it->ith(589) });
  $pio->('pred', [577], sub { my $it = shift; $it->pred(577) ? 1 : 0 });
  $pio->('pred', [578], sub { my $it = shift; $it->pred(578) ? 1 : 0 });
  for my $v (4289, 4290, 2, 3, 4, 1) {
    $pio->('value_to_i', [$v], sub { my $it = shift; $it->value_to_i($v) });
    $pio->('value_to_i_floor', [$v], sub { my $it = shift; $it->value_to_i_floor($v) });
    $pio->('value_to_i_ceil', [$v], sub { my $it = shift; $it->value_to_i_ceil($v) });
  }
  $pio->('value_to_i_estimate', [4171510507], sub { my $it = shift; int($it->value_to_i_estimate(4171510507) / 1000) });  # within +-500 of 197710788: coarse /1000 -> 197710

  # ---- lastfor (t/32-5-lastfor.t) ----
  my $lf = sub { my ($name, $args, $code) = @_; emit($fh, "lastfor_$name", $args, sub { my @r = $code->(); join " ", @r }); };
  $lf->('forprimes_forprimes', [100,1000], sub { my @zn; forprimes { my $p=$_; forprimes { lastfor, push @zn,$_ if $_ % $p == 1; } 1000; } 100; @zn });
  $lf->('forprimes_forcomposites', [100,1000], sub { my @zn; forprimes { my $p=$_; forcomposites { lastfor, push @zn,$_ if $_ % $p == 1; } 1000; } 100; @zn });
  $lf->('forprimes_foroddcomposites', [100,1000], sub { my @zn; forprimes { my $p=$_; foroddcomposites { lastfor, push @zn,$_ if $_ % $p == 1; } 1000; } 100; @zn });
  $lf->('fordivisors_power', ['1..20'], sub { my @p; for my $n (1..20) { fordivisors { lastfor,push @p,$_ if is_power($_) } $n; } @p });
  $lf->('forpart_len4', [7], sub { my $c; forpart { lastfor,return if @_ < 4; $c++; } 7; $c });
  $lf->('forcomp_len4', [7], sub { my $c; forcomp { lastfor,return if @_ < 4; $c++; } 7; $c });
  $lf->('forcomb_sum11', [7,4], sub { my $c; forcomb { lastfor,return if vecsum(@_) > 11; $c++; } 7,4; $c });
  $lf->('forperm_pos3_is5', [7], sub { my $t; forperm { lastfor,return if $_[3]==5; $t++; } 7; $t });
  $lf->('forderange_pos3_is5', [7], sub { my $t = 0; forderange { lastfor,return if $_[3]==5; $t++; } 7; $t });
  $lf->('formultiperm_miles', ['s m i l e'], sub { my $t = 0; formultiperm { lastfor if "miles" eq join("",@_); $t++; } [split(//,"smile")]; $t });
  $lf->('nested_ge7', [20], sub { my @ps; forprimes { lastfor if $_ >= 7; push @ps, $_; forcomposites { push @ps,$_; } $_; } 20; @ps });
  $lf->('forcomposites_gt2000', [20000], sub { my $t = 0; forcomposites { $t=$_; lastfor if $_ > 2000; } 20000; $t });
  $lf->('forpart_0', [0], sub { my $t = 0; forpart { $t++; lastfor; } 0; $t });
  $lf->('forcomp_0', [0], sub { my $t = 0; forcomp { $t++; lastfor; } 0; $t });
  $lf->('forcomb_0', [0], sub { my $t = 0; forcomb { $t++; lastfor; } 0; $t });
  $lf->('forcomb_3_0', [3,0], sub { my $t = 0; forcomb { $t++; lastfor; } 3,0; $t });
  $lf->('forperm_0', [0], sub { my $t = 0; forperm { $t++; lastfor; } 0; $t });
  $lf->('forderange_0', [0], sub { my $t = 0; forderange { $t++; lastfor; } 0; $t });
  $lf->('formultiperm_empty', [''], sub { my $t = 0; formultiperm { $t++; lastfor; } []; $t });
  $lf->('forfactored_0_3', ['0 3'], sub { my $t = 0; forfactored { $t++; lastfor; } 0,3; $t });
  $lf->('forsquarefree_0_3', ['0 3'], sub { my $t = 0; forsquarefree { $t++; lastfor; } 0,3; $t });
  $lf->('forsquarefreeint_0_3', ['0 3'], sub { my $t = 0; forsquarefreeint { $t++; lastfor; } 0,3; $t });
  $lf->('formultiperm_first', ['1 1 2'], sub { my @out; formultiperm { push @out, "@_"; lastfor; } [1,1,2]; @out });
  $lf->('outside_loop', [], sub { eval { lastfor(); 1 } ? 'ok' : 'ERROR' });
  $lf->('after_callback_die', [], sub { eval { forprimes { die "x\n" } 10; 1 }; eval { lastfor(); 1 } ? 'ok' : 'ERROR' });
  $lf->('inner_die_outer_ok', [20], sub { my $n = 0; forprimes { eval { forprimes { die "inner\n" } 10 }; $n++; lastfor; } 20; $n });
  $lf->('forprimes_first_gt_100', [1000], sub { my $r; forprimes { if ($_ > 100) { $r = $_; lastfor } } 1000; $r });
}
1;
