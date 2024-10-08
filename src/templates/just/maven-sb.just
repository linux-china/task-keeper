#!/usr/bin/env just --justfile

# maven build without tests
build:
   mvn -DskipTests clean package

# start spring boot
start:
  mvn -DskipTests spring-boot:run# dependencies tree for compile

# dump compile dependencies to encies.txt
dependencies:
  mvn dependency:tree -Dscope=compile > dependencies.txt

# display updates
updates:
  mvn versions:display-dependency-updates > updates.txt
