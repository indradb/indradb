#!/usr/bin/env python3

import os
import requests
import glob
import re
import random
import json
import sys
import string
import time
from base64 import b64encode
import subprocess
from contextlib import contextmanager

COLOR_SUCCESS = '\033[92m'
COLOR_FAILURE = '\033[91m'
COLOR_END = '\033[0m'

# What the response body is set to if it is unparseable
UNABLE_TO_PARSE_BODY = "<unable to parse body>"

# If a test script has this pattern, the script is expected to trigger a
# (recovered) panic on the server
PANIC_ERROR_EXPECTED_PATTERN = re.compile("-- error: panic")

# If a test script has this pattern, the script is expected to trigger a
# runtime error on the server
RUNTIME_ERROR_EXPECTED_PATTERN = re.compile("-- error: runtime")

# If a test script has this pattern, the script is expected to return an OK
# response
OK_EXPECTED_PATTERN = re.compile("-- ok: (.+)$")

ACCOUNT_ID_MATCHER = re.compile("Account ID: (\d+)")

ACCOUNT_SECRET_MATCHER = re.compile("Account secret: (.+)")

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

@contextmanager
def server():
    """
    Context manager for running the server. This starts the server up, waits
    until its responsive, then yields. When the context manager's execution is
    resumed, it kills the server.
    """

    # Start the process
    server_proc = subprocess.Popen(["./bin/target/debug/nutrino-server"])

    while True:
        # Check if the server is now responding to HTTP requests
        try:
            res = requests.get("http://localhost:8000", timeout=1)

            if res.status_code == 401:
                break
        except requests.exceptions.RequestException:
            pass

        # Server is not yet responding to HTTP requests - let's make sure it's
        # running in the first place
        if server_proc.poll() != None:
            raise Exception("Server failed to start")

        time.sleep(1)

    yield
    server_proc.terminate()

def run_test(name, contents, user_id, secret):
    """Runs a given test"""

    result = Result(name)

    expecting_result_pattern = OK_EXPECTED_PATTERN.search(contents)
    expecting_result = expecting_result_pattern.group(1) if expecting_result_pattern else None
    expecting_runtime_error = RUNTIME_ERROR_EXPECTED_PATTERN.search(contents) != None
    expecting_panic_error = PANIC_ERROR_EXPECTED_PATTERN.search(contents) != None

    res = requests.post(
        "http://localhost:8000/script",
        auth=(user_id, secret),
        data=contents,
    )

    try:
        body = res.json()
    except ValueError:
        body = UNABLE_TO_PARSE_BODY

    if expecting_panic_error:
        if res.status_code == 500 and body == UNABLE_TO_PARSE_BODY:
            result.finish(True, res, body)
        else:
            result.finish(False, res, body)
    elif expecting_runtime_error:
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

    random_value = "".join(random.sample(string.ascii_letters + string.digits, 10))
    email = "test-script-runner-%s@nutrino.com" % random_value
    create_user_output = subprocess.check_output(["./bin/target/debug/nutrino-user", "add", email]).decode("utf-8")
    user_id = ACCOUNT_ID_MATCHER.search(create_user_output).groups()[0]
    secret = ACCOUNT_SECRET_MATCHER.search(create_user_output).groups()[0]

    try:
        with server():
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
    finally:
        subprocess.check_output(["./bin/target/debug/nutrino-user", "remove", user_id])

    if has_failure:
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) > 1:
        main(sys.argv[1])
    else:
        main()
