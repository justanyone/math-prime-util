"""Regression tests for generated bigint status."""
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent


class StatusGeneratorTest(unittest.TestCase):
    def test_new_biguint_form_overrides_reason_and_check_detects_staleness(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            shutil.copy(ROOT / "tools" / "status.py", root / "tools_status.py")
            (root / "src" / "bigint").mkdir(parents=True)
            (root / "src" / "bigint" / "one.rs").write_text("pub fn test_only_name() {}\n")
            (root / "src" / "native.rs").write_text("pub fn test_only_name() {}\n")
            (root / "docs").mkdir()
            (root / "STATUS.md").write_text("- [ ] `test_only_name`\n")
            script = root / "tools_status.py"
            source = script.read_text().replace(
                'ROOT = Path(__file__).resolve().parent.parent', f'ROOT = Path({str(root)!r})'
            ).replace('\nif __name__ == "__main__":',
                      '\nNATIVE_ONLY = [("test group", NO, "why", "test_only_name")]\nif __name__ == "__main__":')
            script.write_text(source)
            run = subprocess.run(["python3", "-I", str(script)], text=True, capture_output=True, check=False)
            self.assertEqual(run.returncode, 0, run.stderr)
            self.assertEqual(run.stdout.count("warning:"), 1)
            self.assertIn("[x] `test_only_name`", (root / "STATUS.md").read_text())
            doc = (root / "docs" / "bigint-coverage.md").read_text()
            self.assertIn("| `test_only_name` |", doc)
            self.assertNotIn("test group", doc)
            current = subprocess.run(["python3", "-I", str(script), "--check"], capture_output=True)
            self.assertEqual(current.returncode, 0)
            (root / "docs" / "bigint-coverage.md").write_text("stale\n")
            stale = subprocess.run(["python3", "-I", str(script), "--check"], capture_output=True)
            self.assertEqual(stale.returncode, 1)


if __name__ == "__main__":
    unittest.main()
