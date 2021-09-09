#!/bin/python3

import json
import os
import re
import sys
import toml


def upgrade_bsp(path, vers):
    '''Update the HAL version in the Cargo.toml file at the given path.'''
    start = -1
    with open(path, 'r') as f:
        lines = f.read().splitlines()
        for i, l in enumerate(lines):
            l = l.strip()
            if l == '[dependencies.atsamd-hal]':
                start = i
    assert lines[start+2].startswith('version = "')
    lines[start+2] = "version = \"%s\"" % vers
    with open(path, 'w') as f:
        f.write('\n'.join(lines) + '\n')


h = toml.load("hal/Cargo.toml")
hal_version = '.'.join(h['package']['version'].split('.')[:-1]) # Trim the patch.


with open('crates.json', 'r') as f:
    j = json.load(f)
    bsps = [b for b in j['boards'].keys() if j['boards'][b]['tier'] == 1]

for bsp in bsps:
    cargo = toml.load("boards/" + bsp + "/Cargo.toml")
    current = cargo['dependencies']['atsamd-hal']['version']
    if current != hal_version:
        print("Upgrading %s from %s to %s." % (bsp, current, hal_version))
        upgrade_bsp("boards/" + bsp + "/Cargo.toml", hal_version)
