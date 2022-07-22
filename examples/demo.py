#!/usr/bin/env python3
"""Exercise a release CLI against a disposable journal, never the user's live data."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, help="use an already-built CLI")
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[1]
    if args.binary is None:
        subprocess.run(["cargo", "build", "--locked", "--release", "--bin", "rustylife"], cwd=repo, check=True)
        metadata = json.loads(subprocess.check_output(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"], cwd=repo, text=True))
        binary = Path(metadata["target_directory"]) / "release" / ("rustylife.exe" if os.name == "nt" else "rustylife")
    else:
        binary = args.binary.resolve()
    with tempfile.TemporaryDirectory(prefix="rustylife-demo-") as directory:
        data = Path(directory) / "life.json"
        backup = Path(directory) / "backup.json"

        def run(*words, day="2022-04-26"):
            output = subprocess.run([str(binary), "--data", str(data), "--on", day, *words],
                                    check=True, capture_output=True, text=True)
            print("$ rustylife", " ".join(words).replace(str(backup), "<backup.json>"))
            print(output.stdout, end="")
            return output.stdout

        run("habit", "add", "Read", day="2022-04-25")
        run("habit", "category", "1", "Learning")
        run("habit", "check", "1", day="2022-04-25")
        run("habit", "check", "1")
        run("goal", "add", "Read twelve books", "12", "books", "2022-08-31", day="2022-04-25")
        run("goal", "advance", "1", "3")
        run("goal", "milestone", "1", "add", "Choose next book")
        run("goal", "milestone", "1", "done", "1")
        assert "1/1 habits completed" in run("today")
        assert "Total: 2/2 (100.0%)" in run("report", "weekly")
        run("report", "monthly")
        run("habit", "calendar", "1", "2022-04-20", "2022-04-26")
        assert "1,Read,2022-04-25,false" in run("export", "habits")
        assert "3,12,books,2022-08-31,active,1,1" in run("export", "goals")
        before = data.read_bytes()
        run("backup", str(backup))
        run("habit", "undo", "1")
        run("restore", str(backup), "--replace")
        assert data.read_bytes() == before
        assert "1/1 habits completed" in run("today")
        print("Demo verified: persistence, reports, exports and exact backup recovery.")


if __name__ == "__main__":
    main()
