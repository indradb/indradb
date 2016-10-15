#!/usr/bin/env python3

import util
import subprocess

def main():
    with util.server():
        subprocess.check_call(["cargo", "test"])

if __name__ == "__main__":
    main()
