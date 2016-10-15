#!/usr/bin/env python3

import requests
import re
import random
import string
import time
import subprocess
from contextlib import contextmanager

ACCOUNT_ID_MATCHER = re.compile("Account ID: (\d+)")

ACCOUNT_SECRET_MATCHER = re.compile("Account secret: (.+)")

@contextmanager
def server():
    """
    Context manager for running the server. This starts the server up, waits
    until its responsive, then yields. When the context manager's execution is
    resumed, it kills the server.
    """

    # Start the process
    server_proc = subprocess.Popen(["./target/debug/nutrino-server"])
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

    try:
        yield
    finally:
        server_proc.terminate()

@contextmanager
def user():
    """
    Context manager for a nutrino user. This creates a user and yields. When
    the context manager's execution is resumed, it deletes the user.
    """

    random_value = "".join(random.sample(string.ascii_letters + string.digits, 10))
    email = "bin-test-%s@nutrino.com" % random_value
    create_user_output = subprocess.check_output(["./target/debug/nutrino-user", "add", email]).decode("utf-8")
    user_id = ACCOUNT_ID_MATCHER.search(create_user_output).groups()[0]
    secret = ACCOUNT_SECRET_MATCHER.search(create_user_output).groups()[0]

    try:
        yield (email, user_id, secret)
    finally:
        subprocess.check_output(["./target/debug/nutrino-user", "remove", user_id])
