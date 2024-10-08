#!/usr/bin/env just --justfile
export PATH := justfile_directory() + "/.venv/bin:" + env_var('PATH')

upgrade:
  uv lock --upgrade

setup:
  uv add --dev pip3 setuptools
