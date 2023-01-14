#!/usr/bin/env python3

from tomlkit import parse, dumps
from pathlib import Path
import sys

hal_path = Path("hal") / "Cargo.toml"
hal = parse(hal_path.read_text())

pacs = {}
for path in Path("pac").glob("atsam*/Cargo.toml"):
    name = path.parent.name
    pacs[name] = parse(path.read_text())

for name, pac in pacs.items():
    hal_version = hal["dependencies"][name]["version"]
    pac_version = pac["package"]["version"]
    if hal_version == pac_version:
        print(f"HAL dependency on {name} is up-to-date.", file=sys.stderr)
    else:
        hal["dependencies"][name]["version"] = pac_version
        msg = f"HAL dependency on {name} upgraded from {hal_version} to {pac_version}."
        print(msg, file=sys.stderr)

hal_path.write_text(dumps(hal))
