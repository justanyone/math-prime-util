#!/usr/bin/env python3
"""Regenerate STATUS.md and docs/bigint-coverage.md from the code.

Port tick: a name is done when `pub fn <name>` exists under src/.
BigUint field: `yes` when `pub fn <name>` exists under src/bigint/ (feature `bigint`); every other name must be in
one of the NATIVE_ONLY groups below, which give the value (`no` or `n.a.`) and the reason. A name that gains a
BigUint form while it is still listed here, or a name that is in no group, makes the run report it and exit 1.

Port rows never edit STATUS.md (it would be a shared file of every row); the maintainer runs this after merges.
Usage: python3 -I tools/status.py [--check]   (--check: exit 1 when a file is stale, change nothing)
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BIGINT = ROOT / "src" / "bigint"
COVERAGE = ROOT / "docs" / "bigint-coverage.md"
PUB_FN = re.compile(r"^\s*pub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)
# `- [x] `name`` then the generated BigUint field, then any hand note (` — why`).
LINE = re.compile(r"^- \[( |x|-)\] `([^`]+)`(?: · bigint: (?:(?! — ).)*)?(.*)$")
SUMMARY = re.compile(r"^BigUint field: .*$")

NA = "n.a."
NO = "no"
# Names without a BigUint form, by reason. `n.a.`: a BigUint form does not apply. `no`: native only in this release.
NATIVE_ONLY = [
    ("integer operators", NA,
     "Plain integer arithmetic, comparison, shifts and parity on `i128`. num-bigint's operators "
     "(`+ - * / % << >>`, `pow`, comparison) and the num-integer `Integer` trait it implements "
     "(`div_floor`, `mod_floor`, `div_rem`, `div_ceil`, `is_even`, `is_odd`, `is_multiple_of`) do the same "
     "on `BigUint` and `BigInt`; a wrapper adds nothing.",
     "absint add1int addint cdivint cdivrem cmpint divint divrem fdivrem is_congruent is_divisible is_even "
     "is_odd lshiftint modint muladdint mulint mulsubint negint powint rashiftint rshiftint signint sub1int "
     "subint tdivrem"),
    ("bit operations", NA,
     "num-bigint's `&`, `|`, `^`, `bit`, `set_bit`, `count_ones`, `trailing_zeros` and `trailing_ones` do "
     "these on `BigUint`; `bitnot` needs a fixed word width, which `BigUint` does not have.",
     "bitand bitandnot bitclear bitflip bitnot bitor bitscan0 bitscan1 bitset bittest bitxor hammingweight"),
    ("generic over the element type", NA,
     "Generic over the element type `T`: these already take `BigUint` elements.",
     "formultiperm forsetproduct vecall vecany vecequal vecextract vecfirst vecfirstidx vecnone vecnotall "
     "vecpairwise vecreduce vecslide vecwindow"),
    ("combinatorial enumeration", NA,
     "Visit every combination, composition, derangement, partition or permutation. The number of objects, "
     "not the integer width, bounds `n` (sizes are `usize`); `lastfor` stops a loop and takes no integer.",
     "forcomb forcomp forderange forpart forperm lastfor"),
    ("real functions", NA,
     "Real-valued: an `f64` argument or result, so the integer backend does not apply (MPU's Math::BigFloat "
     "precision path is not ported).",
     "bernreal bestrational chebyshev_psi chebyshev_theta ExponentialIntegral harmreal inverse_li_nv LambertW "
     "LogarithmicIntegral RiemannR RiemannZeta"),
    ("already any size", NA,
     "Already unbounded: `Pi` returns as many digits as asked as a `String`, and `sumdigits` reads a digit "
     "string of any length.",
     "Pi sumdigits"),
    ("generator state", NA,
     "Seed, generator state and raw random output: fixed-width words, `f64`, bytes, or a shuffle or sample of "
     "the caller's slice, by definition.",
     "csrand drand entropy_bytes irand irand32 irand64 rand random_bytes randperm shuffle srand vecsample"),
    ("configuration", NA,
     "Process settings and the shared sieve cache; `prime_precalc` takes a sieve size held in memory.",
     "prime_get_config prime_memfree prime_precalc prime_set_config"),
    ("not in MPU 0.75", NA,
     "Not in Math::Prime::Util 0.75, so there is nothing to port.",
     "is_higgs_prime"),
    ("rejected past 2^63 by MPU", NA,
     "MPU 0.75 croaks (\"n must fit in native signed integer\") for `n` above 2^63-1, in its pure-Perl "
     "path as in C, so there is no big-integer result to port. The native forms return `None` "
     "(`hclassno`) or panic (`integer_complexity`) there; `tests/compat_bigint_oneint.rs` checks both "
     "against the oracle.",
     "hclassno integer_complexity"),
    ("impractical past 2^64", NA,
     "MPU 0.75 accepts a big `n`, but its pure-Perl path does not finish (each runs past a 30 s limit on a "
     "65-bit input). `farey` first takes the totient sum up to `n`, and the sequence has about 0.3n^2 "
     "terms. `is_catalan_pseudoprime` reduces to the full binomial(n-1, (n-1)/2), primes included. "
     "`is_pillai` steps v = 8, 9, ... through v! mod n, so a prime `n` takes more than 2^64 steps; only "
     "`n` below 23 or with a factor 2, 3, 5 or 7 returns at once (zero), which the native forms match at "
     "the 64-bit edge.",
     "farey is_catalan_pseudoprime is_pillai"),
    ("prime families: exact counts, nth values and lists", NO,
     "Past 2^64 each of these needs a sieve or a count up to the input or over a range in proportion to it, "
     "and MPU's pure-Perl path times out, runs out of memory or refuses the input there. "
     "`bigint::family_bounds` gives their bounds and estimates, `bigint::families` the next and previous "
     "members and the lists of short ranges, and `bigint::family_counts` the exact counts that are practical.",
     "nth_almost_prime nth_omega_prime nth_powerful nth_prime_power nth_semiprime powerful_numbers"),
    ("lucky, Ramanujan and twin-prime sieves", NA,
     "MPU 0.75 itself cannot answer these past 2^64: its XS code takes native integers only, and its "
     "pure-Perl path sieves from the start. `lucky_numbers`, `lucky_count` and `nth_lucky` sieve every lucky "
     "number up to about the argument, `nth_twin_prime` lists the twin primes from 2, and "
     "`is_ramanujan_prime`, `ramanujan_primes`, `ramanujan_prime_count` and `nth_ramanujan_prime` list the "
     "Ramanujan primes from 2. Measured on 2^64 and nearby (25 s, 4 GB): `nth_ramanujan_prime` croaks (float "
     "arithmetic on `Math::BigInt` gives NaN), `is_lucky` returns 0 at once only when its mod 2, 6 or 63 "
     "pretest rejects `n` and croaks the same way for other `n`, and the others run out of time or memory. The "
     "native forms answer to the `u64` limits of MPU's C code; `tests/compat_native_top_lucky.rs` checks "
     "`is_lucky`, `is_ramanujan_prime`, `nth_twin_prime` and `nth_ramanujan_prime` against MPU just below 2^64.",
     "is_lucky is_ramanujan_prime lucky_count lucky_numbers nth_lucky nth_ramanujan_prime nth_twin_prime "
     "ramanujan_prime_count ramanujan_primes"),
]


def pub_fns(path):
    for m in PUB_FN.finditer(path.read_text()):
        if "pub(crate)" not in m.group(0):
            yield m.group(1)


def public_names():
    return {name for path in (ROOT / "src").rglob("*.rs") for name in pub_fns(path)}


def bigint_forms():
    """MPU name -> the `bigint::` modules that hold a `pub fn` of that name."""
    forms = {}
    for path in sorted(BIGINT.rglob("*.rs")):
        rel = path.relative_to(BIGINT).with_suffix("")
        parts = [p for p in rel.parts if p != "mod"]
        module = "::".join(["bigint", *parts])
        for name in pub_fns(path):
            forms.setdefault(name, [])
            if module not in forms[name]:
                forms[name].append(module)
    return forms


def native_only():
    """MPU name -> (group title, value, reason); a name in two groups is an error."""
    table, dup = {}, []
    for title, value, reason, names in NATIVE_ONLY:
        for name in names.split():
            if name in table:
                dup.append(name)
            table[name] = (title, value, reason)
    return table, dup


def classify(names, forms, table):
    """Per name: (value, label, group). Also the names to fix: listed but with a form, or in no group."""
    out, stale, missing = {}, [], []
    for name in names:
        if name in forms:
            mods = " and ".join(f"`{m}`" for m in forms[name])
            out[name] = ("yes", mods, None)
            if name in table:
                stale.append(name)
        elif name in table:
            title, value, _ = table[name]
            out[name] = (value, title, title)
        else:
            out[name] = (NO, "not classified", None)
            missing.append(name)
    return out, stale, missing


def fixture_rows():
    """MPU name -> rows in tests/fixtures/bigint_*.tsv (the oracle replay of the BigUint forms)."""
    rows = {}
    for path in sorted((ROOT / "tests" / "fixtures").glob("bigint_*.tsv")):
        for line in path.read_text().splitlines():
            if line and not line.startswith("#"):
                name = line.split("\t", 1)[0]
                rows[name] = rows.get(name, 0) + 1
    return rows


def coverage_doc(names, cls):
    counts = {v: sum(1 for n in names if cls[n][0] == v) for v in ("yes", NO, NA)}
    lines = [
        "# BigUint coverage",
        "",
        "Generated by `python3 -I tools/status.py` from the source; do not edit by hand.",
        "",
        f"Of the {len(names)} exported names of Math::Prime::Util 0.75, **{counts['yes']}** have a `BigUint` form "
        f"under `math_prime_util::bigint` (cargo feature `bigint`), **{counts[NO]}** stay native only in this release "
        f"(`no`) and for **{counts[NA]}** a `BigUint` form does not apply (`n.a.`). The native `u64`/`u128` forms "
        "are there in every build. [STATUS.md](../STATUS.md) gives the same field per name.",
        "",
        "- `yes`: a `pub fn` of the MPU name exists under `src/bigint/` and takes `&BigUint` (or a range of "
        "`BigUint`).",
        "- `no`: native only in this release; the reason says what covers the gap, if anything.",
        "- `n.a.`: a `BigUint` form does not apply; the reason says why.",
        "",
        "## Names with a BigUint form",
        "",
        "The fixture column counts the rows of `tests/fixtures/bigint_*.tsv`: MPU 0.75's pure-Perl results on "
        "big inputs, replayed by `tests/compat_bigint_*.rs`. The `tests/prop_bigint_*.rs` property "
        "tests check the BigUint forms against the native forms on the `u64` range; the five iterators of "
        "`bigint::counts`, which have no fixture rows, are checked that way and by unit tests past 2^64.",
        "",
        "| Name | Module | Fixture rows |",
        "|---|---|---|",
    ]
    rows = fixture_rows()
    for name in sorted((n for n in names if cls[n][0] == "yes"), key=lambda n: (cls[n][1], n.lower())):
        lines.append(f"| `{name}` | {cls[name][1]} | {rows.get(name, 0)} |")
    lines += ["", "## Native-only names", "",
              "Each group gives the value, the reason, and the names. `n.a.` groups come first.", ""]
    for title, value, reason, group in sorted(NATIVE_ONLY, key=lambda g: g[1] != NA):
        members = [n for n in group.split() if cls.get(n, (None, None, None))[2] == title]
        if not members:
            continue
        lines += [f"### {title}", "", f"`{value}`, {len(members)} names. {reason}", "",
                  ", ".join(f"`{n}`" for n in members), ""]
    return "\n".join(lines).rstrip("\n") + "\n"


def bigint_names():
    names = set()
    for path in (ROOT / "src" / "bigint").rglob("*.rs"):
        for match in BIGINT_LINE.finditer(path.read_text()):
            names.add(match.group(1))
    return names


def main():
    check = "--check" in sys.argv
    names_pub = public_names()
    forms = bigint_forms()
    table, dup = native_only()
    status = ROOT / "STATUS.md"
    text_in = status.read_text()
    names = [m.group(2) for m in map(LINE.match, text_in.splitlines()) if m]
    cls, stale, missing = classify(names, forms, table)
    unknown = sorted(set(table) - set(names))
    out, ticked = [], 0
    counts = {v: sum(1 for n in names if cls[n][0] == v) for v in ("yes", NO, NA)}
    for line in text_in.splitlines():
        m = LINE.match(line)
        if m:
            want = "x" if m.group(2) in names_pub else " "
            if m.group(1) == "-":
                want = "-"
            value, label, _ = cls[m.group(2)]
            line = f"- [{want}] `{m.group(2)}` · bigint: {value}, {label}{m.group(3)}"
            ticked += want == "x"
        elif SUMMARY.match(line):
            line = (f"BigUint field: {counts['yes']} yes, {counts[NO]} no, {counts[NA]} n.a. "
                    "(reasons: [docs/bigint-coverage.md](docs/bigint-coverage.md)).")
        out.append(line)
    text = "\n".join(out) + "\n"
    doc = coverage_doc(names, cls)
    old_lines, new_lines = text_in.splitlines(), text.splitlines()
    changed = sum(a != b for a, b in zip(old_lines, new_lines)) + abs(len(old_lines) - len(new_lines))
    doc_stale = not COVERAGE.exists() or COVERAGE.read_text() != doc
    problems = []
    if stale:
        problems.append(f"have a BigUint form but are listed native-only in tools/status.py: {' '.join(stale)}")
    if missing:
        problems.append(f"have no BigUint form and no NATIVE_ONLY group in tools/status.py: {' '.join(missing)}")
    if dup:
        problems.append(f"are in two NATIVE_ONLY groups: {' '.join(dup)}")
    if unknown:
        problems.append(f"are in NATIVE_ONLY but not in STATUS.md: {' '.join(unknown)}")
    for p in problems:
        print(f"status.py: names that {p}", file=sys.stderr)
    summary = f"{ticked} ticked; bigint {counts['yes']} yes, {counts[NO]} no, {counts[NA]} n.a."
    if check:
        print(f"{summary}; {changed} stale line(s) in STATUS.md, docs/bigint-coverage.md "
              f"{'stale' if doc_stale else 'current'}")
        return 1 if changed or doc_stale or problems else 0
    status.write_text(text)
    COVERAGE.write_text(doc)
    print(f"{summary}; {changed} line(s) changed")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
