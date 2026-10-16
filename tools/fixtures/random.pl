# Fixture family `random`: t/34-random.t, t/35-cipher.t, t/35-rand-tag.t, t/16-randomprime.t.
# Every case starts with csrand($seed); the seed (raw bytes) is the first arg as lowercase hex.
# srand(N) in MPU 0.75 seeds the ChaCha20 CSPRNG with pack("V",N) (pack("V2",lo,hi) when N > 2^32-1),
# so the t files' srand cases appear here as csrand with that byte string.
# entropy_bytes is NOT deterministic and is not covered.
use strict; use warnings;

sub gen_random {
  my $fh = shift;
  my $blake = "BLAKEGrostlJHKeccakSkein--RijndaelSerpentTwofishRC6MARS";
  my $plain = 'Now I’m not gonna make a lot of extravagant claims for this little machine. Sure, it’ll change your whole life for the better, but that’s all.';
  my $key = "A lovely day for a ride";
  my $key2 = "The city needs a car like a fish needs a bicycle.";
  my $nonce = "20170412";
  my @seeds = (
    pack("V", 15), pack("V", 42), pack("V", 7652245), pack("V", 0x521974A3), pack("V", 1),
    pack("V2", 1, 1), pack("V2", 0xdeadbeef, 0x12345678), $nonce,
    pack("A32A8", $key, $nonce), pack("A32A8", $key2, $nonce), pack("A32A8", $key, "Berlin"),
    join("", map { chr($_) } 0..31),
    $blake, pack("A32A8", "x", "y") . "",
  );
  # seeds of 4, 8, 32 and 40 bytes are the core set; the 54-byte BLAKE seed is the t/34-random.t manual seed
  my $hx = sub { unpack("H*", $_[0]) };
  my $c = sub {   # case with seed: $c->(fn, seed, [args], code)
    my ($fn, $seed, $args, $code) = @_;
    emit($fh, $fn, [$hx->($seed), @$args], sub { csrand($seed); $code->() });
  };
  my $big = sub { my @r = @_; map { "$_" } @r };

  # ---- ChaCha test vectors (t/35-cipher.t, t/35-rand-tag.t, t/34-random.t) ----
  for my $s ([pack("A32A8",$key,$nonce),'cipher_key'], [pack("A32A8",$key2,$nonce),'cipher_key2'],
             [pack("A32A8",$key,"Berlin"),'cipher_berlin']) {
    my ($seed, $nm) = @$s;
    $c->("cipher_$nm", $seed, [length($plain)], sub { unpack("H*", $plain ^ random_bytes(length($plain))) });
    $c->("cipher_${nm}_L", $seed, [length($plain)], sub { unpack("L", $plain ^ random_bytes(length($plain))) });
  }
  # published expectations: 351607655 and 3391833874 (little-endian L of the ciphertext)
  my @tag = (pack("V",7652245));
  $c->("tag_irand_irand32_rand", $tag[0], [], sub { my @g = (irand(), irand32(), drand()); @g });
  $c->("tag_irand_irand64", $tag[0], [], sub { irand(); irand64() });
  $c->("tag_irand64", $tag[0], [], sub { irand64() });
  $c->("srand15_random_bytes4", pack("V",15), [4], sub { unpack("H8", random_bytes(4)) });
  $c->("blake_random_bytes7", $blake, [7], sub { unpack("H14", random_bytes(7)) });
  $c->("blake_irand_after7", $blake, [], sub { random_bytes(7); irand() });
  $c->("blake_irand", $blake, [], sub { irand() });
  $c->("blake_drand_after_irand", $blake, [], sub { random_bytes(7); irand(); drand() });
  $c->("srand42_irand", pack("V",42), [], sub { irand() });
  # buffer boundary cases (t/34-random.t)
  for my $tail (1..7) {
    $c->("irand32_after_bytes", pack("V",0x521974A3), [1024-$tail], sub { random_bytes(1024-$tail); irand32() });
    $c->("irand64_after_bytes", pack("V",0x521974A3), [1024-$tail], sub { random_bytes(1024-$tail); irand64() });
  }
  for my $n (0, 1, 63, 64, 65, 1000, 1023, 1024, 1025, 4096) {
    $c->("random_bytes_len_tail", pack("V",0x521974A3), [$n], sub { random_bytes($n); unpack("H*", random_bytes(8)) });
  }

  # ---- general per-seed cases ----
  for my $seed (@seeds) {
    my $L = length($seed);
    $c->("irand", $seed, [], sub { irand() });
    $c->("irand32", $seed, [], sub { irand32() });
    $c->("irand64", $seed, [], sub { irand64() });
    $c->("drand", $seed, [], sub { drand() });
    $c->("drand", $seed, [10], sub { drand(10) });
    $c->("drand", $seed, [-10], sub { drand(-10) });
    $c->("rand", $seed, [], sub { Math::Prime::Util::rand() });
    for my $n (0, 1, 2, 7, 16, 17, 64, 100) {
      $c->("random_bytes", $seed, [$n], sub { unpack("H*", random_bytes($n)) });
    }
    $c->("irand_seq", $seed, [], sub { join " ", map { irand() } 1..20 });
    $c->("irand64_seq", $seed, [], sub { join " ", map { irand64() } 1..20 });
    $c->("irand32_seq", $seed, [], sub { join " ", map { irand32() } 1..20 });
    $c->("drand_seq", $seed, [], sub { join " ", map { drand() } 1..20 });
    $c->("random_bytes_seq", $seed, [64], sub { unpack("H*", random_bytes(64)) });
    $c->("mixed_seq", $seed, [], sub { join " ", irand(), irand64(), unpack("H6",random_bytes(3)), irand32(), urandomb(5), irand() });
    for my $b (0, 1, 2, 5, 8, 16, 31, 32, 33, 63, 64, 65, 71, 100, 128) {
      $c->("urandomb", $seed, [$b], sub { "" . urandomb($b) });
    }
    for my $m (1, 2, 3, 7, 10, 50, 100, 1000, 65536, 4294967295, 4294967296, 4294967297, "9223372036854775808",
               "18446744073709551615", "18446744073709551616", "340282366920938463463374607431768211456", 0) {
      $c->("urandomm", $seed, [$m], sub { "" . urandomm($m) });
    }
    $c->("urandomm_seq", $seed, [10], sub { join " ", map { urandomm(10) } 1..20 });
    for my $r ([5,5],[7,3],[3,8],[0,1],[-3,3],[-5,-1],[0,4294967295],[0,"18446744073709551615"],
               [-10,"18446744073709551615"],["1000000000000","1000000000100"],[0,"100000000000000000000"]) {
      $c->("urandomr", $seed, $r, sub { my $v = &urandomr(@$r); defined $v ? "$v" : undef });
    }
    # primes
    for my $r ([2,2],[2,3],[3,5],[10,20],[8,12],[10,12],[2,20],[20,100],[5678,9876],[27767,88493],
               [17051687,17051899],[16706143,16706143],[3842610773,3842611109],[1000000,2000000],
               ["4294967296","4294967400"],["18446744073709551000","18446744073709551615"],
               ["100000000000000000000000","100000000000000000001000"],
               [0,0],[0,1],[2,1],[3,2],[1294268492,1294268778],[0,2],[1,"1000000000000"]) {
      $c->("random_prime", $seed, $r, sub { my $v = &random_prime(@$r); defined $v ? "$v" : undef });
    }
    for my $h (2,3,4,5,6,7,8,100,1000,1000000,4294967295,"18446744073709551615","100000000000000000000000") {
      $c->("random_prime", $seed, [$h], sub { my $v = random_prime($h); defined $v ? "$v" : undef });
    }
    for my $bad ([undef],[-3],["a"],[2,-4],[2,"a"]) {
      $c->("random_prime", $seed, [map { defined $_ ? $_ : 'undef' } @$bad], sub { "" . &random_prime(@$bad) });
    }
    for my $d (0..11, 15, 19, 20, 25, 30) {
      $c->("random_ndigit_prime", $seed, [$d], sub { "" . random_ndigit_prime($d) });
    }
    for my $b (0, 1, 2, 3, 4, 5, 8, 10, 16, 17, 28, 32, 34, 48, 63, 64, 65, 80, 100, 128) {
      $c->("random_nbit_prime", $seed, [$b], sub { "" . random_nbit_prime($b) });
    }
    for my $b (2, 3, 4, 5, 8, 16, 32, 40, 64) {
      $c->("random_safe_prime", $seed, [$b], sub { "" . random_safe_prime($b) });
    }
    for my $b (127, 128, 129, 200, 247, 256) {
      $c->("random_strong_prime", $seed, [$b], sub { "" . random_strong_prime($b) });
    }
    for my $b (3, 4, 5, 6, 7, 8, 9, 10, 16, 26, 32, 48, 64, 65, 81, 100) {
      $c->("random_semiprime", $seed, [$b], sub { "" . random_semiprime($b) });
      $c->("random_unrestricted_semiprime", $seed, [$b], sub { "" . random_unrestricted_semiprime($b) });
    }
    for my $n (1, 2, 10, 100, 1000, 1000000, "4294967296", "18446744073709551615", "100000000000000000000") {
      $c->("random_factored_integer", $seed, [$n], sub {
        my ($v, $f) = random_factored_integer($n);
        ("$v", map { "$_" } @$f) });
    }
    for my $n (0, 1, 2, 5, 10, 20, 100) {
      $c->("randperm", $seed, [$n], sub { randperm($n) });
    }
    for my $nk ([10,0],[10,1],[10,3],[10,10],[10,20],[100,5],[1000,4],[5,5],[1,1],["4294967296",3]) {
      $c->("randperm", $seed, $nk, sub { &randperm(@$nk) });
    }
    for my $l ([], [1], [1,2], [1..5], [1..10], ["a".."h"], [1..30]) {
      $c->("shuffle", $seed, $l, sub { shuffle(@$l) });
    }
    for my $kl ([0,1..5],[1,1..5],[3,1..9],[5,1..5],[7,1..5],[2,"a".."z"],[4,1..100],[3]) {
      my ($k, @l) = @$kl;
      $c->("vecsample", $seed, [$k, @l], sub { vecsample($k, @l) });
    }
  }
  # tightened semiprime/range checks from t/16-randomprime.t on a few seeds
  for my $seed (pack("V",1), pack("A32A8",$key,$nonce)) {
    for my $b (4..10, 26, 81) {
      $c->("random_semiprime_seq", $seed, [$b], sub { join " ", map { random_semiprime($b) } 1..10 });
    }
    $c->("random_nbit_prime_seq", $seed, [20], sub { join " ", map { random_nbit_prime(20) } 1..20 });
    $c->("random_prime_seq", $seed, [100,1000], sub { join " ", map { random_prime(100,1000) } 1..20 });
  }
}
1;
