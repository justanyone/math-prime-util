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
            (root / "src" / "bigint" / "one.rs").write_text("pub fn absint() {}\n")
            (root / "src" / "native.rs").write_text("pub fn absint() {}\n")
            (root / "docs").mkdir()
            (root / "STATUS.md").write_text("- [ ] `absint`\n")
            script = root / "tools_status.py"
            script.write_text(script.read_text().replace(
                'ROOT = Path(__file__).resolve().parent.parent', f'ROOT = Path({str(root)!r})'
            ))
            run = subprocess.run(["python3", "-I", str(script)], text=True, capture_output=True, check=False)
            self.assertEqual(run.returncode, 1)
            self.assertIn("listed native-only in tools/status.py: absint", run.stderr)
            self.assertIn("[x] `absint` · bigint: yes", (root / "STATUS.md").read_text())

    def test_repository_status_is_current(self):
        run = subprocess.run(["python3", "-I", str(ROOT / "tools" / "status.py"), "--check"],
                             text=True, capture_output=True, check=False)
        self.assertEqual(run.returncode, 0, run.stderr + run.stdout)


if __name__ == "__main__":
    unittest.main()
