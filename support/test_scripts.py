#!/usr/bin/env python3

import os
import requests
import glob
import re
import json
import sys
import util
import subprocess
from contextlib import contextmanager

COLOR_SUCCESS = '\033[92m'
COLOR_FAILURE = '\033[91m'
COLOR_END = '\033[0m'

# What the response body is set to if it is unparseable
UNABLE_TO_PARSE_BODY = "<unable to parse body>"

# If a test script has this pattern, the script is expected to trigger a
# runtime error on the server
RUNTIME_ERROR_EXPECTED_PATTERN = re.compile("-- error: runtime")

# If a test script has this pattern, the script is expected to return an OK
# response
OK_EXPECTED_PATTERN = re.compile("-- ok: (.+)$")

class Result(object):
    """Represents a test run result"""

    def __init__(self, name):
        self.name = name
        self.passed = None
        self.status_code = None
        self.body = None

    def finish(self, passed, res, body):
        """
        Finishes the result
        passed: Whether the test passed
        res: The response object
        """
        self.passed = passed
        self.status_code = res.status_code
        self.body = body

def run_test(name, contents, user_id, secret):
    """Runs a given test"""

    result = Result(name)

    expecting_result_pattern = OK_EXPECTED_PATTERN.search(contents)
    expecting_result = expecting_result_pattern.group(1) if expecting_result_pattern else None
    expecting_runtime_error = RUNTIME_ERROR_EXPECTED_PATTERN.search(contents) != None

    res = requests.post(
        "http://localhost:8000/script",
        auth=(user_id, secret),
        data=contents,
    )

    try:
        body = res.json()
    except ValueError:
        result.finish(True, res, UNABLE_TO_PARSE_BODY)
        return result

    if expecting_runtime_error:
        if res.status_code == 500 and "Script failed: Runtime" in body.get("error", ""):
            result.finish(True, res, body)
        else:
            result.finish(False, res, body)
    else:
        if res.status_code == 200:
            if expecting_result:
                if json.loads(expecting_result) == body:
                    result.finish(True, res, body)
                else:
                    result.finish(False, res, body)
            else:
                result.finish(True, res, body)
        else:
            result.finish(False, res, body)

    return result

def main(pattern="*.lua"):
    has_failure = False

    with util.user() as (email, user_id, secret):
        with util.server():
            for name in glob.glob(os.path.join("bin/support/test_scripts/", pattern)):
                with open(name, "r") as f:
                    contents = f.read()

                contents = contents.replace("__acount_id__", user_id)
                result = run_test(name, contents, user_id, secret)

                if result.passed:
                    print("%s✓ %s%s" % (COLOR_SUCCESS, result.name, COLOR_END))
                else:
                    print("%s✘ %s%s" % (COLOR_FAILURE, result.name, COLOR_END))
                    print("=> Status code: %s" % result.status_code)
                    print("=> Response json: %s" % result.body)
                    has_failure = True

    if has_failure:
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) > 1:
        main(sys.argv[1])
    else:
        main()
