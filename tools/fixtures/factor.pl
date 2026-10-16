# Fixture family `factor`: factoring and the integer-structure predicates.
# Covers MPU t/50-factoring.t and t/26-issquarefree.t, 26-issemiprime.t, 26-isalmostprime.t,
# 26-isomegaprime.t, 26-iscarmichael.t, 26-iscyclic.t, 26-isperfectnumber.t, 26-practical.t, 26-smooth.t.
# divisor_sum results above u64 print as `undef` (u64_or_undef); an empty list (factor(1), no solution) also prints `undef`.
# List results are space-joined; factor_exp prints each pair as p^e. Signed inputs are included where the
# t files use them (is_square_free, is_smooth, is_rough, prime_omega, prime_bigomega, is_cyclic,
# is_perfect_number). Cases whose arguments or results exceed u64 in the t files are left out.
use strict; use warnings;

sub gen_factor {
  my $fh = shift;
  my $max = '18446744073709551615';
  my @e = edges();

  # --- inputs: 50-factoring.t @testn, @testn64, @testdivisors n and the extra semiprimes
  my @testn = (qw/0 1 2 3 4 5 6 7 8 16 57 64 377 9592 78498 664579 5761455
               114256942 2214143 999999929 50847534 455052511 2147483647 4118054813
               30 210 2310 30030 510510 9699690 223092870 1363 989 779 629 403 547308031
               808 2727 12625 34643 134431 221897 496213 692759 1228867
               2231139 2463289 3008891 5115953 6961021 8030207 10486123
               10893343 12327779 701737021 549900 10000142 392498
               30107 115553 123456 456789 174636000 4567890 1032924637 1234567890 5514109 1819015037140
               37607912018 346065536839 600851475143 3204941750802 29844570422669
               279238341033925 2623557157654233 24739954287740860
               3369738766071892021 10023859281455311421
               9007199254740991 9007199254740992 9007199254740993
               6469693230 200560490130 7420738134810 304250263527210
               13082761331670030 614889782588491410 440091295252541 5333042142001571
               79127989298 2339796554687 124838608575421729 1434569741817480287
               1256490565186616147 13356777177440210791 9999986200004761 99999989237606677
               999999866000004473 3747785838079 18446743979220271189 166213 18450931744652885087
               899 667 563777293 11009 2466600463243213733 221 1111 2949544710425713 926269344000
               13581893559735945553 36803912137803496523 36832540935240707603
               752159046363135949 9881022443630858407 429606 999999937 8128 33550336 8589869056
               137438691328 2305843008139952128 4611686018427387904 18446744073709551557/);
  # extra semiprimes: p*q with p,q near 2^16, 2^24 and 2^31.5 (about 2^32, 2^48, 2^63), close and far apart
  my @semis;
  for my $h (16, 24, 31) {
    my $p = next_prime(2**$h + 12345);
    for my $gap (0, 10, 1000, 100000) {
      my $q = next_prime($p + $gap + 1);
      my $n = Math::BigInt->new($p) * $q;
      push @semis, "$n" if $n <= Math::BigInt->new($max);
    }
  }
  { my $p = next_prime(3037000499); my $q = next_prime($p + 200); push @semis, Math::BigInt->new($p)->bmul($q)->bstr if Math::BigInt->new($p)->bmul($q) <= Math::BigInt->new($max); }
  push @semis, map { Math::BigInt->new(next_prime(2**$_ + 7))->bmul(next_prime(2**$_ + 4001))->bstr } (16, 24);
  push @semis, map { Math::BigInt->new(next_prime(2**$_))->bmul(next_prime(2**($_+2)))->bstr } (10, 16, 20, 28);
  push @semis, "9223372036854775783" ;   # a prime near 2^63
  push @semis, Math::BigInt->new('3037000493')->bmul('3037000453')->bstr;
  my @all = (@testn, @semis, @e, 0..300);
  my %seen; @all = grep { !$seen{"$_"}++ } @all;

  for my $n (@all) {
    emit($fh, 'factor', [$n], sub { factor("$n") });
    emit($fh, 'factor_exp', [$n], sub { map { "$_->[0]^$_->[1]" } factor_exp("$n") });
    emit($fh, 'prime_omega', [$n], sub { prime_omega("$n") });
    emit($fh, 'prime_bigomega', [$n], sub { prime_bigomega("$n") });
    emit($fh, 'prime_signature', [$n], sub { prime_signature("$n") });
    emit($fh, 'prime_signature_scalar', [$n], sub { scalar prime_signature("$n") });
    emit($fh, 'is_square_free', [$n], sub { is_square_free("$n") ? 1 : 0 });
    emit($fh, 'is_semiprime', [$n], sub { is_semiprime("$n") ? 1 : 0 });
    emit($fh, 'is_carmichael', [$n], sub { is_carmichael("$n") ? 1 : 0 });
    emit($fh, 'is_quasi_carmichael', [$n], sub { is_quasi_carmichael("$n") });
    emit($fh, 'is_cyclic', [$n], sub { is_cyclic("$n") ? 1 : 0 });
    emit($fh, 'is_perfect_number', [$n], sub { is_perfect_number("$n") ? 1 : 0 });
    emit($fh, 'is_practical', [$n], sub { is_practical("$n") ? 1 : 0 });
    emit($fh, 'squarefree_kernel', [$n], sub { squarefree_kernel("$n") });
    emit($fh, 'powerfree_part', [$n], sub { powerfree_part("$n") });
    emit($fh, 'powerfree_part', [$n, 3], sub { powerfree_part("$n", 3) });
  }
  # signed inputs (documented for these functions; t files use -n)
  for my $n (0..16, 35, 143, 187, 561, 758096738, 434420340, 870589313, 752518565, 7, 100, 1000) {
    for my $f (qw/is_square_free is_cyclic is_perfect_number prime_omega prime_bigomega/) {
      no strict 'refs';
      emit($fh, $f, ["-$n"], sub { my $r = &{"Math::Prime::Util::$f"}("-$n"); $r ? "$r" : 0 });
    }
  }
  for my $n (-20..20) { emit($fh, 'is_cyclic', [$n], sub { is_cyclic($n) ? 1 : 0 }); }
  for my $n (-10..500) { emit($fh, 'is_perfect_number', [$n], sub { is_perfect_number($n) ? 1 : 0 }); }
  emit($fh, 'prime_omega', ['-'.$_], sub { prime_omega('-'.$_) }) for (0,1,2,36,102,392,8593952);
  emit($fh, 'prime_bigomega', ['-'.$_], sub { prime_bigomega('-'.$_) }) for (0,1,2,36,102,392,8593952);
  # --- 26-issquarefree.t
  for my $n (0..16, 758096738,434420340,870589313,695486396,602721315,418431087,
             752518565,723570005,506916483,617459403) {
    emit($fh, 'is_square_free', [$n], sub { is_square_free($n) ? 1 : 0 });
    emit($fh, 'is_square_free', ["-$n"], sub { is_square_free("-$n") ? 1 : 0 });
  }
  # --- 26-issemiprime.t
  for my $n (121,341,2047,7009,28153,10000..10100,752159046363135949,'9881022443630858407') {
    emit($fh, 'is_semiprime', [$n], sub { is_semiprime("$n") ? 1 : 0 });
  }
  # --- 50-factoring.t: divisors, divisor_sum
  my @dn = (0,1,2,3,4,5,6,7,8,9,10,12,16,42,30107,115553,123456,456789,4567890,1032924637,1234567890,5040,
            174636000, 720720, 8648640, 73513440, 1000000007, 4294967296, 9223372036854775807, '9223372036854775808',
            $max, '18446744073709551614', 549900, 1099511627776);
  for my $n (@dn, 0..120) {
    my $nd = eval { divisor_sum("$n", 0) };
    if (defined $nd && $nd <= 5000) {
      emit($fh, 'divisors', [$n], sub { divisors("$n") });
    }
    emit($fh, 'divisors_count', [$n], sub { scalar divisors("$n") }) if defined $nd && $nd <= 5000;
    emit($fh, 'divisor_sum', [$n], sub { u64_or_undef(divisor_sum("$n")) });
    for my $k (0..3) { emit($fh, 'divisor_sum', [$n, $k], sub { u64_or_undef(divisor_sum("$n", $k)) }); }
  }
  for my $nl ([5040,120],[0,0],[0,1],[1,0],[1,1],[1,2],[12,0],[12,1],[12,4],[12,12],[360,36],[360,0],[$max,1000],['18446744073709551615',100000]) {
    my($n,$l)=@$nl; emit($fh, 'divisors', [$n,$l], sub { divisors("$n",$l) });
  }

  # --- specific algorithms, exact inputs of extra_factor_test plus the coverage cases
  my @alg_in = (1,4,9,11,25,30,210,175,403,549900, 0, 2, 3, 7, 15, 91, 143, 8051, 10403, 1000036000099, 5514109, 1819015037140,
                3747785838079, 166213, 563777293, 899, 667, 11009, 1111, 2949544710425713);
  my @alg_semi = (@semis);
  no strict 'refs';
  for my $fn (qw/trial_factor fermat_factor holf_factor lehman_factor squfof_factor prho_factor pbrent_factor
                 pminus1_factor pplus1_factor cheb_factor ecm_factor/) {
    my $code = \&{"Math::Prime::Util::$fn"};
    for my $n (@alg_in) { emit($fh, $fn, [$n], sub { $code->("$n") }); }
    next if $ENV{FIXTURE_FAST};
    for my $n (@alg_semi) { emit($fh, $fn, [$n], sub { $code->("$n") }); }
  }
  # arity variants (t/50-factoring.t "code coverage" subtest)
  my @var = (
    ['trial_factor', 5514109], ['trial_factor', 1819015037140], ['trial_factor', '553402322211286548450', 1],
    ['trial_factor', 1819015037140, 10], ['trial_factor', 1819015037140, 1000], ['trial_factor', 1000003*1000033, 1000],
    ['trial_factor', 1000003*1000033, 1000004], ['trial_factor', $max, 100],
    ['holf_factor', 3747785838079, 80000], ['holf_factor', 91, 1], ['holf_factor', 91, 100],
    ['fermat_factor', '18446743979220271189', 10], ['fermat_factor', 91, 1], ['fermat_factor', 91, 10],
    ['pminus1_factor', 166213], ['pminus1_factor', '18450931744652885087', 100, 1000],
    ['pminus1_factor', 899, 20], ['pminus1_factor', 667, 1000], ['pminus1_factor', 563777293, 1000, 20000],
    ['pminus1_factor', 563777293, 1000], ['pminus1_factor', 563777293, 10],
    ['pplus1_factor', 11009, 5], ['pplus1_factor', 11009], ['pplus1_factor', 11009, 200],
    ['cheb_factor', '2466600463243213733', 1000], ['cheb_factor', '2466600463243213733'],
    ['cheb_factor', 221, 100, $max], ['cheb_factor', 221, 100], ['cheb_factor', 221, 100, 7],
    ['ecm_factor', '13356777177440210791', 2000, 0, 3],
    ['ecm_factor', '13356777177440210791', 2000, 40000, 3], ['ecm_factor', '2949544710425713', 500, 600, 30],
    ['ecm_factor', 1111], ['ecm_factor', 1111, 100], ['ecm_factor', 1111, 100, 1000],
    ['pbrent_factor', 91, 4], ['pbrent_factor', 91, 4, 3], ['pbrent_factor', 8051, 1000], ['pbrent_factor', 8051, 1000, 2],
    ['prho_factor', 8051, 1000], ['prho_factor', 8051, 1000, 3], ['squfof_factor', 8051, 1000], ['lehman_factor', 8051, 1000],
  );
  for my $v (@var) {
    my($fn, @a) = @$v;
    my $code = \&{"Math::Prime::Util::$fn"};
    emit($fh, $fn, [@a], sub { $code->(map { "$_" } @a) });
  }
  # invalid arguments die in MPU (ERROR)
  emit($fh, 'trial_factor', ['18446744073709551616', '10foo'], sub { Math::Prime::Util::trial_factor("18446744073709551616","10foo") });
  emit($fh, 'pbrent_factor', [91, '10foo'], sub { Math::Prime::Util::pbrent_factor(91,"10foo") });
  use strict 'refs';

  # --- 26-isalmostprime.t: is_almost_prime(k,n)
  my %kap = (
    0 => [1], 1 => [2,3,5,7,11,13,17,19,23,29], 2 => [4,6,9,10,14,15,21,22,25,26],
    3 => [8,12,18,20,27,28,30,42,44,45], 4 => [16,24,36,40,54,56,60,81,84,88],
    5 => [32,48,72,80,108,112,120,162,168,176], 6 => [64,96,144,160,216,224,240,324,336,352],
    10 => [1024,1536,2304,2560,3456,3584,3840,5184,5376,5632], 20 => [1048576,1572864,2359296,2621440,3538944,3670016,3932160,5308416,5505024,5767168],
    63 => ['9223372036854775808'], 62 => ['9223372036854775808'], 64 => ['9223372036854775808', '18446744073709551615'],
  );
  for my $k (0..10) { for my $n (0..40) { emit($fh, 'is_almost_prime', [$k, $n], sub { is_almost_prime($k, $n) ? 1 : 0 }); } }
  for my $k (sort { $a <=> $b } keys %kap) { for my $n (@{$kap{$k}}) { emit($fh, 'is_almost_prime', [$k, $n], sub { is_almost_prime($k, "$n") ? 1 : 0 }); } }
  emit($fh, 'is_almost_prime', ['1000000000000000000000000000000', 16], sub { is_almost_prime("1000000000000000000000000000000", 16) ? 1 : 0 });
  emit($fh, 'is_almost_prime', [64, 16], sub { is_almost_prime(64, 16) ? 1 : 0 });
  for my $n (@semis, @e) { for my $k (1..4) { emit($fh, 'is_almost_prime', [$k, $n], sub { is_almost_prime($k, "$n") ? 1 : 0 }); } }

  # --- 26-isomegaprime.t: is_omega_prime(k,n)
  my @fn7 = (10000019, 10000000, 10000005, 10000002, 10000012, 10000080, 10002930, 11741730, 223092870);
  for my $i (0..$#fn7) { my($k,$n)=($i+1,$fn7[$i]); emit($fh, 'is_omega_prime', [$k,$n], sub { is_omega_prime($k, $n) ? 1 : 0 }); }
  for my $k (0..7) { for my $n (10000..10100, 0, 1, 2, 30, 210, 2310, 30030, 510510, 9699690, 223092870, 6469693230) {
    emit($fh, 'is_omega_prime', [$k,$n], sub { is_omega_prime($k, $n) ? 1 : 0 }); } }
  emit($fh, 'is_omega_prime', [1, 0], sub { is_omega_prime(1, 0) ? 1 : 0 });
  for my $n (@e) { for my $k (0..4) { emit($fh, 'is_omega_prime', [$k,$n], sub { is_omega_prime($k, "$n") ? 1 : 0 }); } }

  # --- 26-iscarmichael.t, 26-iscyclic.t, 26-isperfectnumber.t, 26-practical.t
  for my $n (1..20000) { emit($fh, 'is_carmichael', [$n], sub { is_carmichael($n) ? 1 : 0 }) if is_carmichael($n) || $n % 7 == 0 || $n < 2000; }
  for my $n (5000209,5000145,5000069,5000483,5000169,5000001, 41041, 825265, 321197185, 5394826801, 232250619601, 9746347772161, '3825123056546413051', '318665857834031151167461') {
    emit($fh, 'is_carmichael', [$n], sub { is_carmichael("$n") ? 1 : 0 }) if Math::BigInt->new("$n") <= Math::BigInt->new($max); }
  for my $n (1..5000) { emit($fh, 'is_quasi_carmichael', [$n], sub { is_quasi_carmichael($n) }); }
  for my $n (5092583, 777923, '17293822569102705135') { emit($fh, 'is_quasi_carmichael', [$n], sub { is_quasi_carmichael("$n") }); }
  for my $n (qw/9 15 21 25 27 33 35 39 45 49 51 55 57 63 65 69 75 77 81 85 87 91 93 95 99 105 111 115 117 119 121 123 125 129 133 135 141 143 145 147 153 155 159 161 165 169 171 175 177 183 185 187 189 195 32753/, 1..200) {
    emit($fh, 'is_cyclic', [$n], sub { is_cyclic($n) ? 1 : 0 }); }
  for my $n (qw/8128 33550336 8589869056 137438691328 2305843008139952128 8505 12285 19845 28665 31185 198585576189 8 32 2096128 35184367894528 144115187807420416/) {
    emit($fh, 'is_perfect_number', [$n], sub { is_perfect_number("$n") ? 1 : 0 }); }
  for my $n (0..252, 429606, 70, 350, 490, 770, 910, 945, 1190, 1330, 1575, 1610, 1750, 2030, 2170, 2205, 2450, 2584, 2590, 2835, 2870, 3010, 3128, 3290, 3430, 3465, 3710, 3850, 3944, 4095, 4130, 4216, 4270, 4550, 4690, 4725, 5355, 5390, 5775, 5950, 5985, 6370, 6615, 6650, 6825) {
    emit($fh, 'is_practical', [$n], sub { is_practical($n) ? 1 : 0 }); }

  # --- 26-smooth.t: is_smooth, is_rough, smooth_count, rough_count
  for my $k (0..9, 11, 13, 17, 29) {
    for my $n (0..12, 143, 187, 253, 319, 341, 851, 1073, 1147) {
      emit($fh, 'is_smooth', [$n, $k], sub { is_smooth($n, $k) ? 1 : 0 });
      emit($fh, 'is_rough', [$n, $k], sub { is_rough($n, $k) ? 1 : 0 });
      emit($fh, 'is_smooth', ["-$n", $k], sub { is_smooth("-$n", $k) ? 1 : 0 });
      emit($fh, 'is_rough', ["-$n", $k], sub { is_rough("-$n", $k) ? 1 : 0 });
    }
  }
  for my $c ([1000000,10000],[1000127,10000],[1000127,3000],[1000157,3000],['137438953481',3000],['137438953493',3000],['137438953529',3000]) {
    my($n,$k)=@$c;
    emit($fh, 'is_smooth', [$n,$k], sub { is_smooth("$n",$k) ? 1 : 0 });
    emit($fh, 'is_rough', [$n,$k], sub { is_rough("$n",$k) ? 1 : 0 });
  }
  for my $n (@semis, @e, 2*3*5*7*11*13*17*19*23*29*31*37*41*43*47, '614889782588491410') {
    for my $k (0, 1, 2, 3, 10, 100, 1000, 65536, 4294967296, '18446744073709551615') {
      emit($fh, 'is_smooth', [$n,$k], sub { is_smooth("$n","$k") ? 1 : 0 });
      emit($fh, 'is_rough', [$n,$k], sub { is_rough("$n","$k") ? 1 : 0 });
    }
  }
  for my $n (0..5) { for my $k (0..5) {
    emit($fh, 'smooth_count', [$n,$k], sub { smooth_count($n,$k) });
    emit($fh, 'rough_count', [$n,$k], sub { rough_count($n,$k) }); } }
  for my $c ([100,17],[1980627498,9],[10000000,400],[1000,10],[1000000,100],[1000000,1000],[4294967296,50],['100000000000',30],['18446744073709551615',5],[1000000000,2]) {
    my($n,$k)=@$c; emit($fh, 'smooth_count', [$n,$k], sub { smooth_count("$n",$k) }); }
  for my $c ([3700621409,15],[100,5],[1000000,7],[1000000,100],[4294967296,13],['100000000000',19],['18446744073709551615',3],[1000000000,2],[1000000000,0],[1000000000,1],['1000000000000',50]) {
    my($n,$k)=@$c; emit($fh, 'rough_count', [$n,$k], sub { rough_count("$n",$k) }); }
}
1;
