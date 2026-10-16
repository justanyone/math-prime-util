# Fixture family `digits`: t/26-digits.t, t/26-zeckendorf.t, t/19-popcount.t and the integer-op files
# t/18-10-unary_int.t, t/18-15-cmpint.t, t/18-20-addint.t, t/18-22-mulint.t, t/18-23-muladdint.t,
# t/18-26-shiftint.t, t/18-40-divmodrem.t, t/18-70-toint.t, t/18-91-edge.t.
# (MPU 0.75 has no t/18-28-bitops.t; no bit-op function is covered.)
# Signed inputs keep their minus sign; results are written as MPU prints them (bigints in full).
use strict; use warnings;

sub gen_digits {
  my $fh = shift;
  my $B = sub { $_[0] ? 1 : 0 };
  my $L = sub { @_ ? @_ : (q{}) };   # empty list result -> empty field, not undef

  # signed value pool used by the integer-op grids
  my @V = (-3..3, -100, 100, 255, -256, 65535, -65536, 2147483647, -2147483648, 4294967295, -4294967296,
    '9223372036854775807', '-9223372036854775807', '9223372036854775808', '-9223372036854775808',
    '-9223372036854775809', '18446744073709551614', '18446744073709551615', '-18446744073709551615',
    '18446744073709551616', '-18446744073709551616', '36893488147419103232',
    '13282407956253574712', '14991082624209354397', '28778071877862015', '641',
    '-123456789', '987654321', '1178630961471601951655862', '-2555488174170453670799',
    '230948092384903284908329048239084023984092384', '-982349082340982348502392937523840234029384908325098234');

  # ---- unary: t/18-10-unary_int.t
  for my $n (-100..100, @V, '000184467440737095516160', '+000000000000000000000', '-000184467440737095516160') {
    emit($fh, 'absint', [$n], sub { absint($n) });
    emit($fh, 'negint', [$n], sub { negint($n) });
    emit($fh, 'signint', [$n], sub { signint($n) });
    emit($fh, 'add1int', [$n], sub { add1int($n) });
    emit($fh, 'sub1int', [$n], sub { sub1int($n) });
  }

  # ---- binary grids: cmpint, addint, subint, mulint, divint, modint, cdivint, divrem family
  for my $a (@V) { for my $b (@V) {
    emit($fh, 'cmpint', [$a,$b], sub { cmpint($a,$b) });
    emit($fh, 'addint', [$a,$b], sub { addint($a,$b) });
    emit($fh, 'subint', [$a,$b], sub { subint($a,$b) });
    emit($fh, 'mulint', [$a,$b], sub { mulint($a,$b) });
    emit($fh, 'divint', [$a,$b], sub { divint($a,$b) });
    emit($fh, 'modint', [$a,$b], sub { modint($a,$b) });
    emit($fh, 'cdivint', [$a,$b], sub { cdivint($a,$b) });
    emit($fh, 'divrem', [$a,$b], sub { divrem($a,$b) });
    emit($fh, 'tdivrem', [$a,$b], sub { tdivrem($a,$b) });
    emit($fh, 'fdivrem', [$a,$b], sub { fdivrem($a,$b) });
    emit($fh, 'cdivrem', [$a,$b], sub { cdivrem($a,$b) });
  } }
  for my $a (-3..3) { for my $b (-3..3) {
    emit($fh, 'cmpint', [$a,$b], sub { cmpint($a,$b) });
    emit($fh, 'addint', [$a,$b], sub { addint($a,$b) });
    emit($fh, 'subint', [$a,$b], sub { subint($a,$b) });
    emit($fh, 'mulint', [$a,$b], sub { mulint($a,$b) });
    for my $c (-3..3) {
      emit($fh, 'muladdint', [$a,$b,$c], sub { muladdint($a,$b,$c) });
      emit($fh, 'mulsubint', [$a,$b,$c], sub { mulsubint($a,$b,$c) });
    }
  } }
  # t/18-23-muladdint.t selected rows plus boundary mixes
  my @mac = ([3,5,7],[-3,5,7],[3,-5,7],[3,5,-7],[6,7,-1],[1000000,1000000,0],
    [qw/9223372036854775807 1 1/],[qw/9223372036854775807 2 1/],[qw/9223372036854775807 2 -1/],
    [qw/-9223372036854775807 1 -1/],[qw/18446744073709551615 1 0/],[qw/18446744073709551615 1 -1/],
    [qw/18446744073709551615 1 1/],[qw/-1 18446744073709551615 1/],
    [qw/13282407956253574712 14991082624209354397 0/],[qw/4294967295 4294967296 0/],
    [qw/9223372036854775807 -1 0/],[qw/-9223372036854775808 1 0/],[qw/-9223372036854775807 -1 0/]);
  for my $a (@V[0,8,12,14,18,19,21,22,25]) { for my $b (@V[2,5,13,15,20,23]) { for my $c (@V[3,7,16,17,24]) { push @mac, [$a,$b,$c]; } } }
  for my $t (@mac) { my ($a,$b,$c) = @$t;
    emit($fh, 'muladdint', [$a,$b,$c], sub { muladdint($a,$b,$c) });
    emit($fh, 'mulsubint', [$a,$b,$c], sub { mulsubint($a,$b,$c) });
  }
  # t/18-40-divmodrem.t selected
  for my $t ([9949242744253247,64],[-9949242744253247,64],["39458349850349850394853049583049",85889],
             ["39458349850349850394853049583049",-85889],["-39458349850349850394853049583049",85889],
             ["-39458349850349850394853049583049",-85889],["-1117091728166568014",59],
             ["1895315831",-1],["3483637757",-1],["6127303089832103323",-1],["13026328650942325963",-1],
             ["14123555781055773270",2],["12844039487317506779","12844039487317506779"],
             [3,"12844039487317506779"],[-3,"12844039487317506779"],
             ["9223372036854775807","281474976710656"],["9223372036854775808","281474976710656"],
             ["9223372036854775809","281474976710656"],[0,0],[1,0],[-1,0]) {
    my ($a,$b) = @$t;
    emit($fh, $_->[0], [$a,$b], $_->[1]) for
      (['divint', sub { divint($a,$b) }], ['modint', sub { modint($a,$b) }], ['cdivint', sub { cdivint($a,$b) }],
       ['divrem', sub { divrem($a,$b) }], ['tdivrem', sub { tdivrem($a,$b) }],
       ['fdivrem', sub { fdivrem($a,$b) }], ['cdivrem', sub { cdivrem($a,$b) }]);
  }
  for my $n (1024, -1024) { for my $d (1..1025) {
    emit($fh, 'divint', [$n,$d], sub { divint($n,$d) });
    emit($fh, 'modint', [$n,$d], sub { modint($n,$d) });
  } }
  for my $a (-8,-1,1,8) { for my $b (-3,-2,2,3) {
    emit($fh, 'divrem', [$a,$b], sub { divrem($a,$b) });
    emit($fh, 'tdivrem', [$a,$b], sub { tdivrem($a,$b) });
    emit($fh, 'fdivrem', [$a,$b], sub { fdivrem($a,$b) });
    emit($fh, 'cdivrem', [$a,$b], sub { cdivrem($a,$b) });
    emit($fh, 'cdivint', [$a,$b], sub { cdivint($a,$b) });
  } }

  # ---- shifts: t/18-26-shiftint.t
  my @sh = ([0,1],[-1,1],[-5,1],[-8,2],[-65535,15],[-65536,15],[-65535,16],[-65536,16],[-65535,47],[-65536,47],
    [-65535,48],[-65536,48],[-65536,80],[-307385513,6],[-2332267979728172537,6],['-17640827963513397449',6],
    ['-79231600218559026832557301750107210001',6],['-416649423645764932216789232242651032187',6],
    ['2147483648',1],['9223372036854775808',1]);
  for my $n (@V, 0, 1, 2, 7, 255, 65535, '340282366920938463463374607431768211456') { for my $k (0, 1, 3, 7, 15, 31, 32, 33, 63, 64, 65, 80, 128, 256, -1, -5, -64) { push @sh, [$n,$k]; } }
  push @sh, [$_,1] for 0..50; push @sh, [$_,5] for -65..65;
  for my $t (@sh) { my ($n,$k) = @$t;
    emit($fh, 'lshiftint', [$n,$k], sub { lshiftint($n,$k) });
    emit($fh, 'rshiftint', [$n,$k], sub { rshiftint($n,$k) });
    emit($fh, 'rashiftint', [$n,$k], sub { rashiftint($n,$k) });
  }
  for my $n (0..50, -50..-1, '9223372036854775808', '-9223372036854775808') {
    emit($fh, 'lshiftint', [$n], sub { lshiftint($n) });
    emit($fh, 'rshiftint', [$n], sub { rshiftint($n) });
    emit($fh, 'rashiftint', [$n], sub { rashiftint($n) });
  }

  # ---- powint / logint on signed (t/18-91-edge.t); unsigned versions live in family int
  for my $n (0, 1, -1, 2, -2, 3, -3, 10, -10, 255, '-9223372036854775808', '9223372036854775807') {
    for my $k (0, 1, 2, 3, 4, 5, 7, 10, 31, 63, 64, 65, 100) {
      emit($fh, 'powint', [$n,$k], sub { powint($n,$k) });
    }
    emit($fh, 'powint', [$n,-1], sub { powint($n,-1) });
  }
  for my $n (1..70, 255, 256, 1000, 65535, 65536, 4294967295, 4294967296, '9223372036854775808',
             '18446744073709551615', '18446744073709551616', '340282366920938463463374607431768211456', 0, -1) {
    for my $b (2, 3, 10, 16, 1, 0) { emit($fh, 'logint', [$n,$b], sub { logint($n,$b) }); }
  }

  # ---- toint: t/18-70-toint.t (no whitespace args: TSV args are space-separated)
  for my $s (0, 1, -1, 42, -42, 4294967295, 4294967296, '9223372036854775807', '18446744073709551615',
      3.9, -3.9, 0.9, -0.9, 1.0, 0.5, -0.5, 99.99, '0', '42', '-42', '+42', '007', '-007', '00', '1_000_000',
      '99999999999999999999', '18446744073709551616', '000000000000000000000' . '18446744073709551616',
      '+000000000000000000000' . '18446744073709551616', '-000000000000000000000' . '18446744073709551616',
      '0xFF', '-0xFF', '+0X10', '0b1011', '-0B1011', '0o777', '-0O10', '0xF_F_F', '0b1010_0110', '0o7_77',
      '0x00000100', '0b00010000', '0o00000100', '0x100000000000000000000', '-0x100000000000000000000',
      '0b1' . ('0' x 80), '0o4' . ('0' x 26),
      '3.14', '-3.14', '3.9', '-3.9', '3.', '-3.', '.5', '-.5', '+.5', '0.9', '-0.9', '0.0', '99.99', '1.5_0',
      '1e3', '1E3', '1.5e2', '1.5e+2', '3e0', '0e5', '1.5e-1', '1e20', '-1e20', '1.5e25', '123456789012345678901234567890.5',
      'abc', '1x', '0xZZ', '--5', '1.2.3', 'e5', '1e', '0x', '', 'inf', 'nan') {
    next if $s =~ /\s/;
    my $a = $s eq '' ? '(empty)' : $s;
    emit($fh, 'toint', [$a], sub { toint($s) });
  }

  # ---- todigits family: t/26-digits.t
  my @tdn = (0, 1, 77, 143, -143, 900, 1234135634, 56, 6345354, 12345, 4294967295, 4294967296,
    '18446744073709551615', '18446744073709551616', '-18446744073709551616', '+18446744073709551616',
    '000000000000000000000' . '18446744073709551616', '293852387239761276234029385230912847923872323',
    '340282366920938463463374607431768211457', '18446744073709551617', 3333333333333333333333);
  for my $n (@tdn) {
    emit($fh, 'todigits', [$n], sub { $L->(todigits($n)) });
    emit($fh, 'todigitstring', [$n], sub { todigitstring($n) });
    for my $b (2, 3, 7, 8, 9, 11, 16, 21, 36, 37, 100, 4294967296, '18446744073709551616', 1, 0) {
      emit($fh, 'todigits', [$n,$b], sub { $L->(todigits($n,$b)) });
      emit($fh, 'todigitstring', [$n,$b], sub { todigitstring($n,$b) });
    }
  }
  for my $nbl ([0,10,3],[0,10,0],[900,2,0],[900,2,3],[900,2,32],[56,2,8],[6345354,10,4],[12345,8,10],[77,10,1],[77,10,5],[-143,16,6],[255,16,1]) {
    my ($n,$b,$l) = @$nbl;
    emit($fh, 'todigits', [$n,$b,$l], sub { $L->(todigits($n,$b,$l)) });
    emit($fh, 'todigitstring', [$n,$b,$l], sub { todigitstring($n,$b,$l) });
  }
  emit($fh, 'todigitstring', [('3' x 21), 3], sub { todigitstring(('3' x 21), 3) });
  emit($fh, 'todigitstring', [('7' x 26), 9], sub { todigitstring(('7' x 26), 9) });
  emit($fh, 'todigitstring', [('9' x 27), 11], sub { todigitstring(('9' x 27), 11) });

  # fromdigits takes a digit string
  my @fd = (['0'],['1'],['1101'],['24'],['1923'],['5128'],['00123'],['1f',16],['1F',16],['1c8',16],['zzzyzzzyzzzyzzzy',36],
    ['0110',2],['01101',2],['1101',2],['1011011',2],['01101',3],['01101',16],['1234',5],['777',8],['abc',16],
    ['10','4294967296'],['10','1000000000000'],['1f',1],['123',0],['-1c8',16],['+1c8',16],['12',37],['1g',16],['2',2],
    ['9' x 30],['1' x 40, 2],['f' x 20, 16],['3788' . ('9' x 30)],['', 10],['x',10],['-5'],['3486784401',16],
    ['1',2],['10',2],['11111111111111111111111111111111111111111111111111111111111111111',2]);
  push @fd, [todigitstring(56,2), 2], [todigitstring(37,2), 2];
  for my $t (@fd) { my ($s,$b) = @$t;
    emit($fh, 'fromdigits', [$s], sub { fromdigits($s) }) unless defined $b;
    emit($fh, 'fromdigits', [$s,$b], sub { fromdigits($s,$b) }) if defined $b;
  }
  for my $s (qw/101 777 12 99 123/) { for my $b (2, 8, 10, 16, 36) { emit($fh, 'fromdigits', [$s,$b], sub { fromdigits($s,$b) }); } }

  # sumdigits / reverse_digits / is_palindrome / is_harshad / digital_root / mult_digital_root
  my @dn = (0..150, 1200, 123456, -143, 4294967295, '18446744073709551615', '293852387239761276234029385230912847923872323',
    '1000000000000000000000000000000000000000000000000010', '123456789012345678901234567890', 719848917, 532900,
    12345696, 12345697, 2016, 2017, '200000000000000000000', '200000000000000000002', 255, 16, '99999999999999999999');
  for my $n (@dn) {
    emit($fh, 'sumdigits', [$n], sub { sumdigits($n) });
    emit($fh, 'reverse_digits', [$n], sub { reverse_digits($n) });
    emit($fh, 'is_palindrome', [$n], sub { $B->(is_palindrome($n)) });
    emit($fh, 'is_harshad', [$n], sub { $B->(is_harshad($n)) });
    emit($fh, 'digital_root', [$n], sub { digital_root($n) });
    emit($fh, 'mult_digital_root', [$n], sub { mult_digital_root($n) });
    for my $b (2, 3, 7, 16, 36, 4294967296, 1, 0) {
      emit($fh, 'sumdigits', [$n,$b], sub { sumdigits($n,$b) });
      emit($fh, 'reverse_digits', [$n,$b], sub { reverse_digits($n,$b) });
      emit($fh, 'is_palindrome', [$n,$b], sub { $B->(is_palindrome($n,$b)) });
      emit($fh, 'is_harshad', [$n,$b], sub { $B->(is_harshad($n,$b)) });
      emit($fh, 'digital_root', [$n,$b], sub { digital_root($n,$b) });
      emit($fh, 'mult_digital_root', [$n,$b], sub { mult_digital_root($n,$b) });
    }
  }
  for my $t (['-45.36'], ['0x3290f8E'], ['1' x 50], ['-0'], ['0.5']) {
    my ($s) = @$t;
    emit($fh, $_, [$s], eval "sub { $_(\$s) }") for qw/sumdigits reverse_digits digital_root mult_digital_root/;
  }
  for my $t ([9,2],[10,2],[130,3],[130,4],[10,'4294967296'],['18446744073709551617','4294967296'],
             ['340282366920938463463374607431768211457','18446744073709551616'],
             [10,'4294967296'],['18446744073709551618','4294967296'],['340282366920938463463374607431768211458','18446744073709551616'],
             [6,6],[7,6],[14,6],[255,16],[10,'100000000000000000000000000000000000000'],[17,1]) {
    my ($n,$b) = @$t;
    emit($fh, 'is_palindrome', [$n,$b], sub { $B->(is_palindrome($n,$b)) });
    emit($fh, 'is_harshad', [$n,$b], sub { $B->(is_harshad($n,$b)) });
    emit($fh, 'digital_root', [$n,$b], sub { digital_root($n,$b) });
    emit($fh, 'mult_digital_root', [$n,$b], sub { mult_digital_root($n,$b) });
    emit($fh, 'reverse_digits', [$n,$b], sub { reverse_digits($n,$b) });
  }

  # ---- hammingweight: t/19-popcount.t
  for my $n (0, 1, 2, 3, 15, -15, 452398, -452398, 4294967295, '506198202011006650616143',
             '777777777777777714523989234823498234098249108234236', 0..300, '18446744073709551615',
             '18446744073709551616', '-18446744073709551615', '9223372036854775808', '-9223372036854775808') {
    emit($fh, 'hammingweight', [$n], sub { hammingweight($n) });
  }

  # ---- Zeckendorf: t/26-zeckendorf.t
  for my $n (0..300, 4294967295, 4294967296, '18446744073709551615', '18446744073709551616',
             '79228162514264337593543950335', 24, 27, 568, -1, -5) {
    emit($fh, 'tozeckendorf', [$n], sub { tozeckendorf($n) });
  }
  for my $n (0..300, 4294967295, '18446744073709551615', '79228162514264337593543950335') {
    my $z = tozeckendorf($n);
    emit($fh, 'fromzeckendorf', [$z], sub { fromzeckendorf($z) });
  }
  for my $s ('', '0', '00', '11', '0101', '1001', '2', 'abc', '10101', '1010010100000') {
    my $a = $s eq '' ? '(empty)' : $s;
    emit($fh, 'fromzeckendorf', [$a], sub { fromzeckendorf($s) });
  }
}

1;
