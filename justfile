# display help
help:
  cargo run --bin tk -- --help

task-hello:
  cargo run --bin tk -- hello

list:
  cargo run --bin tk -- --list

hello:
  echo "hello"

hello2:
  echo "hello2"

build:
  cargo build --bin tk
  cp target/debug/tk ~/bin
