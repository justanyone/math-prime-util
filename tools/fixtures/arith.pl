# Fixture family `arith`: number-theoretic functions (totients, Moebius, Liouville, valuation, Kronecker,
# CRT, multiplicative order, primitive roots, divisor sums, Ramanujan functions).
# Covers MPU t/19-totients.t, 20-jordantotient.t, 19-moebius.t, 19-liouville.t, 19-mangoldt.t,
# 19-valuation.t, 19-kronecker.t, 19-chinese.t, 19-znorder.t, 19-primroots.t, 26-istotient.t,
# 19-ramanujan.t, 19-divisorsum.t.
# Signed inputs appear where MPU documents them (euler_phi, moebius, valuation, kronecker, chinese,
# znprimroot, is_fundamental). Results that exceed u64 print as `undef` (u64_or_undef); signed results
# (abundance, ramanujan_tau, mertens, ...) print with their minus sign. Arguments beyond u64 are left out.
use strict; use warnings;

sub gen_arith {
  my $fh = shift;
  my $max = '18446744073709551615';
  my @e = edges();
  my @small = (0..300);
  my @mid = (1000, 1729, 4095, 4096, 9973, 10000, 65535, 65536, 123456, 123457, 123456789, 999999937, 1000000007,
             2147483647, 4294967295, 4294967296, 4294967297, 6469693230, 223092870, 600851475143, 1099511627776,
             '9007199254740993', '9223372036854775783', '18446744073709551557', '18446744073709551556', '3369738766071892021');
  my @pts = (@small, @mid, @e);
  my %s; @pts = grep { !$s{"$_"}++ } @pts;
  my $B = sub { Math::BigInt->new("$_[0]") };
  my $m = $B->($max);
  my $o = sub { u64_or_undef($_[0]) };

  # --- euler_phi, carmichael_lambda, one-argument forms
  for my $n (@pts) {
    emit($fh, 'euler_phi', [$n], sub { euler_phi("$n") });
    emit($fh, 'carmichael_lambda', [$n], sub { carmichael_lambda("$n") });
    emit($fh, 'moebius', [$n], sub { moebius("$n") }) if $n ne '0';
    emit($fh, 'liouville', [$n], sub { liouville("$n") });
    emit($fh, 'exp_mangoldt', [$n], sub { exp_mangoldt("$n") });
    emit($fh, 'is_totient', [$n], sub { is_totient("$n") ? 1 : 0 });
    emit($fh, 'is_fundamental', [$n], sub { is_fundamental("$n") ? 1 : 0 });
    emit($fh, 'dedekind_psi', [$n], sub { $o->(dedekind_psi("$n")) });
    emit($fh, 'aliquot_sum', [$n], sub { $o->(aliquot_sum("$n")) });
    emit($fh, 'abundance', [$n], sub { my $v = abundance("$n"); $v < -$m || $v > $m ? 'undef' : "$v" });
    emit($fh, 'sopfr', [$n], sub { sopfr("$n") });
    emit($fh, 'sopf', [$n], sub { sopf("$n") });
    emit($fh, 'znprimroot', [$n], sub { znprimroot("$n") });
    emit($fh, 'qnr', [$n], sub { qnr("$n") });
    for my $k (0..3) { emit($fh, 'divisor_sum', [$n, $k], sub { $o->(divisor_sum("$n", $k)) }); }
    emit($fh, 'divisor_sum', [$n], sub { $o->(divisor_sum("$n")) });
    for my $k (1..4, 6) { emit($fh, 'jordan_totient', [$k, $n], sub { $o->(jordan_totient($k, "$n")) }); }
    emit($fh, 'jordan_totient', [0, $n], sub { $o->(jordan_totient(0, "$n")) });
    emit($fh, 'jordan_totient', [10, $n], sub { $o->(jordan_totient(10, "$n")) }) if $n < 100000;
  }
  # negatives: documented as 0 / |n| (euler_phi(-n)=0, moebius(-n)=moebius(n), znprimroot(-n), is_fundamental(-n))
  for my $n (1..60, 123456, 100000898, 11, '9223372036854775807') {
    emit($fh, 'euler_phi', ["-$n"], sub { euler_phi("-$n") });
    emit($fh, 'moebius', ["-$n"], sub { moebius("-$n") });
    emit($fh, 'znprimroot', ["-$n"], sub { znprimroot("-$n") });
    emit($fh, 'is_fundamental', ["-$n"], sub { is_fundamental("-$n") ? 1 : 0 });
    emit($fh, 'dedekind_psi', ["-$n"], sub { dedekind_psi("-$n") });
    emit($fh, 'carmichael_lambda', ["-$n"], sub { carmichael_lambda("-$n") });
    emit($fh, 'is_totient', ["-$n"], sub { is_totient("-$n") ? 1 : 0 });
  }
  emit($fh, 'moebius', [0], sub { moebius(0) });
  # --- ranges (t/19-totients.t, t/19-moebius.t)
  my @ranges = ([0,0],[0,1],[1,2],[1,3],[2,3],[1,0],[100,90],[0,100],[10,20],[1513,1537],[1,240],[-5,5],[-7,5],[-14,-9],
                [4294967293,4294967295],[4294967293,4294967296],[4294967295,4294967297],[4294967296,4294967298],
                ['18446744073709551613','18446744073709551615'],['18446744073709551600','18446744073709551615'],
                [10000,50000],[1,300],['9223372036854775800','9223372036854775830'],[0,2000]);
  for my $r (@ranges) {
    my($lo,$hi)=@$r;
    emit($fh, 'euler_phi', [$lo,$hi], sub { euler_phi("$lo","$hi") });
    emit($fh, 'moebius', [$lo,$hi], sub { moebius("$lo","$hi") }) if !($lo eq '0' && $hi eq '0');
  }
  emit($fh, 'moebius', [14,'18446744073709551617'], sub { moebius(14, '18446744073709551617') }) if 0;

  # --- mertens, sumliouville
  my @mn = (0,1,2,3,4,5,10,100,1000,10000,8,16,32,64,128,256,512,1024,2048,4096,8192,100000,444444,1000000,10000000,
            6,30,210,2310,30030,510510,9699690,223092870,6433477,109851909,(map { 2**$_ } 14..23),10**8,10**9,
            '10000000000',20000000000,30000000000,40000000000,'100000000000');
  for my $n (@mn) { emit($fh, 'mertens', [$n], sub { mertens($n) }); }
  for my $r ([0,0],[0,10],[1,10],[5,10],[10,5],[12345,12345],[4294967293,4294967295],[12345,444444],[995001,1000000],[1000,2000],[1,1],[2,2]) {
    my($l,$h)=@$r; emit($fh, 'mertens', [$l,$h], sub { mertens($l,$h) });
  }
  emit($fh, 'mertens', [-1,5], sub { mertens(-1,5) });
  emit($fh, 'mertens', [1,-5], sub { mertens(1,-5) });
  my @liou = (qw/24 51 94 183 294 629 1488 3684 8006 8510 32539 57240 103138 238565 444456 820134 1185666 3960407
                 4429677 13719505 29191963 57736144 134185856 262306569 324235872 563441153 1686170713 2489885844
                 23 47 113 163 378 942 1669 2808 8029 9819 23863 39712 87352 210421 363671 562894 1839723 3504755
                 7456642 14807115 22469612 49080461 132842464 146060791 279256445 802149183 1243577750 3639860654
                 1260238066729040 10095256575169232896 1807253903626380 12063177829788352512/);
  for my $n (@liou) { emit($fh, 'liouville', [$n], sub { liouville("$n") }); }
  for my $n (0..40, 100,1000,10000,100000,1000000,10000000,100000000,1000000000,10000000000,100000000000,
             293,468,684,96862,76015169,10097286319,48512,444444,906150257,906180359,906316571) {
    emit($fh, 'sumliouville', [$n], sub { sumliouville($n) });
  }

  # --- exp_mangoldt t cases
  emit($fh, 'exp_mangoldt', [$_], sub { exp_mangoldt($_) }) for (0..11, 25, 27, 399981, 399982, 399983, 823543, 83521, 130321);

  # --- jordan_totient (t/20-jordantotient.t): J_k(n), n = 1..60 for k = 1..7, plus bigint-result cases
  for my $k (1..7) { for my $n (1..60) { emit($fh, 'jordan_totient', [$k,$n], sub { jordan_totient($k,$n) }); } }
  emit($fh, 'jordan_totient', [70, 3], sub { $o->(jordan_totient(70, 3)) });
  emit($fh, 'jordan_totient', [4, 12345], sub { jordan_totient(4, 12345) });

  # --- inverse_totient, sumtotient
  for my $n (0..100, 1728, 362880, 3978374400, 2*12135413, 2*10754819, 10000008, 10000, 82260072, 1000000, 4294967296, 123456, 720720) {
    my $cnt = eval { scalar(my @x = inverse_totient($n)) };
    emit($fh, 'inverse_totient', [$n], sub { inverse_totient($n) }) if defined $cnt && $cnt <= 5000;
    emit($fh, 'inverse_totient_count', [$n], sub { scalar(my @x = inverse_totient($n)) }) if defined $cnt;
  }
  for my $n (0..100, 12345, 654321, 1000000, 1000000000, 7790208951, 10000000000, 100000000000, 123456789012) {
    emit($fh, 'sumtotient', [$n], sub { $o->(sumtotient($n)) });
  }

  # --- valuation (t/19-valuation.t), remove_factors
  my @val = ([-4,2],[0,2],[1,2],[96552,6],[1879048192,2],['4611686018427387904',4],[$max,$max],
             [72,2],[72,3],[72,6],[-72,2],[0,0],[1,0],[5,1],[8,2],[8,3],[1024,2],['9223372036854775808',2],['9223372036854775808',4],
             ['18446744073709551614',2],[1000,10],[1000,-10],[6,-2],[100,5],[100,25],[100,0],[12,1],[12,-1],[0,1],[0,-1]);
  push @val, map { [$_, 2] } (0, 1, 2, 3, 4, 6, 8, 12, 16, 96, 1<<20, 1<<40, 1<<62);
  push @val, map { [3**$_ * 5, 3] } 0..20;
  for my $r (@val) { my($n,$k)=@$r; emit($fh, 'valuation', [$n,$k], sub { valuation("$n","$k") }); }
  for my $r ([72,6],[72,12],[72,8],[5,6],[-72,6],[-12,3],[0,6],[10,1],[10,0],[96552,6],[1000000,10],['9223372036854775808',2],[360,6],[360,2],[1,2]) {
    my($n,$k)=@$r;
    emit($fh, 'remove_factors', [$n,$k], sub { remove_factors("$n","$k") });
    emit($fh, 'remove_factors_exp', [$n,$k], sub { remove_factors_exp("$n","$k") });
  }

  # --- kronecker
  my @kr = ([109981,737777],[737779,121080],[-737779,121080],[737779,-121080],[-737779,-121080],[12345,331],[1001,9907],[19,45],[8,21],[5,21],[5,1237],[10,49],[123,4567],[3,18],[3,-18],
            [-2,0],[-1,0],[0,0],[1,0],[2,0],[-2,1],[-1,1],[0,1],[1,1],[2,1],[-2,-1],[-1,-1],[0,-1],[1,-1],[2,-1],
            [3686556869,428192857],[-1453096827,364435739],[3527710253,-306243569],[-1843526669,-332265377],[321781679,4095783323],[454249403,-79475159],
            ['17483840153492293897',455592493],['-1402663995299718225',391125073],['16715440823750591903',-534621209],['13106964391619451641','16744199040925208803'],
            ['11172354269896048081','10442187294190042188'],['-5694706465843977004','9365273357682496999'],
            ['-9223372036854775808',3],['9223372036854775807','-9223372036854775808'],['-9223372036854775808','-9223372036854775808'],['18446744073709551615','18446744073709551614'],
            ['18446744073709551615','18446744073709551615'],[2,'18446744073709551557'],[-1,'18446744073709551557']);
  for my $a (-12..12) { for my $n (-9..15) { push @kr, [$a,$n]; } }
  for my $r (@kr) { my($a,$n)=@$r; emit($fh, 'kronecker', [$a,$n], sub { kronecker("$a","$n") }); }

  # --- chinese / chinese2: pairs flattened "a1 n1 a2 n2 ..."
  my @crts = ([[]],[[4,5]],[[77,11]],[[0,5],[0,6]],[[14,5],[0,6]],[[10,11],[4,22],[9,19]],[[77,13],[79,17]],[[2,3],[3,5],[2,7]],
              [[10,11],[4,12],[12,13]],[[42,127],[24,128]],[[32,126],[23,129]],[[2328,16256],[410,5418]],[[1,10],[11,100]],[[11,100],[22,100]],
              [[3,5],[2,0]],[[3,0],[2,3]],[[3,5],[3,0],[2,3]],[[5,0],[15,1]],[[15,1],[5,0]],[[2,181],[3,193],[5,227],[30,383]],
              [[14,-5],[17,-6]],[[14,5],[17,-6]],[[-4,17],[-17,19]],[[-4,17],[17,-19]],
              [[14,44381],[87,48473],[19,59467],[74,118751]],[[1753051086,3243410059],[2609156951,2439462460]],
              [[-3105579549,3743000622],[-1097075646,1219365911]],
              [[0,1]],[[0,1],[0,1]],[[1,2],[2,3],[3,5],[4,7]],[[1,2],[0,2]],[[1,4],[3,6]],[[1,4],[2,6]],[[5,12],[17,18]],
              [['18446744073709551614','18446744073709551615'],[1,2]],[[1,'4294967296'],[2,'4294967295']],[[100,'18446744073709551615']],
              [[7,'9223372036854775808'],[3,5]]);
  for my $c (@crts) {
    my @pairs = (@$c == 1 && !@{$c->[0]}) ? () : map { [map { "$_" } @$_] } @$c;   # [[]] means chinese() with no pairs
    my @args = map { @$_ } @pairs;
    my @chk = eval { chinese2(@pairs) };
    next if @chk == 2 && defined $chk[1] && Math::BigInt->new("$chk[1]") > $m;   # modulus beyond u64: left out
    emit($fh, 'chinese', [@args], sub { chinese(@pairs) });
    emit($fh, 'chinese2', [@args], sub { chinese2(@pairs) });
  }

  # --- znorder, is_primitive_root, is_qr
  my @zo = ([1,35],[2,35],[4,35],[6,35],[7,35],[1,1],[0,0],[1,0],[25,0],[19,1],[1,19],[2,19],[3,20],[57,1000000003],[67,999999749],
            [22,999991815],[10,2147475467],[141,2147475467],[7410,2147475467],[31407,2147475467],[2,2405286912458753],[2,1000000000000031],
            [2,'18446744073709551557'],[3,'18446744073709551557'],[2,'9223372036854775837'],[0,1],[0,5],[5,10],[10,10],[2,'18446744073709551615'],
            ['18446744073709551614','18446744073709551615'],[1,'18446744073709551615'],[3,'4294967295'],[7,'4294967296']);
  push @zo, map { [$_, 1000003] } (2,3,5,7,10);
  push @zo, map { [$_, 91] } (2..20);
  for my $r (@zo) { my($a,$n)=@$r; emit($fh, 'znorder', [$a,$n], sub { znorder("$a","$n") }); }
  my @primroots = (1,2,3,4,5,6,7,8,9,10,1729,5109721,17551561,90441961,1407827621,1520874431,1685283601,100000001,2232881419280027,
                   '14123555781055773271',89637484042681,'9223372036854775837',36002292036481,72004584072962,2067900233973681742,
                   '8000468009126059319','-100000898',11,0,12..60,'18446744073709551557','18446744073709551615','18446744073709551614');
  for my $n (@primroots) {
    emit($fh, 'znprimroot', [$n], sub { znprimroot("$n") });
    emit($fh, 'qnr', [$n], sub { qnr("$n") }) if $n !~ /^-/;
  }
  for my $r ([2,0],[19,191],[13,191],[35,982],[74513,2],[74513,3],[3,7],[5,7],[2,7],[94,5109721],[97,17551561],[113,90441961],[2,1729],
             [6,'14123555781055773271'],[5,'9223372036854775837'],[6,2232881419280027],[335,89637484042681],[0,1],[1,1],[1,2],[0,2],[3,10],[7,10],[2,'18446744073709551557']) {
    my($a,$n)=@$r; emit($fh, 'is_primitive_root', [$a,$n], sub { is_primitive_root("$a","$n") });
  }
  for my $a (-5..5) { for my $n (1..20) { emit($fh, 'is_primitive_root', [$a,$n], sub { is_primitive_root($a,$n) }); } }
  for my $a (0..32) { for my $n (0..15, 9, 15) { emit($fh, 'is_qr', [$a,$n], sub { is_qr($a,$n) ? 1 : 0 }); } }
  emit($fh, 'is_qr', ['2636542937688','3409243234243'], sub { is_qr('2636542937688','3409243234243') ? 1 : 0 });
  for my $r ([2,'18446744073709551557'],[3,'18446744073709551557'],['18446744073709551556','18446744073709551557'],[-1,13],[-1,7],[-4,13],['9223372036854775807','18446744073709551615'],[1000000,1000003],[5,'4294967291']) {
    my($a,$n)=@$r; emit($fh, 'is_qr', [$a,$n], sub { is_qr("$a","$n") ? 1 : 0 });
  }
  emit($fh, 'qnr', [$_], sub { qnr($_) }) for (0..60, 5711, 366791, 2737, 9257330, 2**16, 2**32, 2**40);

  # --- is_totient t cases
  for my $n (0..40, (map { 2**29+$_ } 1..80), '9223372036854775836', '9223372036854775828', '9223372036854775842', '281474976710656', '18446744073709551615') {
    emit($fh, 'is_totient', [$n], sub { is_totient("$n") ? 1 : 0 });
  }

  # --- ramanujan_sum, ramanujan_tau, hclassno
  for my $k (0..34) { for my $n (0..34) { emit($fh, 'ramanujan_sum', [$k,$n], sub { ramanujan_sum($k,$n) }); } }
  for my $r ([34,0],[0,34],[1000000007,'18446744073709551615'],[360,720720],[12345,6789],['4294967296','4294967296'],['18446744073709551615','18446744073709551615'],[1000003,1000003*3]) {
    my($k,$n)=@$r; emit($fh, 'ramanujan_sum', [$k,$n], sub { ramanujan_sum("$k","$n") });
  }
  for my $n (-5, 0..60, 100, 106, 243, 1000, 16089, 65536, 100000, 1000000) {
    emit($fh, 'ramanujan_tau', [$n], sub { my $v = ramanujan_tau($n); $v < -$m || $v > $m ? 'undef' : "$v" });
    emit($fh, 'ramanujan_tau_bigint', [$n], sub { ramanujan_tau($n) });
  }
  emit($fh, 'hclassno', [$_], sub { hclassno($_) }) for (-3, 0..60, 163, 427, 907, 1555, 6307, 20563, 30067, 31243, 34483, 4031, 100000, 1000003);

  # --- divisorsum (t/19-divisorsum.t)
  for my $n (1..120, 220, 284, 945, 999999937, 4611686018427387904, '9600000000000000688', 1<<27, 262144, 16384, 2048, 5003, 12345) {
    emit($fh, 'aliquot_sum', [$n], sub { $o->(aliquot_sum("$n")) });
    emit($fh, 'abundance', [$n], sub { my $v = abundance("$n"); $v < -$m || $v > $m ? 'undef' : "$v" });
  }
  for my $r ([1<<27,2],[262144,3],[16384,4],[2048,5],[5003,5],[12345,4],[12345,0],[12345,1],[0,0],[0,1],[0,5]) {
    my($n,$k)=@$r; emit($fh, 'divisor_sum', [$n,$k], sub { $o->(divisor_sum($n,$k)) });
  }
  for my $n (1..200) { for my $k (0..3) { emit($fh, 'divisor_sum', [$n,$k], sub { divisor_sum($n,$k) }); } }
  for my $r ([1,0,10],[2,10],[4,10],[0,1,10],[48,1,10000],[48,10000],[6,100],[8,1000],[12,5000]) {
    emit($fh, 'inverse_sigma0', [@$r], sub { my $l = &inverse_sigma0(@$r); @$l });
    emit($fh, 'inverse_sigma0_count', [@$r], sub { &inverse_sigma0_count(@$r) });
  }
}
1;
