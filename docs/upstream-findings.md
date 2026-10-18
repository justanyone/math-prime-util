# Upstream findings for MPU 0.75

This file records differences found during the Rust port.
It uses the committed MPU 0.75 fixtures. It does not report live Perl results.

## Source limit

The task names an external draft at
`/home/kevin/mpu-fix/upstream-findings-draft.md`.
That file is absent in this checkout. The task describes three confirmed
findings, two probable findings, and six API or documentation differences.
Their text and evidence are not available. Those eleven entries cannot yet
be transcribed or checked. Do not treat the entries below as their replacement.
No upstream report has been sent by this task.

## How to read the entries

Each table row names a function, its input, the stored Perl 0.75 value,
and the test that skips it. Arguments are separated by spaces.
`undef` means that Perl returned no value. `ERROR` means that Perl raised an error.
The linked fixture is the source of each Perl value.

For a precision entry, no corrected mathematical value has been established.
The Perl value remains the reference value. The test comment attributes the
Rust difference to `f64` versus C `long double`. These are approximate
functions or bounds. A different last digit alone does not prove a Perl bug.

For an API entry, no correction to the Perl value is proposed.
The input or result is outside the Rust native API, or the overload is absent.
An overflow maps to `None` where the test states this.
These entries do not show an upstream defect.

## Precision entries

Verification: compare each named skip with its committed fixture row.
The skip tests are in [compat_count.rs](../tests/compat_count.rs) and
[compat_families.rs](../tests/compat_families.rs).
Correct value: not established; keep the Perl reference value shown below.

| Function | Input | Perl 0.75 value | Test that skips the row |
| --- | --- | --- | --- |
| `prime_count_lower` | `10000000000000` | `346065434524` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `100000000000000` | `3204941445272` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `1000000000000000` | `29844569644142` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `10000000000000000` | `279238338819876` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `100000000000000000` | `2623557149453649` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `1000000000000000000` | `24739954261435474` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `10000000000000000000` | `234057667231658186` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `1099511627775` | `41203054803` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `17592186044415` | `597116239543` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `281474976710655` | `8731188518651` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `4503599627370495` | `128625501343625` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `72057594037927935` | `1906879373936436` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `1152921504606846975` | `28423094467794862` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `18446744073709551615` | `425656283922083342` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `9223372036854775807` | `216289611801920544` | `compat_count_prime_count_lower` |
| `prime_count_lower` | `9223372036854775808` | `216289611801920544` | `compat_count_prime_count_lower` |
| `prime_count_upper` | `10000000000000` | `346065645810` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `100000000000000` | `3204942065691` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `1000000000000000` | `29844571475287` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `10000000000000000` | `279238344248556` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `100000000000000000` | `2623557165610822` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `1000000000000000000` | `24739954309690416` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `10000000000000000000` | `234057667376222368` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `1099511627775` | `41203130440` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `17592186044415` | `597116514592` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `281474976710655` | `8731189527167` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `4503599627370495` | `128625505067379` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `72057594037927935` | `1906879387767524` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `1152921504606846975` | `28423094519430928` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `9223372036854775807` | `216289611941014688` | `compat_count_prime_count_upper` |
| `prime_count_upper` | `9223372036854775808` | `216289611941014688` | `compat_count_prime_count_upper` |
| `prime_count_approx` | `10000000000000000` | `279238341360977` | `compat_count_prime_count_approx` |
| `prime_count_approx` | `100000000000000000` | `2623557157055978` | `compat_count_prime_count_approx` |
| `prime_count_approx` | `1000000000000000000` | `24739954284239494` | `compat_count_prime_count_approx` |
| `prime_count_approx` | `10000000000000000000` | `234057667300228940` | `compat_count_prime_count_approx` |
| `prime_count_approx` | `72057594037927935` | `1906879380439853` | `compat_count_prime_count_approx` |
| `prime_count_approx` | `1152921504606846975` | `28423094492202717` | `compat_count_prime_count_approx` |
| `prime_count_approx` | `18446744073709551615` | `425656284014012123` | `compat_count_prime_count_approx` |
| `prime_count_approx` | `9223372036854775807` | `216289611867888729` | `compat_count_prime_count_approx` |
| `prime_count_approx` | `9223372036854775808` | `216289611867888729` | `compat_count_prime_count_approx` |
| `almost_prime_count_approx` | `1 18446744073709551615` | `425656284014012123` | `almost_prime_count_approx_oracle` |
| `almost_prime_count_lower` | `1 18446744073709551615` | `425656283922083342` | `almost_prime_count_lower_oracle` |
| `ramanujan_prime_count_lower` | `18446744073709551615` | `141885427974027780` | `ramanujan_prime_count_lower_oracle` |
| `ramanujan_prime_count_approx` | `18446744073709551615` | `209366672146123394` | `ramanujan_prime_count_approx_oracle` |

Values above come from [count.tsv](../tests/fixtures/count.tsv) and
[families.tsv](../tests/fixtures/families.tsv).
The family comments name fixture lines 2624 and 2658 for the Ramanujan cases.
The almost-prime cases delegate to the prime-count functions.

## Native API entries named in comments

Verification: compare the named input with the fixture and the test guard.
Correct value: the Perl value shown remains valid for the Perl API.
The Rust API has no equivalent call for excluded inputs or missing overloads.
These rows also include named result-overflow cases. Some tests assert `None`
for these cases instead of skipping all checks.

| Function | Input | Perl 0.75 value | Test or helper that excludes the Perl comparison |
| --- | --- | --- | --- |
| `nth_prime_lower` | `425656284035217744` | `18446744073709551557` | `compat_count_nth_prime_lower / check_nth` |
| `nth_prime_upper` | `425656284035217744` | `18446794739597883110` | `compat_count_nth_prime_upper / check_nth` |
| `nth_prime_approx` | `425656284035217744` | `18446744070138417152` | `compat_count_nth_prime_approx / check_nth` |
| `nth_prime_lower` | `425656284035217745` | `18446744073709551557` | `compat_count_nth_prime_lower / check_nth` |
| `nth_prime_upper` | `425656284035217745` | `18446794739597883155` | `compat_count_nth_prime_upper / check_nth` |
| `nth_prime_approx` | `425656284035217745` | `18446744070138417152` | `compat_count_nth_prime_approx / check_nth` |
| `nth_semiprime_approx` | `1234` | `4489` | `nth_semiprime_approx_oracle` |
| `nth_semiprime_approx` | `12345` | `51052` | `nth_semiprime_approx_oracle` |
| `nth_semiprime_approx` | `123456` | `573476` | `nth_semiprime_approx_oracle` |
| `nth_semiprime_approx` | `2147483648` | `14540770648` | `nth_semiprime_approx_oracle` |
| `nth_semiprime_approx` | `4398046511104` | `36676113445988` | `nth_semiprime_approx_oracle` |
| `nth_semiprime_approx` | `100000000000000000` | `1030179406443727040` | `nth_semiprime_approx_oracle` |
| `nth_semiprime_approx` | `288230376151711744` | `3027432768851175168` | `nth_semiprime_approx_oracle` |
| `nth_semiprime_approx` | `1701748900850019776` | `18446744072129557504` | `nth_semiprime_approx_oracle` |
| `almost_prime_count` | `-2 100` | `ERROR` | `almost_prime_count_oracle` |
| `almost_prime_count_approx` | `-2 100` | `ERROR` | `almost_prime_count_approx_oracle` |
| `almost_prime_count_lower` | `-2 100` | `ERROR` | `almost_prime_count_lower_oracle` |
| `nth_almost_prime_upper` | `100 3` | `2852213850513516153367582212096` | `almost_omega_f03::compat_nth_almost_prime_upper` |
| `nth_almost_prime_upper` | `63 2011` | `4878683892660884733952` | `almost_omega_f03::compat_nth_almost_prime_upper` |
| `nth_almost_prime_upper` | `150 4557` | `1584220972135948492436838773688588967086179483648` | `almost_omega_f03::compat_nth_almost_prime_upper` |
| `nth_almost_prime_upper` | `-2 5` | `ERROR` | `almost_omega_f03::compat_nth_almost_prime_upper` |
| `omega_prime_count` | `-2 100` | `ERROR` | `almost_omega_f03::compat_omega_prime_count` |
| `nth_omega_prime` | `-2 5` | `ERROR` | `almost_omega_f03::compat_nth_omega_prime` |
| `next_prime_power` | `18446744073709551557` | `18446744073709551616` | `next_prime_power_oracle` |
| `next_prime_power` | `18446744073709551614` | `18446744073709551616` | `next_prime_power_oracle` |
| `next_prime_power` | `18446744073709551615` | `18446744073709551616` | `next_prime_power_oracle` |
| `prime_power_count` | `123456 133332` | `847` | `prime_power_count_oracle` |
| `prime_power_count` | `0 30` | `16` | `prime_power_count_oracle` |
| `prime_power_count` | `30 60` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `60 90` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `90 120` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `120 150` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `150 180` | `7` | `prime_power_count_oracle` |
| `prime_power_count` | `180 210` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `210 240` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `240 270` | `7` | `prime_power_count_oracle` |
| `prime_power_count` | `270 300` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `300 330` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `330 360` | `7` | `prime_power_count_oracle` |
| `prime_power_count` | `360 390` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `390 420` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `420 450` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `450 480` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `480 510` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `510 540` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `540 570` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `570 600` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `0 0` | `0` | `prime_power_count_oracle` |
| `prime_power_count` | `0 4` | `3` | `prime_power_count_oracle` |
| `prime_power_count` | `0 8` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `0 12` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `0 16` | `10` | `prime_power_count_oracle` |
| `prime_power_count` | `0 20` | `12` | `prime_power_count_oracle` |
| `prime_power_count` | `0 32` | `18` | `prime_power_count_oracle` |
| `prime_power_count` | `1 1` | `0` | `prime_power_count_oracle` |
| `prime_power_count` | `1 5` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `1 9` | `7` | `prime_power_count_oracle` |
| `prime_power_count` | `1 13` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `1 17` | `11` | `prime_power_count_oracle` |
| `prime_power_count` | `1 21` | `12` | `prime_power_count_oracle` |
| `prime_power_count` | `1 33` | `18` | `prime_power_count_oracle` |
| `prime_power_count` | `2 2` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `2 6` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `2 10` | `7` | `prime_power_count_oracle` |
| `prime_power_count` | `2 14` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `2 18` | `11` | `prime_power_count_oracle` |
| `prime_power_count` | `2 22` | `12` | `prime_power_count_oracle` |
| `prime_power_count` | `2 34` | `18` | `prime_power_count_oracle` |
| `prime_power_count` | `3 3` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `3 7` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `3 11` | `7` | `prime_power_count_oracle` |
| `prime_power_count` | `3 15` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `3 19` | `11` | `prime_power_count_oracle` |
| `prime_power_count` | `3 23` | `12` | `prime_power_count_oracle` |
| `prime_power_count` | `3 35` | `17` | `prime_power_count_oracle` |
| `prime_power_count` | `4 4` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `4 8` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `4 12` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `4 16` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `4 20` | `10` | `prime_power_count_oracle` |
| `prime_power_count` | `4 24` | `11` | `prime_power_count_oracle` |
| `prime_power_count` | `4 36` | `16` | `prime_power_count_oracle` |
| `prime_power_count` | `5 5` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `5 9` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `5 13` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `5 17` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `5 21` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `5 25` | `11` | `prime_power_count_oracle` |
| `prime_power_count` | `5 37` | `16` | `prime_power_count_oracle` |
| `prime_power_count` | `6 6` | `0` | `prime_power_count_oracle` |
| `prime_power_count` | `6 10` | `3` | `prime_power_count_oracle` |
| `prime_power_count` | `6 14` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `6 18` | `7` | `prime_power_count_oracle` |
| `prime_power_count` | `6 22` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `6 26` | `10` | `prime_power_count_oracle` |
| `prime_power_count` | `6 38` | `15` | `prime_power_count_oracle` |
| `prime_power_count` | `7 7` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `7 11` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `7 15` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `7 19` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `7 23` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `7 27` | `11` | `prime_power_count_oracle` |
| `prime_power_count` | `7 39` | `15` | `prime_power_count_oracle` |
| `prime_power_count` | `8 8` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `8 12` | `3` | `prime_power_count_oracle` |
| `prime_power_count` | `8 16` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `8 20` | `7` | `prime_power_count_oracle` |
| `prime_power_count` | `8 24` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `8 28` | `10` | `prime_power_count_oracle` |
| `prime_power_count` | `8 40` | `14` | `prime_power_count_oracle` |
| `prime_power_count` | `15 15` | `0` | `prime_power_count_oracle` |
| `prime_power_count` | `15 19` | `3` | `prime_power_count_oracle` |
| `prime_power_count` | `15 23` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `15 27` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `15 31` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `15 35` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `15 47` | `13` | `prime_power_count_oracle` |
| `prime_power_count` | `16 16` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `16 20` | `3` | `prime_power_count_oracle` |
| `prime_power_count` | `16 24` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `16 28` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `16 32` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `16 36` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `16 48` | `13` | `prime_power_count_oracle` |
| `prime_power_count` | `17 17` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `17 21` | `2` | `prime_power_count_oracle` |
| `prime_power_count` | `17 25` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `17 29` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `17 33` | `8` | `prime_power_count_oracle` |
| `prime_power_count` | `17 37` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `17 49` | `13` | `prime_power_count_oracle` |
| `prime_power_count` | `31 31` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `31 35` | `2` | `prime_power_count_oracle` |
| `prime_power_count` | `31 39` | `3` | `prime_power_count_oracle` |
| `prime_power_count` | `31 43` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `31 47` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `31 51` | `7` | `prime_power_count_oracle` |
| `prime_power_count` | `31 63` | `10` | `prime_power_count_oracle` |
| `prime_power_count` | `32 32` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `32 36` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `32 40` | `2` | `prime_power_count_oracle` |
| `prime_power_count` | `32 44` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `32 48` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `32 52` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `32 64` | `10` | `prime_power_count_oracle` |
| `prime_power_count` | `33 33` | `0` | `prime_power_count_oracle` |
| `prime_power_count` | `33 37` | `1` | `prime_power_count_oracle` |
| `prime_power_count` | `33 41` | `2` | `prime_power_count_oracle` |
| `prime_power_count` | `33 45` | `3` | `prime_power_count_oracle` |
| `prime_power_count` | `33 49` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `33 53` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `33 65` | `9` | `prime_power_count_oracle` |
| `prime_power_count` | `40 40` | `0` | `prime_power_count_oracle` |
| `prime_power_count` | `40 44` | `2` | `prime_power_count_oracle` |
| `prime_power_count` | `40 48` | `3` | `prime_power_count_oracle` |
| `prime_power_count` | `40 52` | `4` | `prime_power_count_oracle` |
| `prime_power_count` | `40 56` | `5` | `prime_power_count_oracle` |
| `prime_power_count` | `40 60` | `6` | `prime_power_count_oracle` |
| `prime_power_count` | `40 72` | `10` | `prime_power_count_oracle` |
| `is_perfect_power` | `-100` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-99` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-98` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-97` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-96` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-95` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-94` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-93` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-92` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-91` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-90` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-89` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-88` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-87` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-86` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-85` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-84` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-83` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-82` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-81` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-80` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-79` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-78` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-77` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-76` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-75` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-74` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-73` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-72` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-71` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-70` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-69` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-68` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-67` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-66` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-65` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-64` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-63` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-62` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-61` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-60` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-59` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-58` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-57` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-56` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-55` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-54` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-53` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-52` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-51` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-50` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-49` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-48` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-47` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-46` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-45` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-44` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-43` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-42` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-41` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-40` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-39` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-38` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-37` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-36` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-35` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-34` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-33` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-32` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-31` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-30` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-29` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-28` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-27` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-26` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-25` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-24` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-23` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-22` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-21` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-20` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-19` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-18` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-17` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-16` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-15` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-14` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-13` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-12` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-11` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-10` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-9` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-8` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-7` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-6` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-5` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-4` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-3` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-2` | `0` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-1` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-18446745128696702936` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-18446724184312856125` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-4298942376` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-4291015625` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-2147483648` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-2146689000` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `18446744073709551616` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-8922003266371364726` | `0` | `is_perfect_power_oracle` |
| `next_perfect_power` | `-100` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-99` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-98` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-97` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-96` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-95` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-94` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-93` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-92` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-91` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-90` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-89` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-88` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-87` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-86` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-85` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-84` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-83` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-82` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-81` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-80` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-79` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-78` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-77` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-76` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-75` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-74` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-73` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-72` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-71` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-70` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-69` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-68` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-67` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-66` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-65` | `-64` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-64` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-63` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-62` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-61` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-60` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-59` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-58` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-57` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-56` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-55` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-54` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-53` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-52` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-51` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-50` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-49` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-48` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-47` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-46` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-45` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-44` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-43` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-42` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-41` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-40` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-39` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-38` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-37` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-36` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-35` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-34` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-33` | `-32` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-32` | `-27` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-31` | `-27` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-30` | `-27` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-29` | `-27` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-28` | `-27` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-27` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-26` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-25` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-24` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-23` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-22` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-21` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-20` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-19` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-18` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-17` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-16` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-15` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-14` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-13` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-12` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-11` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-10` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-9` | `-8` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-8` | `-1` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-7` | `-1` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-6` | `-1` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-5` | `-1` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-4` | `-1` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-3` | `-1` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-2` | `-1` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-1` | `0` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-18446745128696702936` | `-18446724184312856125` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-18446724184312856125` | `-18446703239944862784` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-4298942376` | `-4291015625` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-4291015625` | `-4283098624` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-2147483648` | `-2146689000` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-2146689000` | `-2141700569` | `next_perfect_power_oracle` |
| `next_perfect_power` | `18446744073709551616` | `18446744082299486209` | `next_perfect_power_oracle` |
| `next_perfect_power` | `-8922003266371364726` | `-8921996796611463193` | `next_perfect_power_oracle` |
| `nth_perfect_power_approx` | `4297615582` | `18446744109147667342` | `nth_perfect_power_approx_oracle` |
| `nth_perfect_power_lower` | `4297615582` | `18446743767352942184` | `nth_perfect_power_lower_oracle` |
| `nth_perfect_power_upper` | `4297615582` | `18446744717534467451` | `nth_perfect_power_upper_oracle` |
| `is_powerful` | `-8 0` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 1` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 2` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 3` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 4` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 5` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 6` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 7` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 8` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 9` | `0` | `is_powerful_oracle` |
| `is_powerful` | `-8 10` | `0` | `is_powerful_oracle` |
| `powerful_count` | `-16` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-15` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-14` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-13` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-12` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-11` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-10` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-9` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-8` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-7` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-6` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-5` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-4` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-3` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-2` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-1` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-100` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-10` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-4` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-100 0` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-10 0` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-4 0` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-1 0` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-100 2` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-10 2` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-4 2` | `0` | `powerful_count_oracle` |
| `powerful_count` | `-1 2` | `0` | `powerful_count_oracle` |
| `sumpowerful` | `-16` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-15` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-14` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-13` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-12` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-11` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-10` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-9` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-8` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-7` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-6` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-5` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-4` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-3` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-2` | `0` | `sumpowerful_oracle` |
| `sumpowerful` | `-1` | `0` | `sumpowerful_oracle` |
| `powerful_numbers` | `1 121 -1` | `ERROR` | `powerful_numbers_oracle` |
| `minimal_goldbach_pair` | `83778272185315920949659591651127238812` | `5441` | `minimal_goldbach_pair_oracle` |
| `is_sum_of_squares` | `-5 2` | `1` | `is_sum_of_squares_oracle` |
| `partitions` | `500` | `2300165032574323995027` | `partitions_oracle` |
| `partitions` | `501` | `2431070104309287327876` | `partitions_oracle` |
| `partitions` | `1000` | `24061467864032622473692149727991` | `partitions_oracle` |
| `partitions` | `1001` | `25032297938763929621013218349796` | `partitions_oracle` |
| `partitions` | `2347` | `56751384003004060684283391440819878903446789803099` | `partitions_oracle` |
| `partitionsq` | `1000` | `8635565795744155161506` | `partitionsq_oracle` |
| `partitionsq` | `2000` | `106972734349914451123354464808960` | `partitionsq_oracle` |
| `partitionsq` | `-1` | `ERROR` | `partitionsq_oracle` |
| `partitions` | `-1` | `ERROR` | `partitions_oracle` |
| `sqrtmod` | `-1 17` | `4` | `roots::compat_sqrtmod` |
| `allsqrtmod` | `-1 17` | `4 13` | `roots::compat_allsqrtmod` |
| `rootmod` | `14 -3 101` | `17` | `roots::compat_rootmod` |
| `allrootmod` | `14 -3 101` | `17` | `roots::compat_allrootmod` |
| `rootmod` | `13 -6 107` | `49` | `roots::compat_rootmod` |
| `allrootmod` | `13 -6 107` | `49 58` | `roots::compat_allrootmod` |
| `rootmod` | `9 -2 101` | `34` | `roots::compat_rootmod` |
| `allrootmod` | `9 -2 101` | `34 67` | `roots::compat_allrootmod` |
| `ramanujan_tau` | `-5` | `0` | `ramanujan_tau` |
| `trial_factor` | `18446744073709551616 10foo` | `ERROR` | `native` |
| `forprimes` | `abc` | `ERROR` | `compat_iter / check` |
| `fordivisors` | `abc` | `ERROR` | `compat_iter / check` |
| `prime_iterator` | `18446744073709551557` | `18446744073709551557 18446744073709551629 18446744073709551653 18446744073709551667 18446744073709551697 18446744073709551709 18446744073709551757 18446744073709551923 18446744073709551947 18446744073709552009 18446744073709552109 18446744073709552157 18446744073709552213 18446744073709552253 18446744073709552267 18446744073709552333 18446744073709552357 18446744073709552361 18446744073709552373 18446744073709552421 18446744073709552423 18446744073709552501 18446744073709552541 18446744073709552577 18446744073709552597 18446744073709552613 18446744073709552621 18446744073709552697 18446744073709552729 18446744073709552859 18446744073709552901 18446744073709552957 18446744073709552969 18446744073709553023 18446744073709553029 18446744073709553033 18446744073709553099 18446744073709553137 18446744073709553171 18446744073709553291 18446744073709553401 18446744073709553411 18446744073709553447 18446744073709553501 18446744073709553533 18446744073709553543 18446744073709553561 18446744073709553597 18446744073709553653 18446744073709553681` | `compat_iter / check` |
| `prime_iterator_object_iterate` | `18446744073709551557` | `18446744073709551557 18446744073709551629 18446744073709551653 18446744073709551667 18446744073709551697 18446744073709551709 18446744073709551757 18446744073709551923 18446744073709551947 18446744073709552009 18446744073709552109 18446744073709552157 18446744073709552213 18446744073709552253 18446744073709552267 18446744073709552333 18446744073709552357 18446744073709552361 18446744073709552373 18446744073709552421 18446744073709552423 18446744073709552501 18446744073709552541 18446744073709552577 18446744073709552597 18446744073709552613 18446744073709552621 18446744073709552697 18446744073709552729 18446744073709552859 18446744073709552901 18446744073709552957 18446744073709552969 18446744073709553023 18446744073709553029 18446744073709553033 18446744073709553099 18446744073709553137 18446744073709553171 18446744073709553291 18446744073709553401 18446744073709553411 18446744073709553447 18446744073709553501 18446744073709553533 18446744073709553543 18446744073709553561 18446744073709553597 18446744073709553653 18446744073709553681` | `compat_iter / check` |
| `prime_iterator` | `18446744073709551615` | `18446744073709551629 18446744073709551653 18446744073709551667 18446744073709551697 18446744073709551709 18446744073709551757 18446744073709551923 18446744073709551947 18446744073709552009 18446744073709552109 18446744073709552157 18446744073709552213 18446744073709552253 18446744073709552267 18446744073709552333 18446744073709552357 18446744073709552361 18446744073709552373 18446744073709552421 18446744073709552423 18446744073709552501 18446744073709552541 18446744073709552577 18446744073709552597 18446744073709552613 18446744073709552621 18446744073709552697 18446744073709552729 18446744073709552859 18446744073709552901 18446744073709552957 18446744073709552969 18446744073709553023 18446744073709553029 18446744073709553033 18446744073709553099 18446744073709553137 18446744073709553171 18446744073709553291 18446744073709553401 18446744073709553411 18446744073709553447 18446744073709553501 18446744073709553533 18446744073709553543 18446744073709553561 18446744073709553597 18446744073709553653 18446744073709553681 18446744073709553687` | `compat_iter / check` |
| `prime_iterator_object_iterate` | `18446744073709551615` | `18446744073709551629 18446744073709551653 18446744073709551667 18446744073709551697 18446744073709551709 18446744073709551757 18446744073709551923 18446744073709551947 18446744073709552009 18446744073709552109 18446744073709552157 18446744073709552213 18446744073709552253 18446744073709552267 18446744073709552333 18446744073709552357 18446744073709552361 18446744073709552373 18446744073709552421 18446744073709552423 18446744073709552501 18446744073709552541 18446744073709552577 18446744073709552597 18446744073709552613 18446744073709552621 18446744073709552697 18446744073709552729 18446744073709552859 18446744073709552901 18446744073709552957 18446744073709552969 18446744073709553023 18446744073709553029 18446744073709553033 18446744073709553099 18446744073709553137 18446744073709553171 18446744073709553291 18446744073709553401 18446744073709553411 18446744073709553447 18446744073709553501 18446744073709553533 18446744073709553543 18446744073709553561 18446744073709553597 18446744073709553653 18446744073709553681 18446744073709553687` | `compat_iter / check` |
| `prime_iterator` | `100000000000000000000` | `100000000000000000039 100000000000000000129 100000000000000000151 100000000000000000193 100000000000000000207 100000000000000000301 100000000000000000349 100000000000000000361 100000000000000000391 100000000000000000393 100000000000000000441 100000000000000000477 100000000000000000547 100000000000000000559 100000000000000000561 100000000000000000721 100000000000000000741 100000000000000000753 100000000000000000757 100000000000000000763 100000000000000000801 100000000000000000853 100000000000000000961 100000000000000000993 100000000000000001071 100000000000000001107 100000000000000001119 100000000000000001141 100000000000000001197 100000000000000001221 100000000000000001243 100000000000000001261 100000000000000001267 100000000000000001303 100000000000000001323 100000000000000001387 100000000000000001521 100000000000000001537 100000000000000001567 100000000000000001597 100000000000000001633 100000000000000001663 100000000000000001677 100000000000000001821 100000000000000001827 100000000000000001893 100000000000000001921 100000000000000001977 100000000000000001983 100000000000000002013` | `compat_iter / check` |
| `prime_iterator_object_iterate` | `100000000000000000000` | `100000000000000000039 100000000000000000129 100000000000000000151 100000000000000000193 100000000000000000207 100000000000000000301 100000000000000000349 100000000000000000361 100000000000000000391 100000000000000000393 100000000000000000441 100000000000000000477 100000000000000000547 100000000000000000559 100000000000000000561 100000000000000000721 100000000000000000741 100000000000000000753 100000000000000000757 100000000000000000763 100000000000000000801 100000000000000000853 100000000000000000961 100000000000000000993 100000000000000001071 100000000000000001107 100000000000000001119 100000000000000001141 100000000000000001197 100000000000000001221 100000000000000001243 100000000000000001261 100000000000000001267 100000000000000001303 100000000000000001323 100000000000000001387 100000000000000001521 100000000000000001537 100000000000000001567 100000000000000001597 100000000000000001633 100000000000000001663 100000000000000001677 100000000000000001821 100000000000000001827 100000000000000001893 100000000000000001921 100000000000000001977 100000000000000001983 100000000000000002013` | `compat_iter / check` |
| `prime_iterator` | `-2` | `ERROR` | `compat_iter / check` |
| `prime_iterator_object_iterate` | `-2` | `ERROR` | `compat_iter / check` |
| `prime_iterator` | `abc` | `ERROR` | `compat_iter / check` |
| `prime_iterator_object_iterate` | `abc` | `ERROR` | `compat_iter / check` |
| `prime_iterator` | `4.5` | `ERROR` | `compat_iter / check` |
| `prime_iterator_object_iterate` | `4.5` | `ERROR` | `compat_iter / check` |

The other fixture sources are [combinat.tsv](../tests/fixtures/combinat.tsv),
[modular.tsv](../tests/fixtures/modular.tsv), [arith.tsv](../tests/fixtures/arith.tsv),
[factor.tsv](../tests/fixtures/factor.tsv), [iter.tsv](../tests/fixtures/iter.tsv),
and [random.tsv](../tests/fixtures/random.tsv).

### Rational API limits

[compat_rational.rs](../tests/compat_rational.rs), helper `each_native`,
excludes signed and bigint inputs and bigint results. Each public function
has a test named `<function>_oracle`.
It also excludes non-finite `bestrational` inputs, Farey fractions whose
reduced denominator exceeds the order, and tree indices above `u64`.
No correction to Perl is proposed for these rows.

Verification: [rational.c](../reference/mpu/rational.c), `calkin_wilf_n`,
returns failure if the index needs too many bits. `stern_brocot_n` uses that
index. `next_farey` returns failure when the reduced denominator exceeds
the order. The Perl wrapper can use fallback code in these cases.
[rational.tsv](../tests/fixtures/rational.tsv) records its results.

### Other native API limits

The compatibility tests also exclude inputs outside their integer types.
This includes signed modular residues, bigint factors and digit operations,
and results above the native range. The tests retain the Perl fixture values.
`is_divisible` varargs have no Rust overload. No correction to Perl is proposed.
See [compat_digits.rs](../tests/compat_digits.rs), helper `predicate_oracle`,
and [compat_modular.rs](../tests/compat_modular.rs), helpers `unsigned` and
`unsigned_args`.

## Checked areas

The check uses committed fixtures and independent Rust properties.
It does not regenerate the Perl oracle. It does not prove all possible inputs.

- Families: checked skip guards and the compatibility and property tests.
  No new upstream defect was established. Precision and API limits are above.
- Rational: checked the C guards, compatibility tests, continued-fraction
  round trips, tree indices, and Farey neighbor identities.
  No new upstream defect was established. API limits are above.
- Combinat: checked compatibility and property tests.
  Add this area to the checked-no-defect list for the tested native domain.
  Partition overflow and signed-input limits are above.

The property tests cover invalid inputs, repeated calls, and concurrent calls.
Examples are `native_overflow_and_validation`,
`repeated_and_concurrent_calls_are_independent`,
`prime_power_repeated_and_concurrent_calls`, and
`repeat_and_concurrent_partition_calls`.
This documentation change introduces no state or launch path.

## Remaining work

Recover the external draft. Add one entry for each of its eleven findings.
Each entry must name the function, input, Perl 0.75 value, corrected value,
verification method, and the test that skips the row.
Keep confirmed defects separate from probable defects and API differences.

