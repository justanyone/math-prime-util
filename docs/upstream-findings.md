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
| `is_perfect_power` | `18446744073709551616` | `1` | `is_perfect_power_oracle` |
| `is_perfect_power` | `-8922003266371364726` | `0` | `is_perfect_power_oracle` |
| `next_perfect_power` | `18446744073709551616` | `18446744082299486209` | `next_perfect_power_oracle` |
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
| `trial_factor` | `18446744073709551616 10foo` | `ERROR` | `trial_factor_oracle` |
| `forprimes` | `abc` | `ERROR` | `forprimes` |
| `fordivisors` | `abc` | `ERROR` | `fordivisors` |
| `prime_iterator` | `18446744073709551557` | `18446744073709551557 18446744073709551629 18446744073709551653 18446744073709551667 18446744073709551697 18446744073709551709 18446744073709551757 18446744073709551923 18446744073709551947 18446744073709552009 18446744073709552109 18446744073709552157 18446744073709552213 18446744073709552253 18446744073709552267 18446744073709552333 18446744073709552357 18446744073709552361 18446744073709552373 18446744073709552421 18446744073709552423 18446744073709552501 18446744073709552541 18446744073709552577 18446744073709552597 18446744073709552613 18446744073709552621 18446744073709552697 18446744073709552729 18446744073709552859 18446744073709552901 18446744073709552957 18446744073709552969 18446744073709553023 18446744073709553029 18446744073709553033 18446744073709553099 18446744073709553137 18446744073709553171 18446744073709553291 18446744073709553401 18446744073709553411 18446744073709553447 18446744073709553501 18446744073709553533 18446744073709553543 18446744073709553561 18446744073709553597 18446744073709553653 18446744073709553681` | `prime_iterator` |
| `prime_iterator_object_iterate` | `18446744073709551557` | `18446744073709551557 18446744073709551629 18446744073709551653 18446744073709551667 18446744073709551697 18446744073709551709 18446744073709551757 18446744073709551923 18446744073709551947 18446744073709552009 18446744073709552109 18446744073709552157 18446744073709552213 18446744073709552253 18446744073709552267 18446744073709552333 18446744073709552357 18446744073709552361 18446744073709552373 18446744073709552421 18446744073709552423 18446744073709552501 18446744073709552541 18446744073709552577 18446744073709552597 18446744073709552613 18446744073709552621 18446744073709552697 18446744073709552729 18446744073709552859 18446744073709552901 18446744073709552957 18446744073709552969 18446744073709553023 18446744073709553029 18446744073709553033 18446744073709553099 18446744073709553137 18446744073709553171 18446744073709553291 18446744073709553401 18446744073709553411 18446744073709553447 18446744073709553501 18446744073709553533 18446744073709553543 18446744073709553561 18446744073709553597 18446744073709553653 18446744073709553681` | `prime_iterator_object` |
| `prime_iterator` | `18446744073709551615` | `18446744073709551629 18446744073709551653 18446744073709551667 18446744073709551697 18446744073709551709 18446744073709551757 18446744073709551923 18446744073709551947 18446744073709552009 18446744073709552109 18446744073709552157 18446744073709552213 18446744073709552253 18446744073709552267 18446744073709552333 18446744073709552357 18446744073709552361 18446744073709552373 18446744073709552421 18446744073709552423 18446744073709552501 18446744073709552541 18446744073709552577 18446744073709552597 18446744073709552613 18446744073709552621 18446744073709552697 18446744073709552729 18446744073709552859 18446744073709552901 18446744073709552957 18446744073709552969 18446744073709553023 18446744073709553029 18446744073709553033 18446744073709553099 18446744073709553137 18446744073709553171 18446744073709553291 18446744073709553401 18446744073709553411 18446744073709553447 18446744073709553501 18446744073709553533 18446744073709553543 18446744073709553561 18446744073709553597 18446744073709553653 18446744073709553681 18446744073709553687` | `prime_iterator` |
| `prime_iterator_object_iterate` | `18446744073709551615` | `18446744073709551629 18446744073709551653 18446744073709551667 18446744073709551697 18446744073709551709 18446744073709551757 18446744073709551923 18446744073709551947 18446744073709552009 18446744073709552109 18446744073709552157 18446744073709552213 18446744073709552253 18446744073709552267 18446744073709552333 18446744073709552357 18446744073709552361 18446744073709552373 18446744073709552421 18446744073709552423 18446744073709552501 18446744073709552541 18446744073709552577 18446744073709552597 18446744073709552613 18446744073709552621 18446744073709552697 18446744073709552729 18446744073709552859 18446744073709552901 18446744073709552957 18446744073709552969 18446744073709553023 18446744073709553029 18446744073709553033 18446744073709553099 18446744073709553137 18446744073709553171 18446744073709553291 18446744073709553401 18446744073709553411 18446744073709553447 18446744073709553501 18446744073709553533 18446744073709553543 18446744073709553561 18446744073709553597 18446744073709553653 18446744073709553681 18446744073709553687` | `prime_iterator_object` |
| `prime_iterator` | `100000000000000000000` | `100000000000000000039 100000000000000000129 100000000000000000151 100000000000000000193 100000000000000000207 100000000000000000301 100000000000000000349 100000000000000000361 100000000000000000391 100000000000000000393 100000000000000000441 100000000000000000477 100000000000000000547 100000000000000000559 100000000000000000561 100000000000000000721 100000000000000000741 100000000000000000753 100000000000000000757 100000000000000000763 100000000000000000801 100000000000000000853 100000000000000000961 100000000000000000993 100000000000000001071 100000000000000001107 100000000000000001119 100000000000000001141 100000000000000001197 100000000000000001221 100000000000000001243 100000000000000001261 100000000000000001267 100000000000000001303 100000000000000001323 100000000000000001387 100000000000000001521 100000000000000001537 100000000000000001567 100000000000000001597 100000000000000001633 100000000000000001663 100000000000000001677 100000000000000001821 100000000000000001827 100000000000000001893 100000000000000001921 100000000000000001977 100000000000000001983 100000000000000002013` | `prime_iterator` |
| `prime_iterator_object_iterate` | `100000000000000000000` | `100000000000000000039 100000000000000000129 100000000000000000151 100000000000000000193 100000000000000000207 100000000000000000301 100000000000000000349 100000000000000000361 100000000000000000391 100000000000000000393 100000000000000000441 100000000000000000477 100000000000000000547 100000000000000000559 100000000000000000561 100000000000000000721 100000000000000000741 100000000000000000753 100000000000000000757 100000000000000000763 100000000000000000801 100000000000000000853 100000000000000000961 100000000000000000993 100000000000000001071 100000000000000001107 100000000000000001119 100000000000000001141 100000000000000001197 100000000000000001221 100000000000000001243 100000000000000001261 100000000000000001267 100000000000000001303 100000000000000001323 100000000000000001387 100000000000000001521 100000000000000001537 100000000000000001567 100000000000000001597 100000000000000001633 100000000000000001663 100000000000000001677 100000000000000001821 100000000000000001827 100000000000000001893 100000000000000001921 100000000000000001977 100000000000000001983 100000000000000002013` | `prime_iterator_object` |
| `prime_iterator` | `-2` | `ERROR` | `prime_iterator` |
| `prime_iterator_object_iterate` | `-2` | `ERROR` | `prime_iterator_object` |
| `prime_iterator` | `abc` | `ERROR` | `prime_iterator` |
| `prime_iterator_object_iterate` | `abc` | `ERROR` | `prime_iterator_object` |
| `prime_iterator` | `4.5` | `ERROR` | `prime_iterator` |
| `prime_iterator_object_iterate` | `4.5` | `ERROR` | `prime_iterator_object` |

The other fixture sources are [combinat.tsv](../tests/fixtures/combinat.tsv),
[modular.tsv](../tests/fixtures/modular.tsv), [arith.tsv](../tests/fixtures/arith.tsv),
[factor.tsv](../tests/fixtures/factor.tsv), [iter.tsv](../tests/fixtures/iter.tsv),
and [random.tsv](../tests/fixtures/random.tsv).

### Prime-power range overloads

`prime_power_count`, `prime_power_count_approx`, `prime_power_count_lower`,
and `prime_power_count_upper` also skip their two-argument range overloads.
The Rust API takes one bound. The Perl values need no correction.
Verification: the `<function>_oracle` tests reject rows with two arguments;
[families.tsv](../tests/fixtures/families.tsv) preserves every range and value.

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

The random test comment names `random_prime` inputs `-3`, `(2, -4)`,
`(2, a)`, `a`, and `undef`. The native API cannot express these inputs.
The test is `random_prime_oracle` in
[compat_random.rs](../tests/compat_random.rs).
Iterator comments name invalid inputs `-2`, `abc`, and `4.5`.
Their Perl result is an error. Tests in
[compat_iter.rs](../tests/compat_iter.rs) use the iterator name as the test name.
`prime_iterator_object` checks the object rows.


The compatibility tests also exclude inputs outside their integer types.
This includes signed modular residues, bigint factors and digit operations,
and results above the native range. The tests retain the Perl fixture values.
`is_divisible` varargs have no Rust overload. No correction to Perl is proposed.
See [compat_digits.rs](../tests/compat_digits.rs), helper `predicate_oracle`,
and [compat_modular.rs](../tests/compat_modular.rs), helpers `unsigned` and
`unsigned_args`.

## Checked areas: no new defect found

Offline validation passed for `compat_families`, `compat_rational`,
`compat_combinat`, `prop_families`, `prop_rational`, and `prop_combinat`.


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

