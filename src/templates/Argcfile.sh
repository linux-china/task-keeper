#!/usr/bin/env bash

set -e

# @cmd a simple build
build() {
    echo TODO build
}

# @cmd a simple test
test() {
    echo TODO test
}

# See more details at https://github.com/sigoden/argc
eval "$(argc --argc-eval "$0" "$@")"
