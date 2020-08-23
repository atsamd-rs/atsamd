#!/usr/bin/env python

import os
import yaml
import shlex
import subprocess

travis = yaml.safe_load(open('.travis.yml', 'r'))

for env in travis['env']:
	words = shlex.split(env)

	d = {}
	for word in words:
		key_value = word.split('=', 1)
		d[key_value[0]] = key_value[1]

	crate_dir = d["CRATE"]

	cmd = [
	    'cargo',
	    'build',
	]

	examples = d.get("EXAMPLES", None)
	if examples != None:
		cmd.extend(shlex.split(examples))

	features = d.get("FEATURES", None)
	if features != None:
		cmd.extend(shlex.split(features))

	buildmode = d.get("BUILDMODE", None)
	if buildmode != None:
		cmd.extend(shlex.split(buildmode))

	print(cmd)
	
	subprocess.check_call(cmd, cwd=crate_dir)
