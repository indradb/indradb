#!/usr/bin/env python

import os
import re
import shutil
import subprocess

LIB_TESTS = ["braid"]
BIN_TESTS = ["braid_server", "batch", "rest"]
TEST_FILE_PATTERN_TEMPLATE = r"^%s-[0-9a-f]{16}$"

EXCLUDE_PATTERNS = [
    "/.cargo",
    "/usr",
    "lib/src/tests",
    "tests.rs",
    "bin/tests",
]

def get_test_file_name(test_name):
    test_file_pattern = TEST_FILE_PATTERN_TEMPLATE % test_name

    for file in os.listdir("target/debug"):
        if re.match(test_file_pattern, file):
            return file

def main():
    subprocess.check_call(["cargo", "update"], cwd="lib")
    subprocess.check_call(["cargo", "build"], cwd="bin")

    if os.environ["TRAVIS_OS_NAME"] == "linux" and os.environ["TRAVIS_RUST_VERSION"] == "nightly":
        shutil.rmtree("target/kcov", ignore_errors=True)

        subprocess.check_call(["cargo", "test", "--all-features", "--no-run"], cwd="lib")
        subprocess.check_call(["cargo", "test", "--all-features", "--no-run"], cwd="bin")

        for lib_test in LIB_TESTS:
            subprocess.check_call([
                "kcov", "--verify",
                "--exclude-pattern=%s" % ",".join(EXCLUDE_PATTERNS),
                "../target/kcov",
                "../target/debug/%s" % get_test_file_name(lib_test),
            ], cwd="lib")

        for bin_test in BIN_TESTS:
            subprocess.check_call([
                "kcov", "--verify",
                "--exclude-pattern=%s" % ",".join(EXCLUDE_PATTERNS),
                "../target/kcov",
                "../target/debug/%s" % get_test_file_name(bin_test),
            ], cwd="bin")

        subprocess.check_call([
            "kcov", "--merge", "--verify",
            "--exclude-pattern=%s" % ",".join(EXCLUDE_PATTERNS),
            "--coveralls-id=%s" % os.environ["TRAVIS_JOB_ID"],
            "target/kcov", "target/kcov",
        ])
    else:
        subprocess.check_call(["cargo", "test", "--all-features"], cwd="lib")
        subprocess.check_call(["cargo", "test", "--all-features"], cwd="bin")

if __name__ == "__main__":
    main()
