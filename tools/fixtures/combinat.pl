# Fixture family `combinat`: t/24-partitions.t, t/26-binomial.t, t/26-combinatorial.t (numtoperm/permtonum),
# t/26-factorial.t, t/26-stirling.t, t/26-powersum.t, t/26-intcomplexity.t, t/27-bernfrac.t,
# t/20-primorial.t, t/21-conseq-lcm.t, t/26-frobeniusnum.t, t/26-congruentnum.t, t/26-polygonal.t,
# t/26-pillai.t, t/26-ishappy.t, t/26-isfundamental.t, t/26-isdivisible.t, t/26-isodd.t.
# Big results are written in full as MPU prints them, except binomial (u64_or_undef).
use strict; use warnings;

sub gen_combinat {
  my $fh = shift;
  my $B = sub { $_[0] ? 1 : 0 };
  my $L = sub { @_ ? @_ : (q{}) };   # empty list result -> empty field, not undef
  my $MAX = Math::BigInt->new('18446744073709551615');

  # partitions / partitionsq (t/24-partitions.t)
  for my $n (0..120, 255, 256, 300, 500, 501, 1000, 1001, 2347) {
    emit($fh, 'partitions', [$n], sub { partitions($n) });
  }
  for my $n (0..120, 300, 1000, 2000) { emit($fh, 'partitionsq', [$n], sub { partitionsq($n) }); }
  emit($fh, 'partitionsq', [-1], sub { partitionsq(-1) });
  emit($fh, 'partitions', [-1], sub { partitions(-1) });

  # binomial (t/26-binomial.t)
  my @bn = ([20,15],[35,16],[40,19],[67,31],[228,12],[34,16],[62,28],[67,30],[70,25],[125,61],[131,64],
            [177,78],[61,17],[-11,22],[-12,23],[-23,-26],[-12,-23],[12,-23],[12,-12],[-12,0],
            ["36893488147419103233",1],["36893488147419103233",2],["36893488147419103233",3],
            ["-36893488147419103233",2],["-36893488147419103233",3],
            ["18446744073709551615",1],["18446744073709551615",2],["18446744073709551615",0],
            ["18446744073709551615","18446744073709551614"],[64,32],[66,33],[67,33],[68,34]);
  for my $n (-10..10) { for my $k (-10..10) { push @bn, [$n,$k]; } }
  for my $n (-13, 13) { for my $k (-15..15) { push @bn, [$n,$k]; } }
  for my $n (11..70) { for my $k (0, 1, 2, int($n/2), $n-1, $n, $n+1) { push @bn, [$n,$k]; } }
  for my $r (@bn) { my ($n,$k) = @$r; emit($fh, 'binomial', [$n,$k], sub { u64_or_undef(binomial($n,$k)) }); }

  # catalan, bell, fubini, factorial, subfactorial (t/26-factorial.t)
  emit($fh, 'catalan_number', [$_], sub { catalan_number($_) }) for 0..60;
  emit($fh, 'bell_number', [$_], sub { bell_number($_) }) for 0..60;
  emit($fh, 'fubini', [$_], sub { fubini($_) }) for 0..30;
  emit($fh, 'factorial', [$_], sub { factorial($_) }) for (-5, -1, 0..100, 150, 170, 200);
  emit($fh, 'subfactorial', [$_], sub { subfactorial($_) }) for (-5, 0..50, 110);
  for my $k (0..10) { for my $n (-10..10) {
    emit($fh, 'falling_factorial', [$n,$k], sub { falling_factorial($n,$k) });
    emit($fh, 'rising_factorial', [$n,$k], sub { rising_factorial($n,$k) });
  } }
  for my $nk ([515,7],[516,7],[568,7],[89,10],[103,101],["36893488147419103233",2],[509,7],[510,7],[562,7],[80,10]) {
    my ($n,$k) = @$nk;
    emit($fh, 'falling_factorial', [$n,$k], sub { falling_factorial($n,$k) });
    emit($fh, 'rising_factorial', [$n,$k], sub { rising_factorial($n,$k) });
  }
  for my $k (0..4) { for my $n (-1, 0..40, 100) {
    emit($fh, 'multifactorial', [$n,$k], sub { multifactorial($n,$k) });
  } }
  for my $nk ([7,10],[35,2],[100,1],["1000000000000000000000000000000","1000000000000000000000000000001"]) {
    my ($n,$k) = @$nk; emit($fh, 'multifactorial', [$n,$k], sub { multifactorial($n,$k) });
  }

  # stirling (t/26-stirling.t)
  for my $t (1..3) { for my $n (0..22) { for my $m (0..$n+1) {
    emit($fh, 'stirling', [$n,$m,$t], sub { stirling($n,$m,$t) });
  } } }
  for my $n (0..15) { for my $m (0..$n+1) { emit($fh, 'stirling', [$n,$m], sub { stirling($n,$m) }); } }
  emit($fh, 'stirling', [95,78,1], sub { stirling(95,78,1) });
  emit($fh, 'stirling', [114,85,2], sub { stirling(114,85,2) });
  emit($fh, 'stirling', [4,3,4], sub { stirling(4,3,4) });

  # powersum (t/26-powersum.t)
  for my $n (0, 1, 2, 10, 16, 97, 115, 116, 117, 1234, 1711, 9838, 9839, 9840, "36893488147419103232") {
    for my $k (0..12) { emit($fh, 'powersum', [$n,$k], sub { powersum($n,$k) }); }
  }
  emit($fh, 'powersum', [30,80], sub { powersum(30,80) });
  for my $nk ([1624,2],[67,4],[44,5],[19,6],[17,7],[9,8],[2642245,2],[5724,4],[1824,5],[482,6],[288,7],[115,8]) {
    my ($n,$k) = @$nk;
    emit($fh, 'powersum', [$_,$k], sub { powersum($_,$k) }) for ($n-1,$n,$n+1);
  }

  # integer_complexity (t/26-intcomplexity.t)
  emit($fh, 'integer_complexity', [$_], sub { integer_complexity($_) }) for (0..300, map({2**$_} 1..16), map({3**$_} 1..10),
      36, 216, 1296, 100000, 1000000, 9999999);

  # bernfrac / harmfrac (t/27-bernfrac.t)
  emit($fh, 'bernfrac', [$_], sub { bernfrac($_) }) for 0..60;
  emit($fh, 'harmfrac', [$_], sub { harmfrac($_) }) for 0..60;

  # primorial / pn_primorial (t/20-primorial.t)
  emit($fh, 'pn_primorial', [$_], sub { pn_primorial($_) }) for 0..60;
  emit($fh, 'primorial', [$_], sub { primorial($_) }) for (0..120, 541);

  # consecutive_integer_lcm (t/21-conseq-lcm.t)
  emit($fh, 'consecutive_integer_lcm', [$_], sub { consecutive_integer_lcm($_) }) for (0..100, 200);

  # frobenius_number (t/26-frobeniusnum.t)
  my @fr = ([1],[5,1],[6,9,20],[47,74,97,126,157,188],[83,87,93],[5,8,9,12],[7,11,13],[53,71,91],
    [322,654,765],[123,1234,12345],[151,157,251,711],[200,230,528,863,905,976,1355,1725,1796,1808],
    [10,18,26,33,35],[1218,1017,882,11],[43,40,38,37,34],[4,73,63,111],[10,195,218,272,287,324,341,353,499],
    [100,101,159],[133,172,199],[100,101,139],[110,151,201],[110,151,211],[137,251,256],[33,89,147],
    [100,239,543,609],[100,101,110,111],[30,31,43,55,67,98],[2000,3001,4567],[4093,8191,16381,32749,65521],
    [12223,12224,36671],[5123,5692,6055,6371,6899,7300],[1854,2712,2266,7857],
    [12223,12224,36674,61119,85569],[12228,36679,36682,48908,61139,73365],
    [13211,13212,39638,52844,66060,79268,92482],[11615,27638,32124,48384,53542,56230,73104,73884,112951,130204],
    [10000000000,10000008870,10000057783,10000072907],
    [12345,"1494268454735486"],[12345,"14948739119699798"],[4,8,"7000000000000000001"],[4,8,1500000001],
    [],[4093],[20,44],[2,3],[3,5],[6,10,15],[7,9],[1,1],[2,4],[0,5]);
  push @fr, [$_, $_+1] for 2..12;
  push @fr, [$_, $_+2, $_+5] for 3..9;
  emit($fh, 'frobenius_number', [@$_], sub { frobenius_number(@$_) }) for @fr;

  # is_congruent_number (t/26-congruentnum.t)
  emit($fh, 'is_congruent_number', [$_], sub { $B->(is_congruent_number($_)) }) for (-3, 0, 1..400);
  emit($fh, 'is_congruent_number', [1000000+$_], sub { $B->(is_congruent_number(1000000+$_)) }) for 1..10;
  emit($fh, 'is_congruent_number', [$_], sub { $B->(is_congruent_number($_)) })
    for (19,43,59,17,73,89,26,58,74,82,33,57,129,51,123,187,35,91,115,130,290,370,66,114,258,42,186,266,
         969,2193,2409,105,273,345,483,1491,2387,161601,255057,203433,761442,2679170,58654083,53922,73986);

  # is_polygonal (t/26-polygonal.t)
  for my $s (3..25) { emit($fh, 'is_polygonal', [$_,$s], sub { $B->(is_polygonal($_,$s)) }) for 0..200; }
  for my $s (3, 4, 5, 6, 8, 12, 100, 4294967297) { for my $n (@{[ -1, 0, 1, 2, 3, 100, 1000003, "724424175519274711242", "510622052816898545467859772308206986101878" ]}) {
    emit($fh, 'is_polygonal', [$n,$s], sub { $B->(is_polygonal($n,$s)) });
  } }

  # is_pillai (t/26-pillai.t): returns the witness or 0
  emit($fh, 'is_pillai', [$_], sub { is_pillai($_) }) for (0..1000, 1059511);

  # is_happy (t/26-ishappy.t)
  emit($fh, 'is_happy', [$_], sub { is_happy($_) }) for (0..800, 7777777777, 778688, 11025, 69378, 78999,
      10234456789, 1234456789, "1034567892987654301", "13456789298765431", "99194853094755497",
      31, 1880, 7839, 44488, "7899999999999959999999996");
  for my $bx ([3,2,214],[5,2,250],[16,2,149],[36,2,500],[10,3,1000],[10,2,100]) {
    my ($b,$e,$m) = @$bx;
    emit($fh, 'is_happy', [$_,$b], sub { is_happy($_,$b) }) for 0..$m;
    emit($fh, 'is_happy', [$_,$b,$e], sub { is_happy($_,$b,$e) }) for 0..$m;
  }
  emit($fh, 'is_happy', [26353,11,10], sub { is_happy(26353,11,10) });

  # is_fundamental (t/26-isfundamental.t)
  emit($fh, 'is_fundamental', [$_], sub { $B->(is_fundamental($_)) }) for (-200..200,
      "147573952589676413001", "-147573952589676412911");

  # is_divisible / is_congruent (t/26-isdivisible.t)
  my @divt = ([0,0],[17,0],[0,1],[123,1],[-123,1],[0,2],[1,2],[2,2],[-2,2],
    ["340282366920938463463374607431768211456",2],["340282366920938463463374607431768211457",2],
    ["3689348814741910323",3],["3689348814741910322",3],["68056473384187692692674921486353642291",3],
    ["68056473384187692692674921486353642290",3],["3689348813882916864","6442450944"],
    ["68056473384187692688985572671611731968","27670116110564327424"],
    ["408338840305126156152360180103379943424","27670116110564327424"],
    ["10223372036854775807","-10223372036854775807"],["36472996418050588672","33171997"]);
  for my $x (-20..20) { push @divt, [$x,0]; for my $d (-9..9) { next unless $d; push @divt, [$x*7919,$d]; push @divt, [$x*$d,$d]; } }
  for my $x (@{[ 4294967295, "18446744073709551615", "9223372036854775808", "-9223372036854775808" ]}) {
    push @divt, [$x,$_] for (1..9, 4294967296);
  }
  emit($fh, 'is_divisible', [@$_], sub { $B->(&is_divisible(@$_)) }) for @divt;
  my @congt = ([0,0,0],[11,11,0],[3,11,0],[0,0,1],[1,0,1],[0,1,1],[123,456,1],
    ["335812727629498640265","2812431594283598168865",1],["3689348814741910323","858993459","6442450944"],
    ["68056473384187692692674921486353642291","3689348814741910323","27670116110564327424"],
    ["18325193793","-9162596895","13743895344"],["78706108047827420225","-39353054023913710111","59029581035870565168"]);
  for my $c (-2..2) { push @congt, [$_,$c,0] for -3..3; }
  push @congt, [$_,3,13] for -30..40;
  push @congt, [$_,-27,17] for -30..40;
  emit($fh, 'is_congruent', [@$_], sub { $B->(&is_congruent(@$_)) }) for @congt;

  # is_odd / is_even (t/26-isodd.t)
  emit($fh, 'is_odd', [$_], sub { $B->(is_odd($_)) }) for (-50..50, "9832494848388408230982349082309487", "-9832494848388408230982349082309487", "18446744073709551615");
  emit($fh, 'is_even', [$_], sub { $B->(is_even($_)) }) for (-50..50, "98324948483884082309823490823094875", "-98324948483884082309823490823094874", "18446744073709551614");

  # hclassno (Hurwitz class number, not in a named t file; docs)
  emit($fh, 'hclassno', [$_], sub { hclassno($_) }) for (-3..200, 1000003, 4294967295);

  # numtoperm / permtonum (t/26-combinatorial.t)
  my @ntp = ([0,0],[1,0],[1,1],[5,15],[5,-1],[5,"100000000000000000000"],[24,987654321],[14,8467582],
             [3,0],[3,5],[3,6],[3,7],[4,23],[4,24],[6,719],[10,3628799]);
  push @ntp, [4,$_] for 0..23;
  emit($fh, 'numtoperm', [@$_], sub { $L->(&numtoperm(@$_)) }) for @ntp;
  for my $p ([], [0], [6,3,4,2,5,0,1], [reverse(0..14),15..19], [reverse(0..12),reverse(13..25)],
             [numtoperm(14,8467582)], [0,1,2], [2,1,0], [1,0,2], [1,2,0], [1,1], [0,2]) {
    emit($fh, 'permtonum', [@$p], sub { permtonum([@$p]) });
  }
}

1;
