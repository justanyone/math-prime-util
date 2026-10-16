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

## Minimum Rust version

1.85 (edition 2024).

## License

Same terms as MPU and Perl itself: GPL-1.0-or-later OR Artistic-1.0-Perl. See
[LICENSE](LICENSE). Math::Prime::Util is copyright Dana Jacobsen.
