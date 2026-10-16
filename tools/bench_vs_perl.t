#!/usr/bin/env perl
use strict;
use warnings;
use Test::More;
use FindBin;
use File::Temp qw(tempdir);
use File::Path qw(make_path);
use JSON::PP qw(encode_json);
require "$FindBin::Bin/bench_vs_perl.pl";

is_deeply([sample_iterations({iters => [2, 3], times => [8, 18]})],
    [5, 5.2], 'weighted nanoseconds use the exact measured call count');
for my $bad (undef, [], {}, {iters => [], times => []}, {iters => [0], times => [1]},
    {iters => [1], times => []}, {iters => ['NaN'], times => [1]},
    {iters => [1], times => [-1]}) {
    eval { sample_iterations($bad) };
    like($@, qr/Invalid Criterion/, 'reject invalid samples before timing');
}
eval { main('unsupported') };
like($@, qr/Usage:/, 'reject unsupported families before starting processes');
eval { main('int', 'extra') };
like($@, qr/Usage:/, 'reject extra arguments');

# The fake XS functions expose arguments and scalar context, without MPU.
{
    no strict 'refs';
    for my $name (qw(sqrtint rootint is_power logint gcd mulmod powmod invmod)) {
        *{"Math::Prime::Util::$name"} = sub { return join(':', $name, @_); };
    }
}
my %expected = (
    sqrtint => 'sqrtint:1000003', rootint3 => 'rootint:1000003:3',
    rootint7 => 'rootint:1000003:7', is_power_exp => 'is_power:1000003',
    logint10 => 'logint:1000003:10', gcd => 'gcd:1000003:600851475143',
    mulmod => 'mulmod:987658:987657:1000003',
    powmod => 'powmod:987658:1000002:1000003', invmod => 'invmod:987658:1000003',
);
for my $name (sort keys %expected) {
    is(scalar xs_call($name, 1000003)->(), $expected{$name}, "$name uses the Rust arguments");
}
is(scalar xs_call('powmod', '18446744073709551557')->(),
    'powmod:18446744073709539212:18446744073709551556:18446744073709551557',
    'large inputs retain all 64 bits in Perl arithmetic');
my $calls = 0;
ok(time_xs(sub { ++$calls }, 17) > 0, 'monotonic timer advances');
is($calls, 17, 'XS loop runs the exact count');
eval { xs_call('unknown', 1000003) };
like($@, qr/Unknown benchmark/, 'reject unknown benchmark IDs');

# Run the driver twice at once. Fake Cargo writes samples into its own output.
my $tmp = tempdir(CLEANUP => 1);
make_path("$tmp/Math/Prime");
sub write_text {
    my ($path, $text) = @_;
    open my $fh, '>', $path or die $!;
    print {$fh} $text;
    close $fh or die $!;
}
write_text("$tmp/Math/Prime/Util.pm", <<'MODULE');
package Math::Prime::Util;
our $VERSION = $ENV{BENCH_TEST_VERSION} // '0.75';
sub prime_get_config { return { xs => !($ENV{BENCH_TEST_NO_XS} // 0), maxbits => 64 } }
sub prime_set_config { die 'GMP was not disabled' unless @_ == 2 && $_[0] eq 'gmp' && $_[1] == 0 }
sub AUTOLOAD { return 1 }
1;
MODULE
write_text("$tmp/cargo", <<'CARGO');
#!/usr/bin/env perl
use strict;
use warnings;
use File::Path qw(make_path);
use JSON::PP qw(encode_json);
die 'wrong command' unless join(' ', @ARGV) =~ /bench --offline --locked .*--output-format bencher/;
exit 3 if $ENV{BENCH_TEST_CARGO_FAIL};
my $base = $ENV{CRITERION_HOME};
open my $log, '>>', $ENV{BENCH_TEST_LOG} or die $!;
print {$log} "$base\n";
close $log;
for my $name (qw(sqrtint rootint3 rootint7 is_power_exp logint10 gcd mulmod powmod invmod)) {
    my $group = $name =~ /^(mulmod|powmod|invmod)$/ ? 'modular' : 'int';
    for my $n (qw(1000003 4294967311 2305843009213693951 18446744073709551557)) {
        my $dir = "$base/$group/$name/$n/new";
        make_path($dir);
        open my $fh, '>', "$dir/sample.json" or die $!;
        print {$fh} encode_json({iters => [2, 3], times => [2000000000, 3000000000]});
        close $fh;
    }
}
CARGO
chmod 0755, "$tmp/cargo" or die $!;
local $ENV{PERL5LIB} = "$tmp";
local $ENV{PATH} = "$tmp:$ENV{PATH}";
local $ENV{BENCH_TEST_LOG} = "$tmp/paths";
my @children;
for my $i (1 .. 2) {
    my $pid = fork();
    die $! unless defined $pid;
    if (!$pid) {
        open STDOUT, '>', "$tmp/output$i" or die $!;
        open STDERR, '>', "$tmp/error$i" or die $!;
        exec $^X, "$FindBin::Bin/bench_vs_perl.pl", 'int';
        die $!;
    }
    push @children, $pid;
}
for my $pid (@children) {
    waitpid($pid, 0);
    is($?, 0, 'independent driver launch succeeds');
}
my @paths = split /\n/, read_text("$tmp/paths");
is(scalar @paths, 2, 'both launches ran Cargo');
isnt($paths[0], $paths[1], 'concurrent launches have private samples');
for my $i (1 .. 2) {
    my $output = read_text("$tmp/output$i");
    my @rows = $output =~ /^(?:sqrtint|rootint[37]|is_power_exp|logint10|gcd|mulmod|powmod|invmod)\s+/mg;
    is(scalar @rows, 36, 'all workload rows are present');
    like($output, qr/1\.2x bar/, 'performance bar is reported');
    like($output, qr/above/, 'slow ratios do not fail the command');
}
for my $path (@paths) {
    ok(!-d $path, 'private samples are cleaned after success');
}
for my $case (
    ['BENCH_TEST_VERSION', '0.74', qr/Need Math::Prime::Util 0.75/],
    ['BENCH_TEST_NO_XS', '1', qr/Need the 64-bit MPU XS backend/],
    ['BENCH_TEST_CARGO_FAIL', '1', qr/Rust benchmark failed/],
) {
    local $ENV{$case->[0]} = $case->[1];
    my $pid = fork();
    die $! unless defined $pid;
    if (!$pid) {
        open STDOUT, '>', "$tmp/failure-output" or die $!;
        open STDERR, '>', "$tmp/failure-error" or die $!;
        exec $^X, "$FindBin::Bin/bench_vs_perl.pl", 'int';
        die $!;
    }
    waitpid($pid, 0);
    isnt($?, 0, "$case->[0] fails with no ratio report");
    like(read_text("$tmp/failure-error"), $case->[2], 'failure names its cause');
    unlike(read_text("$tmp/failure-output"), qr/Rust ns/, 'no partial report on failure');
}
done_testing();
