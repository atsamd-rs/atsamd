#!/usr/bin/env python

import os
import shlex
import subprocess
import yaml


travis_config = yaml.safe_load(open(".travis.yml", "r"))

for config in travis_config["env"]:
    # build a dict mapping environment variables to their respective values
    env = {}
    for var in shlex.split(config):
        (key, value) = var.split("=", 1)
        env[key] = value

    # for each of the supported environment variables, if a value has been
    # provided then extend the command accordingly
    cmd = ["cargo", "build"]

    variables = ["EXAMPLES", "FEATURES", "BUILDMODE"]
    for var in variables:
        value = env.get(var, None)
        if value is not None:
            cmd.extend(shlex.split(value))

    # print a short banner stating which crate is being built and the command
    # that's being used to build it
    print()
    print("-" * 80)
    print(f"Crate:   {env['CRATE'].split('/')[1]}")  # remove leading 'boards/'
    print(f"Command: {' '.join(cmd)}")
    print()

    # execute the build command for the crate
    subprocess.check_call(cmd, cwd=env["CRATE"])
