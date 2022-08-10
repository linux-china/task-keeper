Task Keeper
=================
tk(Task Keeper) is a tool to manage tasks from different task files,
such as `Makefile`,`justfile`, `package.json` , `deno.jso`, `.fleet/run.json` etc,
and call tasks from different project management tools,
such as `Apache Maven`, `Gradle`, `Cargo` and `npm` etc.

![Task Keeper](./screenshot.png)

# Features

* List tasks from different task files: `tk --list`
* Invoke task: `tk task_name`
* Invoke task from a runner: `tk --runner=npm start`
* Sync tasks between sources: `tk --from=npm --to=fleet task1 task2`
* .env support by default: `tk --no-dotenv` to disable
* `--doctor` support: check your system for potential problems to run tasks

# How to get started?

* Install by cargo: `cargo install task-keeper`
* Download and install from [GitHub Releases](https://github.com/linux-china/task-keeper/releases)

After install, execute `tk --help` for usage. Some commands as following:

* list tasks: `tk --list`
* execute task: `tk task_name`

# Why task keeper?

Sorry, I got lost in different task files and management tools, and sometimes I even can not remember how to run them.

* Find tasks: `Makefile`,`justfile`, `package.json`, `deno.json`, `Taskfile.yml`, `tasks.py`, `README.md` etc, and
  too many task files to check tasks.
* Run task: `just taskName`, `npm run task_name`, `deno task task_name`, `composer run-script task_name` etc
* Check outdated dependencies: `mvn versions:display-dependency-updates`, `./gradlew dependencyUpdates`, `npm outdated`
  etc
* Build project: `mvn -DskipTests package`, `./gradlew assemble`, `cargo build` etc

Too many differences, I want to save my brain and keyboard, and you know MacBook keyboard's price tag is $400+.

# Task runners support

* make(Makefile): https://www.gnu.org/software/make/manual/make.html, [Modern Make](https://github.com/tj/mmake) support
* npm(package.json): https://docs.npmjs.com/cli/v8/using-npm/scripts
* deno(deno.json): https://deno.land/manual/tools/task_runner
* composer(composer.json): https://getcomposer.org/doc/articles/scripts.md
* just(justfile): https://github.com/casey/just
* fleet(fleet/run.json): https://www.jetbrains.com/help/fleet/run-configurations.html#reference
* Rakefile(rake): https://ruby.github.io/rake/
* invoke(tasks.py): https://www.pyinvoke.org/
* task(Taskfile.yml): https://github.com/go-task/task  https://www.sobyte.net/post/2022-04/taskfile/
* cargo-make(Makefile.toml):  https://github.com/sagiegurari/cargo-make
* JBang(jbang-catalog.json): https://www.jbang.dev/documentation/guide/latest/alias_catalogs.html
* proc(Procfile): https://devcenter.heroku.com/articles/procfile
* markdown(README.md): shell code block support
* task.sh: vanilla shell script

# Package manager support

### Common tasks for all package managers:

* init: create project by manager `mvn archetype:generate`, `npm init`, `cargo new xxx` etc
* install: install all dependencies `npm istall`, `dependency:resolve`
* compile: compile source code, not available for some script languages
* build: cargo: `tk build -- --release`, maven: `mvn -DskipTests clean package`, npm: `npm run build`
* start: start project `go run main.go`
* test: run test npm: `npm test`, maven: `mvn test`, cargo: `cargo test`
* doc: generate documentation `mvn javadoc:javadoc`
* deps: list all dependencies
* clean: clean build artifacts, maven: `mvn clean`, cargo: `cargo clean`
* outdated: display outdated dependencies `go list -u -m all`
* update: update outdated dependencies `go get -u`
* add dependency: `tk add dependency` or `tk -D add dependency` or `tk --runner=npm add dependency`

**Attention**: if package manager's task name is in a task runner, and task keeper will not execute package manager's
command.

### Apache Maven

Available

### Gradle

Please set up [gradle-versions-plugin](https://github.com/ben-manes/gradle-versions-plugin) for dependency version
management.
You can transparently add the plugin to every Gradle project that you run via a Gradle init script.
`$HOME/.gradle/init.d/plugins.gradle` with following code:

```
initscript {
  repositories {
     gradlePluginPortal()
  }

  dependencies {
    classpath 'com.github.ben-manes:gradle-versions-plugin:+'
  }
}

allprojects {
  apply plugin: com.github.benmanes.gradle.versions.VersionsPlugin

  tasks.named("dependencyUpdates").configure {
    // configure the task, for example wrt. resolution strategies
  }
}
```

### Sbt

Please add [sbt-updates](https://github.com/rtimush/sbt-updates) and DependencyTreePlugin as global plugins.
`$HOME/.sbt/1.0/plugins/plugins.sbt` with following code:

```
addSbtPlugin("com.timushev.sbt" % "sbt-updates" % "0.6.3")
addDependencyTreePlugin
```

### npm

Available

* If `npm-check` command available, `npm-check -u` will be used as `outdated` task.

### Cargo

Available

### Composer

Available

### Bundler

Available

### Go Module

Available

### CMake

Only support [cmake-conan](https://github.com/conan-io/cmake-conan), and conanfile.txt required.

Default build directory is `cmake-build-debug`, and you override it by `CMAKE_BINARY_DIR=_build tk build`.

### Swift

Available. Please install [swift-outdated](https://github.com/kiliankoe/swift-outdated) for `outdated` operation.

### Bazel

Available.

### Poetry

Available.

### Lein

Available.

Please set up antq for outdated dependencies. `$HOME/.lein/profiles.clj`

```clojure
{
 :user
 {:dependencies [[com.github.liquidz/antq "RELEASE"]]
  :aliases {"outdated" ["run" "-m" "antq.core"]}
 }
}
```

### Mix package manager

Available.

### Rebar3 package manager

Available.

# Task options and global options

Task options are options for task, such as `tk hello --name linux_china`.
Global options are options for task runner and seperated by double dash, such as `tk hello --name Jack -- --verbose`

# Todo

## Task Runners

* Apache Ant(build.xml): https://ant.apache.org/
* jake(Jakefile): https://jakejs.com/docs-page.html#item-overview-jakefiles
* code(.vscode/launch.json): https://code.visualstudio.com/docs/editor/debugging#_launchjson-attributes
* mask(maskfile.md): https://github.com/jacobdeichert/mask

# Package Managers

* Pipenv(Pipfile): https://pipenv.pypa.io/en/latest/
* realize(.realize.yaml): https://github.com/oxequa/realize

# Version detection

such as `.node_version`, `.java_version` 

# Shell scripts in Markdown

```shell
$ ## http-methods
$ curl https://httpbin.org/get
$ curl -X POST https://httpbin.org/post
```

```shell
## print my internet ip
curl --silent https://httpbin.org/ip | jq '.origin'
```

```shell
curl https://httpbin.org/ip \
    --user-agent "Task Keeper/0.1.0" \
    --silent
curl https://httpbin.org/headers
```

