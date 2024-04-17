#!/usr/bin/env bash

set -e

# @cmd a simple build1
build1() {
    echo build1
}

# @cmd a simple test1
test1() {
    echo test1
}

# See more details at https://github.com/sigoden/argc
eval "$(argc --argc-eval "$0" "$@")"
