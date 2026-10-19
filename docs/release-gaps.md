# Release gaps: README and docs before the next crates.io release

Scan of `dev-002` at `2f2ada44` (2026-10-18). Version 1.0.0 is on crates.io
(published from `main` at `b0944acf`). This list is for the next release
(1.1.0). It changes no code and no manifest field. The owner publishes.

## Release commit (owner step)

1. `Cargo.toml` has `publish = false` and `version = "1.0.0"`. The release
   commit removes `publish = false` and sets the new version. `main` already
   has the line removed (PR 124); `dev-002` does not.
2. `README.md` line 68, "Release 1.0.0 published on crates.io", is not ticked
   on `dev-002`. `main` has it ticked (PR 124). Merge `origin/main` into the
   integration branch before the release commit, or tick the line there.
3. There is no `CHANGELOG.md`. Rust crates usually keep one (Keep a Changelog
   format). Add entries for 1.0.0 and the new release.

## README (the crates.io page)

4. The milestone list contradicts the status line. Line 8 says all 428 names
   are ported. Lines 60, 61 and 63 to 67 are not ticked. Line 60 (release
   0.1.0) is closed as superseded by 1.0.0 (row `MPU-MS-PUBLISH-010`). The
   board writes these ticks; let the milestone rows close, or replace the
   list with one status line for the release.
5. There is no usage example. Add a short block, for example `is_prime`,
   `factor` and `nth_prime` from `math_prime_util`.
6. Line 8 says "version 1.0.0" and line 10 names `dev-002`. Update both in
   the release commit. The branch name is repository detail; it can move
   to the Branches section.
7. Line 126 links `Int` to `src/bigint/mod.rs`, a source file. Link the API
   page instead:
   `https://docs.rs/math-prime-util/latest/math_prime_util/trait.Int.html`.
8. Lines 82 to 85 (feature `gmp`, planned) and lines 100 to 102 ("the rest
   of the bigint path is not ported yet") depend on the bigint chain
   (`MPU-BIGINT-*`). Update them when that chain merges.
9. No badges (crates.io version, docs.rs, CI, license). This is optional.

## Crate docs (the docs.rs front page)

10. `src/lib.rs` line 20 says "See `PLAN.md` and `STATUS.md` in the
    repository for what is ported so far." This is out of date for 1.x.
11. The crate docs have no runnable example, no list of cargo features
    (`bigint`) and no module overview. The Rust API Guidelines (C-CRATE-DOC)
    ask for these.
12. `Cargo.toml` has no `[package.metadata.docs.rs]` table. docs.rs then
    builds the default features only, so the `bigint` items (the `BigUint`
    implementation of `Int`) are not on docs.rs. Add `all-features = true`.
    `src/lib.rs` already enables `feature(doc_cfg)` under `docsrs`.
13. Two public functions have no runnable example in their doc comment:
    `is_power_root` (`src/int/mod.rs` line 350) and `sqrmod`
    (`src/modular/mod.rs` line 74). The other 456 `pub fn` items have one.
14. `src/lib.rs` keeps 18 `#[allow(unused_imports)] // until the module has
    public items` lines. Every module has public items now. Remove them
    (code cleanup, no API change).

## Package contents

15. `cargo package --list` gives 365 files: `src` 273, `tests` 59 (with the
    17 fixture files, 2.8 MB before compression), `benches` 19, `docs` 2,
    and `PLAN.md`, `STATUS.md`, `NOTICE`, `LICENSE`, `deny.toml`,
    `rust-toolchain.toml`. The packaged tests read only `tests/fixtures`;
    the one test that runs `tools/` is `#[ignore]`. No change is necessary.
    `deny.toml` and `rust-toolchain.toml` can be excluded (optional).
16. There is no `examples/` directory. This is optional.

## Checked, no gap

- `description`, `license` (SPDX), `repository`, `documentation`, `readme`,
  5 keywords and 2 valid categories are set.
- `rust-version = "1.85"` agrees with the README; `tools/gate.sh` builds
  with 1.85.0.
- `NOTICE` and the README link `docs/upstream-findings.md`.
