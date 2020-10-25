#!/usr/bin/env python

import json
import shlex
import subprocess
from pathlib import PurePath


with open("crates.json", "r") as f:
    crates = json.load(f)

for (board, jobs) in crates["boards"].items():
    print()
    print("-" * 80)
    print(f"Crate:   {board}")
    print(f"Command: {jobs['build']}\n")

    command = shlex.split(jobs["build"])
    crate_path = PurePath("boards", board)

    subprocess.check_call(command, cwd=crate_path)
