# math-prime-util

A Rust port of Perl's [Math::Prime::Util](https://metacpan.org/pod/Math::Prime::Util)
(MPU) 0.75: primality tests, prime sieves and counts, integer factoring, and the
number-theory toolbox around them (totients, Möbius, modular roots and logs,
Lucas sequences, continued fractions, combinatorics, random primes, and more).

Status: **pre-release, under construction.** See [PLAN.md](PLAN.md) for the
phases and [STATUS.md](STATUS.md) for the per-function checklist (428 names).

## Goals

- **Full compatibility.** Same function names and the same answers as MPU, on
  the native `u64`/`u128` path first and big integers later. Every MPU test
  file has a Rust counterpart that replays the oracle's output.
- **Performance.** MPU's C algorithms ported as they are, benchmarked against
  the C on the same machine; target within 1.2× of the C.
- **Exceptional test coverage.** Oracle fixtures, property tests, cross-checks
  against MPU's pure-Perl implementation and other Rust crates, a 95 % line
  coverage gate, mutation testing, and Miri on any `unsafe`.

## Branches

Work lands on a numbered integration branch (`dev-001`, `dev-002`, and so on). Small task branches are cut from it
and merged back by pull request. After every 100 merges the integration branch is merged to `main`, deleted, and the
next number starts from `main`. `main` carries releases only. The rotation is `tools/rotate_dev.py` in the SkyBuild kit.

## Milestones

Each line is ticked, with its date, by the milestone row of the build board when every requirement of that milestone is done.
The phases are described in `PLAN.md` section 6.

- [x] Milestone 1: core ported (sieve, primality, Lucas sequences, configuration) (done 2026-10-17)
- [x] Milestone 2: factoring and number theory ported and benchmarked against Perl (done 2026-10-17)
- [ ] Release 0.1.0 published on crates.io
- [ ] Milestone 3: prime counting, nth prime and prime sums ported
- [ ] Milestone 4: prime families ported
- [ ] Milestone 5: digits, rationals, combinatorics and inverse sigma ported
- [ ] Milestone 6: vector and set operations ported
- [ ] Milestone 7: iterators and the for-family ported
- [ ] Milestone 8: random primes and the random number generator ported
- [ ] Milestone 9: primality proving, real functions and the bigint backend ported
- [ ] Release 1.0.0 published on crates.io

## Minimum Rust version

1.85 (edition 2024).

## License

Same terms as MPU and Perl itself: GPL-1.0-or-later OR Artistic-1.0-Perl. See
[LICENSE](LICENSE). Math::Prime::Util is copyright Dana Jacobsen.
