# display help
help:
  cargo run --bin tk -- --help

task-hello:
  cargo run --bin tk -- hello

list:
  cargo run --bin tk -- --list

vs-tasks:
  cargo run --bin tk -- --runner vscode --list

hello:
  echo "hello"

hello2:
  echo "hello2"

build:
  cargo build --bin tk
  cp target/debug/tk ~/bin
  cp target/debug/sq ~/bin

release:
  cargo build --release --bin tk
  cp target/release/tk ~/.cargo/bin/tk

# use alpine/git container to push the repo
push:
   docker run --rm -it -v "$(pwd)":/repo -v "$HOME/.ssh":/root/.ssh:ro -w /repo alpine/git push origin master
