#!/bin/python3

import os
import re
import sys
import toml

class PacDep(object):
    def __init__(self, name, hal_info, pac_info):
        self._name = name
        h = toml.load("hal/Cargo.toml")
        # Trim the patch from the versions.
        self._hal_version = '.'.join(hal_info['version'].split('.')[:-1])
        self._pac_version = '.'.join(pac_info['package']['version'].split('.')[:-1])
        self._line = None

    @property
    def hal_version(self):
        return self._hal_version

    @property
    def pac_version(self):
        return self._pac_version

    @property
    def line(self):
        return self._line

    @line.setter
    def line(self, value):
        self._line = value

# Populate a dictionary containing the different PACs and their version.
pacs = dict()
h = toml.load("hal/Cargo.toml")
for dep, info in h['dependencies'].items():
    is_pac = os.path.isdir("pac/" + dep)
    if is_pac:
        pacs[dep] = PacDep(dep, info, toml.load("pac/" + dep + "/Cargo.toml"))

# Collect version and line information for each PAC.
with open('hal/Cargo.toml', 'r') as f:
    lines = f.read().splitlines()
    for i, l in enumerate(lines):
        l = l.strip()
        if l.startswith('[dependencies.') and l.endswith(']'):
            crate = re.search(r'(\[dependencies)\.(.+)]', l).group(2)
            if crate in pacs:
                pacs[crate].line = i

# Update our line array
for crate, p in pacs.items():
    if p.hal_version != p.pac_version:
        assert lines[p.line+2].startswith('version = "')
        lines[p.line+2] = "version = \"%s\"" % p.pac_version
        print("HAL dependency on %s upgraded from %s to %s." %
                (crate, p.hal_version, p.pac_version),
                file=sys.stderr)
    else:
        print("HAL dependency on %s is up-to-date." % crate, file=sys.stderr)

with open('hal/Cargo.toml', 'w') as f:
    f.write('\n'.join(lines) + '\n')
