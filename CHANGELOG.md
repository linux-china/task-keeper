<!-- Keep a Changelog guide -> https://keepachangelog.com -->

# Task Keeper Changelog

## [0.30.9] - 2025-12-06

- Fix cmake project detection
- VS Code Task enhancement
- Update to amper 0.9.1
- Update to just 1.44

## [0.30.8] - 2025-10-26

- Add `tk sbom` for Maven and Gradle project to generate SBOM with CycloneDX format: `target/application.cdx.json` or
  `build/application.cdx.json`

Please check `$HOME/.gradle/init.d/plugins.gradle` with the following code:

```
initscript {
  repositories {
     gradlePluginPortal()
  }

  dependencies {
     classpath 'com.github.ben-manes:gradle-versions-plugin:0.53.0'
     classpath 'org.cyclonedx.bom:org.cyclonedx.bom.gradle.plugin:3.0.1'
  }
}

allprojects {
  apply plugin: com.github.benmanes.gradle.versions.VersionsPlugin
  apply plugin: org.cyclonedx.gradle.CyclonedxPlugin

  tasks.named("dependencyUpdates").configure {
    // configure the task, for example wrt. resolution strategies
  }
  
  // https://github.com/CycloneDX/cyclonedx-gradle-plugin/tree/master?tab=readme-ov-file#advanced-configuration
  tasks.named("cyclonedxDirectBom").configure {
     jsonOutput.set(file("build/application.cdx.json"))
     projectType = "application"
  }
}
```

## [0.30.7] - 2025-10-19

- Add `[tool.rye.scripts]`: https://rye.astral.sh/guide/pyproject/#toolryescripts

## [0.30.6] - 2025-10-14

- Add `jake`, `grunt`, `gulp` support for JS/TS project

## [0.30.5] - 2025-10-11

- Make bun as default JS/TS engine if no engine assigned for code block

## [0.30.4] - 2025-09-29

- Update to just 1.43.0
- Update to maven-wrapper-plugin 3.3.4

## [0.30.3] - 2025-08-30

- Update to dotenvx 0.4.14 to fix the decrypt issue
- Add dotenvx for `sq`
- rye removed
- Update initial templates

## [0.30.2] - 2025-08-15

- Update to dotenvx 0.4.8

## [0.30.1] - 2025-08-01

- Update to gradle 9.0.0
- Update to dotenvx 0.3

## [0.30.0] - 2025-07-25

- Add [dotenvx](https://dotenvx.com/) support for `.env` encryption file.

## [0.29.3] - 2025-07-20

- Update just to 1.42.3
- Update maven to 3.9.11
- Update gradle to 8.14.3

## [0.29.2] - 2025-07-02

- Fix `build.xml` parse error
- Update just to 1.41.0

## [0.29.1] - 2025-06-10

- Update to amper 0.7.0
- Apache Maven 3.9.10
- Use `minio-rs 0.3.0`

## [0.29.0] - 2025-05-05

- Add notification support: save output to OSS and notify via NATS

## [0.28.2] - 2025-04-01

- Update to amper 0.6.0
- Update to gradle 8.13

## [0.28.1] - 2025-01-12

- List tasks by default instead of help
- Add jsonc support for VS Code, Zed and Fleet

## [0.28.0] - 2025-01-02

- Add support for `invoke`, `poetry` and `poethepoet` installed by uv
- Update to gradle 8.12

## [0.27.0] - 2024-11-15

- Add [poe](https://github.com/nat-n/poethepoet) support if `[tool.poe]` detected in `pyproject.toml`
- Update to gradle 8.11

## [0.26.0] - 2024-11-12

- `self-update` for `amper`
- Use `uv pip list --outdated` to list outdated packages

## [0.25.0] - 2024-10-13

- Add `sq(Squirrel)` command-line snippets keeper: https://github.com/linux-china/task-keeper#sqsquirrel

## [0.24.3] - 2024-10-10

- Create justfile by project type: Cargo, Zig, uv etc.

## [0.24.2] - 2024-10-05

- Update to gradle 8.10.1
- Add `bun.lock` support
- Remove `init` from project manager

## [0.23.0] - 2024-08-21

- Add uv 0.3 support

## [0.22.2] - 2024-08-20

- Update to Gradle 8.10
- Update to Maven 3.9.9

## [0.22.1] - 2024-06-16

- Add release for cargo, cmake, zig etc

## [0.22.0] - 2024-06-09

- Add [Meson Build](https://mesonbuild.com/) support
- Add [xmake](https://xmake.io/) support

## [0.21.0] - 2024-05-23

- Add [amper](https://github.com/JetBrains/amper) standalone support

## [0.20.0] - 2024-05-09

- Add [nur](https://github.com/ddanier/nur) support
- Add [goreleaser](https://goreleaser.com/) support

## [0.19.0] - 2024-04-23

- Add [cargo-xtask](https://github.com/linux-china/xtask-demo) support
- Add [go-xtask](https://github.com/linux-china/xtask-go-demo) support
- Add `tk --init=argc` to create `Argcfile.sh` file
- Update to `maven-wrapper 3.3.0`

## [0.18.0] - 2024-04-17

- Add [argc](https://github.com/sigoden/argc) support
- Some bug fix for fleet

## [0.17.2] - 2024-04-11

- Add `.justfile` support
- Update to Gradle 8.7
- Add `$PORT` for procfile

## [0.17.1] - 2024-04-07

- Add [just module](https://github.com/casey/just?tab=readme-ov-file#modules1190) support
- Auto detect java version from pom.xml, Gradle java toolchain.

## [0.17.0] - 2024-03-19

- Add Zig Build System `build.zig` support: https://ziglang.org/learn/build-system/

## [0.16.0] - 2024-03-14

- Add `.zed/tasks.json` support: https://zed.dev/docs/tasks

## [0.15.1] - 2024-02-24

- Rename `self_update` to `self-update` for Maven/Gradle wrapper
- Add version lasted or not for for Maven/Gradle wrapper
- Add `php`,`flask` and `fastapi` type for Fleet `run.json` support
- Add `start` for Gradle project if `springframework.boot` or `quarkus` detected

## [0.15.0] - 2024-02-22

- Add `self_update` for Maven/Gradle wrapper
- Add [uv](https://github.com/astral-sh/uv) support if uv command detected

## [0.14.0] - 2024-01-23

- Add [Bun Shell](https://bun.sh/docs/runtime/shell) support: create a `Taskfile.ts` with following code:

```typescript
import {$} from "bun";

export async function hello() {
    await $`echo Hello World!`;
}

export async function list_js() {
    await $`ls *.js`;
}
```

Then run `tk hello` to run task with Bun Shell.

## [0.13.2] - 2023-12-10

- Fix bug to parse ID in Markdown Attributes
- Add exit code to run tasks #9
- Docs: add `.python-version` support
- Docs: add `task - Taskfile.yml`

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
