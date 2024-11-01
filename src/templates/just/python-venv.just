#!/usr/bin/env just --justfile
export PATH := justfile_directory() + "/.venv/bin:" + env_var('PATH')

install:
  pip3 install -r requirements.txt
