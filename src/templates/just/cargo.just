#!/usr/bin/env just --justfile

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1

example:
  cargo run --example exname -- arg1
