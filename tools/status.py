#!/usr/bin/env python3
"""Regenerate the ticks of STATUS.md from the code: a name is done when `pub fn <name>` exists under src/.

Port rows never edit STATUS.md (it would be a shared file of every row); the maintainer runs this after merges.
Usage: python3 -I tools/status.py [--check]   (--check: exit 1 when STATUS.md is stale, change nothing)
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PUB_FN = re.compile(r"^\s*pub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)
LINE = re.compile(r"^- \[( |x)\] `([^`]+)`(.*)$")


def public_names():
    names = set()
    for path in (ROOT / "src").rglob("*.rs"):
        text = path.read_text()
        for m in PUB_FN.finditer(text):
            if "pub(crate)" in m.group(0):
                continue
            names.add(m.group(1))
    return names


def main():
    check = "--check" in sys.argv
    names = public_names()
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
    if check:
        print(f"{ticked} ticked, {changed} stale line(s)")
        return 1 if changed else 0
    status.write_text(text)
    print(f"{ticked} ticked, {changed} line(s) changed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
