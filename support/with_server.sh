#!/bin/bash

set -e
set -a

# Close all child processes on exit
function on_quit {
    kill $PID
}

./target/debug/nutrino-server &
PID=$!
trap on_quit EXIT
$@
