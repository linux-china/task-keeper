<!-- Keep a Changelog guide -> https://keepachangelog.com -->

# Task Keeper Changelog

## [Unreleased]

## [0.13.1] - 2023-10-30

- Fix to run task in README.md
- Add Dart's pubspec.yaml support

## [0.12.4] - 2023-09-09

- Add Bun support if `bun.lockb` detected.

## [0.12.3] - 2023-08-29

- Fix to add envs for Command

## [0.12.2] - 2023-04-30

- Introduce PyProjectToml struct for Rye and Poetry

## [0.12.1] - 2023-04-30

- `.python-version` support: find Python from Rye and pyenv
- Rye detection: `requirements.lock` or `[tool.rye]` in `pyproject.toml`

## [0.12.0] - 2023-04-28

- Add [Rye](https://github.com/mitsuhiko/rye) support
- Add [Rye scripts](https://github.com/mitsuhiko/rye#scripts) support
- Add `.venv/bin` in PATH

## [0.11.2] - 2023-04-23

- Introduce Logos to parse Markdown Attributes

## [0.11.1] - 2023-04-22

- Adjust Markdown Attribute parse to support `{#id desc="description"}` format
- Bug fix for `--runner` for `tk -l`

## [0.11.0] - 2023-04-08

- Add Apache Ant support

## [0.10.0] - 2023-03-08

- Add task validation for npm and composer when executing tasks from manager
- Add VS Code `.vscode/tasks.json` support

## [0.9.0] - 2022-12-31

- Add `java`, `jshelllanguage`, `groovy`, `kotlin` support in README.md, example as following:

~~~markdown
```kotlin {#k1}
fun main() {
    println("Hello world!")
}
```
~~~

~~~markdown
```java {#j1}
public class Demo {
    public static void main(String[] args) {
        System.out.println("Hello World!");
    }
}
```
~~~

~~~markdown
```groovy {#g1}
println "hello"
```
~~~

**Note**: Please install [JBang](https://www.jbang.dev/) first.

## [0.8.0] - 2022-10-20

### Add

- JavaScript/TypeScript code block in Markdown supported now: you can assign `.deno` or `.node` as js engine.

~~~markdown
```javascript {#task_name .deno}
console.log("hello world");
```
~~~

## [0.7.1] - 2022-10-11

### Fixed

- Gradle multi projects support

### Adjusted

- Task code block format adjusted: only `shell` and `sh` supported now

~~~markdown
```shell {#task_name}
curl https://httpbin.org/ip
```
~~~

## [0.6.5] - 2022-10-06

### Added

- requirements.txt support
- pipenv support
- Create Pipfile for pipenv: `tk --init=pipenv`

## [0.6.4] - 2022-09-29

### Added

- Update to Clap 4

## [0.6.3] - 2022-09-15

### Added

- Add laravel and CodeIgniter4 support
- Make conan optional for cmake

### Fixed

- Update testCompile to test-compile for Maven
- Fix bug for `tk --init=jbang`

## [0.6.2] - 2022-08-22

### Added

- Add `.node-modules/bin`, `venv/bin`, `bin` and `.bin` to the PATH environment variable.
- Run command line after double dash: `tk -- java --version`

## [0.6.1] - 2022-08-21

### Fixed

- Remove single `-` from task options

## [0.6.0] - 2022-08-20

### Added

- Version Manager for Java `.java-version` : find Java from `$HOME/.jbang/cache/jdks`
  or `$HOME/.sdkman/candidates/java/`
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
