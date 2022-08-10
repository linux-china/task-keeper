<!-- Keep a Changelog guide -> https://keepachangelog.com -->

# Task Keeper Changelog

## [Unreleased]

### Added

- `--doctor` option added to check your system for potential problems to run tasks
- Create task file for jbang, make, just: `tk --init=jbang`, `tk --init=make`, `tk --init=just`

## [0.4.0] - 2022-08-09

### Added

- Bazel support
- Poetry support
- Maven & Gradle start for Spring Boot and Quarkus
- Makefile: use mmake if available

## [0.3.0] - 2022-08-08

### Added

- CMake ad Conan support
- Swift package manager support
- [JBang](https://www.jbang.dev/) support: jbang-catalog.json
- Adjust project/package managers' display

## [0.2.0] - 2022-08-07

### Added

- Package Manager support: maven, gradle, sbt, cargo, composer etc
- Composer scripts support
- vanilla shell script task.sh support: use `tk --init=shell` to generate `task.sh` file

### Changed

- Command line with pipes supported in Markdown: `curl --silent https://httpbin.org/ip | jq '.origin'`
- Yarn support: if `"packageManager"` in `package.json` contains `yarn`, then use `yarn run` instead of `npm run`
- Ignore runner whe no tasks found

## [0.1.0] - 2022-08-05

### Added

- Task Runner support: make, npm, deno, just, fleet, Rakefile, invoke, task, cargo-make, proc, markdown
- .env support by default: `tk --no-dotenv` to disable
