#!/usr/bin/env python3

from tomlkit import parse, dumps
from pathlib import Path
import json
import sys

hal_path = Path("hal") / "Cargo.toml"
hal = parse(hal_path.read_text())
hal_version = hal["package"]["version"]

crates = Path("crates.json")
boards = json.loads(crates.read_text())["boards"]
board_names = [name for name, info in boards.items() if info["tier"] == 1]

for name in board_names:
    bsp_path = Path("boards") / name / "Cargo.toml"
    bsp = parse(bsp_path.read_text())
    bsp_version = bsp["dependencies"]["atsamd-hal"]["version"]
    if bsp_version != hal_version:
        print(f"Upgrading {name} from {bsp_version} to {hal_version}.", file=sys.stderr)
        bsp["dependencies"]["atsamd-hal"]["version"] = hal_version
        bsp_path.write_text(dumps(bsp))

