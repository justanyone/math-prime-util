#!/usr/bin/env python3
"""Writes src/bigint/proving/class_poly_data.rs: Hilbert class polynomials
H_D(x) for the ECPP downrun of src/bigint/proving.rs.

For each negative discriminant D (D = 0 or 1 mod 4, fundamental or not) the
primitive reduced forms (a, b, c) with b^2 - 4ac = D give the CM points
tau = (-b + sqrt(D)) / (2a), and H_D(x) = prod (x - j(tau)) has integer
coefficients. They are computed with mpmath at a precision checked by the
integrality of the result. The unit test `class_polynomials_give_curves`
checks every entry: a root mod a prime p with 4p = t^2 + |D| v^2 must give
a curve whose order is one of the candidates.

    python3 -I tools/class_polynomials.py            # default bound
    python3 -I tools/class_polynomials.py 6 1500     # class number, |D| bound
"""
import math
import sys
from pathlib import Path

import mpmath

MAX_H = int(sys.argv[1]) if len(sys.argv) > 1 else 6
MAX_ABS_D = int(sys.argv[2]) if len(sys.argv) > 2 else 1500


def reduced_forms(d):
    forms = []
    a = 1
    while 3 * a * a <= -d:
        for b in range(-a + 1, a + 1):
            if (b * b - d) % (4 * a):
                continue
            c = (b * b - d) // (4 * a)
            if c < a or (a == c and b < 0):
                continue
            if math.gcd(math.gcd(a, abs(b)), c) != 1:
                continue
            forms.append((a, b, c))
        a += 1
    return forms


def hilbert(d, forms):
    dps = int(1.5 * math.sqrt(-d) * len(forms)) + 40
    while True:
        mpmath.mp.dps = dps
        roots = []
        for a, b, _ in forms:
            tau = (-b + mpmath.sqrt(mpmath.mpf(d))) / (2 * a)
            roots.append(1728 * mpmath.kleinj(tau))
        poly = [mpmath.mpc(1)]
        for r in roots:
            new = [mpmath.mpc(0)] * (len(poly) + 1)
            for i, c in enumerate(poly):
                new[i + 1] += c
                new[i] -= c * r
            poly = new
        ints = []
        ok = True
        for c in poly:
            re = mpmath.nint(c.real)
            if abs(c.real - re) > mpmath.mpf(10) ** -8 or abs(c.imag) > mpmath.mpf(10) ** -8:
                ok = False
                break
            ints.append(int(re))
        if ok:
            return ints  # low to high, monic
        dps *= 2


def main():
    entries = []
    for d in range(-3, -MAX_ABS_D - 1, -1):
        if d % 4 not in (0, 1):
            continue
        forms = reduced_forms(d)
        if not forms or len(forms) > MAX_H:
            continue
        coeffs = hilbert(d, forms)
        assert coeffs[-1] == 1
        entries.append((d, coeffs[:-1]))
    out = Path(__file__).resolve().parent.parent / "src" / "bigint" / "proving" / "class_poly_data.rs"
    lines = [
        "//! Hilbert class polynomials for the ECPP downrun, written by",
        "//! `tools/class_polynomials.py`; do not edit.",
        "//!",
        "//! `(D, [c0, c1, ..., c(h-1)])` stands for `x^h + c(h-1) x^(h-1) + ... + c0`,",
        f"//! for every discriminant with class number at most {MAX_H} and `|D|` at",
        f"//! most {MAX_ABS_D}: {len(entries)} entries.",
        "",
        "/// The class polynomials, smallest `|D|` first.",
        "#[rustfmt::skip]",
        "pub const CLASS_POLYNOMIALS: &[(i64, &[&str])] = &[",
    ]
    for d, coeffs in entries:
        body = ", ".join(f'"{c}"' for c in coeffs)
        lines.append(f"    ({d}, &[{body}]),")
    lines.append("];")
    out.parent.mkdir(exist_ok=True)
    out.write_text("\n".join(lines) + "\n")
    size = out.stat().st_size
    by_h = {}
    for _, coeffs in entries:
        by_h[len(coeffs)] = by_h.get(len(coeffs), 0) + 1
    print(f"wrote {out} ({size} bytes, {len(entries)} discriminants, by class number {sorted(by_h.items())})")


if __name__ == "__main__":
    main()
