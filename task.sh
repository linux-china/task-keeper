#!/bin/bash

# https://gist.github.com/mohanpedala/1e2ff5661761d3abd0385e8223e16425
set -eo pipefail

# load env vars from .env file
if [ -f .env ]; then
  export "$(< .env sed 's/#.*//g' | xargs)"
fi

function help() {
  echo "$1 <command> [options]"
  echo
  echo "commands: start, stop"
}

function start() {
  echo "starting..."
}

function stop() {
  echo "stopping..."
}

case "$1" in
start)
  start
  ;;
stop)
  stop
  ;;
run-script)
  . ./your_script "$2"
  ;;
*)
  help task
  exit 1
  ;;
esac
