# Fixture family `families`: t/11-twinprimes.t, t/11-semiprimes.t, t/11-almostprimes.t,
# t/11-omegaprimes.t, t/11-ramanujanprimes.t, t/11-primepowers.t, t/11-safeprimes.t, t/26-chenprimes.t,
# t/26-lucky.t, t/26-perfectpowers.t, t/26-powerful.t, t/26-powerfree.t, t/26-goldbach.t,
# t/26-isgaussianprime.t (MPU 0.75 ships no t/26-higgsprimes.t and has no higgs function: omitted).
# Array-ref results are flattened and space-joined; an empty list is written as "undef".
# is_prime_power_root writes "k root" from the two-arg form is_prime_power(n, \$root).
use strict; use warnings;

sub gen_families {
  my $fh = shift;
  # fx(fn, args...): call the MPU function by name in list context, flattening one array-ref result.
  my $fx = sub {
    my ($fn, @a) = @_;
    my $code = Math::Prime::Util->can($fn) or die "no $fn";
    emit($fh, $fn, [@a], sub { my @r = $code->(@a); (@r == 1 && ref($r[0]) eq 'ARRAY') ? @{$r[0]} : @r });
  };
  my $each = sub { my ($fn, $list) = @_; $fx->($fn, ref $_ ? @$_ : $_) for @$list; };
  my $U = '18446744073709551615';

  # ---- twin primes (t/11-twinprimes.t)
  $fx->('twin_primes', 1607);
  $fx->('twin_primes', @$_) for ([6,10],[5,10],[5,11],[5,12],[5,13],[5,16],[4,11],[3,11],[2,11],[1,11],[0,11],
    [29,31],[213897,213997],[4294957296,4294957796],[134217228,134217728],[0,1000],[10000,12000]);
  $fx->('twin_primes', $_) for 0, 1, 2, 3, 4, 5, 100, 1000;
  $each->('nth_twin_prime', [0..60, 100, 1000, 10000, 100000]);
  $each->('twin_prime_count', [0..20, 100, 1000, 12345, 100000, 1000000, 100000000, '10000000000']);
  $fx->('twin_prime_count', @$_) for ([0,100],[3,5],[5,11],[100,1000],[1000,10000],[213897,213997]);
  $each->('twin_prime_count_approx', [0, 1, 2, 3, 5, 10, 100, 1000, 12345, 100000, 10**7, 10**9, '1000000000000', '100000000000000', $U]);
  $each->('nth_twin_prime_approx', [0, 1, 2, 3, 10, 100, 1000, 12345, 100000, 10**6, 10**8, '1000000000']);

  # ---- semiprimes (t/11-semiprimes.t)
  $fx->('semi_primes', 95);
  $fx->('semi_primes', @$_) for ([11,13],[10,10],[10,11],[10,12],[10,13],[10,14],[5,16],[4,11],[3,11],[2,11],
    [1,11],[0,11],[26,33],[25,34],[184279943,184280038],[184279944,184280037],[8589990147,8589990167],
    [0,1000],[10000,12000],["18446744073709551601","18446744073709551615"],["18446744073709551602",$U],
    ["20019396133340429","20019396133340431"]);
  $fx->('semi_primes', $_) for 0, 1, 3, 4, 5, 100, 1000;
  $each->('nth_semiprime', [0, 1..60, 100, 1234, 12345, 123456, 1000000]);
  $each->('semiprime_count', [0..20, 1234, 12345, 123456, 1000000, 100000000]);
  $fx->('semiprime_count', @$_) for ([1000000,1000100],[1000000000,1000000100],[1000000000,1000010000],[10,100],[0,1000]);
  $each->('semiprime_count_approx', [0, 1, 3, 4, 10, 100, 1234, 12345, 123456, 100000000, '100000000000', '100000000000000', '10000000000000000000', $U]);
  $each->('nth_semiprime_approx', [0, 1, 2, 10, 100, 1234, 12345, 123456, 2147483648, '4398046511104', '100000000000000000', '288230376151711744', '1701748900850019776']);

  # ---- almost primes (t/11-almostprimes.t)
  $fx->('almost_primes', 0, $_) for 0, 1, 100000;
  for my $k (0..5) { $fx->('almost_primes', $k, $_) for 1, 30, 100, 232, 500; }
  for my $r ([8,3],[8,4],[8,5],[13,3],[13,4],[13,5]) {
    my ($E, $k) = @$r; my $p = '1' . '0' x ($E-2);
    $fx->('almost_primes', $k, $p.'00', $p.'50');
  }
  $fx->('almost_primes', 2, 1000, 1100); $fx->('almost_primes', 3, 1000, 1100); $fx->('almost_primes', 20, 1, 100000000);
  $fx->('almost_primes', 63, "9223372036854775808", $U);
  for my $k (0..20) { $fx->('almost_prime_count', $k, 1000000); }
  for my $k (0..8) { $fx->('almost_prime_count', $k, $_) for 0, 1, 2, 4, 8, 206, 1024; }
  $fx->('almost_prime_count', 17, 1000000000);
  $fx->('almost_prime_count', 3, 8); $fx->('almost_prime_count', 2, 4); $fx->('almost_prime_count', 1, 2);
  $fx->('almost_prime_count', -2, 100);
  for my $k (1..10) { $fx->('almost_prime_count', $k, 206); }
  for my $f (qw/almost_prime_count_approx almost_prime_count_lower almost_prime_count_upper/) {
    for my $k (0..6, 10, 20, 32, 63) { $fx->($f, $k, $_) for 0, 1, 100, 206, 1000000, 234618, '26843545600', $U; }
    $fx->($f, -2, 100);
  }
  my @nk = ([1,2],[2,34],[3,456],[2,4],[3,4],[4,4],[4,5678],[5,67890],[24,5555],[100,3],[3,59643],
            [32,12],[63,2011],[150,4557],[1,46],[2,67],[3,48],[4,26]);
  for my $f (qw/nth_almost_prime nth_almost_prime_approx nth_almost_prime_lower nth_almost_prime_upper/) {
    $fx->($f, @$_) for @nk;
    $fx->($f, $_, 1) for 0..8, 20, 63;
    $fx->($f, $_, 0) for 0, 1, 2;
    $fx->($f, -2, 5);
  }
  $fx->('nth_almost_prime', 2, $_) for 1..40;
  $fx->('nth_almost_prime', 3, $_) for 1..40;
  # Exercise the generic k >= 5 search, including reduction boundaries.
  for my $k (5..9) {
    $fx->('nth_almost_prime', $k, $_) for 8, 40, 150, 1000, 10000;
  }

  # ---- omega primes (t/11-omegaprimes.t)
  my @omax = (0, 113, 76, 240, 1326, 11130);
  for my $k (1..5) { $fx->('omega_primes', $k, $omax[$k]); }
  $fx->('omega_primes', 0, 1, 200000002);
  $fx->('omega_primes', 10, '6469693230', '9469693230');
  $fx->('omega_primes', 10, '1000000000000', '1000005000001');
  $fx->('omega_primes', $_, 1, 3000) for 0..6;
  $fx->('omega_primes', 2, 1000, 1100); $fx->('omega_primes', 3, '1000000000000', '1000000001000');
  $fx->('omega_primes', 11, '200000000000', '2000000000000');
  for my $k (1..10) { $fx->('omega_prime_count', $k, 206); }
  $fx->('omega_prime_count', $_, 10000) for 0..10;
  $fx->('omega_prime_count', $_, 1000000) for 0..20;
  $fx->('omega_prime_count', $_, 206111) for 1..10;
  $fx->('omega_prime_count', 6, 90000); $fx->('omega_prime_count', 8, 20000000); $fx->('omega_prime_count', -2, 100);
  $fx->('omega_prime_count', 2, 0); $fx->('omega_prime_count', 0, 0);
  $fx->('nth_omega_prime', 0, $_) for 0..4;
  $fx->('nth_omega_prime', $_, 0) for 1..10;
  for my $k (1..5) { $fx->('nth_omega_prime', $k, $_) for 1..40; }
  $fx->('nth_omega_prime', 8, 122); $fx->('nth_omega_prime', -2, 5);
  $fx->('nth_omega_prime', 1, 1000); $fx->('nth_omega_prime', 2, 1000); $fx->('nth_omega_prime', 3, 1000);

  # ---- ramanujan primes (t/11-ramanujanprimes.t)
  $fx->('ramanujan_primes', 983);
  $fx->('ramanujan_primes', @$_) for ([182,226],[11,16],[11,17],[11,18],[11,19],[11,20],[10,11],[11,29],[3,11],
    [2,11],[1,11],[0,11],[599,599],[10000,10100],[0,5000]);
  $fx->('ramanujan_primes', $_) for 0, 1, 2, 10, 11, 100, 1000;
  my @rn = (0..80, 997, 1234, 5000, 23744, 59643, 123456);
  for my $f (qw/nth_ramanujan_prime nth_ramanujan_prime_lower nth_ramanujan_prime_upper nth_ramanujan_prime_approx/) {
    $each->($f, [@rn, 1000000, 5964377]) unless $f eq 'nth_ramanujan_prime';
    $each->($f, \@rn) if $f eq 'nth_ramanujan_prime';
  }
  $each->('is_ramanujan_prime', [0..1000, 19379, 617759, 1673993, '225792607', 3657037, 3657039]);
  my @rc = (0, 1, 2, 3, 10, 11, 17, 29, 100, 1000, 10000, 100000, 1000000, 1673993, 225792607, '4294967296', $U);
  $each->("ramanujan_prime_count$_", \@rc) for qw/_lower _upper _approx/;
  $each->("ramanujan_prime_count", [grep { $_ < 300000000 } @rc]);
  $fx->('ramanujan_prime_count', @$_) for ([0,100],[11,29],[100,1000],[1000,100000]);

  # ---- prime powers (t/11-primepowers.t)
  $fx->('prime_powers', $_) for 0, 1, 2, 3, 4, 97, 300, 1000;
  $fx->('prime_powers', 1441897, 1442897);
  $fx->('prime_powers', "1000000014000000045", "1000000014000000053");
  $fx->('prime_powers', @$_) for ([0,100],[10,10],[16,16],[17,16],[2,2],[1,1]);
  my @pp = (0..130, 255, 256, 257, 511, 512, 513, 1000, 4294967295, 4294967296, 4294967297, '9223372036854775807', '18446744073709551557', '18446744073709551614', $U);
  $each->('is_prime_power', \@pp);
  $each->('next_prime_power', [@pp, map { 2**$_ - 1, 2**$_ } 0..9]);
  $each->('prev_prime_power', [@pp, map { 2**$_ + 1, 2**$_ } 0..9]);
  for my $n (@pp) {
    emit($fh, 'is_prime_power_root', [$n], sub { my $r; my $k = is_prime_power($n, \$r); ($k, $k ? $r : 0) });
  }
  my @pc = (0..80, 100, 300, 513, 1000, 12345, 5964377, 12345678, 100000000, '10000000000', '1000000000000', '4294967296', $U);
  $each->("prime_power_count$_", \@pc) for qw/_approx _lower _upper/;
  $each->("prime_power_count", [grep { $_ <= 1e12 } @pc]);
  $fx->('prime_power_count', 123456, 133332);
  $fx->('prime_power_count', 30*$_, 30*$_+30) for 0..19;
  for my $lo (0..8,15,16,17,31,32,33,40) { $fx->('prime_power_count', $lo, $lo + 4*$_) for 0..5, 8; }
  my @pn = (0..60, 86, 123456, 1<<12, 1000, 5286238, 203280221, 10957, 20942, 51526, 417867, 717421, 8337143);
  $each->("nth_prime_power$_", \@pn) for '', '_approx', '_lower', '_upper';

  # ---- safe primes (t/11-safeprimes.t)
  $each->('is_safe_prime', [0..2000, -59, 890471062883, 800579858627, 764953310311, '284651113549152086747',
                            '220961958687770434157', '270708094586485285875', '18446744073709551557', $U]);

  # ---- chen primes (t/26-chenprimes.t)
  $each->('is_chen_prime', [0..400, 10000000000687, '10000000000079', '18446744073709551557', 4294967291, $U]);
  $each->('next_chen_prime', [0..250, 1000, 10000, 4294967290, '10000000000000000']);

  # ---- lucky (t/26-lucky.t)
  $each->('lucky_numbers', [0, 1, 2, 3, 10, 50, 195, 200, 1000, 12345]);
  $fx->('lucky_numbers', $_, $_ + 3) for 0..45;
  $fx->('lucky_numbers', @$_) for ([0,40],[20,40],[40,0],[51221,51289],[100,1000],[1000,1000],[1,1]);
  $each->('is_lucky', [0..200, 42975, 513, 49023, 120001, 1000047, 1000071, 9999997, 10239, 34639, 53595, 308119, 971011]);
  my @lc = (0..100, 153, 195, 200, 513, 1000, 5964377, 100000, 1000000, 307703784778627, 10000000000);
  $each->("lucky_count$_", \@lc) for qw/_approx _lower _upper/;
  $each->("lucky_count", [grep { $_ <= 1e7 } @lc]);
  $fx->('lucky_count', $_, $_ + 40) for 0..30;
  $fx->('lucky_count', 100, 1000); $fx->('lucky_count', 40, 0);
  my @ln = (0..160, 1<<12, 86, 123456, 5286238, 1144, 3378, 4996, 24406, 69985, 46697909, 1<<31, '8796093022208');
  $each->("nth_lucky$_", \@ln) for qw/_approx _lower _upper/;
  $each->("nth_lucky", [grep { $_ <= 1e6 } @ln]);
  pop @ln for 1..0;

  # ---- perfect powers (t/26-perfectpowers.t)
  my @pp100 = (-100..100, '-18446745128696702936', '-18446724184312856125', '-4298942376', '-4291015625', -2147483648, -2146689000,
               2147395600, 2147483648, 4294836225, 4294967296, '9223372030926249001', '9223372036854775808',
               '18446744065119617025', '18446744073709551616', '12044602655621644287', '-8922003266371364726', $U);
  $each->('is_perfect_power', \@pp100);
  $each->('next_perfect_power', \@pp100);
  $each->('prev_perfect_power', \@pp100);
  my @pcn = (0..90, 100, 1000, 12345678, 123456, 133332, (map { 10**$_ } 0..15), 2048383, 3373286400, '252826822479841', '10000000000000000000', $U);
  $each->("perfect_power_count$_", \@pcn) for '', '_approx', '_lower', '_upper';
  $fx->('perfect_power_count', 123456, 133332); $fx->('perfect_power_count', $_, 16) for 8, 9, 10;
  $fx->('perfect_power_count', 1, 1000); $fx->('perfect_power_count', 100, 10);
  my @pnn = (0..60, 1571, 59643, 15964377, 67224..67229, 4297615579..4297615582, 1000000);
  $each->("nth_perfect_power$_", \@pnn) for '', '_approx', '_lower', '_upper';

  # ---- powerful (t/26-powerful.t)
  $fx->('is_powerful', $_) for 0..258;
  $fx->('is_powerful', $_, 2) for 0..258;
  my @nums = (227411960,105218838,79368063,58308379,210322300,44982156,67831696,165946352,243118692,128757041,150085583);
  for my $k (0, 1, 3 .. 12) { $fx->('is_powerful', $_, $k) for 0..64, @nums; }
  $fx->('is_powerful', -8, $_) for 0..10;
  $fx->('is_powerful', 0, $_) for 0..10;
  $fx->('is_powerful', $_, 0) for 1..32;
  $fx->('is_powerful', $_, 1) for 1..32;
  $fx->('is_powerful', 2, 64); $fx->('is_powerful', 4, 65); $fx->('is_powerful', "2199023255552", 41);
  $fx->('is_powerful', "2199023255552", 42);
  $fx->('is_powerful', 25*$_*$_, 2) for 1..20; $fx->('is_powerful', 149*$_*$_, 2) for 1..20;
  $fx->('is_powerful', 343*$_**3, 3) for 1..20; $fx->('is_powerful', 4489*$_**3, 3) for 1..20;
  $fx->('is_powerful', $_) for "1377276413364943226363244108454842276965894752197358387200000",
    "2346889178458529643625998598305409091755415961600000", '18446744073709551615', '18446744065119617025';
  $fx->('is_powerful', "56648008573112538662596929676588737208124071038924666321487873929306609840197", 30);

  $fx->('powerful_count', $_) for -16..20, -100, -10, -4, 100, 1000, 12345, 1000000;
  $fx->('powerful_count', $_, 0) for -100, -10, -4, -1, 0, 1, 4, 10, 100, 0..20;
  $fx->('powerful_count', $_, 1) for 0..20;
  $fx->('powerful_count', $_, 2) for -100, -10, -4, -1, 0, 1, 4, 10, 100;
  $fx->('powerful_count', $_, 3) for 0..20, 1000, 100000;
  for my $k (4..8) { $fx->('powerful_count', $_, $k) for 0, 1, 100, 10000, 1000000; }
  my @pow14 = (0,14,100,432,2048,6561,16384,59049,131072,262144,531441,1594323,4194304,8388608,16777216,43046721,129140163,268435456,536870912,1162261467,3486784401,8589934592,17179869184,34359738368,68719476736,137438953472,274877906944,549755813888,1099511627776,2199023255552,4398046511104);
  for my $k (1..30) { $fx->('powerful_count', $pow14[$k], $k); $fx->('powerful_count', $pow14[$k]-1, $k); }
  $fx->('powerful_count', 10**$_) for 1..14;
  $fx->('powerful_count', "1" . ("0" x $_), 7) for 1..22;
  $fx->('powerful_count', $U); $fx->('powerful_count', $U, 3);
  $fx->('powerful_count', 10, 100); # 2-arg lo,hi form is not documented: records behaviour

  $fx->('nth_powerful', $_) for 0..60, 100, 1000, 100000, 1000000;
  $fx->('nth_powerful', 100, 6); $fx->('nth_powerful', 12, 15); $fx->('nth_powerful', 10999963, 3);
  $fx->('nth_powerful', 11938035, 3); $fx->('nth_powerful', 11938036, 3);
  $fx->('nth_powerful', $_, 3) for 0..30; $fx->('nth_powerful', $_, 0) for 1..5; $fx->('nth_powerful', $_, 1) for 1..5;
  $fx->('nth_powerful', $_, 4) for 1..30;

  $fx->('sumpowerful', $_) for -16..48, 1000, 100000;
  $fx->('sumpowerful', $_, 3) for 0..48;
  $fx->('sumpowerful', 17411, $_) for 0..16;
  $fx->('sumpowerful', "1234567890123456", 1);
  $fx->('sumpowerful', "1234567890", 2); $fx->('sumpowerful', "1234567890123456", 2);
  $fx->('sumpowerful', 2147516495, $_) for 1..33;
  $fx->('sumpowerful', "1234567890123456", $_) for 3..32;

  $fx->('powerful_numbers', @$_) for ([40,180,3],[9,20,0],[9,20,1],[120],[9,120],[9,200,2],[0,200,3],[1,200,4],
    [1,1000,5],[1000000000000,1010000000000,5],[$U,$U,1],[1,121,"9223372036854775809"],[2,121,"9223372036854775809"],
    [0,3,1],[1,121,-1],[1000],[1,1000,2],[1,1000,3],[1,1000,0]);

  # ---- powerfree (t/26-powerfree.t)
  my @simple = (0..16, 758096738,434420340,870589313,695486396,602721315,418431087,752518565,723570005,506916483,617459403, map { -$_ } 1..32);
  $fx->('is_powerfree', $_) for @simple;
  $fx->('is_powerfree', $_, 3) for @simple;
  $fx->('is_powerfree', $_, 0) for 0..10; $fx->('is_powerfree', $_, 1) for 0..10;
  $fx->('is_powerfree', -8, 2);
  for my $f (qw/is_powerfree powerfree_count powerfree_sum powerfree_part powerfree_part_sum nth_powerfree/) {
    $fx->($f, 8, -1); $fx->($f, 8, "4294967296");
  }
  $fx->('powerfree_count', -8, 2); $fx->('powerfree_sum', -8, 2); $fx->('powerfree_part', -8, 2);
  $fx->('powerfree_part_sum', -8, 2); $fx->('nth_powerfree', -8, 2);
  for my $k (0..10) { for my $n (0..100) { $fx->('powerfree_count', $n, $k); $fx->('powerfree_sum', $n, $k); } }
  $fx->('powerfree_count', $_) for 0, 1, 100, 12345, 1000000;
  $fx->('powerfree_sum', $_) for 0, 1, 100, 12345, 1000000;
  $fx->('powerfree_count', 12345, $_) for 2, 3, 4;
  $fx->('powerfree_sum', 12345, $_) for 2, 3, 4;
  $fx->('powerfree_count', 123456, 32); $fx->('powerfree_sum', 123456, 32);
  $fx->('powerfree_count', $U, 2); $fx->('powerfree_count', "1000000000000", 3); $fx->('powerfree_sum', "1000000000000", 3);
  $fx->('nth_powerfree', 7503); $fx->('nth_powerfree', 10272, 3); $fx->('nth_powerfree', 11408, 4);
  $fx->('nth_powerfree', 915099, 3); $fx->('nth_powerfree', "1000000", 2); $fx->('nth_powerfree', "1000000", 3);
  $fx->('nth_powerfree', "100000000", 5);
  $fx->('nth_powerfree', "15345982395028449439", 3); $fx->('nth_powerfree', $U, 64);
  $fx->('nth_powerfree', $_) for 0..50; $fx->('nth_powerfree', $_, 3) for 0..50;
  $fx->('powerfree_part', $_) for 0..60, -4000;
  $fx->('powerfree_part', $_, 3) for 0..60;
  my $big = "3709362688507618309707310743757146859608351353598858915828644464895074572939593330420817674692554750";
  $fx->('powerfree_part', $big, $_) for 0..7;
  for my $k (0..7) { $fx->('powerfree_part_sum', $_, $k) for 0..32, 54321; }
  $fx->('powerfree_part_sum', $_) for 0, 10, 54321;
  $fx->('squarefree_kernel', $_) for 0, 1, 2, 48, -48, 60, 2*3*3*3*5*5*7, "15110906613599", "4011892804050009", 1..40, $U, '18446744065119617025';
  $fx->('powerfree_part', $_) for 60, 2*3*3*3*5*5*7, "15110906613599", "4011892804050009", -48, 48;

  # ---- goldbach (t/26-goldbach.t)
  my @gbn = (0..34, 50, 66, 130, 258, 4022, 60119912, 15317795894, "3325581707333960528", "83778272185315920949659591651127238812", 100, 1000, 10000, 100000, '18446744073709551614');
  $fx->('minimal_goldbach_pair', $_) for @gbn;
  $fx->('goldbach_pair_count', $_) for grep { $_ < 1e8 } @gbn;
  $fx->('goldbach_pair_count', $_) for 1000000, 3325581707333960528 == 0 ? () : ();
  $fx->('goldbach_pairs', $_) for grep { $_ < 100000 } @gbn;

  # ---- gaussian primes (t/26-isgaussianprime.t)
  my @g = ([29,0],[31,0],[0,-29],[0,-31],[58924,132000511],[519880,-2265929],[20571,150592260]);
  $fx->('is_gaussian_prime', @$_) for @g;
  for my $a (-6..6) { for my $b (-6..6) { $fx->('is_gaussian_prime', $a, $b); } }
  for my $p (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103) {
    $fx->('is_gaussian_prime', $p, 0); $fx->('is_gaussian_prime', -$p, 0); $fx->('is_gaussian_prime', 0, $p);
    $fx->('is_gaussian_prime', 0, -$p); $fx->('is_gaussian_prime', $p, $p); $fx->('is_gaussian_prime', $p, 1);
    $fx->('is_gaussian_prime', -$p, -2);
  }
  $fx->('is_gaussian_prime', @$_) for ([4294967291,0],[0,4294967291],[4294967295,4294967295],
    ["18446744073709551557",0],["-9223372036854775808",1],["9223372036854775807","9223372036854775806"],
    [$U,$U],["18446744073709551615",2]);

  # ---- sum of squares
  $fx->('is_sum_of_squares', $_) for 0..130, 4294967295, '18446744073709551615', '18446744073709551557';
  for my $k (1..5) { $fx->('is_sum_of_squares', $_, $k) for 0..60; }
  $fx->('is_sum_of_squares', $_, 2) for 1000, 1009, 1001, 4294967291, '18446744073709551557';
  $fx->('is_sum_of_squares', -5, 2); $fx->('is_sum_of_squares', 7, 0);
}

1;
