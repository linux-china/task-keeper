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
* Execute command line: `tk -- node hello.js` with correct language version and PATH

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
* Check outdated deps: `mvn versions:display-dependency-updates`, `./gradlew dependencyUpdates`, `npm outdated` etc
* Build project: `mvn -DskipTests package`, `./gradlew assemble`, `cargo build` etc

Too many differences, I want to save my brain and keyboard, and you know MacBook keyboard's price tag is $400+.

# Task runners support

* make(Makefile): https://www.gnu.org/software/make/manual/make.html, [Modern Make](https://github.com/tj/mmake) support
* ant(build.xml): https://ant.apache.org/
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
* VS Code Tasks: https://code.visualstudio.com/docs/editor/tasks
* Rye: https://github.com/mitsuhiko/rye#scripts

**Tips**:

* Deno: please refer https://github.com/ije/esm.sh/releases/tag/v91 for npm packages manager

### Fleet Run configurations

command type support now:

* command
* maven/gradle
* cargo
* go
* maven-run
* cargo-run
* docker-run

For details, please refer https://www.jetbrains.com/help/fleet/run-configurations.html

# Language version detection and PATH

Task Keeper uses `.java-version`, `.node-version`  files to detect language version and bound with local installed SDK.

To make task runner run tasks smoothly, Task Keeper will append following directories to `PATH` automatically:

* `node-modules/.bin`
* `venv/bin` or `.venv/bin`
* `vendor/bin`
* `bin`
* `.bin`

For example, if you use Python virtual env to manage your project, Task Keeper will add `venv/bin` to `PATH`
automatically, and you don't need to do anything.

```
hello:
  python hello.py
```

**Tips**: you can use double dash to run command with language detection and correct `PATH`,
such as `tk -- mvn spring-boot:run`. To make life easy, and you can use `alias e='tk --'` to create an alias,
then you can run `e mvn spring-boot:run` to run your project.

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
* `yarn`, `pnpm`, `bun` support

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

### Python

Available with following tools:

* [Rye](https://github.com/mitsuhiko/rye): please commit `requirements.lock` to git
* [Poetry](https://python-poetry.org/)
* [pipenv](https://pipenv.pypa.io/en/latest/)
* requirements.txt

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

### Dart package manager

Available.

### Tasks from README.md

Task keeper will parse README.md and extract tasks with following code block format:

~~~markdown
```shell {#task_name}
curl https://httpbin.org/ip
```
~~~

Now only `shell`, `sh`, `javascript` and `typescript` are supported.

Run JavaScript/TypeScript by `node` or `deno`:

~~~markdown
```javascript {#task_name .deno}
console.log("hello world");
```
~~~

# Task options/params and global options

* Task options: `tk hello --name linux_china`
* Task params: `tk hello - Jackie`, use `-` to separate task params
* Global options for command:  `tk hello --name Jack -- --verbose`, use `--` double dash to separate global options

# Todo

## Task Runners

* jake(Jakefile): https://jakejs.com/docs-page.html#item-overview-jakefiles
* mask(maskfile.md): https://github.com/jacobdeichert/mask

# Package Managers

* realize(.realize.yaml): https://github.com/oxequa/realize

# Version detection

Task Keeper will detect version configuration file and adjust the environment variables to run tasks.

### Java

`.java-version` is used for version management, and values as following:

* 17: OpenJDK distributed by https://adoptium.net/
* 22.2.r17-grl: GraalVM

Task Keeper will try to find Java from `$HOME/.jbang/cache/jdks` or `$HOME/.sdkman/candidates/java/`.

`.sdkmanrc` support, and set HOME and PATH environment variables automatically. Please refer https://sdkman.io/usage#env
for detail.

### Node.js

`.node-version` is used for Node.js version management, and values as following:

* 18: match major version of Node.js
* 16.16.0: match version of Node.js

Task Keeper will try to find Node from `$HOME/.nvm/versions/node` or `$HOME/.volta/tools/image/node`.

# References

* The Ultimate Guide to Gemfile and
  Gemfile.lock: https://blog.saeloun.com/2022/08/16/understanding_gemfile_and_gemfile_lock
* Your Makefiles are wrong: https://tech.davis-hansson.com/p/make/
* Learn Makefiles With the tastiest examples: https://makefiletutorial.com/
* Taskfile: a modern alternative to Makefile - https://itnext.io/taskfile-a-modern-alternative-to-makefile-6b3f545f77bd

# Task scripts in Markdown

```shell {#demo}
$ curl https://httpbin.org/get
$ curl -X POST https://httpbin.org/post
```

```shell {#myip desc="get my internet ip address"}
curl --silent https://httpbin.org/ip | jq '.origin'
```

```shell {#demo2}
curl https://httpbin.org/ip \
    --user-agent "Task Keeper/0.1.0" \
    --silent
curl https://httpbin.org/headers
```

```typescript {#js2 .deno}
let name: string = "linux_china";
console.log(name);
```
