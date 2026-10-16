# Fixture family `rational`: t/26-contfrac.t, t/26-rationaltrees.t.
# bestrational is bestrational(x, dbound) in MPU 0.75 (x a real number string), not (num,den).
# Fractions "a/b" in next_farey / farey_rank are passed to MPU as [a,b] array references.
# farey_len is scalar(farey(n)) (the sequence length); it is not an MPU function name.
use strict; use warnings;

sub gen_rational {
  my $fh = shift;
  my $L = sub { @_ ? @_ : (q{}) };
  my $fr = sub { [ split m{/}, $_[0] ] };

  # contfrac / from_contfrac / convergents (t/26-contfrac.t)
  my @cf = ([0,1],[1,3],[4,11],[67,29],[121,23],[3,4837],[0xfff1,0x7fed],[83116,51639],[9238492834,2398702938777],
    ["243224233245235253407096734543059","4324213412343432913758138673203834"],[415,93],[649,200],[4,9],
    [22,7],[333,106],[355,113],[377,120],[3927,1250],[103993,33102],[104348,33215],[208341,66317],[312689,99532],
    [833719,265381],[1146408,364913],[4272943,1360120],[80143857,25510582],
    ["262452630335382199398","83541266890691994833"],[144,233],["7540113804746346429","12200160415121876738"],
    ["280571172992510140037611932413038677189525","453973694165307953197296969697410619233826"],
    [62832,20000],[0,2],[8,22],[-8,2],[-93,37],[-312689,99532],[-4,11],[-4,5837],[-4,11111],[-1,11111],
    [-11110,11111],[-11112,11111],[-1,1],[-7,1],[-100,1],[0,1],[1,1],[7,1],[100,1],[1,2],[1,3],[1,100],[0,100],
    ["1180591620717411303425","590295810358705651712"],["18446744073709551615","18446744073709551614"],
    ["18446744073709551615","1"],["1","18446744073709551615"],[5,0],[0,0],[7,-3],[-7,-3],[2,4],[6,4]);
  for my $a (-12..12) { for my $b (1..14) { push @cf, [$a,$b]; } }
  for my $t (@cf) { my ($n,$d) = @$t;
    emit($fh, 'contfrac', [$n,$d], sub { contfrac($n,$d) });
    emit($fh, 'convergents', [$n,$d], sub { map { @$_ } convergents(contfrac($n,$d)) });
  }
  my @fc = ([0],[1],[3],[-3],[-3,7],[0,3],[0,100],[3,7,15,1,292],[3,7,16],[1,1,1,1,1,1,1,1,1,1,1],[0,2,1,3],
    [2,3,4,2],[5,3,1,5],[-3,2,18],[-4,1,6,15,1,292,1,2],[2,"590295810358705651712"],
    [0,(1)x90,2],[0,259,1,1,1,3,1,7,2,3,7,2,1,1,2,4,2,1,10,5,3,1,5,6],
    [3,7,15,1,292,1,1,1,2,1,3,1,14,2,1,1,2,2,2,2,1,84,2,1,1,15,3,13,1,4,2,6,6,99],
    [1,2,3,4,5,6,7,8,9,10],[-1],[-1,1],[-2,1,1],[0,0],[1,0],[5,0,3]);
  for my $l (@fc) {
    emit($fh, 'from_contfrac', [@$l], sub { from_contfrac(@$l) });
    emit($fh, 'convergents_list', [@$l], sub { map { @$_ } convergents(@$l) });
  }
  emit($fh, 'from_contfrac', [], sub { from_contfrac() });
  emit($fh, 'convergents', [], sub { $L->(map { @$_ } convergents()) });
  # convergents of a CF list: name `convergents` takes (num,den) above; the list form is convergents_list.

  # bestrational(x, dbound)
  my @br = ([0,10],[1,10],[7,10],[0.5,10],[2.5,10],[1.618,1],[1.4,1],[1.5,1],[-1.618,1],[-0.5,10],[-2.5,10],
    [2147483647.4,3],["1e-10","4294967295"],[3.14159265358979,5],[3.14159265358979,7],[3.14159265358979,100],
    [3.14159265358979,113],[3.14159265358979,1000],[1.41421356237310,5],[1.41421356237310,70],
    ["3.14159265358979323846",1000],["abc",10],["NaN",10],["Inf",10],[0.1,10],[0.3333,10],[0.3333,1000],
    [-3.14159265358979,50],[123.456,20],[123.456,1000],[1e-5,1000],[0.999,10],[0.001,10],[2.718281828459045,200],[2.718281828459045,0]);
  push @br, [3.14159265358979, $_] for 1..60;
  push @br, [1.41421356237310, $_] for 1..30;
  emit($fh, 'bestrational', [@$_], sub { &bestrational(@$_) }) for @br;

  # rational trees (t/26-rationaltrees.t)
  my @tree;
  my @s = ([1,1]); push @s, [next_calkin_wilf($s[-1][0],$s[-1][1])] for 1..99; push @tree, @s;
  for my $t (@tree, [4,11],[22,7],[37,53],[144,233],[83116,51639],[64,65],[66,65],[32,1],[64,1],
             ["228909276746","645603216423"],[2,4],[0,1],[1,0],[0,0],[-1,2],[1,"18446744073709551615"],
             ["18446744073709551615","18446744073709551614"]) {
    my ($n,$d) = @$t;
    emit($fh, 'next_calkin_wilf', [$n,$d], sub { next_calkin_wilf($n,$d) });
    emit($fh, 'next_stern_brocot', [$n,$d], sub { next_stern_brocot($n,$d) });
    emit($fh, 'calkin_wilf_n', [$n,$d], sub { calkin_wilf_n($n,$d) });
    emit($fh, 'stern_brocot_n', [$n,$d], sub { stern_brocot_n($n,$d) });
  }
  my @sbs = ([1,1]); push @sbs, [next_stern_brocot($sbs[-1][0],$sbs[-1][1])] for 1..99;
  for my $t (@sbs) { my ($n,$d) = @$t;
    emit($fh, 'next_stern_brocot', [$n,$d], sub { next_stern_brocot($n,$d) });
    emit($fh, 'stern_brocot_n', [$n,$d], sub { stern_brocot_n($n,$d) });
    emit($fh, 'calkin_wilf_n', [$n,$d], sub { calkin_wilf_n($n,$d) });
  }
  my @idx = (0..130, 36, 519, 1990, 2730, 123456789, 960, 1423, 111333227, 4294967295, 4294967296,
    "18446744073709551615", "18446744073709551616", "36893488147419103230", "36893488147419103233",
    "27670116110564327423", "55340232221128654848", "1054982144710410407556", "667408827216638861715");
  for my $i (@idx) {
    emit($fh, 'nth_calkin_wilf', [$i], sub { nth_calkin_wilf($i) });
    emit($fh, 'nth_stern_brocot', [$i], sub { nth_stern_brocot($i) });
  }
  for my $i (@idx, 4691, 87339, 1222997, 9786539, 76895573, 357214891, 1431655083, 5726623019, 22906492075, 91625925291, -1) {
    emit($fh, 'nth_stern_diatomic', [$i], sub { nth_stern_diatomic($i) });
  }

  # Farey (n <= 30 for the full list)
  for my $n (0..30) {
    emit($fh, 'farey', [$n], sub { $L->(map { @$_ } farey($n)) });
    emit($fh, 'farey_len', [$n], sub { scalar farey($n) });
  }
  for my $n (1..12) {
    my $len = scalar farey($n);
    for my $k (0..$len+1) { emit($fh, 'farey', [$n,$k], sub { my $r = farey($n,$k); $r ? @$r : () }); }
    my @f = farey($n);
    for my $q (@f, [2,1], [3,2], [5,7], [4,8]) {
      my ($a,$b) = @$q; my $str = "$a/$b";
      emit($fh, 'farey_rank', [$n,$str], sub { farey_rank($n, $fr->($str)) });
      emit($fh, 'next_farey', [$n,$str], sub { my $r = next_farey($n, $fr->($str)); $r ? @$r : () });
    }
  }
  for my $nk ([24,16],[507,427],[100,0],[100,3044],[1000,1],[1000,300000],[1000,304192],["10",33],[5,"10000000000"],[1,0],[1,1],[1,2]) {
    my ($n,$k) = @$nk;
    emit($fh, 'farey', [$n,$k], sub { my $r = farey($n,$k); $r ? @$r : () });
  }
  for my $nf ([24,"2/21"],[507,"3/505"],[9,"5/9"],[9,"1/1"],[9,"2/1"],[9,"0/1"],[100,"1/3"],[100,"1/7"],[1000,"1/2"],[1000,"355/113"],
              [100,"6/9"],[100,"3/7"],[1000000,"1/3"]) {
    my ($n,$f) = @$nf;
    emit($fh, 'farey_rank', [$n,$f], sub { farey_rank($n, $fr->($f)) });
    emit($fh, 'next_farey', [$n,$f], sub { my $r = next_farey($n, $fr->($f)); $r ? @$r : () });
  }
}

1;
