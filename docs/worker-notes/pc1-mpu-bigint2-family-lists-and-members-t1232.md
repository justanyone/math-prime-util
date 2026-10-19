t1232: plan was to generate bigint_families fixtures, bump the row-count assert 30->34, run tools/gate.sh.
Blocked: the sandbox denied sourcing tools/perl-env.sh and running perl tools/gen_fixtures.pl, so no fixtures were generated.
The assert stays at 30 (changing it without the fixtures would fail). The gate and coverage were not run.
