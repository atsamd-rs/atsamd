#!/usr/bin/env python3

import json
import shlex
import subprocess
from pathlib import PurePath


def main(clean=False, just_clean=False):
    with open("crates.json", "r") as f:
        crates = json.load(f)

    for (board, jobs) in crates["boards"].items():
        print()
        print("-" * 80)
        print(f"Crate:   {board}")
        print(f"Command: {jobs['build']}\n")

        command = shlex.split(jobs["build"])
        crate_path = PurePath("boards", board)

        if clean or just_clean:
            subprocess.check_call(shlex.split("cargo clean"), cwd=crate_path)
            subprocess.check_call(shlex.split("cargo update"), cwd=crate_path)
        if not just_clean:
            subprocess.check_call(command, cwd=crate_path)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--clean", action="store_true", help="clean the crate before building")
    parser.add_argument("--just-clean", action="store_true", help="Only clean the crate and delete Cargo.lock")

    args = parser.parse_args()

    main(clean=args.clean, just_clean=args.just_clean)
