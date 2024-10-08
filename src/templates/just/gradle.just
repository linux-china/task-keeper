#!/usr/bin/env just --justfile

build:
  ./gradlew -x test build

dependencies:
  ./gradlew -q dependencies --configuration compileClasspath > ./dependencies.txt

# Report up-to-date dependencies by com.github.ben-manes.versions
updates:
  ./gradlew dependencyUpdates > updates.txt

wrapper:
  ./gradlew wrapper --gradle-version=8.10.1
