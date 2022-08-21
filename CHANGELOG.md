<!-- Keep a Changelog guide -> https://keepachangelog.com -->

# Task Keeper Changelog

## [Unreleased]

## [0.6.1] - 2022-08-1

### Fixed

- Remove single `-` from task options

## [0.6.0] - 2022-08-20

### Added

- Version Manager for Java `.java-version` : find Java from `$HOME/.jbang/cache/jdks` or `$HOME/.sdkman/candidates/java/`
- Version Manager for Node.js `.java-version` : find Node.js from `$HOME/.nvm/versions/node`
  or `$HOME/.volta/tools/image/node`

## [0.5.2] - 2022-08-10

### Added

- packageManager detection for package.json
- Create task file for deno: `tk --init=deno`

## [0.5.1] - 2022-08-09

### Fixed

- Fix Gradle project with Bazel display

## [0.5.0] - 2022-08-10

### Added

- `--doctor` option added to check your system for potential problems to run tasks
- Create task file for jbang, make, just: `tk --init=jbang`, `tk --init=make`, `tk --init=just`
- hex package manager: mix and rebar3 support

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
