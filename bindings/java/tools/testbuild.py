#!/usr/bin/env python3

from pathlib import Path
import subprocess

if __name__ == "__main__":
    basedir = Path(__file__).parent.parent

    output = basedir / "target"
    Path(output).mkdir(exist_ok=True, parents=True)
    cmd = ["cargo", "build", "--target-dir", str(output)]
    print("$ " + subprocess.list2cmdline(cmd))
    subprocess.run(cmd, cwd=basedir, check=True)