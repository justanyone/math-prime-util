# Fixture family `primality`: t/10-isprime.t, t/15-probprime.t, t/17-pseudoprime.t, t/22-aks-prime.t,
# t/29-mersenne.t, t/26-delicateprime.t.
# Functions: is_prime, is_prob_prime, is_bpsw_prime, is_pseudoprime, is_euler_pseudoprime,
# is_strong_pseudoprime, is_lucas_pseudoprime, is_strong_lucas_pseudoprime,
# is_extra_strong_lucas_pseudoprime, is_almost_extra_strong_lucas_pseudoprime, is_euler_plumb_pseudoprime,
# is_perrin_pseudoprime, is_catalan_pseudoprime, is_frobenius_pseudoprime,
# is_frobenius_underwood_pseudoprime, is_frobenius_khashin_pseudoprime, is_aks_prime, is_mersenne_prime,
# is_provable_prime, is_delicate_prime, is_safe_prime, is_chen_prime.
# Results are the raw MPU return values (is_prime / is_prob_prime / is_provable_prime give 0, 1 or 2).
# Left out: bigint inputs (318665857834031151167461, the 1000-digit Perrin PRP), undef inputs,
# random-input tests, EXTENDED_TESTING-only cases.
use strict; use warnings;

sub gen_primality {
  my $fh = shift;
  my $U = Math::BigInt->new('18446744073709551615');
  my $code = sub { my $f = shift; no strict 'refs'; \&{"Math::Prime::Util::$f"} };
  # one case: function name, arg list (all strings)
  my %dup;
  my $one = sub {
    my ($fn, @a) = @_;
    return if $dup{join(' ', $fn, @a)}++;
    my $f = $code->($fn);
    emit($fh, $fn, [@a], sub { $f->(@a) });
  };
  my $each = sub { my ($fn, $list, @extra) = @_; $one->($fn, "$_", @extra) for @$list; };
  my $u64ok = sub { my $n = Math::BigInt->new("$_[0]"); $n >= 0 && $n <= $U };

  # ---- number sets
  my @small = (0..999);
  my @tl = (qw/9 121 341 561 645 703 781 1105 1387 1541 1729 1891 1905 2047 2465 2701 2821
    3277 3281 4033 4369 4371 4681 5461 5611 6601 7813 7957 8321 8401 8911 10585
    12403 13021 14981 15751 15841 16531 18721 19345 23521 24211 25351 29341
    29539 31621 38081 40501 41041 44287 44801 46657 47197 52633 53971 55969
    62745 63139 63973 74593 75361 79003 79381 82513 87913 88357 88573 97567
    101101 340561 488881 852841 1373653 1857241 6733693 9439201 17236801
    23382529 25326001 34657141 56052361 146843929 216821881 3215031751
    2152302898747 3474749660383 341550071728321 3825123056546413051/);
  my @tp = (qw/2 3 7 23 89 113 523 887 1129 1327 9551 15683 19609 31397 155921
    5 11 29 97 127 541 907 1151 1361 9587 15727 19661 31469 156007 360749
    370373 492227 1349651 1357333 2010881 4652507 17051887 20831533 47326913
    122164969 189695893 191913031 387096383 436273291 1294268779 1453168433
    2300942869 3842611109 4302407713 10726905041 20678048681 22367085353 25056082543 42652618807
    127976334671 182226896239 241160624143 297501075799 303371455241
    304599508537 416608695821 461690510011 614487453523 738832927927
    1346294310749 1408695493609 1968188556461 2614941710599/);
  my @carm = grep { is_carmichael($_) } (561 .. 3000000);
  @carm = @carm[0 .. 39] if @carm > 40;
  push @carm, qw/3215031751 9585541 1000000000000000000/;           # first is a base 2,3,5,7 spsp
  @carm = grep { is_carmichael($_) || $_ == 3215031751 } @carm;
  my @carm_big = grep { $_ > 3000000 } ();
  for my $k (3..30) {   # large Carmichael numbers from chernick-type forms: 6k+1,12k+1,18k+1
    for my $m (1 .. 2000) {
      my ($a,$b,$c) = (6*$m+1, 12*$m+1, 18*$m+1);
      if (is_prime($a) && is_prime($b) && is_prime($c)) { push @carm_big, $a*$b*$c; last if @carm_big > 40 }
    }
    last;
  }
  for my $m (2000 .. 400000) {   # 64-bit-range Chernick numbers
    last if @carm_big >= 60;
    my ($a,$b,$c) = (6*$m+1, 12*$m+1, 18*$m+1);
    push @carm_big, Math::BigInt->new($a)*$b*$c if is_prime($a) && is_prime($b) && is_prime($c) && ($m % 97 == 0 || $m > 100000 && $m % 13 == 0);
  }
  push @carm, grep { $u64ok->($_) } @carm_big;
  my @pow;                       # 2^k +- 1 (and 2^k)
  for my $k (1..64) {
    for my $d (-1, 0, 1) { my $v = Math::BigInt->new(2)->bpow($k) + $d; push @pow, "$v" if $u64ok->($v); }
  }
  for my $k (1..20) { for my $d (-1, 1) { my $v = Math::BigInt->new(3)->bpow($k) + $d; push @pow, "$v" if $u64ok->($v); } }
  my @near;                      # windows around powers of two
  for my $b (16, 31, 32, 48, 62, 63, 64) {
    my $c = Math::BigInt->new(2)->bpow($b);
    for my $d (-8 .. 8) { my $v = $c + $d; push @near, "$v" if $u64ok->($v); }
  }
  my @near_prime;                # primes next to the u32 / u64 limits
  { my $p = '4294967291'; my $q = '18446744073709551557';
    push @near_prime, qw/4294967291 4294967311 4294967357 4294967371 4294967377 4294967387 4294967389
                         18446744073709551557 18446744073709551533 18446744073709551521 18446744073709551437
                         9223372036854775783 9223372036854775837 281474976710597 4611686018427387847/; }
  my @edge = map { "$_" } edges();
  my %seen;
  my @big = grep { !$seen{$_}++ } (@tl, @tp, @carm, @pow, @near, @near_prime, @edge);
  @big = grep { $u64ok->($_) } @big;
  my @bigu32 = grep { Math::BigInt->new("$_") > 1000 } @big;   # small ones are covered by 0..999

  # ---- is_prime / is_prob_prime / is_bpsw_prime / is_provable_prime
  $one->('is_prime', '-1'); $one->('is_prime', '-2');
  $one->('is_prob_prime', '-1'); $one->('is_prob_prime', '-2');
  $one->('is_bpsw_prime', '-1');
  $one->('is_prime', "$_") for 0..3572;
  $one->('is_prime', int(2**$_)) for 2..20;
  $each->('is_prime', \@bigu32);
  $one->('is_prob_prime', "$_") for 0..999;
  $each->('is_prob_prime', \@bigu32);
  $one->('is_bpsw_prime', "$_") for 0..999;
  $each->('is_bpsw_prime', \@bigu32);
  $one->('is_bpsw_prime', $_) for 2, 3, 5, 31, 2047;
  $each->('is_provable_prime', [@small, @bigu32]);

  # ---- Fermat / Euler / strong: lists from t/17-pseudoprime.t
  my @psp = (
    [qw/2 341 561 645 1105 1387 1729 1905 2047 2465 2701 2821 3277 4033 4369 4371 4681 5461 6601 7957 8321 8481 8911 10261 10585 11305 12801 13741 13747 13981 14491 15709 15841 16705 18705 18721 19951 23001 23377 25761 29341/],
    [qw/3 91 121 286 671 703 949 1105 1541 1729 1891 2465 2665 2701 2821 3281 3367 3751 4961 5551 6601 7381 8401 8911 10585 11011 12403 14383 15203 15457 15841 16471 16531 18721 19345 23521 24046 24661 24727 28009 29161/],
  );
  my @spsp = (
    [qw/2 2047 3277 4033 4681 8321 15841 29341 42799 49141 52633 65281 74665 80581 85489 88357 90751 1194649/ ],
    [qw/3 121 703 1891 3281 8401 8911 10585 12403 16531 18721 19345 23521 31621 44287 47197 55969 63139 74593 79003 82513 87913 88573 97567/ ],
    [qw/5 781 1541 5461 5611 7813 13021 14981 15751 24211 25351 29539 38081 40501 44801 53971 79381/ ],
    [qw/7 25 325 703 2101 2353 4525 11041 14089 20197 29857 29891 39331 49241 58825 64681 76627 78937 79381 87673 88399 88831/ ],
    [qw/11 133 793 2047 4577 5041 12403 13333 14521 17711 23377 43213 43739 47611 48283 49601 50737 50997 56057 58969 68137 74089 85879 86347 87913 88831/ ],
    [qw/13 85 1099 5149 7107 8911 9637 13019 14491 17803 19757 20881 22177 23521 26521 35371 44173 45629 54097 56033 57205 75241 83333 85285 86347/ ],
    [qw/17 9 91 145 781 1111 2821 4033 4187 5365 5833 6697 7171 15805 19729 21781 22791 24211 26245 31621 33001 33227 34441 35371 38081 42127 49771 71071 74665 77293 78881 88831 96433 97921 98671/ ],
    [qw/19 9 49 169 343 1849 2353 2701 4033 4681 6541 6697 7957 9997 12403 13213 13747 15251 16531 18769 19729 24761 30589 31621 31861 32477 41003 49771 63139 64681 65161 66421 68257 73555 96049/ ],
    [qw/23 169 265 553 1271 2701 4033 4371 4681 6533 6541 7957 8321 8651 8911 9805 14981 18721 25201 31861 34133 44173 47611 47783 50737 57401 62849 82513 96049/ ],
    [qw/29 15 91 341 469 871 2257 4371 4411 5149 6097 8401 11581 12431 15577 16471 19093 25681 28009 29539 31417 33001 48133 49141 54913 79003/ ],
    [qw/31 15 49 133 481 931 6241 8911 9131 10963 11041 14191 17767 29341 56033 58969 68251 79003 83333 87061 88183/ ],
    [qw/37 9 451 469 589 685 817 1333 3781 8905 9271 18631 19517 20591 25327 34237 45551 46981 47587 48133 59563 61337 68101 68251 73633 79381 79501 83333 84151 96727/ ],
    [qw/61 217 341 1261 2701 3661 6541 6697 7613 13213 16213 22177 23653 23959 31417 50117 61777 63139 67721 76301 77421 79381 80041/ ],
    [qw/73 205 259 533 1441 1921 2665 3439 5257 15457 23281 24617 26797 27787 28939 34219 39481 44671 45629 64681 67069 76429 79501 93521/ ],
    [qw/325 341 343 697 1141 2059 2149 3097 3537 4033 4681 4941 5833 6517 7987 8911 12403 12913 15043 16021 20017 22261 23221 24649 24929 31841 35371 38503 43213 44173 47197 50041 55909 56033 58969 59089 61337 65441 68823 72641 76793 78409 85879/ ],
    [qw/9375 11521 14689 17893 18361 20591 28093 32809 37969 44287 60701 70801 79957 88357 88831 94249 96247 99547/ ],
    [qw/28178 28179 29381 30353 34441 35371 37051 38503 43387 50557 51491 57553 79003 82801 83333 87249 88507 97921 99811/ ],
    [qw/75088 75089 79381 81317 91001 100101 111361 114211 136927 148289 169641 176661 191407 195649/ ],
    [qw/450775 465991 468931 485357 505441 536851 556421 578771 585631 586249 606361 631651 638731 641683 645679/ ],
    [qw/642735 653251 653333 663181 676651 714653 759277 794683 805141 844097 872191 874171 894671/ ],
    [qw/9780504 9780505 9784915 9826489 9882457 9974791 10017517 10018081 10084177 10188481 10247357 10267951 10392241 10427209 10511201/ ],
    [qw/203659041 204172939 204456793 206407057 206976001 207373483 209301121 210339397 211867969 212146507 212337217 212355793 214400629 214539841 215161459/ ],
    [qw/553174392 553174393 553945231 554494951 554892787 555429169 557058133 557163157 557165209 558966793 559407061 560291719 561008251 563947141/ ],
    [qw/1005905886 1005905887 1007713171 1008793699 1010415421 1010487061 1010836369 1012732873 1015269391 1016250247 1018405741 1020182041/ ],
    [qw/1340600841 1345289261 1345582981 1347743101 1348964401 1350371821 1353332417 1355646961 1357500901 1361675929 1364378203 1366346521 1367104639/ ],
    [qw/1795265022 1795265023 1797174457 1797741901 1804469753 1807751977 1808043283 1808205701 1813675681 1816462201 1817936371 1819050257/ ],
    [qw/3046413974 3046413975 3048698683 3051199817 3068572849 3069705673 3070556233 3079010071 3089940811 3090723901 3109299161 3110951251 3113625601/ ],
    [qw/3613982119 3626488471 3630467017 3643480501 3651840727 3653628247 3654142177 3672033223 3672036061 3675774019 3687246109 3690036017 3720856369/ ],
  );
  my @bases = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 61, 73, 325, 9375);
  # per-base lists, as in the t file
  for my $p (@psp)  { my ($b, @v) = @$p; $one->('is_pseudoprime', $_, $b) for @v; }
  for my $p (@spsp) { my ($b, @v) = @$p; $one->('is_strong_pseudoprime', $_, $b) for @v; }
  # the same values against fixed bases (adds the negative cases)
  my %vals; $vals{$_}++ for map { @$_[1..$#$_] } @psp, @spsp;
  my @allv = sort { $a <=> $b } keys %vals;
  for my $v (@allv) {
    for my $b (2, 3, 5) { $one->('is_strong_pseudoprime', $v, $b); }
    for my $b (2, 3) { $one->('is_pseudoprime', $v, $b); $one->('is_euler_pseudoprime', $v, $b); }
  }
  # large ones
  $one->('is_pseudoprime', '4398117272641', 3);
  $one->('is_pseudoprime', 143168581, 2, 3, 5, 7, 11);
  $one->('is_strong_pseudoprime', '1099558795087', 3);
  # invalid bases and tiny n
  $one->('is_pseudoprime', 2047, 0); $one->('is_pseudoprime', 3, 1); $one->('is_pseudoprime', 3, 3);
  $one->('is_euler_pseudoprime', 703, 1); $one->('is_euler_pseudoprime', 3, 1); $one->('is_euler_pseudoprime', 3, 3);
  $one->('is_strong_pseudoprime', 2047, 0); $one->('is_strong_pseudoprime', 2047, 1); $one->('is_strong_pseudoprime', 3, 1);
  $one->('is_strong_pseudoprime', 0, 2); $one->('is_strong_pseudoprime', 1, 2);
  $one->('is_strong_pseudoprime', 2, 2); $one->('is_strong_pseudoprime', 3, 2);
  # bases >= n
  for my $nb ([3,3],[11,11],[89,5785],[257,6168],[367,367],[367,1101],[49001,921211727],[331,921211727],[49117,921211727]) {
    $one->('is_strong_pseudoprime', @$nb);
  }
  # base 2 (default) on the small integers, one-argument arity
  $one->('is_strong_pseudoprime', $_) for 0..4032;
  for my $n (0..999) { $one->('is_pseudoprime', $n); $one->('is_euler_pseudoprime', $n); }
  $one->('is_pseudoprime', $_) for 162193, 452051, 40165093, 66437841;
  $one->('is_pseudoprime', $_, 2, 3, 5) for 18595801, 22066201;
  $one->('is_pseudoprime', $_, 2) for 40165093, 66437841;
  # A014233: first strong pseudoprime to the first N prime bases
  my @sp = qw/2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71 73 79 83 89 97/;
  my @phis = qw/2047 1373653 25326001 3215031751 2152302898747 3474749660383 341550071728321 341550071728321/;
  for my $i (0..$#phis) { $one->('is_strong_pseudoprime', $phis[$i], @sp[0..$i]); }
  # base sets on the whole composite/prime/near sets: 1 base list and multi-base lists
  for my $n (@big) {
    $one->('is_strong_pseudoprime', $n);
    $one->('is_strong_pseudoprime', $n, 2, 3);
    $one->('is_strong_pseudoprime', $n, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37);
    $one->('is_strong_pseudoprime', $n, 2, 325, 9375, 28178, 450775, 9780504, 1795265022);
    $one->('is_strong_pseudoprime', $n, 2, 7, 61);
    $one->('is_pseudoprime', $n);
    $one->('is_pseudoprime', $n, 3);
    $one->('is_euler_pseudoprime', $n);
    $one->('is_euler_pseudoprime', $n, 3);
  }
  # the first-1000 integers with a base other than 2
  for my $n (0..999) {
    $one->('is_pseudoprime', $n, 3); $one->('is_euler_pseudoprime', $n, 3); $one->('is_strong_pseudoprime', $n, 3);
    $one->('is_strong_pseudoprime', $n, 2, 3, 5);
  }

  # ---- Lucas family
  my @lucas_fns = qw/is_lucas_pseudoprime is_strong_lucas_pseudoprime is_extra_strong_lucas_pseudoprime
                     is_almost_extra_strong_lucas_pseudoprime/;
  for my $fn (@lucas_fns) { $one->($fn, "$_") for 0..999; $each->($fn, \@bigu32); }
  my @lp = (
    ['is_lucas_pseudoprime', [qw/323 377 1159 1829 3827 5459 5777 9071 9179 10877 11419 11663 13919 14839 16109 16211 18407 18971 19043 2199055761527/]],
    ['is_strong_lucas_pseudoprime', [qw/5459 5777 10877 16109 18971 22499 24569 25199 40309 58519 75077 97439 100127 113573 115639 130139 4294967311 4294967357 12598021314449/]],
    ['is_extra_strong_lucas_pseudoprime', [qw/989 3239 5777 10877 27971 29681 30739 31631 39059 72389 73919 75077 100127 113573 125249 137549 137801 153931 155819 4294967311 4294967357 10099386070337/]],
    ['is_almost_extra_strong_lucas_pseudoprime', [qw/989 3239 5777 10469 10877 27971 29681 30739 31631 39059 72389 73919 75077 100127 113573 125249 137549 137801 153931 154697 155819 4294967311 4294967357 10071551814917/]],
  );
  for my $l (@lp) { my ($fn, $v) = @$l; $each->($fn, $v); for my $o (@lucas_fns) { next if $o eq $fn; $each->($o, $v) } }
  $each->('is_almost_extra_strong_lucas_pseudoprime', [qw/3239 4531 5777 10877 12209 21899 31631 31831 32129 34481 36079 37949 47849 50959 51641 62479 73919 75077 97109 100127 108679 113573 116899 154697 161027/], 2);
  for my $inc (1, 2, 3, 5, 7, 10, 100, 256) {   # arity 2; the increment must be 1..256
    $one->('is_almost_extra_strong_lucas_pseudoprime', $_, $inc) for qw/2 3 4 5 9 15 31 91 319 259 989 5777 10469 4294967311 4294967291/;
  }
  $one->('is_almost_extra_strong_lucas_pseudoprime', @$_) for [5777,0],[2,0],[4,257],[5,5],[31,28],[319,148],[259,256],[5777,257],[3,1];
  $one->('is_strong_lucas_pseudoprime', 2);
  $one->('is_lucas_pseudoprime', $_) for 9, 16, 100, 102, 2047, 2048, 5781, 9000, 14381;
  $one->('is_extra_strong_lucas_pseudoprime', $_) for 2..100;

  # ---- Euler-Plumb
  $each->('is_euler_plumb_pseudoprime', [0..999]);
  $each->('is_euler_plumb_pseudoprime', [qw/1729 1905 2047 2465 3277 4033 4681 8321 12801 15841 16705 18705 25761 29341 33153 34945 41041 42799 46657 49141 52633 65281 74665 75361 80581 85489 87249 88357 90751/]);
  $each->('is_euler_plumb_pseudoprime', \@bigu32);
  # Euler lists from the t file (bases 2, 3, 29)
  $each->('is_euler_pseudoprime', [qw/561 1105 1729 1905 2047 2465 3277 4033 4681 6601 8321 8481 10585 12801 15841 16705 18705 25761 29341 30121 33153 34945 41041 42799 46657 49141 52633 62745 65281 74665 75361 80581 85489 87249 88357 90751/]);
  $each->('is_euler_pseudoprime', [qw/121 703 1729 1891 2821 3281 7381 8401 8911 10585 12403 15457 15841 16531 18721 19345 23521 24661 28009 29341 31621 41041 44287 46657 47197 49141 50881 52633 55969 63139 63973 74593 75361 79003 82513 87913 88573 93961 97567/], 3);
  $each->('is_euler_pseudoprime', [qw/15 91 341 469 871 2257 4371 4411 5149 5185 6097 8401 8841 11581 12431 15577 15841 16471 19093 22281 25681 27613 28009 29539 31417 33001 41041 46657 48133 49141 54913 57889 79003 98301/], 29);

  # ---- Perrin (arity 1 and 2), Catalan, Frobenius family
  my @perrin = qw/271441 904631 16532714 24658561 27422714 27664033 46672291 102690901 130944133 196075949 214038533 517697641 545670533 801123451/;
  $each->('is_perrin_pseudoprime', [0..999]);
  $each->('is_perrin_pseudoprime', [@perrin, @bigu32]);
  for my $r (0..3) {
    $each->('is_perrin_pseudoprime', [0..300], $r);
    $each->('is_perrin_pseudoprime', [@perrin, '36407440637569', '364573433665', @near_prime, @carm[0..20]], $r);
  }
  $one->('is_perrin_pseudoprime', 271441, $_) for -1, 4, '4294967296';
  $each->('is_catalan_pseudoprime', [0..999]);
  $each->('is_catalan_pseudoprime', [5907, 1194649, 12327121, 561, 1105, 1729, 2047, 3215031751, 4294967291, 4294967297]);
  $each->('is_catalan_pseudoprime', [grep { $_ < 5000000 } @big]);
  # Frobenius (Zhou-Duo "Frobenius(P,Q)") and its arity
  $each->('is_frobenius_pseudoprime', [0..999]);
  $each->('is_frobenius_pseudoprime', \@bigu32);
  my @frob = ([1, -1, [qw/4181 5777 6721 10877 13201 15251 34561 51841 64079 64681 67861 68251 75077 90061 96049 97921 100127/]],
              [3, -5, [qw/13333 44801 486157 1615681 3125281 4219129 9006401 12589081 13404751 15576571 16719781/]]);
  for my $f (@frob) {
    my ($P, $Q, $v) = @$f;
    $one->('is_frobenius_pseudoprime', $_, $P, $Q) for @$v, 0..200;
    $one->('is_frobenius_pseudoprime', $_, $P, $Q) for grep { $_ > 1000 } @tp[0..30], @carm[0..30];
  }
  for my $pq ([2,2],[0,2],[4182,-1],[1,1],[1,-1],[3,-5],[5,-3],[-1,-1],[2,-1],[1,2],[3,1],[4,1],[0,1]) {
    $one->('is_frobenius_pseudoprime', $_, @$pq) for 5, 7, 11, 91, 101, 561, 1105, 4181, 5777, 13333, 100127;
  }
  $one->('is_frobenius_pseudoprime', @$_) for [101,2,2],[91,2,2],[561,0,2],[4181,4182,-1],[91,3],[91,0,0],[91,3,2],[5,3,2],[-5,0,0],
     [101,46339,1],[101,-46339,1],[101,1,46339],[101,1,-46339],[101,3037000498,1],[101,-3037000498,1],
     [101,1,3037000498],[101,1,-3037000498],[101,3037000497,3037000497],[101,-3037000497,-3037000497],[101,3037000498,3037000496];
  for my $fn (qw/is_frobenius_underwood_pseudoprime is_frobenius_khashin_pseudoprime/) {
    $one->($fn, "$_") for 0..999;
    $each->($fn, \@bigu32);
    $each->($fn, [@allv]);
    $each->($fn, [@perrin, qw/4181 5777 6721 10877 13201 15251 34561 51841 13333 44801 486157 1615681 3125281 4219129 9006401 12589081 13404751 15576571 16719781/]);
  }

  # ---- AKS (slow): n <= 2000, then a few larger values
  $one->('is_aks_prime', '-1'); $one->('is_aks_prime', '-2');
  $one->('is_aks_prime', $_) for 0..2000;
  $one->('is_aks_prime', $_) for 877, 101, 15481, 12109, 69197, 69199, 536891893, 999983, 1000003, 999999, 1000001, 65537, 104729, 524287, 1299709, 1299710, 4294967291;

  # ---- Mersenne exponents
  $one->('is_mersenne_prime', $_) for 0..130;
  $one->('is_mersenne_prime', $_) for '-5', 521, 607, 1279, 2203, 2281, 3217, 4253, 4423, 9689, 9941, 520, 606, 1000, 2047;

  # ---- delicate / safe / chen
  my @a050249 = (294001, 505447, 584141, 604171, 971767, 1062599, 1282529, 1524181, 2017963, 2474431, 2690201, 3085553, 3326489, 4393139, 5152507, 5564453, 5575259, 6173731, 6191371, 6236179, 6463267, 6712591, 7204777, 7469789, 7469797);
  my @a186995 = (127, 2, 373, 83, 28151, 223, 6211, 2789, 294001, 3347, 20837899, 4751, 6588721, 484439, 862789, 10513, 2078920243, 10909, 169402249, 2823167, 267895961, 68543, 1016960933671, 181141, 121660507, 6139219, 11646280537, 488651);
  $one->('is_delicate_prime', $_) for 504991, 929573, @a050249, '999999999998832431';
  $one->('is_delicate_prime', $_) for 0..999;
  $one->('is_delicate_prime', $a186995[$_-2], $_) for 2 .. 2+$#a186995;
  my %dr = (2 => [2,277], 3 => [2,283], 5 => [2,739], 7 => [200,1381]);
  for my $b (sort keys %dr) { $one->('is_delicate_prime', $_, $b) for $dr{$b}[0] .. $dr{$b}[1]; }
  $one->('is_delicate_prime', $_, 11) for @{primes(3000, 11500)};
  for my $b (4, 6, 8, 9, 12, 13, 16, 36) { $one->('is_delicate_prime', $_, $b) for 0..80; }
  $one->('is_delicate_prime', $_, 10) for @tp[0..40];
  $each->('is_delicate_prime', \@near_prime);

  $one->('is_safe_prime', '-5');
  $one->('is_safe_prime', "$_") for 0..999;
  my @safe;
  for my $c (qw/4294967296 9223372036854775808 18446744073709551615 1099511627776 281474976710656 1000000000000/) {
    my $p = Math::BigInt->new($c); my $got = 0;
    for (my $q = $p; $got < 6 && $q > $p - 2000000; $q = prev_prime($q)) {
      if (is_safe_prime($q)) { push @safe, "$q"; $got++; }
      $q = Math::BigInt->new("$q");
    }
  }
  $each->('is_safe_prime', [@safe, map { my $x = Math::BigInt->new($_); ("$x", $x*2+1 <= $U ? "".($x*2+1) : ()) } grep { Math::BigInt->new($_) < Math::BigInt->new('9223372036854775807') } @safe]);
  $each->('is_safe_prime', \@big);

  $one->('is_chen_prime', '-5');
  $one->('is_chen_prime', "$_") for 0..999;
  my @chen;
  for my $c (qw/4294967296 9223372036854775808 18446744073709551000 1099511627776 281474976710656/) {
    my $q = Math::BigInt->new($c);
    for (1..40) { $q = Math::BigInt->new("".next_prime($q)); last unless $u64ok->($q); push @chen, "$q"; }
  }
  $each->('is_chen_prime', [@chen]);
  $each->('is_chen_prime', \@big);
}
1;
