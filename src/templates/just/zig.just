#!/usr/bin/env just --justfile

build:
  zig build--summary all

run:
  zig buildrun --summary all
