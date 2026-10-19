#!/usr/bin/env python3
"""Regenerate STATUS.md and the BigUint coverage report from Rust public functions.

Port rows never edit STATUS.md (it would be a shared file of every row); the maintainer runs this after merges.
Usage: python3 -I tools/status.py [--check]   (--check: exit 1 when STATUS.md is stale, change nothing)
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PUB_FN = re.compile(r"^\s*pub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)
LINE = re.compile(r"^- \[( |x)\] `([^`]+)`(.*)$")
BIGINT_LINE = re.compile(r"^\s*pub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)

# Keep exclusions here for functions whose bigint forms are not planned.
REASONS = {
    "Requires a native machine word": {"rand_bits", "urandomr", "urandomm"},
}


def public_names():
    names = set()
    for path in (ROOT / "src").rglob("*.rs"):
        text = path.read_text()
        for m in PUB_FN.finditer(text):
            if "pub(crate)" in m.group(0):
                continue
            names.add(m.group(1))
    return names


def bigint_names():
    names = set()
    for path in (ROOT / "src" / "bigint").rglob("*.rs"):
        for match in BIGINT_LINE.finditer(path.read_text()):
            names.add(match.group(1))
    return names


def main():
    check = "--check" in sys.argv
    names = public_names()
    big_names = bigint_names()
    stale = [(group, name) for group, entries in REASONS.items() for name in sorted(entries & big_names)]
    for group, name in stale:
        print(f"warning: {name} has a BigUint form; removing stale entry from '{group}'")
    status = ROOT / "STATUS.md"
    out, ticked, changed = [], 0, 0
    for line in status.read_text().splitlines():
        m = LINE.match(line)
        if m:
            want = "x" if m.group(2) in names else " "
            if m.group(1) == "-":
                want = "-"
            if want != m.group(1):
                changed += 1
            line = f"- [{want}] `{m.group(2)}`{m.group(3)}"
            ticked += want == "x"
        out.append(line)
    text = "\n".join(out) + "\n"
    coverage = ROOT / "docs" / "bigint-coverage.md"
    coverage_lines = ["# BigUint coverage", "", "Names without a BigUint form are grouped by reason.", ""]
    for group, entries in REASONS.items():
        remaining = sorted(entries - big_names)
        if remaining:
            coverage_lines += [f"## {group}", ""] + [f"- `{name}`" for name in remaining] + [""]
    coverage_text = "\n".join(coverage_lines).rstrip() + "\n"
    coverage_changed = not coverage.exists() or coverage.read_text() != coverage_text
    if check:
        print(f"{ticked} ticked, {changed} stale line(s); coverage {'stale' if coverage_changed else 'current'}")
        return 1 if changed or coverage_changed else 0
    status.write_text(text)
    coverage.write_text(coverage_text)
    print(f"{ticked} ticked, {changed} line(s) changed; coverage {'updated' if coverage_changed else 'current'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
