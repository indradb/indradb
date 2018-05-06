#!/usr/bin/env python

import os
import re
import shutil
import subprocess

LIB_TESTS = ["indradb"]
BIN_TESTS = ["indradb_server", "batch"]
TEST_FILE_PATTERN_TEMPLATE = r"^%s-[0-9a-f]{16}$"

EXCLUDE_PATTERNS = [
    "/.cargo",
    "/usr",
    "lib/src/tests",
    "lib/src/benches",
    "tests.rs",
]

LINUX = os.environ["TRAVIS_OS_NAME"] == "linux"
NIGHTLY = os.environ["TRAVIS_RUST_VERSION"] == "nightly"

def get_test_file_name(test_name):
    test_file_pattern = TEST_FILE_PATTERN_TEMPLATE % test_name

    for file in os.listdir("target/debug"):
        if re.match(test_file_pattern, file):
            return file

def run(args, cwd="."):
    print("%s => %s" % (cwd, args))
    subprocess.check_call(args, cwd=cwd)

def lib():
    run(["cargo", "update"], cwd="lib")

    if LINUX and NIGHTLY:
        run(["cargo", "test", "--features=test-suite,postgres-datastore,rocksdb-datastore", "--no-run"], cwd="lib")

        for lib_test in LIB_TESTS:
            run([
                "kcov", "--verify",
                "--exclude-pattern=%s" % ",".join(EXCLUDE_PATTERNS),
                "../target/kcov",
                "../target/debug/%s" % get_test_file_name(lib_test),
            ], cwd="lib")
    else:
        run(["cargo", "test", "--features=test-suite,postgres-datastore,rocksdb-datastore"], cwd="lib")

def bin():
    if NIGHTLY:
        run(["cargo", "build"], cwd="bin")

        if LINUX:
            run(["cargo", "test", "--features=test-suite", "--no-run"], cwd="bin")

            for lib_test in LIB_TESTS:
                run([
                    "kcov", "--verify",
                    "--exclude-pattern=%s" % ",".join(EXCLUDE_PATTERNS),
                    "../target/kcov",
                    "../target/debug/%s" % get_test_file_name(lib_test),
                ], cwd="lib")
        else:
            run(["cargo", "test", "--features=test-suite"], cwd="bin")
    else:
        print("Skipping bin tests as the compiler is not nightly")

def coverage():
    if LINUX and NIGHTLY:
        run([
            "kcov", "--merge", "--verify",
            "--exclude-pattern=%s" % ",".join(EXCLUDE_PATTERNS),
            "--coveralls-id=%s" % os.environ["TRAVIS_JOB_ID"],
            "target/kcov", "target/kcov",
        ])

if __name__ == "__main__":
    if LINUX and NIGHTLY:
        shutil.rmtree("target/kcov", ignore_errors=True)

    lib(LINUX, NIGHTLY)
    bin(LINUX, NIGHTLY)
    covdrage(LINUX, NIGHTLY)
