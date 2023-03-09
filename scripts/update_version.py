#!/usr/bin/env python3

import os
import re
import sys
import time
import subprocess

VERSION_MATCHER = re.compile(r'^version = "([^"]+)\.([^"]+)\.([^"]+)"$')

def run(args, cwd="."):
    print("%s => %s" % (cwd, args))
    subprocess.check_call(args, cwd=cwd)

def parse_version(s):
    try:
        new_version = tuple([int(x) for x in s.split(".")])
        assert len(new_version) == 3
        return new_version
    except:
        raise Exception("Invalid version specification")

def update_version(path, new_version, categories):
    with open(path, "r") as f:
        contents = f.read().splitlines()

    in_appropriate_category = False

    for i, line in enumerate(contents):
        if line.startswith("["):
            in_appropriate_category = line in categories
        elif in_appropriate_category:
            match = VERSION_MATCHER.match(line)

            if match:
                old_version = tuple([int(x) for x in match.groups()])
                assert new_version > old_version, "New version should be greater than the old version"
                contents[i] = 'version = "{}"'.format(".".join(str(x) for x in new_version))

    with open(path, "w") as f:
        f.write("\n".join(contents))

def update_package_version(path, new_version):
    update_version(path, new_version, {"[package]"})

def update_main_dependency_versions(path, new_version):
    update_version(path, new_version, {"[dependencies.indradb-lib]", "[dependencies.indradb-proto]"})

def update_plugin_dependency_versions(path, new_version):
    update_version(path, new_version, {"[dependencies.indradb-plugin-host]"})

def update_all_versions(path, new_version):
    update_package_version(path, new_version)
    update_main_dependency_versions(path, new_version)

def main():
    if len(sys.argv) < 3:
        raise Exception("No versions specified")

    main_new_version = parse_version(sys.argv[1])
    plugin_host_new_version = parse_version(sys.argv[2])

    update_all_versions("lib/Cargo.toml", main_new_version)
    update_all_versions("proto/Cargo.toml", main_new_version)
    update_all_versions("server/Cargo.toml", main_new_version)
    update_all_versions("client/Cargo.toml", main_new_version)

    update_package_version("plugins/host/Cargo.toml", plugin_host_new_version)
    update_plugin_dependency_versions("proto/Cargo.toml", plugin_host_new_version)
    update_main_dependency_versions("plugins/host/Cargo.toml", main_new_version)

    run(["make", "check", "test"])

    # a github action will pickup this tag push and create a release
    new_version_str = "v{}".format(".".join(str(x) for x in main_new_version))
    run(["git", "commit", "-a", "-m", new_version_str])
    run(["git", "tag", "-a", new_version_str, "-m", new_version_str])
    run(["git", "push", "origin", new_version_str])

    run(["cargo", "publish"], cwd="lib")
    time.sleep(15) # wait for lib to be available on crates.io
    run(["cargo", "publish"], cwd="plugins/host")
    time.sleep(15) # wait for lib and plugin host to be accessible on crates.io
    run(["cargo", "publish"], cwd="proto")
    time.sleep(15) # wait for proto to be accessible on crates.io
    run(["cargo", "publish"], cwd="server")
    run(["cargo", "publish"], cwd="client")

if __name__ == "__main__":
    main()
