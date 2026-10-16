#!/usr/bin/env perl
use strict;
use warnings;
use FindBin;
use lib "$FindBin::Bin/../.perl-local/lib/perl5";
use Cwd qw(abs_path);
use File::Copy qw(copy);
use File::Path qw(make_path);
use File::Temp qw(tempdir);
use JSON::PP qw(decode_json encode_json);
use Time::HiRes qw(clock_gettime CLOCK_MONOTONIC);

# Workloads from benches/int.rs. These names are Criterion benchmark IDs.
# No algorithm is ported here. Time includes the Perl-to-XS call cost.
my @inputs = qw(1000003 4294967311 2305843009213693951 18446744073709551557);
my @names = qw(sqrtint rootint3 rootint7 is_power_exp logint10 gcd mulmod powmod invmod);

sub read_text {
    my ($path) = @_;
    open my $fh, '<', $path or die "Cannot read $path: $!\n";
    local $/;
    return <$fh>;
}

sub sample_iterations {
    my ($sample) = @_;
    my $iters = $sample->{iters};
    my $times = $sample->{times};
    die "Invalid Criterion sample\n" unless ref($iters) eq 'ARRAY'
        && ref($times) eq 'ARRAY' && @$iters && @$iters == @$times;
    my ($count, $elapsed) = (0, 0);
    for my $i (0 .. $#$iters) {
        die "Invalid Criterion iteration count\n" unless defined($iters->[$i])
            && $iters->[$i] =~ /\A[0-9]+(?:\.0+)?\z/ && $iters->[$i] > 0;
        die "Invalid Criterion elapsed time\n" unless defined($times->[$i])
            && $times->[$i] =~ /\A[0-9]+(?:\.[0-9]+)?\z/ && $times->[$i] > 0;
        $count += $iters->[$i];
        $elapsed += $times->[$i];
    }
    return ($count, $elapsed / $count);
}

sub xs_call {
    my ($name, $n) = @_;
    return sub { Math::Prime::Util::sqrtint($n) } if $name eq 'sqrtint';
    return sub { Math::Prime::Util::rootint($n, 3) } if $name eq 'rootint3';
    return sub { Math::Prime::Util::rootint($n, 7) } if $name eq 'rootint7';
    return sub { Math::Prime::Util::is_power($n) } if $name eq 'is_power_exp';
    return sub { Math::Prime::Util::logint($n, 10) } if $name eq 'logint10';
    return sub { Math::Prime::Util::gcd($n, 600851475143) } if $name eq 'gcd';
    my $a = $n - 12345;
    return sub { Math::Prime::Util::mulmod($a, $a - 1, $n) } if $name eq 'mulmod';
    return sub { Math::Prime::Util::powmod($a, $n - 1, $n) } if $name eq 'powmod';
    return sub { Math::Prime::Util::invmod($a, $n) } if $name eq 'invmod';
    die "Unknown benchmark: $name\n";
}

sub time_xs {
    my ($call, $count) = @_;
    my $start = clock_gettime(CLOCK_MONOTONIC);
    for (my $i = 0; $i < $count; ++$i) { my $result = $call->(); }
    my $elapsed = clock_gettime(CLOCK_MONOTONIC) - $start;
    die "XS timer did not advance\n" unless $elapsed > 0;
    return $elapsed * 1e9 / $count;
}

sub main {
    die "Usage: perl tools/bench_vs_perl.pl int\n" unless @_ == 1 && $_[0] eq 'int';
    eval { require Math::Prime::Util; 1 }
        or die "Need a local Math::Prime::Util 0.75 XS install. No network bootstrap is run.\n";
    die "Need Math::Prime::Util 0.75\n" unless $Math::Prime::Util::VERSION eq '0.75';
    my $config = Math::Prime::Util::prime_get_config();
    die "Need the 64-bit MPU XS backend\n" unless $config->{xs} && $config->{maxbits} == 64;
    # Do not let an optional GMP backend replace the native XS measurements.
    Math::Prime::Util::prime_set_config(gmp => 0);
    my $root = abs_path("$FindBin::Bin/..");
    make_path("$root/target");
    # A private manifest supplies harness=false, absent from the crate manifest.
    # Each launch owns its output. Cargo serializes access to its build cache.
    my $run = tempdir('bench-vs-perl-XXXXXX', DIR => "$root/target", CLEANUP => 1);
    my $manifest = read_text("$root/Cargo.toml");
    $manifest .= "\n[lib]\npath = " . encode_json("$root/src/lib.rs")
        . "\n[[bench]]\nname = \"int\"\nharness = false\npath = "
        . encode_json("$root/benches/int.rs") . "\n";
    open my $fh, '>', "$run/Cargo.toml" or die "Cannot write manifest: $!\n";
    print {$fh} $manifest;
    close $fh or die "Cannot close manifest: $!\n";
    copy("$root/Cargo.lock", "$run/Cargo.lock") or die "Cannot copy lockfile: $!\n";
    system('sh', "$root/tools/bench_rust.sh", "$run/Cargo.toml", "$run/criterion", "$root/target") == 0
        or die "Rust benchmark failed\n";
    my @rows;
    for my $name (@names) {
        my $group = $name =~ /\A(?:mulmod|powmod|invmod)\z/ ? 'modular' : 'int';
        for my $n (@inputs) {
            my $sample = decode_json(read_text("$run/criterion/$group/$name/$n/new/sample.json"));
            my ($count, $rust_ns) = sample_iterations($sample);
            my $c_ns = time_xs(xs_call($name, $n), $count);
            push @rows, [$name, $n, $count, $rust_ns, $c_ns];
        }
    }
    print "MPU 0.75 XS; C ns includes Perl dispatch and loop cost. Ratio = Rust / C.\n";
    print "Same inputs and total measured iteration counts as Criterion; warmup is excluded.\n";
    printf "%-13s %20s %14s %12s %12s %8s %s\n",
        qw(function input iterations), 'Rust ns', 'C ns', 'ratio', '1.2x bar';
    for my $row (@rows) {
        my ($name, $n, $count, $rust, $c) = @$row;
        my $ratio = $rust / $c;
        printf "%-13s %20s %14.0f %12.3f %12.3f %7.3fx %s\n",
            $name, $n, $count, $rust, $c, $ratio, $ratio <= 1.2 ? 'within' : 'above';
    }
    return 0; # The performance bar is a report, not a gate.
}

exit main(@ARGV) unless caller;
1;

__END__

=head1 NAME

bench_vs_perl.pl - Compare the integer Criterion workloads with MPU XS.

=head1 USAGE

  perl tools/bench_vs_perl.pl int

Use a local MPU 0.75 XS install and cached Cargo dependencies. This command
uses no network. It runs the workloads in benches/int.rs. It replays each
sample's total iteration count through XS and reports nanoseconds per call.
The rootint3 and rootint7 rows call rootint with degree 3 and 7.
The is_power_exp row calls Perl's is_power in scalar context.
The C column includes Perl loop and call costs. It is not a pure C timer.
The 1.2x bar is reported and does not change the exit status.
Each launch has private samples. Concurrent launches can affect CPU timing.
No baseline or fixture file is changed.

=cut
