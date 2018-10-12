#!/usr/bin/env python

import os
import re
import shutil
import subprocess

LIB_TESTS = ["indradb"]
BIN_TESTS = ["common"]
TEST_FILE_PATTERN_TEMPLATE = r"^%s-[0-9a-f]{16}$"

EXCLUDE_PATTERNS = [
    "/.cargo",
    "/usr",
    "lib/src/tests",
    "lib/src/benches",
    "tests.rs"
]

def get_test_file_name(test_name):
    test_file_pattern = TEST_FILE_PATTERN_TEMPLATE % test_name

    for file in os.listdir("target/debug"):
        if re.match(test_file_pattern, file):
            return file

    raise Exception("No file matching the pattern `%s` in `target/debug`" % test_file_pattern)

def run(args, cwd="."):
    print("%s => %s" % (cwd, args))
    subprocess.check_call(args, cwd=cwd)

def main():
    run(["cargo", "build"], cwd="bin")

    if os.environ["TRAVIS_OS_NAME"] == "linux" and os.environ["TRAVIS_RUST_VERSION"] == "nightly":
        shutil.rmtree("target/kcov", ignore_errors=True)

        run(["cargo", "test", "--features=test-suite,rocksdb-datastore", "--no-run"], cwd="lib")
        run(["cargo", "test", "--features=test-suite", "--no-run"], cwd="bin")

        for lib_test in LIB_TESTS:
            run([
                "kcov", "--verify",
                "--exclude-pattern=%s" % ",".join(EXCLUDE_PATTERNS),
                "../target/kcov",
                "../target/debug/%s" % get_test_file_name(lib_test),
            ], cwd="lib")

        for bin_test in BIN_TESTS:
            run([
                "kcov", "--verify",
                "--exclude-pattern=%s" % ",".join(EXCLUDE_PATTERNS),
                "../target/kcov",
                "../target/debug/%s" % get_test_file_name(bin_test),
            ], cwd="bin")

        run([
            "kcov", "--merge", "--verify",
            "--exclude-pattern=%s" % ",".join(EXCLUDE_PATTERNS),
            "--coveralls-id=%s" % os.environ["TRAVIS_JOB_ID"],
            "target/kcov", "target/kcov",
        ])
    else:
        run(["cargo", "test", "--features=test-suite,rocksdb-datastore"], cwd="lib")
        run(["cargo", "test", "--features=test-suite"], cwd="bin")

if __name__ == "__main__":
    main()
