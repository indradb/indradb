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
    "target",
    "/usr",
    "lib/src/tests",
    "lib/src/benches",
    "bin/src/common/autogen",
    "tests.rs"
]

def get_test_file_name(test_name):
    test_file_pattern = TEST_FILE_PATTERN_TEMPLATE % test_name

    for file in os.listdir("target/debug"):
        if re.match(test_file_pattern, file):
            return file

    raise Exception("No file matching the pattern `%s` in `target/debug`" % test_file_pattern)

def run(args, cwd=".", env=None):
    print("%s => %s" % (cwd, args))
    subprocess.check_call(args, cwd=cwd, env=env)

def run_tests(*args):
    test_bin_env = os.environ.copy()
    test_bin_env["ROCKSDB_MAX_OPEN_FILES"] = "1"

    lib_args = ["cargo", "test", "--features=test-suite,rocksdb-datastore"]
    lib_args.extend(args)
    run(lib_args, cwd="lib")

    bin_args = ["cargo", "test", "--features=test-suite"]
    bin_args.extend(args)
    run(bin_args, cwd="bin", env=test_bin_env)

def main():
    if os.environ["TRAVIS_OS_NAME"] == "linux" and os.environ["TRAVIS_RUST_VERSION"] == "stable":
        shutil.rmtree("target/kcov", ignore_errors=True)
        run_tests("--no-run")

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
    elif os.environ["TRAVIS_OS_NAME"] == "osx":
        # As of around Nov 27 2018, there seems to be some changes to TravisCI that drop the open file limit on macOS.
        # Test runs with default parallelism fail because they open too many files. To accomodate this, we disable
        # parallel test execution on macOS.
        # FIXME: audit IndraDB and find out why so many files are opened
        run_tests("--", "--test-threads=1")
    else:
        run_tests()

if __name__ == "__main__":
    main()
